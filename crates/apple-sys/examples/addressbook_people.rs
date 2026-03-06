//! Query the shared AddressBook for contacts.
//!
//! Uses the AddressBook ObjC API to access ABAddressBook and list people.

use apple_sys::AddressBook::*;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== AddressBook Framework ===\n");

        // Use ObjC ABAddressBook API
        let ab = ABAddressBook::sharedAddressBook();

        if ab.0.is_null() {
            println!("Could not access shared address book (permission may be denied).");
            return;
        }

        // Get "me" record
        let me = IABAddressBook::me(&ab);
        if !me.0.is_null() {
            let first: id = IABRecord::valueForProperty_(&me, nsstring(c"First"));
            let last: id = IABRecord::valueForProperty_(&me, nsstring(c"Last"));
            println!(
                "Me card: {} {}",
                nsobj_to_string(first),
                nsobj_to_string(last)
            );
        } else {
            println!("No 'Me' card set.");
        }

        // Count all people
        let people = IABAddressBook::people(&ab);
        if !people.0.is_null() {
            let count = INSArray::<id>::count(&people);
            println!("Total people in address book: {count}");

            // Print first few names
            let show = count.min(5);
            for i in 0..show {
                let person: id = INSArray::<id>::objectAtIndex_(&people, i);
                let person = ABPerson(person);
                let first: id = IABRecord::valueForProperty_(&person, nsstring(c"First"));
                let last: id = IABRecord::valueForProperty_(&person, nsstring(c"Last"));
                println!(
                    "  #{}: {} {}",
                    i + 1,
                    nsobj_to_string(first),
                    nsobj_to_string(last)
                );
            }
            if count > show {
                println!("  ... and {} more", count - show);
            }
        }

        // Count groups
        let groups = IABAddressBook::groups(&ab);
        if !groups.0.is_null() {
            let count = INSArray::<id>::count(&groups);
            println!("\nTotal groups: {count}");
            for i in 0..count.min(5) {
                let group: id = INSArray::<id>::objectAtIndex_(&groups, i);
                let group = ABGroup(group);
                let name: id = IABRecord::valueForProperty_(&group, nsstring(c"GroupName"));
                println!("  Group: {}", nsobj_to_string(name));
            }
        }
    }

    println!("\nDone.");
}
