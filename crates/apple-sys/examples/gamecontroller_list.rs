//! List connected game controllers via GameController.framework.
//!
//! Queries GCController for connected controllers and displays
//! their names, vendor, and supported profiles.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool, NSDictionary_NSExtendedDictionary};
use apple_sys::GameController::*;
use apple_sys::objc::id;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Game Controllers ===\n");

        // Get connected controllers
        let controllers = GCController::controllers();
        let count = INSArray::<id>::count(&controllers);
        println!("Connected controllers: {}\n", count);

        for i in 0..count {
            let ctrl_id = INSArray::<id>::objectAtIndex_(&controllers, i);
            let ctrl = GCController(ctrl_id);
            let vendor = <GCController as PGCDevice>::vendorName(&ctrl);
            let product = <GCController as PGCDevice>::productCategory(&ctrl);
            let attached = <GCController as IGCController>::isAttachedToDevice(&ctrl);
            let snapshot = <GCController as GCController_Snapshot>::isSnapshot(&ctrl);

            println!("Controller {}:", i + 1);
            println!("  Vendor:   {}", nsstring_to_string(vendor));
            println!("  Category: {}", nsstring_to_string(product));
            println!("  Attached: {}", attached.0);
            println!("  Snapshot: {}", snapshot.0);

            // Check profiles
            let extended = <GCController as IGCController>::extendedGamepad(&ctrl);
            let micro = <GCController as IGCController>::microGamepad(&ctrl);
            let gamepad = <GCController as IGCController>::gamepad(&ctrl);

            print!("  Profiles:");
            if !extended.0.is_null() {
                print!(" ExtendedGamepad");
            }
            if !gamepad.0.is_null() {
                print!(" Gamepad");
            }
            if !micro.0.is_null() {
                print!(" MicroGamepad");
            }
            println!();

            // Physical input profile
            let profile = <GCController as PGCDevice>::physicalInputProfile(&ctrl);
            if !profile.0.is_null() {
                let buttons = IGCPhysicalInputProfile::buttons(&profile);
                let btn_keys = NSDictionary_NSExtendedDictionary::<id, id>::allKeys(&buttons);
                let btn_count = INSArray::<id>::count(&btn_keys);
                println!("  Buttons: {}", btn_count);
            }
            println!();
        }

        if count == 0 {
            println!("No controllers connected.");
            println!("Connect a game controller (Xbox, DualSense, etc.) to see details.");
        }

        // Check for current controller
        let current = GCController::current();
        if !current.0.is_null() {
            let vendor = <GCController as PGCDevice>::vendorName(&current);
            println!("Current controller: {}", nsstring_to_string(vendor));
        }
    }

    println!("\nDone.");
}
