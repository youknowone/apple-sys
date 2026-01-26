//! Generate QuickLook thumbnails for files via generated bindings.
//!
//! Uses QLThumbnailGenerator and QLThumbnailGenerationRequest to
//! create a thumbnail request and query its properties.

use apple_sys::CoreFoundation::CGSize;
use apple_sys::QuickLookThumbnailing::*;

mod common;
use common::nsobj_to_string;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== QuickLook Thumbnails ===\n");

        // QLThumbnailGenerator
        let generator = QLThumbnailGenerator::sharedGenerator();
        println!(
            "QLThumbnailGenerator: {}",
            if generator.0.is_null() {
                "unavailable".to_string()
            } else {
                nsobj_to_string(generator.0)
            }
        );

        // Create a thumbnail request for a known file
        {
            let path = nsstring(c"/System/Library/CoreServices/Finder.app");
            let url = NSURL::fileURLWithPath_(path);

            let size = CGSize {
                width: 256.0,
                height: 256.0,
            };

            // representationTypes: 1 = icon, 2 = lowQualityThumbnail, 4 = thumbnail
            let req = QLThumbnailGenerationRequest::alloc();
            let req_ptr =
                IQLThumbnailGenerationRequest::initWithFileAtURL_size_scale_representationTypes_(
                    &req, url, size, 2.0, 7, // all representation types
                );

            if !req_ptr.is_null() {
                let req = QLThumbnailGenerationRequest(req_ptr);
                println!(
                    "\nQLThumbnailGenerationRequest: {}",
                    nsobj_to_string(req_ptr)
                );

                let req_size = IQLThumbnailGenerationRequest::size(&req);
                println!("  Size: {}x{}", req_size.width, req_size.height);

                let scale = IQLThumbnailGenerationRequest::scale(&req);
                println!("  Scale: {}", scale);

                let min_dim = IQLThumbnailGenerationRequest::minimumDimension(&req);
                println!("  Minimum dimension: {}", min_dim);

                let icon_mode = IQLThumbnailGenerationRequest::iconMode(&req).0;
                println!("  Icon mode: {}", icon_mode);

                let rep_types = IQLThumbnailGenerationRequest::representationTypes(&req);
                println!("  Representation types: {}", rep_types);
            } else {
                println!("  Failed to create QLThumbnailGenerationRequest");
            }
        }
    }

    println!("\nDone.");
}
