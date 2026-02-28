//! Test app that initializes the Apple Matter framework and lists
//! any commissioned Matter accessories via MTRDeviceControllerFactory.
//!
//! apple-sys provides framework linking and type definitions.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::objc::id;

use apple_sys::Matter::*;

mod common;
use common::nsobj_to_string;

fn main() {
    println!("=== Matter Device Controller Factory Test ===\n");

    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        // Get the shared factory singleton
        let factory = MTRDeviceControllerFactory::sharedInstance();
        println!("Factory: {}", nsobj_to_string(factory.0));

        // 2. Check if the factory is already running
        let is_running = IMTRDeviceControllerFactory::isRunning(&factory).0;
        println!("Factory running: {is_running}");

        // 3. Query known fabrics (available even when factory is not running)
        let fabrics = IMTRDeviceControllerFactory::knownFabrics(&factory);
        if !fabrics.0.is_null() {
            let fabric_count = INSArray::<id>::count(&fabrics);
            println!("Known fabrics: {fabric_count}");
        }

        // Starting the factory requires an MTRStorage-conforming object.
        // CLI apps without entitlements cannot create controllers, so
        // we only inspect the current state.

        if !is_running {
            println!("Factory is not running.");
            println!("This is expected for CLI apps without proper Matter entitlements.");
        }
    }

    println!("\nDone.");
}
