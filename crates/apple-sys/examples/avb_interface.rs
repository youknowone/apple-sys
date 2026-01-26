//! Inspect AudioVideoBridging network interfaces.
//!
//! Queries AVBInterface for available AVB-capable Ethernet interfaces
//! and displays their names and properties.

use apple_sys::AudioVideoBridging::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AudioVideoBridging Interfaces ===\n");

        let iface_names = AVBInterface::supportedInterfaces();
        if iface_names.0.is_null() {
            println!("No supported Ethernet interfaces found.");
            return;
        }

        let count = INSArray::<id>::count(&iface_names);
        println!("Supported AVB Ethernet interfaces: {count}");

        for i in 0..count {
            let name: id = INSArray::<id>::objectAtIndex_(&iface_names, i);
            println!("\n  Interface #{}: {}", i + 1, nsobj_to_string(name));

            // Create an AVBEthernetInterface for this name
            let iface = AVBEthernetInterface::alloc();
            let iface_ptr = IAVBInterface::initWithInterfaceName_(&iface, NSString(name));
            if !iface_ptr.is_null() {
                let iface = AVBEthernetInterface(iface_ptr);
                let iface_name = IAVBInterface::interfaceName(&iface);
                println!("    Name: {}", nsstring_to_string(iface_name));
            }
        }
    }

    println!("\nDone.");
}
