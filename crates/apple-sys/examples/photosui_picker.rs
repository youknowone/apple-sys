//! Create PHPickerConfiguration and set filter/selectionLimit via generated bindings.
//!
//! Demonstrates PhotosUI framework by creating a picker configuration
//! and setting various filter and selection options.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::PhotosUI::*;

#[link(name = "PhotosUI", kind = "framework")]
unsafe extern "C" {}

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== PhotosUI ===\n");

        // PHPickerConfiguration
        println!("--- PHPickerConfiguration ---");
        {
            let config = PHPickerConfiguration::alloc();
            let config_id = IPHPickerConfiguration::init(&config);
            if !config_id.is_null() {
                let config = PHPickerConfiguration(config_id);
                println!("  Config: {}", nsobj_to_string(config_id));

                // Default selection limit
                let limit = IPHPickerConfiguration::selectionLimit(&config);
                println!("  Default selection limit: {}", limit);

                // Set selection limit to 5
                IPHPickerConfiguration::setSelectionLimit_(&config, 5);
                let new_limit = IPHPickerConfiguration::selectionLimit(&config);
                println!("  After setSelectionLimit(5): {}", new_limit);

                // Preferred asset representation mode
                let pref_mode = IPHPickerConfiguration::preferredAssetRepresentationMode(&config);
                let mode_name = match pref_mode {
                    0 => "Automatic",
                    1 => "Current",
                    2 => "Compatible",
                    _ => "Unknown",
                };
                println!(
                    "  Preferred asset representation mode: {} ({})",
                    mode_name, pref_mode
                );

                // Selection (single vs ordered)
                let selection = IPHPickerConfiguration::selection(&config);
                let sel_name = match selection {
                    0 => "Default",
                    1 => "Ordered",
                    _ => "Unknown",
                };
                println!("  Selection behavior: {} ({})", sel_name, selection);

                // Set to ordered
                IPHPickerConfiguration::setSelection_(&config, 1);
                let new_selection = IPHPickerConfiguration::selection(&config);
                println!("  After setSelection(ordered): {}", new_selection);

                // Create a filter for images only
                let images_filter = PHPickerFilter::imagesFilter();
                if !images_filter.0.is_null() {
                    IPHPickerConfiguration::setFilter_(&config, images_filter);
                    let current_filter = IPHPickerConfiguration::filter(&config);
                    println!("  Filter (images): {}", nsobj_to_string(current_filter.0));
                }

                // Check other filters
                let videos_filter = PHPickerFilter::videosFilter();
                println!("  Videos filter available: {}", !videos_filter.0.is_null());

                let live_photos_filter = PHPickerFilter::livePhotosFilter();
                println!(
                    "  Live photos filter available: {}",
                    !live_photos_filter.0.is_null()
                );

                let screenshots_filter = PHPickerFilter::screenshotsFilter();
                println!(
                    "  Screenshots filter available: {}",
                    !screenshots_filter.0.is_null()
                );

                // Updated description after changes
                println!("\n  Updated config: {}", nsobj_to_string(config.0));
            }
        }
    }

    println!("\nDone.");
}
