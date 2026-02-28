//! List supported video encoder and decoder types.
//!
//! Demonstrates VideoToolbox's codec enumeration APIs.

#![allow(non_upper_case_globals)]

use apple_sys::CoreFoundation::{
    CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef, CFDictionaryGetValueIfPresent,
    CFDictionaryRef, CFNumberGetValue, CFNumberRef, CFRelease, CFStringCreateWithCString,
    CFStringEncoding, CFStringGetCString, CFStringGetLength, CFStringGetMaximumSizeForEncoding,
    CFStringRef, CFTypeRef, kCFAllocatorDefault,
};
use apple_sys::CoreMedia::*;
use apple_sys::VideoToolbox::*;
use std::ptr;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;

unsafe fn cfstring_to_string(cf: CFStringRef) -> String {
    if cf.is_null() {
        return String::new();
    }
    let len = unsafe { CFStringGetLength(cf) };
    let max_size = unsafe { CFStringGetMaximumSizeForEncoding(len, kCFStringEncodingUTF8) } + 1;
    let mut buf = vec![0u8; max_size as usize];
    if unsafe { CFStringGetCString(cf, buf.as_mut_ptr() as _, max_size, kCFStringEncodingUTF8) }
        != 0
    {
        let cstr = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as _) };
        cstr.to_string_lossy().to_string()
    } else {
        format!("<CFString len={}>", len)
    }
}

unsafe fn fourcc_to_string(code: CMVideoCodecType) -> String {
    let bytes = code.to_be_bytes();
    if bytes.iter().all(|b| b.is_ascii_graphic() || *b == b' ') {
        format!("'{}'", std::str::from_utf8(&bytes).unwrap_or("????"))
    } else {
        format!("0x{:08X}", code)
    }
}

fn main() {
    unsafe {
        println!("=== VideoToolbox Codec Enumeration ===\n");

        // 1. Copy supported encoder list
        let mut encoders: CFArrayRef = ptr::null();
        let status = VTCopySupportedPropertyDictionaryForEncoder(
            1920,
            1080,
            0x61766331, // 'avc1' = H.264
            ptr::null(),
            ptr::null_mut(),
            &mut encoders as *mut _ as *mut _,
        );

        if status == 0 {
            println!("H.264 encoder properties available");
        } else {
            println!("H.264 encoder query returned: {}", status);
        }

        // 2. List video encoder types
        let mut encoder_list: CFArrayRef = ptr::null();
        let status = VTCopyVideoEncoderList(ptr::null(), &mut encoder_list);
        if status == 0 && !encoder_list.is_null() {
            let count = CFArrayGetCount(encoder_list);
            println!("\nVideo Encoders ({} available):", count);

            let name_key = CFStringCreateWithCString(
                kCFAllocatorDefault,
                c"EncoderName".as_ptr(),
                kCFStringEncodingUTF8,
            );
            let codec_key = CFStringCreateWithCString(
                kCFAllocatorDefault,
                c"CodecType".as_ptr(),
                kCFStringEncodingUTF8,
            );

            for i in 0..count {
                let dict = CFArrayGetValueAtIndex(encoder_list, i) as CFDictionaryRef;
                if dict.is_null() {
                    continue;
                }

                // Get encoder name
                let mut name_val: *const std::os::raw::c_void = ptr::null();
                let name = if CFDictionaryGetValueIfPresent(
                    dict,
                    name_key as *const _,
                    &mut name_val,
                ) != 0
                {
                    cfstring_to_string(name_val as CFStringRef)
                } else {
                    "<unknown>".to_string()
                };

                // Get codec type
                let mut codec_val: *const std::os::raw::c_void = ptr::null();
                let codec =
                    if CFDictionaryGetValueIfPresent(dict, codec_key as *const _, &mut codec_val)
                        != 0
                    {
                        let mut code: u32 = 0;
                        CFNumberGetValue(
                            codec_val as CFNumberRef,
                            3, // kCFNumberSInt32Type
                            &mut code as *mut _ as *mut _,
                        );
                        fourcc_to_string(code)
                    } else {
                        "?".to_string()
                    };

                println!("  {} [{}]", name, codec);
            }

            CFRelease(name_key as CFTypeRef);
            CFRelease(codec_key as CFTypeRef);
            CFRelease(encoder_list as CFTypeRef);
        }
    }

    println!("\nDone.");
}
