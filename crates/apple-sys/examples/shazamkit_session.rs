//! Verify ShazamKit bindings by creating an SHSession, querying its
//! catalog, and getting the default SHMediaLibrary.

use apple_sys::ShazamKit::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ShazamKit Session & Library ===\n");

        // SHSession
        println!("--- SHSession ---");
        let session = SHSession::alloc();
        let session_ptr = INSObject::init(&session);
        if !session_ptr.is_null() {
            let session = SHSession(session_ptr);
            println!("Session: {}", nsobj_to_string(session_ptr));

            let catalog = ISHSession::catalog(&session);
            println!("  catalog: {}", nsobj_to_string(catalog.0));

            let delegate = ISHSession::delegate(&session);
            println!("  delegate: {:?}", delegate);
        } else {
            println!("Failed to create SHSession.");
        }

        // SHMediaLibrary
        println!("\n--- SHMediaLibrary ---");
        let library = SHMediaLibrary::defaultLibrary();
        if !library.0.is_null() {
            println!("Default library: {}", nsobj_to_string(library.0));
        } else {
            println!("Default library: (null)");
        }

        // SHCustomCatalog
        println!("\n--- SHCustomCatalog ---");
        let catalog = SHCustomCatalog::alloc();
        let catalog_ptr = INSObject::init(&catalog);
        if !catalog_ptr.is_null() {
            let catalog = SHCustomCatalog(catalog_ptr);
            println!("CustomCatalog: {}", nsobj_to_string(catalog_ptr));

            let min_query_sig_dur = ISHCatalog::minimumQuerySignatureDuration(&catalog);
            println!("  minimumQuerySignatureDuration: {min_query_sig_dur}");

            let max_query_sig_dur = ISHCatalog::maximumQuerySignatureDuration(&catalog);
            println!("  maximumQuerySignatureDuration: {max_query_sig_dur}");
        } else {
            println!("Failed to create SHCustomCatalog.");
        }

        // SHSignatureGenerator
        println!("\n--- SHSignatureGenerator ---");
        let sig_gen = SHSignatureGenerator::alloc();
        let sig_gen_ptr = INSObject::init(&sig_gen);
        if !sig_gen_ptr.is_null() {
            println!("SignatureGenerator: {}", nsobj_to_string(sig_gen_ptr));
        } else {
            println!("Failed to create SHSignatureGenerator.");
        }
    }

    println!("\nDone.");
}
