//! Verify SecurityUI bindings by exercising SFCertificatePresentation
//! creation and property access.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::SecurityUI::*;

mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SecurityUI Certificate Presentation ===\n");

        // SFCertificatePresentation - create with default init
        println!("--- SFCertificatePresentation ---");
        let pres = SFCertificatePresentation::alloc();
        let pres_ptr = ISFCertificatePresentation::init(&pres);
        if !pres_ptr.is_null() {
            let pres = SFCertificatePresentation(pres_ptr);
            println!("Instance: {}", nsobj_to_string(pres_ptr));

            let title = ISFCertificatePresentation::title(&pres);
            println!("  title: {}", nsstring_to_string(title));

            let message = ISFCertificatePresentation::message(&pres);
            println!("  message: {}", nsstring_to_string(message));

            let trust = ISFCertificatePresentation::trust(&pres);
            println!("  trust: {:?}", trust);

            let help_url = ISFCertificatePresentation::helpURL(&pres);
            println!("  helpURL: {}", nsobj_to_string(help_url.0));

            // Set a title and verify
            let test_title = nsstring(c"Test Certificate");
            ISFCertificatePresentation::setTitle_(&pres, test_title);
            let read_back = ISFCertificatePresentation::title(&pres);
            println!("  title (after set): {}", nsstring_to_string(read_back));

            // Set a message and verify
            let test_msg = nsstring(c"This is a test message");
            ISFCertificatePresentation::setMessage_(&pres, test_msg);
            let read_back = ISFCertificatePresentation::message(&pres);
            println!("  message (after set): {}", nsstring_to_string(read_back));
        } else {
            println!("SFCertificatePresentation: requires initialization parameters");
        }
    }

    println!("\nDone.");
}
