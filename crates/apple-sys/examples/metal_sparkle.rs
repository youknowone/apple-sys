//! Metal graphics demo: cosmic nebula with twinkling stars.
//!
//! Renders an animated starfield with nebula clouds, aurora waves,
//! shooting stars, and lens flares using a fullscreen fragment shader.

#![allow(non_upper_case_globals)]

use apple_sys::AddressBook::NSApplication_NSEvent;
use apple_sys::AppKit::*;
use apple_sys::CoreFoundation::{CGPoint, CGRect, CGSize, INSObject};
use apple_sys::Foundation::{NSAutoreleasePool, NSDate};
use apple_sys::Metal::*;
use apple_sys::QuartzCore::{CALayer, CAMetalLayer, ICAMetalLayer};
use apple_sys::objc::BOOL;

use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::msg_send;
use objc2::runtime::{AnyClass, AnyObject};
mod common;
use common::nsstring;

type NSEventMask = u64;
const NSApplicationActivationPolicyRegular: isize = 0;
const NSWindowStyleMaskTitled: usize = 1 << 0;
const NSWindowStyleMaskClosable: usize = 1 << 1;
const NSWindowStyleMaskMiniaturizable: usize = 1 << 2;
const NSWindowStyleMaskResizable: usize = 1 << 3;
const NSBackingStoreBuffered: usize = 2;

const MTLPixelFormatBGRA8Unorm: usize = 80;
const MTLPrimitiveTypeTriangle: usize = 3;
const MTLLoadActionClear: usize = 2;
const MTLStoreActionStore: usize = 1;

/// Wrapper for `void *` to satisfy objc2's Encode trait.
#[repr(transparent)]
#[derive(Copy, Clone)]
struct VoidPtr(*mut std::ffi::c_void);

unsafe impl Encode for VoidPtr {
    const ENCODING: Encoding = Encoding::Pointer(&Encoding::Void);
}
unsafe impl RefEncode for VoidPtr {
    const ENCODING_REF: Encoding = Encoding::Pointer(&<Self as Encode>::ENCODING);
}

fn rect(x: f64, y: f64, w: f64, h: f64) -> CGRect {
    CGRect {
        origin: CGPoint { x, y },
        size: CGSize {
            width: w,
            height: h,
        },
    }
}

const SHADER_SRC: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float2 uv;
};

vertex VertexOut vertex_main(uint vid [[vertex_id]]) {
    float2 pos = float2((vid << 1) & 2, vid & 2);
    VertexOut out;
    out.position = float4(pos * 2.0 - 1.0, 0.0, 1.0);
    out.uv = float2(pos.x, 1.0 - pos.y);
    return out;
}

float hash(float2 p) {
    p = fract(p * float2(234.34, 435.345));
    p += dot(p, p + 34.23);
    return fract(p.x * p.y);
}

float noise(float2 p) {
    float2 i = floor(p);
    float2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + float2(1, 0)), f.x),
               mix(hash(i + float2(0, 1)), hash(i + float2(1, 1)), f.x), f.y);
}

float fbm(float2 p) {
    float v = 0.0, a = 0.5;
    float2x2 rot = float2x2(float2(cos(0.5), sin(0.5)),
                             float2(-sin(0.5), cos(0.5)));
    for (int i = 0; i < 6; i++) {
        v += a * noise(p);
        p = rot * p * 2.0 + 0.5;
        a *= 0.5;
    }
    return v;
}

float star(float2 uv, float flare) {
    float d = length(uv);
    float m = 0.015 / d;
    float rays = max(0.0, 1.0 - abs(uv.x * uv.y * 1000.0));
    m += rays * flare;
    m *= smoothstep(1.0, 0.2, d);
    return m;
}

