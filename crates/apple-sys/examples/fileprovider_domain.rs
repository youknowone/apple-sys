//! Inspect FileProvider framework domain management.
//!
//! Gets NSFileProviderManager.defaultManager and queries
//! available signal and enumeration methods.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::FileProvider::*;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== FileProvider Domain Management ===\n");

        // NSFileProviderManager
        println!("--- NSFileProviderManager ---");
        println!("  Class: NSFileProviderManager");

        let mgr = NSFileProviderManager::defaultManager();
        if !mgr.0.is_null() {
            println!("  Default manager: {}", nsobj_to_string(mgr.0));
        } else {
            println!("  defaultManager returned nil (no default provider)");
        }

        // NSFileProviderDomain
        println!("\n--- NSFileProviderDomain ---");
        println!("  Class: NSFileProviderDomain");

        let identifier = nsstring(c"com.example.test");
        let display_name = nsstring(c"Test Domain");
        let domain = NSFileProviderDomain::alloc();
        let domain_ptr = INSFileProviderDomain::initWithIdentifier_displayName_(
            &domain,
            identifier,
            display_name,
        );

        if !domain_ptr.is_null() {
            let domain = NSFileProviderDomain(domain_ptr);
            let id_val = INSFileProviderDomain::identifier(&domain);
            let name_val = INSFileProviderDomain::displayName(&domain);
            let is_disconnected = INSFileProviderDomain::isDisconnected(&domain);

            println!("  Created domain:");
            println!("    Identifier:    {}", nsstring_to_string(id_val));
            println!("    Display name:  {}", nsstring_to_string(name_val));
            println!("    Disconnected:  {}", is_disconnected.0);
        } else {
            println!("  (failed to create domain instance)");
        }

        // NSFileProviderItemIdentifier well-known values
        println!("\n--- Well-known Item Identifiers ---");
        println!("  (These are string constants defined by the framework)");
        println!("  RootContainer:       NSFileProviderRootContainerItemIdentifier");
        println!("  WorkingSet:          NSFileProviderWorkingSetContainerItemIdentifier");
        println!("  TrashContainer:      NSFileProviderTrashContainerItemIdentifier");
    }

    println!("\nDone.");
}
