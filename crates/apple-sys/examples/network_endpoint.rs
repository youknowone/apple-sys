//! Create and inspect Network framework endpoints.
//!
//! Demonstrates the nw_endpoint API: creating host endpoints
//! and querying their properties.

use apple_sys::Network::*;
use std::ffi::{CStr, CString};

fn main() {
    unsafe {
        println!("=== Network Endpoint Demo ===\n");

        // 1. Create a host endpoint
        let host = CString::new("www.apple.com").unwrap();
        let port = CString::new("443").unwrap();
        let endpoint = nw_endpoint_create_host(host.as_ptr(), port.as_ptr());

        let hostname = nw_endpoint_get_hostname(endpoint.clone());
        let port_num = nw_endpoint_get_port(endpoint.clone());
        println!(
            "Endpoint 1: {}:{}",
            CStr::from_ptr(hostname).to_string_lossy(),
            port_num
        );

        // 2. Create another endpoint
        let host2 = CString::new("localhost").unwrap();
        let port2 = CString::new("8080").unwrap();
        let endpoint2 = nw_endpoint_create_host(host2.as_ptr(), port2.as_ptr());

        let hostname2 = nw_endpoint_get_hostname(endpoint2.clone());
        let port_num2 = nw_endpoint_get_port(endpoint2.clone());
        println!(
            "Endpoint 2: {}:{}",
            CStr::from_ptr(hostname2).to_string_lossy(),
            port_num2
        );

        // 3. Create more endpoints to demonstrate various hosts
        let hosts = [
            ("dns.google", "53"),
            ("github.com", "22"),
            ("0.0.0.0", "80"),
        ];
        println!();
        for (h, p) in &hosts {
            let host_c = CString::new(*h).unwrap();
            let port_c = CString::new(*p).unwrap();
            let ep = nw_endpoint_create_host(host_c.as_ptr(), port_c.as_ptr());
            let name = nw_endpoint_get_hostname(ep.clone());
            let port_val = nw_endpoint_get_port(ep.clone());
            println!(
                "Endpoint: {}:{}",
                CStr::from_ptr(name).to_string_lossy(),
                port_val
            );
            nw_release(ep.0 as *mut _);
        }

        // 4. Create a Bonjour service descriptor
        let service_name = CString::new("My Service").unwrap();
        let service_type = CString::new("_http._tcp").unwrap();
        let domain = CString::new("local.").unwrap();
        let descriptor = nw_advertise_descriptor_create_bonjour_service(
            service_name.as_ptr(),
            service_type.as_ptr(),
            domain.as_ptr(),
        );
        println!("\nBonjour descriptor created for _http._tcp");
        nw_release(descriptor.0 as *mut _);

        // Cleanup
        nw_release(endpoint.0 as *mut _);
        nw_release(endpoint2.0 as *mut _);
    }

    println!("\nDone.");
}
