//! Generate random numbers using GameplayKit.
//!
//! Uses GKRandomSource, GKARC4RandomSource, and GKMersenneTwisterRandomSource
//! to produce random integers and demonstrate shuffling.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{
    INSArray, INSMutableArray, INSNumber, NSArray, NSAutoreleasePool, NSMutableArray, NSNumber,
    NSNumber_NSNumberCreation,
};
use apple_sys::GameplayKit::*;
use apple_sys::objc::id;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== GameplayKit Random ===\n");

        // GKRandomSource (system default)
        let src = GKRandomSource::alloc();
        let src_ptr = INSObject::init(&src);
        let src = GKRandomSource(src_ptr);
        print!("GKRandomSource:       ");
        for _ in 0..8 {
            let n = <GKRandomSource as PGKRandom>::nextInt(&src);
            print!("{:12} ", n);
        }
        println!();

        // GKARC4RandomSource
        let src = GKARC4RandomSource::alloc();
        let src_ptr = INSObject::init(&src);
        let src = GKARC4RandomSource(src_ptr);
        print!("GKARC4RandomSource:   ");
        for _ in 0..8 {
            let n = <GKARC4RandomSource as PGKRandom>::nextInt(&src);
            print!("{:12} ", n);
        }
        println!();

        // GKMersenneTwisterRandomSource
        let src = GKMersenneTwisterRandomSource::alloc();
        let src_ptr = INSObject::init(&src);
        let src = GKMersenneTwisterRandomSource(src_ptr);
        print!("GKMersenneTwister:    ");
        for _ in 0..8 {
            let n = <GKMersenneTwisterRandomSource as PGKRandom>::nextInt(&src);
            print!("{:12} ", n);
        }
        println!();

        // nextIntWithUpperBound: for dice rolls
        let src = GKRandomSource::alloc();
        let src_ptr = INSObject::init(&src);
        let src = GKRandomSource(src_ptr);
        print!("\nDice rolls (d6):      ");
        for _ in 0..20 {
            let n = <GKRandomSource as PGKRandom>::nextIntWithUpperBound_(&src, 6);
            print!("{} ", n + 1);
        }
        println!();

        // nextUniform: for float [0, 1)
        print!("Uniform floats:       ");
        for _ in 0..6 {
            let f = <GKRandomSource as PGKRandom>::nextUniform(&src);
            print!("{:.4} ", f);
        }
        println!();

        // Shuffle an NSArray
        let arr = NSMutableArray::alloc();
        let arr = NSMutableArray(INSObject::init(&arr));
        for i in 1..=10i64 {
            let num = NSNumber::numberWithLongLong_(i);
            INSMutableArray::<id>::addObject_(&arr, num.0);
        }

        let src = GKRandomSource::alloc();
        let src_ptr = INSObject::init(&src);
        let src = GKRandomSource(src_ptr);
        let shuffled = IGKRandomSource::arrayByShufflingObjectsInArray_(&src, NSArray(arr.0));
        let count = INSArray::<id>::count(&shuffled);
        print!("\nShuffled [1..10]:      ");
        for i in 0..count {
            let obj = INSArray::<id>::objectAtIndex_(&shuffled, i);
            let n = INSNumber::longLongValue(&NSNumber(obj));
            print!("{} ", n);
        }
        println!();
    }

    println!("\nDone.");
}
