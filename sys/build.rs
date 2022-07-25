use apple_bindgen::Builder;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=build_features.inc.rs");

    let frameworks = include!("build_features.inc.rs");

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
            Builder::with_builtin_config(framework, "macosx").expect("sdk lookup failed");
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
