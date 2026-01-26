//! Print sizes and field info of IOUSBHost descriptor structs.
//!
//! Demonstrates the USB descriptor struct layouts from the IOUSBHost framework
//! including device, configuration, interface, endpoint, and BOS descriptors.

use apple_sys::IOUSBHost::*;

fn main() {
    println!("=== IOUSBHost Descriptor Structs ===\n");

    println!("--- IOUSBDescriptorHeader ---");
    println!(
        "  size: {} bytes",
        std::mem::size_of::<IOUSBDescriptorHeader>()
    );
    println!(
        "  align: {} bytes",
        std::mem::align_of::<IOUSBDescriptorHeader>()
    );

    println!("\n--- IOUSBDeviceDescriptor ---");
    println!(
        "  size: {} bytes",
        std::mem::size_of::<IOUSBDeviceDescriptor>()
    );
    println!(
        "  align: {} bytes",
        std::mem::align_of::<IOUSBDeviceDescriptor>()
    );
    println!("  fields: bLength, bDescriptorType, bcdUSB, bDeviceClass,");
    println!("          bDeviceSubClass, bDeviceProtocol, bMaxPacketSize0,");
    println!("          idVendor, idProduct, bcdDevice,");
    println!("          iManufacturer, iProduct, iSerialNumber, bNumConfigurations");

    // Create a zeroed descriptor and show its default debug output
    let dev_desc = IOUSBDeviceDescriptor {
        bLength: std::mem::size_of::<IOUSBDeviceDescriptor>() as u8,
        bDescriptorType: 1, // DEVICE
        bcdUSB: 0x0200,     // USB 2.0
        bDeviceClass: 0,
        bDeviceSubClass: 0,
        bDeviceProtocol: 0,
        bMaxPacketSize0: 64,
        idVendor: 0x05AC, // Apple
        idProduct: 0x8600,
        bcdDevice: 0x0100,
        iManufacturer: 1,
        iProduct: 2,
        iSerialNumber: 3,
        bNumConfigurations: 1,
    };
    println!("  example: {:?}", dev_desc);

    println!("\n--- IOUSBConfigurationDescriptor ---");
    println!(
        "  size: {} bytes",
        std::mem::size_of::<IOUSBConfigurationDescriptor>()
    );
    println!(
        "  align: {} bytes",
        std::mem::align_of::<IOUSBConfigurationDescriptor>()
    );
    println!("  fields: bLength, bDescriptorType, wTotalLength, bNumInterfaces,");
    println!("          bConfigurationValue, iConfiguration, bmAttributes, MaxPower");

    let config_desc = IOUSBConfigurationDescriptor {
        bLength: std::mem::size_of::<IOUSBConfigurationDescriptor>() as u8,
        bDescriptorType: 2, // CONFIGURATION
        wTotalLength: 32,
        bNumInterfaces: 1,
        bConfigurationValue: 1,
        iConfiguration: 0,
        bmAttributes: 0x80, // Bus powered
        MaxPower: 250,      // 500mA
    };
    println!("  example: {:?}", config_desc);

    println!("\n--- IOUSBInterfaceDescriptor ---");
    println!(
        "  size: {} bytes",
        std::mem::size_of::<IOUSBInterfaceDescriptor>()
    );
    println!(
        "  align: {} bytes",
        std::mem::align_of::<IOUSBInterfaceDescriptor>()
    );
    println!("  fields: bLength, bDescriptorType, bInterfaceNumber,");
    println!("          bAlternateSetting, bNumEndpoints,");
    println!("          bInterfaceClass, bInterfaceSubClass, bInterfaceProtocol, iInterface");

    let iface_desc = IOUSBInterfaceDescriptor {
        bLength: std::mem::size_of::<IOUSBInterfaceDescriptor>() as u8,
        bDescriptorType: 4, // INTERFACE
        bInterfaceNumber: 0,
        bAlternateSetting: 0,
        bNumEndpoints: 2,
        bInterfaceClass: 0xFF, // Vendor-specific
        bInterfaceSubClass: 0x01,
        bInterfaceProtocol: 0x01,
        iInterface: 0,
    };
    println!("  example: {:?}", iface_desc);

    println!("\n--- IOUSBEndpointDescriptor ---");
    println!(
        "  size: {} bytes",
        std::mem::size_of::<IOUSBEndpointDescriptor>()
    );
    println!(
        "  align: {} bytes",
        std::mem::align_of::<IOUSBEndpointDescriptor>()
    );
    println!("  fields: bLength, bDescriptorType, bEndpointAddress,");
    println!("          bmAttributes, wMaxPacketSize, bInterval");

    let ep_desc = IOUSBEndpointDescriptor {
        bLength: std::mem::size_of::<IOUSBEndpointDescriptor>() as u8,
        bDescriptorType: 5,     // ENDPOINT
        bEndpointAddress: 0x81, // EP1 IN
        bmAttributes: 0x02,     // Bulk
        wMaxPacketSize: 512,
        bInterval: 0,
    };
    println!("  example: {:?}", ep_desc);

    println!("\n--- IOUSBBOSDescriptor ---");
    println!(
        "  size: {} bytes",
        std::mem::size_of::<IOUSBBOSDescriptor>()
    );
    println!(
        "  align: {} bytes",
        std::mem::align_of::<IOUSBBOSDescriptor>()
    );
    println!("  fields: bLength, bDescriptorType, wTotalLength, bNumDeviceCaps");

    let bos_desc = IOUSBBOSDescriptor {
        bLength: std::mem::size_of::<IOUSBBOSDescriptor>() as u8,
        bDescriptorType: 15, // BOS
        wTotalLength: 22,
        bNumDeviceCaps: 2,
    };
    println!("  example: {:?}", bos_desc);

    println!("\n--- IOUSBDeviceCapabilityUSB2Extension ---");
    println!(
        "  size: {} bytes",
        std::mem::size_of::<IOUSBDeviceCapabilityUSB2Extension>()
    );

    println!("\n--- IOUSBDeviceCapabilitySuperSpeedUSB ---");
    println!(
        "  size: {} bytes",
        std::mem::size_of::<IOUSBDeviceCapabilitySuperSpeedUSB>()
    );

    println!("\nDone.");
}
