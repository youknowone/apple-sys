use crate::Builder;

#[test]
fn generate_core_foundation() {
    Builder::with_builtin_config("CoreFoundation", "macosx")
        .expect("macosx sdk not found")
        .generate()
        .expect("generate failed");
}
