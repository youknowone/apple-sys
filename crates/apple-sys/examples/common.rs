#![allow(dead_code)]

use apple_sys::Foundation::*;
use apple_sys::objc::id;
use objc2::msg_send;
use std::ffi::CStr;

/// Convert any NSObject (via `-description` then `-UTF8String`) to a Rust `String`.
///
/// Returns `"(null)"` for null pointers.
pub unsafe fn nsobj_to_string(obj: id) -> String {
    if obj.is_null() {
        return "(null)".to_string();
    }
    // obj is raw `id` — no concrete type for trait dispatch.
    let desc: NSString = unsafe { msg_send![&*obj, description] };
    if desc.0.is_null() {
        return "(nil)".to_string();
    }
    let cstr = unsafe { desc.UTF8String() };
    if cstr.is_null() {
        return "(nil)".to_string();
    }
    unsafe { CStr::from_ptr(cstr) }
        .to_string_lossy()
        .into_owned()
}

/// Convert an `NSString` to a Rust `String`.
///
/// Returns an empty string for null pointers.
pub unsafe fn nsstring_to_string(s: NSString) -> String {
    if s.0.is_null() {
        return String::new();
    }
    let cstr = unsafe { s.UTF8String() };
    if cstr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(cstr) }
        .to_string_lossy()
        .into_owned()
}

/// Create an `NSString` from a `&CStr`.
pub unsafe fn nsstring(s: &CStr) -> NSString {
    let obj = NSString::alloc();
    let ptr = unsafe { obj.initWithUTF8String_(s.as_ptr()) };
    NSString(ptr)
}
