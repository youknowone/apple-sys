//! Check HealthKit data availability and quantity types.
//!
//! Creates HKHealthStore, checks isHealthDataAvailable, and queries
//! HKQuantityType for step count and other common types.

use apple_sys::HealthKit::*;

mod common;
use common::nsobj_to_string;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== HealthKit Store ===\n");

        // HKHealthStore
        println!("--- HKHealthStore ---");
        let available = HKHealthStore::isHealthDataAvailable();
        println!("  Health data available: {}", available.0);

        if available.0 {
            let store = HKHealthStore::alloc();
            let store_ptr = INSObject::init(&store);
            if !store_ptr.is_null() {
                let store = HKHealthStore(store_ptr);
                println!("  Store: {}", nsobj_to_string(store.0));

                // Check authorization status for step count
                let step_id = nsstring(c"HKQuantityTypeIdentifierStepCount");
                let step_type = HKQuantityType::quantityTypeForIdentifier_(step_id);

                if !step_type.0.is_null() {
                    let auth = IHKHealthStore::authorizationStatusForType_(
                        &store,
                        HKObjectType(step_type.0),
                    );
                    let auth_str = match auth {
                        0 => "NotDetermined",
                        1 => "SharingDenied",
                        2 => "SharingAuthorized",
                        _ => "Unknown",
                    };
                    println!("  Step count auth: {auth_str} ({auth})");
                }
            }
        }

        // HKQuantityType - common types
        println!("\n--- HKQuantityType (common identifiers) ---");
        let identifiers = [
            (c"HKQuantityTypeIdentifierStepCount", "Step Count"),
            (
                c"HKQuantityTypeIdentifierDistanceWalkingRunning",
                "Distance Walking/Running",
            ),
            (c"HKQuantityTypeIdentifierHeartRate", "Heart Rate"),
            (
                c"HKQuantityTypeIdentifierActiveEnergyBurned",
                "Active Energy Burned",
            ),
            (
                c"HKQuantityTypeIdentifierBasalEnergyBurned",
                "Basal Energy Burned",
            ),
            (c"HKQuantityTypeIdentifierBodyMass", "Body Mass"),
            (c"HKQuantityTypeIdentifierHeight", "Height"),
            (c"HKQuantityTypeIdentifierBloodGlucose", "Blood Glucose"),
            (
                c"HKQuantityTypeIdentifierOxygenSaturation",
                "Oxygen Saturation",
            ),
            (
                c"HKQuantityTypeIdentifierBodyTemperature",
                "Body Temperature",
            ),
        ];

        for (id_cstr, name) in &identifiers {
            let id_str = nsstring(id_cstr);
            let qt = HKQuantityType::quantityTypeForIdentifier_(id_str);
            let status = if qt.0.is_null() { "N/A" } else { "OK" };
            println!("  [{status}] {name}");
        }

        // HKUnit
        println!("\n--- HKUnit (common units) ---");
        let count_unit = HKUnit::countUnit();
        let meter_unit = HKUnit::meterUnit();
        let gram_unit = HKUnit::gramUnit();
        let second_unit = HKUnit::secondUnit();
        let joule_unit = HKUnit::jouleUnit();
        let kelvin_unit = HKUnit::kelvinUnit();

        println!("  Count:  {}", nsobj_to_string(count_unit));
        println!("  Meter:  {}", nsobj_to_string(meter_unit));
        println!("  Gram:   {}", nsobj_to_string(gram_unit));
        println!("  Second: {}", nsobj_to_string(second_unit));
        println!("  Joule:  {}", nsobj_to_string(joule_unit));
        println!("  Kelvin: {}", nsobj_to_string(kelvin_unit));

        // Create compound unit
        let bpm_str = nsstring(c"count/min");
        let bpm = HKUnit::unitFromString_(bpm_str);
        println!("  BPM:    {}", nsobj_to_string(bpm));
    }

    println!("\nDone.");
}
