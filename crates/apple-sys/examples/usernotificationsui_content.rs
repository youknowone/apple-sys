//! Create notification content and inspect UserNotificationsUI types.
//!
//! Demonstrates UNMutableNotificationContent creation,
//! UNUserNotificationCenter access, and UserNotificationsUI constants.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::UserNotifications::*;
use apple_sys::UserNotificationsUI::*;
mod common;
use common::{nsstring, nsstring_to_string};

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== UserNotificationsUI ===\n");

        // 1. Get the current notification center
        let center =
            <UNUserNotificationCenter as IUNUserNotificationCenter>::currentNotificationCenter();
        println!("UNUserNotificationCenter: {:p}", center.0);

        let supports = IUNUserNotificationCenter::supportsContentExtensions(&center);
        println!("Supports content extensions: {}", supports.0);

        // 2. Create a mutable notification content
        let content = UNMutableNotificationContent::alloc();
        let content_ptr = INSObject::init(&content);
        let content = UNMutableNotificationContent(content_ptr);
        println!("\nUNMutableNotificationContent created: {:p}", content.0);

        // Set properties
        let title = nsstring(c"Hello from Rust");
        IUNMutableNotificationContent::setTitle_(&content, title);

        let subtitle = nsstring(c"apple-sys example");
        IUNMutableNotificationContent::setSubtitle_(&content, subtitle);

        let body = nsstring(c"This notification was created using apple_sys bindings.");
        IUNMutableNotificationContent::setBody_(&content, body);

        // Read them back via the IUNNotificationContent trait
        let read_title = IUNNotificationContent::title(&content);
        let read_subtitle = IUNNotificationContent::subtitle(&content);
        let read_body = IUNNotificationContent::body(&content);
        let read_category = IUNNotificationContent::categoryIdentifier(&content);
        let read_thread = IUNNotificationContent::threadIdentifier(&content);

        println!("\nNotification content:");
        println!("  Title:      \"{}\"", nsstring_to_string(read_title));
        println!("  Subtitle:   \"{}\"", nsstring_to_string(read_subtitle));
        println!("  Body:       \"{}\"", nsstring_to_string(read_body));
        println!("  Category:   \"{}\"", nsstring_to_string(read_category));
        println!("  Thread ID:  \"{}\"", nsstring_to_string(read_thread));

        // 3. Get the default notification sound and set it
        let sound = <UNNotificationSound as IUNNotificationSound>::defaultSound();
        println!("\nDefault notification sound: {:p}", sound.0);
        IUNMutableNotificationContent::setSound_(&content, sound);
        println!("Sound set on content.");

        // 4. UserNotificationsUI type sizes and constants
        println!("\nUserNotificationsUI types:");
        println!(
            "  UNNotificationContentExtensionMediaPlayPauseButtonType: {} bytes",
            std::mem::size_of::<UNNotificationContentExtensionMediaPlayPauseButtonType>()
        );
        println!(
            "  UNNotificationContentExtensionResponseOption: {} bytes",
            std::mem::size_of::<UNNotificationContentExtensionResponseOption>()
        );

        // MediaPlayPauseButtonType values:
        //   0 = None, 1 = Default, 2 = Overlay
        println!("\nMediaPlayPauseButtonType values:");
        println!("  None:    0");
        println!("  Default: 1");
        println!("  Overlay: 2");

        // ResponseOption values:
        //   0 = DoNotDismiss, 1 = Dismiss, 2 = DismissAndForwardAction
        println!("\nResponseOption values:");
        println!("  DoNotDismiss:            0");
        println!("  Dismiss:                 1");
        println!("  DismissAndForwardAction: 2");
    }

    println!("\nDone.");
}
