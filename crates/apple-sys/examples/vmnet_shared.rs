//! Query vmnet shared interface list and attempt to create a network configuration.
//!
//! Demonstrates vmnet_copy_shared_interface_list and
//! vmnet_network_configuration_create. Requires root privileges
//! for full functionality.

use apple_sys::vmnet::*;
use std::ffi::CStr;

// vmnet_return_t values
const VMNET_SUCCESS: vmnet_return_t = 1000;

// vmnet operating modes
const VMNET_HOST_MODE: vmnet_mode_t = 1000;
const VMNET_SHARED_MODE: vmnet_mode_t = 1001;

fn vmnet_status_name(status: vmnet_return_t) -> &'static str {
    match status {
        1000 => "VMNET_SUCCESS",
        1001 => "VMNET_FAILURE",
        1002 => "VMNET_MEM_FAILURE",
        1003 => "VMNET_INVALID_ARGUMENT",
        1004 => "VMNET_SETUP_INCOMPLETE",
        1005 => "VMNET_INVALID_ACCESS",
        1006 => "VMNET_PACKET_TOO_BIG",
        1007 => "VMNET_BUFFER_EXHAUSTED",
        1008 => "VMNET_TOO_MANY_PACKETS",
        1009 => "VMNET_SHARING_SERVICE_BUSY",
        _ => "UNKNOWN",
    }
}

fn main() {
    unsafe {
        println!("=== vmnet ===\n");

        // 1. Print vmnet configuration key names
        println!("vmnet configuration keys:");
        let key_pairs: &[(&str, *const std::os::raw::c_char)] = &[
            ("operation_mode", vmnet_operation_mode_key),
            ("shared_interface_name", vmnet_shared_interface_name_key),
            ("mac_address", vmnet_mac_address_key),
            ("allocate_mac_address", vmnet_allocate_mac_address_key),
            ("mtu", vmnet_mtu_key),
            ("max_packet_size", vmnet_max_packet_size_key),
            ("interface_id", vmnet_interface_id_key),
            ("start_address", vmnet_start_address_key),
            ("end_address", vmnet_end_address_key),
            ("subnet_mask", vmnet_subnet_mask_key),
            ("enable_isolation", vmnet_enable_isolation_key),
        ];

        for (label, key) in key_pairs {
            if !key.is_null() {
                let s = CStr::from_ptr(*key).to_string_lossy();
                println!("  {} = \"{}\"", label, s);
            }
        }

        // 2. Copy the shared interface list
        println!("\nQuerying shared interface list...");
        let list = vmnet_copy_shared_interface_list();
        if !list.0.is_null() {
            println!(
                "  vmnet_copy_shared_interface_list returned an object: {:p}",
                list.0
            );
            // The returned object is an xpc_array; we cannot easily introspect it
            // without xpc APIs, but having a non-null result confirms vmnet is available.
        } else {
            println!("  vmnet_copy_shared_interface_list returned null.");
            println!("  (May require root privileges or specific entitlements.)");
        }

        // 3. Try to create a network configuration in shared mode
        println!("\nCreating vmnet network configuration (shared mode)...");
        let mut status: vmnet_return_t = 0;
        let config = vmnet_network_configuration_create(VMNET_SHARED_MODE, &mut status);
        println!(
            "  vmnet_network_configuration_create: {} ({})",
            status,
            vmnet_status_name(status)
        );

        if status == VMNET_SUCCESS && !config.is_null() {
            println!("  Configuration created successfully.");

            // Disable some features for a simpler config
            vmnet_network_configuration_disable_dns_proxy(config);
            println!("  Disabled DNS proxy.");

            vmnet_network_configuration_disable_router_advertisement(config);
            println!("  Disabled router advertisement.");
        } else {
            println!("  Configuration creation failed (requires root or entitlements).");
        }

        // 4. Try host mode too
        println!("\nCreating vmnet network configuration (host mode)...");
        let mut status2: vmnet_return_t = 0;
        let config2 = vmnet_network_configuration_create(VMNET_HOST_MODE, &mut status2);
        println!(
            "  vmnet_network_configuration_create: {} ({})",
            status2,
            vmnet_status_name(status2)
        );

        if status2 == VMNET_SUCCESS && !config2.is_null() {
            println!("  Host-mode configuration created successfully.");
        } else {
            println!("  Host-mode configuration failed (requires root or entitlements).");
        }

        // 5. Show struct sizes
        println!("\nvmnet struct sizes:");
        println!("  vmpktdesc: {} bytes", std::mem::size_of::<vmpktdesc>());
        println!("  iovec:     {} bytes", std::mem::size_of::<iovec>());
        println!("  in_addr:   {} bytes", std::mem::size_of::<in_addr>());
        println!("  in6_addr:  {} bytes", std::mem::size_of::<in6_addr>());
    }

    println!("\nDone.");
}
