//! Create PKPassLibrary and query passes via generated bindings.
//!
//! Demonstrates PassKit framework by creating a PKPassLibrary,
//! querying passes, and checking library capabilities.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::objc::id;

use apple_sys::PassKit::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== PassKit ===\n");

        // Check if PassKit is available
        println!("--- PKPassLibrary ---");
        let is_available = PKPassLibrary::isPassLibraryAvailable().0;
        println!("  Pass library available: {}", is_available);

        // Create a library instance
        let library = PKPassLibrary::alloc();
        let library = PKPassLibrary(INSObject::init(&library));
        if !library.0.is_null() {
            println!("  Library: {}", common::nsobj_to_string(library.0));

            // Query passes
            let passes = IPKPassLibrary::passes(&library);
            if !passes.0.is_null() {
                let count = INSArray::<id>::count(&passes);
                println!("  Total passes: {}", count);

                // Show details for first few
                let show_count = count.min(5);
                for i in 0..show_count {
                    let pass = PKPass(INSArray::<id>::objectAtIndex_(&passes, i));

                    let pass_type = IPKPass::passType(&pass);
                    let org_name = IPKPass::organizationName(&pass);
                    let desc_str = IPKPass::localizedDescription(&pass);
                    let serial = IPKPass::serialNumber(&pass);

                    let type_name = match pass_type {
                        0 => "Barcode",
                        1 => "SecureElement",
                        _ => "Unknown",
                    };

                    println!("    Pass [{}]:", i);
                    println!("      Type: {} ({})", type_name, pass_type);
                    println!("      Organization: {}", nsstring_to_string(org_name));
                    println!("      Description: {}", nsstring_to_string(desc_str));
                    println!("      Serial: {}", nsstring_to_string(serial));
                }

                if count > show_count {
                    println!("    ... and {} more passes", count - show_count);
                }
            } else {
                println!("  Passes: nil (no access or empty)");
            }
        }

        // PKPaymentAuthorizationController
        println!("\n--- PKPaymentAuthorizationController ---");
        let can_make = PKPaymentAuthorizationController::canMakePayments().0;
        println!("  Can make payments: {}", can_make);
    }

    println!("\nDone.");
}
