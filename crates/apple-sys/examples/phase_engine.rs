//! Create PHASEEngine and inspect its properties via generated bindings.
//!
//! Demonstrates the PHASE (Physical Audio Spatialization Engine) framework
//! by creating an engine and querying its configuration.

use apple_sys::PHASE::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== PHASE (Physical Audio Spatialization Engine) ===\n");

        // Create PHASEEngine
        println!("--- PHASEEngine ---");
        {
            // PHASEUpdateMode: 0 = automatic, 1 = manual
            let engine = PHASEEngine::alloc();
            let engine = PHASEEngine(IPHASEEngine::initWithUpdateMode_(&engine, 0));

            if !engine.0.is_null() {
                println!("  Engine: {}", nsobj_to_string(engine.0));

                let output_spatial = IPHASEEngine::outputSpatializationMode(&engine);
                let spatial_name = match output_spatial {
                    0 => "Automatic",
                    1 => "AlwaysUseBinaural",
                    2 => "AlwaysUseChannelBased",
                    _ => "Unknown",
                };
                println!(
                    "  Output spatialization mode: {} ({})",
                    spatial_name, output_spatial
                );

                let render_fmt = IPHASEEngine::renderingState(&engine);
                let render_name = match render_fmt {
                    0 => "Stopped",
                    1 => "Started",
                    2 => "Paused",
                    _ => "Unknown",
                };
                println!("  Rendering state: {} ({})", render_name, render_fmt);

                // Root object
                let root = IPHASEEngine::rootObject(&engine);
                if !root.0.is_null() {
                    println!("  Root object: {}", nsobj_to_string(root.0));
                }

                // Asset registry
                let registry = IPHASEEngine::assetRegistry(&engine);
                if !registry.0.is_null() {
                    println!("  Asset registry: {}", nsobj_to_string(registry.0));
                }

                // Stop and clean up
                IPHASEEngine::stop(&engine);
                println!("\n  Engine stopped.");
            } else {
                println!("  Failed to create PHASEEngine");
            }
        }
    }

    println!("\nDone.");
}
