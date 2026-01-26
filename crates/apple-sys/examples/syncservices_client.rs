//! Verify SyncServices bindings by getting the shared ISyncManager
//! and querying its state.

use apple_sys::SyncServices::*;

mod common;
use common::nsobj_to_string;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SyncServices Client ===\n");

        // ISyncManager
        println!("--- ISyncManager ---");
        let manager = ISyncManager::sharedManager();
        if !manager.0.is_null() {
            println!("Shared manager: {}", nsobj_to_string(manager.0));

            let enabled = IISyncManager::isEnabled(&manager);
            println!("  isEnabled: {}", enabled.0);

            let disabled_reason = IISyncManager::syncDisabledReason(&manager);
            println!(
                "  syncDisabledReason: {}",
                nsobj_to_string(disabled_reason.0)
            );

            let modes = IISyncManager::requestModes(&manager);
            if !modes.0.is_null() {
                let count = INSArray::<id>::count(&modes);
                println!("  requestModes count: {count}");
                for i in 0..count.min(10) {
                    let mode = INSArray::<id>::objectAtIndex_(&modes, i);
                    println!("    [{i}] {}", nsobj_to_string(mode));
                }
            } else {
                println!("  requestModes: (null)");
            }

            // Try to get a client by identifier (will be null if not registered)
            let test_id = nsstring(c"com.example.test-sync-client");
            let client = IISyncManager::clientWithIdentifier_(&manager, test_id);
            println!(
                "  clientWithIdentifier (test): {}",
                nsobj_to_string(client.0)
            );
        } else {
            println!("Failed to get shared manager.");
        }
    }

    println!("\nDone.");
}
