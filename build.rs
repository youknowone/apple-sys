use serde::Deserialize;
use std::{collections::HashMap, env, io::Write, path::PathBuf};

#[derive(Deserialize, Default)]
struct Config {
    #[serde(default)]
    opaque_types: Vec<String>,
    #[serde(default)]
    blocklist_items: Vec<String>,
    #[serde(default)]
    replacements: Vec<String>,
}

impl Config {
    fn chain<'a>(
        &'a self,
        default: &'a Self,
        get: impl Fn(&Self) -> &Vec<String>,
    ) -> impl Iterator<Item = &'a str> {
        get(self)
            .iter()
            .chain(get(default).iter())
            .map(String::as_str)
    }
    fn opaque_types<'a>(&'a self, default: &'a Self) -> impl Iterator<Item = &'a str> {
        self.chain(default, |c| &c.opaque_types)
    }
    fn blocklist_items<'a>(&'a self, default: &'a Self) -> impl Iterator<Item = &'a str> {
        self.chain(default, |c| &c.blocklist_items)
    }
    fn replacements<'a>(&'a self, default: &'a Self) -> impl Iterator<Item = &'a str> {
        self.chain(default, |c| &c.replacements)
    }
}

fn sdk_path() -> Result<String, std::io::Error> {
    use std::process::Command;
    let sdk = "macosx";
    let output = Command::new("xcrun")
        .args(&["--sdk", sdk, "--show-sdk-path"])
        .output()?
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    Ok(prefix_str.trim_end().to_string())
}

fn load_config() -> HashMap<String, Config> {
    toml::from_str(include_str!("Bindgen.toml")).expect("Bindgen.toml is corrupted")
}

fn build(
    framework: &str,
    config: &Config,
    default: &Config,
    sdk_path: Option<&str>,
    target: Option<&str>,
    layout_tests: bool,
) {
    println!("cargo:rustc-link-lib=framework={framework}");

    // Begin building the bindgen params.
    let mut builder = bindgen::Builder::default();

    let mut clang_args = vec!["-x", "objective-c", "-fblocks", "-fmodules"];
    let target_arg;
    if let Some(target) = target {
        target_arg = format!("--target={}", target);
        clang_args.push(&target_arg);
    }

    if let Some(sdk_path) = sdk_path {
        clang_args.extend(&["-isysroot", sdk_path]);
    }

    builder = builder
        .clang_args(&clang_args)
        .layout_tests(layout_tests)
        .rustfmt_bindings(true);

    for opaque_type in config.opaque_types(default) {
        builder = builder.opaque_type(opaque_type);
    }
    for blocklist_item in config.blocklist_items(default) {
        builder = builder.blocklist_item(blocklist_item);
    }

    builder = builder.header_contents(&format!("{framework}.h"), &format!("@import {framework};"));

    // Generate the bindings.
    let bindings = builder.generate().expect("unable to generate bindings");

    // Get the cargo out directory.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

    // TODO: find the best way to do this post-processing
    let mut out = bindings.to_string();
    for replacement in config.replacements(default) {
        let (old, new) = replacement
            .split_once(" #=># ")
            .expect("Bindgen.toml is misformatted");
        out = out.replace(old, new);
    }

    // Write them to the crate root.
    let mut file = std::fs::File::create(out_dir.join(format!("{framework}.rs")))
        .expect("could not open bindings file");
    file.write_all(out.as_bytes())
        .expect("could not write bindings");
}

fn main() {
    let frameworks = include!("build_features.inc.rs");
    let config = load_config();
    let empty_config = Config::default();
    let default_config = config
        .get("default")
        .expect("[default] not fonud in Bindgen.toml");

    #[cfg(not(feature = "__allow_empty"))]
    if frameworks.is_empty() {
        panic!("apple-sys accepts module names as features. Empty feature maybe a mistake. If this is intentional, add to features: '__allow_empty'");
    }

    let target = std::env::var("TARGET").ok();
    let directory = sdk_path().ok();
    for framework in frameworks {
        build(
            framework,
            config.get(framework).unwrap_or(&empty_config),
            default_config,
            directory.as_ref().map(String::as_str),
            target.as_ref().map(String::as_str),
            false,
        );
    }
}
