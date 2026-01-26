//! Query the default exception handler using ExceptionHandling framework.
//!
//! Gets NSExceptionHandler.defaultExceptionHandler and inspects its
//! exception handling mask and related settings.

use apple_sys::ExceptionHandling::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ExceptionHandling Framework ===\n");

        // NSExceptionHandler
        println!("--- NSExceptionHandler ---");
        let handler = NSExceptionHandler::defaultExceptionHandler();
        if !handler.0.is_null() {
            println!("  Default handler: {}", nsobj_to_string(handler.0));

            let mask = INSExceptionHandler::exceptionHandlingMask(&handler);
            println!("  Exception handling mask: 0x{mask:x} ({mask})");

            // Decode mask bits
            let flags = [
                (1, "LogUncaughtException"),
                (2, "HandleUncaughtException"),
                (4, "LogUncaughtSystemException"),
                (8, "HandleUncaughtSystemException"),
                (16, "LogUncaughtRuntimeError"),
                (32, "HandleUncaughtRuntimeError"),
                (64, "LogTopLevelException"),
                (128, "HandleTopLevelException"),
                (256, "LogOtherException"),
                (512, "HandleOtherException"),
            ];

            let mut active_flags = Vec::new();
            for (bit, name) in &flags {
                if mask & bit != 0 {
                    active_flags.push(*name);
                }
            }

            if active_flags.is_empty() {
                println!("  Active flags: (none)");
            } else {
                println!("  Active flags:");
                for flag in &active_flags {
                    println!("    - {flag}");
                }
            }

            // Check delegate
            let delegate = INSExceptionHandler::delegate(&handler);
            println!("  Delegate: {}", nsobj_to_string(delegate));

            // Set and read back a mask
            let new_mask: u64 = 1 | 4 | 16; // Log all uncaught
            INSExceptionHandler::setExceptionHandlingMask_(&handler, new_mask);
            let read_mask = INSExceptionHandler::exceptionHandlingMask(&handler);
            println!("\n  After setting mask to 0x{new_mask:x}:");
            println!("  Read back mask: 0x{read_mask:x}");

            // Restore original
            INSExceptionHandler::setExceptionHandlingMask_(&handler, mask);
            println!("  Restored original mask: 0x{mask:x}");
        } else {
            println!("  defaultExceptionHandler returned nil");
        }
    }

    println!("\nDone.");
}
