//! Evaluate JavaScript expressions using JavaScriptCore.
//!
//! Creates a JSC global context, evaluates several expressions,
//! and prints the results.

use apple_sys::JavaScriptCore::*;
use std::ffi::CString;
use std::ptr;

unsafe fn js_string_to_rust(_ctx: JSContextRef, js_str: JSStringRef) -> String {
    let max_len = unsafe { JSStringGetMaximumUTF8CStringSize(js_str) };
    let mut buf = vec![0u8; max_len];
    unsafe { JSStringGetUTF8CString(js_str, buf.as_mut_ptr() as _, max_len) };
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..end]).into_owned()
}

unsafe fn eval(ctx: JSGlobalContextRef, script: &str) -> String {
    let c_script = CString::new(script).unwrap();
    let js_script = unsafe { JSStringCreateWithUTF8CString(c_script.as_ptr()) };

    let mut exception: JSValueRef = ptr::null_mut();
    let result = unsafe {
        JSEvaluateScript(
            ctx,
            js_script,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            &mut exception,
        )
    };
    unsafe { JSStringRelease(js_script) };

    if !exception.is_null() {
        let exc_str = unsafe { JSValueToStringCopy(ctx, exception, ptr::null_mut()) };
        let s = unsafe { js_string_to_rust(ctx, exc_str) };
        unsafe { JSStringRelease(exc_str) };
        return format!("Error: {}", s);
    }

    if result.is_null() {
        return "undefined".to_string();
    }

    let result_str = unsafe { JSValueToStringCopy(ctx, result, ptr::null_mut()) };
    let s = unsafe { js_string_to_rust(ctx, result_str) };
    unsafe { JSStringRelease(result_str) };
    s
}

fn main() {
    unsafe {
        println!("=== JavaScriptCore Evaluation ===\n");

        let ctx = JSGlobalContextCreate(ptr::null_mut());
        assert!(!ctx.is_null(), "Failed to create JSC context");

        let expressions = [
            "2 + 3 * 4",
            "'Hello' + ' ' + 'World'",
            "Math.PI.toFixed(10)",
            "JSON.stringify({name: 'apple-sys', version: 1})",
            "Array.from({length: 5}, (_, i) => i * i)",
            "new Date().toISOString()",
            "Object.keys(Math).slice(0, 10).join(', ')",
            "(() => { let fib = n => n <= 1 ? n : fib(n-1) + fib(n-2); return fib(10); })()",
        ];

        for expr in &expressions {
            let result = eval(ctx, expr);
            println!("  {} = {}", expr, result);
        }

        // Check syntax validation
        println!("\nSyntax check:");
        let valid = CString::new("function f() { return 42; }").unwrap();
        let js_valid = JSStringCreateWithUTF8CString(valid.as_ptr());
        let ok = JSCheckScriptSyntax(ctx, js_valid, ptr::null_mut(), 0, ptr::null_mut());
        println!("  Valid syntax:   {}", ok);
        JSStringRelease(js_valid);

        let invalid = CString::new("function f( { }").unwrap();
        let js_invalid = JSStringCreateWithUTF8CString(invalid.as_ptr());
        let ok = JSCheckScriptSyntax(ctx, js_invalid, ptr::null_mut(), 0, ptr::null_mut());
        println!("  Invalid syntax: {}", ok);
        JSStringRelease(js_invalid);

        JSGlobalContextRelease(ctx);
    }

    println!("\nDone.");
}
