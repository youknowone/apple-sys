//! Inspect AppleScriptKit plugin infrastructure.
//!
//! Creates an ASKPluginObject instance and verifies the generated bindings
//! work for the AppleScriptKit framework.

use apple_sys::AppleScriptKit::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::objc::id;
use objc2::msg_send;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AppleScriptKit Plugin ===\n");

        // Create an ASKPluginObject instance using the generated bindings
        let obj = ASKPluginObject::alloc();
        let obj_ptr: id = msg_send![&*obj.0, init];
        if obj_ptr.is_null() {
            println!("Failed to create ASKPluginObject instance.");
            return;
        }
        let obj = ASKPluginObject(obj_ptr);
        println!("ASKPluginObject instance created.");
        println!("Description: {}", nsobj_to_string(obj.0));
    }

    println!("\nDone.");
}
