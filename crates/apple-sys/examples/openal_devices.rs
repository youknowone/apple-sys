//! Enumerate OpenAL audio devices.
//!
//! Lists default output/capture devices and all available devices
//! using the ALC_ENUMERATE_ALL_EXT extension.

use apple_sys::OpenAL::*;
use std::ffi::CStr;

const ALC_DEFAULT_DEVICE_SPECIFIER: i32 = 0x1004;
const ALC_DEVICE_SPECIFIER: i32 = 0x1005;
const ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER: i32 = 0x311;
const ALC_CAPTURE_DEVICE_SPECIFIER: i32 = 0x310;

/// Parse a double-null-terminated string list into individual strings.
unsafe fn parse_device_list(ptr: *const i8) -> Vec<String> {
    if ptr.is_null() {
        return vec![];
    }
    let mut devices = vec![];
    let mut cur = ptr;
    loop {
        let s = unsafe { CStr::from_ptr(cur) };
        if s.to_bytes().is_empty() {
            break;
        }
        devices.push(s.to_string_lossy().into_owned());
        cur = unsafe { cur.add(s.to_bytes_with_nul().len()) };
    }
    devices
}

fn main() {
    unsafe {
        println!("=== OpenAL Audio Devices ===\n");

        // Default output device
        let default = alcGetString(std::ptr::null_mut(), ALC_DEFAULT_DEVICE_SPECIFIER);
        if !default.is_null() {
            let name = CStr::from_ptr(default).to_string_lossy();
            println!("Default output device: {}", name);
        }

        // All output devices
        let all = alcGetString(std::ptr::null_mut(), ALC_DEVICE_SPECIFIER);
        let devices = parse_device_list(all);
        println!("\nOutput devices ({}):", devices.len());
        for (i, d) in devices.iter().enumerate() {
            println!("  {}: {}", i, d);
        }

        // Default capture device
        let cap_default = alcGetString(std::ptr::null_mut(), ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER);
        if !cap_default.is_null() {
            let name = CStr::from_ptr(cap_default).to_string_lossy();
            println!("\nDefault capture device: {}", name);
        }

        // All capture devices
        let cap_all = alcGetString(std::ptr::null_mut(), ALC_CAPTURE_DEVICE_SPECIFIER);
        let cap_devices = parse_device_list(cap_all);
        println!("\nCapture devices ({}):", cap_devices.len());
        for (i, d) in cap_devices.iter().enumerate() {
            println!("  {}: {}", i, d);
        }

        // Open default device and query info
        let device = alcOpenDevice(std::ptr::null());
        if !device.is_null() {
            let mut major: i32 = 0;
            let mut minor: i32 = 0;
            alcGetIntegerv(device, 0x1000, 1, &mut major); // ALC_MAJOR_VERSION
            alcGetIntegerv(device, 0x1001, 1, &mut minor); // ALC_MINOR_VERSION
            println!("\nOpenAL version: {}.{}", major, minor);
            alcCloseDevice(device);
        }
    }

    println!("\nDone.");
}
