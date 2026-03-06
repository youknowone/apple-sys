//! Check CoreLocation authorization and services.
//!
//! Uses CLLocationManager to query location services status,
//! authorization, and heading availability.

use apple_sys::CoreData::CLLocationCoordinate2D;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::CoreLocation::*;
use apple_sys::Foundation::{NSAutoreleasePool, NSString, NSString_NSStringExtensionMethods};
#[allow(unused_imports)]
use apple_sys::objc::BOOL;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreLocation ===\n");

        // Location services enabled (class method)
        let enabled = CLLocationManager::class_locationServicesEnabled();
        println!("Location services enabled: {}", enabled.0);

        // Heading available
        let heading = CLLocationManager::class_headingAvailable();
        println!("Heading available: {}", heading.0);

        // Significant location change monitoring
        let sig_change = CLLocationManager::significantLocationChangeMonitoringAvailable();
        println!("Significant location change: {}", sig_change.0);

        // Create a CLCircularRegion to test region monitoring
        let center = CLLocationCoordinate2D {
            latitude: 37.7749,
            longitude: -122.4194,
        };
        let region = CLCircularRegion::alloc();
        let region_ptr =
            ICLCircularRegion::initWithCenter_radius_identifier_(&region, center, 100.0, {
                let s = NSString::alloc();
                NSString(s.initWithUTF8String_(c"test-region".as_ptr()))
            });
        if !region_ptr.is_null() {
            let region = CLCircularRegion(region_ptr);
            let r = ICLCircularRegion::radius(&region);
            let c = ICLCircularRegion::center(&region);
            println!("\nCLCircularRegion created:");
            println!("  Center: ({:.4}, {:.4})", c.latitude, c.longitude);
            println!("  Radius: {r:.1}m");

            // Test containsCoordinate
            let inside = ICLRegion::containsCoordinate_(&region, center);
            println!("  Contains center point: {}", inside.0);
        }

        // Create a manager instance
        let manager = CLLocationManager::alloc();
        let manager_ptr = INSObject::init(&manager);
        if !manager_ptr.is_null() {
            let manager = CLLocationManager(manager_ptr);
            let auth = ICLLocationManager::authorizationStatus(&manager);
            let auth_name = match auth {
                0 => "Not Determined",
                1 => "Restricted",
                2 => "Denied",
                3 => "Authorized Always",
                4 => "Authorized When In Use",
                _ => "Unknown",
            };
            println!("\nAuthorization status: {} ({})", auth_name, auth);

            let accuracy = ICLLocationManager::desiredAccuracy(&manager);
            println!("Desired accuracy: {:.1}m", accuracy);

            let filter = ICLLocationManager::distanceFilter(&manager);
            println!("Distance filter: {:.1}m", filter);
        }
    }

    println!("\nDone.");
}
