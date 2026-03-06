//! Demonstrate QuartzCore 3D transform arithmetic and timing.
//!
//! Uses CATransform3D functions for matrix math and CACurrentMediaTime
//! for high-precision timing.

use apple_sys::QuartzCore::*;
use std::f64::consts::PI;

fn main() {
    unsafe {
        println!("=== QuartzCore Transform & Timing ===\n");

        // 1. Current media time
        let t0 = CACurrentMediaTime();
        println!("Current media time: {:.6}s", t0);

        // 2. Identity check
        let ident = CATransform3DIdentity;
        println!(
            "CATransform3DIdentity is identity: {}",
            CATransform3DIsIdentity(ident)
        );

        // 3. Translation
        let translate = CATransform3DMakeTranslation(10.0, 20.0, 30.0);
        println!(
            "\nTranslation(10,20,30): m41={}, m42={}, m43={}",
            translate.m41, translate.m42, translate.m43
        );
        println!("  is identity: {}", CATransform3DIsIdentity(translate));

        // 4. Scale
        let scale = CATransform3DMakeScale(2.0, 3.0, 4.0);
        println!(
            "Scale(2,3,4): m11={}, m22={}, m33={}",
            scale.m11, scale.m22, scale.m33
        );

        // 5. Rotation (90 degrees around Z axis)
        let rotate = CATransform3DMakeRotation(PI / 2.0, 0.0, 0.0, 1.0);
        println!(
            "Rotate(90° Z): m11={:.3}, m12={:.3}, m21={:.3}, m22={:.3}",
            rotate.m11, rotate.m12, rotate.m21, rotate.m22
        );

        // 6. Concatenation
        let combined = CATransform3DConcat(translate, scale);
        println!(
            "\nTranslate * Scale: m41={}, m42={}, m43={}",
            combined.m41, combined.m42, combined.m43
        );
        println!(
            "  diag: m11={}, m22={}, m33={}",
            combined.m11, combined.m22, combined.m33
        );

        // 7. Inversion
        let inv = CATransform3DInvert(scale);
        println!(
            "Inverse(Scale): m11={:.3}, m22={:.3}, m33={:.3}",
            inv.m11, inv.m22, inv.m33
        );
        let product = CATransform3DConcat(scale, inv);
        println!(
            "Scale * Inv(Scale) is identity: {}",
            CATransform3DIsIdentity(product)
        );

        // 8. Affine check
        let is_affine = CATransform3DIsAffine(translate);
        println!("\nTranslation is affine: {}", is_affine);
        if is_affine {
            let affine = CATransform3DGetAffineTransform(translate);
            println!("  CGAffineTransform tx={}, ty={}", affine.tx, affine.ty);
        }

        // 9. Frame rate range
        let range = CAFrameRateRangeMake(30.0, 120.0, 60.0);
        println!(
            "\nFrame rate range: min={}, max={}, preferred={}",
            range.minimum, range.maximum, range.preferred
        );
        let same = CAFrameRateRangeIsEqualToRange(range, range);
        println!("Range equals itself: {}", same);

        // 10. Timing
        let t1 = CACurrentMediaTime();
        println!("\nElapsed: {:.6}s", t1 - t0);
    }

    println!("\nDone.");
}
