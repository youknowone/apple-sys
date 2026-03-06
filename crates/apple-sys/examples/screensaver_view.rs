//! Inspect ScreenSaver framework classes via generated bindings.
//!
//! Creates ScreenSaverDefaults with a module name and creates a
//! ScreenSaverView instance to query its properties.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSUserDefaults, NSAutoreleasePool, NSPoint, NSRect, NSSize};
use apple_sys::ScreenSaver::*;
use apple_sys::objc::BOOL;

mod common;
use common::nsobj_to_string;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ScreenSaver Framework ===\n");

        // ScreenSaverView: create an instance and query properties
        println!("--- ScreenSaverView ---");
        {
            let frame = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize {
                    width: 800.0,
                    height: 600.0,
                },
            };
            let view = ScreenSaverView::alloc();
            let view_ptr = IScreenSaverView::initWithFrame_isPreview_(&view, frame, BOOL(true));
            if !view_ptr.is_null() {
                let view = ScreenSaverView(view_ptr);
                println!("  Instance: {}", nsobj_to_string(view_ptr));

                let is_animating = IScreenSaverView::isAnimating(&view).0;
                println!("  Is animating: {}", is_animating);

                let is_preview = IScreenSaverView::isPreview(&view).0;
                println!("  Is preview: {}", is_preview);

                let interval = IScreenSaverView::animationTimeInterval(&view);
                println!("  Animation time interval: {}", interval);

                let has_sheet = IScreenSaverView::hasConfigureSheet(&view).0;
                println!("  Has configure sheet: {}", has_sheet);

                let backing_store = ScreenSaverView::backingStoreType();
                println!("  Backing store type: {}", backing_store);

                let gamma_fade = ScreenSaverView::performGammaFade().0;
                println!("  Perform gamma fade: {}", gamma_fade);
            } else {
                println!("  Failed to create ScreenSaverView instance");
            }
        }

        // ScreenSaverDefaults
        println!("\n--- ScreenSaverDefaults ---");
        let module_name = nsstring(c"com.apple.ScreenSaver.Basic");
        let defaults_ptr = ScreenSaverDefaults::defaultsForModuleWithName_(module_name);
        if !defaults_ptr.is_null() {
            println!(
                "Defaults for com.apple.ScreenSaver.Basic: {}",
                nsobj_to_string(defaults_ptr)
            );

            let key = nsstring(c"ShowClock");
            let defaults = ScreenSaverDefaults(defaults_ptr);
            let value = INSUserDefaults::objectForKey_(&defaults, key);
            println!("ShowClock value: {}", nsobj_to_string(value));
        } else {
            println!("Failed to get defaults for module.");
        }
    }

    println!("\nDone.");
}
