//! Exercise IdentityLookup message filter bindings.
//!
//! Creates ILMessageFilterQueryResponse to set/get filter actions,
//! and inspects ILMessageFilterCapabilitiesQueryResponse sub-actions.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::IdentityLookup::*;
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== IdentityLookup Message Filter ===\n");

        // ILMessageFilterExtension
        println!("--- ILMessageFilterExtension ---");
        let ext = ILMessageFilterExtension::alloc();
        let ext_ptr = INSObject::init(&ext);
        if !ext_ptr.is_null() {
            println!("  Instance: {}", nsobj_to_string(ext_ptr));
        } else {
            println!("  (init returned nil)");
        }

        // ILMessageFilterQueryResponse - set and get action/subAction
        println!("\n--- ILMessageFilterQueryResponse ---");
        let resp = ILMessageFilterQueryResponse::alloc();
        let resp_ptr = INSObject::init(&resp);
        if !resp_ptr.is_null() {
            let resp = ILMessageFilterQueryResponse(resp_ptr);

            let action = IILMessageFilterQueryResponse::action(&resp);
            println!("  Default action: {action} (0=None, 1=Allow, 2=Filter, 3=Junk)");

            // Set action to Filter
            IILMessageFilterQueryResponse::setAction_(&resp, 2);
            let new_action = IILMessageFilterQueryResponse::action(&resp);
            println!("  After setAction(Filter): {new_action}");

            // Set sub-action
            let sub = IILMessageFilterQueryResponse::subAction(&resp);
            println!("  Default subAction: {sub}");

            IILMessageFilterQueryResponse::setSubAction_(&resp, 1);
            let new_sub = IILMessageFilterQueryResponse::subAction(&resp);
            println!("  After setSubAction(1): {new_sub}");
        }

        // ILMessageFilterCapabilitiesQueryResponse
        println!("\n--- ILMessageFilterCapabilitiesQueryResponse ---");
        let caps = ILMessageFilterCapabilitiesQueryResponse::alloc();
        let caps_ptr = INSObject::init(&caps);
        if !caps_ptr.is_null() {
            let caps = ILMessageFilterCapabilitiesQueryResponse(caps_ptr);

            let trans = IILMessageFilterCapabilitiesQueryResponse::transactionalSubActions(&caps);
            let promo = IILMessageFilterCapabilitiesQueryResponse::promotionalSubActions(&caps);
            let t_count = if !trans.0.is_null() {
                INSArray::<id>::count(&trans)
            } else {
                0
            };
            let p_count = if !promo.0.is_null() {
                INSArray::<id>::count(&promo)
            } else {
                0
            };
            println!("  Transactional sub-actions: {t_count}");
            println!("  Promotional sub-actions:   {p_count}");
        }

        // ILMessageFilterQueryRequest
        println!("\n--- ILMessageFilterQueryRequest ---");
        let req = ILMessageFilterQueryRequest::alloc();
        let req_ptr = INSObject::init(&req);
        if !req_ptr.is_null() {
            let req = ILMessageFilterQueryRequest(req_ptr);
            let sender = IILMessageFilterQueryRequest::sender(&req);
            let body = IILMessageFilterQueryRequest::messageBody(&req);
            let country = IILMessageFilterQueryRequest::receiverISOCountryCode(&req);
            println!("  sender: {}", nsobj_to_string(sender.0));
            println!("  messageBody: {}", nsobj_to_string(body.0));
            println!("  receiverISOCountryCode: {}", nsobj_to_string(country.0));
        }

        // ILClassificationResponse
        println!("\n--- ILClassificationResponse ---");
        let cls_resp = ILClassificationResponse::alloc();
        let cls_resp_ptr = IILClassificationResponse::initWithClassificationAction_(
            &cls_resp, 1, // ILClassificationActionReportNotJunk
        );
        if !cls_resp_ptr.is_null() {
            let cls_resp = ILClassificationResponse(cls_resp_ptr);
            let action = IILClassificationResponse::action(&cls_resp);
            println!("  Action: {action} (1=ReportNotJunk)");
        }
    }

    println!("\nDone.");
}
