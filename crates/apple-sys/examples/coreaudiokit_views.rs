//! Inspect CoreAudioKit view classes and audio unit view configuration.
//!
//! Creates an AUAudioUnitViewConfiguration with specific dimensions
//! and queries its properties through the generated bindings.

use apple_sys::CoreAudioKit::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreAudioKit View Classes ===\n");

        // AUAudioUnitViewConfiguration
        println!("--- AUAudioUnitViewConfiguration ---");
        let config = AUAudioUnitViewConfiguration::alloc();
        let config_ptr = IAUAudioUnitViewConfiguration::initWithWidth_height_hostHasController_(
            &config,
            400.0,
            300.0,
            BOOL(false),
        );
        if !config_ptr.is_null() {
            let config = AUAudioUnitViewConfiguration(config_ptr);
            let width = IAUAudioUnitViewConfiguration::width(&config);
            let height = IAUAudioUnitViewConfiguration::height(&config);
            let has_ctrl = IAUAudioUnitViewConfiguration::hostHasController(&config);
            println!(
                "  Created config: {width}x{height}, hostHasController={}",
                has_ctrl.0
            );
        } else {
            println!("  (failed to create AUAudioUnitViewConfiguration)");
        }

        // Create another config with different dimensions
        let config2 = AUAudioUnitViewConfiguration::alloc();
        let config2_ptr = IAUAudioUnitViewConfiguration::initWithWidth_height_hostHasController_(
            &config2,
            800.0,
            600.0,
            BOOL(true),
        );
        if !config2_ptr.is_null() {
            let config2 = AUAudioUnitViewConfiguration(config2_ptr);
            let width = IAUAudioUnitViewConfiguration::width(&config2);
            let height = IAUAudioUnitViewConfiguration::height(&config2);
            let has_ctrl = IAUAudioUnitViewConfiguration::hostHasController(&config2);
            println!(
                "  Created config: {width}x{height}, hostHasController={}",
                has_ctrl.0
            );
        }

        // AUGenericViewController
        println!("\n--- AUGenericViewController ---");
        let vc = AUGenericViewController::alloc();
        let vc_ptr = INSObject::init(&vc);
        if !vc_ptr.is_null() {
            let vc = AUGenericViewController(vc_ptr);
            let au = IAUGenericViewController::auAudioUnit(&vc);
            println!("  AUGenericViewController created.");
            println!("  auAudioUnit: {}", nsobj_to_string(au.0));
        } else {
            println!("  (failed to create AUGenericViewController)");
        }
    }

    println!("\nDone.");
}
