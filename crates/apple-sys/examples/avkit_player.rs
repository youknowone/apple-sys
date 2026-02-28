//! Create an AVPlayerView and inspect its default configuration.
//!
//! Uses the AVKit framework to examine player view properties.

use apple_sys::AVKit::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AVKit Player View ===\n");

        // Create AVPlayerView
        let view = AVPlayerView::alloc();
        let view_ptr = INSObject::init(&view);
        if view_ptr.is_null() {
            println!("Failed to create AVPlayerView.");
            return;
        }
        let view = AVPlayerView(view_ptr);
        println!("AVPlayerView created.");

        // controlsStyle: AVPlayerViewControlsStyle
        let controls_style = view.controlsStyle();
        let style_name = match controls_style {
            0 => "None",
            1 => "Inline",
            2 => "Floating",
            3 => "Minimal",
            4 => "Default",
            _ => "Unknown",
        };
        println!("Controls style: {style_name} ({controls_style})");

        let shows_full_screen = view.showsFullScreenToggleButton();
        println!("Shows full-screen button: {}", shows_full_screen.0);

        let shows_sharing = view.showsSharingServiceButton();
        println!("Shows sharing button: {}", shows_sharing.0);

        let shows_timeline = view.showsTimecodes();
        println!("Shows timecodes: {}", shows_timeline.0);

        let allows_pip = view.allowsPictureInPicturePlayback();
        println!("Allows PiP: {}", allows_pip.0);

        let player = view.player();
        println!(
            "Player: {}",
            if player.0.is_null() { "none" } else { "set" }
        );

        // Check AVRoutePickerView
        let picker = AVRoutePickerView::alloc();
        let picker_ptr = INSObject::init(&picker);
        if !picker_ptr.is_null() {
            let picker = AVRoutePickerView(picker_ptr);
            println!("\n--- AVRoutePickerView ---");
            let active = picker.isRoutePickerButtonBordered();
            println!("Route picker button bordered: {}", active.0);
        }
    }

    println!("\nDone.");
}
