//! Initialize DVDPlayback framework.
//!
//! Demonstrates DVDInitialize and DVDDispose, querying whether
//! DVD playback hardware is available.

use apple_sys::DVDPlayback::*;

fn dvd_error_string(err: i32) -> &'static str {
    match err {
        0 => "noErr",
        -70001 => "kDVDErrorInitializingLib",
        -70002 => "kDVDErrorUninitializedLib",
        -70003 => "kDVDErrorNotAllowedDuringPlayback",
        -70012 => "kDVDErrorNotSupportedConfiguration",
        _ => "unknown",
    }
}

fn main() {
    unsafe {
        println!("=== DVDPlayback Framework ===\n");

        // Try to initialize
        println!("Calling DVDInitialize...");
        let err = DVDInitialize();
        println!("  Result: {} ({})", dvd_error_string(err), err);

        if err == 0 {
            // Check if media is present
            let mut has_media: u8 = 0;
            let err = DVDHasMedia(&mut has_media);
            println!("  Has DVD media: {} (err={})", has_media != 0, err);

            // Check playback state
            let mut state: DVDState = 0;
            let err = DVDGetState(&mut state);
            let state_name = match state {
                0 => "Unknown",
                1 => "Playing",
                2 => "PlayingStill",
                3 => "Paused",
                4 => "Stopped",
                5 => "Scanning",
                6 => "Idle",
                7 => "PlayingSlow",
                _ => "Other",
            };
            println!("  Playback state: {} (err={})", state_name, err);

            // Dispose
            let err = DVDDispose();
            println!("  DVDDispose: {} ({})", dvd_error_string(err), err);
        } else {
            println!("  DVD playback not available on this system.");
        }
    }

    println!("\nDone.");
}
