//! Initialize a Kerberos context and query default realm and ccache.
//!
//! Demonstrates krb5_init_context, krb5_get_default_realm,
//! krb5_cc_default, and the CCAPI cc_initialize function.

use apple_sys::Kerberos::*;
use std::ffi::CStr;
use std::ptr;

fn main() {
    unsafe {
        println!("=== Kerberos Framework ===\n");

        // Initialize a krb5 context
        let mut ctx: krb5_context = ptr::null_mut();
        let err = krb5_init_context(&mut ctx);
        println!(
            "krb5_init_context: {}",
            if err == 0 { "OK" } else { "FAILED" }
        );

        if err != 0 {
            // Try to get error message for the code
            let msg = error_message(err as errcode_t);
            if !msg.is_null() {
                let s = CStr::from_ptr(msg).to_string_lossy();
                eprintln!("  Error: {} (code={})", s, err);
            }
            println!("\nDone.");
            return;
        }

        // Get default realm
        let mut realm: *mut std::os::raw::c_char = ptr::null_mut();
        let err = krb5_get_default_realm(ctx, &mut realm);
        if err == 0 && !realm.is_null() {
            let realm_str = CStr::from_ptr(realm).to_string_lossy();
            println!("Default realm: {}", realm_str);
            krb5_free_default_realm(ctx, realm);
        } else {
            let msg = krb5_get_error_message(ctx, err);
            if !msg.is_null() {
                let s = CStr::from_ptr(msg).to_string_lossy();
                println!("Default realm: unavailable ({})", s);
                krb5_free_error_message(ctx, msg);
            } else {
                println!("Default realm: unavailable (err={})", err);
            }
        }

        // Get default credential cache name
        let cc_name = krb5_cc_default_name(ctx);
        if !cc_name.is_null() {
            let name_str = CStr::from_ptr(cc_name).to_string_lossy();
            println!("Default ccache name: {}", name_str);
        }

        // Open default credential cache
        let mut ccache: krb5_ccache = ptr::null_mut();
        let err = krb5_cc_default(ctx, &mut ccache);
        if err == 0 && !ccache.is_null() {
            let cc_type = krb5_cc_get_type(ctx, ccache);
            let cc_name = krb5_cc_get_name(ctx, ccache);

            let type_str = if !cc_type.is_null() {
                CStr::from_ptr(cc_type).to_string_lossy().into_owned()
            } else {
                "(unknown)".to_string()
            };
            let name_str = if !cc_name.is_null() {
                CStr::from_ptr(cc_name).to_string_lossy().into_owned()
            } else {
                "(unknown)".to_string()
            };

            println!("\nDefault credential cache:");
            println!("  Type: {}", type_str);
            println!("  Name: {}", name_str);

            krb5_cc_close(ctx, ccache);
        } else {
            println!("\nDefault credential cache: unavailable (err={})", err);
        }

        // Use CCAPI to get version info
        let mut cc_ctx: cc_context_t = ptr::null_mut();
        let mut supported_version: cc_int32 = 0;
        let mut vendor: *const std::os::raw::c_char = ptr::null();
        let cc_err = cc_initialize(&mut cc_ctx, 2, &mut supported_version, &mut vendor);
        println!("\nCCAPI cc_initialize:");
        println!("  Return: {}", cc_err);
        if cc_err == 0 {
            println!("  Supported version: {}", supported_version);
            if !vendor.is_null() {
                let vendor_str = CStr::from_ptr(vendor).to_string_lossy();
                println!("  Vendor: {}", vendor_str);
            }
        }

        // Use error_message for a few common error codes
        println!("\nSample error messages:");
        for code in [0i64, -1765328384, -1765328383, -1765328382] {
            let msg = error_message(code as errcode_t);
            if !msg.is_null() {
                let s = CStr::from_ptr(msg).to_string_lossy();
                println!("  {:>12}: {}", code, s);
            }
        }

        krb5_free_context(ctx);
        println!("\nContext freed.");
    }

    println!("\nDone.");
}
