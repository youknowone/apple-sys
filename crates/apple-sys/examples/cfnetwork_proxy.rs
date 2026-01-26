//! Display system proxy settings via CFNetwork.
//!
//! Uses CFNetworkCopySystemProxySettings to retrieve the system's
//! HTTP/HTTPS/SOCKS proxy configuration.

use apple_sys::CFNetwork::*;
use apple_sys::CoreFoundation::*;
use std::ptr;

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
        println!("=== System Proxy Settings ===\n");

        let settings = CFNetworkCopySystemProxySettings();
        if settings.is_null() {
            println!("No proxy settings available");
            return;
        }

        // Print number of keys
        let count = CFDictionaryGetCount(settings);
        println!("Proxy dictionary has {} key(s)\n", count);

        if count == 0 {
            CFRelease(settings as _);
            return;
        }

        // Get all keys and values
        let mut keys = vec![ptr::null(); count as usize];
        let mut values = vec![ptr::null(); count as usize];
        CFDictionaryGetKeysAndValues(settings, keys.as_mut_ptr(), values.as_mut_ptr());

        for i in 0..count as usize {
            let key = keys[i] as CFStringRef;
            let key_str = cfstring_to_string(key);

            let value = values[i];
            let type_id = CFGetTypeID(value);

            let val_str = if type_id == CFStringGetTypeID() {
                cfstring_to_string(value as CFStringRef)
            } else if type_id == CFNumberGetTypeID() {
                let mut n: i64 = 0;
                let cf_number_sint64_type: CFNumberType = 4;
                CFNumberGetValue(
                    value as CFNumberRef,
                    cf_number_sint64_type,
                    &mut n as *mut _ as _,
                );
                n.to_string()
            } else if type_id == CFBooleanGetTypeID() {
                if CFBooleanGetValue(value as CFBooleanRef) != 0 {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            } else if type_id == CFArrayGetTypeID() {
                let arr = value as CFArrayRef;
                let len = CFArrayGetCount(arr);
                format!("[Array, {} item(s)]", len)
            } else if type_id == CFDictionaryGetTypeID() {
                let dict = value as CFDictionaryRef;
                let len = CFDictionaryGetCount(dict);
                format!("[Dict, {} key(s)]", len)
            } else {
                format!("[TypeID={}]", type_id)
            };

            println!("  {}: {}", key_str, val_str);
        }

        CFRelease(settings as _);
    }

    println!("\nDone.");
}
