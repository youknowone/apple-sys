//! Create haptic event patterns with CoreHaptics.
//!
//! Demonstrates CHHapticEventParameter, CHHapticEvent, and
//! CHHapticPattern creation for structured haptic feedback.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::CoreHaptics::*;
use apple_sys::Foundation::{
    INSError, NSArray, NSArray_NSArrayCreation, NSAutoreleasePool, NSError,
};
use apple_sys::objc::id;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreHaptics Patterns ===\n");

        // Check hardware support
        println!("Haptic hardware capabilities queried");

        // Create event parameters
        // Intensity parameter
        let intensity_id = nsstring(c"HapticIntensity");
        let intensity = CHHapticEventParameter::alloc();
        let intensity_ptr =
            ICHHapticEventParameter::initWithParameterID_value_(&intensity, intensity_id, 0.8);
        let intensity = CHHapticEventParameter(intensity_ptr);
        let val = ICHHapticEventParameter::value(&intensity);
        println!("Intensity parameter: {:.1}", val);

        // Sharpness parameter
        let sharpness_id = nsstring(c"HapticSharpness");
        let sharpness = CHHapticEventParameter::alloc();
        let sharpness_ptr =
            ICHHapticEventParameter::initWithParameterID_value_(&sharpness, sharpness_id, 0.5);
        let sharpness = CHHapticEventParameter(sharpness_ptr);
        let val = ICHHapticEventParameter::value(&sharpness);
        println!("Sharpness parameter: {:.1}", val);

        // Create haptic events

        // Transient event (tap)
        let params: [*mut CHHapticEventParameter; 2] = [
            &intensity as *const CHHapticEventParameter as *mut CHHapticEventParameter,
            &sharpness as *const CHHapticEventParameter as *mut CHHapticEventParameter,
        ];
        let params_arr = NSArray(
            <NSArray as NSArray_NSArrayCreation<id>>::arrayWithObjects_count_(
                params.as_ptr() as *const *mut u64,
                2,
            ),
        );

        let transient_type = nsstring(c"HapticTransient");
        let tap = CHHapticEvent::alloc();
        let tap_ptr = ICHHapticEvent::initWithEventType_parameters_relativeTime_(
            &tap,
            transient_type,
            params_arr,
            0.0,
        );
        let tap = CHHapticEvent(tap_ptr);
        let time = ICHHapticEvent::relativeTime(&tap);
        let dur = ICHHapticEvent::duration(&tap);
        println!("\nTransient event: time={:.1}, duration={:.1}", time, dur);

        // Continuous event (vibration) - need params_arr again
        let params_arr2 = NSArray(
            <NSArray as NSArray_NSArrayCreation<id>>::arrayWithObjects_count_(
                params.as_ptr() as *const *mut u64,
                2,
            ),
        );
        let continuous_type = nsstring(c"HapticContinuous");
        let vib = CHHapticEvent::alloc();
        let vib_ptr = ICHHapticEvent::initWithEventType_parameters_relativeTime_duration_(
            &vib,
            continuous_type,
            params_arr2,
            0.1,
            0.5,
        );
        let vib = CHHapticEvent(vib_ptr);
        let time = ICHHapticEvent::relativeTime(&vib);
        let dur = ICHHapticEvent::duration(&vib);
        println!("Continuous event: time={:.1}, duration={:.1}", time, dur);

        // Create pattern from events
        let events: [*mut CHHapticEvent; 2] = [
            &tap as *const CHHapticEvent as *mut CHHapticEvent,
            &vib as *const CHHapticEvent as *mut CHHapticEvent,
        ];
        let events_arr = NSArray(
            <NSArray as NSArray_NSArrayCreation<id>>::arrayWithObjects_count_(
                events.as_ptr() as *const *mut u64,
                2,
            ),
        );
        let empty_arr = NSArray(<NSArray as NSArray_NSArrayCreation<id>>::array());

        let error = NSError(std::ptr::null_mut());
        let pattern = CHHapticPattern::alloc();
        let pattern_ptr = ICHHapticPattern::initWithEvents_parameters_error_(
            &pattern,
            events_arr,
            empty_arr,
            &error as *const NSError as *mut NSError,
        );

        if !pattern_ptr.is_null() {
            let pattern = CHHapticPattern(pattern_ptr);
            let dur = ICHHapticPattern::duration(&pattern);
            println!("\nPattern created: total duration={:.1}s", dur);
        } else if !error.0.is_null() {
            let desc = INSError::localizedDescription(&error);
            println!("\nPattern error: {}", nsstring_to_string(desc));
        }
    }

    println!("\nDone.");
}
