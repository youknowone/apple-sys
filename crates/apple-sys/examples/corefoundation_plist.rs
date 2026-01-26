//! Create a property list dictionary and serialize it to XML.
//!
//! Demonstrates CoreFoundation's CFDictionary, CFString, CFNumber,
//! CFBoolean, CFArray, and CFPropertyList APIs.

#![allow(non_upper_case_globals)]

use apple_sys::CoreFoundation::*;
use std::os::raw::c_void;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
const kCFNumberFloat64Type: CFNumberType = 13;
const kCFNumberSInt32Type: CFNumberType = 3;

unsafe fn cfstring(s: &std::ffi::CStr) -> CFStringRef {
    unsafe { CFStringCreateWithCString(kCFAllocatorDefault, s.as_ptr(), kCFStringEncodingUTF8) }
}

fn main() {
    unsafe {
        println!("=== CoreFoundation Property List Demo ===\n");

        // 1. Create the root dictionary
        let dict = CFDictionaryCreateMutable(
            kCFAllocatorDefault,
            0,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );
        assert!(!dict.is_null(), "Failed to create dictionary");

        // 2. String entry
        let key = cfstring(c"AppName");
        let val = cfstring(c"apple-sys");
        CFDictionarySetValue(dict, key as *const c_void, val as *const c_void);
        CFRelease(key as CFTypeRef);
        CFRelease(val as CFTypeRef);

        // 3. Floating-point number
        let key = cfstring(c"Version");
        let version: f64 = 0.3;
        let val = CFNumberCreate(
            kCFAllocatorDefault,
            kCFNumberFloat64Type,
            &version as *const f64 as *const c_void,
        );
        CFDictionarySetValue(dict, key as *const c_void, val as *const c_void);
        CFRelease(key as CFTypeRef);
        CFRelease(val as CFTypeRef);

        // 4. Boolean
        let key = cfstring(c"Active");
        CFDictionarySetValue(dict, key as *const c_void, kCFBooleanTrue as *const c_void);
        CFRelease(key as CFTypeRef);

        // 5. Array of framework names
        let key = cfstring(c"Frameworks");
        let array = CFArrayCreateMutable(kCFAllocatorDefault, 0, &kCFTypeArrayCallBacks);
        for name in [c"CoreFoundation", c"AppKit", c"Metal", c"WebKit"] {
            let s = cfstring(name);
            CFArrayAppendValue(array, s as *const c_void);
            CFRelease(s as CFTypeRef);
        }
        CFDictionarySetValue(dict, key as *const c_void, array as *const c_void);
        CFRelease(key as CFTypeRef);
        CFRelease(array as CFTypeRef);

        // 6. Nested dictionary
        let key = cfstring(c"Metadata");
        let nested = CFDictionaryCreateMutable(
            kCFAllocatorDefault,
            0,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );
        let nk = cfstring(c"Platform");
        let nv = cfstring(c"macOS");
        CFDictionarySetValue(nested, nk as *const c_void, nv as *const c_void);
        CFRelease(nk as CFTypeRef);
        CFRelease(nv as CFTypeRef);

        let nk = cfstring(c"FrameworkCount");
        let count: i32 = 200;
        let nv = CFNumberCreate(
            kCFAllocatorDefault,
            kCFNumberSInt32Type,
            &count as *const i32 as *const c_void,
        );
        CFDictionarySetValue(nested, nk as *const c_void, nv as *const c_void);
        CFRelease(nk as CFTypeRef);
        CFRelease(nv as CFTypeRef);

        CFDictionarySetValue(dict, key as *const c_void, nested as *const c_void);
        CFRelease(key as CFTypeRef);
        CFRelease(nested as CFTypeRef);

        // 7. Serialize to XML plist
        let xml_data = CFPropertyListCreateXMLData(kCFAllocatorDefault, dict as CFPropertyListRef);
        assert!(!xml_data.is_null(), "Failed to create XML data");

        let bytes = CFDataGetBytePtr(xml_data);
        let len = CFDataGetLength(xml_data) as usize;
        let xml = std::str::from_utf8(std::slice::from_raw_parts(bytes, len))
            .expect("Invalid UTF-8 in plist output");
        println!("{xml}");

        CFRelease(xml_data as CFTypeRef);
        CFRelease(dict as CFTypeRef);
    }

    println!("Done.");
}
