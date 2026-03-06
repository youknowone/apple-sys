//! Query accessibility settings using the Accessibility framework.
//!
//! Reads various accessibility preference flags such as animated images,
//! horizontal text layout, assistive access, and more.

use apple_sys::Accessibility::*;

fn main() {
    unsafe {
        println!("=== Accessibility Settings ===\n");

        let animated = AXAnimatedImagesEnabled();
        println!("Animated images enabled:                {}", animated.0);

        let horiz_text = AXPrefersHorizontalTextLayout();
        println!("Prefers horizontal text layout:         {}", horiz_text.0);

        let assistive = AXAssistiveAccessEnabled();
        println!("Assistive access enabled:               {}", assistive.0);

        let non_blink = AXPrefersNonBlinkingTextInsertionIndicator();
        println!("Prefers non-blinking text indicator:    {}", non_blink.0);

        let action_slider = AXPrefersActionSliderAlternative();
        println!(
            "Prefers action slider alternative:      {}",
            action_slider.0
        );

        let show_borders = AXShowBordersEnabled();
        println!("Show borders enabled:                   {}", show_borders.0);
    }

    println!("\nDone.");
}
