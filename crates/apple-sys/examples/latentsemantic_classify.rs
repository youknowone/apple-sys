//! Text classification with LatentSemanticMapping.
//!
//! Creates an LSM map, trains it with sample text categories,
//! then classifies new text against the trained categories.

use apple_sys::CoreFoundation::{
    CFRelease, CFStringCreateWithCString, CFStringEncoding, CFStringRef, kCFAllocatorDefault,
};
use apple_sys::LatentSemanticMapping::*;
use std::ffi::CString;

const UTF8_ENCODING: CFStringEncoding = 0x08000100;

unsafe fn make_cfstring(s: &str) -> CFStringRef {
    let c = CString::new(s).unwrap();
    unsafe { CFStringCreateWithCString(kCFAllocatorDefault, c.as_ptr(), UTF8_ENCODING) }
}

unsafe fn add_training_text(map: LSMMapRef, category: LSMCategory, words: &[&str]) {
    let text = unsafe { LSMTextCreate(kCFAllocatorDefault, map) };
    assert!(!text.is_null(), "LSMTextCreate failed");
    for &word in words {
        let cf = unsafe { make_cfstring(word) };
        unsafe { LSMTextAddWord(text, cf) };
        unsafe { CFRelease(cf as _) };
    }
    let status = unsafe { LSMMapAddText(map, text, category) };
    assert_eq!(status, 0, "LSMMapAddText failed: {}", status);
    unsafe { CFRelease(text as _) };
}

fn main() {
    unsafe {
        println!("=== Latent Semantic Mapping ===\n");

        // Create LSM map
        let map = LSMMapCreate(kCFAllocatorDefault, 0);
        assert!(!map.is_null(), "LSMMapCreate failed");
        println!("Created LSM map (TypeID: {})", LSMMapGetTypeID());

        LSMMapStartTraining(map);

        // Add categories
        let sports = LSMMapAddCategory(map);
        let tech = LSMMapAddCategory(map);
        println!("Categories: sports={}, tech={}", sports, tech);

        // Train with sample data
        add_training_text(
            map,
            sports,
            &[
                "football",
                "soccer",
                "basketball",
                "tennis",
                "athlete",
                "game",
                "score",
                "team",
                "championship",
                "player",
            ],
        );
        add_training_text(
            map,
            sports,
            &[
                "baseball", "hockey", "golf", "swimming", "marathon", "stadium", "coach",
                "training", "medal", "race",
            ],
        );
        add_training_text(
            map,
            tech,
            &[
                "computer",
                "software",
                "programming",
                "algorithm",
                "database",
                "network",
                "server",
                "code",
                "developer",
                "cloud",
            ],
        );
        add_training_text(
            map,
            tech,
            &[
                "processor",
                "memory",
                "storage",
                "compiler",
                "binary",
                "internet",
                "protocol",
                "encryption",
                "machine",
                "learning",
            ],
        );
        println!("Training data added");

        // Compile the map
        let status = LSMMapCompile(map);
        assert_eq!(status, 0, "LSMMapCompile failed: {}", status);
        println!("Map compiled\n");

        // Classify test inputs
        let tests = [
            &["quarterback", "touchdown", "stadium", "fans"][..],
            &["python", "compiler", "algorithm", "code"][..],
            &["running", "marathon", "athlete"][..],
            &["server", "database", "network"][..],
            &["ball", "game"][..],
        ];

        for test_words in &tests {
            let text = LSMTextCreate(kCFAllocatorDefault, map);
            for &w in *test_words {
                let cf = make_cfstring(w);
                LSMTextAddWord(text, cf);
                CFRelease(cf as _);
            }

            let result = LSMResultCreate(kCFAllocatorDefault, map, text, 2, 0);
            if result.is_null() {
                println!("  {:?} -> no result", test_words);
                CFRelease(text as _);
                continue;
            }

            let count = LSMResultGetCount(result);
            print!("  {:?} -> ", test_words);
            for i in 0..count {
                let cat = LSMResultGetCategory(result, i);
                let score = LSMResultGetScore(result, i);
                let label = if cat == sports {
                    "sports"
                } else if cat == tech {
                    "tech"
                } else {
                    "unknown"
                };
                if i > 0 {
                    print!(", ");
                }
                print!("{}({:.3})", label, score);
            }
            println!();

            CFRelease(result as _);
            CFRelease(text as _);
        }

        CFRelease(map as _);
    }

    println!("\nDone.");
}
