//! Create LAAuthenticationView and inspect its class structure.
//!
//! Demonstrates LocalAuthenticationEmbeddedUI framework usage
//! by creating an authentication view and querying its properties.

use apple_sys::AppKit::INSView;
use apple_sys::CoreFoundation::PNSObject;
use apple_sys::LocalAuthentication::*;
use apple_sys::LocalAuthenticationEmbeddedUI::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== LocalAuthenticationEmbeddedUI ===\n");

        // Create an LAContext first
        println!("--- LAContext ---");
        let context = LAContext::alloc();
        let context_ptr = INSObject::init(&context);
        if !context_ptr.is_null() {
            let context = LAContext(context_ptr);
            println!("  Context: {}", nsobj_to_string(context.0));

            let interaction_not_allowed = ILAContext::interactionNotAllowed(&context);
            println!("  Interaction not allowed: {}", interaction_not_allowed.0);

            // Check biometry type
            let biometry_type = ILAContext::biometryType(&context);
            let type_name = match biometry_type {
                0 => "None",
                1 => "TouchID",
                2 => "FaceID",
                3 => "OpticID",
                _ => "Unknown",
            };
            println!("  Biometry type: {} ({})", type_name, biometry_type);

            // Now create LAAuthenticationView with the context
            println!("\n--- LAAuthenticationView ---");
            let view = LAAuthenticationView::alloc();
            let view_ptr = ILAAuthenticationView::initWithContext_(&view, context);
            if !view_ptr.is_null() {
                let view = LAAuthenticationView(view_ptr);
                println!("  View: {}", nsobj_to_string(view.0));

                let hidden = INSView::isHidden(&view);
                println!("  Hidden: {}", hidden.0);

                // Check superclass
                let super_cls = PNSObject::superclass(&view);
                if !super_cls.0.is_null() {
                    println!("  Superclass: {}", nsobj_to_string(super_cls.0));
                }
            } else {
                println!("  Failed to create LAAuthenticationView");
            }
        }
    }

    println!("\nDone.");
}
