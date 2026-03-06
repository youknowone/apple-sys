//! Access the BGTaskScheduler shared instance from BackgroundTasks framework.
//!
//! Queries the shared scheduler, creates task requests, and inspects their properties.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

use apple_sys::BackgroundTasks::*;

mod common;
use common::nsobj_to_string;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== BackgroundTasks Scheduler ===\n");

        let sched = BGTaskScheduler::sharedScheduler();
        if sched.0.is_null() {
            println!("Failed to get BGTaskScheduler.sharedScheduler.");
            return;
        }
        println!("BGTaskScheduler.sharedScheduler obtained.");
        println!("Description: {}", nsobj_to_string(sched.0));

        // Create BGAppRefreshTaskRequest
        let ident = nsstring(c"com.example.refresh");
        let refresh_req = BGAppRefreshTaskRequest::alloc();
        let refresh_ptr = IBGAppRefreshTaskRequest::initWithIdentifier_(&refresh_req, ident);
        if !refresh_ptr.is_null() {
            let refresh_req = BGAppRefreshTaskRequest(refresh_ptr);
            let task_id = IBGTaskRequest::identifier(&refresh_req);
            println!("\nBGAppRefreshTaskRequest created.");
            println!("  Identifier: {}", common::nsstring_to_string(task_id));
        }

        // Create BGProcessingTaskRequest
        let ident = nsstring(c"com.example.test");
        let req = BGProcessingTaskRequest::alloc();
        let req_ptr = IBGProcessingTaskRequest::initWithIdentifier_(&req, ident);
        if !req_ptr.is_null() {
            let req = BGProcessingTaskRequest(req_ptr);
            println!("\nBGProcessingTaskRequest created.");

            let task_id = IBGTaskRequest::identifier(&req);
            let requires_network = IBGProcessingTaskRequest::requiresNetworkConnectivity(&req);
            let requires_power = IBGProcessingTaskRequest::requiresExternalPower(&req);
            println!("  Identifier: {}", common::nsstring_to_string(task_id));
            println!("  Requires network: {}", requires_network.0);
            println!("  Requires external power: {}", requires_power.0);
        }

        // Cancel all tasks (cleanup)
        IBGTaskScheduler::cancelAllTaskRequests(&sched);
        println!("\nCancelled all pending task requests (cleanup).");
    }

    println!("\nDone.");
}
