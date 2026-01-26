//! Initialize GLUT and create a window.
//!
//! Demonstrates glutInit, glutCreateWindow, glutGet for
//! querying screen dimensions, and glutDestroyWindow.

use apple_sys::GLUT::*;
use std::ffi::CString;
use std::os::raw::c_int;

// GLUT query constants (not exported as bindgen constants)
const GLUT_SCREEN_WIDTH: u32 = 200;
const GLUT_SCREEN_HEIGHT: u32 = 201;
const GLUT_WINDOW_X: u32 = 100;
const GLUT_WINDOW_Y: u32 = 101;
const GLUT_WINDOW_WIDTH: u32 = 102;
const GLUT_WINDOW_HEIGHT: u32 = 103;
const GLUT_ELAPSED_TIME: u32 = 700;

// Display mode flags
const GLUT_RGBA: u32 = 0x0000;
const GLUT_DOUBLE: u32 = 0x0002;
const GLUT_DEPTH: u32 = 0x0010;

fn main() {
    unsafe {
        println!("=== GLUT Framework ===\n");

        // Initialize GLUT
        let mut argc: c_int = 1;
        let prog = CString::new("glut_init").unwrap();
        let mut argv = [prog.as_ptr() as *mut _, std::ptr::null_mut()];
        glutInit(&mut argc, argv.as_mut_ptr());
        println!("glutInit completed.");

        // Set display mode
        glutInitDisplayMode(GLUT_RGBA | GLUT_DOUBLE | GLUT_DEPTH);
        println!("Display mode set: RGBA | DOUBLE | DEPTH");

        // Query screen dimensions
        let screen_w = glutGet(GLUT_SCREEN_WIDTH);
        let screen_h = glutGet(GLUT_SCREEN_HEIGHT);
        println!("\nScreen dimensions: {}x{}", screen_w, screen_h);

        // Set window position and size
        glutInitWindowPosition(100, 100);
        glutInitWindowSize(640, 480);

        // Create a window
        let title = CString::new("apple-sys GLUT Example").unwrap();
        let win = glutCreateWindow(title.as_ptr());
        println!("Created window: id={}", win);

        // Query window properties
        let win_x = glutGet(GLUT_WINDOW_X);
        let win_y = glutGet(GLUT_WINDOW_Y);
        let win_w = glutGet(GLUT_WINDOW_WIDTH);
        let win_h = glutGet(GLUT_WINDOW_HEIGHT);
        println!("  Position: ({}, {})", win_x, win_y);
        println!("  Size:     {}x{}", win_w, win_h);

        // Check elapsed time
        let elapsed = glutGet(GLUT_ELAPSED_TIME);
        println!("\nElapsed time since glutInit: {} ms", elapsed);

        // Check extension support
        let ext = CString::new("GL_ARB_texture_non_power_of_two").unwrap();
        let supported = glutExtensionSupported(ext.as_ptr());
        println!(
            "GL_ARB_texture_non_power_of_two: {}",
            if supported != 0 {
                "supported"
            } else {
                "not supported"
            }
        );

        // Clean up
        glutDestroyWindow(win);
        println!("\nWindow destroyed.");
    }

    println!("\nDone.");
}
