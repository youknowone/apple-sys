//! List disc recording (optical drive) devices.
//!
//! Uses DRCopyDeviceArray to enumerate available
//! CD/DVD burners and their capabilities.

use apple_sys::CoreFoundation::{
    CFArrayGetCount, CFArrayGetValueAtIndex, CFDictionaryGetValue, CFRelease,
    CFStringCreateWithCString, CFStringEncoding, CFStringGetCString, CFStringGetLength,
    CFStringGetMaximumSizeForEncoding, CFStringRef,
};
use apple_sys::DiscRecording::*;

const UTF8_ENCODING: CFStringEncoding = 0x08000100;

unsafe fn cfstring_to_string(s: CFStringRef) -> String {
    if s.is_null() {
        return String::new();
    }
    let len = unsafe { CFStringGetLength(s) };
    let max_size = unsafe { CFStringGetMaximumSizeForEncoding(len, UTF8_ENCODING) } + 1;
    let mut buf = vec![0u8; max_size as usize];
    if unsafe { CFStringGetCString(s, buf.as_mut_ptr() as _, max_size, UTF8_ENCODING) } != 0 {
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        String::from_utf8_lossy(&buf[..end]).into_owned()
    } else {
        String::new()
    }
}

fn main() {
    unsafe {
        println!("=== DiscRecording Devices ===\n");

        let devices = DRCopyDeviceArray();
        if devices.is_null() {
            println!("No disc recording devices found (or framework unavailable).");
            println!("\nDone.");
            return;
        }

        let count = CFArrayGetCount(devices);
        println!("Optical drives found: {}\n", count);

        for i in 0..count {
            let device = CFArrayGetValueAtIndex(devices, i) as DRDeviceRef;
            let valid = DRDeviceIsValid(device);
            println!("Device {}:", i + 1);
            println!("  Valid: {}", valid != 0);

            // Get device info dictionary
            let info = DRDeviceCopyInfo(device);
            if !info.is_null() {
                // Try to get vendor and product
                let vendor_key = CFStringCreateWithCString(
                    std::ptr::null(),
                    c"DRDeviceVendorNameKey".as_ptr(),
                    UTF8_ENCODING,
                );
                let product_key = CFStringCreateWithCString(
                    std::ptr::null(),
                    c"DRDeviceProductNameKey".as_ptr(),
                    UTF8_ENCODING,
                );

                let vendor = CFDictionaryGetValue(info, vendor_key as _) as CFStringRef;
                let product = CFDictionaryGetValue(info, product_key as _) as CFStringRef;
                println!("  Vendor:  {}", cfstring_to_string(vendor));
                println!("  Product: {}", cfstring_to_string(product));

                CFRelease(vendor_key as _);
                CFRelease(product_key as _);
                CFRelease(info as _);
            }
            println!();
        }

        CFRelease(devices as _);
    }

    println!("Done.");
}
