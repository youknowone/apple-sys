//! List system font families using CoreText.
//!
//! Enumerates all installed font families and shows PostScript names
//! and glyph counts for the first 30 families.

#![allow(non_upper_case_globals)]

use apple_sys::CoreFoundation::*;
use apple_sys::CoreText::*;
use std::ffi::CStr;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;

unsafe fn cfstring_to_string(cfstr: CFStringRef) -> String {
    if cfstr.is_null() {
        return String::new();
    }
    let mut buf = [0i8; 256];
    let ok = unsafe {
        CFStringGetCString(
            cfstr,
            buf.as_mut_ptr(),
            buf.len() as CFIndex,
            kCFStringEncodingUTF8,
        )
    };
    if ok != 0 {
        unsafe { CStr::from_ptr(buf.as_ptr()) }
            .to_string_lossy()
            .into_owned()
    } else {
        let len = unsafe { CFStringGetLength(cfstr) };
        format!("<CFString len={len}>")
    }
}

fn main() {
    unsafe {
        println!("=== CoreText Font Enumeration Demo ===\n");

        let families = CTFontManagerCopyAvailableFontFamilyNames();
        assert!(!families.is_null(), "Failed to get font families");

        let count = CFArrayGetCount(families);
        println!("Found {count} font families:\n");

        for i in 0..count {
            let name = CFArrayGetValueAtIndex(families, i) as CFStringRef;
            let family = cfstring_to_string(name);

            let font = CTFontCreateWithName(name, 12.0, std::ptr::null());
            if font.is_null() {
                if i < 30 {
                    println!("  {family} (could not instantiate)");
                }
                continue;
            }

            let glyph_count = CTFontGetGlyphCount(font);
            let ps_name = CTFontCopyPostScriptName(font);
            let ps = cfstring_to_string(ps_name);
            if !ps_name.is_null() {
                CFRelease(ps_name as CFTypeRef);
            }

            if i < 30 {
                println!("  {family}");
                println!("    PostScript: {ps}  |  Glyphs: {glyph_count}");
            }

            CFRelease(font as CFTypeRef);
        }

        if count > 30 {
            println!("\n  ... and {} more families", count - 30);
        }

        println!("\nTotal: {count} font families.");
        CFRelease(families as CFTypeRef);
    }
}
