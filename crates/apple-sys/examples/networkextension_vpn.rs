//! Query NEVPNManager shared manager for VPN configuration.
//!
//! Gets the shared NEVPNManager instance and queries connection status
//! and protocol configuration details via generated bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::NetworkExtension::*;
use apple_sys::objc::id;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== NetworkExtension VPN ===\n");

        // NEVPNManager
        println!("--- NEVPNManager ---");
        let manager = NEVPNManager::sharedManager();
        if !manager.0.is_null() {
            let enabled = INEVPNManager::isEnabled(&manager).0;
            println!("  Enabled: {}", enabled);

            let on_demand = INEVPNManager::isOnDemandEnabled(&manager).0;
            println!("  On-demand enabled: {}", on_demand);

            // Query connection status
            let connection = INEVPNManager::connection(&manager);
            if !connection.0.is_null() {
                let status = INEVPNConnection::status(&connection);
                let status_name = match status {
                    0 => "Invalid",
                    1 => "Disconnected",
                    2 => "Connecting",
                    3 => "Connected",
                    4 => "Reasserting",
                    5 => "Disconnecting",
                    _ => "Unknown",
                };
                println!("  Connection status: {} ({})", status_name, status);
            } else {
                println!("  Connection: nil");
            }

            // Protocol configuration
            let proto_config = INEVPNManager::protocolConfiguration(&manager);
            if !proto_config.0.is_null() {
                let server = INEVPNProtocol::serverAddress(&proto_config);
                println!("  Server address: {}", nsstring_to_string(server));

                let disconnect_on_sleep = INEVPNProtocol::disconnectOnSleep(&proto_config).0;
                println!("  Disconnect on sleep: {}", disconnect_on_sleep);
            } else {
                println!("  Protocol configuration: not configured");
            }

            // Localized description
            let loc_desc = INEVPNManager::localizedDescription(&manager);
            if !loc_desc.0.is_null() {
                println!("  Localized description: {}", nsstring_to_string(loc_desc));
            }

            // On-demand rules
            let rules = INEVPNManager::onDemandRules(&manager);
            if !rules.0.is_null() {
                let count = INSArray::<id>::count(&rules);
                println!("  On-demand rules: {}", count);
            } else {
                println!("  On-demand rules: none");
            }
        }

        // NEFilterManager
        println!("\n--- NEFilterManager ---");
        let filter_mgr = NEFilterManager::sharedManager();
        if !filter_mgr.0.is_null() {
            let enabled = INEFilterManager::isEnabled(&filter_mgr).0;
            println!("  Content filter enabled: {}", enabled);
        }

        // NEDNSSettingsManager
        println!("\n--- NEDNSSettingsManager ---");
        let dns_mgr = NEDNSSettingsManager::sharedManager();
        if !dns_mgr.0.is_null() {
            let enabled = INEDNSSettingsManager::isEnabled(&dns_mgr).0;
            println!("  DNS settings enabled: {}", enabled);
        }
    }

    println!("\nDone.");
}
