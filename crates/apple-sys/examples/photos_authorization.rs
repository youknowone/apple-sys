//! Query Photos framework authorization status.
//!
//! Uses PHPhotoLibrary to check authorization and
//! list available PHAssetCollectionType values.

use apple_sys::Photos::*;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Photos Framework ===\n");

        // Check authorization for read-write (access level 1)
        let auth_rw = PHPhotoLibrary::authorizationStatusForAccessLevel_(1);
        let auth_name = |s: i64| match s {
            0 => "Not Determined",
            1 => "Restricted",
            2 => "Denied",
            3 => "Authorized",
            4 => "Limited",
            _ => "Unknown",
        };
        println!(
            "Authorization (read-write): {} ({})",
            auth_name(auth_rw),
            auth_rw
        );

        // Check authorization for add-only (access level 2)
        let auth_add = PHPhotoLibrary::authorizationStatusForAccessLevel_(2);
        println!(
            "Authorization (add-only):   {} ({})",
            auth_name(auth_add),
            auth_add
        );

        // Shared library
        let shared = PHPhotoLibrary::sharedPhotoLibrary();
        if !shared.0.is_null() {
            println!("\nShared photo library: {:?}", shared.0);
        }
    }

    println!("\nDone.");
}
