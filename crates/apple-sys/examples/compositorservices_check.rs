//! Explore CompositorServices framework types and error domain.
//!
//! CompositorServices is primarily used for visionOS spatial rendering.
//! On macOS, we can inspect the error domain and type definitions.

#![allow(non_upper_case_globals)]

use apple_sys::CompositorServices::*;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;

unsafe fn cfstring_to_string(cf: CFStringRef) -> String {
    if cf.is_null() {
        return "(null)".to_string();
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

fn main() {
    unsafe {
        println!("=== CompositorServices ===\n");

        // 1. Access the configuration error domain
        let domain = cp_layer_renderer_configuration_error_domain;
        println!("Error domain: \"{}\"", cfstring_to_string(domain));

        // 2. Create a CFError using the CompositorServices error domain
        let desc_key = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"kCFErrorLocalizedDescriptionKey".as_ptr(),
            kCFStringEncodingUTF8,
        );
        let desc_val = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"Layer renderer configuration is invalid".as_ptr(),
            kCFStringEncodingUTF8,
        );

        let mut info_keys = [desc_key as *const std::os::raw::c_void];
        let mut info_values = [desc_val as *const std::os::raw::c_void];
        let user_info = CFDictionaryCreate(
            kCFAllocatorDefault,
            info_keys.as_mut_ptr(),
            info_values.as_mut_ptr(),
            1,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );

        let error = CFErrorCreate(
            kCFAllocatorDefault,
            domain,
            -1, // example error code
            user_info,
        );

        if !error.is_null() {
            let err_domain = CFErrorGetDomain(error);
            let err_code = CFErrorGetCode(error);
            let err_desc = CFErrorCopyDescription(error);

            println!("\nCFError with CompositorServices domain:");
            println!("  Domain: \"{}\"", cfstring_to_string(err_domain));
            println!("  Code:   {}", err_code);
            println!("  Description: \"{}\"", cfstring_to_string(err_desc));

            if !err_desc.is_null() {
                CFRelease(err_desc as CFTypeRef);
            }
            CFRelease(error as CFTypeRef);
        }

        CFRelease(user_info as CFTypeRef);
        CFRelease(desc_val as CFTypeRef);
        CFRelease(desc_key as CFTypeRef);

        // 3. Show type definitions and sizes
        println!("\nCompositorServices types:");
        println!("  cp_time: {} bytes", std::mem::size_of::<cp_time>());
        println!(
            "  cp_layer_renderer_configuration_error_code: {} bytes (= CFIndex)",
            std::mem::size_of::<cp_layer_renderer_configuration_error_code>()
        );
        println!(
            "  cp_axis_direction_convention: {} bytes (u8)",
            std::mem::size_of::<cp_axis_direction_convention>()
        );
        println!(
            "  cp_layer_renderer_layout: {} bytes (u32)",
            std::mem::size_of::<cp_layer_renderer_layout>()
        );
        println!(
            "  cp_drawable_state: {} bytes (u32)",
            std::mem::size_of::<cp_drawable_state>()
        );
        println!(
            "  cp_drawable_target: {} bytes (u32)",
            std::mem::size_of::<cp_drawable_target>()
        );
        println!(
            "  cp_layer_renderer_state: {} bytes (u32)",
            std::mem::size_of::<cp_layer_renderer_state>()
        );

        // 4. Demonstrate cp_time usage
        let time = cp_time {
            cp_mach_abs_time: 123456789,
        };
        println!("\ncp_time example:");
        println!("  cp_mach_abs_time: {}", time.cp_mach_abs_time);
    }

    println!("\nDone.");
}
