//! Explore IOBluetoothUI framework C API.
//!
//! Calls IOBluetoothGetPairingController and IOBluetoothGetDeviceSelectorController
//! to verify the generated bindings link correctly.

use apple_sys::IOBluetoothUI::*;

fn main() {
    println!("=== IOBluetoothUI ===\n");

    unsafe {
        let pairing = IOBluetoothGetPairingController();
        println!(
            "IOBluetoothGetPairingController: {}",
            if pairing.is_null() { "null" } else { "valid" }
        );

        let selector = IOBluetoothGetDeviceSelectorController();
        println!(
            "IOBluetoothGetDeviceSelectorController: {}",
            if selector.is_null() { "null" } else { "valid" }
        );
    }

    println!("\nDone.");
}
