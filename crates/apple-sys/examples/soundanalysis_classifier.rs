//! Verify SoundAnalysis bindings by creating an SNClassifySoundRequest
//! and querying its properties.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, INSError, NSAutoreleasePool, NSError};
use apple_sys::SoundAnalysis::*;
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SoundAnalysis Classifier ===\n");

        // SNClassifySoundRequest with built-in classifier
        println!("--- SNClassifySoundRequest ---");
        let classifier_id = nsstring(c"SNClassifierIdentifierVersion1");
        let request = SNClassifySoundRequest::alloc();
        let error: *mut NSError = std::ptr::null_mut();
        let request_ptr = ISNClassifySoundRequest::initWithClassifierIdentifier_error_(
            &request,
            classifier_id,
            error,
        );

        if !request_ptr.is_null() {
            let request = SNClassifySoundRequest(request_ptr);
            println!("Request (Version1): {}", nsobj_to_string(request_ptr));

            let overlap_factor = ISNClassifySoundRequest::overlapFactor(&request);
            println!("  overlapFactor: {overlap_factor}");

            let window_dur = ISNClassifySoundRequest::windowDuration(&request);
            let dur_value = window_dur.value;
            let dur_timescale = window_dur.timescale;
            println!("  windowDuration: value={dur_value}, timescale={dur_timescale}");

            let window_constraint = ISNClassifySoundRequest::windowDurationConstraint(&request);
            if !window_constraint.0.is_null() {
                let constraint_type = ISNTimeDurationConstraint::type_(&window_constraint);
                println!("  windowDurationConstraint type: {constraint_type}");
            }

            let known = ISNClassifySoundRequest::knownClassifications(&request);
            if !known.0.is_null() {
                let count = INSArray::<id>::count(&known);
                println!("  knownClassifications count: {count}");
                for i in 0..count.min(10) {
                    let item = INSArray::<id>::objectAtIndex_(&known, i);
                    println!("    [{i}] {}", nsobj_to_string(item));
                }
                if count > 10 {
                    println!("    ... and {} more", count - 10);
                }
            }

            // Adjust overlap factor
            ISNClassifySoundRequest::setOverlapFactor_(&request, 0.75);
            let new_overlap = ISNClassifySoundRequest::overlapFactor(&request);
            println!("  overlapFactor (after set to 0.75): {new_overlap}");
        } else if !error.is_null() {
            let desc = INSError::localizedDescription(&NSError(error as _));
            println!("Init error: {}", nsobj_to_string(desc.0));
        }

        // SNClassification
        println!("\n--- SNClassification ---");
        let classification = SNClassification::alloc();
        let classification_ptr = INSObject::init(&classification);
        if !classification_ptr.is_null() {
            let _classification = SNClassification(classification_ptr);
            println!("Classification: {}", nsobj_to_string(classification_ptr));
        } else {
            println!("SNClassification: requires initialization parameters.");
        }

        // SNClassificationResult
        println!("\n--- SNClassificationResult ---");
        let result = SNClassificationResult::alloc();
        let result_ptr = INSObject::init(&result);
        if !result_ptr.is_null() {
            let _result = SNClassificationResult(result_ptr);
            println!("ClassificationResult: {}", nsobj_to_string(result_ptr));
        } else {
            println!("SNClassificationResult: requires initialization parameters.");
        }
    }

    println!("\nDone.");
}
