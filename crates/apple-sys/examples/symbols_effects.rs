//! Verify Symbols framework bindings by creating various NSSymbolEffect
//! instances and querying their properties.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::NSAutoreleasePool;
use apple_sys::Symbols::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Symbols Effects ===\n");

        // NSSymbolBounceEffect
        println!("--- NSSymbolBounceEffect ---");
        let effect_ptr = NSSymbolBounceEffect::effect();
        if !effect_ptr.is_null() {
            println!("  effect: {}", nsobj_to_string(effect_ptr));

            let down_ptr = NSSymbolBounceEffect::bounceDownEffect();
            println!("  bounceDownEffect: {}", nsobj_to_string(down_ptr));

            let up_ptr = NSSymbolBounceEffect::bounceUpEffect();
            println!("  bounceUpEffect: {}", nsobj_to_string(up_ptr));

            // Test by-layer and whole-symbol variants
            let bounce = NSSymbolBounceEffect(effect_ptr);
            let by_layer = INSSymbolBounceEffect::effectWithByLayer(&bounce);
            println!("  effectWithByLayer: {}", nsobj_to_string(by_layer));

            let whole = INSSymbolBounceEffect::effectWithWholeSymbol(&bounce);
            println!("  effectWithWholeSymbol: {}", nsobj_to_string(whole));
        }

        // NSSymbolPulseEffect
        println!("\n--- NSSymbolPulseEffect ---");
        let effect_ptr = NSSymbolPulseEffect::effect();
        if !effect_ptr.is_null() {
            let effect = NSSymbolPulseEffect(effect_ptr);
            println!("  effect: {}", nsobj_to_string(effect_ptr));

            let by_layer = INSSymbolPulseEffect::effectWithByLayer(&effect);
            println!("  effectWithByLayer: {}", nsobj_to_string(by_layer));

            let whole = INSSymbolPulseEffect::effectWithWholeSymbol(&effect);
            println!("  effectWithWholeSymbol: {}", nsobj_to_string(whole));
        }

        // NSSymbolScaleEffect
        println!("\n--- NSSymbolScaleEffect ---");
        let effect_ptr = NSSymbolScaleEffect::effect();
        if !effect_ptr.is_null() {
            println!("  effect: {}", nsobj_to_string(effect_ptr));

            let up = NSSymbolScaleEffect::scaleUpEffect();
            println!("  scaleUpEffect: {}", nsobj_to_string(up));

            let down = NSSymbolScaleEffect::scaleDownEffect();
            println!("  scaleDownEffect: {}", nsobj_to_string(down));
        }

        // NSSymbolAppearEffect
        println!("\n--- NSSymbolAppearEffect ---");
        let effect_ptr = NSSymbolAppearEffect::effect();
        if !effect_ptr.is_null() {
            println!("  effect: {}", nsobj_to_string(effect_ptr));

            let up = NSSymbolAppearEffect::appearUpEffect();
            println!("  appearUpEffect: {}", nsobj_to_string(up));

            let down = NSSymbolAppearEffect::appearDownEffect();
            println!("  appearDownEffect: {}", nsobj_to_string(down));
        }

        // NSSymbolDisappearEffect
        println!("\n--- NSSymbolDisappearEffect ---");
        let effect_ptr = NSSymbolDisappearEffect::effect();
        if !effect_ptr.is_null() {
            println!("  effect: {}", nsobj_to_string(effect_ptr));

            let up = NSSymbolDisappearEffect::disappearUpEffect();
            println!("  disappearUpEffect: {}", nsobj_to_string(up));

            let down = NSSymbolDisappearEffect::disappearDownEffect();
            println!("  disappearDownEffect: {}", nsobj_to_string(down));
        }

        // NSSymbolWiggleEffect
        println!("\n--- NSSymbolWiggleEffect ---");
        let effect_ptr = NSSymbolWiggleEffect::effect();
        if !effect_ptr.is_null() {
            println!("  effect: {}", nsobj_to_string(effect_ptr));

            let cw = NSSymbolWiggleEffect::wiggleClockwiseEffect();
            println!("  wiggleClockwiseEffect: {}", nsobj_to_string(cw));

            let ccw = NSSymbolWiggleEffect::wiggleCounterClockwiseEffect();
            println!("  wiggleCounterClockwiseEffect: {}", nsobj_to_string(ccw));

            let custom = NSSymbolWiggleEffect::wiggleCustomAngleEffect_(45.0);
            println!("  wiggleCustomAngleEffect(45): {}", nsobj_to_string(custom));
        }

        // NSSymbolRotateEffect
        println!("\n--- NSSymbolRotateEffect ---");
        let effect_ptr = NSSymbolRotateEffect::effect();
        if !effect_ptr.is_null() {
            println!("  effect: {}", nsobj_to_string(effect_ptr));

            let cw = NSSymbolRotateEffect::rotateClockwiseEffect();
            println!("  rotateClockwiseEffect: {}", nsobj_to_string(cw));

            let ccw = NSSymbolRotateEffect::rotateCounterClockwiseEffect();
            println!("  rotateCounterClockwiseEffect: {}", nsobj_to_string(ccw));
        }

        // NSSymbolBreatheEffect
        println!("\n--- NSSymbolBreatheEffect ---");
        let effect_ptr = NSSymbolBreatheEffect::effect();
        if !effect_ptr.is_null() {
            println!("  effect: {}", nsobj_to_string(effect_ptr));

            let pulse = NSSymbolBreatheEffect::breathePulseEffect();
            println!("  breathePulseEffect: {}", nsobj_to_string(pulse));

            let plain = NSSymbolBreatheEffect::breathePlainEffect();
            println!("  breathePlainEffect: {}", nsobj_to_string(plain));
        }

        // NSSymbolVariableColorEffect
        println!("\n--- NSSymbolVariableColorEffect ---");
        let effect_ptr = NSSymbolVariableColorEffect::effect();
        if !effect_ptr.is_null() {
            let effect = NSSymbolVariableColorEffect(effect_ptr);
            println!("  effect: {}", nsobj_to_string(effect_ptr));

            let iterative = INSSymbolVariableColorEffect::effectWithIterative(&effect);
            println!("  effectWithIterative: {}", nsobj_to_string(iterative));

            let cumulative = INSSymbolVariableColorEffect::effectWithCumulative(&effect);
            println!("  effectWithCumulative: {}", nsobj_to_string(cumulative));
        }

        // NSSymbolEffectOptions
        println!("\n--- NSSymbolEffectOptions ---");
        let opts = NSSymbolEffectOptions::options();
        if !opts.is_null() {
            let opts = NSSymbolEffectOptions(opts);
            println!("  options: {}", nsobj_to_string(opts.0));

            let repeating = INSSymbolEffectOptions::optionsWithRepeating(&opts);
            println!("  optionsWithRepeating: {}", nsobj_to_string(repeating));

            let non_repeating = INSSymbolEffectOptions::optionsWithNonRepeating(&opts);
            println!(
                "  optionsWithNonRepeating: {}",
                nsobj_to_string(non_repeating)
            );

            let count_3 = INSSymbolEffectOptions::optionsWithRepeatCount_(&opts, 3);
            println!("  optionsWithRepeatCount(3): {}", nsobj_to_string(count_3));

            let speed = INSSymbolEffectOptions::optionsWithSpeed_(&opts, 2.0);
            println!("  optionsWithSpeed(2.0): {}", nsobj_to_string(speed));
        }
    }

    println!("\nDone.");
}
