//! Exercise InputMethodKit server and input controller bindings.
//!
//! Creates an IMKServer with a connection name and bundle identifier,
//! and queries its properties using the generated bindings.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::InputMethodKit::*;
use apple_sys::objc::id;

mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== InputMethodKit ===\n");

        // IMKServer - create with a connection name and bundle ID
        println!("--- IMKServer ---");
        let server = IMKServer::alloc();
        let conn_name = nsstring(c"TestIMKConnection");
        let bundle_id = nsstring(c"com.example.testIMK");
        let server_ptr = IIMKServer::initWithName_bundleIdentifier_(&server, conn_name, bundle_id);
        if !server_ptr.is_null() {
            let server = IMKServer(server_ptr);
            let bundle = IIMKServer::bundle(&server);
            if !bundle.0.is_null() {
                println!("  Bundle: {}", nsobj_to_string(bundle.0));
            } else {
                println!("  Bundle: (nil - no matching bundle)");
            }
            let dead_key = IIMKServer::lastKeyEventWasDeadKey(&server);
            println!("  lastKeyEventWasDeadKey: {}", dead_key.0);
            let palette = IIMKServer::paletteWillTerminate(&server);
            println!("  paletteWillTerminate: {}", palette.0);
        } else {
            println!("  (initWithName returned nil)");
        }

        // IMKInputController
        println!("\n--- IMKInputController ---");
        let controller = IMKInputController::alloc();
        // initWithServer:delegate:client: - pass nil server, nil delegate, nil client
        let nil: id = std::ptr::null_mut();
        let ctrl_ptr = IIMKInputController::initWithServer_delegate_client_(
            &controller,
            IMKServer(nil),
            nil,
            nil,
        );
        if !ctrl_ptr.is_null() {
            let controller = IMKInputController(ctrl_ptr);
            let server = IIMKInputController::server(&controller);
            println!("  Server: {}", nsobj_to_string(server.0));
            let delegate = IIMKInputController::delegate(&controller);
            println!("  Delegate: {}", nsobj_to_string(delegate));
        } else {
            println!("  (init returned nil - requires valid server)");
        }

        // IMK command extern strings
        println!("\n--- IMK extern strings ---");
        let cmd_menu = kIMKCommandMenuItemName;
        let cmd_client = kIMKCommandClientName;
        println!(
            "  kIMKCommandMenuItemName: {}",
            nsstring_to_string(cmd_menu)
        );
        println!(
            "  kIMKCommandClientName:   {}",
            nsstring_to_string(cmd_client)
        );
    }

    println!("\nDone.");
}
