//! Access the ClassKit shared data store.
//!
//! Gets CLSDataStore shared instance, inspects the main app context,
//! and queries active context and delegate state.

use apple_sys::ClassKit::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ClassKit Data Store ===\n");

        let store = CLSDataStore::shared();
        if store.0.is_null() {
            println!("Failed to get CLSDataStore.shared.");
            return;
        }
        println!("CLSDataStore.shared obtained.");
        println!("Description: {}", nsobj_to_string(store.0));

        // Check main app context
        let main_ctx = ICLSDataStore::mainAppContext(&store);
        if !main_ctx.0.is_null() {
            println!("\n--- Main App Context ---");
            let title = ICLSContext::title(&main_ctx);
            let identifier = ICLSContext::identifier(&main_ctx);
            let ctx_type = ICLSContext::type_(&main_ctx);
            println!("  Title:      {}", nsstring_to_string(title));
            println!("  Identifier: {}", nsstring_to_string(identifier));
            println!("  Type:       {ctx_type}");

            let is_active = ICLSContext::isActive(&main_ctx);
            println!("  Active:     {}", is_active.0);
        } else {
            println!("\nNo main app context (ClassKit may not be configured for this app).");
        }

        // Check active context
        let active = ICLSDataStore::activeContext(&store);
        if !active.0.is_null() {
            println!("\nActive context: {}", nsobj_to_string(active.0));
        } else {
            println!("No active context.");
        }

        // Delegate
        let delegate = ICLSDataStore::delegate(&store);
        println!(
            "Delegate: {}",
            if (delegate as *const u64).is_null() {
                "not set"
            } else {
                "set"
            }
        );

        // Running app context
        let running = ICLSDataStore::runningActivity(&store);
        if !running.0.is_null() {
            println!("Running activity: {}", nsobj_to_string(running.0));
        } else {
            println!("No running activity.");
        }
    }

    println!("\nDone.");
}
