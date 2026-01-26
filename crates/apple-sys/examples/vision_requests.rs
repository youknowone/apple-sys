//! Create Vision framework analysis requests.
//!
//! Demonstrates VNDetectFaceRectanglesRequest, VNRecognizeTextRequest,
//! and VNDetectBarcodesRequest creation and configuration.

use apple_sys::Vision::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Vision Framework Requests ===\n");

        // VNDetectFaceRectanglesRequest
        let face_req = VNDetectFaceRectanglesRequest::alloc();
        let face_req_ptr = INSObject::init(&face_req);
        let face_req = VNDetectFaceRectanglesRequest(face_req_ptr);
        let revision = face_req.revision();
        println!("Face detection request:");
        println!("  Revision: {}", revision);

        // Supported revisions
        let revisions = VNDetectFaceRectanglesRequest::supportedRevisions();
        let rev_count = INSIndexSet::count(&revisions);
        print!("  Supported revisions: ");
        for i in 0..rev_count {
            let idx = INSIndexSet::containsIndex_(&revisions, i + 1);
            if idx.0 {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", i + 1);
            }
        }
        println!();

        // VNRecognizeTextRequest
        let text_req = VNRecognizeTextRequest::alloc();
        let text_req_ptr = INSObject::init(&text_req);
        let text_req = VNRecognizeTextRequest(text_req_ptr);
        let revision = text_req.revision();
        println!("\nText recognition request:");
        println!("  Revision: {}", revision);

        // Get supported languages
        let error: *mut NSError = std::ptr::null_mut();
        let langs = text_req.supportedRecognitionLanguagesAndReturnError_(error);
        if !langs.0.is_null() {
            let lang_count = INSArray::<id>::count(&langs);
            print!("  Languages ({}):", lang_count);
            for i in 0..lang_count.min(10) {
                let lang = INSArray::<id>::objectAtIndex_(&langs, i);
                print!(" {}", nsstring_to_string(NSString(lang)));
            }
            if lang_count > 10 {
                print!(" ... +{}", lang_count - 10);
            }
            println!();
        }

        // VNDetectBarcodesRequest
        let barcode_req = VNDetectBarcodesRequest::alloc();
        let barcode_req_ptr = INSObject::init(&barcode_req);
        let barcode_req = VNDetectBarcodesRequest(barcode_req_ptr);
        let revision = barcode_req.revision();
        println!("\nBarcode detection request:");
        println!("  Revision: {}", revision);

        let symbologies = barcode_req.symbologies();
        if !symbologies.0.is_null() {
            let count = INSArray::<id>::count(&symbologies);
            print!("  Symbologies ({}):", count);
            for i in 0..count.min(8) {
                let sym = INSArray::<id>::objectAtIndex_(&symbologies, i);
                print!(" {}", nsstring_to_string(NSString(sym)));
            }
            if count > 8 {
                print!(" ...");
            }
            println!();
        }

        // VNDetectHumanBodyPoseRequest
        let body_req = VNDetectHumanBodyPoseRequest::alloc();
        let body_req_ptr = INSObject::init(&body_req);
        let body_req = VNDetectHumanBodyPoseRequest(body_req_ptr);
        let revision = body_req.revision();
        println!("\nHuman body pose request:");
        println!("  Revision: {}", revision);
    }

    println!("\nDone.");
}
