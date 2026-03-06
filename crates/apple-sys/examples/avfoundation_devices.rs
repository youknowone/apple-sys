//! List available audio and video capture devices using AVFoundation.
//!
//! Enumerates cameras and microphones connected to the system.

use apple_sys::AVFoundation::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::objc::id;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AVFoundation Capture Device Enumeration ===\n");

        // Default video device
        let video_type = nsstring(c"vide"); // AVMediaTypeVideo
        let default_video = AVCaptureDevice::defaultDeviceWithMediaType_(video_type);
        if !default_video.0.is_null() {
            let name = default_video.localizedName();
            println!("Default video device: {}", nsstring_to_string(name));
        } else {
            println!("Default video device: (none)");
        }

        // Default audio device
        let audio_type = nsstring(c"soun"); // AVMediaTypeAudio
        let default_audio = AVCaptureDevice::defaultDeviceWithMediaType_(audio_type);
        if !default_audio.0.is_null() {
            let name = default_audio.localizedName();
            println!("Default audio device: {}", nsstring_to_string(name));
        } else {
            println!("Default audio device: (none)");
        }

        // List all devices (deprecated but reliable across macOS versions)
        #[allow(deprecated)]
        let all_devices = AVCaptureDevice::devices();
        let count = INSArray::<id>::count(&all_devices);
        println!("\nTotal capture devices: {count}");

        // Video devices
        println!("\n--- Video Devices ---");
        let mut found = false;
        for i in 0..count {
            let dev: id = INSArray::<id>::objectAtIndex_(&all_devices, i);
            let dev = AVCaptureDevice(dev);
            let has_video = dev.hasMediaType_(video_type);
            if has_video.0 {
                found = true;
                print_device_info(&dev, i as usize);
            }
        }
        if !found {
            println!("  (none)");
        }

        // Audio devices
        println!("\n--- Audio Devices ---");
        found = false;
        for i in 0..count {
            let dev: id = INSArray::<id>::objectAtIndex_(&all_devices, i);
            let dev = AVCaptureDevice(dev);
            let has_audio = dev.hasMediaType_(audio_type);
            if has_audio.0 {
                found = true;
                print_device_info(&dev, i as usize);
            }
        }
        if !found {
            println!("  (none)");
        }
    }

    println!("\nDone.");
}

unsafe fn print_device_info(dev: &AVCaptureDevice, idx: usize) {
    let name = unsafe { dev.localizedName() };
    let model = unsafe { dev.modelID() };
    let uid = unsafe { dev.uniqueID() };
    let connected = unsafe { dev.isConnected() };

    println!("  Device #{idx}:");
    println!("    Name:      {}", unsafe { nsstring_to_string(name) });
    println!("    Model:     {}", unsafe { nsstring_to_string(model) });
    println!("    UniqueID:  {}", unsafe { nsstring_to_string(uid) });
    println!("    Connected: {}", connected.0);
}
