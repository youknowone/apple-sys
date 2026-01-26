//! Query QLPreviewPanel shared instance using QuickLookUI.
//!
//! Retrieves the shared preview panel and checks its full-screen state
//! and current controller information.

use apple_sys::QuickLookUI::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== QuickLookUI Preview Panel ===\n");

        println!("QLPreviewPanel class: available");

        // Check if a shared panel exists without creating one
        let exists = QLPreviewPanel::sharedPreviewPanelExists();
        println!("Shared panel exists: {}", exists.0);

        // Get the shared panel (creates it lazily)
        let panel = QLPreviewPanel::sharedPreviewPanel();
        if !panel.0.is_null() {
            println!("Shared panel: {}", nsobj_to_string(panel.0));

            let in_fullscreen = IQLPreviewPanel::isInFullScreenMode(&panel);
            println!("In full screen mode: {}", in_fullscreen.0);

            let current_controller = IQLPreviewPanel::currentController(&panel);
            println!(
                "Current controller: {}",
                nsobj_to_string(current_controller)
            );

            let delegate = IQLPreviewPanel::delegate(&panel);
            println!("Delegate: {}", nsobj_to_string(delegate));
        } else {
            println!("Failed to get shared preview panel.");
        }
    }

    println!("\nDone.");
}
