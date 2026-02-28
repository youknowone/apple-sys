//! Verify VideoSubscriberAccount bindings by creating a VSAccountManager,
//! VSAccountMetadataRequest, and inspecting VSSubscriptionRegistrationCenter.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::VideoSubscriberAccount::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== VideoSubscriberAccount Manager ===\n");

        // VSAccountManager
        println!("--- VSAccountManager ---");
        let manager = VSAccountManager::alloc();
        let manager_ptr = INSObject::init(&manager);
        if !manager_ptr.is_null() {
            let manager = VSAccountManager(manager_ptr);
            println!("Manager: {}", nsobj_to_string(manager_ptr));

            let delegate = IVSAccountManager::delegate(&manager);
            println!("  delegate: {:?}", delegate);
        } else {
            println!("Failed to create VSAccountManager.");
        }

        // VSAccountMetadataRequest
        println!("\n--- VSAccountMetadataRequest ---");
        let request = VSAccountMetadataRequest::alloc();
        let request_ptr = INSObject::init(&request);
        if !request_ptr.is_null() {
            let request = VSAccountMetadataRequest(request_ptr);
            println!("Request: {}", nsobj_to_string(request_ptr));

            let interruption = IVSAccountMetadataRequest::isInterruptionAllowed(&request);
            println!("  isInterruptionAllowed: {}", interruption.0);

            let force_auth = IVSAccountMetadataRequest::forceAuthentication(&request);
            println!("  forceAuthentication: {}", force_auth.0);

            let include_provider =
                IVSAccountMetadataRequest::includeAccountProviderIdentifier(&request);
            println!("  includeAccountProviderIdentifier: {}", include_provider.0);

            let include_auth_exp =
                IVSAccountMetadataRequest::includeAuthenticationExpirationDate(&request);
            println!(
                "  includeAuthenticationExpirationDate: {}",
                include_auth_exp.0
            );

            let channel_id = IVSAccountMetadataRequest::channelIdentifier(&request);
            println!("  channelIdentifier: {}", nsstring_to_string(channel_id));

            let supported_auth_schemes =
                IVSAccountMetadataRequest::supportedAuthenticationSchemes(&request);
            println!(
                "  supportedAuthenticationSchemes: {}",
                nsobj_to_string(supported_auth_schemes.0)
            );

            let verification_token = IVSAccountMetadataRequest::verificationToken(&request);
            println!(
                "  verificationToken: {}",
                nsstring_to_string(verification_token)
            );

            let featured_provider =
                IVSAccountMetadataRequest::featuredAccountProviderIdentifiers(&request);
            println!(
                "  featuredAccountProviderIdentifiers: {}",
                nsobj_to_string(featured_provider.0)
            );
        } else {
            println!("Failed to create VSAccountMetadataRequest.");
        }

        // VSSubscriptionRegistrationCenter
        println!("\n--- VSSubscriptionRegistrationCenter ---");
        let center = VSSubscriptionRegistrationCenter::defaultSubscriptionRegistrationCenter();
        if !center.0.is_null() {
            println!("Default center: {}", nsobj_to_string(center.0));
        } else {
            println!("Default center: (null)");
        }

        // VSSubscription
        println!("\n--- VSSubscription ---");
        let sub = VSSubscription::alloc();
        let sub_ptr = INSObject::init(&sub);
        if !sub_ptr.is_null() {
            let sub = VSSubscription(sub_ptr);
            println!("Subscription: {}", nsobj_to_string(sub_ptr));

            let expiration = IVSSubscription::expirationDate(&sub);
            println!("  expirationDate: {}", nsobj_to_string(expiration.0));

            let access_level = IVSSubscription::accessLevel(&sub);
            println!("  accessLevel: {access_level}");

            let billing_id = IVSSubscription::billingIdentifier(&sub);
            println!("  billingIdentifier: {}", nsstring_to_string(billing_id));
        } else {
            println!("Failed to create VSSubscription.");
        }
    }

    println!("\nDone.");
}
