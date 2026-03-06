//! Query CloudKit container info and account status.
//!
//! Uses CKContainer to check the default container identifier
//! and available database types.

use apple_sys::CloudKit::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CloudKit Container ===\n");

        // Default container
        let container = CKContainer::defaultContainer();
        if !container.0.is_null() {
            let identifier = container.containerIdentifier();
            println!("Default container: {}", nsstring_to_string(identifier));
        } else {
            println!("No default container available");
        }
    }

    println!("\nDone.");
}
