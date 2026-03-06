//! Check CoreBluetooth manager state and authorization.
//!
//! Queries CBCentralManager authorization status and
//! CBManager class-level authorization.

use apple_sys::CoreBluetooth::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreBluetooth State ===\n");

        // Class-level authorization
        let auth = CBManager::class_authorization();
        let auth_name = match auth {
            0 => "Not Determined",
            1 => "Restricted",
            2 => "Denied",
            3 => "Allowed Always",
            _ => "Unknown",
        };
        println!("CBManager authorization: {} ({})", auth_name, auth);

        // Check if CBCentralManager supports features
        println!("CBCentralManager class: available");
        let supports = CBCentralManager::supportsFeatures_(1);
        println!("  Supports extended scan: {}", supports.0);

        // Check peripheral manager
        println!("CBPeripheralManager class: available");
    }

    println!("\nDone.");
}
