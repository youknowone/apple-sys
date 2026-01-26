//! Access the CalendarStore to list calendars and their properties.
//!
//! Uses CalCalendarStore to enumerate available calendars.

use apple_sys::CalendarStore::*;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CalendarStore Events ===\n");

        let store = CalCalendarStore::defaultCalendarStore();
        if store.0.is_null() {
            println!("Could not get defaultCalendarStore (Calendar access may be denied).");
            return;
        }
        println!("CalCalendarStore.defaultCalendarStore obtained.\n");

        // List calendars
        let calendars = ICalCalendarStore::calendars(&store);
        if calendars.0.is_null() {
            println!("No calendars found.");
            return;
        }

        let count = INSArray::<id>::count(&calendars);
        println!("Total calendars: {count}\n");

        for i in 0..count {
            let cal: id = INSArray::<id>::objectAtIndex_(&calendars, i);
            let cal = CalCalendar(cal);
            let title = ICalCalendar::title(&cal);
            let uid = ICalCalendar::uid(&cal);
            let notes = ICalCalendar::notes(&cal);
            let cal_type = ICalCalendar::type_(&cal);

            let editable = ICalCalendar::isEditable(&cal);

            println!("Calendar #{}: {}", i + 1, nsstring_to_string(title));
            println!("  UID:      {}", nsstring_to_string(uid));
            println!("  Type:     {}", nsstring_to_string(cal_type));
            println!("  Editable: {}", editable.0);
            if !notes.0.is_null() {
                let notes_str = nsstring_to_string(notes);
                if !notes_str.is_empty() {
                    println!("  Notes:    {notes_str}");
                }
            }

            // Color
            let color = ICalCalendar::color(&cal);
            if !color.0.is_null() {
                println!("  Color:    {}", nsobj_to_string(color.0));
            }

            println!();
        }
    }

    println!("Done.");
}
