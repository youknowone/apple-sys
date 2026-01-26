//! Inspect AVRouting custom routing controller.
//!
//! Creates an AVCustomRoutingController and examines its properties.

use apple_sys::AVRouting::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AVRouting Controller ===\n");

        let ctrl = AVCustomRoutingController::alloc();
        let ctrl_ptr = INSObject::init(&ctrl);
        if ctrl_ptr.is_null() {
            println!("Failed to create AVCustomRoutingController.");
            return;
        }
        let ctrl = AVCustomRoutingController(ctrl_ptr);
        println!("AVCustomRoutingController created.");

        // Query known route IPs
        let known_ips = IAVCustomRoutingController::knownRouteIPs(&ctrl);
        if !known_ips.0.is_null() {
            let count = INSArray::<id>::count(&known_ips);
            println!("Known route IPs: {count}");
            for i in 0..count {
                let ip: id = INSArray::<id>::objectAtIndex_(&known_ips, i);
                println!("  IP #{}: {}", i + 1, nsobj_to_string(ip));
            }
        } else {
            println!("Known route IPs: (none)");
        }

        // Check authorized routes
        let auth_routes = IAVCustomRoutingController::authorizedRoutes(&ctrl);
        if !auth_routes.0.is_null() {
            let count = INSArray::<id>::count(&auth_routes);
            println!("Authorized routes: {count}");
            for i in 0..count {
                let route: id = INSArray::<id>::objectAtIndex_(&auth_routes, i);
                println!("  Route #{}: {}", i + 1, nsobj_to_string(route));
            }
        } else {
            println!("Authorized routes: (none)");
        }

        // Check custom action items
        let custom_items = IAVCustomRoutingController::customActionItems(&ctrl);
        if !custom_items.0.is_null() {
            let count = INSArray::<id>::count(&custom_items);
            println!("Custom action items: {count}");
        } else {
            println!("Custom action items: (none)");
        }

        // Delegate
        let delegate = IAVCustomRoutingController::delegate(&ctrl);
        println!(
            "Delegate: {}",
            if (delegate as *const u64).is_null() {
                "not set"
            } else {
                "set"
            }
        );
    }

    println!("\nDone.");
}
