//! Create an ACAccountStore and query available account types.
//!
//! Uses the Accounts framework to inspect supported social account types.

use apple_sys::Accounts::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::objc::id;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Accounts Framework ===\n");

        let store = ACAccountStore::alloc();
        let store = INSObject::init(&store);
        if store.is_null() {
            println!("Failed to create ACAccountStore.");
            return;
        }
        let store = ACAccountStore(store);
        println!("ACAccountStore created successfully.");

        // Query known account type identifiers
        let type_ids = [
            ("Twitter", c"com.apple.twitter"),
            ("Facebook", c"com.apple.facebook"),
            ("Sina Weibo", c"com.apple.sinaweibo"),
            ("Tencent Weibo", c"com.apple.tencentweibo"),
            ("LinkedIn", c"com.apple.linkedin"),
        ];

        println!("\nAccount type availability:");
        for (name, identifier) in &type_ids {
            let type_id = nsstring(identifier);
            let account_type =
                IACAccountStore::accountTypeWithAccountTypeIdentifier_(&store, type_id);
            if !account_type.0.is_null() {
                let desc = IACAccountType::accountTypeDescription(&account_type);
                println!(
                    "  {name}: available (description: {})",
                    nsstring_to_string(desc)
                );
            } else {
                println!("  {name}: not available");
            }
        }

        // List all accounts in the store
        let accounts = IACAccountStore::accounts(&store);
        if !accounts.0.is_null() {
            let count = INSArray::<id>::count(&accounts);
            println!("\nTotal accounts in store: {count}");
            for i in 0..count {
                let acct: id = INSArray::<id>::objectAtIndex_(&accounts, i);
                let acct = ACAccount(acct);
                let username = IACAccount::username(&acct);
                let acct_type = IACAccount::accountType(&acct);
                let type_desc = IACAccountType::accountTypeDescription(&acct_type);
                println!(
                    "  Account #{i}: user={}, type={}",
                    nsstring_to_string(username),
                    nsstring_to_string(type_desc)
                );
            }
        } else {
            println!("\nNo accounts found (or access not granted).");
        }
    }

    println!("\nDone.");
}
