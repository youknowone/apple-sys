//! List ColorSync ICC profiles installed on the system.
//!
//! Demonstrates ColorSync's profile iteration and property query APIs.

#![allow(non_upper_case_globals)]

use apple_sys::ColorSync::*;
use apple_sys::CoreFoundation::*;
use std::os::raw::c_void;
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
        String::new()
    }
}

fn main() {
    unsafe {
        println!("=== ColorSync Profile Listing ===\n");

        // 1. Create built-in sRGB profile
        let srgb_name = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"kCGColorSpaceSRGB".as_ptr(),
            kCFStringEncodingUTF8,
        );
        let srgb = ColorSyncProfileCreateWithName(srgb_name);
        if !srgb.is_null() {
            println!("sRGB profile created successfully");
            let url = ColorSyncProfileGetURL(srgb, ptr::null_mut());
            if !url.is_null() {
                let url_str = CFURLGetString(url);
                println!("  URL: {}", cfstring_to_string(url_str));
            }
            CFRelease(srgb as CFTypeRef);
        } else {
            println!(
                "Could not create sRGB profile (ColorSyncProfileCreateWithName returned null)"
            );
        }
        CFRelease(srgb_name as CFTypeRef);

        // 2. Create Display P3 profile
        let p3_name = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"kCGColorSpaceDisplayP3".as_ptr(),
            kCFStringEncodingUTF8,
        );
        let p3 = ColorSyncProfileCreateWithName(p3_name);
        if !p3.is_null() {
            println!("Display P3 profile created successfully");
            let url = ColorSyncProfileGetURL(p3, ptr::null_mut());
            if !url.is_null() {
                let url_str = CFURLGetString(url);
                println!("  URL: {}", cfstring_to_string(url_str));
            }
            CFRelease(p3 as CFTypeRef);
        } else {
            println!("Display P3 profile not available");
        }
        CFRelease(p3_name as CFTypeRef);

        // 3. Iterate installed profiles
        println!("\nInstalled ICC profiles:");
        let mut seed: u32 = 0;
        let mut error: CFErrorRef = ptr::null_mut();
        ColorSyncIterateInstalledProfiles(
            Some(profile_iter_callback),
            &mut seed,
            ptr::null_mut(),
            &mut error,
        );
        if !error.is_null() {
            println!("  (iteration error)");
            CFRelease(error as CFTypeRef);
        }
    }

    println!("\nDone.");
}

static mut PROFILE_COUNT: u32 = 0;

unsafe extern "C" fn profile_iter_callback(
    profile_info: CFDictionaryRef,
    context: *mut c_void,
) -> bool {
    let _ = context;
    unsafe {
        PROFILE_COUNT += 1;
        if PROFILE_COUNT > 10 {
            if PROFILE_COUNT == 11 {
                println!("  ... (truncated)");
            }
            return PROFILE_COUNT < 100; // stop after 100 to avoid flooding
        }

        // Try to get profile name
        let name_key = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"ProfileDescription".as_ptr(),
            kCFStringEncodingUTF8,
        );
        let mut name_val: *const c_void = ptr::null();
        if CFDictionaryGetValueIfPresent(profile_info, name_key as *const c_void, &mut name_val)
            != 0
        {
            let name = cfstring_to_string(name_val as CFStringRef);
            println!("  {}", name);
        }
        CFRelease(name_key as CFTypeRef);
        true // continue iteration
    }
}
