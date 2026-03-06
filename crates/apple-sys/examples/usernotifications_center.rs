//! Query UNUserNotificationCenter using UserNotifications.
//!
//! Gets the current notification center and checks content extension
//! support, notification settings, and authorization capabilities.

use apple_sys::CoreFoundation::{INSObject, PNSObject};
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::UserNotifications::*;
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== UserNotifications Center ===\n");

        println!("UNUserNotificationCenter class: available");

        let center = UNUserNotificationCenter::currentNotificationCenter();
        if !center.0.is_null() {
            println!("Current center: {}", nsobj_to_string(center.0));

            let supports_content_ext =
                IUNUserNotificationCenter::supportsContentExtensions(&center);
            println!("Supports content extensions: {}", supports_content_ext.0);

            let delegate = IUNUserNotificationCenter::delegate(&center);
            println!("Delegate: {}", nsobj_to_string(delegate as id));

            let responds_auth = PNSObject::respondsToSelector_(
                &center,
                objc2::sel!(requestAuthorizationWithOptions:completionHandler:),
            );
            println!("Responds to requestAuthorization: {}", responds_auth.0);

            let responds_add = PNSObject::respondsToSelector_(
                &center,
                objc2::sel!(addNotificationRequest:withCompletionHandler:),
            );
            println!("Responds to addNotificationRequest: {}", responds_add.0);

            let responds_pending = PNSObject::respondsToSelector_(
                &center,
                objc2::sel!(getPendingNotificationRequestsWithCompletionHandler:),
            );
            println!(
                "Responds to getPendingNotificationRequests: {}",
                responds_pending.0
            );

            let responds_delivered = PNSObject::respondsToSelector_(
                &center,
                objc2::sel!(getDeliveredNotificationsWithCompletionHandler:),
            );
            println!(
                "Responds to getDeliveredNotifications: {}",
                responds_delivered.0
            );
        } else {
            println!("Failed to get current notification center.");
        }

        println!("\n--- UNNotificationContent ---");
        let content = UNMutableNotificationContent::alloc();
        let content_ptr = INSObject::init(&content);
        if !content_ptr.is_null() {
            let content = UNMutableNotificationContent(content_ptr);
            println!("UNMutableNotificationContent: available");
            println!("  Content: {}", nsobj_to_string(content_ptr));

            let title = IUNMutableNotificationContent::title(&content);
            println!("  Title: {}", nsstring_to_string(title));

            let body = IUNMutableNotificationContent::body(&content);
            println!("  Body: {}", nsstring_to_string(body));

            let badge = IUNMutableNotificationContent::badge(&content);
            println!("  Badge: {}", nsobj_to_string(badge.0));

            let sound = IUNMutableNotificationContent::sound(&content);
            println!("  Sound: {}", nsobj_to_string(sound.0));
        } else {
            println!("UNMutableNotificationContent: not available");
        }

        // UNNotificationCategory and UNNotificationAction have bindings
        println!("\nUNNotificationCategory: available");
        println!("UNNotificationAction: available");
    }

    println!("\nDone.");
}
