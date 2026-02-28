//! Create a CNContactPicker and inspect its configuration.
//!
//! Uses the ContactsUI framework to examine contact picker properties,
//! and checks CNContactStore and CNContactViewController bindings.

use apple_sys::Contacts::*;
use apple_sys::ContactsUI::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ContactsUI Picker ===\n");

        // CNContactPicker
        let picker = CNContactPicker::alloc();
        let picker_ptr: id = objc2::msg_send![&*picker.0, init];
        if picker_ptr.is_null() {
            println!("Failed to create CNContactPicker.");
            return;
        }
        let picker = CNContactPicker(picker_ptr);
        println!("CNContactPicker created.");

        // Check displayedKeys
        let displayed_keys = ICNContactPicker::displayedKeys(&picker);
        if !displayed_keys.0.is_null() {
            let count = INSArray::<id>::count(&displayed_keys);
            println!("Displayed keys: {count}");
            for i in 0..count {
                let key: id = INSArray::<id>::objectAtIndex_(&displayed_keys, i);
                println!("  Key #{}: {}", i + 1, nsobj_to_string(key));
            }
        } else {
            println!("Displayed keys: (default/none set)");
        }

        // Delegate
        let delegate = ICNContactPicker::delegate(&picker);
        println!(
            "Delegate: {}",
            if (delegate as *const u64).is_null() {
                "not set"
            } else {
                "set"
            }
        );

        println!("Description: {}", nsobj_to_string(picker.0));

        // CNContactViewController - class method
        let descriptor = CNContactViewController::descriptorForRequiredKeys();
        if !descriptor.is_null() {
            println!(
                "\nCNContactViewController.descriptorForRequiredKeys: {}",
                nsobj_to_string(descriptor as id)
            );
        }

        // CNContactStore
        let store = CNContactStore::alloc();
        let store_ptr = INSObject::init(&store);
        if !store_ptr.is_null() {
            let store = CNContactStore(store_ptr);
            println!("\n--- CNContactStore ---");
            println!("CNContactStore created.");

            // Check default container
            let container = ICNContactStore::defaultContainerIdentifier(&store);
            if !container.0.is_null() {
                println!("Default container: {}", nsstring_to_string(container));
            }
        }
    }

    println!("\nDone.");
}
