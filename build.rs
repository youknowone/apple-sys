use serde::Deserialize;
use std::{collections::HashMap, env, io::Write, path::PathBuf};

#[derive(Deserialize)]
struct ConfigMap {
    #[serde(flatten)]
    map: HashMap<String, Config>,
}

impl ConfigMap {
    fn get(&self, name: &str) -> ChainedConfig {
        let mut configs = Vec::new();
        let mut name = Some(name.to_owned());
        while let Some(config) = name.map(|s| self.map.get(s.as_str())).flatten() {
            name = config.deps.clone();
            configs.push(config.clone());
        }
        configs.push(self.map.get("default").unwrap().clone());
        ChainedConfig(configs)
    }
}
#[derive(Deserialize, Default, Clone)]
struct Config {
    deps: Option<String>,
    #[serde(default)]
    opaque_types: Vec<String>,
    #[serde(default)]
    blocklist_items: Vec<String>,
    #[serde(default)]
    replacements: Vec<String>,
}

struct ChainedConfig(Vec<Config>);

impl ChainedConfig {
    fn chain<'a>(&'a self, get: impl Fn(&Config) -> &Vec<String>) -> impl Iterator<Item = &str> {
        let mut config_iter = self.0.iter();
        let mut item_iter: Box<dyn Iterator<Item = &'a String>> =
            Box::new(get(config_iter.next().unwrap()).iter());
        for config in config_iter {
            item_iter = Box::new(item_iter.chain(get(config).iter()));
        }
        item_iter.map(String::as_str)
    }
    fn opaque_types<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.chain(|c| &c.opaque_types)
    }
    fn blocklist_items<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.chain(|c| &c.blocklist_items)
    }
    fn replacements<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.chain(|c| &c.replacements)
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

fn build(
    framework: &str,
    config: ChainedConfig,
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

    for opaque_type in config.opaque_types() {
        builder = builder.opaque_type(opaque_type);
    }
    for blocklist_item in config.blocklist_items() {
        builder = builder.blocklist_item(blocklist_item);
    }

    builder = builder.header_contents(&format!("{framework}.h"), &format!("@import {framework};"));

    // println!("cargo:error={:?}", builder.command_line_flags().join(" "));

    // Generate the bindings.
    let bindings = builder.generate().expect("unable to generate bindings");

    // Get the cargo out directory.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

    // TODO: find the best way to do this post-processing
    let mut out = bindings.to_string();
    for replacement in config.replacements() {
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
    println!("cargo:rerun-if-changed=build_features.inc.rs");
    println!("cargo:rerun-if-changed=Bindgen.toml");

    let frameworks = include!("build_features.inc.rs");
    let config: ConfigMap =
        toml::from_str(include_str!("Bindgen.toml")).expect("Bindgen.toml is corrupted");

    #[cfg(not(feature = "__allow_empty"))]
    if frameworks.is_empty() {
        panic!("apple-sys accepts module names as features. Empty feature maybe a mistake. If this is intentional, add to features: '__allow_empty'");
    }

    let target = std::env::var("TARGET").ok();
    let directory = sdk_path().ok();
    for framework in frameworks {
        build(
            framework,
            config.get(framework),
            directory.as_ref().map(String::as_str),
            target.as_ref().map(String::as_str),
            false,
        );
    }
}
