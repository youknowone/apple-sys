//! Query MediaPlayer Now Playing info center.
//!
//! Uses MPNowPlayingInfoCenter and MPRemoteCommandCenter
//! to check media playback command availability.

use apple_sys::MediaPlayer::*;
use apple_sys::objc::id;
mod common;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MediaPlayer ===\n");

        // Now Playing Info Center
        let center = MPNowPlayingInfoCenter::defaultCenter();
        if !center.0.is_null() {
            let playback_state = IMPNowPlayingInfoCenter::playbackState(&center);
            let state_name = match playback_state {
                0 => "Unknown",
                1 => "Playing",
                2 => "Paused",
                3 => "Stopped",
                4 => "Interrupted",
                _ => "Other",
            };
            println!("Now Playing Info Center:");
            println!("  Playback state: {} ({})", state_name, playback_state);

            let info = IMPNowPlayingInfoCenter::nowPlayingInfo(&center);
            if !info.0.is_null() {
                let count = INSDictionary::<NSString, id>::count(&info);
                println!("  Now playing info keys: {}", count);
            } else {
                println!("  No now playing info set");
            }
        }

        // Remote Command Center
        let rcc = MPRemoteCommandCenter::sharedCommandCenter();
        if !rcc.0.is_null() {
            println!("\nRemote Command Center:");

            macro_rules! check_cmd {
                ($rcc:expr, $method:ident, $label:expr) => {
                    let cmd = IMPRemoteCommandCenter::$method(&$rcc);
                    if !cmd.0.is_null() {
                        let enabled = IMPRemoteCommand::isEnabled(&cmd);
                        println!(
                            "  {}: {}",
                            $label,
                            if enabled.0 { "enabled" } else { "disabled" }
                        );
                    }
                };
            }
            check_cmd!(rcc, playCommand, "Play");
            check_cmd!(rcc, pauseCommand, "Pause");
            check_cmd!(rcc, stopCommand, "Stop");
            check_cmd!(rcc, togglePlayPauseCommand, "Toggle Play/Pause");
            check_cmd!(rcc, nextTrackCommand, "Next Track");
            check_cmd!(rcc, previousTrackCommand, "Previous Track");
            check_cmd!(rcc, skipForwardCommand, "Skip Forward");
            check_cmd!(rcc, skipBackwardCommand, "Skip Backward");
            check_cmd!(rcc, likeCommand, "Like");
            check_cmd!(rcc, dislikeCommand, "Dislike");
            check_cmd!(rcc, bookmarkCommand, "Bookmark");
        }
    }

    println!("\nDone.");
}
