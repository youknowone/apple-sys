//! Explore GSS-API credential management.
//!
//! Demonstrates gss_import_name to create a GSS name,
//! buffer set management, and GSSNameCreateDisplayString
//! for Apple's high-level GSS API.

use apple_sys::CoreFoundation::{
    CFRelease, CFStringEncoding, CFStringGetCString, CFStringGetLength,
    CFStringGetMaximumSizeForEncoding, CFStringRef,
};
use apple_sys::GSS::*;
use std::ptr;

const GSS_S_COMPLETE: u32 = 0;
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
        println!("=== GSS-API Credentials ===\n");

        // Import a name using GSS_C_NT_USER_NAME
        let user_name = b"testuser@EXAMPLE.COM";
        let mut name_buf = gss_buffer_desc_struct {
            length: user_name.len(),
            value: user_name.as_ptr() as *mut _,
        };

        let mut minor: OM_uint32 = 0;
        let mut gss_name: gss_name_t = ptr::null_mut();
        let major = gss_import_name(
            &mut minor,
            &mut name_buf,
            &raw const __gss_c_nt_user_name_oid_desc as gss_const_OID,
            &mut gss_name,
        );
        println!("gss_import_name (user):");
        println!("  Major: 0x{:08x}, Minor: 0x{:08x}", major, minor);

        if major == GSS_S_COMPLETE && !gss_name.is_null() {
            println!("  Name imported successfully");

            // Display the name using Apple's high-level API
            let display = GSSNameCreateDisplayString(gss_name);
            if !display.is_null() {
                println!("  Display string: {}", cfstring_to_string(display));
                CFRelease(display as _);
            }

            // Export the name to a buffer
            let mut export_buf = gss_buffer_desc_struct {
                length: 0,
                value: ptr::null_mut(),
            };
            let major = gss_export_name(&mut minor, gss_name, &mut export_buf);
            if major == GSS_S_COMPLETE {
                println!("  Exported name: {} bytes", export_buf.length);
                gss_release_buffer(&mut minor, &mut export_buf);
            } else {
                println!(
                    "  Export failed (major=0x{:08x}) - may need mechanism",
                    major
                );
            }

            gss_release_name(&mut minor, &mut gss_name);
        } else {
            println!("  Failed to import name");
        }

        // Import a hostbased service name
        let svc_name = b"HTTP@www.example.com";
        let mut svc_buf = gss_buffer_desc_struct {
            length: svc_name.len(),
            value: svc_name.as_ptr() as *mut _,
        };

        let mut gss_svc: gss_name_t = ptr::null_mut();
        let major = gss_import_name(
            &mut minor,
            &mut svc_buf,
            &raw const __gss_c_nt_hostbased_service_x_oid_desc as gss_const_OID,
            &mut gss_svc,
        );
        println!("\ngss_import_name (hostbased service):");
        println!("  Major: 0x{:08x}, Minor: 0x{:08x}", major, minor);

        if major == GSS_S_COMPLETE && !gss_svc.is_null() {
            let display = GSSNameCreateDisplayString(gss_svc);
            if !display.is_null() {
                println!("  Display string: {}", cfstring_to_string(display));
                CFRelease(display as _);
            }

            // Compare the two names (should be different)
            if !gss_name.is_null() {
                let mut name_equal: i32 = 0;
                let major = gss_compare_name(&mut minor, gss_name, gss_svc, &mut name_equal);
                if major == GSS_S_COMPLETE {
                    println!("  Names equal: {}", name_equal != 0);
                }
            }

            gss_release_name(&mut minor, &mut gss_svc);
        }

        // Create and release an empty buffer set
        let mut buffer_set: gss_buffer_set_t = ptr::null_mut();
        let major = gss_create_empty_buffer_set(&mut minor, &mut buffer_set);
        println!("\ngss_create_empty_buffer_set:");
        println!("  Major: 0x{:08x}", major);
        if major == GSS_S_COMPLETE && !buffer_set.is_null() {
            println!("  Buffer set created at {:?}", buffer_set);
            gss_release_buffer_set(&mut minor, &mut buffer_set);
            println!("  Buffer set released");
        }

        // Check OID equality
        let eq = gss_oid_equal(
            &raw const __gss_c_nt_user_name_oid_desc as gss_const_OID,
            &raw const __gss_c_nt_user_name_oid_desc as gss_const_OID,
        );
        println!("\nOID equality (user_name == user_name): {}", eq != 0);

        let neq = gss_oid_equal(
            &raw const __gss_c_nt_user_name_oid_desc as gss_const_OID,
            &raw const __gss_c_nt_hostbased_service_x_oid_desc as gss_const_OID,
        );
        println!("OID equality (user_name == hostbased): {}", neq != 0);
    }

    println!("\nDone.");
}
