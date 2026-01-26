//! List available speech synthesis voices.
//!
//! Uses NSSpeechSynthesizer to enumerate system voices and display
//! their names and language identifiers.

use apple_sys::AppKit::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, INSDictionary, NSAutoreleasePool, NSString};
use apple_sys::objc::id;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Speech Synthesis Voices ===\n");

        // Get all available voices
        let voices = NSSpeechSynthesizer::availableVoices();
        let count = INSArray::<id>::count(&voices);
        println!("Available voices: {}\n", count);

        // Default voice
        let default_voice = NSSpeechSynthesizer::defaultVoice();
        println!("Default voice: {}", nsstring_to_string(default_voice));

        // List voices with attributes
        println!("\nVoice details:");
        for i in 0..count {
            let voice_id = INSArray::<id>::objectAtIndex_(&voices, i);
            let attrs = NSSpeechSynthesizer::attributesForVoice_(NSString(voice_id));
            if attrs.0.is_null() {
                continue;
            }

            let name_key = nsstring(c"VoiceName");
            let name = INSDictionary::<(), ()>::objectForKey_(&attrs, name_key.0);

            let lang_key = nsstring(c"VoiceLocaleIdentifier");
            let locale = INSDictionary::<(), ()>::objectForKey_(&attrs, lang_key.0);

            let gender_key = nsstring(c"VoiceGender");
            let gender = INSDictionary::<(), ()>::objectForKey_(&attrs, gender_key.0);

            let name_str = nsstring_to_string(NSString(name));
            let locale_str = nsstring_to_string(NSString(locale));
            let gender_str = nsstring_to_string(NSString(gender));

            let g = match gender_str.as_str() {
                "VoiceGenderMale" => "M",
                "VoiceGenderFemale" => "F",
                "VoiceGenderNeuter" | "VoiceGenderNeutral" => "N",
                _ => "?",
            };

            println!("  {:3}. {:30} {:8} {}", i + 1, name_str, locale_str, g);
        }
    }

    println!("\nDone.");
}
