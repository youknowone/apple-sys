//! Check developer tool authorization using ExecutionPolicy framework.
//!
//! Creates an EPDeveloperTool and queries its authorizationStatus.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::ExecutionPolicy::*;
use apple_sys::Foundation::NSAutoreleasePool;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ExecutionPolicy Developer Tool ===\n");

        // EPDeveloperTool
        println!("--- EPDeveloperTool ---");
        let tool = EPDeveloperTool::alloc();
        let tool_ptr = INSObject::init(&tool);
        if !tool_ptr.is_null() {
            let tool = EPDeveloperTool(tool_ptr);
            println!("  Instance: {}", nsobj_to_string(tool.0));

            let auth_status = IEPDeveloperTool::authorizationStatus(&tool);
            let status_str = match auth_status {
                0 => "NotDetermined",
                1 => "Restricted",
                2 => "Denied",
                3 => "Authorized",
                _ => "Unknown",
            };
            println!("  Authorization status: {status_str} ({auth_status})");
        } else {
            println!("  Failed to create EPDeveloperTool instance");
        }

        // EPExecutionPolicy
        println!("\n--- EPExecutionPolicy ---");
        let policy = EPExecutionPolicy::alloc();
        let policy_ptr = INSObject::init(&policy);
        if !policy_ptr.is_null() {
            println!("  Instance: {}", nsobj_to_string(policy_ptr));
        } else {
            println!("  Failed to create EPExecutionPolicy instance");
        }
    }

    println!("\nDone.");
}
