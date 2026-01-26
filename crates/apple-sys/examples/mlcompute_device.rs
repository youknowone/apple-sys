//! Query MLCompute devices and framework capabilities.
//!
//! Gets MLCDevice for CPU and GPU, checks device type and description,
//! and explores MLCGraph and MLCTensor creation.

use apple_sys::MLCompute::*;

use objc2::runtime::AnyObject;
mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MLCompute ===\n");

        // MLCDevice - CPU
        println!("--- MLCDevice (CPU) ---");
        let cpu_device = MLCDevice(MLCDevice::cpuDevice());
        if !cpu_device.0.is_null() {
            let device_type = IMLCDevice::type_(&cpu_device);
            let type_name = match device_type {
                0 => "CPU",
                1 => "GPU",
                2 => "Any",
                _ => "Unknown",
            };
            println!("  Type: {} ({})", type_name, device_type);

            let desc = PNSObject::description(&cpu_device);
            println!("  Description: {}", nsstring_to_string(desc));
        }

        // MLCDevice - GPU
        println!("\n--- MLCDevice (GPU) ---");
        let gpu_device = MLCDevice(MLCDevice::gpuDevice());
        if !gpu_device.0.is_null() {
            let device_type = IMLCDevice::type_(&gpu_device);
            let type_name = match device_type {
                0 => "CPU",
                1 => "GPU",
                2 => "Any",
                _ => "Unknown",
            };
            println!("  Type: {} ({})", type_name, device_type);

            let desc = PNSObject::description(&gpu_device);
            println!("  Description: {}", nsstring_to_string(desc));

            let gpu_devices = IMLCDevice::gpuDevices(&gpu_device);
            if !gpu_devices.0.is_null() {
                let count = INSArray::<id>::count(&gpu_devices);
                println!("  GPU devices: {}", count);
                for i in 0..count {
                    let gpu = INSArray::<id>::objectAtIndex_(&gpu_devices, i);
                    // MTLDevice is a protocol with no concrete struct.
                    let name: *mut AnyObject = objc2::msg_send![&*gpu, name];
                    println!("    [{}] {}", i, nsstring_to_string(NSString(name as _)));
                }
            }
        } else {
            println!("  No GPU device available");
        }

        // MLCDevice - ANE (any device)
        println!("\n--- MLCDevice (Any/ANE) ---");
        let ane_device = MLCDevice(MLCDevice::deviceWithType_(2)); // MLCDeviceTypeAny
        if !ane_device.0.is_null() {
            let desc = PNSObject::description(&ane_device);
            println!("  Description: {}", nsstring_to_string(desc));
        } else {
            println!("  Not available");
        }

        // MLCGraph
        println!("\n--- MLCGraph ---");
        let graph = MLCGraph(MLCGraph::graph());
        if !graph.0.is_null() {
            let desc = PNSObject::description(&graph);
            println!("  Graph: {}", nsstring_to_string(desc));

            let layers = IMLCGraph::layers(&graph);
            if !layers.0.is_null() {
                let count = INSArray::<id>::count(&layers);
                println!("  Layers (empty graph): {}", count);
            }

            // summarizedDescription
            let summary = IMLCGraph::summarizedDOTDescription(&graph);
            if !summary.0.is_null() {
                let summary_str = nsstring_to_string(summary);
                if !summary_str.is_empty() {
                    let truncated = if summary_str.len() > 100 {
                        format!("{}...", &summary_str[..100])
                    } else {
                        summary_str
                    };
                    println!("  DOT description: {}", truncated);
                }
            }
        }

        // MLCActivationType debug descriptions
        println!("\n--- MLCActivationType Debug Descriptions ---");
        for (val, expected) in [
            (0i32, "None"),
            (1, "ReLU"),
            (2, "Linear"),
            (3, "Sigmoid"),
            (4, "HardSigmoid"),
            (5, "Tanh"),
        ] {
            let ns = MLCActivationTypeDebugDescription(val);
            let desc = nsstring_to_string(ns);
            println!("  {} => {} (expected: {})", val, desc, expected);
        }
    }

    println!("\nDone.");
}
