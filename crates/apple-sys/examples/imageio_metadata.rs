//! Read image metadata from a PNG file created in-memory.
//!
//! Demonstrates ImageIO's CGImageSource and CGImageDestination APIs:
//! creating an image in memory, writing it to a data buffer, then
//! reading back its properties.

#![allow(non_upper_case_globals)]

use apple_sys::CoreFoundation::*;
use apple_sys::CoreGraphics::*;
use apple_sys::ImageIO::*;
use std::os::raw::c_void;
use std::ptr;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;

fn main() {
    unsafe {
        println!("=== ImageIO Metadata Demo ===\n");

        // 1. Create a small 4x4 RGBA image in a CGImage
        let width: usize = 4;
        let height: usize = 4;
        let bpp = 4; // RGBA
        let mut pixels = vec![0u8; width * height * bpp];
        // Fill with a gradient pattern
        for y in 0..height {
            for x in 0..width {
                let i = (y * width + x) * bpp;
                pixels[i] = (x * 64) as u8; // R
                pixels[i + 1] = (y * 64) as u8; // G
                pixels[i + 2] = 128; // B
                pixels[i + 3] = 255; // A
            }
        }

        let colorspace = CGColorSpaceCreateDeviceRGB();
        let data_provider = CGDataProviderCreateWithData(
            ptr::null_mut(),
            pixels.as_ptr() as *const c_void,
            pixels.len(),
            None,
        );
        let image = CGImageCreate(
            width,
            height,
            8,           // bits per component
            32,          // bits per pixel
            width * bpp, // bytes per row
            colorspace,
            1 | (2 << 12), // kCGImageAlphaLast | kCGBitmapByteOrderDefault
            data_provider,
            ptr::null(),
            false as _,
            0, // kCGRenderingIntentDefault
        );
        assert!(!image.is_null(), "Failed to create CGImage");

        // 2. Write image to an in-memory CFData as PNG
        let mutable_data = CFDataCreateMutable(kCFAllocatorDefault, 0);
        let png_type = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"public.png".as_ptr(),
            kCFStringEncodingUTF8,
        );
        let dest = CGImageDestinationCreateWithData(
            mutable_data as CFMutableDataRef,
            png_type,
            1,
            ptr::null(),
        );
        assert!(!dest.is_null(), "Failed to create image destination");

        CGImageDestinationAddImage(dest, image, ptr::null());
        let ok = CGImageDestinationFinalize(dest);
        assert!(ok, "Failed to finalize image destination");

        let data_len = CFDataGetLength(mutable_data);
        println!("PNG data size: {} bytes", data_len);

        // 3. Read back the PNG using CGImageSource
        let source = CGImageSourceCreateWithData(mutable_data as CFDataRef, ptr::null());
        assert!(!source.is_null(), "Failed to create image source");

        let count = CGImageSourceGetCount(source);
        println!("Image count in source: {}", count);

        // 4. Get image properties (metadata)
        let props = CGImageSourceCopyPropertiesAtIndex(source, 0, ptr::null());
        if !props.is_null() {
            let n = CFDictionaryGetCount(props);
            println!("Property count: {}", n);

            // Read PixelWidth
            let width_key = CFStringCreateWithCString(
                kCFAllocatorDefault,
                c"PixelWidth".as_ptr(),
                kCFStringEncodingUTF8,
            );
            let mut width_val: *const c_void = ptr::null();
            if CFDictionaryGetValueIfPresent(props, width_key as *const c_void, &mut width_val) != 0
            {
                let mut w: i32 = 0;
                CFNumberGetValue(
                    width_val as CFNumberRef,
                    3,
                    &mut w as *mut i32 as *mut c_void,
                );
                println!("PixelWidth: {}", w);
            }
            CFRelease(width_key as CFTypeRef);

            // Read PixelHeight
            let height_key = CFStringCreateWithCString(
                kCFAllocatorDefault,
                c"PixelHeight".as_ptr(),
                kCFStringEncodingUTF8,
            );
            let mut height_val: *const c_void = ptr::null();
            if CFDictionaryGetValueIfPresent(props, height_key as *const c_void, &mut height_val)
                != 0
            {
                let mut h: i32 = 0;
                CFNumberGetValue(
                    height_val as CFNumberRef,
                    3,
                    &mut h as *mut i32 as *mut c_void,
                );
                println!("PixelHeight: {}", h);
            }
            CFRelease(height_key as CFTypeRef);
            CFRelease(props as CFTypeRef);
        }

        // Cleanup
        CFRelease(source as CFTypeRef);
        CFRelease(dest as CFTypeRef);
        CFRelease(mutable_data as CFTypeRef);
        CFRelease(png_type as CFTypeRef);
        CGImageRelease(image);
        CGDataProviderRelease(data_provider);
        CGColorSpaceRelease(colorspace);
    }

    println!("\nDone.");
}
