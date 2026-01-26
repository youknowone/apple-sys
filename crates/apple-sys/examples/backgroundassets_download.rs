//! Inspect BackgroundAssets download manager.
//!
//! Accesses BADownloadManager shared instance and queries current downloads.

use apple_sys::BackgroundAssets::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== BackgroundAssets Download Manager ===\n");

        let mgr = BADownloadManager::sharedManager();
        if mgr.0.is_null() {
            println!("Failed to get BADownloadManager.sharedManager.");
            return;
        }
        println!("BADownloadManager.sharedManager obtained.");
        println!("Description: {}", nsobj_to_string(mgr.0));

        // Try to fetch current downloads
        let error = NSError(std::ptr::null_mut());
        let downloads = IBADownloadManager::fetchCurrentDownloads_(
            &mgr,
            &error as *const NSError as *mut NSError,
        );

        if !downloads.0.is_null() {
            let count = INSArray::<id>::count(&downloads);
            println!("\nCurrent downloads: {count}");
            for i in 0..count {
                let dl: id = INSArray::<id>::objectAtIndex_(&downloads, i);
                let dl = BADownload(dl);
                let identifier = IBADownload::identifier(&dl);
                let state = IBADownload::state(&dl);
                let state_name = match state {
                    0 => "created",
                    1 => "downloading",
                    2 => "finished",
                    3 => "failed",
                    _ => "unknown",
                };
                println!(
                    "  Download: {} (state: {state_name})",
                    nsstring_to_string(identifier)
                );
            }
        } else if !error.0.is_null() {
            let desc = INSError::localizedDescription(&error);
            println!("Error fetching downloads: {}", nsstring_to_string(desc));
        } else {
            println!("No current downloads.");
        }

        // Check delegate
        let delegate = IBADownloadManager::delegate(&mgr);
        println!(
            "\nDelegate: {}",
            if (delegate as *const u64).is_null() {
                "not set"
            } else {
                "set"
            }
        );
    }

    println!("\nDone.");
}
