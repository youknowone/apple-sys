use std::{collections::HashMap, env, path::PathBuf};

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

fn load_config() -> HashMap<String, HashMap<String, Vec<String>>> {
    toml::from_str(include_str!("Bindgen.toml")).expect("Bindgen.toml is corrupted")
}

fn build(
    framework: &str,
    config: &HashMap<String, Vec<String>>,
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

    for opaque_type in config
        .get("opaque_types")
        .map(|v| v.as_slice())
        .unwrap_or(&[])
    {
        builder = builder.opaque_type(opaque_type);
    }
    for blocklist_item in config
        .get("blocklist_items")
        .map(|v| v.as_slice())
        .unwrap_or(&[])
    {
        builder = builder.blocklist_item(blocklist_item);
    }

    builder = builder.header_contents(&format!("{framework}.h"), &format!("@import {framework};"));

    // Generate the bindings.
    let bindings = builder.generate().expect("unable to generate bindings");

    // Get the cargo out directory.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

    // Write them to the crate root.
    bindings
        .write_to_file(out_dir.join(format!("{framework}.rs")))
        .expect("could not write bindings");
}

fn main() {
    let frameworks = include!("build_features.inc.rs");
    let config = load_config();
    let empty_config = HashMap::new();

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
            directory.as_ref().map(String::as_str),
            target.as_ref().map(String::as_str),
            false,
        );
    }
}
