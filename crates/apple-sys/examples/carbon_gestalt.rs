//! Explore Carbon HIToolbox event loop API.
//!
//! Gets the main event loop, creates and inspects a Carbon event,
//! and queries the current event queue depth.

use apple_sys::Carbon::*;
use std::ptr;

fn main() {
    unsafe {
        println!("=== Carbon HIToolbox Event API ===\n");

        // Get the main event loop
        let main_loop = GetMainEventLoop();
        println!("Main event loop:    {:?}", main_loop);

        // Get the current event loop (same as main in a simple app)
        let current_loop = GetCurrentEventLoop();
        println!("Current event loop: {:?}", current_loop);

        // Convert to CFRunLoop to show interoperability
        let cf_runloop = GetCFRunLoopFromEventLoop(main_loop);
        println!("CFRunLoop ref:      {:?}", cf_runloop);

        // Get the main event queue and check how many events are pending
        let main_queue = GetMainEventQueue();
        let num_events = GetNumEventsInQueue(main_queue);
        println!("\nMain event queue:   {:?}", main_queue);
        println!("Pending events:     {}", num_events);

        // Create a custom Carbon event
        // Use application-defined class 'test' and kind 1
        let event_class: u32 = u32::from_be_bytes(*b"test");
        let event_kind: u32 = 1;
        let mut event: EventRef = ptr::null_mut();
        let err = CreateEvent(
            ptr::null(), // default allocator
            event_class,
            event_kind,
            0.0, // current time
            0,   // kEventAttributeNone
            &mut event,
        );

        if err == 0 {
            let cls = GetEventClass(event);
            let kind = GetEventKind(event);
            let time = GetEventTime(event);
            let retain = GetEventRetainCount(event);

            let cls_bytes = cls.to_be_bytes();
            let cls_str = std::str::from_utf8(&cls_bytes).unwrap_or("????");

            println!("\nCreated Carbon event:");
            println!("  Class:        '{}' (0x{:08x})", cls_str, cls);
            println!("  Kind:         {}", kind);
            println!("  Time:         {:.3}s", time);
            println!("  Retain count: {}", retain);

            // Copy the event
            let copy = CopyEvent(event);
            if !copy.is_null() {
                println!(
                    "  Copied event: {:?} (class='{}')",
                    copy,
                    std::str::from_utf8(&GetEventClass(copy).to_be_bytes()).unwrap_or("????"),
                );
                ReleaseEvent(copy);
            }

            ReleaseEvent(event);
        } else {
            eprintln!("CreateEvent failed: {}", err);
        }

        // Query current keyboard modifier state
        let modifiers = GetCurrentEventKeyModifiers();
        let button = GetCurrentEventButtonState();
        println!("\nCurrent input state:");
        println!("  Key modifiers: 0x{:08x}", modifiers);
        println!("  Button state:  0x{:08x}", button);
    }

    println!("\nDone.");
}
