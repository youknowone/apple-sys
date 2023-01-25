use apple_bindgen::Builder;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=macos.inc.rs");
    println!("cargo:rerun-if-changed=ios.inc.rs");

    let target = std::env::var("TARGET").expect("env TARGET must be set");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    let frameworks = match target_os.as_str() {
        "macos" => include!("macos.inc.rs"),
        "ios" => include!("ios.inc.rs"),
        unknown => panic!("unexpected target_os: {}", unknown),
    };

    let platform = apple_sdk::Platform::from_target_triple(&target)
        .expect("Unknown apple platform. please report it.");

    #[cfg(not(feature = "__allow_empty"))]
    if frameworks.is_empty() {
        panic!("apple-sys accepts module names as features. Empty feature maybe a mistake. If this is intentional, add to features: '__allow_empty'");
    }

    // Get the cargo out directory.
    let out_dir =
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

    for framework in frameworks {
        println!("cargo:rustc-link-lib=framework={framework}");

        let mut builder =
            Builder::with_builtin_config(framework, &platform).expect("sdk lookup failed");
        if let Ok(target) = std::env::var("TARGET") {
            builder = builder.target(target);
        }
        let out = builder.generate().expect("binding generation failed");

        // Write them to the crate root.
        let mut file = std::fs::File::create(out_dir.join(format!("{framework}.rs")))
            .expect("could not open bindings file");
        file.write_all(out.as_bytes())
            .expect("could not write bindings");
    }
}
