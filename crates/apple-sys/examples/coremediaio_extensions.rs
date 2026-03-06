//! Query CoreMediaIO system object properties.
//!
//! Uses CMIOObjectGetPropertyDataSize and CMIOObjectHasProperty to inspect
//! the system-level CoreMediaIO object and enumerate available devices.

#![allow(non_upper_case_globals)]

use apple_sys::CoreMediaIO::*;

// CoreMediaIO constants from the framework headers
const kCMIOObjectSystemObject: CMIOObjectID = 1;
// 'glob' selector for kCMIOObjectPropertyClass
const kCMIOObjectPropertyOwnedObjects: CMIOObjectPropertySelector =
    u32::from_be_bytes([b'o', b'w', b'n', b'd']);
const kCMIOObjectPropertyClass: CMIOObjectPropertySelector =
    u32::from_be_bytes([b'c', b'l', b'a', b's']);
const kCMIOObjectPropertyName: CMIOObjectPropertySelector =
    u32::from_be_bytes([b'l', b'n', b'a', b'm']);
const kCMIOHardwarePropertyDevices: CMIOObjectPropertySelector =
    u32::from_be_bytes([b'd', b'e', b'v', b'#']);
const kCMIOHardwarePropertyAllowScreenCaptureDevices: CMIOObjectPropertySelector =
    u32::from_be_bytes([b'y', b'e', b's', b'2']);
const kCMIOObjectPropertyScopeGlobal: CMIOObjectPropertyScope =
    u32::from_be_bytes([b'g', b'l', b'o', b'b']);
const kCMIOObjectPropertyElementMain: CMIOObjectPropertyElement = 0;

fn fourcc_str(v: u32) -> String {
    let bytes = v.to_be_bytes();
    if bytes.iter().all(|b| b.is_ascii_graphic() || *b == b' ') {
        format!("'{}'", String::from_utf8_lossy(&bytes))
    } else {
        format!("0x{v:08x}")
    }
}

fn main() {
    unsafe {
        println!("=== CoreMediaIO System Object Properties ===\n");

        // Check if system object has the devices property
        let devices_addr = CMIOObjectPropertyAddress {
            mSelector: kCMIOHardwarePropertyDevices,
            mScope: kCMIOObjectPropertyScopeGlobal,
            mElement: kCMIOObjectPropertyElementMain,
        };

        let has_devices = CMIOObjectHasProperty(kCMIOObjectSystemObject, &devices_addr);
        println!("System object has 'dev#' property: {}", has_devices != 0);

        if has_devices != 0 {
            let mut data_size: u32 = 0;
            let status = CMIOObjectGetPropertyDataSize(
                kCMIOObjectSystemObject,
                &devices_addr,
                0,
                std::ptr::null(),
                &mut data_size,
            );
            if status == 0 {
                let device_count = data_size as usize / std::mem::size_of::<CMIODeviceID>();
                println!("Devices property data size: {data_size} bytes ({device_count} devices)");

                if device_count > 0 {
                    let mut devices = vec![0u32; device_count];
                    let mut actual_size = data_size;
                    let status = CMIOObjectGetPropertyData(
                        kCMIOObjectSystemObject,
                        &devices_addr,
                        0,
                        std::ptr::null(),
                        data_size,
                        &mut actual_size,
                        devices.as_mut_ptr() as *mut std::ffi::c_void,
                    );

                    if status == 0 {
                        println!("\n--- Devices ---");
                        for (i, &dev_id) in devices.iter().enumerate() {
                            println!("  Device #{i}: ID={dev_id}");

                            // Query device name
                            let name_addr = CMIOObjectPropertyAddress {
                                mSelector: kCMIOObjectPropertyName,
                                mScope: kCMIOObjectPropertyScopeGlobal,
                                mElement: kCMIOObjectPropertyElementMain,
                            };
                            if CMIOObjectHasProperty(dev_id, &name_addr) != 0 {
                                let mut name_size: u32 = 0;
                                let s = CMIOObjectGetPropertyDataSize(
                                    dev_id,
                                    &name_addr,
                                    0,
                                    std::ptr::null(),
                                    &mut name_size,
                                );
                                if s == 0 {
                                    println!("    Name property size: {name_size} bytes");
                                }
                            }

                            // Query device class
                            let class_addr = CMIOObjectPropertyAddress {
                                mSelector: kCMIOObjectPropertyClass,
                                mScope: kCMIOObjectPropertyScopeGlobal,
                                mElement: kCMIOObjectPropertyElementMain,
                            };
                            if CMIOObjectHasProperty(dev_id, &class_addr) != 0 {
                                let mut class_val: u32 = 0;
                                let mut actual: u32 = 4;
                                let s = CMIOObjectGetPropertyData(
                                    dev_id,
                                    &class_addr,
                                    0,
                                    std::ptr::null(),
                                    4,
                                    &mut actual,
                                    &mut class_val as *mut u32 as *mut std::ffi::c_void,
                                );
                                if s == 0 {
                                    println!("    Class: {}", fourcc_str(class_val));
                                }
                            }
                        }
                    } else {
                        println!("  Failed to get device list: status={status}");
                    }
                }
            } else {
                println!("  Failed to get property data size: status={status}");
            }
        }

        // Check screen capture devices property
        println!("\n--- Screen Capture Devices ---");
        let screen_addr = CMIOObjectPropertyAddress {
            mSelector: kCMIOHardwarePropertyAllowScreenCaptureDevices,
            mScope: kCMIOObjectPropertyScopeGlobal,
            mElement: kCMIOObjectPropertyElementMain,
        };
        let has_screen = CMIOObjectHasProperty(kCMIOObjectSystemObject, &screen_addr);
        println!("Has AllowScreenCaptureDevices: {}", has_screen != 0);

        if has_screen != 0 {
            let mut settable: u8 = 0;
            let s =
                CMIOObjectIsPropertySettable(kCMIOObjectSystemObject, &screen_addr, &mut settable);
            if s == 0 {
                println!("AllowScreenCaptureDevices is settable: {}", settable != 0);
            }
        }

        // Check owned objects
        let owned_addr = CMIOObjectPropertyAddress {
            mSelector: kCMIOObjectPropertyOwnedObjects,
            mScope: kCMIOObjectPropertyScopeGlobal,
            mElement: kCMIOObjectPropertyElementMain,
        };
        let has_owned = CMIOObjectHasProperty(kCMIOObjectSystemObject, &owned_addr);
        println!("\nHas OwnedObjects property: {}", has_owned != 0);
        if has_owned != 0 {
            let mut size: u32 = 0;
            let s = CMIOObjectGetPropertyDataSize(
                kCMIOObjectSystemObject,
                &owned_addr,
                0,
                std::ptr::null(),
                &mut size,
            );
            if s == 0 {
                let count = size as usize / std::mem::size_of::<CMIOObjectID>();
                println!("Owned objects count: {count}");
            }
        }
    }

    println!("\nDone.");
}
