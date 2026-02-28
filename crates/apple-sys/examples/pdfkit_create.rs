//! Create a PDF document with PDFKit.
//!
//! Demonstrates PDFDocument, PDFPage creation, and metadata access.

use apple_sys::CoreFoundation::{INSObject, PNSObject};
use apple_sys::Foundation::{
    INSArray, INSData, INSMutableDictionary, NSArray, NSAutoreleasePool, NSDictionary,
    NSDictionary_NSExtendedDictionary, NSMutableDictionary, NSString,
};
use apple_sys::PDFKit::*;
use apple_sys::objc::id;
mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== PDFKit Document ===\n");

        // Create an empty PDFDocument
        let doc = PDFDocument::alloc();
        let doc = PDFDocument(INSObject::init(&doc));
        assert!(!doc.0.is_null(), "Failed to create PDFDocument");

        let page_count = IPDFDocument::pageCount(&doc);
        println!("Empty document pages: {}", page_count);

        // Create pages and insert them
        for i in 0..3 {
            let page = PDFPage::alloc();
            let page = PDFPage(INSObject::init(&page));
            assert!(!page.0.is_null(), "Failed to create PDFPage");

            IPDFDocument::insertPage_atIndex_(&doc, page, i as u64);
        }

        let page_count = IPDFDocument::pageCount(&doc);
        println!("After adding pages: {}", page_count);

        // Read page info
        for i in 0..page_count {
            let page = IPDFDocument::pageAtIndex_(&doc, i);
            let label = IPDFPage::label(&page);
            let desc = PNSObject::description(&page);
            println!(
                "  Page {}: label='{}' ({})",
                i,
                nsstring_to_string(label),
                nsstring_to_string(desc)
            );
        }

        // Document attributes
        let attrs = IPDFDocument::documentAttributes(&doc);
        if !attrs.0.is_null() {
            let keys = <NSDictionary as NSDictionary_NSExtendedDictionary<id, id>>::allKeys(&attrs);
            let key_count = <NSArray as INSArray<id>>::count(&keys);
            println!("\nDocument attributes: {}", key_count);
            for i in 0..key_count {
                let key = <NSArray as INSArray<id>>::objectAtIndex_(&keys, i);
                println!("  - {}", nsstring_to_string(NSString(key as _)));
            }
        }

        // Set document attributes
        let new_attrs = NSMutableDictionary::alloc();
        let new_attrs = NSMutableDictionary(INSObject::init(&new_attrs));
        let title_key = nsstring(c"Title");
        let title_val = nsstring(c"apple-sys Demo PDF");
        <NSMutableDictionary as INSMutableDictionary<id, id>>::setObject_forKey_(
            &new_attrs,
            title_val.0,
            title_key.0 as _,
        );
        let author_key = nsstring(c"Author");
        let author_val = nsstring(c"apple-sys");
        <NSMutableDictionary as INSMutableDictionary<id, id>>::setObject_forKey_(
            &new_attrs,
            author_val.0,
            author_key.0 as _,
        );
        IPDFDocument::setDocumentAttributes_(&doc, NSDictionary(new_attrs.0));

        println!("\nAttributes set: Title='apple-sys Demo PDF', Author='apple-sys'");

        // Check if we can write
        let data = IPDFDocument::dataRepresentation(&doc);
        if !data.0.is_null() {
            let len = INSData::length(&data);
            println!("PDF data size: {} bytes", len);
        }
    }

    println!("\nDone.");
}
