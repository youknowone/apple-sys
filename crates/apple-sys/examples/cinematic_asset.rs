//! Inspect Cinematic framework types and detection utilities.
//!
//! Creates CNDetection and CNDecision instances and exercises their properties
//! using the generated Cinematic framework bindings.

use apple_sys::Cinematic::*;
use apple_sys::CoreFoundation::{CGPoint, CGRect, CGSize};
use apple_sys::CoreMedia::CMTime;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Cinematic Framework ===\n");

        // CNRenderingSessionAttributes
        let attrs = CNRenderingSessionAttributes::alloc();
        let attrs_ptr = ICNRenderingSessionAttributes::init(&attrs);
        if !attrs_ptr.is_null() {
            let attrs = CNRenderingSessionAttributes(attrs_ptr);
            let version = ICNRenderingSessionAttributes::renderingVersion(&attrs);
            println!("CNRenderingSessionAttributes created.");
            println!("  Rendering version: {version}");
        } else {
            println!(
                "CNRenderingSessionAttributes: failed to init (may require hardware support)."
            );
        }

        // CNDetection - create with zero values to test bindings
        let detection = CNDetection::alloc();
        let time = CMTime {
            value: 0,
            timescale: 600,
            flags: 1,
            epoch: 0,
        };
        let rect = CGRect {
            origin: CGPoint { x: 0.0, y: 0.0 },
            size: CGSize {
                width: 0.5,
                height: 0.5,
            },
        };
        let det_ptr = ICNDetection::initWithTime_detectionType_normalizedRect_focusDisparity_(
            &detection, time, 1, rect, 0.5,
        );
        if !det_ptr.is_null() {
            let detection = CNDetection(det_ptr);
            let det_type = ICNDetection::detectionType(&detection);
            let disparity = ICNDetection::focusDisparity(&detection);
            let det_rect = ICNDetection::normalizedRect(&detection);
            let det_id = ICNDetection::detectionID(&detection);
            println!("\nCNDetection created.");
            println!("  Detection type: {det_type}");
            println!("  Focus disparity: {disparity}");
            println!(
                "  Normalized rect: ({}, {}, {}, {})",
                det_rect.origin.x, det_rect.origin.y, det_rect.size.width, det_rect.size.height,
            );
            println!("  Detection ID: {det_id}");
        } else {
            println!("\nCNDetection: failed to init.");
        }

        // CNDetection class method: isValidDetectionID
        let valid = CNDetection::isValidDetectionID_(0);
        println!("\nCNDetection.isValidDetectionID(0): {}", valid.0);
        let valid = CNDetection::isValidDetectionID_(42);
        println!("CNDetection.isValidDetectionID(42): {}", valid.0);

        // CNDetection class method: accessibilityLabelForDetectionType
        for det_type in 0..=3 {
            let label = CNDetection::accessibilityLabelForDetectionType_(det_type);
            println!(
                "CNDetection.accessibilityLabel(type={det_type}): {}",
                nsstring_to_string(label)
            );
        }
    }

    println!("\nDone.");
}
