//! Create PencilKit ink tools and drawing objects.
//!
//! Uses PKInkingTool, PKEraserTool, and PKDrawing
//! to demonstrate PencilKit tool creation.

use apple_sys::AppKit::{INSColor, NSColor};
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, INSData, NSAutoreleasePool};
use apple_sys::PencilKit::*;
use apple_sys::objc::id;
mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== PencilKit ===\n");

        // Create ink objects
        let ink_types = [
            (c"com.apple.ink.pen", "Pen"),
            (c"com.apple.ink.pencil", "Pencil"),
            (c"com.apple.ink.marker", "Marker"),
            (c"com.apple.ink.monoline", "Monoline"),
            (c"com.apple.ink.fountainPen", "Fountain Pen"),
            (c"com.apple.ink.watercolor", "Watercolor"),
            (c"com.apple.ink.crayon", "Crayon"),
        ];

        println!("Ink types:");
        for (type_id, name) in &ink_types {
            let blue = NSColor::blueColor();
            let ink = PKInk::alloc();
            let ink_id = IPKInk::initWithInkType_color_(&ink, nsstring(type_id), blue);
            if !ink_id.is_null() {
                let ink = PKInk(ink_id);
                let ink_type = IPKInk::inkType(&ink);
                println!("  {} ({})", name, nsstring_to_string(ink_type));
            } else {
                println!("  {} -- not available", name);
            }
        }

        // Eraser tool
        {
            println!("\nEraser types:");
            // PKEraserType: 0=vector, 1=bitmap, 2=fixedWidthBitmap
            for (eraser_type, name) in [(0i64, "Vector"), (1i64, "Bitmap")] {
                let eraser = PKEraserTool::alloc();
                let eraser_id = IPKEraserTool::initWithEraserType_(&eraser, eraser_type);
                if !eraser_id.is_null() {
                    let eraser = PKEraserTool(eraser_id);
                    let width = IPKEraserTool::width(&eraser);
                    let default_w =
                        <PKEraserTool as IPKEraserTool>::defaultWidthForEraserType_(eraser_type);
                    let min_w =
                        <PKEraserTool as IPKEraserTool>::minimumWidthForEraserType_(eraser_type);
                    let max_w =
                        <PKEraserTool as IPKEraserTool>::maximumWidthForEraserType_(eraser_type);
                    println!(
                        "  {}: width={:.1} (default={:.1}, min={:.1}, max={:.1})",
                        name, width, default_w, min_w, max_w
                    );
                }
            }
        }

        // Empty drawing
        {
            let drawing = PKDrawing::alloc();
            let drawing_id = IPKDrawing::init(&drawing);
            if !drawing_id.is_null() {
                let drawing = PKDrawing(drawing_id);
                let strokes = IPKDrawing::strokes(&drawing);
                let count = INSArray::<id>::count(&strokes);
                println!("\nNew PKDrawing: {} strokes", count);

                // Serialize to data
                let data = IPKDrawing::dataRepresentation(&drawing);
                if !data.0.is_null() {
                    let len = INSData::length(&data);
                    println!("  Serialized size: {} bytes", len);
                }
            }
        }
    }

    println!("\nDone.");
}
