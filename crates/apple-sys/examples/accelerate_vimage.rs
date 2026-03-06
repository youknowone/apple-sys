//! Demonstrate image processing using Accelerate's vImage framework.
//!
//! Creates a simple grayscale image buffer, applies a separable convolution
//! (box blur), and prints pixel statistics before and after.

use apple_sys::Accelerate::*;

fn main() {
    unsafe {
        // Create a simple 8x8 grayscale image with a bright center
        let width: usize = 8;
        let height: usize = 8;
        let bytes_per_row = width;
        let mut src_data = vec![0u8; width * height];

        // Draw a bright 4x4 square in the center
        for y in 2..6 {
            for x in 2..6 {
                src_data[y * width + x] = 255;
            }
        }

        println!("Source image (8x8 grayscale):");
        print_image(&src_data, width, height);

        // Set up vImage buffers
        let src_buf = vImage_Buffer {
            data: src_data.as_mut_ptr() as *mut std::ffi::c_void,
            height: height as vImagePixelCount,
            width: width as vImagePixelCount,
            rowBytes: bytes_per_row,
        };

        let mut dst_data = vec![0u8; width * height];
        let dst_buf = vImage_Buffer {
            data: dst_data.as_mut_ptr() as *mut std::ffi::c_void,
            height: height as vImagePixelCount,
            width: width as vImagePixelCount,
            rowBytes: bytes_per_row,
        };

        // Apply separable 3-tap box blur kernel: [1/3, 1/3, 1/3]
        let kernel: [f32; 3] = [1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0];
        let kvimage_edge_extend: vImage_Flags = 8; // kvImageEdgeExtend
        let err = vImageSepConvolve_Planar8(
            &src_buf,
            &dst_buf,
            std::ptr::null_mut(), // tempBuffer
            0,                    // srcOffsetToROI_X
            0,                    // srcOffsetToROI_Y
            kernel.as_ptr(),      // kernelX
            3,                    // kernelX_width
            kernel.as_ptr(),      // kernelY
            3,                    // kernelY_width
            0.0,                  // bias
            0,                    // backgroundColor
            kvimage_edge_extend,
        );

        if err != 0 {
            eprintln!("vImageSepConvolve_Planar8 failed: {err}");
            return;
        }

        println!("\nAfter 3x3 box blur (separable convolution):");
        print_image(&dst_data, width, height);

        // Print statistics
        let src_sum: u64 = src_data.iter().map(|&x| x as u64).sum();
        let dst_sum: u64 = dst_data.iter().map(|&x| x as u64).sum();
        let pixel_count = (width * height) as f64;
        println!("\nStatistics:");
        println!(
            "  Source avg intensity: {:.1}",
            src_sum as f64 / pixel_count
        );
        println!(
            "  Blurred avg intensity: {:.1}",
            dst_sum as f64 / pixel_count
        );
    }
}

fn print_image(data: &[u8], width: usize, height: usize) {
    for y in 0..height {
        print!("  ");
        for x in 0..width {
            let val = data[y * width + x];
            let ch = match val {
                0 => '.',
                1..=63 => '\u{2591}',    // light shade
                64..=127 => '\u{2592}',  // medium shade
                128..=191 => '\u{2593}', // dark shade
                _ => '\u{2588}',         // full block
            };
            print!("{ch}");
        }
        println!();
    }
}
