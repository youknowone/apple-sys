//! Verify SharedWithYouCore bindings by exercising SWCollaborationCoordinator,
//! SWCollaborationMetadata, and SWCollaborationOption.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::SharedWithYouCore::*;

mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SharedWithYouCore Collaboration ===\n");

        // SWCollaborationCoordinator
        println!("--- SWCollaborationCoordinator ---");
        let coordinator = SWCollaborationCoordinator::sharedCoordinator();
        if !coordinator.0.is_null() {
            println!("Shared coordinator: {}", nsobj_to_string(coordinator.0));

            let handler = ISWCollaborationCoordinator::actionHandler(&coordinator);
            println!("  actionHandler: {:?}", handler);
        } else {
            println!("Failed to get shared coordinator.");
        }

        // SWCollaborationMetadata
        println!("\n--- SWCollaborationMetadata ---");
        let local_id = nsstring(c"com.example.test-collaboration");
        let meta = SWCollaborationMetadata::alloc();
        let meta_ptr = ISWCollaborationMetadata::initWithLocalIdentifier_(&meta, local_id);
        if !meta_ptr.is_null() {
            let meta = SWCollaborationMetadata(meta_ptr);
            println!("Metadata: {}", nsobj_to_string(meta_ptr));

            let local_id = ISWCollaborationMetadata::localIdentifier(&meta);
            println!("  localIdentifier: {}", nsstring_to_string(local_id));

            let collab_id = ISWCollaborationMetadata::collaborationIdentifier(&meta);
            println!(
                "  collaborationIdentifier: {}",
                nsstring_to_string(collab_id)
            );

            let title = ISWCollaborationMetadata::title(&meta);
            println!("  title: {}", nsstring_to_string(title));

            // Set and verify title
            let test_title = nsstring(c"Test Document");
            ISWCollaborationMetadata::setTitle_(&meta, test_title);
            let read_back = ISWCollaborationMetadata::title(&meta);
            println!("  title (after set): {}", nsstring_to_string(read_back));

            let share_opts = ISWCollaborationMetadata::defaultShareOptions(&meta);
            println!("  defaultShareOptions: {}", nsobj_to_string(share_opts.0));
        } else {
            println!("Failed to create SWCollaborationMetadata.");
        }

        // SWCollaborationOption
        println!("\n--- SWCollaborationOption ---");
        let opt_title = nsstring(c"Anyone with the link");
        let opt_id = nsstring(c"option-anyone");
        let option = SWCollaborationOption::optionWithTitle_identifier_(opt_title, opt_id);
        if !option.0.is_null() {
            println!("Option: {}", nsobj_to_string(option.0));

            let title = ISWCollaborationOption::title(&option);
            println!("  title: {}", nsstring_to_string(title));

            let identifier = ISWCollaborationOption::identifier(&option);
            println!("  identifier: {}", nsstring_to_string(identifier));

            let selected = ISWCollaborationOption::isSelected(&option);
            println!("  isSelected: {}", selected.0);

            let subtitle = ISWCollaborationOption::subtitle(&option);
            println!("  subtitle: {}", nsstring_to_string(subtitle));
        } else {
            println!("Failed to create SWCollaborationOption.");
        }

        // SWPerson
        println!("\n--- SWPerson ---");
        let person = SWPerson::alloc();
        let person_ptr = INSObject::init(&person);
        if !person_ptr.is_null() {
            println!("Person: {}", nsobj_to_string(person_ptr));
        } else {
            println!("SWPerson: requires initialization parameters.");
        }
    }

    println!("\nDone.");
}
