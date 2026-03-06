//! Query OpenGL renderer information via CGL.
//!
//! Uses CGLQueryRendererInfo and CGLDescribeRenderer to list all
//! available OpenGL renderers and their capabilities.

#![allow(non_upper_case_globals)]

use apple_sys::OpenGL::*;
use std::ptr;

const kCGLRPRendererID: u32 = 70;
const kCGLRPVideoMemoryMegabytes: u32 = 131;
const kCGLRPTextureMemoryMegabytes: u32 = 132;
const kCGLRPAccelerated: u32 = 73;
const kCGLRPOnline: u32 = 125;
const kCGLRPMaxSampleBuffers: u32 = 100;
const kCGLRPMaxSamples: u32 = 101;

fn main() {
    unsafe {
        println!("=== OpenGL Renderer Information ===\n");

        // Query CGL version
        let mut major: i32 = 0;
        let mut minor: i32 = 0;
        CGLGetVersion(&mut major, &mut minor);
        println!("CGL version: {}.{}", major, minor);

        // Query all renderers on display mask 0xFFFFFFFF (all displays)
        let mut rend_info: CGLRendererInfoObj = ptr::null_mut();
        let mut rend_count: i32 = 0;
        let err = CGLQueryRendererInfo(0xFFFFFFFF, &mut rend_info, &mut rend_count);
        if err != 0 {
            let msg = CGLErrorString(err);
            if !msg.is_null() {
                let s = std::ffi::CStr::from_ptr(msg).to_string_lossy();
                eprintln!("CGLQueryRendererInfo failed: {}", s);
            }
            return;
        }

        println!("Found {} renderer(s)\n", rend_count);

        for i in 0..rend_count {
            let get = |prop: u32| -> i32 {
                let mut val: i32 = 0;
                CGLDescribeRenderer(rend_info, i, prop, &mut val);
                val
            };

            let renderer_id = get(kCGLRPRendererID);
            let accel = get(kCGLRPAccelerated);
            let online = get(kCGLRPOnline);
            let vram = get(kCGLRPVideoMemoryMegabytes);
            let tram = get(kCGLRPTextureMemoryMegabytes);
            let max_samples = get(kCGLRPMaxSamples);
            let max_sample_bufs = get(kCGLRPMaxSampleBuffers);

            println!("Renderer {}:", i);
            println!("  ID:              0x{:08x}", renderer_id);
            println!(
                "  Accelerated:     {}",
                if accel != 0 { "Yes" } else { "No" }
            );
            println!(
                "  Online:          {}",
                if online != 0 { "Yes" } else { "No" }
            );
            println!("  Video memory:    {} MB", vram);
            println!("  Texture memory:  {} MB", tram);
            println!("  Max samples:     {}", max_samples);
            println!("  Max sample bufs: {}", max_sample_bufs);
            println!();
        }

        CGLDestroyRendererInfo(rend_info);
    }

    println!("Done.");
}
