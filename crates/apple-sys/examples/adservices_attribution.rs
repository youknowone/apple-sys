//! Query ad attribution token using AdServices framework.
//!
//! Attempts to retrieve an attribution token via AAAttribution.

use apple_sys::AdServices::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSError, INSString, NSAutoreleasePool, NSError, NSString};
mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AdServices Attribution ===\n");

        // Try to get an attribution token
        let error = NSError(std::ptr::null_mut());
        let token =
            AAAttribution::attributionTokenWithError_(&error as *const NSError as *mut NSError);

        if !token.0.is_null() {
            let token_str = nsstring_to_string(token);
            // Token can be long, show a truncated version
            if token_str.len() > 80 {
                println!("Attribution token: {}...", &token_str[..80]);
            } else {
                println!("Attribution token: {token_str}");
            }
            let len = INSString::length(&NSString(token.0));
            println!("Token length: {len} characters");
        } else if !error.0.is_null() {
            let desc = INSError::localizedDescription(&error);
            let code = INSError::code(&error);
            println!(
                "Failed to get attribution token (code {code}): {}",
                nsstring_to_string(desc)
            );
        } else {
            println!("No token and no error returned.");
        }
    }

    println!("\nDone.");
}
