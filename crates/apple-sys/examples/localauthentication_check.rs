//! Check biometric authentication capabilities via LocalAuthentication.
//!
//! Uses LAContext to query whether Touch ID, Face ID, or Optic ID
//! is available on this system.

use apple_sys::LocalAuthentication::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== LocalAuthentication ===\n");

        let ctx = LAContext::alloc();
        let ctx_ptr = INSObject::init(&ctx);
        assert!(!ctx_ptr.is_null(), "Failed to create LAContext");
        let ctx = LAContext(ctx_ptr);

        // Check biometric type
        let bio_type = ILAContext::biometryType(&ctx);
        let bio_name = match bio_type {
            0 => "None",
            1 => "Touch ID",
            2 => "Face ID",
            3 => "Optic ID",
            _ => "Unknown",
        };
        println!("Biometry type: {} ({})", bio_name, bio_type);

        // Check if biometric auth is available
        let mut error: *mut NSError = std::ptr::null_mut();
        // LAPolicyDeviceOwnerAuthenticationWithBiometrics = 1
        let available = ILAContext::canEvaluatePolicy_error_(&ctx, 1, error);
        println!("Biometric auth available: {}", available.0);
        if !error.is_null() {
            let err = NSError(error as id);
            let desc = INSError::localizedDescription(&err);
            println!("  Reason: {}", nsstring_to_string(desc));
            let code = INSError::code(&err);
            println!("  Error code: {}", code);
        }

        // Check device owner auth (passcode fallback)
        error = std::ptr::null_mut();
        // LAPolicyDeviceOwnerAuthentication = 2
        let available = ILAContext::canEvaluatePolicy_error_(&ctx, 2, error);
        println!("Device owner auth available: {}", available.0);
        if !error.is_null() {
            let err = NSError(error as id);
            let desc = INSError::localizedDescription(&err);
            println!("  Reason: {}", nsstring_to_string(desc));
        }

        // Query interaction not allowed setting
        let not_interactive = ILAContext::interactionNotAllowed(&ctx);
        println!("Interaction not allowed: {}", not_interactive.0);
    }

    println!("\nDone.");
}
