//! Query OSLog store for recent log entries.
//!
//! Creates an OSLogStore for the current process scope
//! and queries recent log entries.

use apple_sys::OSLog::*;

#[link(name = "OSLog", kind = "framework")]
unsafe extern "C" {}

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== OSLog ===\n");

        // Create OSLogStore for current process
        println!("--- OSLogStore ---");
        {
            let mut error = NSError(std::ptr::null_mut());

            // Scope 1 = currentProcessIdentifier
            let store_id = <OSLogStore as IOSLogStore>::storeWithScope_error_(
                1,
                &mut error as *mut NSError as *mut _,
            );
            let store = OSLogStore(store_id);

            if !store.0.is_null() {
                let desc = PNSObject::description(&store);
                println!("  Store: {}", nsstring_to_string(desc));

                // Get entries enumerator with a position
                // Create a position for the last 5 seconds
                let now = NSDate(NSDate::date());
                let five_secs_ago = NSDate(NSDate_NSExtendedDate::dateByAddingTimeInterval_(
                    &now, -5.0f64,
                ));

                let position = IOSLogStore::positionWithDate_(&store, five_secs_ago);
                if !position.0.is_null() {
                    let pos_desc = PNSObject::description(&position);
                    println!("  Position: {}", nsstring_to_string(pos_desc));
                }

                // Enumerate entries
                let mut entry_error = NSError(std::ptr::null_mut());
                let enumerator =
                    IOSLogStore::entriesEnumeratorWithOptions_position_predicate_error_(
                        &store,
                        0,
                        position,
                        NSPredicate(std::ptr::null_mut()),
                        &mut entry_error as *mut NSError as *mut _,
                    );

                if !enumerator.0.is_null() {
                    println!("\n  Recent log entries (last 5 seconds):");
                    let mut count = 0u32;
                    let max_entries = 10;

                    loop {
                        let entry_id = INSEnumerator::<OSLogEntry>::nextObject(&enumerator);
                        if entry_id.is_null() || count >= max_entries {
                            break;
                        }

                        let entry = OSLogEntry(entry_id);
                        let entry_log = OSLogEntryLog(entry_id);

                        let composed = IOSLogEntry::composedMessage(&entry);
                        let date = IOSLogEntry::date(&entry);
                        let level: OSLogEntryLogLevel = IOSLogEntryLog::level(&entry_log);
                        let subsystem = POSLogEntryWithPayload::subsystem(&entry_log);
                        let category = POSLogEntryWithPayload::category(&entry_log);

                        let level_name = match level as u8 {
                            0 => "Undefined",
                            1 => "Debug",
                            2 => "Info",
                            3 => "Notice",
                            4 => "Error",
                            5 => "Fault",
                            _ => "Other",
                        };

                        let date_desc = PNSObject::description(&date);
                        let date_str = nsstring_to_string(date_desc);
                        let msg = nsstring_to_string(composed);
                        let sub = nsstring_to_string(subsystem);
                        let cat = nsstring_to_string(category);

                        // Truncate long messages
                        let msg_display = if msg.len() > 80 {
                            format!("{}...", &msg[..80])
                        } else {
                            msg
                        };

                        println!(
                            "    [{}] {} {}/{}: {}",
                            level_name, date_str, sub, cat, msg_display
                        );
                        count += 1;
                    }

                    if count == 0 {
                        println!("    (no entries in the last 5 seconds)");
                    } else {
                        println!("    ... showed {} entries", count);
                    }
                } else if !entry_error.0.is_null() {
                    let err_desc = INSError::localizedDescription(&entry_error);
                    println!("  Enumerator error: {}", nsstring_to_string(err_desc));
                }
            } else if !error.0.is_null() {
                let err_desc = INSError::localizedDescription(&error);
                println!("  Error creating store: {}", nsstring_to_string(err_desc));
            }
        }
    }

    println!("\nDone.");
}
