//! Verify SharedWithYou bindings by creating an SWHighlightCenter,
//! querying highlights, and checking class method availability.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::SharedWithYou::*;
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SharedWithYou Highlights ===\n");

        // SWHighlightCenter
        println!("--- SWHighlightCenter ---");
        let center = SWHighlightCenter::alloc();
        let center_ptr = INSObject::init(&center);
        if !center_ptr.is_null() {
            let center = SWHighlightCenter(center_ptr);
            println!("Center: {}", nsobj_to_string(center_ptr));

            let highlights = ISWHighlightCenter::highlights(&center);
            if !highlights.0.is_null() {
                let count = INSArray::<id>::count(&highlights);
                println!("  highlights count: {count}");

                for i in 0..count.min(5) {
                    let highlight_ptr = INSArray::<id>::objectAtIndex_(&highlights, i);
                    let highlight = SWHighlight(highlight_ptr);
                    let url = ISWHighlight::URL(&highlight);
                    println!("  [{i}] URL: {}", nsobj_to_string(url.0));
                }
                if count > 5 {
                    println!("  ... and {} more", count - 5);
                }
            } else {
                println!("  highlights: (null)");
            }

            let title = SWHighlightCenter::highlightCollectionTitle();
            println!("  highlightCollectionTitle: {}", nsstring_to_string(title));

            let collab_available = SWHighlightCenter::isSystemCollaborationSupportAvailable();
            println!(
                "  isSystemCollaborationSupportAvailable: {}",
                collab_available.0
            );

            let delegate = ISWHighlightCenter::delegate(&center);
            println!("  delegate: {:?}", delegate);
        } else {
            println!("Failed to create SWHighlightCenter.");
        }

        // SWAttributionView
        println!("\n--- SWAttributionView ---");
        let view = SWAttributionView::alloc();
        let view_ptr = INSObject::init(&view);
        if !view_ptr.is_null() {
            let view = SWAttributionView(view_ptr);
            println!("AttributionView: {}", nsobj_to_string(view_ptr));

            let display_ctx = ISWAttributionView::displayContext(&view);
            println!("  displayContext: {display_ctx}");

            let bg_style = ISWAttributionView::backgroundStyle(&view);
            println!("  backgroundStyle: {bg_style}");

            let enables_marquee = ISWAttributionView::enablesMarquee(&view);
            println!("  enablesMarquee: {}", enables_marquee.0);

            let max_width = ISWAttributionView::preferredMaxLayoutWidth(&view);
            println!("  preferredMaxLayoutWidth: {max_width}");
        } else {
            println!("Failed to create SWAttributionView.");
        }
    }

    println!("\nDone.");
}
