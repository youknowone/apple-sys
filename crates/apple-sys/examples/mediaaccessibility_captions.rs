//! Query MediaAccessibility caption appearance preferences.
//!
//! Calls C functions from the MediaAccessibility framework to read
//! the user's caption display type, selected languages,
//! preferred media characteristics, font style, and colors.

use apple_sys::CoreFoundation::{
    CFArrayGetCount, CFArrayGetValueAtIndex, CFRelease, CFStringGetCString, CFStringGetLength,
    CFStringGetMaximumSizeForEncoding, CFStringRef, CFTypeRef,
};
use apple_sys::MediaAccessibility::*;

const DOMAIN_DEFAULT: MACaptionAppearanceDomain = 0;

unsafe fn cfstring_to_string(s: CFStringRef) -> String {
    if s.is_null() {
        return String::new();
    }
    let len = unsafe { CFStringGetLength(s) };
    let max_size = unsafe { CFStringGetMaximumSizeForEncoding(len, 0x08000100) } + 1;
    let mut buf = vec![0u8; max_size as usize];
    let ok = unsafe { CFStringGetCString(s, buf.as_mut_ptr() as *mut i8, max_size, 0x08000100) };
    if ok != 0 {
        let cstr = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const i8) };
        cstr.to_string_lossy().into_owned()
    } else {
        String::from("<encoding error>")
    }
}

fn main() {
    unsafe {
        println!("=== MediaAccessibility Caption Preferences ===\n");

        // Display type
        let display_type = MACaptionAppearanceGetDisplayType(DOMAIN_DEFAULT);
        let type_name = match display_type {
            0 => "ForcedOnly",
            1 => "Automatic",
            2 => "AlwaysOn",
            _ => "Unknown",
        };
        println!("Caption display type: {} ({})", type_name, display_type);

        // Is customized?
        let customized = MACaptionAppearanceIsCustomized(DOMAIN_DEFAULT);
        println!("Caption appearance customized: {}", customized);

        // Selected languages
        let languages = MACaptionAppearanceCopySelectedLanguages(DOMAIN_DEFAULT);
        if !languages.is_null() {
            let count = CFArrayGetCount(languages);
            println!("\nSelected caption languages ({}):", count);
            for i in 0..count {
                let lang = CFArrayGetValueAtIndex(languages, i) as CFStringRef;
                println!("  - {}", cfstring_to_string(lang));
            }
            CFRelease(languages as CFTypeRef);
        } else {
            println!("\nNo selected caption languages");
        }

        // Preferred captioning media characteristics
        let chars = MACaptionAppearanceCopyPreferredCaptioningMediaCharacteristics(DOMAIN_DEFAULT);
        if !chars.is_null() {
            let count = CFArrayGetCount(chars);
            println!("\nPreferred captioning media characteristics ({}):", count);
            for i in 0..count {
                let c = CFArrayGetValueAtIndex(chars, i) as CFStringRef;
                println!("  - {}", cfstring_to_string(c));
            }
            CFRelease(chars as CFTypeRef);
        } else {
            println!("\nNo preferred captioning media characteristics");
        }

        // Foreground opacity
        let mut behavior: MACaptionAppearanceBehavior = 0;
        let fg_opacity = MACaptionAppearanceGetForegroundOpacity(DOMAIN_DEFAULT, &mut behavior);
        println!(
            "\nForeground opacity: {:.2} (behavior: {})",
            fg_opacity, behavior
        );

        // Background opacity
        let bg_opacity = MACaptionAppearanceGetBackgroundOpacity(DOMAIN_DEFAULT, &mut behavior);
        println!(
            "Background opacity: {:.2} (behavior: {})",
            bg_opacity, behavior
        );

        // Window opacity
        let win_opacity = MACaptionAppearanceGetWindowOpacity(DOMAIN_DEFAULT, &mut behavior);
        println!(
            "Window opacity: {:.2} (behavior: {})",
            win_opacity, behavior
        );

        // Window rounded corner radius
        let corner = MACaptionAppearanceGetWindowRoundedCornerRadius(DOMAIN_DEFAULT, &mut behavior);
        println!(
            "Window corner radius: {:.1} (behavior: {})",
            corner, behavior
        );

        // Relative character size
        let char_size = MACaptionAppearanceGetRelativeCharacterSize(DOMAIN_DEFAULT, &mut behavior);
        println!(
            "Relative character size: {:.0}% (behavior: {})",
            char_size * 100.0,
            behavior
        );

        // Text edge style
        let edge_style = MACaptionAppearanceGetTextEdgeStyle(DOMAIN_DEFAULT, &mut behavior);
        let edge_name = match edge_style {
            0 => "Undefined",
            1 => "None",
            2 => "Raised",
            3 => "Depressed",
            4 => "Uniform",
            5 => "DropShadow",
            _ => "Unknown",
        };
        println!("Text edge style: {} ({})", edge_name, edge_style);

        // Audible media preferred characteristics
        let audible = MAAudibleMediaCopyPreferredCharacteristics();
        if !audible.is_null() {
            let count = CFArrayGetCount(audible);
            println!("\nAudible media preferred characteristics ({}):", count);
            for i in 0..count {
                let c = CFArrayGetValueAtIndex(audible, i) as CFStringRef;
                println!("  - {}", cfstring_to_string(c));
            }
            CFRelease(audible as CFTypeRef);
        }

        // Dim flashing lights
        let dim = MADimFlashingLightsEnabled();
        println!("\nDim flashing lights enabled: {}", dim);

        // Image captioning metadata tag path
        let tag_path = MAImageCaptioningCopyMetadataTagPath();
        if !tag_path.is_null() {
            println!(
                "Image captioning metadata tag: {}",
                cfstring_to_string(tag_path)
            );
            CFRelease(tag_path as CFTypeRef);
        }
    }

    println!("\nDone.");
}
