//! Check NearbyInteraction capabilities.
//!
//! Uses NISession and NIDeviceCapability to query
//! UWB hardware support and measurement capabilities.

use apple_sys::NearbyInteraction::*;

use objc2::runtime::AnyObject;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== NearbyInteraction ===\n");

        // NIDeviceCapability is a protocol with no concrete struct;
        // PNIDeviceCapability trait exists but has no implementors.
        let caps = NISession::deviceCapabilities();
        if !caps.is_null() {
            let caps_obj = caps as *mut AnyObject;
            let precise: bool = objc2::msg_send![caps_obj, supportsPreciseDistanceMeasurement];
            let direction: bool = objc2::msg_send![caps_obj, supportsDirectionMeasurement];
            let camera: bool = objc2::msg_send![caps_obj, supportsCameraAssistance];
            let extended: bool = objc2::msg_send![caps_obj, supportsExtendedDistanceMeasurement];

            println!("Device capabilities:");
            println!("  Precise distance: {}", precise);
            println!("  Direction: {}", direction);
            println!("  Camera assistance: {}", camera);
            println!("  Extended distance: {}", extended);
        } else {
            println!("Device capabilities: not available");
        }

        // Create a session
        let session = NISession::alloc();
        let session = NISession(INSObject::init(&session));
        if !session.0.is_null() {
            let token = INISession::discoveryToken(&session);
            println!("\nNISession created");
            println!(
                "  Discovery token: {}",
                if token.0.is_null() { "none" } else { "present" }
            );

            // Invalidate
            INISession::invalidate(&session);
            println!("  Session invalidated");
        }
    }

    println!("\nDone.");
}
