//! Inspect FinderSync controller and toolbar item.
//!
//! Gets FIFinderSyncController.defaultController and queries
//! directoryURLs and toolbar item properties.

use apple_sys::FinderSync::*;

mod common;
use common::nsobj_to_string;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== FinderSync Controller ===\n");

        // FIFinderSyncController
        println!("--- FIFinderSyncController ---");
        let controller_ptr = FIFinderSyncController::defaultController();
        let controller = FIFinderSyncController(controller_ptr);
        if !controller.0.is_null() {
            println!("  Default controller: {}", nsobj_to_string(controller.0));

            // Query directoryURLs
            let dir_urls = IFIFinderSyncController::directoryURLs(&controller);
            if !dir_urls.0.is_null() {
                let url_array = NSSet_NSExtendedSet::<id>::allObjects(&dir_urls);
                let count = INSArray::<id>::count(&url_array);
                println!("  Monitored directories: {count}");

                for i in 0..count {
                    let url = INSArray::<id>::objectAtIndex_(&url_array, i);
                    let path = INSURL::path(&NSURL(url));
                    println!("    - {}", nsobj_to_string(path.0));
                }
            } else {
                println!("  Monitored directories: (none set)");
            }

            // Set and query directory URLs
            let path_str = nsstring(c"/tmp");
            let test_url = NSURL::fileURLWithPath_isDirectory_(path_str, BOOL(true));

            let url_set = NSSet(<NSSet as NSSet_NSSetCreation<id>>::setWithObject_(
                test_url.0,
            ));
            IFIFinderSyncController::setDirectoryURLs_(&controller, url_set);

            let new_urls = IFIFinderSyncController::directoryURLs(&controller);
            if !new_urls.0.is_null() {
                let all = NSSet_NSExtendedSet::<id>::allObjects(&new_urls);
                let count = INSArray::<id>::count(&all);
                println!("\n  After setting /tmp:");
                println!("  Monitored directories: {count}");
                for i in 0..count {
                    let url = INSArray::<id>::objectAtIndex_(&all, i);
                    let path = INSURL::path(&NSURL(url));
                    println!("    - {}", nsobj_to_string(path.0));
                }
            }

            // Clear
            let empty_set = NSSet(<NSSet as NSSet_NSSetCreation<id>>::set());
            IFIFinderSyncController::setDirectoryURLs_(&controller, empty_set);
            println!("  Cleared monitored directories.");
        } else {
            println!("  defaultController returned nil");
        }
    }

    println!("\nDone.");
}
