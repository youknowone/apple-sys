use crate::Builder;

#[test]
fn generate_core_foundation() {
    Builder::with_builtin_config("CoreFoundation")
        .sdk("macosx")
        .unwrap()
        .generate();
}
