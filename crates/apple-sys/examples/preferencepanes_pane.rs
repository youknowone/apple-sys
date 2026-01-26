//! Create NSPreferencePane and inspect its properties via generated bindings.
//!
//! Demonstrates PreferencePanes framework by creating an NSPreferencePane
//! instance with the main bundle and querying its properties.

use apple_sys::PreferencePanes::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== PreferencePanes ===\n");

        // NSPreferencePane
        println!("--- NSPreferencePane ---");
        {
            // Create an instance with the main bundle
            let main_bundle = NSBundle::mainBundle();

            let pane = NSPreferencePane::alloc();
            let pane = NSPreferencePane(INSPreferencePane::initWithBundle_(&pane, main_bundle));

            if !pane.0.is_null() {
                println!("  Instance: {}", nsobj_to_string(pane.0));

                // Check bundle
                let bundle = INSPreferencePane::bundle(&pane);
                if !bundle.0.is_null() {
                    let bundle_id = INSBundle::bundleIdentifier(&bundle);
                    println!("  Bundle ID: {}", nsstring_to_string(bundle_id));
                }

                // Check main view (will be nil without a nib)
                let main_view = INSPreferencePane::mainView(&pane);
                println!(
                    "  Main view: {}",
                    if main_view.0.is_null() {
                        "nil (no nib loaded)"
                    } else {
                        "loaded"
                    }
                );

                // Check if selected
                let is_selected = INSPreferencePane::isSelected(&pane).0;
                println!("  Selected: {}", is_selected);
            } else {
                println!("  Failed to create instance (expected without proper bundle)");
            }
        }
    }

    println!("\nDone.");
}
