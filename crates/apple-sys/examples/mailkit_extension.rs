//! Exercise MailKit message action and email address bindings.
//!
//! Creates MEEmailAddress, MEMessageAction factory instances, and
//! MEAddressAnnotation objects to verify the generated MailKit bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

use apple_sys::MailKit::*;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MailKit ===\n");

        // MEEmailAddress
        println!("--- MEEmailAddress ---");
        let addr = MEEmailAddress::alloc();
        let raw = nsstring(c"User Name <user@example.com>");
        let addr = MEEmailAddress(IMEEmailAddress::initWithRawString_(&addr, raw));
        if !addr.0.is_null() {
            let raw_str = IMEEmailAddress::rawString(&addr);
            let addr_str = IMEEmailAddress::addressString(&addr);
            println!("  Raw string: {}", nsstring_to_string(raw_str));
            println!("  Address:    {}", nsstring_to_string(addr_str));
        }

        // MEMessageAction factory methods
        println!("\n--- MEMessageAction ---");
        let trash = MEMessageAction::moveToTrashAction();
        if !trash.0.is_null() {
            println!("  moveToTrashAction: created");
        }
        let archive = MEMessageAction::moveToArchiveAction();
        if !archive.0.is_null() {
            println!("  moveToArchiveAction: created");
        }
        let junk = MEMessageAction::moveToJunkAction();
        if !junk.0.is_null() {
            println!("  moveToJunkAction: created");
        }
        let read = MEMessageAction::markAsReadAction();
        if !read.0.is_null() {
            println!("  markAsReadAction: created");
        }
        let unread = MEMessageAction::markAsUnreadAction();
        if !unread.0.is_null() {
            println!("  markAsUnreadAction: created");
        }

        // MEMessageAction with flag
        let flagged = MEMessageAction::flagActionWithFlag_(0); // MEMessageActionFlagNone
        if !flagged.is_null() {
            println!("  flagActionWithFlag(0): created");
        }

        // MEMessageActionDecision
        println!("\n--- MEMessageActionDecision ---");
        if !trash.0.is_null() {
            let decision = MEMessageActionDecision::decisionApplyingAction_(trash);
            if !decision.is_null() {
                println!("  Decision applying trash action: created");
            }
        }

        let invoke_body = MEMessageActionDecision::invokeAgainWithBody();
        if !invoke_body.0.is_null() {
            println!("  invokeAgainWithBody: created");
        }

        // MEAddressAnnotation
        println!("\n--- MEAddressAnnotation ---");
        let success_msg = nsstring(c"Verified sender");
        let success = MEAddressAnnotation::successWithLocalizedDescription_(success_msg);
        if !success.0.is_null() {
            println!("  Success annotation: created");
        }

        let warn_msg = nsstring(c"Suspicious sender");
        let warning = MEAddressAnnotation::warningWithLocalizedDescription_(warn_msg);
        if !warning.0.is_null() {
            println!("  Warning annotation: created");
        }

        let err_msg = nsstring(c"Invalid sender");
        let error = MEAddressAnnotation::errorWithLocalizedDescription_(err_msg);
        if !error.0.is_null() {
            println!("  Error annotation: created");
        }
    }

    println!("\nDone.");
}
