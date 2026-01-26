//! Exercise IntentsUI shortcut button bindings.
//!
//! Creates INUIAddVoiceShortcutButton with a style, queries and modifies
//! its properties (style, cornerRadius) through the generated bindings.

use apple_sys::IntentsUI::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== IntentsUI Shortcut Button ===\n");

        // INUIAddVoiceShortcutButton
        println!("--- INUIAddVoiceShortcutButton ---");
        let btn = INUIAddVoiceShortcutButton::alloc();
        // Style 4 = INUIAddVoiceShortcutButtonStyleAutomatic
        let btn_ptr = IINUIAddVoiceShortcutButton::initWithStyle_(&btn, 4);
        if !btn_ptr.is_null() {
            let btn = INUIAddVoiceShortcutButton(btn_ptr);

            let style = IINUIAddVoiceShortcutButton::style(&btn);
            let style_str = match style {
                0 => "White",
                1 => "WhiteOutline",
                2 => "Black",
                3 => "BlackOutline",
                4 => "Automatic",
                5 => "AutomaticOutline",
                _ => "Unknown",
            };
            println!("  Style: {style_str} ({style})");

            let radius = IINUIAddVoiceShortcutButton::cornerRadius(&btn);
            println!("  Corner radius: {radius}");

            // Modify corner radius
            IINUIAddVoiceShortcutButton::setCornerRadius_(&btn, 12.0);
            let new_radius = IINUIAddVoiceShortcutButton::cornerRadius(&btn);
            println!("  After setCornerRadius(12.0): {new_radius}");

            // Change style
            IINUIAddVoiceShortcutButton::setStyle_(&btn, 2); // Black
            let new_style = IINUIAddVoiceShortcutButton::style(&btn);
            println!("  After setStyle(Black): {new_style}");

            // Check shortcut (should be nil by default)
            let shortcut = IINUIAddVoiceShortcutButton::shortcut(&btn);
            if !shortcut.0.is_null() {
                println!("  Shortcut: {}", nsobj_to_string(shortcut.0));
            } else {
                println!("  Shortcut: (nil - none set)");
            }
        } else {
            println!("  (initWithStyle returned nil)");
        }
    }

    println!("\nDone.");
}
