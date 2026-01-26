//! Explore Uniform Type Identifiers.
//!
//! Uses UTType to look up common file types and query
//! their properties (identifier, MIME type, extension).

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::UniformTypeIdentifiers::*;

use std::ffi::CStr;
mod common;
use common::nsstring;
use common::nsstring_to_string;

unsafe fn describe_type(identifier: &CStr) {
    let id_str = unsafe { nsstring(identifier) };
    let ut_ptr = unsafe { UTType::typeWithIdentifier_(id_str) };
    if ut_ptr.is_null() {
        return;
    }
    let ut = UTType(ut_ptr);
    let ident = unsafe { IUTType::identifier(&ut) };
    let desc = unsafe { IUTType::localizedDescription(&ut) };
    let ext = unsafe { IUTType::preferredFilenameExtension(&ut) };
    let mime = unsafe { IUTType::preferredMIMEType(&ut) };
    let is_public = unsafe { IUTType::isPublicType(&ut) };
    let is_declared = unsafe { IUTType::isDeclared(&ut) };

    println!("  {}:", unsafe { nsstring_to_string(ident) });
    println!("    Description: {}", unsafe { nsstring_to_string(desc) });
    println!("    Extension:   {}", unsafe { nsstring_to_string(ext) });
    println!("    MIME type:   {}", unsafe { nsstring_to_string(mime) });
    println!("    Public: {}, Declared: {}", is_public.0, is_declared.0);
}

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Uniform Type Identifiers ===\n");

        let identifiers = [
            c"public.plain-text",
            c"public.html",
            c"public.json",
            c"public.png",
            c"public.jpeg",
            c"public.pdf",
            c"public.mp3",
            c"public.mpeg-4",
            c"public.zip-archive",
            c"com.apple.application-bundle",
            c"public.folder",
        ];

        println!("Known types:");
        for id in &identifiers {
            describe_type(id);
            println!();
        }

        // Look up by file extension
        println!("Types by extension:");
        let extensions = [c"rs", c"toml", c"swift", c"py", c"cpp"];
        for ext in &extensions {
            let ext_str = nsstring(ext);
            let ut_ptr = UTType::typeWithFilenameExtension_(ext_str);
            if !ut_ptr.is_null() {
                let ut = UTType(ut_ptr);
                let ident = IUTType::identifier(&ut);
                let desc = IUTType::localizedDescription(&ut);
                println!(
                    "  .{} → {} ({})",
                    CStr::from_ptr(ext.as_ptr()).to_string_lossy(),
                    nsstring_to_string(ident),
                    nsstring_to_string(desc)
                );
            }
        }
    }

    println!("\nDone.");
}
