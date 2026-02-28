//! Query local users via OpenDirectory.
//!
//! Uses ODSession, ODNode, and ODQuery to enumerate user records
//! from the local directory service via generated bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{
    INSArray, INSError, NSArray, NSArray_NSArrayCreation, NSAutoreleasePool, NSError,
};
use apple_sys::OpenDirectory::*;
use apple_sys::objc::{BOOL, id};
mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== OpenDirectory Users ===\n");

        // Get default session
        let session = ODSession::defaultSession();
        assert!(!session.0.is_null(), "Failed to get default ODSession");
        println!("ODSession: default session acquired");

        // Connect to local node
        let mut error = NSError(std::ptr::null_mut());
        let node_name = nsstring(c"/Local/Default");
        let node = ODNode::alloc();
        let node = ODNode(IODNode::initWithSession_name_error_(
            &node,
            session,
            node_name,
            &mut error as *mut NSError as *mut _,
        ));

        if node.0.is_null() {
            if !error.0.is_null() {
                let desc = INSError::localizedDescription(&error);
                eprintln!(
                    "Failed to connect to local node: {}",
                    nsstring_to_string(desc)
                );
            }
            return;
        }

        let node_name_str = IODNode::nodeName(&node);
        println!("Connected to node: {}", nsstring_to_string(node_name_str));

        // Query for user records
        let record_type = nsstring(c"dsRecTypeStandard:Users");
        let attribute = nsstring(c"dsAttrTypeStandard:RecordName");
        let value: id = std::ptr::null_mut();
        let match_type: u32 = 0x2001; // kODMatchAny

        let attr_name = nsstring(c"dsAttrTypeStandard:RecordName");
        let attr_uid = nsstring(c"dsAttrTypeStandard:UniqueID");
        let attr_shell = nsstring(c"dsAttrTypeStandard:UserShell");
        let attrs: [id; 3] = [attr_name.0, attr_uid.0, attr_shell.0];
        let return_attrs = NSArray(
            <NSArray as NSArray_NSArrayCreation<()>>::arrayWithObjects_count_(
                attrs.as_ptr() as *const *mut u64,
                3,
            ),
        );

        error = NSError(std::ptr::null_mut());
        let query = ODQuery::alloc();
        let query = ODQuery(
            IODQuery::initWithNode_forRecordTypes_attribute_matchType_queryValues_returnAttributes_maximumResults_error_(
                &query,
                node,
                record_type.0,
                attribute,
                match_type,
                value,
                return_attrs.0,
                20,
                &mut error as *mut NSError as *mut _,
            ),
        );

        if query.0.is_null() {
            if !error.0.is_null() {
                let desc = INSError::localizedDescription(&error);
                eprintln!("Query creation failed: {}", nsstring_to_string(desc));
            }
            return;
        }

        // Execute synchronous query
        error = NSError(std::ptr::null_mut());
        let results = IODQuery::resultsAllowingPartial_error_(
            &query,
            BOOL(false),
            &mut error as *mut NSError as *mut _,
        );

        if results.0.is_null() {
            println!("No results (or error)");
            return;
        }

        let results_arr = NSArray(results.0);
        let count = INSArray::<id>::count(&results_arr);
        println!("\nLocal users (up to 20): {}\n", count);

        for i in 0..count {
            let record_ptr = INSArray::<id>::objectAtIndex_(&results_arr, i);
            let record = ODRecord(record_ptr);
            let name = IODRecord::recordName(&record);
            let record_type = IODRecord::recordType(&record);
            println!(
                "  {:2}. {} ({})",
                i + 1,
                nsstring_to_string(name),
                nsstring_to_string(record_type)
            );
        }
    }

    println!("\nDone.");
}
