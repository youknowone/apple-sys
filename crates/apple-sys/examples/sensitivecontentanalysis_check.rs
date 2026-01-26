//! Check SCSensitivityAnalyzer using SensitiveContentAnalysis.
//!
//! Creates an SCSensitivityAnalyzer instance and queries its
//! analysis policy and capabilities.

use apple_sys::SensitiveContentAnalysis::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SensitiveContentAnalysis ===\n");

        println!("SCSensitivityAnalyzer class: available");

        let analyzer = SCSensitivityAnalyzer::alloc();
        let analyzer_ptr = INSObject::init(&analyzer);

        if !analyzer_ptr.is_null() {
            let analyzer = SCSensitivityAnalyzer(analyzer_ptr);
            println!("Analyzer: {}", nsobj_to_string(analyzer_ptr));

            let policy = ISCSensitivityAnalyzer::analysisPolicy(&analyzer);
            let policy_name = match policy {
                0 => "Disabled",
                1 => "Simple Interventions",
                2 => "Descriptive Interventions",
                _ => "Unknown",
            };
            println!("Analysis policy: {policy_name} ({policy})");

            let responds_analyze = PNSObject::respondsToSelector_(
                &analyzer,
                objc2::sel!(analyzeImage:completionHandler:),
            );
            println!(
                "Responds to analyzeImage:completionHandler: {}",
                responds_analyze.0
            );

            let responds_video = PNSObject::respondsToSelector_(
                &analyzer,
                objc2::sel!(analyzeVideoFile:completionHandler:),
            );
            println!(
                "Responds to analyzeVideoFile:completionHandler: {}",
                responds_video.0
            );
        } else {
            println!("Failed to create analyzer instance.");
        }

        // SCSensitivityAnalysis is available in the bindings
        println!("\n--- Related Classes ---");
        println!("SCSensitivityAnalysis: available");
    }

    println!("\nDone.");
}
