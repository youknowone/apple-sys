//! List available OSA scripting languages.
//!
//! Uses OSALanguage to enumerate installed scripting
//! languages (AppleScript, JavaScript for Automation, etc.).

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::OSAKit::*;
use apple_sys::objc::id;

mod common;
use common::nsstring_to_string;

fn fourcc_to_string(code: u32) -> String {
    let bytes = code.to_be_bytes();
    bytes
        .iter()
        .map(|&b| {
            if b.is_ascii_graphic() || b == b' ' {
                b as char
            } else {
                '?'
            }
        })
        .collect()
}

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== OSAKit Languages ===\n");

        // Default language
        let default_lang = OSALanguage::defaultLanguage();
        if !default_lang.0.is_null() {
            let name = IOSALanguage::name(&default_lang);
            println!("Default language: {}", nsstring_to_string(name));
        }

        // All available languages
        let languages = OSALanguage::availableLanguages();
        let count = INSArray::<id>::count(&languages);
        println!("\nAvailable languages: {}\n", count);

        for i in 0..count {
            let lang = OSALanguage(INSArray::<id>::objectAtIndex_(&languages, i));
            let name = IOSALanguage::name(&lang);
            let info = IOSALanguage::info(&lang);
            let version = IOSALanguage::version(&lang);
            let lang_type = IOSALanguage::type_(&lang);
            let sub_type = IOSALanguage::subType(&lang);
            let manufacturer = IOSALanguage::manufacturer(&lang);
            let thread_safe = IOSALanguage::isThreadSafe(&lang).0;

            println!("{}. {}", i + 1, nsstring_to_string(name));
            println!("   Info:         {}", nsstring_to_string(info));
            println!("   Version:      {}", nsstring_to_string(version));
            println!(
                "   Type:         '{}' (0x{:08X})",
                fourcc_to_string(lang_type),
                lang_type
            );
            println!(
                "   SubType:      '{}' (0x{:08X})",
                fourcc_to_string(sub_type),
                sub_type
            );
            println!(
                "   Manufacturer: '{}' (0x{:08X})",
                fourcc_to_string(manufacturer),
                manufacturer
            );
            println!("   Thread-safe:  {}", thread_safe);
            println!();
        }
    }

    println!("Done.");
}
