//! List music library albums via iTunesLibrary.framework.
//!
//! Retrieves all media items from the music library and groups
//! them by album, showing track counts.

use apple_sys::iTunesLibrary::*;
use std::collections::BTreeMap;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== iTunes Library Albums ===\n");

        // Initialize ITLibrary
        let error: *mut NSError = std::ptr::null_mut();
        let lib_ptr =
            ITLibrary::libraryWithAPIVersion_error_(NSString(c"1.1".as_ptr() as id), error);

        if lib_ptr.is_null() {
            if !error.is_null() {
                let desc = INSError::localizedDescription(&NSError(error as id));
                eprintln!("Failed to open library: {}", nsstring_to_string(desc));
            } else {
                eprintln!("Failed to open library (unknown error)");
            }
            return;
        }

        let lib = ITLibrary(lib_ptr);

        // Get all media items
        let items = IITLibrary::allMediaItems(&lib);
        let count = INSArray::<id>::count(&items);
        println!("Total media items: {}", count);

        // Group by album title
        let mut albums: BTreeMap<String, (String, u32)> = BTreeMap::new();
        for i in 0..count {
            let item_id = INSArray::<id>::objectAtIndex_(&items, i);
            let item = ITLibMediaItem(item_id);
            let album = IITLibMediaItem::album(&item);
            if album.0.is_null() {
                continue;
            }
            let title = IITLibAlbum::title(&album);
            let title_str = nsstring_to_string(title);
            if title_str.is_empty() {
                continue;
            }
            let artist = IITLibAlbum::albumArtist(&album);
            let artist_str = nsstring_to_string(artist);

            let entry = albums.entry(title_str).or_insert((artist_str, 0));
            entry.1 += 1;
        }

        println!("Unique albums: {}\n", albums.len());

        // Print first 30 albums
        for (i, (title, (artist, tracks))) in albums.iter().enumerate() {
            if i >= 30 {
                println!("  ... and {} more", albums.len() - 30);
                break;
            }
            let artist_display = if artist.is_empty() {
                "Unknown Artist"
            } else {
                artist.as_str()
            };
            println!(
                "  {:3}. {} - {} ({} tracks)",
                i + 1,
                artist_display,
                title,
                tracks
            );
        }
    }

    println!("\nDone.");
}
