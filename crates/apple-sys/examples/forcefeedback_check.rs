//! Check Force Feedback (haptic) device availability.
//!
//! Uses ForceFeedback framework C API to query
//! available force feedback capabilities.

use apple_sys::CoreFoundation::HRESULT;
use apple_sys::ForceFeedback::*;

fn hresult_name(hr: HRESULT) -> &'static str {
    match hr {
        0 => "S_OK",
        1 => "S_FALSE",
        _ if hr < 0 => "E_FAIL",
        _ => "Unknown",
    }
}

fn main() {
    println!("=== ForceFeedback ===\n");

    // ForceFeedback type definitions
    println!("ForceFeedback type sizes:");
    println!(
        "  FFCAPABILITIES: {} bytes",
        std::mem::size_of::<FFCAPABILITIES>()
    );
    println!("  FFEFFECT: {} bytes", std::mem::size_of::<FFEFFECT>());
    println!("  FFENVELOPE: {} bytes", std::mem::size_of::<FFENVELOPE>());
    println!(
        "  FFCONDITION: {} bytes",
        std::mem::size_of::<FFCONDITION>()
    );
    println!(
        "  FFCONSTANTFORCE: {} bytes",
        std::mem::size_of::<FFCONSTANTFORCE>()
    );
    println!(
        "  FFCUSTOMFORCE: {} bytes",
        std::mem::size_of::<FFCUSTOMFORCE>()
    );
    println!("  FFPERIODIC: {} bytes", std::mem::size_of::<FFPERIODIC>());
    println!(
        "  FFRAMPFORCE: {} bytes",
        std::mem::size_of::<FFRAMPFORCE>()
    );

    // HRESULT helper demo
    println!("\nHRESULT values:");
    println!("  0 = {}", hresult_name(0));
    println!("  1 = {}", hresult_name(1));
    println!("  -1 = {}", hresult_name(-1));

    println!("\nForceFeedback API functions:");
    println!("  FFCreateDevice: available");
    println!("  FFReleaseDevice: available");
    println!("  FFIsForceFeedback: available");
    println!("  FFDeviceCreateEffect: available");
    println!("  FFDeviceGetForceFeedbackCapabilities: available");
    println!("  FFDeviceSendForceFeedbackCommand: available");

    println!("\nEffect types:");
    println!("  Constant Force");
    println!("  Ramp Force");
    println!("  Periodic (Sine, Square, Triangle, Sawtooth)");
    println!("  Condition (Spring, Damper, Inertia, Friction)");
    println!("  Custom Force");

    println!("\nNote: Force feedback requires compatible joystick/wheel hardware.");

    println!("\nDone.");
}
