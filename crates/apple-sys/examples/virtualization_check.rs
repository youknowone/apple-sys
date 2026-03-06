//! Check Virtualization framework capabilities.
//!
//! Queries VZVirtualMachineConfiguration for supported
//! hardware and available VM configuration options.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSError, NSAutoreleasePool, NSError};
use apple_sys::Virtualization::*;
use apple_sys::objc::id;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Virtualization Framework ===\n");

        let config = VZVirtualMachineConfiguration::alloc();
        let config_ptr = INSObject::init(&config);
        if !config_ptr.is_null() {
            let min_cpu = VZVirtualMachineConfiguration::minimumAllowedCPUCount();
            let max_cpu = VZVirtualMachineConfiguration::maximumAllowedCPUCount();
            let min_mem = VZVirtualMachineConfiguration::minimumAllowedMemorySize();
            let max_mem = VZVirtualMachineConfiguration::maximumAllowedMemorySize();

            println!("\nVM Configuration limits:");
            println!("  CPU count:   {} - {}", min_cpu, max_cpu);
            println!(
                "  Memory size: {} MB - {} MB",
                min_mem / (1024 * 1024),
                max_mem / (1024 * 1024)
            );

            let config = VZVirtualMachineConfiguration(config_ptr);
            let error: *mut NSError = std::ptr::null_mut();
            let valid = config.validateWithError_(error);
            if !valid.0 && !error.is_null() {
                let desc = INSError::localizedDescription(&NSError(error as id));
                println!("\nEmpty config validation: {}", nsstring_to_string(desc));
            }
        }
    }

    println!("\nDone.");
}
