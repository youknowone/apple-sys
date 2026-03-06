//! Query the advertising identifier using AdSupport framework.
//!
//! Gets the ASIdentifierManager sharedManager and reads the advertising
//! identifier and tracking enabled status.

use apple_sys::AdSupport::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AdSupport Identifier ===\n");

        let mgr = ASIdentifierManager::sharedManager();
        if mgr.0.is_null() {
            println!("Failed to get shared ASIdentifierManager.");
            return;
        }

        let tracking_enabled = IASIdentifierManager::isAdvertisingTrackingEnabled(&mgr);
        println!("Advertising tracking enabled: {}", tracking_enabled.0);

        let ad_id = IASIdentifierManager::advertisingIdentifier(&mgr);
        let id_str = nsobj_to_string(ad_id.0);
        println!("Advertising identifier:       {id_str}");

        // Check if identifier is all zeros (limited tracking)
        if id_str.contains("00000000-0000-0000-0000-000000000000") {
            println!("\nNote: All-zero identifier indicates tracking is limited/denied.");
        }
    }

    println!("\nDone.");
}
