//! Inspect CallKit call directory manager and call observer.
//!
//! Gets CXCallDirectoryManager shared instance, creates CXCallController
//! and CXCallObserver, and queries active calls.

use apple_sys::CallKit::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CallKit Directory Manager ===\n");

        // CXCallDirectoryManager
        let mgr = CXCallDirectoryManager::sharedInstance();
        if mgr.0.is_null() {
            println!("Failed to get CXCallDirectoryManager.sharedInstance.");
            return;
        }
        println!("CXCallDirectoryManager.sharedInstance obtained.");
        println!("Description: {}", nsobj_to_string(mgr.0));

        // CXCallController
        println!("\n--- CXCallController ---");
        let ctrl = CXCallController::alloc();
        let ctrl_ptr = INSObject::init(&ctrl);
        if !ctrl_ptr.is_null() {
            let ctrl = CXCallController(ctrl_ptr);
            println!("CXCallController created.");

            let call_observer = ICXCallController::callObserver(&ctrl);
            if !call_observer.0.is_null() {
                let calls = ICXCallObserver::calls(&call_observer);
                if !calls.0.is_null() {
                    let count = INSArray::<id>::count(&calls);
                    println!("Active calls: {count}");
                }
            }
        }

        // CXCallObserver
        println!("\n--- CXCallObserver ---");
        let obs = CXCallObserver::alloc();
        let obs_ptr = INSObject::init(&obs);
        if !obs_ptr.is_null() {
            let obs = CXCallObserver(obs_ptr);
            let calls = ICXCallObserver::calls(&obs);
            if !calls.0.is_null() {
                let count = INSArray::<id>::count(&calls);
                println!("Observed calls: {count}");
            }
        }

        // CXProviderConfiguration
        println!("\n--- CXProviderConfiguration ---");
        let config = CXProviderConfiguration::alloc();
        let config_ptr = INSObject::init(&config);
        if !config_ptr.is_null() {
            let config = CXProviderConfiguration(config_ptr);
            let max_calls = ICXProviderConfiguration::maximumCallsPerCallGroup(&config);
            let max_groups = ICXProviderConfiguration::maximumCallGroups(&config);
            let supports_video = ICXProviderConfiguration::supportsVideo(&config);
            println!("CXProviderConfiguration created.");
            println!("  Max calls per group: {max_calls}");
            println!("  Max call groups: {max_groups}");
            println!("  Supports video: {}", supports_video.0);
        }
    }

    println!("\nDone.");
}
