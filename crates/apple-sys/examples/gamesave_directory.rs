//! Inspect GameSave synced directory state types.
//!
//! Creates GSSyncedDirectoryState and GSSyncedDirectoryVersion to exercise
//! the generated GameSave bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::GameSave::*;
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== GameSave Framework ===\n");

        // GSSyncedDirectoryVersion
        println!("--- GSSyncedDirectoryVersion ---");
        let ver = GSSyncedDirectoryVersion::alloc();
        let ver_ptr = IGSSyncedDirectoryVersion::init(&ver);
        if !ver_ptr.is_null() {
            let ver = GSSyncedDirectoryVersion(ver_ptr);
            let is_local = IGSSyncedDirectoryVersion::isLocal(&ver);
            println!("  isLocal: {}", is_local.0);

            let name = IGSSyncedDirectoryVersion::localizedNameOfSavingComputer(&ver);
            println!("  savingComputer: {}", nsstring_to_string(name));

            let url = IGSSyncedDirectoryVersion::url(&ver);
            println!("  URL: {}", nsobj_to_string(url.0));

            let desc = IGSSyncedDirectoryVersion::description(&ver);
            println!("  Description: {}", nsstring_to_string(desc));
        } else {
            println!("  (init returned nil)");
        }

        // GSSyncedDirectoryState
        println!("\n--- GSSyncedDirectoryState ---");
        let state = GSSyncedDirectoryState::alloc();
        let state_ptr = IGSSyncedDirectoryState::init(&state);
        if !state_ptr.is_null() {
            let state = GSSyncedDirectoryState(state_ptr);
            let sync_state = IGSSyncedDirectoryState::state(&state);
            let state_name = match sync_state {
                0 => "Unknown",
                1 => "Pending",
                2 => "InProgress",
                3 => "Succeeded",
                4 => "Failed",
                _ => "Other",
            };
            println!("  Sync state: {state_name} ({sync_state})");

            let url = IGSSyncedDirectoryState::url(&state);
            println!("  URL: {}", nsobj_to_string(url.0));

            let conflicts = IGSSyncedDirectoryState::conflictedVersions(&state);
            if !conflicts.0.is_null() {
                let count = INSArray::<id>::count(&conflicts);
                println!("  Conflicted versions: {count}");
            } else {
                println!("  Conflicted versions: (nil)");
            }

            let err = IGSSyncedDirectoryState::error(&state);
            if !err.0.is_null() {
                println!("  Error: {}", nsobj_to_string(err.0));
            } else {
                println!("  Error: (none)");
            }
        } else {
            println!("  (init returned nil)");
        }

        // GSSyncedDirectory
        println!("\n--- GSSyncedDirectory ---");
        let dir = GSSyncedDirectory::alloc();
        let dir_ptr = IGSSyncedDirectory::init(&dir);
        if !dir_ptr.is_null() {
            let dir = GSSyncedDirectory(dir_ptr);
            let dir_state = IGSSyncedDirectory::directoryState(&dir);
            if !dir_state.0.is_null() {
                let s = IGSSyncedDirectoryState::state(&dir_state);
                println!("  Directory state: {s}");
            } else {
                println!("  Directory state: (nil)");
            }
        } else {
            println!("  (init returned nil - requires container identifier)");
        }
    }

    println!("\nDone.");
}
