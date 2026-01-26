//! Create MTKView and query its properties.
//!
//! Demonstrates MetalKit by creating an MTKView with a Metal device,
//! then querying preferredFramesPerSecond, colorPixelFormat, and more.

use apple_sys::Metal::*;
use apple_sys::MetalKit::*;
use objc2::runtime::AnyObject;
mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MetalKit ===\n");

        // Get default Metal device
        let device = MTLCreateSystemDefaultDevice();
        if device.is_null() {
            println!("No Metal device available");
            return;
        }
        // MTLDevice is a protocol with no concrete struct;
        // PMTLDevice trait exists but has no implementors.
        let device_ptr = device as *mut AnyObject;
        let dev_name: *mut AnyObject = objc2::msg_send![device_ptr, name];
        println!(
            "Metal device: {}",
            nsstring_to_string(NSString(dev_name as _))
        );

        // Create MTKView using alloc/init, then set device
        {
            let view = MTKView::alloc();
            let view = MTKView(INSObject::init(&view));

            if !view.0.is_null() {
                // Set the Metal device
                IMTKView::setDevice_(&view, device);

                println!("\n--- MTKView Properties ---");

                let view_device = IMTKView::device(&view);
                if !view_device.is_null() {
                    let name: *mut AnyObject =
                        objc2::msg_send![view_device as *mut AnyObject, name];
                    println!("  Device: {}", nsstring_to_string(NSString(name as _)));
                }

                let fps = IMTKView::preferredFramesPerSecond(&view);
                println!("  Preferred FPS: {}", fps);

                let color_fmt = IMTKView::colorPixelFormat(&view);
                let fmt_name = match color_fmt {
                    80 => "BGRA8Unorm",
                    81 => "BGRA8Unorm_sRGB",
                    70 => "RGBA8Unorm",
                    115 => "RGBA16Float",
                    _ => "Other",
                };
                println!("  Color pixel format: {} ({})", fmt_name, color_fmt);

                let depth_fmt = IMTKView::depthStencilPixelFormat(&view);
                let depth_name = match depth_fmt {
                    0 => "Invalid (none)",
                    252 => "Depth32Float",
                    253 => "Stencil8",
                    260 => "Depth32Float_Stencil8",
                    _ => "Other",
                };
                println!("  Depth/stencil format: {} ({})", depth_name, depth_fmt);

                let sample_count = IMTKView::sampleCount(&view);
                println!("  Sample count: {}", sample_count);

                let paused = IMTKView::isPaused(&view);
                println!("  Paused: {}", paused.0);

                let enable_set_needs = IMTKView::enableSetNeedsDisplay(&view);
                println!("  EnableSetNeedsDisplay: {}", enable_set_needs.0);

                let framebuffer_only = IMTKView::framebufferOnly(&view);
                println!("  Framebuffer only: {}", framebuffer_only.0);

                let presents_with_transaction = IMTKView::presentsWithTransaction(&view);
                println!(
                    "  Presents with transaction: {}",
                    presents_with_transaction.0
                );

                let auto_resize = IMTKView::autoResizeDrawable(&view);
                println!("  Auto resize drawable: {}", auto_resize.0);

                let clear_depth = IMTKView::clearDepth(&view);
                println!("  Clear depth: {}", clear_depth);

                let clear_stencil = IMTKView::clearStencil(&view);
                println!("  Clear stencil: {}", clear_stencil);

                // Check preferred device
                let preferred_device = IMTKView::preferredDevice(&view);
                if !preferred_device.is_null() {
                    let pref_name: *mut AnyObject =
                        objc2::msg_send![preferred_device as *mut AnyObject, name];
                    println!(
                        "  Preferred device: {}",
                        nsstring_to_string(NSString(pref_name as _))
                    );
                }

                // Check current drawable
                let drawable = IMTKView::currentDrawable(&view);
                println!(
                    "  Current drawable: {}",
                    if drawable.is_null() {
                        "none"
                    } else {
                        "available"
                    }
                );

                // Check current render pass descriptor
                let rpd = IMTKView::currentRenderPassDescriptor(&view);
                println!(
                    "  Current render pass descriptor: {}",
                    if rpd.0.is_null() { "none" } else { "available" }
                );
            } else {
                println!("Failed to create MTKView");
            }
        }
    }

    println!("\nDone.");
}