float3 starField(float2 uv, float t) {
    float3 col = 0.0;
    float2 gv = fract(uv) - 0.5;
    float2 id = floor(uv);

    for (int y = -1; y <= 1; y++) {
        for (int x = -1; x <= 1; x++) {
            float2 offs = float2(x, y);
            float n = hash(id + offs);
            float size = fract(n * 345.32);
            float twinkle = sin(t * (fract(n * 2345.2) * 6.0 + 0.5) + n * 6.283) * 0.5 + 0.5;
            twinkle = pow(twinkle, 4.0);

            float2 spos = gv - offs - float2(n - 0.5, fract(n * 34.0) - 0.5);
            float s = star(spos, smoothstep(0.85, 1.0, size) * twinkle);

            float3 c = sin(float3(0.2, 0.3, 0.9) * fract(n * 2345.2) * 123.2
                         + float3(0, 2, 4)) * 0.5 + 0.5;
            c = mix(c, float3(1.0), 0.6);

            col += s * size * (0.2 + twinkle * 0.8) * c;
        }
    }
    return col;
}

fragment float4 fragment_main(VertexOut in [[stage_in]],
                              constant float &time [[buffer(0)]]) {
    float2 uv = (in.uv - 0.5) * float2(1.5, 1.0);
    float t = time;

    // Deep space
    float3 col = float3(0.01, 0.0, 0.03);

    // Nebula clouds (fractal Brownian motion)
    float2 nuv = uv * 1.5;
    float n1 = fbm(nuv + float2(t * 0.03, t * 0.02));
    float n2 = fbm(nuv + float2(5.2, 1.3) + float2(-t * 0.02, t * 0.04));
    float n3 = fbm(nuv * 0.8 + float2(13.7, 7.1) + float2(t * 0.015, -t * 0.01));

    col += float3(0.5, 0.1, 0.7) * smoothstep(0.35, 0.75, n1) * 0.5;
    col += float3(0.1, 0.4, 0.8) * smoothstep(0.4, 0.8, n2) * 0.4;
    col += float3(0.8, 0.15, 0.35) * smoothstep(0.3, 0.7, n3) * 0.35;

    // Aurora waves
    float a1 = sin(uv.x * 2.5 + t * 0.7) * cos(uv.y * 3.5 + t * 0.5);
    float a2 = sin(uv.x * 3.5 - t * 0.4) * cos(uv.y * 2.0 + t * 0.9);
    float aurora = smoothstep(-0.1, 0.3, a1 * a2);
    float3 aCol = mix(float3(0.0, 0.9, 0.4), float3(0.3, 0.5, 1.0),
                       sin(t * 0.3 + uv.x * 2.0) * 0.5 + 0.5);
    col += aCol * aurora * 0.12;

    // Star layers with parallax
    for (float i = 0.0; i < 4.0; i += 1.0) {
        float depth = fract(i * 0.25 + t * 0.02);
        float scale = mix(30.0, 5.0, depth);
        float fade = depth * smoothstep(1.0, 0.85, depth);
        float2 drift = float2(t * 0.08 * (i + 1.0), t * 0.03 * sin(i));
        col += starField(uv * scale + i * 453.2 + drift, t) * fade;
    }

    // Big bright stars with lens flares
    for (float i = 0.0; i < 3.0; i += 1.0) {
        float2 bUv = uv * (1.5 + i * 0.7);
        float2 bId = floor(bUv);
        float2 bGv = fract(bUv) - 0.5;
        float bn = hash(bId + i * 99.0);
        if (bn > 0.92) {
            float2 bp = bGv - float2(fract(bn * 567.8) - 0.5,
                                      fract(bn * 123.4) - 0.5);
            float bFlare = sin(t * 1.3 + bn * 20.0) * 0.5 + 0.5;
            float bs = star(bp, bFlare * 0.6) * 2.5;

            float3 bCol = 0.5 + 0.5 * cos(float3(0.0, 2.0, 4.0) + bn * 6.283);
            bCol = mix(bCol, float3(1.0), 0.5);
            col += bs * bCol;

            // Lens flare cross
            float cross_val = exp(-abs(bp.x) * 15.0) * exp(-abs(bp.y) * 80.0)
                            + exp(-abs(bp.x) * 80.0) * exp(-abs(bp.y) * 15.0);
            col += cross_val * bCol * bFlare * 0.3;
        }
    }

    // Shooting star (occasional)
    float shootPhase = fract(t * 0.15);
    if (shootPhase < 0.2) {
        float shootT = shootPhase / 0.2;
        float shootIdx = floor(t * 0.15);
        float sn = hash(float2(shootIdx, 42.0));
        float2 shootStart = float2(sn * 2.0 - 1.0, 0.3 + fract(sn * 123.4) * 0.3);
        float2 shootDir = normalize(float2(-0.7, -0.5));
        float2 shootPos = shootStart + shootDir * shootT * 1.5;

        float2 d = uv - shootPos;
        float along = dot(d, shootDir);
        float perp = length(d - shootDir * along);

        float trail = smoothstep(0.0, -0.3, along) * smoothstep(-0.5, 0.0, along);
        float width = exp(-perp * 200.0);
        float shoot = trail * width
                    * smoothstep(0.0, 0.05, shootT) * smoothstep(1.0, 0.8, shootT);
        col += shoot * float3(0.8, 0.9, 1.0) * 3.0;
    }

    // Color grading + vignette
    col = pow(col, float3(0.95));
    col *= float3(1.05, 1.0, 0.98);
    float vig = 1.0 - dot(in.uv - 0.5, in.uv - 0.5) * 1.8;
    col *= smoothstep(0.0, 0.6, vig);

    return float4(saturate(col), 1.0);
}
"#;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        // --- NSApplication ---
        let app = NSApplication::sharedApplication();
        INSApplication::setActivationPolicy_(
            &app,
            NSApplicationActivationPolicyRegular as NSApplicationActivationPolicy,
        );

        // --- Metal device & command queue ---
        // Metal types (MTLDevice, MTLCommandQueue, MTLLibrary, etc.) are protocols
        // with no concrete struct; P-traits exist but have no implementors.
        let device = MTLCreateSystemDefaultDevice() as *mut AnyObject;
        assert!(!device.is_null(), "Metal is not supported");
        let queue: *mut AnyObject = msg_send![device, newCommandQueue];

        // --- Compile shaders ---
        let src = std::ffi::CString::new(SHADER_SRC).unwrap();
        let mut err: *mut AnyObject = std::ptr::null_mut();
        let library: *mut AnyObject = msg_send![device,
            newLibraryWithSource: nsstring(&src),
            options: std::ptr::null::<AnyObject>(),
            error: &mut err
        ];
        if library.is_null() {
            let desc: *mut AnyObject = msg_send![err, localizedDescription];
            let cstr: *const i8 = msg_send![desc, UTF8String];
            let msg = std::ffi::CStr::from_ptr(cstr).to_string_lossy();
            panic!("Shader compilation failed: {msg}");
        }

        let vert_fn: *mut AnyObject =
            msg_send![library, newFunctionWithName: nsstring(c"vertex_main")];
        let frag_fn: *mut AnyObject =
            msg_send![library, newFunctionWithName: nsstring(c"fragment_main")];

        // --- Render pipeline ---
        let desc = MTLRenderPipelineDescriptor::alloc();
        let desc = MTLRenderPipelineDescriptor(INSObject::init(&desc));
        IMTLRenderPipelineDescriptor::setVertexFunction_(&desc, vert_fn as _);
        IMTLRenderPipelineDescriptor::setFragmentFunction_(&desc, frag_fn as _);

        let attachments = IMTLRenderPipelineDescriptor::colorAttachments(&desc);
        let att0 = IMTLRenderPipelineColorAttachmentDescriptorArray::objectAtIndexedSubscript_(
            &attachments,
            0,
        );
        IMTLRenderPipelineColorAttachmentDescriptor::setPixelFormat_(
            &att0,
            MTLPixelFormatBGRA8Unorm as MTLPixelFormat,
        );

        let pipeline: *mut AnyObject = msg_send![device,
            newRenderPipelineStateWithDescriptor: &*desc,
            error: &mut err
        ];
        assert!(!pipeline.is_null(), "Pipeline creation failed");

        // --- Window ---
        let style = NSWindowStyleMaskTitled
            | NSWindowStyleMaskClosable
            | NSWindowStyleMaskMiniaturizable
            | NSWindowStyleMaskResizable;

        let window = NSWindow::alloc();
        let window = NSWindow(INSWindow::initWithContentRect_styleMask_backing_defer_(
            &window,
            rect(100.0, 100.0, 800.0, 600.0),
            style as NSWindowStyleMask,
            NSBackingStoreBuffered as NSBackingStoreType,
            BOOL(false),
        ));
        INSWindow::setTitle_(&window, nsstring(c"Metal Sparkle - apple-sys"));

        // --- CAMetalLayer ---
        // bindgen limitation: ICALayer::layer() hardcodes receiver to CALayer,
        // so CAMetalLayer::layer() would create a CALayer instead of CAMetalLayer.
        let layer_cls = AnyClass::get(c"CAMetalLayer").unwrap();
        let metal_layer: *mut AnyObject = msg_send![layer_cls, layer];
        let metal_layer = CAMetalLayer(metal_layer as _);
        ICAMetalLayer::setDevice_(&metal_layer, device as _);
        ICAMetalLayer::setPixelFormat_(&metal_layer, MTLPixelFormatBGRA8Unorm as MTLPixelFormat);
        ICAMetalLayer::setFramebufferOnly_(&metal_layer, BOOL(true));

        let content_view = INSWindow::contentView(&window);
        INSView::setWantsLayer_(&content_view, BOOL(true));
        INSView::setLayer_(&content_view, CALayer(metal_layer.0));

        // --- Time uniform buffer ---
        let time_buf: *mut AnyObject =
            msg_send![device, newBufferWithLength: 16usize, options: 0usize];

        // --- Show window ---
        INSWindow::makeKeyAndOrderFront_(&window, std::ptr::null_mut());
        INSApplication::activateIgnoringOtherApps_(&app, BOOL(true));

        // --- Render loop ---
        let start = std::time::Instant::now();
        let run_mode = nsstring(c"kCFRunLoopDefaultMode");

        loop {
            let loop_pool = NSAutoreleasePool::alloc();
            let _loop_pool = INSObject::init(&loop_pool);

            // Process all pending events
            loop {
                let event = <NSApplication as NSApplication_NSEvent>::nextEventMatchingMask_untilDate_inMode_dequeue_(
                    &app,
                    u64::MAX as NSEventMask,
                    NSDate(std::ptr::null_mut()),
                    run_mode,
                    BOOL(true),
                );
                if event.0.is_null() {
                    break;
                }
                <NSApplication as NSApplication_NSEvent>::sendEvent_(&app, event);
            }

            // Exit when window is closed
            if !INSWindow::isVisible(&window).0 {
                break;
            }

            // Update time uniform
            let time = start.elapsed().as_secs_f32();
            let contents: VoidPtr = msg_send![time_buf, contents];
            *(contents.0 as *mut f32) = time;

            // Get next drawable
            let drawable = ICAMetalLayer::nextDrawable(&metal_layer);
            if (drawable as *mut AnyObject).is_null() {
                std::thread::sleep(std::time::Duration::from_millis(1));
                continue;
            }

            // Render pass
            let pass_desc = MTLRenderPassDescriptor::renderPassDescriptor();
            let pass_atts = IMTLRenderPassDescriptor::colorAttachments(&pass_desc);
            let pass_att0 = IMTLRenderPassColorAttachmentDescriptorArray::objectAtIndexedSubscript_(
                &pass_atts, 0,
            );

            let texture: *mut AnyObject = msg_send![drawable as *mut AnyObject, texture];
            IMTLRenderPassAttachmentDescriptor::setTexture_(&pass_att0, texture as _);
            IMTLRenderPassAttachmentDescriptor::setLoadAction_(
                &pass_att0,
                MTLLoadActionClear as MTLLoadAction,
            );
            IMTLRenderPassAttachmentDescriptor::setStoreAction_(
                &pass_att0,
                MTLStoreActionStore as MTLStoreAction,
            );

            // Encode draw commands
            let cmd_buf: *mut AnyObject = msg_send![queue, commandBuffer];
            let encoder: *mut AnyObject =
                msg_send![cmd_buf, renderCommandEncoderWithDescriptor: &*pass_desc];

            let _: () = msg_send![encoder, setRenderPipelineState: pipeline];
            let _: () = msg_send![encoder,
                setFragmentBuffer: time_buf, offset: 0usize, atIndex: 0usize];
            let _: () = msg_send![encoder,
                drawPrimitives: MTLPrimitiveTypeTriangle,
                vertexStart: 0usize, vertexCount: 3usize];
            let _: () = msg_send![encoder, endEncoding];

            let _: () = msg_send![cmd_buf, presentDrawable: drawable as *mut AnyObject];
            let _: () = msg_send![cmd_buf, commit];

            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
