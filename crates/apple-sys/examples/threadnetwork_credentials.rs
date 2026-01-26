//! Verify ThreadNetwork bindings by creating a THClient and
//! inspecting THCredentials properties.

use apple_sys::ThreadNetwork::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ThreadNetwork Credentials ===\n");

        // THClient
        println!("--- THClient ---");
        let client = THClient::alloc();
        let client_ptr = INSObject::init(&client);
        if !client_ptr.is_null() {
            println!("Client: {}", nsobj_to_string(client_ptr));
        } else {
            println!("Failed to create THClient instance.");
        }

        // THCredentials
        println!("\n--- THCredentials ---");
        let cred = THCredentials::alloc();
        let cred_ptr = INSObject::init(&cred);
        if !cred_ptr.is_null() {
            let cred = THCredentials(cred_ptr);
            println!("Credentials: {}", nsobj_to_string(cred_ptr));

            let network_name = ITHCredentials::networkName(&cred);
            println!("  networkName: {}", nsstring_to_string(network_name));

            let extended_pan = ITHCredentials::extendedPANID(&cred);
            println!("  extendedPANID: {}", nsobj_to_string(extended_pan.0));

            let border_agent = ITHCredentials::borderAgentID(&cred);
            println!("  borderAgentID: {}", nsobj_to_string(border_agent.0));

            let active_dataset = ITHCredentials::activeOperationalDataSet(&cred);
            println!(
                "  activeOperationalDataSet: {}",
                nsobj_to_string(active_dataset.0)
            );

            let network_key = ITHCredentials::networkKey(&cred);
            println!("  networkKey: {}", nsobj_to_string(network_key.0));

            let pskc = ITHCredentials::PSKC(&cred);
            println!("  PSKC: {}", nsobj_to_string(pskc.0));

            let channel = ITHCredentials::channel(&cred);
            println!("  channel: {channel}");

            let pan_id = ITHCredentials::panID(&cred);
            println!("  panID: {}", nsobj_to_string(pan_id.0));

            let creation_date = ITHCredentials::creationDate(&cred);
            println!("  creationDate: {}", nsobj_to_string(creation_date.0));

            let last_modified = ITHCredentials::lastModificationDate(&cred);
            println!(
                "  lastModificationDate: {}",
                nsobj_to_string(last_modified.0)
            );
        } else {
            println!("THCredentials: requires initialization parameters.");
        }
    }

    println!("\nDone.");
}
