//! Query connected external accessories using ExternalAccessory framework.
//!
//! Gets EAAccessoryManager.sharedAccessoryManager and queries
//! the list of connected accessories using the generated bindings.

use apple_sys::ExternalAccessory::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ExternalAccessory Manager ===\n");

        // EAAccessoryManager
        println!("--- EAAccessoryManager ---");
        let mgr = EAAccessoryManager::sharedAccessoryManager();
        if !mgr.0.is_null() {
            println!("  Manager: {}", nsobj_to_string(mgr.0));

            // Query connected accessories
            let accessories = IEAAccessoryManager::connectedAccessories(&mgr);
            if !accessories.0.is_null() {
                let count = INSArray::<id>::count(&accessories);
                println!("  Connected accessories: {count}");

                for i in 0..count {
                    let acc_id = INSArray::<id>::objectAtIndex_(&accessories, i);
                    let acc = EAAccessory(acc_id);
                    let name = IEAAccessory::name(&acc);
                    let manufacturer = IEAAccessory::manufacturer(&acc);
                    let model = IEAAccessory::modelNumber(&acc);
                    let serial = IEAAccessory::serialNumber(&acc);
                    let fw_ver = IEAAccessory::firmwareRevision(&acc);
                    let hw_ver = IEAAccessory::hardwareRevision(&acc);
                    let connected = IEAAccessory::isConnected(&acc);
                    let conn_id = IEAAccessory::connectionID(&acc);
                    let dock_type = IEAAccessory::dockType(&acc);

                    println!("\n  Accessory #{i}:");
                    println!("    Name:         {}", nsstring_to_string(name));
                    println!("    Manufacturer: {}", nsstring_to_string(manufacturer));
                    println!("    Model:        {}", nsstring_to_string(model));
                    println!("    Serial:       {}", nsstring_to_string(serial));
                    println!("    FW revision:  {}", nsstring_to_string(fw_ver));
                    println!("    HW revision:  {}", nsstring_to_string(hw_ver));
                    println!("    Connected:    {}", connected.0);
                    println!("    Connection ID: {conn_id}");
                    println!("    Dock type:    {}", nsstring_to_string(dock_type));

                    // Protocol strings
                    let protocols = IEAAccessory::protocolStrings(&acc);
                    if !protocols.0.is_null() {
                        let pcount = INSArray::<id>::count(&protocols);
                        println!("    Protocols ({pcount}):");
                        for j in 0..pcount {
                            let proto = INSArray::<id>::objectAtIndex_(&protocols, j);
                            println!("      - {}", nsstring_to_string(NSString(proto)));
                        }
                    }
                }

                if count == 0 {
                    println!("  (no accessories connected)");
                }
            } else {
                println!("  connectedAccessories returned nil");
            }
        } else {
            println!("  sharedAccessoryManager returned nil");
        }

        // EAWiFiUnconfiguredAccessoryBrowser
        println!("\n--- EAWiFiUnconfiguredAccessoryBrowser ---");
        let browser = EAWiFiUnconfiguredAccessoryBrowser::alloc();
        let browser_ptr = IEAWiFiUnconfiguredAccessoryBrowser::initWithDelegate_queue_(
            &browser,
            std::ptr::null_mut(),
            NSObject(std::ptr::null_mut()),
        );
        if !browser_ptr.is_null() {
            let browser = EAWiFiUnconfiguredAccessoryBrowser(browser_ptr);
            println!("  EAWiFiUnconfiguredAccessoryBrowser created.");
            let accessories =
                IEAWiFiUnconfiguredAccessoryBrowser::unconfiguredAccessories(&browser);
            if !accessories.0.is_null() {
                println!(
                    "  Unconfigured accessories: {}",
                    nsobj_to_string(accessories.0)
                );
            }
        } else {
            println!("  (failed to create EAWiFiUnconfiguredAccessoryBrowser)");
        }
    }

    println!("\nDone.");
}
