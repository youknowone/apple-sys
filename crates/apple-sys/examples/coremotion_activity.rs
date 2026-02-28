//! Check CoreMotion activity and pedometer capabilities.
//!
//! Queries CMMotionActivityManager, CMPedometer, and CMAltimeter for
//! hardware capability availability.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::CoreMotion::*;
use apple_sys::Foundation::NSAutoreleasePool;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreMotion Capability Check ===\n");

        // CMMotionActivityManager
        println!("--- CMMotionActivityManager ---");
        let available = CMMotionActivityManager::isActivityAvailable();
        println!("  Activity available: {}", available.0);

        let auth_status = CMMotionActivityManager::authorizationStatus();
        let auth_str = match auth_status {
            0 => "NotDetermined",
            1 => "Restricted",
            2 => "Denied",
            3 => "Authorized",
            _ => "Unknown",
        };
        println!("  Authorization status: {auth_str} ({auth_status})");

        // CMPedometer
        println!("\n--- CMPedometer ---");
        let step_counting = CMPedometer::isStepCountingAvailable();
        let distance = CMPedometer::isDistanceAvailable();
        let floor_counting = CMPedometer::isFloorCountingAvailable();
        let pace = CMPedometer::isPaceAvailable();
        let cadence = CMPedometer::isCadenceAvailable();
        let event_tracking = CMPedometer::isPedometerEventTrackingAvailable();

        println!("  Step counting:  {}", step_counting.0);
        println!("  Distance:       {}", distance.0);
        println!("  Floor counting: {}", floor_counting.0);
        println!("  Pace:           {}", pace.0);
        println!("  Cadence:        {}", cadence.0);
        println!("  Event tracking: {}", event_tracking.0);

        let auth = CMPedometer::authorizationStatus();
        let auth_str = match auth {
            0 => "NotDetermined",
            1 => "Restricted",
            2 => "Denied",
            3 => "Authorized",
            _ => "Unknown",
        };
        println!("  Authorization:  {auth_str} ({auth})");

        // CMAltimeter
        println!("\n--- CMAltimeter ---");
        let rel_alt = CMAltimeter::isRelativeAltitudeAvailable();
        let abs_alt = CMAltimeter::isAbsoluteAltitudeAvailable();
        println!("  Relative altitude: {}", rel_alt.0);
        println!("  Absolute altitude: {}", abs_alt.0);

        let auth = CMAltimeter::authorizationStatus();
        let auth_str = match auth {
            0 => "NotDetermined",
            1 => "Restricted",
            2 => "Denied",
            3 => "Authorized",
            _ => "Unknown",
        };
        println!("  Authorization:     {auth_str} ({auth})");

        // CMMotionManager
        println!("\n--- CMMotionManager ---");
        let mgr = CMMotionManager::alloc();
        let mgr_ptr = INSObject::init(&mgr);
        if !mgr_ptr.is_null() {
            let mgr = CMMotionManager(mgr_ptr);
            let accel = ICMMotionManager::isAccelerometerAvailable(&mgr);
            let gyro = ICMMotionManager::isGyroAvailable(&mgr);
            let magneto = ICMMotionManager::isMagnetometerAvailable(&mgr);
            let device_motion = ICMMotionManager::isDeviceMotionAvailable(&mgr);

            println!("  Accelerometer:  {}", accel.0);
            println!("  Gyroscope:      {}", gyro.0);
            println!("  Magnetometer:   {}", magneto.0);
            println!("  Device motion:  {}", device_motion.0);
        }
    }

    println!("\nDone.");
}
