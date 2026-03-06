//! Exercise JavaRuntimeSupport UI renderer and font rendering APIs.
//!
//! Creates JRSUI renderer and control objects, queries font rendering style
//! properties, and checks scroll-to-click preference.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::JavaRuntimeSupport::*;
use apple_sys::objc::id;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== JavaRuntimeSupport ===\n");

        // JRSUI Renderer
        println!("--- JRSUIRenderer ---");
        let renderer = JRSUIRendererCreate();
        if !renderer.is_null() {
            println!("  Renderer created: {:?}", renderer);
            JRSUIRendererRelease(renderer);
            println!("  Renderer released");
        } else {
            println!("  (failed to create renderer)");
        }

        // JRSUI Control
        println!("\n--- JRSUIControl ---");
        let control = JRSUIControlCreate(0); // isFlipped = false
        if !control.is_null() {
            println!("  Control created (isFlipped=false): {:?}", control);

            // Set widget, state, size
            JRSUIControlSetWidget(control, 0);
            JRSUIControlSetState(control, 0);
            JRSUIControlSetSize(control, 0);
            JRSUIControlSetDirection(control, 0);
            JRSUIControlSetOrientation(control, 0);
            println!("  Widget/state/size/direction/orientation set to 0");

            JRSUIControlSetShowArrows(control, 1);
            JRSUIControlSetAnimating(control, 0);
            println!("  ShowArrows=true, Animating=false");

            // Check scroll-to-click preference
            let scroll_to_click = JRSUIControlShouldScrollToClick();
            println!("  ShouldScrollToClick: {scroll_to_click}");

            JRSUIControlRelease(control);
            println!("  Control released");
        } else {
            println!("  (failed to create control)");
        }

        // JRS Font Rendering Style
        println!("\n--- JRSFontRenderingStyle ---");
        // Get rendering style from hints
        let style = JRSFontGetRenderingStyleForHints(0, 0);
        println!("  Style from hints(0,0): {style}");

        let uses_fractional = JRSFontStyleUsesFractionalMetrics(style);
        let is_antialiased = JRSFontStyleIsAntialiased(style);
        println!("  Uses fractional metrics: {uses_fractional}");
        println!("  Is antialiased: {is_antialiased}");

        // JRSInputMethodController
        println!("\n--- JRSInputMethodController ---");
        let controller = JRSInputMethodController::controller();
        if !controller.0.is_null() {
            let im_name = IJRSInputMethodController::currentInputMethodName(&controller);
            let im_locale = IJRSInputMethodController::currentInputMethodLocale(&controller);
            println!("  Current input method: {}", nsstring_to_string(im_name));
            println!("  Current locale: {}", nsstring_to_string(im_locale));

            let locales = IJRSInputMethodController::availableInputMethodLocales(&controller);
            if !locales.0.is_null() {
                let count = INSArray::<id>::count(&locales);
                println!("  Available locales: {count}");
            }
        } else {
            println!("  (controller not available)");
        }

        // JRSDrag
        println!("\n--- JRSDrag ---");
        let actions = JRSDrag::currentAllowableActions();
        let modifiers = JRSDrag::currentModifiers();
        println!("  Current allowable actions: {actions}");
        println!("  Current modifiers: {modifiers}");
    }

    println!("\nDone.");
}
