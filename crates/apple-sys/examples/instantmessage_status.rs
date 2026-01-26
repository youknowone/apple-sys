//! Query InstantMessage AV manager and services.
//!
//! Gets IMAVManager.sharedAVManager state and queries IMService.allServices
//! for available messaging services.

use apple_sys::InstantMessage::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== InstantMessage Framework ===\n");

        // IMAVManager
        println!("--- IMAVManager ---");
        let mgr = IMAVManager::sharedAVManager();
        if !mgr.0.is_null() {
            println!("  Shared manager: {}", nsobj_to_string(mgr.0));

            let state = IIMAVManager::state(&mgr);
            let state_str = match state {
                0 => "Idle",
                1 => "Pending",
                2 => "Running",
                _ => "Unknown",
            };
            println!("  State: {state_str} ({state})");
        } else {
            println!("  sharedAVManager returned nil");
        }

        // IMAVButton factory methods
        println!("\n--- IMAVButton ---");
        let play_btn = IMAVButton::playPauseButton();
        if !play_btn.0.is_null() {
            let state = IIMAVButton::state(&play_btn);
            println!("  Play/Pause button state: {state}");
        }

        let mute_btn = IMAVButton::muteButton();
        if !mute_btn.0.is_null() {
            let state = IIMAVButton::state(&mute_btn);
            println!("  Mute button state: {state}");
        }

        // IMAVSlider factory
        let time_slider = IMAVSlider::timeSlider();
        if !time_slider.0.is_null() {
            let min = IIMAVSlider::minValue(&time_slider);
            let max = IIMAVSlider::maxValue(&time_slider);
            println!("  Time slider range: {min}..{max}");
        }

        // IMAVControlBar
        println!("\n--- IMAVControlBar ---");
        let bar = IMAVControlBar::alloc();
        let bar_ptr = INSObject::init(&bar);
        if !bar_ptr.is_null() {
            let bar = IMAVControlBar(bar_ptr);
            if !play_btn.0.is_null() {
                IIMAVControlBar::addControl_(&bar, IMAVControl(play_btn.0));
            }
            let controls = IIMAVControlBar::controls(&bar);
            if !controls.0.is_null() {
                let count = INSArray::<id>::count(&controls);
                println!("  Controls in bar: {count}");
            }
            IIMAVControlBar::removeAllControls(&bar);
        }

        // IMService
        println!("\n--- IMService ---");
        let services = IMService::allServices();
        if !services.0.is_null() {
            let count = INSArray::<id>::count(&services);
            println!("  Available services: {count}");

            for i in 0..count {
                let svc_id = INSArray::<id>::objectAtIndex_(&services, i);
                let svc = IMService(svc_id);
                let name = IIMService::name(&svc);
                let status = IIMService::status(&svc);
                let status_str = match status {
                    0 => "Disconnected",
                    1 => "Connecting",
                    2 => "Connected",
                    3 => "Disconnecting",
                    _ => "Unknown",
                };
                println!("    {}: {} ({})", i, nsstring_to_string(name), status_str);
            }
        } else {
            println!("  allServices returned nil");
        }

        // IMService notification center
        let nc = IMService::notificationCenter();
        if !nc.0.is_null() {
            println!("  Notification center: {}", nsobj_to_string(nc.0));
        }
    }

    println!("\nDone.");
}
