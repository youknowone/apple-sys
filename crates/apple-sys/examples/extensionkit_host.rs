//! Inspect ExtensionKit classes and create extension-related objects.
//!
//! Creates EXHostViewController and EXAppExtensionBrowserViewController,
//! exercises their bindings, and tests NSError creation for the extension domain.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::ExtensionKit::*;
use apple_sys::Foundation::{INSError, NSAutoreleasePool, NSDictionary, NSError};
use apple_sys::objc::id;
use objc2::msg_send;

mod common;
use common::nsobj_to_string;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ExtensionKit Classes ===\n");

        // EXHostViewController
        println!("--- EXHostViewController ---");
        let vc = EXHostViewController::alloc();
        let vc_ptr: id = msg_send![&*vc.0, init];
        if !vc_ptr.is_null() {
            let vc = EXHostViewController(vc_ptr);
            let placeholder = IEXHostViewController::placeholderView(&vc);
            println!("  EXHostViewController created.");
            println!("  Placeholder view: {}", nsobj_to_string(placeholder.0));

            let delegate = IEXHostViewController::delegate(&vc);
            println!(
                "  Delegate: {}",
                if delegate.is_null() { "not set" } else { "set" }
            );
        } else {
            println!("  (cannot create without configuration)");
        }

        // EXAppExtensionBrowserViewController
        println!("\n--- EXAppExtensionBrowserViewController ---");
        let browser = EXAppExtensionBrowserViewController::alloc();
        let browser_ptr: id = msg_send![&*browser.0, init];
        if !browser_ptr.is_null() {
            let _browser = EXAppExtensionBrowserViewController(browser_ptr);
            println!("  EXAppExtensionBrowserViewController created.");
        } else {
            println!("  (cannot create EXAppExtensionBrowserViewController)");
        }

        // Test NSError with ExtensionKit domain
        println!("\n--- Extension Error Domain ---");
        let domain = nsstring(c"EXExtensionKit");
        let err_ptr =
            NSError::errorWithDomain_code_userInfo_(domain, 0, NSDictionary(std::ptr::null_mut()));
        if !err_ptr.is_null() {
            let err = NSError(err_ptr);
            let err_domain = INSError::domain(&err);
            let err_code = INSError::code(&err);
            let localized = INSError::localizedDescription(&err);
            println!("  Test error domain: {}", nsobj_to_string(err_domain.0));
            println!("  Test error code: {err_code}");
            println!("  Description: {}", nsobj_to_string(localized.0));
        }
    }

    println!("\nDone.");
}
