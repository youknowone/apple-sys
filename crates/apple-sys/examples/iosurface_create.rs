//! Create and inspect an IOSurface.
//!
//! Uses IOSurface ObjC API to create an in-memory surface,
//! then queries its dimensions, format, and allocation size.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{
    INSMutableDictionary, NSAutoreleasePool, NSDictionary, NSMutableDictionary, NSNumber,
    NSNumber_NSNumberCreation,
};
use apple_sys::IOSurface::*;
use apple_sys::objc::id;

mod common;
use common::nsstring;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== IOSurface Creation ===\n");

        // Property limits
        println!("IOSurface type ID: {}", IOSurfaceGetTypeID());

        let max_width = IOSurfaceGetPropertyMaximum(kIOSurfaceWidth);
        let max_height = IOSurfaceGetPropertyMaximum(kIOSurfaceHeight);
        println!("Max width:  {}", max_width);
        println!("Max height: {}", max_height);

        let align = IOSurfaceGetPropertyAlignment(kIOSurfaceBytesPerRow);
        println!("BytesPerRow alignment: {}", align);

        // Create a surface via ObjC
        let width: usize = 320;
        let height: usize = 240;
        let bytes_per_element: usize = 4;
        let bytes_per_row: usize = width * bytes_per_element;

        let dict = NSMutableDictionary::alloc();
        let dict = NSMutableDictionary(INSObject::init(&dict));

        let nsnumber_usize =
            |val: usize| -> id { NSNumber::numberWithUnsignedInteger_(val as u64).0 };

        INSMutableDictionary::<(), ()>::setObject_forKey_(
            &dict,
            nsnumber_usize(width),
            nsstring(c"IOSurfaceWidth").0 as *mut u64,
        );
        INSMutableDictionary::<(), ()>::setObject_forKey_(
            &dict,
            nsnumber_usize(height),
            nsstring(c"IOSurfaceHeight").0 as *mut u64,
        );
        INSMutableDictionary::<(), ()>::setObject_forKey_(
            &dict,
            nsnumber_usize(bytes_per_element),
            nsstring(c"IOSurfaceBytesPerElement").0 as *mut u64,
        );
        INSMutableDictionary::<(), ()>::setObject_forKey_(
            &dict,
            nsnumber_usize(bytes_per_row),
            nsstring(c"IOSurfaceBytesPerRow").0 as *mut u64,
        );

        // pixel format 'BGRA' = 0x42475241
        let pixel_format: usize = 0x42475241;
        INSMutableDictionary::<(), ()>::setObject_forKey_(
            &dict,
            nsnumber_usize(pixel_format),
            nsstring(c"IOSurfacePixelFormat").0 as *mut u64,
        );

        let surf = IOSurface::alloc();
        let surf_ptr = IIOSurface::initWithProperties_(&surf, NSDictionary(dict.0));
        let surf = IOSurface(surf_ptr);

        if !surf.0.is_null() {
            let w = IIOSurface::width(&surf);
            let h = IIOSurface::height(&surf);
            let bpr = IIOSurface::bytesPerRow(&surf);
            let bpe = IIOSurface::bytesPerElement(&surf);
            let alloc_size = IIOSurface::allocationSize(&surf);
            let sid = IIOSurface::surfaceID(&surf);
            let seed = IIOSurface::seed(&surf);
            let pf = IIOSurface::pixelFormat(&surf);

            println!("\nCreated IOSurface:");
            println!("  Surface ID:      {}", sid);
            println!("  Dimensions:      {}x{}", w, h);
            println!("  Bytes/row:       {}", bpr);
            println!("  Bytes/element:   {}", bpe);
            println!("  Allocation size: {} bytes", alloc_size);
            println!("  Pixel format:    0x{:08X}", pf);
            println!("  Seed:            {}", seed);
        } else {
            println!("Failed to create IOSurface");
        }
    }

    println!("\nDone.");
}
