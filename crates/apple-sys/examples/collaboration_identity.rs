//! Query identity authorities using the Collaboration framework.
//!
//! Accesses CBIdentityAuthority to list the default and local authorities,
//! and searches for the current user identity.

use apple_sys::Collaboration::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{
    INSArray, INSDictionary, INSProcessInfo, NSAutoreleasePool, NSProcessInfo, NSString,
};
use apple_sys::objc::id;
mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Collaboration Identity ===\n");

        // Default authority
        let default_auth = CBIdentityAuthority::defaultIdentityAuthority();
        if !default_auth.0.is_null() {
            let name = ICBIdentityAuthority::localizedName(&default_auth);
            println!("Default authority: {}", nsstring_to_string(name));
        }

        // Local authority
        let local_auth = CBIdentityAuthority::localIdentityAuthority();
        if !local_auth.0.is_null() {
            let name = ICBIdentityAuthority::localizedName(&local_auth);
            println!("Local authority:   {}", nsstring_to_string(name));
        }

        // Managed authority
        let managed_auth = CBIdentityAuthority::managedIdentityAuthority();
        if !managed_auth.0.is_null() {
            let name = ICBIdentityAuthority::localizedName(&managed_auth);
            println!("Managed authority: {}", nsstring_to_string(name));
        }

        // Try to find current user identity
        println!("\n--- Current User Identity ---");

        // Get current username from NSProcessInfo
        let proc_info = NSProcessInfo::processInfo();
        let env = INSProcessInfo::environment(&proc_info);
        let user_key = nsstring(c"USER");
        let username = INSDictionary::<id, id>::objectForKey_(&env, user_key.0);

        if !username.is_null() {
            let uname = nsobj_to_string(username);
            println!("Looking up user: {uname}");

            let authority = if !default_auth.0.is_null() {
                default_auth
            } else {
                local_auth
            };

            let identity = CBIdentity::identityWithName_authority_(NSString(username), authority);

            if !identity.0.is_null() {
                let full_name = ICBIdentity::fullName(&identity);
                let posix_name = ICBIdentity::posixName(&identity);
                let uuid = ICBIdentity::UUIDString(&identity);
                let is_hidden = ICBIdentity::isHidden(&identity);

                println!("  Full name:  {}", nsstring_to_string(full_name));
                println!("  POSIX name: {}", nsstring_to_string(posix_name));
                println!("  UUID:       {}", nsstring_to_string(uuid));
                println!("  Hidden:     {}", is_hidden.0);

                // Aliases
                let aliases = ICBIdentity::aliases(&identity);
                if !aliases.0.is_null() {
                    let alias_count = INSArray::<id>::count(&aliases);
                    println!("  Aliases: {alias_count}");
                }
            } else {
                println!("  Identity not found for '{uname}'.");
            }
        }
    }

    println!("\nDone.");
}
