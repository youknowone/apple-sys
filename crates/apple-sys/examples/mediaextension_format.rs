//! Exercise MediaExtension format reader types.
//!
//! Creates MEFormatReaderInstantiationOptions, MEFileInfo, and METrackInfo
//! instances, setting and querying their properties.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::MediaExtension::*;
use apple_sys::objc::BOOL;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MediaExtension ===\n");

        // MEFormatReaderInstantiationOptions
        println!("--- MEFormatReaderInstantiationOptions ---");
        let opts = MEFormatReaderInstantiationOptions::alloc();
        let opts_ptr = INSObject::init(&opts);
        if !opts_ptr.is_null() {
            let opts = MEFormatReaderInstantiationOptions(opts_ptr);
            let incremental =
                <MEFormatReaderInstantiationOptions as IMEFormatReaderInstantiationOptions>::allowIncrementalFragmentParsing(&opts);
            println!("  allowIncrementalFragmentParsing: {}", incremental.0);
        } else {
            println!("  (init returned nil)");
        }

        // MEFileInfo
        println!("\n--- MEFileInfo ---");
        let info = MEFileInfo::alloc();
        let info_ptr = INSObject::init(&info);
        if !info_ptr.is_null() {
            let info = MEFileInfo(info_ptr);

            let dur = <MEFileInfo as IMEFileInfo>::duration(&info);
            let dur_value = dur.value;
            let dur_timescale = dur.timescale;
            println!("  Duration value: {}", dur_value);
            println!("  Duration timescale: {}", dur_timescale);

            let frag_status = <MEFileInfo as IMEFileInfo>::fragmentsStatus(&info);
            let status_name = match frag_status {
                0 => "NoFragments",
                1 => "ContainsFragments",
                2 => "FragmentsComplete",
                _ => "Unknown",
            };
            println!("  Fragments status: {status_name} ({frag_status})");

            let sidecar = <MEFileInfo as IMEFileInfo>::sidecarFileName(&info);
            println!("  Sidecar filename: {}", nsstring_to_string(sidecar));

            let sidecar_name = nsstring(c"test.sidecar");
            <MEFileInfo as IMEFileInfo>::setSidecarFileName_(&info, sidecar_name);
            let updated = <MEFileInfo as IMEFileInfo>::sidecarFileName(&info);
            println!(
                "  After setSidecarFileName: {}",
                nsstring_to_string(updated)
            );
        } else {
            println!("  (init returned nil)");
        }

        // METrackInfo
        println!("\n--- METrackInfo ---");
        let track = METrackInfo::alloc();
        let track_ptr = INSObject::init(&track);
        if !track_ptr.is_null() {
            let track = METrackInfo(track_ptr);
            let enabled = <METrackInfo as IMETrackInfo>::isEnabled(&track);
            println!("  isEnabled: {}", enabled.0);

            let media_type = <METrackInfo as IMETrackInfo>::mediaType(&track);
            println!("  mediaType: {media_type}");

            let track_id = <METrackInfo as IMETrackInfo>::trackID(&track);
            println!("  trackID: {track_id}");

            // OptionalProperties
            let natural_timescale =
                <METrackInfo as METrackInfo_OptionalProperties>::naturalTimescale(&track);
            println!("  naturalTimescale: {natural_timescale}");

            // VideoSpecificOptionalProperties
            let frame_rate =
                <METrackInfo as METrackInfo_VideoSpecificOptionalProperties>::nominalFrameRate(
                    &track,
                );
            println!("  nominalFrameRate: {frame_rate}");

            let reorder = <METrackInfo as METrackInfo_VideoSpecificOptionalProperties>::requiresFrameReordering(&track);
            println!("  requiresFrameReordering: {}", reorder.0);

            <METrackInfo as METrackInfo_VideoSpecificOptionalProperties>::setNominalFrameRate_(
                &track, 29.97,
            );
            let updated_rate =
                <METrackInfo as METrackInfo_VideoSpecificOptionalProperties>::nominalFrameRate(
                    &track,
                );
            println!("  After setNominalFrameRate(29.97): {updated_rate}");

            <METrackInfo as IMETrackInfo>::setEnabled_(&track, BOOL(true));
            let updated_enabled = <METrackInfo as IMETrackInfo>::isEnabled(&track);
            println!("  After setEnabled(true): {}", updated_enabled.0);

            // LanguageTagOptionalProperties
            let lang =
                <METrackInfo as METrackInfo_LanguageTagOptionalProperties>::extendedLanguageTag(
                    &track,
                );
            println!("  extendedLanguageTag: {}", nsstring_to_string(lang));
        } else {
            println!("  (init returned nil)");
        }

        // MEDecodeFrameOptions
        println!("\n--- MEDecodeFrameOptions ---");
        let opts = MEDecodeFrameOptions::alloc();
        let opts_ptr = INSObject::init(&opts);
        if !opts_ptr.is_null() {
            let opts = MEDecodeFrameOptions(opts_ptr);
            let do_not_output =
                <MEDecodeFrameOptions as IMEDecodeFrameOptions>::doNotOutputFrame(&opts);
            println!("  doNotOutputFrame: {}", do_not_output.0);

            let realtime = <MEDecodeFrameOptions as IMEDecodeFrameOptions>::realTimePlayback(&opts);
            println!("  realTimePlayback: {}", realtime.0);

            <MEDecodeFrameOptions as IMEDecodeFrameOptions>::setRealTimePlayback_(
                &opts,
                BOOL(true),
            );
            let updated = <MEDecodeFrameOptions as IMEDecodeFrameOptions>::realTimePlayback(&opts);
            println!("  After setRealTimePlayback(true): {}", updated.0);
        } else {
            println!("  (init returned nil)");
        }
    }

    println!("\nDone.");
}
