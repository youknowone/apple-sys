//! List connected MIDI devices, entities, and endpoints using CoreMIDI.
//!
//! Demonstrates CoreMIDI's device enumeration and property query APIs.
//! This is a pure C API example — no Objective-C message sending needed.

#![allow(non_upper_case_globals)]

use apple_sys::CoreFoundation::{
    CFIndex, CFRelease, CFStringEncoding, CFStringGetCString, CFStringRef, CFTypeRef,
};
use apple_sys::CoreMIDI::*;
use std::ffi::CStr;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;

unsafe fn midi_string_property(obj: MIDIObjectRef, property: CFStringRef) -> String {
    let mut cfstr: CFStringRef = std::ptr::null();
    let status = unsafe { MIDIObjectGetStringProperty(obj, property, &mut cfstr) };
    if status != 0 || cfstr.is_null() {
        return "(unknown)".to_string();
    }
    let result = unsafe { cfstring_to_string(cfstr) };
    unsafe { CFRelease(cfstr as CFTypeRef) };
    result
}

unsafe fn cfstring_to_string(cfstr: CFStringRef) -> String {
    if cfstr.is_null() {
        return String::new();
    }
    let mut buf = [0i8; 256];
    let ok = unsafe {
        CFStringGetCString(
            cfstr,
            buf.as_mut_ptr(),
            buf.len() as CFIndex,
            kCFStringEncodingUTF8,
        )
    };
    if ok != 0 {
        unsafe { CStr::from_ptr(buf.as_ptr()) }
            .to_string_lossy()
            .into_owned()
    } else {
        "<CFString>".to_string()
    }
}

fn main() {
    unsafe {
        println!("=== CoreMIDI Device Enumeration ===\n");

        // Summary counts
        let num_devices = MIDIGetNumberOfDevices();
        let num_sources = MIDIGetNumberOfSources();
        let num_dests = MIDIGetNumberOfDestinations();
        let num_external = MIDIGetNumberOfExternalDevices();

        println!("Devices:      {num_devices}");
        println!("Sources:      {num_sources}");
        println!("Destinations: {num_dests}");
        println!("External:     {num_external}");

        // Enumerate devices
        if num_devices == 0 {
            println!("\nNo MIDI devices found.");
        }

        for d in 0..num_devices {
            let device = MIDIGetDevice(d);
            let name = midi_string_property(device, kMIDIPropertyName);
            let manufacturer = midi_string_property(device, kMIDIPropertyManufacturer);
            let model = midi_string_property(device, kMIDIPropertyModel);

            println!("\n--- Device #{d}: {name} ---");
            println!("  Manufacturer: {manufacturer}");
            println!("  Model:        {model}");

            let num_entities = MIDIDeviceGetNumberOfEntities(device);
            println!("  Entities:     {num_entities}");

            for e in 0..num_entities {
                let entity = MIDIDeviceGetEntity(device, e);
                let ename = midi_string_property(entity, kMIDIPropertyName);

                let nsrc = MIDIEntityGetNumberOfSources(entity);
                let ndst = MIDIEntityGetNumberOfDestinations(entity);
                println!("    Entity #{e}: {ename} ({nsrc} src, {ndst} dst)");

                for s in 0..nsrc {
                    let ep = MIDIEntityGetSource(entity, s);
                    let epname = midi_string_property(ep, kMIDIPropertyName);
                    println!("      Source #{s}: {epname}");
                }

                for d in 0..ndst {
                    let ep = MIDIEntityGetDestination(entity, d);
                    let epname = midi_string_property(ep, kMIDIPropertyName);
                    println!("      Dest   #{d}: {epname}");
                }
            }
        }

        // List all sources and destinations globally
        if num_sources > 0 {
            println!("\n--- All MIDI Sources ---");
            for i in 0..num_sources {
                let ep = MIDIGetSource(i);
                let name = midi_string_property(ep, kMIDIPropertyName);
                println!("  [{i}] {name}");
            }
        }

        if num_dests > 0 {
            println!("\n--- All MIDI Destinations ---");
            for i in 0..num_dests {
                let ep = MIDIGetDestination(i);
                let name = midi_string_property(ep, kMIDIPropertyName);
                println!("  [{i}] {name}");
            }
        }
    }

    println!("\nDone.");
}
