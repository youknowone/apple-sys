//! Create and inspect a CVPixelBuffer.
//!
//! Demonstrates CoreVideo's CVPixelBuffer APIs: creating a pixel buffer,
//! locking it, writing pixel data, and querying its properties.

use apple_sys::AddressBook::CVPixelBufferRef;
use apple_sys::CoreFoundation::{OSType, kCFAllocatorDefault};
use apple_sys::CoreVideo::*;
use std::ptr;

fn main() {
    unsafe {
        println!("=== CoreVideo PixelBuffer Demo ===\n");

        // 1. Create a 320x240 BGRA pixel buffer
        let width: usize = 320;
        let height: usize = 240;
        let pixel_format: OSType = 0x42475241; // 'BGRA' = kCVPixelFormatType_32BGRA

        let mut pixel_buffer: CVPixelBufferRef = ptr::null_mut();
        let status = CVPixelBufferCreate(
            kCFAllocatorDefault,
            width,
            height,
            pixel_format,
            ptr::null(),
            &mut pixel_buffer,
        );
        assert_eq!(status, 0, "CVPixelBufferCreate failed: {}", status);
        assert!(!pixel_buffer.is_null());
        println!("Created pixel buffer: {:?}", pixel_buffer);

        // 2. Query properties
        let w = CVPixelBufferGetWidth(pixel_buffer);
        let h = CVPixelBufferGetHeight(pixel_buffer);
        let bpr = CVPixelBufferGetBytesPerRow(pixel_buffer);
        let fmt = CVPixelBufferGetPixelFormatType(pixel_buffer);
        let planes = CVPixelBufferGetPlaneCount(pixel_buffer);
        println!("Width: {}", w);
        println!("Height: {}", h);
        println!("Bytes per row: {}", bpr);
        println!(
            "Pixel format: 0x{:08X} ('{}')",
            fmt,
            std::str::from_utf8(&fmt.to_be_bytes()).unwrap_or("????")
        );
        println!("Plane count: {} (0 = non-planar)", planes);

        // 3. Lock and write a gradient pattern
        let lock_status = CVPixelBufferLockBaseAddress(pixel_buffer, 0);
        assert_eq!(lock_status, 0, "Lock failed");

        let base = CVPixelBufferGetBaseAddress(pixel_buffer);
        assert!(!base.is_null(), "Base address is null");

        // Write BGRA gradient
        for y in 0..h {
            let row = (base as *mut u8).add(y * bpr);
            for x in 0..w {
                let px = row.add(x * 4);
                *px = (x * 255 / w) as u8; // B
                *px.add(1) = (y * 255 / h) as u8; // G
                *px.add(2) = 128; // R
                *px.add(3) = 255; // A
            }
        }
        println!("Wrote gradient pattern ({} bytes)", h * bpr);

        let unlock_status = CVPixelBufferUnlockBaseAddress(pixel_buffer, 0);
        assert_eq!(unlock_status, 0, "Unlock failed");

        // 4. Check if the buffer is planar
        let is_planar = CVPixelBufferIsPlanar(pixel_buffer);
        println!("Is planar: {}", is_planar != 0);

        // 5. Data size
        let data_size = CVPixelBufferGetDataSize(pixel_buffer);
        println!("Total data size: {} bytes", data_size);

        // Cleanup
        CVPixelBufferRelease(pixel_buffer);
    }

    println!("\nDone.");
}
