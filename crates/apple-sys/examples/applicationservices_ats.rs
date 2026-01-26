//! Enumerate installed fonts via ATS (Apple Type Services).
//!
//! Uses ATSFontIterator to walk through all available fonts
//! and prints their names via ATSFontGetName.

use apple_sys::ApplicationServices::*;
use apple_sys::CoreFoundation::*;
use std::ptr;

const UTF8_ENCODING: CFStringEncoding = 0x08000100;

// ATSFontRef is UInt32 but owned by CoreText; define locally.
type ATSFontRef = u32;

unsafe fn cfstring_to_string(s: CFStringRef) -> String {
    if s.is_null() {
        return String::new();
    }
    let len = unsafe { CFStringGetLength(s) };
    let max_size = unsafe { CFStringGetMaximumSizeForEncoding(len, UTF8_ENCODING) } + 1;
    let mut buf = vec![0u8; max_size as usize];
    if unsafe { CFStringGetCString(s, buf.as_mut_ptr() as _, max_size, UTF8_ENCODING) } != 0 {
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        String::from_utf8_lossy(&buf[..end]).into_owned()
    } else {
        String::new()
    }
}

fn main() {
    unsafe {
        println!("=== ApplicationServices ATS Font Enumeration ===\n");

        // Query ATS generation (a counter that changes when fonts change)
        let generation = ATSGetGeneration();
        println!("ATS generation: {}\n", generation);

        // Create a font iterator for all fonts (global context)
        let mut iterator: ATSFontIterator = ptr::null_mut();
        let err = ATSFontIteratorCreate(
            0, // kATSFontContextGlobal
            ptr::null(),
            ptr::null_mut(),
            0, // kATSOptionFlagsDefault
            &mut iterator,
        );
        if err != 0 {
            eprintln!("ATSFontIteratorCreate failed: {}", err);
            return;
        }

        println!("Installed fonts (first 20):\n");
        let mut count = 0u32;
        loop {
            let mut font: ATSFontRef = 0;
            let err = ATSFontIteratorNext(iterator, &mut font);
            if err != 0 {
                break;
            }

            // Get the font name
            let mut name: CFStringRef = ptr::null();
            let name_err = ATSFontGetName(font, 0, &mut name);
            let font_name = if name_err == 0 && !name.is_null() {
                let s = cfstring_to_string(name);
                CFRelease(name as _);
                s
            } else {
                "(unknown)".to_string()
            };

            // Get PostScript name
            let mut ps_name: CFStringRef = ptr::null();
            let ps_err = ATSFontGetPostScriptName(font, 0, &mut ps_name);
            let ps_name_str = if ps_err == 0 && !ps_name.is_null() {
                let s = cfstring_to_string(ps_name);
                CFRelease(ps_name as _);
                s
            } else {
                "(unknown)".to_string()
            };

            count += 1;
            if count <= 20 {
                println!(
                    "  {:>3}. [ref={}] {} (PostScript: {})",
                    count, font, font_name, ps_name_str
                );
            }
        }

        if count > 20 {
            println!("  ... and {} more", count - 20);
        }
        println!("\nTotal fonts enumerated: {}", count);

        ATSFontIteratorRelease(&mut iterator);
    }

    println!("\nDone.");
}
