//! List available smart card readers via PC/SC.
//!
//! Uses SCardEstablishContext and SCardListReaders to enumerate
//! connected smart card readers on the system.

use apple_sys::PCSC::*;
use std::ffi::CStr;
use std::ptr;

const SCARD_SCOPE_SYSTEM: u32 = 2;
const SCARD_S_SUCCESS: i32 = 0;
const SCARD_E_NO_READERS_AVAILABLE: i32 = -2146435026i64 as i32;

fn main() {
    unsafe {
        println!("=== PC/SC Smart Card Readers ===\n");

        // Establish context
        let mut context: SCARDCONTEXT = 0;
        let rv = SCardEstablishContext(SCARD_SCOPE_SYSTEM, ptr::null(), ptr::null(), &mut context);
        if rv != SCARD_S_SUCCESS {
            eprintln!("SCardEstablishContext failed: 0x{:08x}", rv as u32);
            return;
        }
        println!("PC/SC context established");

        // Check validity
        let rv = SCardIsValidContext(context);
        println!("Context valid: {}", rv == SCARD_S_SUCCESS);

        // List readers — first call to get buffer size
        let mut readers_len: u32 = 0;
        let rv = SCardListReaders(context, ptr::null(), ptr::null_mut(), &mut readers_len);

        if rv == SCARD_E_NO_READERS_AVAILABLE || readers_len == 0 {
            println!("\nNo smart card readers found.");
            SCardReleaseContext(context);
            println!("\nDone.");
            return;
        }

        if rv != SCARD_S_SUCCESS {
            eprintln!("SCardListReaders (size query) failed: 0x{:08x}", rv as u32);
            SCardReleaseContext(context);
            return;
        }

        // Second call to get actual reader names (multi-string, null-separated)
        let mut buf = vec![0i8; readers_len as usize];
        let rv = SCardListReaders(context, ptr::null(), buf.as_mut_ptr(), &mut readers_len);

        if rv != SCARD_S_SUCCESS {
            eprintln!("SCardListReaders failed: 0x{:08x}", rv as u32);
            SCardReleaseContext(context);
            return;
        }

        // Parse multi-string (null-separated, double-null terminated)
        let mut readers = vec![];
        let mut offset = 0;
        while offset < readers_len as usize {
            let s = CStr::from_ptr(buf.as_ptr().add(offset));
            if s.to_bytes().is_empty() {
                break;
            }
            readers.push(s.to_string_lossy().into_owned());
            offset += s.to_bytes_with_nul().len();
        }

        println!("\nFound {} reader(s):", readers.len());
        for (i, name) in readers.iter().enumerate() {
            println!("  {}: {}", i, name);
        }

        SCardReleaseContext(context);
    }

    println!("\nDone.");
}
