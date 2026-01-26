//! Enumerate Audio Components via AudioUnit/AudioToolbox.
//!
//! Uses AudioComponentFindNext and AudioComponentCopyName to list
//! available audio units (effects, instruments, generators, etc.).

use apple_sys::AudioToolbox::*;
use apple_sys::AudioUnit::*;
use std::ptr;

const UTF8_ENCODING: CFStringEncoding = 0x08000100;

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
        println!("=== AudioToolbox Components ===\n");

        // Search for all audio components
        let desc = AudioComponentDescription {
            componentType: 0,
            componentSubType: 0,
            componentManufacturer: 0,
            componentFlags: 0,
            componentFlagsMask: 0,
        };

        let mut component: AudioComponent = ptr::null_mut();
        let mut count = 0u32;
        let mut by_type: std::collections::BTreeMap<String, Vec<String>> =
            std::collections::BTreeMap::new();

        loop {
            component = AudioComponentFindNext(component, &desc);
            if component.is_null() {
                break;
            }
            count += 1;

            let mut comp_desc = AudioComponentDescription {
                componentType: 0,
                componentSubType: 0,
                componentManufacturer: 0,
                componentFlags: 0,
                componentFlagsMask: 0,
            };
            AudioComponentGetDescription(component, &mut comp_desc);

            let mut name_ref: CFStringRef = ptr::null();
            AudioComponentCopyName(component, &mut name_ref);
            let name = if !name_ref.is_null() {
                let s = cfstring_to_string(name_ref);
                CFRelease(name_ref as _);
                s
            } else {
                "Unknown".to_string()
            };

            let type_str = fourcc_to_string(comp_desc.componentType);
            by_type.entry(type_str).or_default().push(name);
        }

        println!("Total audio components: {}\n", count);

        for (type_code, names) in &by_type {
            let type_name = match type_code.as_str() {
                "auou" => "Output",
                "aumu" => "Music Instrument",
                "aumx" => "Mixer",
                "aufc" => "Format Converter",
                "aufx" => "Effect",
                "augn" => "Generator",
                "aupn" => "Panner",
                "aumi" => "MIDI Processor",
                _ => type_code.as_str(),
            };
            println!(
                "{} [{}] ({} components):",
                type_name,
                type_code,
                names.len()
            );
            for name in names.iter().take(10) {
                println!("  - {}", name);
            }
            if names.len() > 10 {
                println!("  ... and {} more", names.len() - 10);
            }
            println!();
        }
    }

    println!("Done.");
}
