//! Query NCWidgetController and notification center classes via generated bindings.
//!
//! Gets NCWidgetController widgetController singleton and creates
//! NCWidgetListViewController / NCWidgetSearchViewController to
//! test their property accessors.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::NotificationCenter::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== NotificationCenter ===\n");

        // NCWidgetController
        println!("--- NCWidgetController ---");
        let controller = NCWidgetController(NCWidgetController::widgetController());
        if !controller.0.is_null() {
            println!("  Controller: {}", nsobj_to_string(controller.0));
        } else {
            println!("  widgetController returned nil");
        }

        // Also try defaultWidgetController
        let default_ctrl = NCWidgetController::defaultWidgetController();
        println!(
            "  defaultWidgetController: {}",
            if default_ctrl.0.is_null() {
                "nil"
            } else {
                "available"
            }
        );

        // NCWidgetListViewController
        println!("\n--- NCWidgetListViewController ---");
        {
            let vc = NCWidgetListViewController::alloc();
            let vc_ptr = INSObject::init(&vc);
            if !vc_ptr.is_null() {
                let vc = NCWidgetListViewController(vc_ptr);
                println!("  Instance: {}", nsobj_to_string(vc_ptr));

                let contents = INCWidgetListViewController::contents(&vc);
                println!(
                    "  Contents: {}",
                    if contents.0.is_null() { "nil" } else { "set" }
                );

                let min_rows = INCWidgetListViewController::minimumVisibleRowCount(&vc);
                println!("  Minimum visible row count: {}", min_rows);

                let has_dividers = INCWidgetListViewController::hasDividerLines(&vc).0;
                println!("  Has divider lines: {}", has_dividers);

                let editing = INCWidgetListViewController::editing(&vc).0;
                println!("  Editing: {}", editing);

                let shows_add = INCWidgetListViewController::showsAddButtonWhenEditing(&vc).0;
                println!("  Shows add button when editing: {}", shows_add);

                let delegate = INCWidgetListViewController::delegate(&vc);
                println!(
                    "  Delegate: {}",
                    if delegate.is_null() { "nil" } else { "set" }
                );
            } else {
                println!("  Failed to create instance");
            }
        }

        // NCWidgetSearchViewController
        println!("\n--- NCWidgetSearchViewController ---");
        {
            let vc = NCWidgetSearchViewController::alloc();
            let vc_ptr = INSObject::init(&vc);
            if !vc_ptr.is_null() {
                let vc = NCWidgetSearchViewController(vc_ptr);
                println!("  Instance: {}", nsobj_to_string(vc_ptr));

                let search_desc = INCWidgetSearchViewController::searchDescription(&vc);
                println!(
                    "  Search description: {}",
                    if search_desc.0.is_null() {
                        "nil".to_string()
                    } else {
                        common::nsstring_to_string(search_desc)
                    }
                );

                let placeholder =
                    INCWidgetSearchViewController::searchResultsPlaceholderString(&vc);
                println!(
                    "  Search results placeholder: {}",
                    if placeholder.0.is_null() {
                        "nil".to_string()
                    } else {
                        common::nsstring_to_string(placeholder)
                    }
                );

                let results = INCWidgetSearchViewController::searchResults(&vc);
                println!(
                    "  Search results: {}",
                    if results.0.is_null() { "nil" } else { "set" }
                );

                let delegate = INCWidgetSearchViewController::delegate(&vc);
                println!(
                    "  Delegate: {}",
                    if delegate.is_null() { "nil" } else { "set" }
                );
            } else {
                println!("  Failed to create instance");
            }
        }
    }

    println!("\nDone.");
}
