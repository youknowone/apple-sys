//! Query shareable content with ScreenCaptureKit.
//!
//! Lists available SCDisplay and SCWindow instances
//! via SCShareableContent synchronous fetch.

use apple_sys::AppKit::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ScreenCaptureKit ===\n");

        // SCShareableContent is async-only, so we use NSScreen for display info
        let screens = NSScreen::screens();
        let count = INSArray::<id>::count(&screens);
        println!("\nDisplays (via NSScreen): {}", count);

        for i in 0..count {
            let screen_ptr = INSArray::<id>::objectAtIndex_(&screens, i);
            let screen = NSScreen(screen_ptr);
            let name = INSScreen::localizedName(&screen);
            let scale = INSScreen::backingScaleFactor(&screen);
            let depth = INSScreen::depth(&screen);

            let desc = PNSObject::description(&screen);

            println!(
                "  {}. {} (scale={:.0}x, depth={})",
                i + 1,
                nsstring_to_string(name),
                scale,
                depth
            );
            println!("     {}", nsstring_to_string(desc));
        }
    }

    println!("\nDone.");
}
