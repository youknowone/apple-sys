//! Verify SecurityFoundation bindings by exercising SFAuthorization
//! class methods and instance methods.

use apple_sys::SecurityFoundation::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SecurityFoundation Authorization ===\n");

        // Get the shared authorization via class method
        let auth_ptr = SFAuthorization::authorization();
        if !auth_ptr.is_null() {
            let auth = SFAuthorization(auth_ptr);
            println!("Authorization: {}", nsobj_to_string(auth_ptr));

            // Get the underlying AuthorizationRef
            let auth_ref = ISFAuthorization::authorizationRef(&auth);
            println!("AuthorizationRef: {:?}", auth_ref);

            // Invalidate credentials (safe operation - just clears cached credentials)
            ISFAuthorization::invalidateCredentials(&auth);
            println!("Invalidated credentials.");
        } else {
            println!("Failed to get authorization instance.");
        }

        // Create a fresh authorization with default flags
        println!("\n--- Fresh Authorization ---");
        let fresh = SFAuthorization::alloc();
        let fresh_ptr = ISFAuthorization::init(&fresh);
        if !fresh_ptr.is_null() {
            let fresh_auth = SFAuthorization(fresh_ptr);
            println!("Fresh auth: {}", nsobj_to_string(fresh_ptr));

            let auth_ref = ISFAuthorization::authorizationRef(&fresh_auth);
            println!("AuthorizationRef: {:?}", auth_ref);

            ISFAuthorization::invalidateCredentials(&fresh_auth);
            println!("Invalidated fresh credentials.");
        } else {
            println!("Failed to create fresh authorization.");
        }
    }

    println!("\nDone.");
}
