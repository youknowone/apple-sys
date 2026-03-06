//! List paired Bluetooth devices via IOBluetooth.
//!
//! Uses IOBluetoothDevice to enumerate paired devices
//! and show their names and addresses.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::IOBluetooth::*;
use apple_sys::objc::id;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== IOBluetooth Devices ===\n");

        // Get paired devices
        let paired = IOBluetoothDevice::pairedDevices();

        if !paired.0.is_null() {
            let count = INSArray::<id>::count(&paired);
            println!("Paired Bluetooth devices: {}\n", count);

            for i in 0..count {
                let dev_id = INSArray::<id>::objectAtIndex_(&paired, i);
                let device = IOBluetoothDevice(dev_id);
                let name = IIOBluetoothDevice::getName(&device);
                let addr = IIOBluetoothDevice::getNameOrAddress(&device);
                let class = IIOBluetoothDevice::getClassOfDevice(&device);
                let major = IIOBluetoothDevice::getDeviceClassMajor(&device);
                let minor = IIOBluetoothDevice::getDeviceClassMinor(&device);
                let connected = IIOBluetoothDevice::isConnected(&device);

                let major_name = match major {
                    1 => "Computer",
                    2 => "Phone",
                    3 => "LAN/Network",
                    4 => "Audio/Video",
                    5 => "Peripheral",
                    6 => "Imaging",
                    7 => "Wearable",
                    8 => "Toy",
                    9 => "Health",
                    _ => "Other",
                };

                println!(
                    "  {:2}. {} ({})",
                    i + 1,
                    nsstring_to_string(name),
                    nsstring_to_string(addr)
                );
                println!(
                    "      Class: 0x{:06X} ({}, minor={}), Connected: {}",
                    class, major_name, minor, connected.0
                );
            }
        } else {
            println!("No paired devices found (or Bluetooth unavailable).");
        }

        // Host controller info
        let hc_ptr = IOBluetoothHostController::defaultController();
        let hc = IOBluetoothHostController(hc_ptr);
        if !hc.0.is_null() {
            let power = IIOBluetoothHostController::powerState(&hc);
            let addr = IIOBluetoothHostController::addressAsString(&hc);
            let name = IIOBluetoothHostController::nameAsString(&hc);
            println!("\nHost controller:");
            println!("  Name:    {}", nsstring_to_string(name));
            println!("  Address: {}", nsstring_to_string(addr));
            println!("  Power:   {}", if power == 1 { "On" } else { "Off" });
        }
    }

    println!("\nDone.");
}
