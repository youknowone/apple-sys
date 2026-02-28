//! Inspect AutomaticAssessmentConfiguration framework.
//!
//! Creates an AEAssessmentConfiguration and examines its properties.

use apple_sys::AutomaticAssessmentConfiguration::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AutomaticAssessmentConfiguration ===\n");

        // Create AEAssessmentConfiguration
        let config = AEAssessmentConfiguration::alloc();
        let config_ptr = INSObject::init(&config);
        if config_ptr.is_null() {
            println!("Failed to create AEAssessmentConfiguration.");
            return;
        }
        let config = AEAssessmentConfiguration(config_ptr);
        println!("AEAssessmentConfiguration created.");

        // Query default property values
        let autocorrect = IAEAssessmentConfiguration::autocorrectMode(&config);
        println!("Autocorrect mode: {autocorrect}");

        let spell_check = IAEAssessmentConfiguration::allowsSpellCheck(&config);
        println!("Allows spell check: {}", spell_check.0);

        let predictive = IAEAssessmentConfiguration::allowsPredictiveKeyboard(&config);
        println!("Allows predictive keyboard: {}", predictive.0);

        let keyboard_shortcuts = IAEAssessmentConfiguration::allowsKeyboardShortcuts(&config);
        println!("Allows keyboard shortcuts: {}", keyboard_shortcuts.0);

        let activity_cont = IAEAssessmentConfiguration::allowsActivityContinuation(&config);
        println!("Allows activity continuation: {}", activity_cont.0);

        let accessible = IAEAssessmentConfiguration::allowsAccessibilitySpeech(&config);
        println!("Allows accessibility speech: {}", accessible.0);

        // Check AEAssessmentSession class
        let session = AEAssessmentSession::alloc();
        let session_ptr = IAEAssessmentSession::initWithConfiguration_(&session, config);
        if !session_ptr.is_null() {
            let session = AEAssessmentSession(session_ptr);
            println!("\nAEAssessmentSession created.");
            let active = IAEAssessmentSession::isActive(&session);
            println!("Session active: {}", active.0);
        }
    }

    println!("\nDone.");
}
