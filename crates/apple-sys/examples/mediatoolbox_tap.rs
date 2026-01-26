//! Query media type names and create an audio processing tap.
//!
//! Demonstrates MTAudioProcessingTap creation and
//! MTCopyLocalizedNameForMediaType / MTCopyLocalizedNameForMediaSubType.

#![allow(non_upper_case_globals)]

use apple_sys::CoreAudio::AudioBufferList;
use apple_sys::CoreFoundation::*;
use apple_sys::CoreMedia::*;
use apple_sys::MediaToolbox::*;
use std::ptr;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;

unsafe fn cfstring_to_string(cf: CFStringRef) -> String {
    if cf.is_null() {
        return String::new();
    }
    let len = unsafe { CFStringGetLength(cf) };
    let max_size = unsafe { CFStringGetMaximumSizeForEncoding(len, kCFStringEncodingUTF8) } + 1;
    let mut buf = vec![0u8; max_size as usize];
    if unsafe { CFStringGetCString(cf, buf.as_mut_ptr() as _, max_size, kCFStringEncodingUTF8) }
        != 0
    {
        let cstr = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as _) };
        cstr.to_string_lossy().to_string()
    } else {
        format!("<CFString len={}>", len)
    }
}

fn fourcc(s: &[u8; 4]) -> u32 {
    ((s[0] as u32) << 24) | ((s[1] as u32) << 16) | ((s[2] as u32) << 8) | (s[3] as u32)
}

fn main() {
    unsafe {
        println!("=== MediaToolbox ===\n");

        // 1. Query localized names for common media types
        // CMMediaType: 'vide' = video, 'soun' = audio, 'text' = text
        let media_types: &[(&str, u32)] = &[
            ("video", fourcc(b"vide")),
            ("audio", fourcc(b"soun")),
            ("text", fourcc(b"text")),
            ("closedCaption", fourcc(b"clcp")),
            ("subtitle", fourcc(b"sbtl")),
            ("timecode", fourcc(b"tmcd")),
            ("metadata", fourcc(b"meta")),
        ];

        println!("Localized media type names:");
        for (label, media_type) in media_types {
            let name = MTCopyLocalizedNameForMediaType(*media_type);
            if !name.is_null() {
                println!("  {} => {}", label, cfstring_to_string(name));
                CFRelease(name as CFTypeRef);
            } else {
                println!("  {} => (no localized name)", label);
            }
        }

        // 2. Query localized names for video subtypes (codec FourCCs)
        let video_type = fourcc(b"vide");
        let video_subtypes: &[(&str, u32)] = &[
            ("H.264", fourcc(b"avc1")),
            ("HEVC", fourcc(b"hvc1")),
            ("Apple ProRes 422", fourcc(b"apcn")),
            ("JPEG", fourcc(b"jpeg")),
        ];

        println!("\nLocalized video subtype names:");
        for (label, subtype) in video_subtypes {
            let name = MTCopyLocalizedNameForMediaSubType(video_type, *subtype);
            if !name.is_null() {
                println!("  {} => {}", label, cfstring_to_string(name));
                CFRelease(name as CFTypeRef);
            } else {
                println!("  {} => (no localized name)", label);
            }
        }

        // 3. Get the CFTypeID for MTAudioProcessingTap
        let type_id = MTAudioProcessingTapGetTypeID();
        println!("\nMTAudioProcessingTap CFTypeID: {}", type_id);

        // 4. Register professional video workflow format readers
        MTRegisterProfessionalVideoWorkflowFormatReaders();
        println!("MTRegisterProfessionalVideoWorkflowFormatReaders called.");

        // 5. Attempt to create an MTAudioProcessingTap with minimal callbacks
        unsafe extern "C" fn tap_init(
            _tap: MTAudioProcessingTapRef,
            _client_info: *mut std::os::raw::c_void,
            _storage_out: *mut *mut std::os::raw::c_void,
        ) {
        }

        unsafe extern "C" fn tap_finalize(_tap: MTAudioProcessingTapRef) {}

        unsafe extern "C" fn tap_process(
            _tap: MTAudioProcessingTapRef,
            _frames: CMItemCount,
            _flags: MTAudioProcessingTapFlags,
            _buf: *mut AudioBufferList,
            _frames_out: *mut CMItemCount,
            _flags_out: *mut MTAudioProcessingTapFlags,
        ) {
        }

        let callbacks = MTAudioProcessingTapCallbacks {
            version: 0, // kMTAudioProcessingTapCallbacksVersion_0
            clientInfo: ptr::null_mut(),
            init: Some(tap_init),
            finalize: Some(tap_finalize),
            prepare: None,
            unprepare: None,
            process: Some(tap_process),
        };

        let mut tap: MTAudioProcessingTapRef = ptr::null();
        let status = MTAudioProcessingTapCreate(
            kCFAllocatorDefault,
            &callbacks,
            0, // kMTAudioProcessingTapCreationFlag_PreEffects
            &mut tap,
        );

        if status == 0 && !tap.is_null() {
            println!("MTAudioProcessingTap created successfully.");
            CFRelease(tap as CFTypeRef);
            println!("MTAudioProcessingTap released.");
        } else {
            println!(
                "MTAudioProcessingTapCreate returned status: {} (tap null: {})",
                status,
                tap.is_null()
            );
        }
    }

    println!("\nDone.");
}
