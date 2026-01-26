//! Verify SensorKit bindings by creating an SRSensorReader, querying
//! authorization status, and inspecting SRDevice and SRFetchRequest.

use apple_sys::SensorKit::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SensorKit Sensor Reader ===\n");

        // Create an SRSensorReader with the ambient light sensor
        println!("--- SRSensorReader ---");
        let reader = SRSensorReader::alloc();
        let reader_ptr = ISRSensorReader::initWithSensor_(&reader, SRSensorAmbientLightSensor);
        if !reader_ptr.is_null() {
            let reader = SRSensorReader(reader_ptr);
            println!("Reader: {}", nsobj_to_string(reader_ptr));

            let sensor = ISRSensorReader::sensor(&reader);
            println!("  sensor: {}", nsstring_to_string(sensor));

            let auth_status = ISRSensorReader::authorizationStatus(&reader);
            println!("  authorizationStatus: {auth_status}");

            let delegate = ISRSensorReader::delegate(&reader);
            println!("  delegate: {:?}", delegate);
        } else {
            println!("Failed to create SRSensorReader.");
        }

        // SRDevice
        println!("\n--- SRDevice ---");
        let device = SRDevice::currentDevice();
        if !device.0.is_null() {
            println!("Current device: {}", nsobj_to_string(device.0));

            let name = ISRDevice::name(&device);
            println!("  name: {}", nsstring_to_string(name));

            let model = ISRDevice::model(&device);
            println!("  model: {}", nsstring_to_string(model));

            let system_name = ISRDevice::systemName(&device);
            println!("  systemName: {}", nsstring_to_string(system_name));

            let system_version = ISRDevice::systemVersion(&device);
            println!("  systemVersion: {}", nsstring_to_string(system_version));

            let product_type = ISRDevice::productType(&device);
            println!("  productType: {}", nsstring_to_string(product_type));
        } else {
            println!("SRDevice::currentDevice() returned null.");
        }

        // SRFetchRequest
        println!("\n--- SRFetchRequest ---");
        let fetch = SRFetchRequest::alloc();
        let fetch_ptr = INSObject::init(&fetch);
        if !fetch_ptr.is_null() {
            let fetch = SRFetchRequest(fetch_ptr);
            println!("FetchRequest: {}", nsobj_to_string(fetch_ptr));

            let from = ISRFetchRequest::from(&fetch);
            println!("  from: {from}");

            let to = ISRFetchRequest::to(&fetch);
            println!("  to: {to}");

            let device = ISRFetchRequest::device(&fetch);
            println!("  device: {}", nsobj_to_string(device.0));
        } else {
            println!("Failed to create SRFetchRequest.");
        }

        // List known sensor constants
        println!("\n--- Sensor Constants ---");
        let sensors: &[(&str, SRSensor)] = &[
            ("AmbientLightSensor", SRSensorAmbientLightSensor),
            ("Accelerometer", SRSensorAccelerometer),
            ("RotationRate", SRSensorRotationRate),
            ("Visits", SRSensorVisits),
            ("PedometerData", SRSensorPedometerData),
            ("DeviceUsageReport", SRSensorDeviceUsageReport),
            ("MessagesUsageReport", SRSensorMessagesUsageReport),
            ("PhoneUsageReport", SRSensorPhoneUsageReport),
            ("OnWristState", SRSensorOnWristState),
            ("KeyboardMetrics", SRSensorKeyboardMetrics),
        ];
        for (name, sensor) in sensors {
            println!("  {name}: {}", nsstring_to_string(*sensor));
        }
    }

    println!("\nDone.");
}
