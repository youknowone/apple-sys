//! Create MetalFX scaler descriptors and query their properties.
//!
//! Demonstrates MetalFX framework by creating MTLFXTemporalScalerDescriptor
//! and MTLFXSpatialScalerDescriptor and inspecting their properties.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::MetalFX::*;

mod common;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MetalFX ===\n");

        // MTLFXTemporalScalerDescriptor
        println!("--- MTLFXTemporalScalerDescriptor ---");
        let desc_obj = MTLFXTemporalScalerDescriptor::alloc();
        let desc_obj = MTLFXTemporalScalerDescriptor(INSObject::init(&desc_obj));
        if !desc_obj.0.is_null() {
            // Set input/output dimensions
            IMTLFXTemporalScalerDescriptor::setInputWidth_(&desc_obj, 1920);
            IMTLFXTemporalScalerDescriptor::setInputHeight_(&desc_obj, 1080);
            IMTLFXTemporalScalerDescriptor::setOutputWidth_(&desc_obj, 3840);
            IMTLFXTemporalScalerDescriptor::setOutputHeight_(&desc_obj, 2160);

            let input_w = IMTLFXTemporalScalerDescriptor::inputWidth(&desc_obj);
            let input_h = IMTLFXTemporalScalerDescriptor::inputHeight(&desc_obj);
            let output_w = IMTLFXTemporalScalerDescriptor::outputWidth(&desc_obj);
            let output_h = IMTLFXTemporalScalerDescriptor::outputHeight(&desc_obj);

            println!("  Input:  {}x{}", input_w, input_h);
            println!("  Output: {}x{}", output_w, output_h);
            println!("  Scale factor: {:.1}x", output_w as f64 / input_w as f64);

            let color_fmt = IMTLFXTemporalScalerDescriptor::colorTextureFormat(&desc_obj);
            let depth_fmt = IMTLFXTemporalScalerDescriptor::depthTextureFormat(&desc_obj);
            let motion_fmt = IMTLFXTemporalScalerDescriptor::motionTextureFormat(&desc_obj);
            let output_fmt = IMTLFXTemporalScalerDescriptor::outputTextureFormat(&desc_obj);
            println!("  Color texture format:  {}", color_fmt);
            println!("  Depth texture format:  {}", depth_fmt);
            println!("  Motion texture format: {}", motion_fmt);
            println!("  Output texture format: {}", output_fmt);

            let auto_exp = IMTLFXTemporalScalerDescriptor::isAutoExposureEnabled(&desc_obj);
            println!("  Auto exposure enabled: {}", auto_exp.0);

            let content_props =
                IMTLFXTemporalScalerDescriptor::isInputContentPropertiesEnabled(&desc_obj);
            println!("  Input content properties enabled: {}", content_props.0);

            let min_scale = IMTLFXTemporalScalerDescriptor::inputContentMinScale(&desc_obj);
            let max_scale = IMTLFXTemporalScalerDescriptor::inputContentMaxScale(&desc_obj);
            println!("  Input content min scale: {}", min_scale);
            println!("  Input content max scale: {}", max_scale);

            let reactive_mask =
                IMTLFXTemporalScalerDescriptor::isReactiveMaskTextureEnabled(&desc_obj);
            println!("  Reactive mask enabled: {}", reactive_mask.0);
        } else {
            println!("  (init returned nil)");
        }

        // MTLFXSpatialScalerDescriptor
        println!("\n--- MTLFXSpatialScalerDescriptor ---");
        let desc_obj = MTLFXSpatialScalerDescriptor::alloc();
        let desc_obj = MTLFXSpatialScalerDescriptor(INSObject::init(&desc_obj));
        if !desc_obj.0.is_null() {
            IMTLFXSpatialScalerDescriptor::setInputWidth_(&desc_obj, 1280);
            IMTLFXSpatialScalerDescriptor::setInputHeight_(&desc_obj, 720);
            IMTLFXSpatialScalerDescriptor::setOutputWidth_(&desc_obj, 2560);
            IMTLFXSpatialScalerDescriptor::setOutputHeight_(&desc_obj, 1440);

            let input_w = IMTLFXSpatialScalerDescriptor::inputWidth(&desc_obj);
            let input_h = IMTLFXSpatialScalerDescriptor::inputHeight(&desc_obj);
            let output_w = IMTLFXSpatialScalerDescriptor::outputWidth(&desc_obj);
            let output_h = IMTLFXSpatialScalerDescriptor::outputHeight(&desc_obj);

            println!("  Input:  {}x{}", input_w, input_h);
            println!("  Output: {}x{}", output_w, output_h);
            println!("  Scale factor: {:.1}x", output_w as f64 / input_w as f64);

            let color_fmt = IMTLFXSpatialScalerDescriptor::colorTextureFormat(&desc_obj);
            let output_fmt = IMTLFXSpatialScalerDescriptor::outputTextureFormat(&desc_obj);
            println!("  Color texture format:  {}", color_fmt);
            println!("  Output texture format: {}", output_fmt);
        } else {
            println!("  (init returned nil)");
        }
    }

    println!("\nDone.");
}
