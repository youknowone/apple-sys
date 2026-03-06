//! Generate a QuickLook thumbnail for a file.
//!
//! Demonstrates QLThumbnailImageCreate and QLThumbnail APIs.

#![allow(non_upper_case_globals)]

use apple_sys::CoreFoundation::*;
use apple_sys::CoreGraphics::*;
use apple_sys::QuickLook::*;
use std::ptr;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;

fn main() {
    unsafe {
        println!("=== QuickLook ===\n");

        // 1. Print CFTypeIDs
        let thumb_request_type_id = QLThumbnailRequestGetTypeID();
        let preview_request_type_id = QLPreviewRequestGetTypeID();
        let thumb_type_id = QLThumbnailGetTypeID();
        println!("QLThumbnailRequest CFTypeID: {}", thumb_request_type_id);
        println!("QLPreviewRequest CFTypeID:   {}", preview_request_type_id);
        println!("QLThumbnail CFTypeID:        {}", thumb_type_id);

        // 2. Create a file URL for /System/Library/CoreServices/Finder.app (always present)
        let path = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"/System/Library/CoreServices/Finder.app".as_ptr(),
            kCFStringEncodingUTF8,
        );
        let url = CFURLCreateWithFileSystemPath(
            kCFAllocatorDefault,
            path,
            0, // kCFURLPOSIXPathStyle
            1, // isDirectory = true
        );

        // 3. Try QLThumbnailImageCreate
        println!("\nGenerating thumbnail for Finder.app...");
        let max_size = CGSize {
            width: 128.0,
            height: 128.0,
        };
        let image = QLThumbnailImageCreate(kCFAllocatorDefault, url, max_size, ptr::null());
        if !image.is_null() {
            let width = CGImageGetWidth(image);
            let height = CGImageGetHeight(image);
            let bpp = CGImageGetBitsPerPixel(image);
            let bpc = CGImageGetBitsPerComponent(image);
            println!(
                "  Thumbnail created: {}x{} ({} bpp, {} bpc)",
                width, height, bpp, bpc
            );
            CGImageRelease(image);
        } else {
            println!("  QLThumbnailImageCreate returned null (no thumbnail available).");
        }

        // 4. Use QLThumbnailCreate + QLThumbnailCopyImage (async-capable API)
        println!("\nUsing QLThumbnailCreate...");
        let thumbnail = QLThumbnailCreate(kCFAllocatorDefault, url, max_size, ptr::null());
        if !thumbnail.is_null() {
            let doc_url = QLThumbnailCopyDocumentURL(thumbnail);
            if !doc_url.is_null() {
                let url_str = CFURLGetString(doc_url);
                let len = CFStringGetLength(url_str);
                let max = CFStringGetMaximumSizeForEncoding(len, kCFStringEncodingUTF8) + 1;
                let mut buf = vec![0u8; max as usize];
                if CFStringGetCString(url_str, buf.as_mut_ptr() as _, max, kCFStringEncodingUTF8)
                    != 0
                {
                    let cstr = std::ffi::CStr::from_ptr(buf.as_ptr() as _);
                    println!("  Document URL: {}", cstr.to_string_lossy());
                }
                CFRelease(doc_url as CFTypeRef);
            }

            let thumb_max = QLThumbnailGetMaximumSize(thumbnail);
            println!("  Maximum size: {}x{}", thumb_max.width, thumb_max.height);

            let cancelled = QLThumbnailIsCancelled(thumbnail);
            println!("  Cancelled: {}", cancelled != 0);

            // Try synchronous image copy
            let image = QLThumbnailCopyImage(thumbnail);
            if !image.is_null() {
                let w = CGImageGetWidth(image);
                let h = CGImageGetHeight(image);
                println!("  Image: {}x{}", w, h);
                CGImageRelease(image);
            } else {
                println!("  No synchronous image (would need dispatch for async).");
            }

            let content_rect = QLThumbnailGetContentRect(thumbnail);
            println!(
                "  Content rect: ({}, {}, {}, {})",
                content_rect.origin.x,
                content_rect.origin.y,
                content_rect.size.width,
                content_rect.size.height
            );

            CFRelease(thumbnail as CFTypeRef);
        } else {
            println!("  QLThumbnailCreate returned null.");
        }

        CFRelease(url as CFTypeRef);
        CFRelease(path as CFTypeRef);
    }

    println!("\nDone.");
}
