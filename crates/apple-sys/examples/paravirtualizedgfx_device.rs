//! Create PGDeviceDescriptor and PGDisplayDescriptor via ParavirtualizedGraphics bindings.
//!
//! Demonstrates ParavirtualizedGraphics framework by creating descriptors
//! and querying their properties through generated trait methods.

use apple_sys::ParavirtualizedGraphics::*;

mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ParavirtualizedGraphics ===\n");

        // PGDeviceDescriptor
        println!("--- PGDeviceDescriptor ---");
        {
            let desc_obj = PGDeviceDescriptor::alloc();
            let desc_obj = PGDeviceDescriptor(INSObject::init(&desc_obj));
            if !desc_obj.0.is_null() {
                println!("  Descriptor: {}", nsobj_to_string(desc_obj.0));

                let mmio_length = IPGDeviceDescriptor::mmioLength(&desc_obj);
                println!("  mmioLength: {}", mmio_length);

                // Set and verify mmioLength
                IPGDeviceDescriptor::setMmioLength_(&desc_obj, 0x10000);
                let new_mmio = IPGDeviceDescriptor::mmioLength(&desc_obj);
                println!("  mmioLength after set(0x10000): 0x{:X}", new_mmio);

                let device_ptr = IPGDeviceDescriptor::device(&desc_obj);
                println!("  device ptr: {:?}", device_ptr);
            } else {
                println!("  Failed to create PGDeviceDescriptor");
            }
        }

        // PGDisplayDescriptor
        println!("\n--- PGDisplayDescriptor ---");
        {
            let desc_obj = PGDisplayDescriptor::alloc();
            let desc_obj = PGDisplayDescriptor(INSObject::init(&desc_obj));
            if !desc_obj.0.is_null() {
                println!("  Descriptor: {}", nsobj_to_string(desc_obj.0));

                // Query and set name
                let name = IPGDisplayDescriptor::name(&desc_obj);
                println!(
                    "  Default name: {}",
                    if name.0.is_null() {
                        "nil".to_string()
                    } else {
                        nsstring_to_string(name)
                    }
                );

                let test_name = nsstring(c"TestDisplay");
                IPGDisplayDescriptor::setName_(&desc_obj, test_name);
                let updated_name = IPGDisplayDescriptor::name(&desc_obj);
                println!("  Name after set: {}", nsstring_to_string(updated_name));

                // Query sizeInMillimeters
                let size = IPGDisplayDescriptor::sizeInMillimeters(&desc_obj);
                println!("  Size in mm: {}x{}", size.width, size.height);

                // Set size
                let new_size = NSSize {
                    width: 600.0,
                    height: 340.0,
                };
                IPGDisplayDescriptor::setSizeInMillimeters_(&desc_obj, new_size);
                let updated_size = IPGDisplayDescriptor::sizeInMillimeters(&desc_obj);
                println!(
                    "  Size after set: {}x{}",
                    updated_size.width, updated_size.height
                );
            } else {
                println!("  Failed to create PGDisplayDescriptor");
            }
        }
    }

    println!("\nDone.");
}
