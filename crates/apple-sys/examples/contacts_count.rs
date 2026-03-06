//! Query the Contacts store for authorization status and container info.
//!
//! Uses CNContactStore to check authorization and list contact containers.

use apple_sys::Contacts::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, INSError, NSAutoreleasePool, NSError, NSPredicate};
use apple_sys::objc::id;
mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Contacts Framework ===\n");

        // Check authorization status (0=Contacts entity type)
        let auth_status = CNContactStore::authorizationStatusForEntityType_(0);
        let status_name = match auth_status {
            0 => "Not Determined",
            1 => "Restricted",
            2 => "Denied",
            3 => "Authorized",
            4 => "Limited",
            _ => "Unknown",
        };
        println!("Authorization status: {} ({})", status_name, auth_status);

        // Create store instance
        let store = CNContactStore::alloc();
        let store_ptr = INSObject::init(&store);
        assert!(!store_ptr.is_null(), "Failed to create CNContactStore");
        let store = CNContactStore(store_ptr);

        // Try to get containers
        let error = NSError(std::ptr::null_mut());
        let containers = ICNContactStore::containersMatchingPredicate_error_(
            &store,
            NSPredicate(std::ptr::null_mut()),
            &error as *const NSError as *mut NSError,
        );

        if !containers.0.is_null() {
            let count = INSArray::<id>::count(&containers);
            println!("Containers: {}", count);

            for i in 0..count {
                let container: id = INSArray::<id>::objectAtIndex_(&containers, i);
                let container = CNContainer(container);
                let name = ICNContainer::name(&container);
                let identifier = ICNContainer::identifier(&container);
                let ctype = ICNContainer::type_(&container);
                let type_name = match ctype {
                    0 => "Unassigned",
                    1 => "Local",
                    2 => "Exchange",
                    3 => "CardDAV",
                    _ => "Other",
                };
                println!(
                    "  {:2}. {} ({}) [{}]",
                    i + 1,
                    nsstring_to_string(name),
                    type_name,
                    nsstring_to_string(identifier)
                );
            }
        } else if !error.0.is_null() {
            let desc = INSError::localizedDescription(&error);
            println!("Error accessing contacts: {}", nsstring_to_string(desc));
        }

        // Default container
        let default = ICNContactStore::defaultContainerIdentifier(&store);
        if !default.0.is_null() {
            println!("Default container ID: {}", nsstring_to_string(default));
        }
    }

    println!("\nDone.");
}
