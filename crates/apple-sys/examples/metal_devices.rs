//! Enumerate Metal GPU devices and their capabilities.
//!
//! Uses MTLCopyAllDevices to list available GPUs and
//! queries their properties via generated bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSArray, NSAutoreleasePool, NSString};
use apple_sys::Metal::*;
use apple_sys::objc::id;
use objc2::runtime::AnyObject;
mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Metal GPU Device Enumeration ===\n");

        // 1. Get system default device
        let default_device = MTLCreateSystemDefaultDevice() as *mut AnyObject;
        if !default_device.is_null() {
            // MTLDevice is a protocol with no concrete struct;
            // PMTLDevice trait exists but has no implementors.
            let name: *mut AnyObject = objc2::msg_send!(default_device, name);
            println!(
                "Default device: {}",
                nsstring_to_string(NSString(name as _))
            );
        } else {
            println!("No default Metal device");
        }

        // 2. Copy all available devices
        let devices = MTLCopyAllDevices();
        let devices_arr = NSArray(devices.0);
        if devices.0.is_null() {
            println!("MTLCopyAllDevices returned null");
            return;
        }

        let count = INSArray::<id>::count(&devices_arr);
        println!("\nTotal Metal devices: {}\n", count);

        for i in 0..count {
            let device: id = INSArray::<id>::objectAtIndex_(&devices_arr, i);
            if device.is_null() {
                continue;
            }

            // MTLDevice is a protocol (see above).
            let name: *mut AnyObject = objc2::msg_send!(device, name);
            let is_low_power: bool = objc2::msg_send!(device, isLowPower);
            let is_removable: bool = objc2::msg_send!(device, isRemovable);
            let has_unified_memory: bool = objc2::msg_send!(device, hasUnifiedMemory);
            let recommended_max: u64 = objc2::msg_send!(device, recommendedMaxWorkingSetSize);
            let max_buf: u64 = objc2::msg_send!(device, maxBufferLength);
            let max_threadgroup_mem: u64 = objc2::msg_send!(device, maxThreadgroupMemoryLength);

            println!("Device {}: {}", i, nsstring_to_string(NSString(name as _)));
            println!("  Low power: {}", is_low_power);
            println!("  Removable: {}", is_removable);
            println!("  Unified memory: {}", has_unified_memory);
            println!(
                "  Recommended max working set: {} MB",
                recommended_max / (1024 * 1024)
            );
            println!("  Max buffer length: {} MB", max_buf / (1024 * 1024));
            println!(
                "  Max threadgroup memory: {} KB",
                max_threadgroup_mem / 1024
            );
        }
    }

    println!("\nDone.");
}
