//! Query SFSpeechRecognizer using Speech framework.
//!
//! Gets supported locales, creates a recognizer with the default locale,
//! and checks availability and authorization status.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{
    INSArray, NSAutoreleasePool, NSLocale, NSLocale_NSExtendedLocale, NSSet_NSExtendedSet,
};
use apple_sys::Speech::*;
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Speech Recognition ===\n");

        println!("SFSpeechRecognizer class: available");

        let auth_status = SFSpeechRecognizer::authorizationStatus();
        let auth_name = match auth_status {
            0 => "Not Determined",
            1 => "Denied",
            2 => "Restricted",
            3 => "Authorized",
            _ => "Unknown",
        };
        println!("Authorization status: {auth_name} ({auth_status})");

        // Get supported locales
        let locales = SFSpeechRecognizer::supportedLocales();
        if !locales.0.is_null() {
            let all_locales = NSSet_NSExtendedSet::<id>::allObjects(&locales);
            let count = INSArray::<id>::count(&all_locales);
            println!("\nSupported locales ({count} total):");

            let mut locale_strs = Vec::new();
            for i in 0..count {
                let locale_id = INSArray::<id>::objectAtIndex_(&all_locales, i);
                let locale = NSLocale(locale_id);
                let identifier = locale.localeIdentifier();
                locale_strs.push(nsobj_to_string(identifier.0));
            }
            locale_strs.sort();
            for (i, loc) in locale_strs.iter().take(15).enumerate() {
                println!("  [{i:2}] {loc}");
            }
            if count > 15 {
                println!("  ... and {} more", count - 15);
            }
        }

        // Create recognizer with default locale
        println!("\n--- Default Recognizer ---");
        let recognizer = SFSpeechRecognizer::alloc();
        let recognizer_ptr = INSObject::init(&recognizer);
        if !recognizer_ptr.is_null() {
            let recognizer = SFSpeechRecognizer(recognizer_ptr);

            let available = ISFSpeechRecognizer::isAvailable(&recognizer);
            println!("Available: {}", available.0);

            let locale = ISFSpeechRecognizer::locale(&recognizer);
            let locale_obj = NSLocale(locale.0);
            let locale_id = locale_obj.localeIdentifier();
            println!("Locale: {}", nsobj_to_string(locale_id.0));

            let supports_on_device = ISFSpeechRecognizer::supportsOnDeviceRecognition(&recognizer);
            println!("Supports on-device recognition: {}", supports_on_device.0);

            let default_task = ISFSpeechRecognizer::defaultTaskHint(&recognizer);
            let task_name = match default_task {
                0 => "Unspecified",
                1 => "Dictation",
                2 => "Search",
                3 => "Confirmation",
                _ => "Unknown",
            };
            println!("Default task hint: {task_name} ({default_task})");

            let queue = ISFSpeechRecognizer::queue(&recognizer);
            println!("Queue: {}", nsobj_to_string(queue.0));
        } else {
            println!("Failed to create recognizer (no locale available?).");
        }
    }

    println!("\nDone.");
}
