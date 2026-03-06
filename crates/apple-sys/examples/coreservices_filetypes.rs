//! Query file type handlers using CoreServices LaunchServices.
//!
//! Lists applications that can handle specific URL schemes (http, mailto, etc.)
//! and shows the default handler for each.

use apple_sys::CoreFoundation::*;
use apple_sys::CoreServices::*;

fn cfstring_to_string(cf: CFStringRef) -> Option<String> {
    if cf.is_null() {
        return None;
    }
    unsafe {
        let mut buf = [0u8; 1024];
        let ok = CFStringGetCString(
            cf,
            buf.as_mut_ptr() as *mut i8,
            buf.len() as CFIndex,
            0x08000100,
        );
        if ok != 0 {
            let cstr = std::ffi::CStr::from_ptr(buf.as_ptr() as *const i8);
            Some(cstr.to_string_lossy().into_owned())
        } else {
            None
        }
    }
}

fn cfstring_create(s: &str) -> CFStringRef {
    unsafe {
        CFStringCreateWithBytes(
            std::ptr::null(),
            s.as_ptr(),
            s.len() as CFIndex,
            0x08000100, // kCFStringEncodingUTF8
            0,          // isExternalRepresentation = false
        )
    }
}

fn main() {
    let schemes = ["http", "https", "mailto", "ssh", "ftp", "tel"];

    println!("URL Scheme Handlers:");
    println!("{:-<60}", "");

    for scheme in &schemes {
        let cf_scheme = cfstring_create(scheme);
        if cf_scheme.is_null() {
            continue;
        }

        unsafe {
            // Get default handler
            let default_handler = LSCopyDefaultHandlerForURLScheme(cf_scheme);
            let default_name =
                cfstring_to_string(default_handler).unwrap_or_else(|| "(none)".to_string());

            println!("  {scheme}://");
            println!("    Default: {default_name}");

            // Get all handlers
            let all_handlers = LSCopyAllHandlersForURLScheme(cf_scheme);
            if !all_handlers.is_null() {
                let count = CFArrayGetCount(all_handlers);
                if count > 0 {
                    print!("    All ({count}): ");
                    for i in 0..count {
                        let handler = CFArrayGetValueAtIndex(all_handlers, i) as CFStringRef;
                        if let Some(name) = cfstring_to_string(handler) {
                            if i > 0 {
                                print!(", ");
                            }
                            print!("{name}");
                        }
                    }
                    println!();
                }
                CFRelease(all_handlers as CFTypeRef);
            }

            if !default_handler.is_null() {
                CFRelease(default_handler as CFTypeRef);
            }
            CFRelease(cf_scheme as CFTypeRef);
        }
        println!();
    }
}
