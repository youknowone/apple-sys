//! Check MetricKit diagnostic/metric manager.
//!
//! Uses MXMetricManager to inspect available metric
//! payload types and diagnostic categories.

use apple_sys::MetricKit::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MetricKit ===\n");

        // MXMetricManager
        let mgr = MXMetricManager::sharedManager();
        if !mgr.0.is_null() {
            println!("MXMetricManager: available");

            // Past payloads
            let payloads = IMXMetricManager::pastPayloads(&mgr);
            let p_count = INSArray::<id>::count(&payloads);
            println!("  Past metric payloads: {}", p_count);

            // Past diagnostic payloads
            let diag = IMXMetricManager::pastDiagnosticPayloads(&mgr);
            let d_count = INSArray::<id>::count(&diag);
            println!("  Past diagnostic payloads: {}", d_count);

            // Show first payload info if available
            if p_count > 0 {
                let payload = MXMetricPayload(INSArray::<id>::objectAtIndex_(&payloads, 0));
                let meta = IMXMetricPayload::metaData(&payload);
                if !meta.0.is_null() {
                    let region = IMXMetaData::regionFormat(&meta);
                    let os_ver = IMXMetaData::osVersion(&meta);
                    let device = IMXMetaData::deviceType(&meta);
                    println!("\n  Latest payload metadata:");
                    println!("    Region: {}", nsstring_to_string(region));
                    println!("    OS:     {}", nsstring_to_string(os_ver));
                    println!("    Device: {}", nsstring_to_string(device));
                }
            }
        } else {
            println!("MXMetricManager: not available");
        }
    }

    println!("\nDone.");
}
