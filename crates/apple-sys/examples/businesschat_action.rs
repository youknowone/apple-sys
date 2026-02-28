//! Inspect BusinessChat framework classes.
//!
//! Creates a BCChatAction and BCChatButton, and accesses the framework's
//! extern constants (BCParameterNameIntent, etc.).

use apple_sys::BusinessChat::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== BusinessChat ===\n");

        // BCChatAction
        let action = BCChatAction::alloc();
        let action_ptr = INSObject::init(&action);
        if !action_ptr.is_null() {
            let action = BCChatAction(action_ptr);
            println!("BCChatAction instance created.");
            println!("  Description: {}", nsobj_to_string(action.0));
        } else {
            println!("Failed to create BCChatAction instance.");
        }

        // BCChatButton - create with style 0 (light)
        let btn = BCChatButton::alloc();
        let btn_ptr = IBCChatButton::initWithStyle_(&btn, 0);
        if !btn_ptr.is_null() {
            let btn = BCChatButton(btn_ptr);
            println!("\nBCChatButton (light style) created.");
            println!("  Description: {}", nsobj_to_string(btn.0));
        } else {
            println!("\nFailed to create BCChatButton.");
        }

        // Access exported constants
        println!("\n--- BCParameterName Constants ---");
        println!(
            "  BCParameterNameIntent: {}",
            nsstring_to_string(BCParameterNameIntent)
        );
        println!(
            "  BCParameterNameGroup:  {}",
            nsstring_to_string(BCParameterNameGroup)
        );
        println!(
            "  BCParameterNameBody:   {}",
            nsstring_to_string(BCParameterNameBody)
        );
    }

    println!("\nDone.");
}
