//! Exercise InstallerPlugins section and state bindings.
//!
//! Creates InstallerSection and InstallerState instances, querying
//! their default property values through the generated bindings.

use apple_sys::InstallerPlugins::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== InstallerPlugins Framework ===\n");

        // InstallerSection
        println!("--- InstallerSection ---");
        let section = InstallerSection::alloc();
        let section_ptr = INSObject::init(&section);
        if !section_ptr.is_null() {
            let section = InstallerSection(section_ptr);

            let should_load = IInstallerSection::shouldLoad(&section);
            println!("  shouldLoad: {}", should_load.0);

            let title = IInstallerSection::title(&section);
            println!("  title: {}", nsstring_to_string(title));

            let bundle = IInstallerSection::bundle(&section);
            if !bundle.0.is_null() {
                println!("  bundle: {}", nsobj_to_string(bundle.0));
            } else {
                println!("  bundle: (nil)");
            }

            let shared = IInstallerSection::sharedDictionary(&section);
            if !shared.0.is_null() {
                println!("  sharedDictionary: {}", nsobj_to_string(shared.0));
            } else {
                println!("  sharedDictionary: (nil)");
            }

            let first = IInstallerSection::firstPane(&section);
            if !first.0.is_null() {
                println!("  firstPane: {}", nsobj_to_string(first.0));
            } else {
                println!("  firstPane: (nil)");
            }

            let active = IInstallerSection::activePane(&section);
            if !active.0.is_null() {
                println!("  activePane: {}", nsobj_to_string(active.0));
            } else {
                println!("  activePane: (nil)");
            }
        } else {
            println!("  (init returned nil)");
        }

        // InstallerState
        println!("\n--- InstallerState ---");
        let state = InstallerState::alloc();
        let state_ptr = INSObject::init(&state);
        if !state_ptr.is_null() {
            let state = InstallerState(state_ptr);

            let agreed = IInstallerState::licenseAgreed(&state);
            println!("  licenseAgreed: {}", agreed.0);

            let target_vol = IInstallerState::targetVolumePath(&state);
            println!("  targetVolumePath: {}", nsstring_to_string(target_vol));

            let target = IInstallerState::targetPath(&state);
            println!("  targetPath: {}", nsstring_to_string(target));

            let started = IInstallerState::installStarted(&state);
            println!("  installStarted: {}", started.0);

            let succeeded = IInstallerState::installSucceeded(&state);
            println!("  installSucceeded: {}", succeeded.0);

            let choices = IInstallerState::choiceDictionaries(&state);
            if !choices.0.is_null() {
                let count = INSArray::<id>::count(&choices);
                println!("  choiceDictionaries: {count} entries");
            } else {
                println!("  choiceDictionaries: (nil)");
            }
        } else {
            println!("  (init returned nil)");
        }
    }

    println!("\nDone.");
}
