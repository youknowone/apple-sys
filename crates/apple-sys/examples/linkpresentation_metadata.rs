//! Create and inspect LinkPresentation metadata.
//!
//! Uses LPLinkMetadata to create rich link previews
//! with custom titles and URLs.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSURL, NSAutoreleasePool, NSURL};
use apple_sys::LinkPresentation::*;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== LinkPresentation ===\n");

        let urls = [
            (c"https://www.apple.com", c"Apple"),
            (c"https://developer.apple.com", c"Apple Developer"),
            (c"https://www.rust-lang.org", c"Rust Programming Language"),
        ];

        for (url_str, title) in &urls {
            let metadata = LPLinkMetadata::alloc();
            let metadata_ptr = INSObject::init(&metadata);
            let metadata = LPLinkMetadata(metadata_ptr);

            // Set URL
            let ns_url_str = nsstring(url_str);
            let url = NSURL(NSURL::URLWithString_(ns_url_str));
            ILPLinkMetadata::setURL_(&metadata, url);

            // Set original URL
            ILPLinkMetadata::setOriginalURL_(&metadata, url);

            // Set title
            let ns_title = nsstring(title);
            ILPLinkMetadata::setTitle_(&metadata, ns_title);

            // Read back
            let got_url = ILPLinkMetadata::URL(&metadata);
            let got_title = ILPLinkMetadata::title(&metadata);
            let got_orig = ILPLinkMetadata::originalURL(&metadata);
            let icon = ILPLinkMetadata::iconProvider(&metadata);
            let image = ILPLinkMetadata::imageProvider(&metadata);

            let url_desc = INSURL::absoluteString(&got_url);
            let orig_desc = INSURL::absoluteString(&got_orig);

            println!("Link: {}", nsstring_to_string(got_title));
            println!("  URL:          {}", nsstring_to_string(url_desc));
            println!("  Original URL: {}", nsstring_to_string(orig_desc));
            println!(
                "  Icon:         {}",
                if icon.0.is_null() { "none" } else { "set" }
            );
            println!(
                "  Image:        {}",
                if image.0.is_null() { "none" } else { "set" }
            );
            println!();
        }
    }

    println!("Done.");
}
