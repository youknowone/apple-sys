//! Query RPScreenRecorder status using ReplayKit.
//!
//! Retrieves the shared screen recorder and checks availability,
//! microphone, and camera recording states via generated bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::ReplayKit::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ReplayKit Screen Recorder ===\n");

        let recorder = RPScreenRecorder::sharedRecorder();
        if !recorder.0.is_null() {
            println!("Shared recorder: {}", nsobj_to_string(recorder.0));

            let available = IRPScreenRecorder::isAvailable(&recorder);
            println!("Available: {}", available.0);

            let recording = IRPScreenRecorder::isRecording(&recorder);
            println!("Recording: {}", recording.0);

            let mic_enabled = IRPScreenRecorder::isMicrophoneEnabled(&recorder);
            println!("Microphone enabled: {}", mic_enabled.0);

            let camera_enabled = IRPScreenRecorder::isCameraEnabled(&recorder);
            println!("Camera enabled: {}", camera_enabled.0);
        } else {
            println!("Failed to get shared recorder.");
        }
    }

    println!("\nDone.");
}
