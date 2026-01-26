//! List available CoreImage filters by category.
//!
//! Uses CIFilter filterNamesInCategories: to enumerate
//! all image processing filters available on this system.

use apple_sys::AddressBook::CIFilter_CIFilterRegistry;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::CoreImage::*;
use apple_sys::Foundation::{
    INSArray, NSArray, NSArray_NSArrayCreation, NSAutoreleasePool, NSString,
};
use apple_sys::objc::id;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== CoreImage Filters ===\n");

        // Get all filter names
        let all_names = CIFilter::filterNamesInCategories_(NSArray(std::ptr::null_mut()));
        let total = INSArray::<id>::count(&all_names);
        println!("Total available CIFilters: {}\n", total);

        // List by category
        let categories = [
            (c"CICategoryBlur", "Blur"),
            (c"CICategoryColorAdjustment", "Color Adjustment"),
            (c"CICategoryColorEffect", "Color Effect"),
            (c"CICategoryDistortionEffect", "Distortion"),
            (c"CICategoryGenerator", "Generator"),
            (c"CICategoryGeometryAdjustment", "Geometry"),
            (c"CICategorySharpen", "Sharpen"),
            (c"CICategoryStylize", "Stylize"),
            (c"CICategoryTileEffect", "Tile Effect"),
            (c"CICategoryTransition", "Transition"),
        ];

        for (cat_id, cat_name) in &categories {
            let cat_str = nsstring(cat_id);
            let cat_arr = NSArray(<NSArray as NSArray_NSArrayCreation<id>>::arrayWithObject_(
                cat_str.0,
            ));
            let names = CIFilter::filterNamesInCategories_(cat_arr);
            let count = INSArray::<id>::count(&names);

            println!("{} ({} filters):", cat_name, count);
            let show = if count > 5 { 5 } else { count };
            for i in 0..show {
                let name: id = INSArray::<id>::objectAtIndex_(&names, i);
                println!("  - {}", nsstring_to_string(NSString(name)));
            }
            if count > 5 {
                println!("  ... and {} more", count - 5);
            }
            println!();
        }
    }

    println!("Done.");
}
