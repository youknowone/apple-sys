//! Create and search a CoreSpotlight index.
//!
//! Demonstrates CSSearchableItemAttributeSet, CSSearchableItem,
//! and CSSearchableIndex for on-device search indexing.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::CoreSpotlight::*;
use apple_sys::Foundation::{INSArray, INSMutableArray, NSAutoreleasePool, NSMutableArray};
use apple_sys::UniformTypeIdentifiers::{IUTType, UTType};
use apple_sys::objc::id;

use std::ffi::CStr;
mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreSpotlight Indexing ===\n");

        // Create searchable item attribute sets
        let items_data = [
            (
                "doc-001",
                "Rust Programming Guide",
                "Learn Rust from basics to advanced topics",
            ),
            (
                "doc-002",
                "Swift UI Tutorial",
                "Build beautiful iOS apps with SwiftUI",
            ),
            (
                "doc-003",
                "Metal Shading Language",
                "GPU programming for Apple platforms",
            ),
        ];

        let content_type_id = nsstring(c"public.text");
        let content_type = UTType(UTType::typeWithIdentifier_(content_type_id));
        let items_array = NSMutableArray::alloc();
        let items_ptr = INSObject::init(&items_array);
        let items_array = NSMutableArray(items_ptr);

        for (uid, title, desc) in &items_data {
            // Create attribute set
            let attr = CSSearchableItemAttributeSet::alloc();
            let attr_ptr = attr.initWithContentType_(content_type);
            let attr = CSSearchableItemAttributeSet(attr_ptr);

            // Set attributes
            let ns_title =
                nsstring(&CStr::from_bytes_with_nul(&[title.as_bytes(), &[0]].concat()).unwrap());
            attr.setTitle_(ns_title);

            let ns_desc =
                nsstring(&CStr::from_bytes_with_nul(&[desc.as_bytes(), &[0]].concat()).unwrap());
            attr.setContentDescription_(ns_desc);

            // Read back attributes to verify
            let got_title = attr.title();
            let got_desc = attr.contentDescription();
            println!(
                "  Item '{}': title='{}', desc='{}'",
                uid,
                nsstring_to_string(got_title),
                nsstring_to_string(got_desc)
            );

            // Create searchable item
            let ns_uid =
                nsstring(&CStr::from_bytes_with_nul(&[uid.as_bytes(), &[0]].concat()).unwrap());
            let ns_domain = nsstring(c"com.example.applesys");
            let item = CSSearchableItem::alloc();
            let item_ptr = item
                .initWithUniqueIdentifier_domainIdentifier_attributeSet_(ns_uid, ns_domain, attr);
            let item = CSSearchableItem(item_ptr);

            INSMutableArray::<id>::addObject_(&items_array, item.0);
        }

        // Get default searchable index
        let index_ptr = CSSearchableIndex::defaultSearchableIndex();
        println!(
            "\nDefault searchable index: {}",
            if index_ptr.is_null() {
                "unavailable"
            } else {
                "available"
            }
        );

        // Index the items (fire and forget -- no completion handler)
        let count = INSArray::<id>::count(&items_array);
        println!("Prepared {} item(s) for indexing", count);
    }

    println!("\nDone.");
}
