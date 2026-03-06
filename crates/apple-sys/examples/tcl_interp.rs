//! Create a Tcl interpreter, evaluate expressions, and query variables.
//!
//! Demonstrates Tcl_CreateInterp, Tcl_Eval, Tcl_GetVar,
//! Tcl_GetVersion, and Tcl_DeleteInterp.

use apple_sys::Tcl::*;
use std::ffi::CStr;

const TCL_OK: i32 = 0;

fn tcl_result_name(code: i32) -> &'static str {
    match code {
        0 => "TCL_OK",
        1 => "TCL_ERROR",
        2 => "TCL_RETURN",
        3 => "TCL_BREAK",
        4 => "TCL_CONTINUE",
        _ => "UNKNOWN",
    }
}

fn main() {
    unsafe {
        println!("=== Tcl ===\n");

        // 1. Query Tcl version
        let mut major: i32 = 0;
        let mut minor: i32 = 0;
        let mut patch: i32 = 0;
        let mut type_: i32 = 0;
        Tcl_GetVersion(&mut major, &mut minor, &mut patch, &mut type_);
        println!(
            "Tcl version: {}.{}.{} (type {})",
            major, minor, patch, type_
        );

        // 2. Initialize Tcl for finding packages
        Tcl_FindExecutable(c"tcl_interp_example".as_ptr());

        // 3. Create an interpreter
        let interp = Tcl_CreateInterp();
        if interp.is_null() {
            println!("Failed to create Tcl interpreter.");
            return;
        }
        println!("Tcl interpreter created.");

        // 4. Initialize standard Tcl library
        let ret = Tcl_Init(interp);
        println!("Tcl_Init: {}", tcl_result_name(ret));

        // 5. Evaluate some Tcl expressions
        let scripts: &[(&str, &CStr)] = &[
            ("arithmetic", c"expr {6 * 7}"),
            ("string length", c"string length \"hello world\""),
            ("list", c"list apple banana cherry"),
            ("info patchlevel", c"info patchlevel"),
            (
                "clock format",
                c"clock format [clock seconds] -format {%Y-%m-%d %H:%M:%S}",
            ),
        ];

        println!("\nEvaluating Tcl scripts:");
        for (label, script) in scripts {
            let ret = Tcl_Eval(interp, script.as_ptr());
            let result = Tcl_GetStringResult(interp);
            let result_str = if !result.is_null() {
                CStr::from_ptr(result).to_string_lossy()
            } else {
                "(null)".into()
            };
            println!("  {} => {} [{}]", label, result_str, tcl_result_name(ret));
        }

        // 6. Set and get variables
        println!("\nVariable operations:");
        let var_name = c"myVar";
        let var_value = c"Hello from Rust";
        let set_result = Tcl_SetVar(interp, var_name.as_ptr(), var_value.as_ptr(), 0);
        if !set_result.is_null() {
            println!("  Set myVar = \"Hello from Rust\"");
        }

        let get_result = Tcl_GetVar(interp, var_name.as_ptr(), 0);
        if !get_result.is_null() {
            let val = CStr::from_ptr(get_result).to_string_lossy();
            println!("  Get myVar = \"{}\"", val);
        }

        // 7. Test error handling
        println!("\nError handling:");
        let bad_script = c"nonexistent_command";
        let ret = Tcl_Eval(interp, bad_script.as_ptr());
        let result = Tcl_GetStringResult(interp);
        if ret != TCL_OK && !result.is_null() {
            let err = CStr::from_ptr(result).to_string_lossy();
            println!("  Expected error: {} [{}]", err, tcl_result_name(ret));
        }

        // 8. Memory allocation round-trip
        println!("\nMemory allocation:");
        let size = 256u32;
        let ptr = Tcl_Alloc(size);
        if !ptr.is_null() {
            println!("  Tcl_Alloc({}) = {:p}", size, ptr);
            // Write and read back
            *ptr = b'T' as i8;
            *ptr.add(1) = 0;
            println!("  Wrote 'T' at allocated memory.");
            Tcl_Free(ptr);
            println!("  Tcl_Free called.");
        } else {
            println!("  Tcl_Alloc returned null.");
        }

        // 9. Clean up
        Tcl_DeleteInterp(interp);
        println!("\nTcl interpreter deleted.");
    }

    println!("\nDone.");
}
