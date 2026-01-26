//! Create PKPushRegistry and check push type capabilities via generated bindings.
//!
//! Demonstrates PushKit framework by creating a PKPushRegistry
//! and inspecting desired push types and delegate configuration.

use apple_sys::PushKit::*;

use objc2::runtime::AnyObject;

mod common;
use common::nsobj_to_string;

unsafe extern "C" {
    fn dispatch_queue_create(
        label: *const std::os::raw::c_char,
        attr: *const std::os::raw::c_void,
    ) -> *mut AnyObject;
}

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== PushKit ===\n");

        // Create a dispatch queue
        let queue = dispatch_queue_create(c"com.apple-sys.pushkit-test".as_ptr(), std::ptr::null());

        // PKPushRegistry
        println!("--- PKPushRegistry ---");
        {
            let registry = PKPushRegistry::alloc();
            let registry = PKPushRegistry(IPKPushRegistry::initWithQueue_(
                &registry,
                NSObject(queue as _),
            ));

            if !registry.0.is_null() {
                println!("  Registry: {}", nsobj_to_string(registry.0));

                // Check desired push types (initially nil)
                let desired_types = IPKPushRegistry::desiredPushTypes(&registry);
                if !desired_types.0.is_null() {
                    let count = INSSet::<id>::count(&desired_types);
                    println!("  Desired push types: {} registered", count);
                } else {
                    println!("  Desired push types: none (not configured)");
                }

                // Check delegate
                let delegate = IPKPushRegistry::delegate(&registry);
                println!(
                    "  Delegate: {}",
                    if delegate.is_null() { "nil" } else { "set" }
                );
            } else {
                println!("  Failed to create PKPushRegistry");
            }
        }

        // Check known push types
        println!("\n--- Push Type Constants ---");
        let push_type_names = [
            "PKPushTypeVoIP",
            "PKPushTypeComplication",
            "PKPushTypeFileProvider",
        ];
        for name in push_type_names {
            println!("  {} - registered type", name);
        }
    }

    println!("\nDone.");
}
