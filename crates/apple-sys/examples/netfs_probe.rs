//! Probe network filesystem protocols.
//!
//! Demonstrates NetFS's URL probing and remount APIs to discover
//! which file-sharing protocols are available for a given host.

#![allow(non_upper_case_globals)]

use apple_sys::NetFS::*;
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

fn main() {
    unsafe {
        println!("=== NetFS Protocol Probe ===\n");

        // Probe localhost for supported file-sharing protocols
        let hosts = ["localhost", "127.0.0.1"];
        for host in &hosts {
            let hostname = CFStringCreateWithCString(
                kCFAllocatorDefault,
                std::ffi::CString::new(*host).unwrap().as_ptr(),
                kCFStringEncodingUTF8,
            );
            let result = NetFSMountURLProbe(hostname);
            if !result.is_null() {
                println!("{}: protocol = {}", host, cfstring_to_string(result));
                CFRelease(result as CFTypeRef);
            } else {
                println!("{}: no protocol detected (null)", host);
            }
            CFRelease(hostname as CFTypeRef);
        }

        // Test URL string conversion helper
        let test_str = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"smb://server/share".as_ptr(),
            kCFStringEncodingUTF8,
        );
        let cstr = NetFSCFStringtoCString(test_str);
        if !cstr.is_null() {
            println!(
                "\nNetFSCFStringtoCString: {}",
                std::ffi::CStr::from_ptr(cstr).to_string_lossy()
            );
            libc::free(cstr as _);
        }
        CFRelease(test_str as CFTypeRef);

        // Try to get remount URL for root volume
        let root_url = CFURLCreateWithString(
            kCFAllocatorDefault,
            CFStringCreateWithCString(
                kCFAllocatorDefault,
                c"file:///".as_ptr(),
                kCFStringEncodingUTF8,
            ),
            ptr::null(),
        );
        let remount_url = NetFSCopyURLForRemountingVolume(root_url);
        if !remount_url.is_null() {
            let url_str = CFURLGetString(remount_url);
            println!("Root volume remount URL: {}", cfstring_to_string(url_str));
            CFRelease(remount_url as CFTypeRef);
        } else {
            println!("\nRoot volume has no remote remount URL (local disk)");
        }
        CFRelease(root_url as CFTypeRef);
    }

    println!("\nDone.");
}
