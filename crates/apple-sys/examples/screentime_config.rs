//! Verify ScreenTime bindings by exercising STWebpageController
//! and STWebHistory APIs.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{NSAutoreleasePool, NSError, NSString};
use apple_sys::ScreenTime::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ScreenTime Framework ===\n");

        // STWebpageController
        println!("--- STWebpageController ---");
        let webpage = STWebpageController::alloc();
        let webpage_ptr = INSObject::init(&webpage);
        if !webpage_ptr.is_null() {
            let webpage = STWebpageController(webpage_ptr);
            println!("STWebpageController: {}", nsobj_to_string(webpage_ptr));

            let suppressed = ISTWebpageController::suppressUsageRecording(&webpage);
            println!("  suppressUsageRecording: {}", suppressed.0);

            let url = ISTWebpageController::URL(&webpage);
            println!("  URL: {}", nsobj_to_string(url.0));

            let playing = ISTWebpageController::URLIsPlayingVideo(&webpage);
            println!("  URLIsPlayingVideo: {}", playing.0);

            let pip = ISTWebpageController::URLIsPictureInPicture(&webpage);
            println!("  URLIsPictureInPicture: {}", pip.0);

            let blocked = ISTWebpageController::URLIsBlocked(&webpage);
            println!("  URLIsBlocked: {}", blocked.0);

            let profile_id = ISTWebpageController::profileIdentifier(&webpage);
            println!("  profileIdentifier: {}", nsstring_to_string(profile_id));
        } else {
            println!("Failed to create STWebpageController.");
        }

        // STWebHistory
        println!("\n--- STWebHistory ---");
        let history = STWebHistory::alloc();
        let error: *mut NSError = std::ptr::null_mut();
        let history_ptr = ISTWebHistory::initWithBundleIdentifier_error_(
            &history,
            NSString(std::ptr::null_mut()),
            error,
        );
        if !history_ptr.is_null() {
            println!("STWebHistory: {}", nsobj_to_string(history_ptr));
        } else {
            println!("STWebHistory: requires valid bundle identifier to init");
        }

        // STScreenTimeConfiguration
        println!("\n--- STScreenTimeConfiguration ---");
        let config = STScreenTimeConfiguration::alloc();
        let config_ptr = INSObject::init(&config);
        if !config_ptr.is_null() {
            let config = STScreenTimeConfiguration(config_ptr);
            let enforces = ISTScreenTimeConfiguration::enforcesChildRestrictions(&config);
            println!("  enforcesChildRestrictions: {}", enforces.0);
        } else {
            println!("STScreenTimeConfiguration: requires observer context to init");
        }
    }

    println!("\nDone.");
}
