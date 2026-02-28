//! Create and inspect FSKit volume management types.
//!
//! Creates FSEntityIdentifier, FSFileName, FSItemAttributes, and FSContainerStatus
//! instances using the generated bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::FSKit::*;
use apple_sys::Foundation::{INSData, NSAutoreleasePool, NSUUID};

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== FSKit Volume Types ===\n");

        // FSEntityIdentifier with UUID
        println!("--- FSEntityIdentifier ---");
        let uuid = NSUUID::alloc();
        let uuid = NSUUID(INSObject::init(&uuid));
        let entity = FSEntityIdentifier::alloc();
        let entity_ptr = IFSEntityIdentifier::initWithUUID_(&entity, uuid);
        if !entity_ptr.is_null() {
            let entity = FSEntityIdentifier(entity_ptr);
            let entity_uuid = IFSEntityIdentifier::uuid(&entity);
            println!("  UUID: {}", nsobj_to_string(entity_uuid.0));
            let qualifier = IFSEntityIdentifier::qualifier(&entity);
            println!("  Qualifier data length: {}", INSData::length(&qualifier));
        } else {
            println!("  (init returned nil)");
        }

        // FSFileName
        println!("\n--- FSFileName ---");
        let name = FSFileName::alloc();
        let name_ptr = IFSFileName::initWithCString_(&name, c"test_volume.dat".as_ptr());
        if !name_ptr.is_null() {
            let name = FSFileName(name_ptr);
            let s = IFSFileName::string(&name);
            println!("  String: {}", nsstring_to_string(s));
            let data = IFSFileName::data(&name);
            println!("  Data length: {}", INSData::length(&data));
        }

        // FSFileName from string
        let name2 = FSFileName::nameWithCString_(c"another_file.img".as_ptr());
        if !name2.is_null() {
            let name2 = FSFileName(name2);
            let s2 = IFSFileName::string(&name2);
            println!("  Factory string: {}", nsstring_to_string(s2));
        }

        // FSContainerStatus
        println!("\n--- FSContainerStatus ---");
        let active = FSContainerStatus::active();
        if !active.0.is_null() {
            let state = IFSContainerStatus::state(&active);
            println!("  Active status state: {state}");
        }

        let ready = FSContainerStatus::ready();
        if !ready.0.is_null() {
            let state = IFSContainerStatus::state(&ready);
            println!("  Ready status state: {state}");
        }

        // FSItemAttributes
        println!("\n--- FSItemAttributes ---");
        let attrs = FSItemAttributes::alloc();
        let attrs_ptr = INSObject::init(&attrs);
        if !attrs_ptr.is_null() {
            println!("  Created: {}", nsobj_to_string(attrs_ptr));
        }

        // FSClient (shared instance)
        println!("\n--- FSClient ---");
        let client = FSClient::sharedInstance();
        if !client.0.is_null() {
            println!("  Shared instance: {}", nsobj_to_string(client.0));
        } else {
            println!("  (shared instance not available without entitlements)");
        }
    }

    println!("\nDone.");
}
