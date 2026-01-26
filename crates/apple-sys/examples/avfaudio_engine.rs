//! Create an AVAudioEngine and inspect its input/output nodes and formats.
//!
//! Uses the AVFAudio framework to explore the audio processing graph.

use apple_sys::AVFAudio::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AVFAudio Engine ===\n");

        // Create AVAudioEngine
        let engine = AVAudioEngine::alloc();
        let engine_ptr = INSObject::init(&engine);
        if engine_ptr.is_null() {
            println!("Failed to create AVAudioEngine.");
            return;
        }
        let engine = AVAudioEngine(engine_ptr);
        println!("AVAudioEngine created.");

        // Output node
        let output_node = engine.outputNode();
        if !output_node.0.is_null() {
            println!("\n--- Output Node ---");
            let out_fmt = output_node.outputFormatForBus_(0);
            if !out_fmt.0.is_null() {
                let sample_rate = out_fmt.sampleRate();
                let channels = out_fmt.channelCount();
                let interleaved = out_fmt.isInterleaved();
                println!("  Sample rate:  {sample_rate:.0} Hz");
                println!("  Channels:     {channels}");
                println!("  Interleaved:  {}", interleaved.0);
                println!("  Format:       {}", nsobj_to_string(out_fmt.0));
            }
        }

        // Input node
        let input_node = engine.inputNode();
        if !input_node.0.is_null() {
            println!("\n--- Input Node ---");
            let in_fmt = input_node.outputFormatForBus_(0);
            if !in_fmt.0.is_null() {
                let sample_rate = in_fmt.sampleRate();
                let channels = in_fmt.channelCount();
                let interleaved = in_fmt.isInterleaved();
                println!("  Sample rate:  {sample_rate:.0} Hz");
                println!("  Channels:     {channels}");
                println!("  Interleaved:  {}", interleaved.0);
                println!("  Format:       {}", nsobj_to_string(in_fmt.0));
            }
        } else {
            println!("\nNo input node available (no microphone?).");
        }

        // Main mixer node
        let mixer = engine.mainMixerNode();
        if !mixer.0.is_null() {
            println!("\n--- Main Mixer Node ---");
            let volume = mixer.volume();
            // PAVAudioStereoMixing has pan() but is not impl'd for AVAudioMixerNode
            // (missing protocol conformance in bindings).
            let pan: f32 = objc2::msg_send![&*mixer, pan];
            let out_fmt = mixer.outputFormatForBus_(0);
            println!("  Volume: {volume:.2}");
            println!("  Pan:    {pan:.2}");
            if !out_fmt.0.is_null() {
                let sr = out_fmt.sampleRate();
                let ch = out_fmt.channelCount();
                println!("  Output: {sr:.0} Hz, {ch} ch");
            }
        }

        let running = engine.isRunning();
        println!("\nEngine running: {}", running.0);
    }

    println!("\nDone.");
}
