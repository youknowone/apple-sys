//! Check SACrashDetectionManager availability using SafetyKit.
//!
//! Queries the crash detection manager for device support and
//! authorization status via generated bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::SafetyKit::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SafetyKit Crash Detection ===\n");

        // Check if the device supports crash detection
        let supported = SACrashDetectionManager::isAvailable();
        println!("Crash detection available: {}", supported.0);

        // Create an instance to check authorization
        let manager = SACrashDetectionManager::alloc();
        let manager_ptr = INSObject::init(&manager);
        if !manager_ptr.is_null() {
            let manager = SACrashDetectionManager(manager_ptr);
            println!("Manager instance: {}", nsobj_to_string(manager_ptr));

            // Authorization status: 0=notDetermined, 1=denied, 2=authorized
            let auth_status = ISACrashDetectionManager::authorizationStatus(&manager);
            let status_name = match auth_status {
                0 => "Not Determined",
                1 => "Denied",
                2 => "Authorized",
                _ => "Unknown",
            };
            println!("Authorization status: {status_name} ({auth_status})");
        } else {
            println!("Failed to create manager instance.");
        }
    }

    println!("\nDone.");
}
