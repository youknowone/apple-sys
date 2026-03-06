//! List calendars via EventKit.
//!
//! Uses EKEventStore to access the user's calendars and display
//! their names, types, and colors.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::EventKit::*;
use apple_sys::Foundation::{INSArray, NSAutoreleasePool};
use apple_sys::objc::id;
mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== EventKit Calendars ===\n");

        let store = EKEventStore::alloc();
        let store_ptr = INSObject::init(&store);
        assert!(!store_ptr.is_null(), "Failed to create EKEventStore");
        let store = EKEventStore(store_ptr);

        // List event calendars (type 0 = Event)
        let calendars = IEKEventStore::calendarsForEntityType_(&store, 0);
        let count = INSArray::<id>::count(&calendars);
        println!("Event calendars: {}", count);

        for i in 0..count {
            let cal_id = INSArray::<id>::objectAtIndex_(&calendars, i);
            let cal = EKCalendar(cal_id);
            let title = IEKCalendar::title(&cal);
            let source = IEKCalendar::source(&cal);
            let source_title = IEKSource::title(&source);
            let cal_type = IEKCalendar::type_(&cal);
            let allows_modify = IEKCalendar::allowsContentModifications(&cal);

            let type_name = match cal_type {
                0 => "Local",
                1 => "CalDAV",
                2 => "Exchange",
                3 => "Subscription",
                4 => "Birthday",
                _ => "Other",
            };

            println!(
                "  {:2}. {} ({}{})",
                i + 1,
                nsstring_to_string(title),
                type_name,
                if allows_modify.0 { "" } else { ", read-only" }
            );
            if !source.0.is_null() {
                println!("      Source: {}", nsstring_to_string(source_title));
            }
        }

        // List reminder calendars (type 1 = Reminder)
        let reminders = IEKEventStore::calendarsForEntityType_(&store, 1);
        let rcount = INSArray::<id>::count(&reminders);
        println!("\nReminder calendars: {}", rcount);

        for i in 0..rcount {
            let cal_id = INSArray::<id>::objectAtIndex_(&reminders, i);
            let title = IEKCalendar::title(&EKCalendar(cal_id));
            println!("  {:2}. {}", i + 1, nsstring_to_string(title));
        }

        // Default calendar
        let default_cal = IEKEventStore::defaultCalendarForNewEvents(&store);
        if !default_cal.0.is_null() {
            let title = IEKCalendar::title(&default_cal);
            println!("\nDefault calendar: {}", nsstring_to_string(title));
        }
    }

    println!("\nDone.");
}
