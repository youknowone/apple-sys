//! Create MapKit coordinate regions and annotations.
//!
//! Demonstrates CLLocationCoordinate2D, MKCoordinateRegion,
//! and MKPointAnnotation for map-based data.

use apple_sys::CoreData::CLLocationCoordinate2D;
use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::MapKit::*;
use std::ffi::CStr;
mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MapKit Regions & Annotations ===\n");

        // Define some famous locations
        let locations = [
            ("Apple Park", 37.3349, -122.0090),
            ("Eiffel Tower", 48.8584, 2.2945),
            ("Tokyo Tower", 35.6586, 139.7454),
            ("Sydney Opera House", -33.8568, 151.2153),
            ("Machu Picchu", -13.1631, -72.5450),
        ];

        // Create annotations
        for (name, lat, lon) in &locations {
            let ann = MKPointAnnotation::alloc();
            let ann = MKPointAnnotation(INSObject::init(&ann));

            let coord = CLLocationCoordinate2D {
                latitude: *lat,
                longitude: *lon,
            };
            IMKPointAnnotation::setCoordinate_(&ann, coord);

            let ns_title =
                nsstring(CStr::from_bytes_with_nul(&[name.as_bytes(), &[0]].concat()).unwrap());
            IMKShape::setTitle_(&ann, ns_title);

            // Read back
            let got_coord = IMKPointAnnotation::coordinate(&ann);
            let got_title = PMKAnnotation::title(&ann);
            println!(
                "  {} ({:.4}, {:.4})",
                nsstring_to_string(got_title),
                got_coord.latitude,
                got_coord.longitude
            );
        }

        // Create a region spanning Apple Park area
        println!("\nCoordinate regions:");
        let center = CLLocationCoordinate2D {
            latitude: 37.3349,
            longitude: -122.0090,
        };

        // 1km span
        let span = MKCoordinateSpan {
            latitudeDelta: 0.01,
            longitudeDelta: 0.01,
        };
        let region = MKCoordinateRegion { center, span };
        println!(
            "  Apple Park (1km): center=({:.4}, {:.4}) span=({:.4}, {:.4})",
            region.center.latitude,
            region.center.longitude,
            region.span.latitudeDelta,
            region.span.longitudeDelta
        );

        // MKMapPoint conversions
        let map_point = MKMapPointForCoordinate(center);
        println!("  Map point: ({:.1}, {:.1})", map_point.x, map_point.y);

        let meters_per_point = MKMetersPerMapPointAtLatitude(center.latitude);
        println!(
            "  Meters per map point at lat {:.4}: {:.6}",
            center.latitude, meters_per_point
        );

        let distance = MKMetersBetweenMapPoints(
            MKMapPointForCoordinate(CLLocationCoordinate2D {
                latitude: 37.3349,
                longitude: -122.0090,
            }),
            MKMapPointForCoordinate(CLLocationCoordinate2D {
                latitude: 48.8584,
                longitude: 2.2945,
            }),
        );
        println!(
            "\n  Distance Apple Park -> Eiffel Tower: {:.0} km",
            distance / 1000.0
        );
    }

    println!("\nDone.");
}
