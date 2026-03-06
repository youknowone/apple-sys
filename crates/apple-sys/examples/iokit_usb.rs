//! List USB devices using IOKit framework.
//!
//! Enumerates all USB host devices via IOServiceGetMatchingServices and prints
//! their names using IORegistryEntryGetName.

use apple_sys::CoreFoundation::CFDictionaryRef;
use apple_sys::IOKit::*;

fn main() {
    unsafe {
        let matching = IOServiceMatching(c"IOUSBHostDevice".as_ptr());
        if matching.is_null() {
            eprintln!("IOServiceMatching returned null");
            return;
        }

        let mut iterator: io_iterator_t = 0;
        let kr = IOServiceGetMatchingServices(0, matching as CFDictionaryRef, &mut iterator);
        if kr != 0 {
            eprintln!("IOServiceGetMatchingServices failed: {kr}");
            return;
        }

        println!("USB Devices:");
        let mut name_buf = [0i8; 128];
        let mut count = 0u32;
        loop {
            let service = IOIteratorNext(iterator);
            if service == 0 {
                break;
            }

            let kr = IORegistryEntryGetName(service, name_buf.as_mut_ptr());
            if kr == 0 {
                let name = std::ffi::CStr::from_ptr(name_buf.as_ptr());
                println!("  - {}", name.to_string_lossy());
                count += 1;
            }
            IOObjectRelease(service);
        }
        IOObjectRelease(iterator);
        println!("Total: {count} USB devices found");
    }
}
