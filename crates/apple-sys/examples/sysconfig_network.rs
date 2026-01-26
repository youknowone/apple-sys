//! List network interfaces using SystemConfiguration framework.
//!
//! Queries all network interfaces and prints their BSD name, display name,
//! hardware address, and interface type.

use apple_sys::SystemConfiguration::*;

fn cfstring_to_string(cf: CFStringRef) -> Option<String> {
    if cf.is_null() {
        return None;
    }
    unsafe {
        let mut buf = [0u8; 512];
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

fn main() {
    unsafe {
        let interfaces = SCNetworkInterfaceCopyAll();
        if interfaces.is_null() {
            eprintln!("Failed to copy network interfaces");
            return;
        }

        let count = CFArrayGetCount(interfaces);
        println!("Network Interfaces ({count} total):");
        println!("{:-<60}", "");

        for i in 0..count {
            let iface = CFArrayGetValueAtIndex(interfaces, i) as SCNetworkInterfaceRef;
            if iface.is_null() {
                continue;
            }

            let bsd_name = cfstring_to_string(SCNetworkInterfaceGetBSDName(iface))
                .unwrap_or_else(|| "(none)".to_string());
            let display_name = cfstring_to_string(SCNetworkInterfaceGetLocalizedDisplayName(iface))
                .unwrap_or_else(|| "(unknown)".to_string());
            let iface_type = cfstring_to_string(SCNetworkInterfaceGetInterfaceType(iface))
                .unwrap_or_else(|| "(unknown)".to_string());
            let hw_addr = cfstring_to_string(SCNetworkInterfaceGetHardwareAddressString(iface))
                .unwrap_or_else(|| "(none)".to_string());

            println!("  {bsd_name}");
            println!("    Display Name: {display_name}");
            println!("    Type:         {iface_type}");
            println!("    HW Address:   {hw_addr}");
            println!();
        }

        CFRelease(interfaces as CFTypeRef);
    }
}
