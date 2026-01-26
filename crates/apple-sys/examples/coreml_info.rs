//! Inspect CoreML model capabilities.
//!
//! Uses MLModel to check available compute devices
//! and MLMultiArray creation.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::CoreML::*;
use apple_sys::Foundation::*;
use apple_sys::objc::id;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreML ===\n");

        // Create a MLMultiArray
        // Shape: [3, 4]
        let n3 = NSNumber::numberWithInteger_(3);
        let n4 = NSNumber::numberWithInteger_(4);
        let shape = NSArray(
            <NSArray as NSArray_NSArrayCreation<id>>::arrayWithObjects_count_(
                [n3.0, n4.0].as_ptr() as *const *mut u64,
                2,
            ),
        );

        let error = NSError(std::ptr::null_mut());
        // MLMultiArrayDataTypeFloat32 = 0x10000 | 32
        let data_type: MLMultiArrayDataType = 0x10000 | 32;
        let multi = MLMultiArray::alloc();
        let multi_ptr = multi.initWithShape_dataType_error_(
            shape,
            data_type,
            &error as *const NSError as *mut NSError,
        );

        if !multi_ptr.is_null() {
            let multi = MLMultiArray(multi_ptr);
            let count = IMLMultiArray::count(&multi);
            let shape_out = IMLMultiArray::shape(&multi);
            let _strides = IMLMultiArray::strides(&multi);
            let dt = IMLMultiArray::dataType(&multi);

            println!("\nMLMultiArray [3,4] Float32:");
            println!("  Count:    {}", count);
            println!("  DataType: 0x{:X}", dt);

            let shape_count = INSArray::<id>::count(&shape_out);
            print!("  Shape:    [");
            for i in 0..shape_count {
                let n = NSNumber(INSArray::<id>::objectAtIndex_(&shape_out, i));
                let v = INSNumber::integerValue(&n);
                if i > 0 {
                    print!(", ");
                }
                print!("{}", v);
            }
            println!("]");

            let strides = IMLMultiArray::strides(&multi);
            let strides_count = INSArray::<id>::count(&strides);
            print!("  Strides:  [");
            for i in 0..strides_count {
                let n = NSNumber(INSArray::<id>::objectAtIndex_(&strides, i));
                let v = INSNumber::integerValue(&n);
                if i > 0 {
                    print!(", ");
                }
                print!("{}", v);
            }
            println!("]");
        } else if !error.0.is_null() {
            let desc = INSError::localizedDescription(&error);
            println!(
                "\nError creating MLMultiArray: {}",
                nsstring_to_string(desc)
            );
        }

        // MLModelConfiguration
        let config = MLModelConfiguration::alloc();
        let config_ptr = INSObject::init(&config);
        if !config_ptr.is_null() {
            let config = MLModelConfiguration(config_ptr);
            let compute = IMLModelConfiguration::computeUnits(&config);
            let compute_name = match compute {
                0 => "CPU Only",
                1 => "CPU and GPU",
                2 => "All (CPU+GPU+ANE)",
                _ => "Unknown",
            };
            println!("\nDefault compute units: {} ({})", compute_name, compute);
        }
    }

    println!("\nDone.");
}
