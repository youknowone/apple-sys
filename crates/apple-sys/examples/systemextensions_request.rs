//! Verify SystemExtensions bindings by getting the shared manager
//! and workspace, and querying system extension info.

use apple_sys::SystemExtensions::*;

mod common;
use common::nsobj_to_string;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SystemExtensions Framework ===\n");

        // OSSystemExtensionManager
        println!("--- OSSystemExtensionManager ---");
        let manager = OSSystemExtensionManager::sharedManager();
        if !manager.0.is_null() {
            println!("Shared manager: {}", nsobj_to_string(manager.0));
        } else {
            println!("Shared manager: (null)");
        }

        // OSSystemExtensionsWorkspace
        println!("\n--- OSSystemExtensionsWorkspace ---");
        let workspace = OSSystemExtensionsWorkspace::sharedWorkspace();
        if !workspace.0.is_null() {
            println!("Shared workspace: {}", nsobj_to_string(workspace.0));

            // Query system extensions for a well-known bundle ID
            let bundle_id = nsstring(c"com.apple.Safari");
            let error: *mut NSError = std::ptr::null_mut();
            let extensions =
                IOSSystemExtensionsWorkspace::systemExtensionsForApplicationWithBundleID_error_(
                    &workspace, bundle_id, error,
                );
            if !extensions.0.is_null() {
                let count = INSSet::<id>::count(&extensions);
                println!("  extensions for com.apple.Safari: {count}");
            } else if !error.is_null() {
                let desc = INSError::localizedDescription(&NSError(error as id));
                println!("  query error: {}", nsobj_to_string(desc.0));
            } else {
                println!("  extensions: (null)");
            }
        } else {
            println!("Shared workspace: (null)");
        }

        // OSSystemExtensionProperties
        println!("\n--- OSSystemExtensionProperties ---");
        let props = OSSystemExtensionProperties::alloc();
        let props_ptr = INSObject::init(&props);
        if !props_ptr.is_null() {
            let props = OSSystemExtensionProperties(props_ptr);
            println!("Properties: {}", nsobj_to_string(props_ptr));

            let url = IOSSystemExtensionProperties::URL(&props);
            println!("  URL: {}", nsobj_to_string(url.0));

            let bundle_id = IOSSystemExtensionProperties::bundleIdentifier(&props);
            println!("  bundleIdentifier: {}", nsstring_to_string(bundle_id));

            let version = IOSSystemExtensionProperties::bundleVersion(&props);
            println!("  bundleVersion: {}", nsstring_to_string(version));

            let short_version = IOSSystemExtensionProperties::bundleShortVersion(&props);
            println!(
                "  bundleShortVersion: {}",
                nsstring_to_string(short_version)
            );

            let enabled = IOSSystemExtensionProperties::isEnabled(&props);
            println!("  isEnabled: {}", enabled.0);

            let awaiting = IOSSystemExtensionProperties::isAwaitingUserApproval(&props);
            println!("  isAwaitingUserApproval: {}", awaiting.0);

            let uninstalling = IOSSystemExtensionProperties::isUninstalling(&props);
            println!("  isUninstalling: {}", uninstalling.0);
        } else {
            println!("OSSystemExtensionProperties: requires real extension context.");
        }
    }

    println!("\nDone.");
}
