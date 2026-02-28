//! Inspect DeviceDiscoveryExtension classes.
//!
//! Creates a DDDevice with basic parameters and checks class availability
//! for DDDiscoverySession and related types.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::DeviceDiscoveryExtension::*;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::UniformTypeIdentifiers::UTType;
mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== DeviceDiscoveryExtension ===\n");

        // DDDevice
        println!("--- DDDevice ---");
        println!("  Class: DDDevice");

        // Create a DDDevice instance
        let display_name = nsstring(c"Test Device");
        let category: DDDeviceCategory = 0; // DDDeviceCategoryHiFiSpeaker
        let protocol_type = nsstring(c"com.example.protocol");
        let identifier = nsstring(c"test-identifier");

        let device = DDDevice::alloc();
        let device_ptr = IDDDevice::initWithDisplayName_category_protocolType_identifier_(
            &device,
            display_name,
            category,
            UTType(protocol_type.0),
            identifier,
        );

        if !device_ptr.is_null() {
            let device = DDDevice(device_ptr);
            let name = IDDDevice::displayName(&device);
            let cat = IDDDevice::category(&device);
            let proto = IDDDevice::protocolType(&device);

            let cat_str = match cat {
                0 => "HiFiSpeaker",
                1 => "HiFiSpeakerMultiple",
                2 => "TVWithMediaBox",
                3 => "TV",
                4 => "LaptopComputer",
                5 => "DesktopComputer",
                _ => "Unknown",
            };

            println!("  Created DDDevice:");
            println!("    Display name: {}", nsstring_to_string(name));
            println!("    Category:     {cat_str} ({cat})");
            println!("    Protocol:     {}", nsobj_to_string(proto.0));
        } else {
            println!("  (failed to create DDDevice instance)");
        }

        // DDDiscoverySession
        println!("\n--- DDDiscoverySession ---");
        println!("  Class: DDDiscoverySession");

        // DDDeviceEvent
        println!("\n--- DDDeviceEvent ---");
        println!("  Class: DDDeviceEvent");
    }

    println!("\nDone.");
}
