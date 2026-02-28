//! Create MLMediaLibrary and query media sources.
//!
//! Demonstrates MediaLibrary framework by creating an MLMediaLibrary
//! instance with options and querying its media sources.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{
    INSArray, INSDictionary, NSAutoreleasePool, NSDictionary, NSDictionary_NSExtendedDictionary,
    NSString,
};
use apple_sys::MediaLibrary::*;
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MediaLibrary ===\n");

        // Create MLMediaLibrary with empty options
        let empty_dict = NSDictionary(INSObject::init(&NSDictionary::alloc()));

        let lib = MLMediaLibrary::alloc();
        let lib = MLMediaLibrary(IMLMediaLibrary::initWithOptions_(&lib, empty_dict));

        if !lib.0.is_null() {
            println!("MLMediaLibrary: {}", nsobj_to_string(lib.0));

            // Query media sources (returns NSDictionary of MLMediaSource)
            let sources = IMLMediaLibrary::mediaSources(&lib);
            if !sources.0.is_null() {
                let count = INSDictionary::<NSString, id>::count(&sources);
                println!("Media sources found: {}", count);

                let keys = NSDictionary_NSExtendedDictionary::<NSString, id>::allKeys(&sources);
                let key_count = INSArray::<id>::count(&keys);
                for i in 0..key_count {
                    let key = INSArray::<id>::objectAtIndex_(&keys, i);
                    let source = INSDictionary::<NSString, id>::objectForKey_(&sources, key);
                    let source = MLMediaSource(source);
                    let source_id = IMLMediaSource::mediaSourceIdentifier(&source);
                    println!(
                        "  Source: {} -> {}",
                        nsstring_to_string(NSString(key)),
                        nsstring_to_string(source_id)
                    );

                    // Query source attributes
                    let attrs = IMLMediaSource::attributes(&source);
                    if !attrs.0.is_null() {
                        let attr_count = INSDictionary::<NSString, id>::count(&attrs);
                        println!("    Attributes: {attr_count} entries");
                    }

                    // Query root media group
                    let root = IMLMediaSource::rootMediaGroup(&source);
                    if !root.0.is_null() {
                        let group_name = IMLMediaGroup::name(&root);
                        println!("    Root group: {}", nsstring_to_string(group_name));

                        let children = IMLMediaGroup::childGroups(&root);
                        if !children.0.is_null() {
                            let child_count = INSArray::<id>::count(&children);
                            println!("    Child groups: {child_count}");
                        }
                    }
                }
            } else {
                println!("Media sources: (nil - loading asynchronously)");
                println!("  Note: MLMediaLibrary loads sources asynchronously.");
                println!("  In a real app, observe MLMediaLibraryDidLoad notification.");
            }
        } else {
            println!("Failed to create MLMediaLibrary");
        }
    }

    println!("\nDone.");
}
