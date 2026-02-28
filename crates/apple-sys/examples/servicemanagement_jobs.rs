//! Query ServiceManagement for launch daemon/agent jobs.
//!
//! Uses SMCopyAllJobDictionaries to list registered
//! launch daemons and agents.

use apple_sys::CoreFoundation::{
    CFArrayGetCount, CFArrayGetValueAtIndex, CFDictionaryGetValue, CFDictionaryRef,
    CFNumberGetValue, CFNumberRef, CFRelease, CFStringCreateWithCString, CFStringEncoding,
    CFStringGetCString, CFStringGetLength, CFStringGetMaximumSizeForEncoding, CFStringRef,
};
use apple_sys::ServiceManagement::*;

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
        println!("=== ServiceManagement Jobs ===\n");

        // kSMDomainSystemLaunchd and kSMDomainUserLaunchd
        let system_domain = CFStringCreateWithCString(
            std::ptr::null(),
            c"kSMDomainSystemLaunchd".as_ptr(),
            UTF8_ENCODING,
        );
        let user_domain = CFStringCreateWithCString(
            std::ptr::null(),
            c"kSMDomainUserLaunchd".as_ptr(),
            UTF8_ENCODING,
        );

        for (domain, label) in [
            (user_domain, "User LaunchAgents"),
            (system_domain, "System LaunchDaemons"),
        ] {
            let jobs = SMCopyAllJobDictionaries(domain);
            if jobs.is_null() {
                println!("{}: (unable to query)", label);
                continue;
            }

            let count = CFArrayGetCount(jobs);
            println!("{}: {} jobs", label, count);

            let label_key =
                CFStringCreateWithCString(std::ptr::null(), c"Label".as_ptr(), UTF8_ENCODING);
            let pid_key =
                CFStringCreateWithCString(std::ptr::null(), c"PID".as_ptr(), UTF8_ENCODING);

            let show = if count > 10 { 10 } else { count };
            for i in 0..show {
                let dict = CFArrayGetValueAtIndex(jobs, i) as CFDictionaryRef;
                let name = CFDictionaryGetValue(dict, label_key as _) as CFStringRef;
                let pid_ref = CFDictionaryGetValue(dict, pid_key as _) as CFNumberRef;

                let name_str = cfstring_to_string(name);
                let mut pid: i64 = 0;
                let has_pid = if !pid_ref.is_null() {
                    CFNumberGetValue(pid_ref, 4, &mut pid as *mut i64 as _) != 0
                } else {
                    false
                };

                if has_pid && pid > 0 {
                    println!("  {} (pid={})", name_str, pid);
                } else {
                    println!("  {} (not running)", name_str);
                }
            }
            if count > 10 {
                println!("  ... and {} more", count - 10);
            }
            println!();

            CFRelease(label_key as _);
            CFRelease(pid_key as _);
            CFRelease(jobs as _);
        }

        CFRelease(system_domain as _);
        CFRelease(user_domain as _);
    }

    println!("Done.");
}
