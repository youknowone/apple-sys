//! Check MetalPerformanceShaders image descriptor and data type capabilities.
//!
//! Creates MPSImageDescriptor instances with various configurations and
//! inspects MPS data type layout via generated bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::MetalPerformanceShaders::*;

mod common;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MetalPerformanceShaders ===\n");

        // MPSImageDescriptor
        println!("--- MPSImageDescriptor ---");
        {
            // Create a descriptor for Unorm8, 256x256, 4 channels
            let desc_id = <MPSImageDescriptor as IMPSImageDescriptor>::imageDescriptorWithChannelFormat_width_height_featureChannels_(
                0u64,  // MPSImageFeatureChannelFormatUnorm8
                256,
                256,
                4,
            );
            let desc = MPSImageDescriptor(desc_id);

            if !desc.0.is_null() {
                let width = IMPSImageDescriptor::width(&desc);
                let height = IMPSImageDescriptor::height(&desc);
                let channels = IMPSImageDescriptor::featureChannels(&desc);
                let num_images = IMPSImageDescriptor::numberOfImages(&desc);
                let chan_fmt = IMPSImageDescriptor::channelFormat(&desc);

                println!("  Width: {}", width);
                println!("  Height: {}", height);
                println!("  Feature channels: {}", channels);
                println!("  Number of images: {}", num_images);

                let chan_name = match chan_fmt {
                    0 => "Unorm8",
                    1 => "Unorm16",
                    2 => "Float16",
                    3 => "Float32",
                    _ => "Other",
                };
                println!("  Channel format: {} ({})", chan_name, chan_fmt);

                let usage = IMPSImageDescriptor::usage(&desc);
                println!("  Usage flags: 0x{:X}", usage);
            }

            // Create another descriptor with Float16
            let desc2_id = <MPSImageDescriptor as IMPSImageDescriptor>::imageDescriptorWithChannelFormat_width_height_featureChannels_(
                2u64,  // Float16
                1024,
                1024,
                3,
            );
            let desc2 = MPSImageDescriptor(desc2_id);
            if !desc2.0.is_null() {
                println!("\n  Second descriptor (1024x1024 Float16 3ch):");
                let width = IMPSImageDescriptor::width(&desc2);
                let height = IMPSImageDescriptor::height(&desc2);
                let channels = IMPSImageDescriptor::featureChannels(&desc2);
                println!("    {}x{}, {} channels", width, height, channels);
            }
        }

        // MPS data types
        println!("\n--- MPS Data Types ---");
        println!(
            "  MPSOffset size: {} bytes",
            std::mem::size_of::<MPSOffset>()
        );
        println!(
            "  MPSOrigin size: {} bytes",
            std::mem::size_of::<MPSOrigin>()
        );
        println!("  MPSSize size: {} bytes", std::mem::size_of::<MPSSize>());
        println!(
            "  MPSRegion size: {} bytes",
            std::mem::size_of::<MPSRegion>()
        );
        println!(
            "  MPSScaleTransform size: {} bytes",
            std::mem::size_of::<MPSScaleTransform>()
        );
    }

    println!("\nDone.");
}
