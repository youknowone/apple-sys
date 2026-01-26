//! Query smart card readers via CryptoTokenKit.
//!
//! Uses TKSmartCardSlotManager to enumerate available
//! smart card reader slots.

use apple_sys::CryptoTokenKit::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CryptoTokenKit Smart Card ===\n");

        // TKSmartCardSlotManager
        let mgr = TKSmartCardSlotManager::defaultManager();
        if !mgr.0.is_null() {
            let slot_names = ITKSmartCardSlotManager::slotNames(&mgr);
            let count = INSArray::<id>::count(&slot_names);
            println!("Smart card slots: {}", count);

            for i in 0..count {
                let name_id = INSArray::<id>::objectAtIndex_(&slot_names, i);
                println!("  {}. {}", i + 1, nsstring_to_string(NSString(name_id)));
            }

            if count == 0 {
                println!("  (No smart card readers connected)");
            }
        } else {
            println!("TKSmartCardSlotManager not available");
        }

        // TKTokenWatcher
        let watcher = TKTokenWatcher::alloc();
        let watcher_ptr = INSObject::init(&watcher);
        if !watcher_ptr.is_null() {
            let watcher = TKTokenWatcher(watcher_ptr);
            let token_ids = ITKTokenWatcher::tokenIDs(&watcher);
            let count = INSArray::<id>::count(&token_ids);
            println!("\nToken IDs ({}):", count);
            for i in 0..count {
                let tid = INSArray::<id>::objectAtIndex_(&token_ids, i);
                println!("  {}", nsstring_to_string(NSString(tid)));
            }
            if count == 0 {
                println!("  (No tokens present)");
            }
        }
    }

    println!("\nDone.");
}
