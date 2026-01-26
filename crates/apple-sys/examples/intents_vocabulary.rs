//! Inspect Siri Intents vocabulary and intent classes.
//!
//! Uses INVocabulary and INPreferences to check
//! Siri integration capabilities.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::Intents::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Intents (Siri) ===\n");

        // INPreferences - Siri authorization
        let auth = INPreferences::siriAuthorizationStatus();
        let auth_name = match auth {
            0 => "Not Determined",
            1 => "Restricted",
            2 => "Denied",
            3 => "Authorized",
            _ => "Unknown",
        };
        println!("Siri authorization: {} ({})", auth_name, auth);

        let lang = INPreferences::siriLanguageCode();
        println!("Siri language: {}", nsstring_to_string(lang));

        // INVocabulary
        let vocab_ptr = INVocabulary::sharedVocabulary();
        let vocab = INVocabulary(vocab_ptr);
        if !vocab.0.is_null() {
            println!("\nINVocabulary: available");
        }

        // Create a sample interaction
        let intent = INIntent::alloc();
        let intent_ptr = INSObject::init(&intent);
        if !intent_ptr.is_null() {
            let intent = INIntent(intent_ptr);
            let identifier = IINIntent::identifier(&intent);
            let desc = IINIntent::intentDescription(&intent);
            println!("\nSample INIntent:");
            println!("  Identifier: {}", nsstring_to_string(identifier));
            println!("  Description: {}", nsstring_to_string(desc));
        }
    }

    println!("\nDone.");
}
