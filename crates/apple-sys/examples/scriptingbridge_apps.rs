//! List running applications via ScriptingBridge.
//!
//! Uses SBApplication and NSWorkspace to demonstrate
//! ScriptingBridge framework access.

use apple_sys::AppKit::*;
use apple_sys::ScriptingBridge::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ScriptingBridge ===\n");

        println!("SBApplication class: available");

        // runningApplications is a public @property on NSWorkspace (declared in
        // NSRunningApplication.h category), but not captured in bindings.
        let ws = NSWorkspace::sharedWorkspace();
        let apps: id = objc2::msg_send![ws.0, runningApplications];
        let apps_arr = NSArray(apps);
        let count = INSArray::<id>::count(&apps_arr);

        println!("Running applications: {}\n", count);

        // Show first 15 scriptable apps
        let mut shown = 0;
        for i in 0..count {
            if shown >= 15 {
                break;
            }
            let app_ptr = INSArray::<id>::objectAtIndex_(&apps_arr, i);
            let app = NSRunningApplication(app_ptr);
            let bundle_id = INSRunningApplication::bundleIdentifier(&app);
            let name = INSRunningApplication::localizedName(&app);
            let pid = INSRunningApplication::processIdentifier(&app);
            let active = INSRunningApplication::isActive(&app);

            let bid = nsstring_to_string(bundle_id);
            if bid.is_empty() {
                continue;
            }

            // Try to create SBApplication for this bundle ID
            let sb_app = SBApplication::applicationWithBundleIdentifier_(bundle_id);
            let scriptable = !sb_app.0.is_null();

            println!(
                "  {:3}. {} (pid={}) {}{}",
                shown + 1,
                nsstring_to_string(name),
                pid,
                if active.0 { "[active] " } else { "" },
                if scriptable { "[scriptable]" } else { "" }
            );
            shown += 1;
        }

        if count > 15 {
            println!("  ... and {} more", count - 15);
        }
    }

    println!("\nDone.");
}
