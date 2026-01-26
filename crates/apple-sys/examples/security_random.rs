//! Generate cryptographically secure random bytes using the Security framework.
//!
//! Uses SecRandomCopyBytes to generate random data and prints it as hex.

use apple_sys::Security::*;

fn main() {
    unsafe {
        // Generate 32 random bytes
        let mut bytes = [0u8; 32];
        let status = SecRandomCopyBytes(
            kSecRandomDefault,
            bytes.len(),
            bytes.as_mut_ptr() as *mut std::ffi::c_void,
        );

        if status != 0 {
            eprintln!("SecRandomCopyBytes failed with status: {status}");
            return;
        }

        print!("Random bytes (32): ");
        for b in &bytes {
            print!("{b:02x}");
        }
        println!();
    }
}
