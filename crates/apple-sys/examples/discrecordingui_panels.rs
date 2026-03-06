//! Inspect DiscRecordingUI burn and erase setup panels.
//!
//! Creates DRBurnSetupPanel and DREraseSetupPanel instances
//! and queries their properties.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::DiscRecordingUI::*;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== DiscRecordingUI Setup Panels ===\n");

        // DRBurnSetupPanel
        println!("--- DRBurnSetupPanel ---");
        let panel = DRBurnSetupPanel::setupPanel();
        if !panel.0.is_null() {
            println!("  Panel: {}", nsobj_to_string(panel.0));
            let burn_obj = IDRBurnSetupPanel::burnObject(&panel);
            println!("  Burn object: {}", nsobj_to_string(burn_obj.0));
        } else {
            println!("  setupPanel returned nil");
        }

        // DREraseSetupPanel
        println!("\n--- DREraseSetupPanel ---");
        let panel = DREraseSetupPanel::setupPanel();
        if !panel.0.is_null() {
            println!("  Panel: {}", nsobj_to_string(panel.0));
            let erase_obj = IDREraseSetupPanel::eraseObject(&panel);
            println!("  Erase object: {}", nsobj_to_string(erase_obj.0));
        } else {
            println!("  setupPanel returned nil");
        }

        // DRBurnProgressPanel
        println!("\n--- DRBurnProgressPanel ---");
        let panel = DRBurnProgressPanel::progressPanel();
        if !panel.0.is_null() {
            println!("  Panel: {}", nsobj_to_string(panel.0));
            let verbose = IDRBurnProgressPanel::verboseProgressStatus(&panel);
            println!("  Verbose progress: {}", verbose.0);
        } else {
            println!("  progressPanel returned nil");
        }

        // DREraseProgressPanel
        println!("\n--- DREraseProgressPanel ---");
        let panel = DREraseProgressPanel::progressPanel();
        if !panel.0.is_null() {
            println!("  Panel: {}", nsobj_to_string(panel.0));
        } else {
            println!("  progressPanel returned nil");
        }
    }

    println!("\nDone.");
}
