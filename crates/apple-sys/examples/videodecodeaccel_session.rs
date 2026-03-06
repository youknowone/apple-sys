//! Attempt to create a VideoDecodeAcceleration decoder session.
//!
//! Demonstrates VDADecoderCreate with H.264 configuration.
//! The decoder creation will fail without valid avcC data, but this
//! shows the proper API usage pattern.

#![allow(non_upper_case_globals)]

use apple_sys::CoreFoundation::{
    CFDataCreate, CFDictionaryCreate, CFIndex, CFNumberCreate, CFNumberType, CFRelease,
    CFStringEncoding, CFStringGetCString, CFStringGetLength, CFStringGetMaximumSizeForEncoding,
    CFStringRef, CFTypeRef, OSStatus, kCFAllocatorDefault, kCFTypeDictionaryKeyCallBacks,
    kCFTypeDictionaryValueCallBacks,
};
use apple_sys::VideoDecodeAcceleration::*;
use std::ptr;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
const kCFNumberSInt32Type: CFNumberType = 3;

fn status_name(status: OSStatus) -> &'static str {
    match status {
        0 => "noErr",
        -50 => "paramErr",
        -8960 => "kVDADecoderHardwareNotSupportedErr",
        -8961 => "kVDADecoderFormatNotSupportedErr",
        -8969 => "kVDADecoderConfigurationError",
        -8970 => "kVDADecoderDecoderFailedErr",
        _ => "unknown",
    }
}

fn main() {
    unsafe {
        println!("=== VideoDecodeAcceleration ===\n");

        // 1. Print configuration key strings
        println!("VDA configuration keys:");
        let keys: &[(&str, CFStringRef)] = &[
            ("Height", kVDADecoderConfiguration_Height),
            ("Width", kVDADecoderConfiguration_Width),
            ("SourceFormat", kVDADecoderConfiguration_SourceFormat),
            ("avcCData", kVDADecoderConfiguration_avcCData),
        ];
        for (label, key) in keys {
            if !key.is_null() {
                let len = CFStringGetLength(*key);
                let max = CFStringGetMaximumSizeForEncoding(len, kCFStringEncodingUTF8) + 1;
                let mut buf = vec![0u8; max as usize];
                if CFStringGetCString(*key, buf.as_mut_ptr() as _, max, kCFStringEncodingUTF8) != 0
                {
                    let s = std::ffi::CStr::from_ptr(buf.as_ptr() as _);
                    println!("  {} = \"{}\"", label, s.to_string_lossy());
                }
            }
        }

        // 2. Build a decoder configuration dictionary
        // We'll use H.264 (avc1) format with 1920x1080
        let width_val: i32 = 1920;
        let height_val: i32 = 1080;
        let format_val: i32 = i32::from_be_bytes(*b"avc1"); // 'avc1' FourCC

        let width_num = CFNumberCreate(
            kCFAllocatorDefault,
            kCFNumberSInt32Type,
            &width_val as *const _ as _,
        );
        let height_num = CFNumberCreate(
            kCFAllocatorDefault,
            kCFNumberSInt32Type,
            &height_val as *const _ as _,
        );
        let format_num = CFNumberCreate(
            kCFAllocatorDefault,
            kCFNumberSInt32Type,
            &format_val as *const _ as _,
        );

        // Create a minimal (invalid) avcC data blob
        // A real app would extract this from an MP4/MOV container
        let avcc_bytes: [u8; 7] = [
            0x01, // configurationVersion
            0x64, // AVCProfileIndication (High)
            0x00, // profile_compatibility
            0x28, // AVCLevelIndication (4.0)
            0xFF, // lengthSizeMinusOne = 3 (4 bytes)
            0xE1, // numOfSequenceParameterSets = 1
            0x00, // SPS length (invalid, but just for demonstration)
        ];
        let avcc_data = CFDataCreate(
            kCFAllocatorDefault,
            avcc_bytes.as_ptr(),
            avcc_bytes.len() as CFIndex,
        );

        let mut dict_keys: [*const std::os::raw::c_void; 4] = [
            kVDADecoderConfiguration_Width as *const _,
            kVDADecoderConfiguration_Height as *const _,
            kVDADecoderConfiguration_SourceFormat as *const _,
            kVDADecoderConfiguration_avcCData as *const _,
        ];
        let mut dict_values: [*const std::os::raw::c_void; 4] = [
            width_num as *const _,
            height_num as *const _,
            format_num as *const _,
            avcc_data as *const _,
        ];

        let config = CFDictionaryCreate(
            kCFAllocatorDefault,
            dict_keys.as_mut_ptr(),
            dict_values.as_mut_ptr(),
            4,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );

        println!("\nDecoder configuration:");
        println!("  Width:  {}", width_val);
        println!("  Height: {}", height_val);
        println!("  Format: avc1 (H.264)");
        println!("  avcC:   {} bytes (minimal/test)", avcc_bytes.len());

        // 3. Attempt to create a VDA decoder
        println!("\nCreating VDA decoder...");
        let mut decoder: VDADecoder = ptr::null_mut();
        let status = VDADecoderCreate(
            config,
            ptr::null(),     // destinationImageBufferAttributes
            ptr::null_mut(), // outputCallback (null = no callback)
            ptr::null_mut(), // refcon
            &mut decoder,
        );

        println!("  VDADecoderCreate: {} ({})", status, status_name(status));

        if status == 0 && !decoder.is_null() {
            println!("  Decoder created successfully!");

            // Flush the decoder
            let flush_status = VDADecoderFlush(decoder, 0);
            println!(
                "  VDADecoderFlush: {} ({})",
                flush_status,
                status_name(flush_status)
            );

            // Destroy the decoder
            let destroy_status = VDADecoderDestroy(decoder);
            println!(
                "  VDADecoderDestroy: {} ({})",
                destroy_status,
                status_name(destroy_status)
            );
        } else {
            println!("  Decoder creation failed (expected with minimal avcC data).");
            println!("  Note: VDA is deprecated in favor of VideoToolbox.");
        }

        // Cleanup
        CFRelease(config as CFTypeRef);
        CFRelease(avcc_data as CFTypeRef);
        CFRelease(format_num as CFTypeRef);
        CFRelease(height_num as CFTypeRef);
        CFRelease(width_num as CFTypeRef);
    }

    println!("\nDone.");
}
