//! Demonstrate CoreMedia time arithmetic.
//!
//! CoreMedia's CMTime is the fundamental time representation for
//! audio/video processing on Apple platforms. This example shows
//! time creation, arithmetic, comparison, and conversion.

use apple_sys::CoreMedia::*;

/// Extract value and timescale from a packed CMTime struct.
fn vt(t: CMTime) -> (i64, i32) {
    let v = { t.value };
    let ts = { t.timescale };
    (v, ts)
}

fn main() {
    unsafe {
        println!("=== CoreMedia Time Arithmetic ===\n");

        // 1. Create times with different timescales
        let t1 = CMTimeMake(3, 1); // 3 seconds (3/1)
        let t2 = CMTimeMake(1500, 1000); // 1.5 seconds (1500/1000)
        let t3 = CMTimeMake(30, 24); // 1.25 seconds at 24fps (30/24 frames)

        let (v, ts) = vt(t1);
        println!("t1 = {}/{} = {:.3}s", v, ts, CMTimeGetSeconds(t1));
        let (v, ts) = vt(t2);
        println!("t2 = {}/{} = {:.3}s", v, ts, CMTimeGetSeconds(t2));
        let (v, ts) = vt(t3);
        println!(
            "t3 = {}/{} = {:.3}s (30 frames at 24fps)",
            v,
            ts,
            CMTimeGetSeconds(t3)
        );

        // 2. Addition
        let sum = CMTimeAdd(t1, t2);
        let (v, ts) = vt(sum);
        println!("\nt1 + t2 = {}/{} = {:.3}s", v, ts, CMTimeGetSeconds(sum));

        // 3. Subtraction
        let diff = CMTimeSubtract(t1, t2);
        let (v, ts) = vt(diff);
        println!("t1 - t2 = {}/{} = {:.3}s", v, ts, CMTimeGetSeconds(diff));

        // 4. Comparison
        let cmp = CMTimeCompare(t1, t2);
        let cmp_str = match cmp {
            x if x < 0 => "less than",
            0 => "equal to",
            _ => "greater than",
        };
        println!("\nt1 is {} t2", cmp_str);

        // 5. Create from seconds
        let t4 = CMTimeMakeWithSeconds(2.5, 600); // 2.5s with timescale 600
        let (v, ts) = vt(t4);
        println!(
            "\nFrom 2.5s (timescale 600): {}/{} = {:.3}s",
            v,
            ts,
            CMTimeGetSeconds(t4)
        );

        // 6. Multiply and convert timescale
        let doubled = CMTimeMultiply(t2, 2);
        let (v, ts) = vt(doubled);
        println!("t2 * 2 = {}/{} = {:.3}s", v, ts, CMTimeGetSeconds(doubled));

        let converted = CMTimeConvertScale(t3, 600, 1); // Convert 24fps to 600 timescale
        let (v, ts) = vt(converted);
        println!(
            "t3 at timescale 600: {}/{} = {:.3}s",
            v,
            ts,
            CMTimeGetSeconds(converted)
        );

        // 7. Special values
        let zero = kCMTimeZero;
        let invalid = kCMTimeInvalid;
        let zero_flags = { zero.flags };
        let invalid_flags = { invalid.flags };
        println!(
            "\nkCMTimeZero: {:.3}s (valid={})",
            CMTimeGetSeconds(zero),
            (zero_flags & (1 << 0)) != 0
        );
        println!("kCMTimeInvalid valid={}", (invalid_flags & (1 << 0)) != 0);

        // 8. Time range
        let range = CMTimeRangeMake(t1, t2);
        let duration_s = CMTimeGetSeconds(range.duration);
        let start_s = CMTimeGetSeconds(range.start);
        println!(
            "\nTime range: start={:.3}s duration={:.3}s",
            start_s, duration_s
        );
    }

    println!("\nDone.");
}
