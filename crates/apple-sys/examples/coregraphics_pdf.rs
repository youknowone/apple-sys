//! Generate a PDF file with shapes and text using CoreGraphics.
//!
//! Creates /tmp/apple_sys_demo.pdf with colored rectangles, a bar chart,
//! and styled text.

#![allow(non_upper_case_globals)]

use apple_sys::CoreFoundation::*;
use apple_sys::CoreGraphics::*;
use std::ptr;

const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
const kCFURLPOSIXPathStyle: CFURLPathStyle = 0;
const kCGEncodingMacRoman: CGTextEncoding = 0;

fn main() {
    let output_path = "/tmp/apple_sys_demo.pdf";
    println!("=== CoreGraphics PDF Generation Demo ===\n");
    println!("Output: {output_path}");

    unsafe {
        // Create URL for output file
        let path_str = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"/tmp/apple_sys_demo.pdf".as_ptr(),
            kCFStringEncodingUTF8,
        );
        let url = CFURLCreateWithFileSystemPath(
            kCFAllocatorDefault,
            path_str,
            kCFURLPOSIXPathStyle,
            0, // not a directory
        );
        CFRelease(path_str as CFTypeRef);

        // US Letter page: 612 x 792 points
        let page = CGRect {
            origin: CGPoint { x: 0.0, y: 0.0 },
            size: CGSize {
                width: 612.0,
                height: 792.0,
            },
        };

        let ctx = CGPDFContextCreateWithURL(url, &page, ptr::null());
        assert!(!ctx.is_null(), "Failed to create PDF context");
        CFRelease(url as CFTypeRef);

        // === Page 1 ===
        CGContextBeginPage(ctx, &page);

        // Light background
        CGContextSetRGBFillColor(ctx, 0.95, 0.95, 1.0, 1.0);
        CGContextFillRect(ctx, page);

        // Title
        CGContextSetRGBFillColor(ctx, 0.1, 0.1, 0.3, 1.0);
        CGContextSelectFont(ctx, c"Helvetica-Bold".as_ptr(), 28.0, kCGEncodingMacRoman);
        let title = b"apple-sys PDF Demo";
        CGContextShowTextAtPoint(ctx, 50.0, 720.0, title.as_ptr() as _, title.len());

        // Subtitle
        CGContextSetRGBFillColor(ctx, 0.4, 0.4, 0.5, 1.0);
        CGContextSelectFont(ctx, c"Helvetica".as_ptr(), 14.0, kCGEncodingMacRoman);
        let subtitle = b"Generated with CoreGraphics via apple-sys";
        CGContextShowTextAtPoint(ctx, 50.0, 695.0, subtitle.as_ptr() as _, subtitle.len());

        // Row of filled rectangles
        let colors: [(f64, f64, f64); 5] = [
            (0.90, 0.20, 0.20),
            (0.20, 0.70, 0.30),
            (0.20, 0.40, 0.90),
            (0.95, 0.70, 0.10),
            (0.60, 0.20, 0.80),
        ];
        for (i, (r, g, b)) in colors.iter().enumerate() {
            let x = 50.0 + (i as f64) * 110.0;
            CGContextSetRGBFillColor(ctx, *r, *g, *b, 0.85);
            CGContextFillRect(
                ctx,
                CGRect {
                    origin: CGPoint { x, y: 580.0 },
                    size: CGSize {
                        width: 90.0,
                        height: 90.0,
                    },
                },
            );
        }

        // Row of stroked rectangles
        CGContextSetLineWidth(ctx, 3.0);
        for (i, (r, g, b)) in colors.iter().enumerate() {
            let x = 50.0 + (i as f64) * 110.0;
            CGContextSetRGBStrokeColor(ctx, *r, *g, *b, 1.0);
            CGContextStrokeRect(
                ctx,
                CGRect {
                    origin: CGPoint { x, y: 460.0 },
                    size: CGSize {
                        width: 90.0,
                        height: 90.0,
                    },
                },
            );
        }

        // Bar chart
        CGContextSetRGBFillColor(ctx, 0.2, 0.2, 0.3, 1.0);
        CGContextSelectFont(ctx, c"Helvetica-Bold".as_ptr(), 16.0, kCGEncodingMacRoman);
        let heading = b"Framework Popularity";
        CGContextShowTextAtPoint(ctx, 50.0, 420.0, heading.as_ptr() as _, heading.len());

        let bars: [(&[u8], f64, (f64, f64, f64)); 5] = [
            (b"Foundation", 100.0, (0.2, 0.6, 0.9)),
            (b"AppKit", 85.0, (0.9, 0.3, 0.3)),
            (b"Metal", 70.0, (0.3, 0.8, 0.3)),
            (b"CoreML", 55.0, (0.9, 0.6, 0.1)),
            (b"WebKit", 45.0, (0.6, 0.3, 0.8)),
        ];

        CGContextSelectFont(ctx, c"Helvetica".as_ptr(), 11.0, kCGEncodingMacRoman);
        for (i, (label, pct, (r, g, b))) in bars.iter().enumerate() {
            let y = 380.0 - (i as f64) * 40.0;
            let bar_w = pct / 100.0 * 400.0;

            // Bar
            CGContextSetRGBFillColor(ctx, *r, *g, *b, 0.85);
            CGContextFillRect(
                ctx,
                CGRect {
                    origin: CGPoint { x: 140.0, y },
                    size: CGSize {
                        width: bar_w,
                        height: 25.0,
                    },
                },
            );

            // Label
            CGContextSetRGBFillColor(ctx, 0.2, 0.2, 0.2, 1.0);
            CGContextShowTextAtPoint(ctx, 50.0, y + 7.0, label.as_ptr() as _, label.len());
        }

        // Footer
        CGContextSetRGBFillColor(ctx, 0.5, 0.5, 0.5, 1.0);
        CGContextSelectFont(
            ctx,
            c"Helvetica-Oblique".as_ptr(),
            10.0,
            kCGEncodingMacRoman,
        );
        let footer = b"Created by apple-sys coregraphics_pdf example";
        CGContextShowTextAtPoint(ctx, 50.0, 30.0, footer.as_ptr() as _, footer.len());

        CGContextEndPage(ctx);
        CGContextRelease(ctx);

        println!("\nPDF created successfully!");
        println!("Open with: open {output_path}");
    }
}
