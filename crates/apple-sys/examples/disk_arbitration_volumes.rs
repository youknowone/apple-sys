//! Query mounted volume information using DiskArbitration.
//!
//! Shows the root volume's disk description and probes common BSD disk names.

#![allow(non_upper_case_globals)]

use apple_sys::CoreFoundation::*;
use apple_sys::DiskArbitration::*;
use std::ffi::CStr;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
const kCFURLPOSIXPathStyle: CFURLPathStyle = 0;
const kCFNumberSInt64Type: CFNumberType = 4;

unsafe fn cfstring(s: &std::ffi::CStr) -> CFStringRef {
    unsafe { CFStringCreateWithCString(kCFAllocatorDefault, s.as_ptr(), kCFStringEncodingUTF8) }
}

unsafe fn cfstring_to_string(cfstr: CFStringRef) -> String {
    if cfstr.is_null() {
        return String::new();
    }
    let mut buf = [0i8; 512];
    let ok = unsafe {
        CFStringGetCString(
            cfstr,
            buf.as_mut_ptr(),
            buf.len() as CFIndex,
            kCFStringEncodingUTF8,
        )
    };
    if ok != 0 {
        unsafe { CStr::from_ptr(buf.as_ptr()) }
            .to_string_lossy()
            .into_owned()
    } else {
        "<CFString>".to_string()
    }
}

unsafe fn print_disk_description(desc: CFDictionaryRef) {
    // Print well-known description keys
    let keys = [
        c"DAVolumeName",
        c"DAMediaName",
        c"DAVolumeKind",
        c"DAMediaContent",
        c"DADeviceModel",
    ];
    for key_cstr in &keys {
        let key = unsafe { cfstring(key_cstr) };
        let val = unsafe { CFDictionaryGetValue(desc, key as *const std::os::raw::c_void) };
        if !val.is_null() {
            let val_desc = unsafe { CFCopyDescription(val as CFTypeRef) };
            let val_str = unsafe { cfstring_to_string(val_desc) };
            unsafe { CFRelease(val_desc as CFTypeRef) };
            let label = key_cstr
                .to_str()
                .unwrap()
                .strip_prefix("DA")
                .unwrap_or(key_cstr.to_str().unwrap());
            println!("    {label}: {val_str}");
        }
        unsafe { CFRelease(key as CFTypeRef) };
    }

    // Media size (numeric)
    let size_key = unsafe { cfstring(c"DAMediaSize") };
    let size_val = unsafe { CFDictionaryGetValue(desc, size_key as *const std::os::raw::c_void) };
    if !size_val.is_null() {
        let mut size: i64 = 0;
        if unsafe {
            CFNumberGetValue(
                size_val as CFNumberRef,
                kCFNumberSInt64Type,
                &mut size as *mut i64 as *mut std::os::raw::c_void,
            )
        } != 0
        {
            let gb = size as f64 / (1024.0 * 1024.0 * 1024.0);
            println!("    MediaSize: {gb:.2} GB ({size} bytes)");
        }
    }
    unsafe { CFRelease(size_key as CFTypeRef) };
}

fn main() {
    unsafe {
        println!("=== DiskArbitration Volume Info Demo ===\n");

        let session = DASessionCreate(kCFAllocatorDefault);
        assert!(!session.is_null(), "Failed to create DA session");

        // Root volume via path
        let root_str = cfstring(c"/");
        let root_url = CFURLCreateWithFileSystemPath(
            kCFAllocatorDefault,
            root_str,
            kCFURLPOSIXPathStyle,
            1, // isDirectory
        );
        CFRelease(root_str as CFTypeRef);

        let root_disk = DADiskCreateFromVolumePath(kCFAllocatorDefault, session, root_url);
        CFRelease(root_url as CFTypeRef);

        if !root_disk.is_null() {
            let bsd = DADiskGetBSDName(root_disk);
            if !bsd.is_null() {
                println!("Root volume (/):");
                println!("  BSD name: {}", CStr::from_ptr(bsd).to_string_lossy());

                let desc = DADiskCopyDescription(root_disk);
                if !desc.is_null() {
                    print_disk_description(desc);
                    CFRelease(desc as CFTypeRef);
                }

                // Whole disk
                let whole = DADiskCopyWholeDisk(root_disk);
                if !whole.is_null() {
                    let whole_bsd = DADiskGetBSDName(whole);
                    if !whole_bsd.is_null() {
                        println!(
                            "\nWhole disk: {}",
                            CStr::from_ptr(whole_bsd).to_string_lossy()
                        );
                        let wdesc = DADiskCopyDescription(whole);
                        if !wdesc.is_null() {
                            print_disk_description(wdesc);
                            CFRelease(wdesc as CFTypeRef);
                        }
                    }
                    CFRelease(whole as CFTypeRef);
                }
            }
            CFRelease(root_disk as CFTypeRef);
        }

        // Probe common BSD disk names
        println!("\nProbing common partitions:");
        let names = [
            c"disk0", c"disk0s1", c"disk0s2", c"disk0s3", c"disk1", c"disk1s1", c"disk1s2",
            c"disk1s3", c"disk2", c"disk3",
        ];
        for name in &names {
            let disk = DADiskCreateFromBSDName(kCFAllocatorDefault, session, name.as_ptr());
            if disk.is_null() {
                continue;
            }
            let desc = DADiskCopyDescription(disk);
            if !desc.is_null() {
                println!("\n  {}:", name.to_str().unwrap());
                print_disk_description(desc);
                CFRelease(desc as CFTypeRef);
            }
            CFRelease(disk as CFTypeRef);
        }

        CFRelease(session as CFTypeRef);
    }

    println!("\nDone.");
}
