//! Query app tracking transparency authorization status.
//!
//! Uses ATTrackingManager to read the current trackingAuthorizationStatus.

use apple_sys::AppTrackingTransparency::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== App Tracking Transparency ===\n");

        let status = ATTrackingManager::trackingAuthorizationStatus();
        let status_name = match status {
            0 => "Not Determined",
            1 => "Restricted",
            2 => "Denied",
            3 => "Authorized",
            _ => "Unknown",
        };

        println!("Tracking authorization status: {status_name} ({status})");

        match status {
            0 => println!("\nThe user has not yet been asked for tracking permission."),
            1 => println!("\nTracking is restricted (e.g., parental controls)."),
            2 => println!("\nThe user has denied tracking permission."),
            3 => println!("\nThe user has authorized tracking."),
            _ => println!("\nUnexpected status value."),
        }
    }

    println!("\nDone.");
}
