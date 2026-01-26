//! Display system information using Foundation APIs.
//!
//! Shows hostname, OS version, hardware specs, thermal state,
//! selected environment variables, and disk usage.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::*;
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Foundation System Information ===\n");

        // --- NSProcessInfo ---
        let proc_info = NSProcessInfo::processInfo();

        let hostname = INSProcessInfo::hostName(&proc_info);
        let os_ver = INSProcessInfo::operatingSystemVersionString(&proc_info);
        let proc_name = INSProcessInfo::processName(&proc_info);
        let phys_mem = INSProcessInfo::physicalMemory(&proc_info);
        let cpu_count = INSProcessInfo::processorCount(&proc_info);
        let active_cpus = INSProcessInfo::activeProcessorCount(&proc_info);
        let uptime = INSProcessInfo::systemUptime(&proc_info);

        println!("--- Process Info ---");
        println!("  Hostname:        {}", nsstring_to_string(hostname));
        println!("  Process:         {}", nsstring_to_string(proc_name));
        println!("  OS Version:      {}", nsstring_to_string(os_ver));
        println!(
            "  Physical Memory: {:.2} GB",
            phys_mem as f64 / (1024.0 * 1024.0 * 1024.0)
        );
        println!("  Processors:      {cpu_count} ({active_cpus} active)");

        let hours = (uptime / 3600.0) as u64;
        let mins = ((uptime % 3600.0) / 60.0) as u64;
        println!("  System Uptime:   {hours}h {mins}m");

        // Thermal state
        let thermal = NSProcessInfo_NSProcessInfoThermalState::thermalState(&proc_info);
        let thermal_name = match thermal {
            0 => "Nominal",
            1 => "Fair",
            2 => "Serious",
            3 => "Critical",
            _ => "Unknown",
        };
        println!("  Thermal State:   {thermal_name}");

        // Low power mode
        let low_power = NSProcessInfo_NSProcessInfoPowerState::isLowPowerModeEnabled(&proc_info);
        println!("  Low Power Mode:  {}", low_power.0);

        // --- Environment ---
        println!("\n--- Environment (selected) ---");
        let env = INSProcessInfo::environment(&proc_info);
        for key_name in [c"USER", c"HOME", c"SHELL", c"LANG", c"TERM"] {
            let key = nsstring(key_name);
            let val = INSDictionary::<NSString, id>::objectForKey_(&env, key.0);
            if !val.is_null() {
                println!("  {}: {}", key_name.to_str().unwrap(), nsobj_to_string(val));
            }
        }

        // --- Disk Usage ---
        println!("\n--- Disk Usage (/) ---");
        let fm = NSFileManager::defaultManager();

        let root = nsstring(c"/");
        let error: *mut NSError = std::ptr::null_mut();
        let attrs =
            INSFileManager::attributesOfFileSystemForPath_error_(&fm, root, error as *mut _);

        if !attrs.0.is_null() {
            let size_key = nsstring(c"NSFileSystemSize");
            let free_key = nsstring(c"NSFileSystemFreeSize");

            let size_num = INSDictionary::<NSString, id>::objectForKey_(&attrs, size_key.0);
            let free_num = INSDictionary::<NSString, id>::objectForKey_(&attrs, free_key.0);

            if !size_num.is_null() && !free_num.is_null() {
                let total = INSNumber::unsignedLongLongValue(&NSNumber(size_num)) as u64;
                let free = INSNumber::unsignedLongLongValue(&NSNumber(free_num)) as u64;
                let used = total - free;

                let to_gb = |b: u64| b as f64 / (1024.0 * 1024.0 * 1024.0);
                println!("  Total: {:.2} GB", to_gb(total));
                println!("  Used:  {:.2} GB", to_gb(used));
                println!("  Free:  {:.2} GB", to_gb(free));
                println!("  Usage: {:.1}%", used as f64 / total as f64 * 100.0);
            }
        } else if !error.is_null() {
            let desc = INSError::localizedDescription(&NSError(error as _));
            println!("  Error: {}", nsstring_to_string(desc));
        }
    }

    println!("\nDone.");
}
