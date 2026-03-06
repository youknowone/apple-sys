//! Check DeviceCheck and App Attest capabilities.
//!
//! Queries DCDevice.currentDevice.isSupported and
//! DCAppAttestService.shared.isSupported.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

use apple_sys::DeviceCheck::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== DeviceCheck & App Attest ===\n");

        // DCDevice
        println!("--- DCDevice ---");
        let device = DCDevice::currentDevice();
        if !device.0.is_null() {
            let supported = IDCDevice::isSupported(&device);
            println!("  Current device: {}", nsobj_to_string(device.0));
            println!("  DeviceCheck supported: {}", supported.0);
        } else {
            println!("  currentDevice returned nil");
        }

        // DCAppAttestService
        println!("\n--- DCAppAttestService ---");
        let shared = DCAppAttestService::sharedService();
        if !shared.0.is_null() {
            let supported = IDCAppAttestService::isSupported(&shared);
            println!("  Shared service: {}", nsobj_to_string(shared.0));
            println!("  App Attest supported: {}", supported.0);
        } else {
            println!("  sharedService returned nil");
        }
    }

    println!("\nDone.");
}
