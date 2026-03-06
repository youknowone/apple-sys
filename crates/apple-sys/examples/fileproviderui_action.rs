//! Inspect FileProviderUI action extension classes.
//!
//! Creates FPUIActionExtensionContext and FPUIActionExtensionViewController,
//! and exercises their generated bindings including the FPUIErrorDomain constant.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::FileProviderUI::*;
use apple_sys::Foundation::{INSError, NSAutoreleasePool, NSDictionary, NSError};

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== FileProviderUI Classes ===\n");

        // FPUIActionExtensionContext
        println!("--- FPUIActionExtensionContext ---");
        let ctx = FPUIActionExtensionContext::alloc();
        let ctx_ptr = INSObject::init(&ctx);
        if !ctx_ptr.is_null() {
            let ctx = FPUIActionExtensionContext(ctx_ptr);
            println!("  FPUIActionExtensionContext created.");

            let domain_id = IFPUIActionExtensionContext::domainIdentifier(&ctx);
            println!("  Domain identifier: {}", nsstring_to_string(domain_id));
        } else {
            println!("  (failed to create FPUIActionExtensionContext)");
        }

        // FPUIActionExtensionViewController
        println!("\n--- FPUIActionExtensionViewController ---");
        let vc = FPUIActionExtensionViewController::alloc();
        let vc_ptr = INSObject::init(&vc);
        if !vc_ptr.is_null() {
            let vc = FPUIActionExtensionViewController(vc_ptr);
            println!("  FPUIActionExtensionViewController created.");

            let ext_ctx = IFPUIActionExtensionViewController::extensionContext(&vc);
            println!("  Extension context: {}", nsobj_to_string(ext_ctx.0));
        } else {
            println!("  (failed to create FPUIActionExtensionViewController)");
        }

        // FPUIErrorDomain constant
        println!("\n--- FPUIErrorDomain ---");
        println!("  Error domain: {}", nsstring_to_string(FPUIErrorDomain));

        // Create an NSError with the FPUIErrorDomain
        let err_ptr = NSError::errorWithDomain_code_userInfo_(
            FPUIErrorDomain,
            0,
            NSDictionary(std::ptr::null_mut()),
        );
        if !err_ptr.is_null() {
            let err = NSError(err_ptr);
            let code = INSError::code(&err);
            let desc = INSError::localizedDescription(&err);
            println!("  Test error code: {code}");
            println!("  Test error description: {}", nsobj_to_string(desc.0));
        }
    }

    println!("\nDone.");
}
