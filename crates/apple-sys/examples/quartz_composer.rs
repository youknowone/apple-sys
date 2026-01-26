//! Query QCCompositionRepository for available compositions via generated bindings.
//!
//! Gets the shared QCCompositionRepository and queries allCompositions
//! to list available Quartz Composer compositions.

use apple_sys::Quartz::*;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Quartz Composer ===\n");

        // QCCompositionRepository
        println!("--- QCCompositionRepository ---");
        {
            let repo = QCCompositionRepository::sharedCompositionRepository();
            if !repo.0.is_null() {
                println!("  Repository: {}", common::nsobj_to_string(repo.0));

                // Get all compositions
                let compositions = IQCCompositionRepository::allCompositions(&repo);
                if !compositions.0.is_null() {
                    let count = INSArray::<id>::count(&compositions);
                    println!("  Total compositions: {}", count);

                    // Show first few using QCComposition trait methods
                    let show_count = count.min(10);
                    for i in 0..show_count {
                        let comp_id = INSArray::<id>::objectAtIndex_(&compositions, i);
                        let comp = QCComposition(comp_id);

                        let identifier = QCComposition_QCCompositionRepository::identifier(&comp);
                        let id_str = nsstring_to_string(identifier);

                        // Use attributes() dictionary to get the name
                        let attrs = IQCComposition::attributes(&comp);
                        let name_key = nsstring(c"QCCompositionAttributeNameKey");
                        let name = INSDictionary::<(), ()>::objectForKey_(&attrs, name_key.0);
                        let name_str = nsstring_to_string(NSString(name));

                        let display = if name_str.is_empty() {
                            id_str.clone()
                        } else {
                            format!("{} ({})", name_str, id_str)
                        };

                        println!("    [{}] {}", i, display);

                        // Show protocols using the trait method
                        let protocols = IQCComposition::protocols(&comp);
                        if !protocols.0.is_null() {
                            let proto_count = INSArray::<id>::count(&protocols);
                            if proto_count > 0 {
                                print!("         protocols: ");
                                for j in 0..proto_count {
                                    let proto = INSArray::<id>::objectAtIndex_(&protocols, j);
                                    if j > 0 {
                                        print!(", ");
                                    }
                                    print!("{}", nsstring_to_string(NSString(proto)));
                                }
                                println!();
                            }
                        }
                    }

                    if count > show_count {
                        println!("    ... and {} more compositions", count - show_count);
                    }
                } else {
                    println!("  No compositions found");
                }

                // Try to find compositions with specific protocol
                let protocol_key = nsstring(c"/desktop");
                let protocols = NSArray(
                    <NSArray as NSArray_NSArrayCreation<()>>::arrayWithObject_(protocol_key.0),
                );

                let filtered = IQCCompositionRepository::compositionsWithProtocols_andAttributes_(
                    &repo,
                    protocols,
                    NSDictionary(std::ptr::null_mut()),
                );

                if !filtered.0.is_null() {
                    let fcount = INSArray::<id>::count(&filtered);
                    println!("\n  Desktop compositions (/desktop protocol): {}", fcount);
                }
            } else {
                println!("  sharedCompositionRepository returned nil");
            }
        }
    }

    println!("\nDone.");
}
