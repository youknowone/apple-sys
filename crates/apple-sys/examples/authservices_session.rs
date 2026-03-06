//! Inspect AuthenticationServices credential identity store.
//!
//! Accesses ASCredentialIdentityStore sharedStore and checks its state.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;

use apple_sys::AuthenticationServices::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AuthenticationServices ===\n");

        // ASCredentialIdentityStore
        let store = ASCredentialIdentityStore::sharedStore();
        if store.0.is_null() {
            println!("Failed to get shared credential identity store.");
            return;
        }
        println!("ASCredentialIdentityStore.sharedStore obtained.");
        println!("Store description: {}", nsobj_to_string(store.0));

        // Check ASAuthorizationAppleIDProvider
        let provider = ASAuthorizationAppleIDProvider::alloc();
        let provider_ptr = INSObject::init(&provider);
        if !provider_ptr.is_null() {
            let provider = ASAuthorizationAppleIDProvider(provider_ptr);
            println!("\nASAuthorizationAppleIDProvider created.");

            // Create a request
            let request = IASAuthorizationAppleIDProvider::createRequest(&provider);
            if !request.0.is_null() {
                println!("Authorization request created.");
                let desc = nsobj_to_string(request.0);
                println!("Request: {desc}");
            }
        }

        // Check ASAuthorizationPasswordProvider
        let pwd_provider = ASAuthorizationPasswordProvider::alloc();
        let pwd_ptr = INSObject::init(&pwd_provider);
        if !pwd_ptr.is_null() {
            let pwd_provider = ASAuthorizationPasswordProvider(pwd_ptr);
            println!("\nASAuthorizationPasswordProvider created.");
            let request = IASAuthorizationPasswordProvider::createRequest(&pwd_provider);
            if !request.0.is_null() {
                println!("Password request: {}", nsobj_to_string(request.0));
            }
        }

        // Check ASWebAuthenticationSessionWebBrowserSessionManager
        let mgr = ASWebAuthenticationSessionWebBrowserSessionManager::sharedManager();
        if !mgr.0.is_null() {
            println!("\nASWebAuthenticationSessionWebBrowserSessionManager available.");
            let session_handler =
                IASWebAuthenticationSessionWebBrowserSessionManager::sessionHandler(&mgr);
            println!(
                "Session handler: {}",
                if (session_handler as *const u64).is_null() {
                    "not set"
                } else {
                    "set"
                }
            );
        }
    }

    println!("\nDone.");
}
