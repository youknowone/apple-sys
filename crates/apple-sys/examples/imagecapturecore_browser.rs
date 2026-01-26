//! Browse for image capture devices using ImageCaptureCore.
//!
//! Creates ICDeviceBrowser and checks its browsing state, device type mask,
//! and exercises start/stop lifecycle.

use apple_sys::ImageCaptureCore::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ImageCaptureCore Device Browser ===\n");

        // ICDeviceBrowser
        println!("--- ICDeviceBrowser ---");
        let browser = ICDeviceBrowser::alloc();
        let browser_ptr = INSObject::init(&browser);
        if !browser_ptr.is_null() {
            let browser = ICDeviceBrowser(browser_ptr);
            println!("  Browser: {}", nsobj_to_string(browser.0));

            let browsing = IICDeviceBrowser::isBrowsing(&browser);
            println!("  Is browsing: {}", browsing.0);

            // Check browsed device types mask
            let mask = IICDeviceBrowser::browsedDeviceTypeMask(&browser);
            println!("  Browsed device type mask: 0x{mask:x}");

            // Decode mask
            let types = [
                (0x00000001, "Camera"),
                (0x00000002, "Scanner"),
                (0x00000004, "LocalDevice"),
                (0x00000008, "SharedDevice"),
                (0x00000010, "BonjourDevice"),
                (0x00000020, "BluetoothDevice"),
            ];
            for (bit, name) in &types {
                if mask & bit != 0 {
                    println!("    - {name}");
                }
            }

            // Query devices (before starting browse, should be empty)
            let devices = IICDeviceBrowser::devices(&browser);
            if !devices.0.is_null() {
                let count = INSArray::<id>::count(&devices);
                println!("  Devices found (before browse): {count}");
            }

            // Start a brief browse
            IICDeviceBrowser::start(&browser);
            let browsing_after = IICDeviceBrowser::isBrowsing(&browser);
            println!("\n  After start:");
            println!("  Is browsing: {}", browsing_after.0);

            // Stop immediately
            IICDeviceBrowser::stop(&browser);
            let browsing_stopped = IICDeviceBrowser::isBrowsing(&browser);
            println!("  After stop:  {}", browsing_stopped.0);
        } else {
            println!("  Failed to create ICDeviceBrowser");
        }
    }

    println!("\nDone.");
}
