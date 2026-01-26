//! Per-framework integration tests.
//!
//! Each module is cfg-gated so only the framework under test compiles.
//! Run with: cargo test --features FrameworkName

// ==================== CoreFoundation ====================
#[cfg(all(target_os = "macos", feature = "CoreFoundation"))]
mod core_foundation {
    use apple_sys::CoreFoundation::*;

    // kCFStringEncodingUTF8 is a C #define macro, not exported by bindgen.
    const CF_STRING_ENCODING_UTF8: CFStringEncoding = 0x08000100;
    // kCFNumberSInt64Type = 4
    const CF_NUMBER_SINT64_TYPE: CFNumberType = 4;

    /// CFString round-trip: create from C string, check length, release.
    #[test]
    fn cfstring_create_and_get_length() {
        unsafe {
            let cstr = b"Hello, CoreFoundation!\0".as_ptr() as *const i8;
            let s = CFStringCreateWithCString(std::ptr::null(), cstr, CF_STRING_ENCODING_UTF8);
            assert!(!s.is_null());
            let len = CFStringGetLength(s);
            assert_eq!(len, 22);
            CFRelease(s as CFTypeRef);
        }
    }

    /// CFArray: create empty mutable array, append values, check count.
    #[test]
    fn cfarray_mutable_append_and_count() {
        unsafe {
            let arr = CFArrayCreateMutable(std::ptr::null(), 0, &kCFTypeArrayCallBacks);
            assert!(!arr.is_null());
            assert_eq!(CFArrayGetCount(arr as CFArrayRef), 0);

            let cstr = b"item\0".as_ptr() as *const i8;
            let val = CFStringCreateWithCString(std::ptr::null(), cstr, CF_STRING_ENCODING_UTF8);
            CFArrayAppendValue(arr, val as *const _);
            CFArrayAppendValue(arr, val as *const _);
            assert_eq!(CFArrayGetCount(arr as CFArrayRef), 2);

            CFRelease(val as CFTypeRef);
            CFRelease(arr as CFTypeRef);
        }
    }

    /// CFDictionary: create with key-value pair, retrieve value.
    #[test]
    fn cfdictionary_create_and_get_value() {
        unsafe {
            let k = CFStringCreateWithCString(
                std::ptr::null(),
                b"key\0".as_ptr() as *const i8,
                CF_STRING_ENCODING_UTF8,
            );
            let v = CFStringCreateWithCString(
                std::ptr::null(),
                b"value\0".as_ptr() as *const i8,
                CF_STRING_ENCODING_UTF8,
            );
            let mut keys: [*const std::os::raw::c_void; 1] = [k as *const _];
            let mut vals: [*const std::os::raw::c_void; 1] = [v as *const _];
            let dict = CFDictionaryCreate(
                std::ptr::null(),
                keys.as_mut_ptr(),
                vals.as_mut_ptr(),
                1,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            );
            assert!(!dict.is_null());
            assert_eq!(CFDictionaryGetCount(dict), 1);

            let got = CFDictionaryGetValue(dict, k as *const _);
            assert!(!got.is_null());
            assert!(CFEqual(got as CFTypeRef, v as CFTypeRef) != 0);

            CFRelease(dict as CFTypeRef);
            CFRelease(k as CFTypeRef);
            CFRelease(v as CFTypeRef);
        }
    }

    /// CFNumber: create from i64, read back.
    #[test]
    fn cfnumber_create_and_get_value() {
        unsafe {
            let val: i64 = 42;
            let num = CFNumberCreate(
                std::ptr::null(),
                CF_NUMBER_SINT64_TYPE,
                &val as *const _ as *const _,
            );
            assert!(!num.is_null());

            let mut out: i64 = 0;
            let ok = CFNumberGetValue(num, CF_NUMBER_SINT64_TYPE, &mut out as *mut _ as *mut _);
            assert!(ok != 0);
            assert_eq!(out, 42);

            CFRelease(num as CFTypeRef);
        }
    }

    /// CFAbsoluteTimeGetCurrent returns a plausible timestamp.
    #[test]
    fn absolute_time_is_plausible() {
        unsafe {
            let t = CFAbsoluteTimeGetCurrent();
            // CF absolute time epoch is 2001-01-01; any time after 2020 is > 600M seconds
            assert!(t > 600_000_000.0);
        }
    }

    /// CFData: create from bytes, get length, get byte pointer.
    #[test]
    fn cfdata_create_and_access() {
        unsafe {
            let bytes = b"test data";
            let data = CFDataCreate(std::ptr::null(), bytes.as_ptr(), bytes.len() as _);
            assert!(!data.is_null());
            assert_eq!(CFDataGetLength(data), 9);

            let ptr = CFDataGetBytePtr(data);
            assert!(!ptr.is_null());
            assert_eq!(*ptr, b't');

            CFRelease(data as CFTypeRef);
        }
    }

    /// CFSet: create and count.
    #[test]
    fn cfset_create_and_count() {
        unsafe {
            let s1 = CFStringCreateWithCString(
                std::ptr::null(),
                b"a\0".as_ptr() as *const i8,
                CF_STRING_ENCODING_UTF8,
            );
            let s2 = CFStringCreateWithCString(
                std::ptr::null(),
                b"b\0".as_ptr() as *const i8,
                CF_STRING_ENCODING_UTF8,
            );
            let mut values: [*const std::os::raw::c_void; 2] = [s1 as _, s2 as _];
            let set = CFSetCreate(
                std::ptr::null(),
                values.as_mut_ptr(),
                2,
                &kCFTypeSetCallBacks,
            );
            assert!(!set.is_null());
            assert_eq!(CFSetGetCount(set), 2);

            CFRelease(set as CFTypeRef);
            CFRelease(s1 as CFTypeRef);
            CFRelease(s2 as CFTypeRef);
        }
    }
}

// ==================== ColorSync ====================
// ColorSync has no C functions exported; only opaque ref types.
#[cfg(all(target_os = "macos", feature = "ColorSync"))]
mod color_sync {
    use apple_sys::ColorSync::*;

    #[test]
    fn types_are_distinct() {
        let _p: ColorSyncProfileRef = std::ptr::null();
        let _t: ColorSyncTransformRef = std::ptr::null_mut();
        let _m: ColorSyncMutableProfileRef = std::ptr::null_mut();
        assert!(std::mem::size_of::<ColorSyncAlphaInfo>() > 0);
    }
}

// ==================== IOKit ====================
// IOKit functions lack export markers; only types available via scanner.
#[cfg(all(target_os = "macos", feature = "IOKit"))]
mod iokit {
    use apple_sys::IOKit::*;

    #[test]
    fn mach_port_types_are_sized() {
        assert!(std::mem::size_of::<io_object_t>() > 0);
        assert!(std::mem::size_of::<io_service_t>() > 0);
        assert!(std::mem::size_of::<io_connect_t>() > 0);
        assert!(std::mem::size_of::<io_iterator_t>() > 0);
    }

    #[test]
    fn notification_port_ref_is_nullable() {
        let p: IONotificationPortRef = std::ptr::null_mut();
        assert!(p.is_null());
    }

    #[test]
    fn io_return_constants() {
        let _r: IOReturn = 0;
        assert_eq!(std::mem::size_of::<IOReturn>(), 4);
    }
}

// ==================== CoreGraphics ====================
#[cfg(all(target_os = "macos", feature = "CoreGraphics"))]
mod core_graphics {
    use apple_sys::CoreFoundation::*;
    use apple_sys::CoreGraphics::*;

    /// Create device RGB color space, verify component count and model.
    #[test]
    fn color_space_create_device_rgb() {
        unsafe {
            let cs = CGColorSpaceCreateDeviceRGB();
            assert!(!cs.is_null());
            assert_eq!(CGColorSpaceGetNumberOfComponents(cs), 3);
            assert_eq!(CGColorSpaceGetModel(cs), 1); // kCGColorSpaceModelRGB
            CGColorSpaceRelease(cs);
        }
    }

    /// Create device gray color space.
    #[test]
    fn color_space_create_device_gray() {
        unsafe {
            let cs = CGColorSpaceCreateDeviceGray();
            assert!(!cs.is_null());
            assert_eq!(CGColorSpaceGetNumberOfComponents(cs), 1);
            assert_eq!(CGColorSpaceGetModel(cs), 0); // kCGColorSpaceModelMonochrome
            CGColorSpaceRelease(cs);
        }
    }

    /// Create a color in device RGB space, verify components.
    #[test]
    fn color_create_and_get_components() {
        unsafe {
            let cs = CGColorSpaceCreateDeviceRGB();
            let components: [CGFloat; 4] = [1.0, 0.0, 0.0, 1.0]; // red
            let color = CGColorCreate(cs, components.as_ptr());
            assert!(!color.is_null());
            assert_eq!(CGColorGetNumberOfComponents(color), 4);
            assert_eq!(CGColorGetAlpha(color), 1.0);
            CGColorRelease(color);
            CGColorSpaceRelease(cs);
        }
    }

    /// CGRect geometry helpers.
    #[test]
    fn rect_geometry() {
        let r = CGRect {
            origin: CGPoint { x: 10.0, y: 20.0 },
            size: CGSize {
                width: 100.0,
                height: 50.0,
            },
        };
        unsafe {
            assert_eq!(CGRectGetMinX(r), 10.0);
            assert_eq!(CGRectGetMaxX(r), 110.0);
            assert_eq!(CGRectGetMidY(r), 45.0);
            assert_eq!(CGRectGetWidth(r), 100.0);
            assert_eq!(CGRectGetHeight(r), 50.0);
            assert!(!CGRectIsEmpty(r));
            assert!(!CGRectIsNull(r));
            assert!(CGRectContainsPoint(r, CGPoint { x: 50.0, y: 30.0 }));
            assert!(!CGRectContainsPoint(r, CGPoint { x: 200.0, y: 200.0 }));
        }
    }

    /// CGRect intersection and union.
    #[test]
    fn rect_intersection_and_union() {
        unsafe {
            let a = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize {
                    width: 100.0,
                    height: 100.0,
                },
            };
            let b = CGRect {
                origin: CGPoint { x: 50.0, y: 50.0 },
                size: CGSize {
                    width: 100.0,
                    height: 100.0,
                },
            };
            assert!(CGRectIntersectsRect(a, b));

            let inter = CGRectIntersection(a, b);
            assert_eq!(CGRectGetWidth(inter), 50.0);
            assert_eq!(CGRectGetHeight(inter), 50.0);

            let uni = CGRectUnion(a, b);
            assert_eq!(CGRectGetWidth(uni), 150.0);
            assert_eq!(CGRectGetHeight(uni), 150.0);
        }
    }

    /// CGPath: create mutable path, add rect, verify bounding box.
    #[test]
    fn path_create_and_bounding_box() {
        unsafe {
            let path = CGPathCreateMutable();
            assert!(!path.is_null());
            assert!(CGPathIsEmpty(path as CGPathRef));

            let rect = CGRect {
                origin: CGPoint { x: 10.0, y: 20.0 },
                size: CGSize {
                    width: 30.0,
                    height: 40.0,
                },
            };
            CGPathAddRect(path, std::ptr::null(), rect);
            assert!(!CGPathIsEmpty(path as CGPathRef));

            let bbox = CGPathGetBoundingBox(path as CGPathRef);
            assert_eq!(CGRectGetMinX(bbox), 10.0);
            assert_eq!(CGRectGetMinY(bbox), 20.0);
            assert_eq!(CGRectGetWidth(bbox), 30.0);
            assert_eq!(CGRectGetHeight(bbox), 40.0);

            CGPathRelease(path as CGPathRef);
        }
    }

    /// CGBitmapContext: create, draw, verify dimensions.
    #[test]
    fn bitmap_context_create_and_draw() {
        unsafe {
            let cs = CGColorSpaceCreateDeviceRGB();
            let ctx = CGBitmapContextCreate(
                std::ptr::null_mut(),
                64,
                32,
                8,      // bits per component
                64 * 4, // bytes per row
                cs,
                1, // kCGImageAlphaPremultipliedLast
            );
            assert!(!ctx.is_null());
            assert_eq!(CGBitmapContextGetWidth(ctx), 64);
            assert_eq!(CGBitmapContextGetHeight(ctx), 32);

            // Fill with red
            CGContextSetRGBFillColor(ctx, 1.0, 0.0, 0.0, 1.0);
            let fill_rect = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize {
                    width: 64.0,
                    height: 32.0,
                },
            };
            CGContextFillRect(ctx, fill_rect);

            // Verify we can read back data
            let data = CGBitmapContextGetData(ctx);
            assert!(!data.is_null());

            CGContextRelease(ctx);
            CGColorSpaceRelease(cs);
        }
    }

    /// CGColorSpaceGetTypeID returns a valid type ID.
    #[test]
    fn type_ids() {
        unsafe {
            assert!(CGColorSpaceGetTypeID() > 0);
            assert!(CGPathGetTypeID() > 0);
            assert!(CGDataProviderGetTypeID() > 0);
        }
    }
}

// ==================== CoreText ====================
#[cfg(all(target_os = "macos", feature = "CoreText"))]
mod core_text {
    use apple_sys::CoreFoundation::*;
    use apple_sys::CoreText::*;

    const CF_STRING_ENCODING_UTF8: CFStringEncoding = 0x08000100;

    /// Create a CTFont by name and verify size.
    #[test]
    fn ctfont_create_with_name() {
        unsafe {
            let name = CFStringCreateWithCString(
                std::ptr::null(),
                b"Helvetica\0".as_ptr() as *const i8,
                CF_STRING_ENCODING_UTF8,
            );
            let font = CTFontCreateWithName(name, 12.0, std::ptr::null());
            assert!(!font.is_null());
            assert_eq!(CTFontGetSize(font), 12.0);
            CFRelease(font as CFTypeRef);
            CFRelease(name as CFTypeRef);
        }
    }

    /// CTFont: copy family name and full name.
    #[test]
    fn ctfont_copy_names() {
        unsafe {
            let name = CFStringCreateWithCString(
                std::ptr::null(),
                b"Helvetica\0".as_ptr() as *const i8,
                CF_STRING_ENCODING_UTF8,
            );
            let font = CTFontCreateWithName(name, 14.0, std::ptr::null());

            let family = CTFontCopyFamilyName(font);
            assert!(!family.is_null());
            assert!(CFStringGetLength(family) > 0);
            CFRelease(family as CFTypeRef);

            let full = CTFontCopyFullName(font);
            assert!(!full.is_null());
            assert!(CFStringGetLength(full) > 0);
            CFRelease(full as CFTypeRef);

            let ps = CTFontCopyPostScriptName(font);
            assert!(!ps.is_null());
            assert!(CFStringGetLength(ps) > 0);
            CFRelease(ps as CFTypeRef);

            CFRelease(font as CFTypeRef);
            CFRelease(name as CFTypeRef);
        }
    }

    /// CTFont: typographic metrics (ascent, descent, leading).
    #[test]
    fn ctfont_metrics() {
        unsafe {
            let name = CFStringCreateWithCString(
                std::ptr::null(),
                b"Helvetica\0".as_ptr() as *const i8,
                CF_STRING_ENCODING_UTF8,
            );
            let font = CTFontCreateWithName(name, 24.0, std::ptr::null());

            let ascent = CTFontGetAscent(font);
            let descent = CTFontGetDescent(font);
            assert!(ascent > 0.0, "ascent should be positive");
            assert!(descent > 0.0, "descent should be positive (unsigned)");

            let glyph_count = CTFontGetGlyphCount(font);
            assert!(glyph_count > 0, "font should have glyphs");

            CFRelease(font as CFTypeRef);
            CFRelease(name as CFTypeRef);
        }
    }

    /// CTFontDescriptor: create with name and size, copy attribute.
    #[test]
    fn ctfont_descriptor() {
        unsafe {
            let name = CFStringCreateWithCString(
                std::ptr::null(),
                b"Courier\0".as_ptr() as *const i8,
                CF_STRING_ENCODING_UTF8,
            );
            let desc = CTFontDescriptorCreateWithNameAndSize(name, 10.0);
            assert!(!desc.is_null());
            let tid = CTFontDescriptorGetTypeID();
            assert!(tid > 0);

            let attrs = CTFontDescriptorCopyAttributes(desc);
            assert!(!attrs.is_null());
            assert!(CFDictionaryGetCount(attrs) > 0);
            CFRelease(attrs as CFTypeRef);

            CFRelease(desc as CFTypeRef);
            CFRelease(name as CFTypeRef);
        }
    }

    /// CTLine: create from attributed string, get glyph count and runs.
    #[test]
    fn ctline_create_and_get_glyph_count() {
        unsafe {
            let text = CFStringCreateWithCString(
                std::ptr::null(),
                b"Hello\0".as_ptr() as *const i8,
                CF_STRING_ENCODING_UTF8,
            );
            let attr_str = CFAttributedStringCreate(std::ptr::null(), text, std::ptr::null());
            assert!(!attr_str.is_null());

            let line = CTLineCreateWithAttributedString(attr_str);
            assert!(!line.is_null());
            assert!(CTLineGetGlyphCount(line) >= 5);

            let runs = CTLineGetGlyphRuns(line);
            assert!(!runs.is_null());
            assert!(
                CFArrayGetCount(runs) > 0,
                "line should have at least one run"
            );

            CFRelease(line as CFTypeRef);
            CFRelease(attr_str as CFTypeRef);
            CFRelease(text as CFTypeRef);
        }
    }

    /// Type IDs are valid.
    #[test]
    fn type_ids() {
        unsafe {
            assert!(CTFontGetTypeID() > 0);
            assert!(CTFontDescriptorGetTypeID() > 0);
            assert!(CTLineGetTypeID() > 0);
        }
    }
}

// ==================== ImageIO ====================
#[cfg(all(target_os = "macos", feature = "ImageIO"))]
mod image_io {
    use apple_sys::CoreFoundation::*;
    use apple_sys::ImageIO::*;

    /// Type IDs are distinct and valid.
    #[test]
    fn type_ids() {
        unsafe {
            let src_tid = CGImageSourceGetTypeID();
            let dst_tid = CGImageDestinationGetTypeID();
            let meta_tid = CGImageMetadataGetTypeID();
            assert!(src_tid > 0);
            assert!(dst_tid > 0);
            assert!(meta_tid > 0);
            assert_ne!(src_tid, dst_tid);
            assert_ne!(src_tid, meta_tid);
        }
    }

    /// CGImageSourceCopyTypeIdentifiers returns supported formats.
    #[test]
    fn source_type_identifiers() {
        unsafe {
            let arr = CGImageSourceCopyTypeIdentifiers();
            assert!(!arr.is_null());
            let count = CFArrayGetCount(arr);
            assert!(count > 0, "should support at least one image format");
            CFRelease(arr as CFTypeRef);
        }
    }

    /// CGImageDestinationCopyTypeIdentifiers returns supported output formats.
    #[test]
    fn destination_type_identifiers() {
        unsafe {
            let arr = CGImageDestinationCopyTypeIdentifiers();
            assert!(!arr.is_null());
            let count = CFArrayGetCount(arr);
            assert!(count > 0, "should support at least one output format");
            CFRelease(arr as CFTypeRef);
        }
    }

    /// CGImageMetadata: create mutable, copy (should be empty).
    #[test]
    fn image_metadata_create_and_copy() {
        unsafe {
            let meta = CGImageMetadataCreateMutable();
            assert!(!meta.is_null());

            let tags = CGImageMetadataCopyTags(meta as _);
            // Empty metadata has null or empty tag array
            if !tags.is_null() {
                assert_eq!(CFArrayGetCount(tags), 0);
                CFRelease(tags as CFTypeRef);
            }

            CFRelease(meta as CFTypeRef);
        }
    }

    /// CGImageSource: create incremental source.
    #[test]
    fn incremental_source_create() {
        unsafe {
            let src = CGImageSourceCreateIncremental(std::ptr::null());
            assert!(!src.is_null());
            let status = CGImageSourceGetStatus(src);
            // Incremental source with no data should be incomplete
            assert_ne!(status, 0); // != kCGImageStatusComplete
            CFRelease(src as CFTypeRef);
        }
    }
}

// ==================== CFNetwork ====================
// CFNetwork functions use multi-line export patterns not yet scanned.
// Test exported constants (statics) which ARE available.
#[cfg(all(target_os = "macos", feature = "CFNetwork"))]
mod cfnetwork {
    use apple_sys::CFNetwork::*;
    use apple_sys::CoreFoundation::*;

    /// CFNetwork error domain string is non-null.
    #[test]
    fn error_domain_statics() {
        unsafe {
            assert!(!kCFErrorDomainCFNetwork.is_null());
            assert!(CFStringGetLength(kCFErrorDomainCFNetwork) > 0);
        }
    }

    /// HTTP version strings are non-null.
    #[test]
    fn http_version_statics() {
        unsafe {
            assert!(!kCFHTTPVersion1_0.is_null());
            assert!(!kCFHTTPVersion1_1.is_null());
            assert!(!kCFHTTPVersion2_0.is_null());
            assert!(CFStringGetLength(kCFHTTPVersion1_1) > 0);
        }
    }

    /// Stream property keys are non-null.
    #[test]
    fn stream_property_statics() {
        unsafe {
            assert!(!kCFStreamPropertySSLContext.is_null());
            assert!(!kCFStreamPropertySSLPeerTrust.is_null());
            assert!(!kCFStreamSSLLevel.is_null());
        }
    }

    /// Opaque ref types are pointer-sized.
    #[test]
    fn ref_types_layout() {
        assert_eq!(
            std::mem::size_of::<CFHTTPMessageRef>(),
            std::mem::size_of::<*mut u8>()
        );
        assert_eq!(
            std::mem::size_of::<CFHostRef>(),
            std::mem::size_of::<*mut u8>()
        );
        assert_eq!(
            std::mem::size_of::<CFNetServiceRef>(),
            std::mem::size_of::<*mut u8>()
        );
    }
}

// ==================== DiskArbitration ====================
#[cfg(all(target_os = "macos", feature = "DiskArbitration"))]
mod disk_arbitration {
    use apple_sys::CoreFoundation::{CFAllocatorGetDefault, CFRelease, CFTypeID, CFTypeRef};
    use apple_sys::DiskArbitration::*;

    /// Create a DA session, verify type ID, and release.
    #[test]
    fn session_create_typeid_release() {
        unsafe {
            let session = DASessionCreate(CFAllocatorGetDefault());
            assert!(!session.is_null(), "DASessionCreate returned null");

            let tid: CFTypeID = DASessionGetTypeID();
            assert!(tid > 0, "DASessionGetTypeID should be positive");

            CFRelease(session as CFTypeRef);
        }
    }

    /// Create an approval session and release.
    #[test]
    fn approval_session_create_release() {
        unsafe {
            let session = DAApprovalSessionCreate(CFAllocatorGetDefault());
            assert!(!session.is_null());

            let tid = DAApprovalSessionGetTypeID();
            assert!(tid > 0);

            CFRelease(session as CFTypeRef);
        }
    }

    /// DADiskGetTypeID returns a valid type ID.
    #[test]
    fn disk_type_id() {
        unsafe {
            let tid = DADiskGetTypeID();
            assert!(tid > 0);
        }
    }
}

// ==================== Security ====================
#[cfg(all(target_os = "macos", feature = "Security"))]
mod security {
    use apple_sys::Security::*;

    /// Verify Security ref types are correctly sized pointers.
    #[test]
    fn security_ref_types() {
        assert_eq!(
            std::mem::size_of::<AuthorizationRef>(),
            std::mem::size_of::<*mut u8>()
        );
        assert_eq!(
            std::mem::size_of::<SecKeyRef>(),
            std::mem::size_of::<*mut u8>()
        );
    }

    /// Authorization structs are correctly sized.
    #[test]
    fn authorization_structs_layout() {
        assert!(std::mem::size_of::<AuthorizationItem>() > 0);
        assert!(std::mem::size_of::<AuthorizationRights>() > 0);
        assert!(std::mem::size_of::<AuthorizationRights>() >= 2 * std::mem::size_of::<usize>());
    }

    /// SecTransformNoData returns a sentinel value.
    #[test]
    fn sec_transform_no_data() {
        unsafe {
            let sentinel = SecTransformNoData();
            assert!(
                !sentinel.is_null(),
                "SecTransformNoData should return non-null sentinel"
            );
        }
    }
}

// ==================== CoreServices ====================
// Sub-frameworks: AE (AppleEvents), FSEvents
#[cfg(all(target_os = "macos", feature = "CoreServices"))]
mod core_services {
    use apple_sys::CoreServices::*;

    /// AE sub-framework: initialize, create a typed desc, check, dispose.
    #[test]
    fn ae_create_and_dispose_desc() {
        unsafe {
            let mut desc: AEDesc = std::mem::zeroed();
            AEInitializeDesc(&mut desc);

            let val: i32 = 12345;
            let status = AECreateDesc(
                0x6C6F6E67, // 'long' as DescType
                &val as *const _ as *const _,
                std::mem::size_of::<i32>() as _,
                &mut desc,
            );
            assert_eq!(status, 0, "AECreateDesc should succeed");

            let dispose_status = AEDisposeDesc(&mut desc);
            assert_eq!(dispose_status, 0, "AEDisposeDesc should succeed");
        }
    }

    /// AE sub-framework: create an AEDescList, add items, count.
    #[test]
    fn ae_create_list_and_count() {
        unsafe {
            let mut list: AEDescList = std::mem::zeroed();
            AEInitializeDesc(&mut list as *mut _ as *mut AEDesc);
            let status = AECreateList(std::ptr::null(), 0, 0, &mut list);
            assert_eq!(status, 0, "AECreateList should succeed");

            let mut count: ::std::os::raw::c_long = 0;
            let count_status = AECountItems(&list, &mut count);
            assert_eq!(count_status, 0);
            assert_eq!(count, 0, "empty list should have 0 items");

            AEDisposeDesc(&mut list as *mut _ as *mut AEDesc);
        }
    }

    /// FSEvents sub-framework: current event ID is positive.
    #[test]
    fn fs_events_current_event_id() {
        unsafe {
            let eid = FSEventsGetCurrentEventId();
            assert!(
                eid > 0,
                "FSEventsGetCurrentEventId should return positive value"
            );
        }
    }

    /// FSEvents: two consecutive calls return non-decreasing IDs.
    #[test]
    fn fs_events_id_monotonic() {
        unsafe {
            let id1 = FSEventsGetCurrentEventId();
            let id2 = FSEventsGetCurrentEventId();
            assert!(
                id2 >= id1,
                "event IDs should be monotonically non-decreasing"
            );
        }
    }
}

// ==================== Foundation ====================
#[cfg(all(target_os = "macos", feature = "Foundation"))]
mod foundation {
    use apple_sys::Foundation::*;

    /// NSPageSize returns a positive power-of-two page size.
    #[test]
    fn ns_page_size() {
        unsafe {
            let ps = NSPageSize();
            assert!(ps > 0);
            assert!(ps.is_power_of_two(), "page size should be power of 2");
        }
    }

    /// NSRoundUpToMultipleOfPageSize rounds correctly.
    #[test]
    fn ns_round_up_page_size() {
        unsafe {
            let ps = NSPageSize();
            let rounded = NSRoundUpToMultipleOfPageSize(1);
            assert_eq!(rounded, ps, "rounding 1 up should give one page");

            let rounded2 = NSRoundUpToMultipleOfPageSize(ps + 1);
            assert_eq!(rounded2, ps * 2, "ps+1 should round up to 2*ps");
        }
    }

    /// NSUserName returns a non-null string.
    #[test]
    fn ns_user_name() {
        unsafe {
            let name = NSUserName();
            assert!(!name.0.is_null(), "NSUserName should return non-null");
        }
    }

    /// NSFullUserName returns a non-null string.
    #[test]
    fn ns_full_user_name() {
        unsafe {
            let name = NSFullUserName();
            assert!(!name.0.is_null(), "NSFullUserName should return non-null");
        }
    }

    /// NSHomeDirectory returns a non-null string.
    #[test]
    fn ns_home_directory() {
        unsafe {
            let home = NSHomeDirectory();
            assert!(!home.0.is_null(), "NSHomeDirectory should return non-null");
        }
    }

    /// NSTemporaryDirectory returns a non-null string.
    #[test]
    fn ns_temporary_directory() {
        unsafe {
            let tmp = NSTemporaryDirectory();
            assert!(
                !tmp.0.is_null(),
                "NSTemporaryDirectory should return non-null"
            );
        }
    }

    /// NSDefaultMallocZone returns a non-null zone.
    #[test]
    fn ns_default_malloc_zone() {
        unsafe {
            let zone = NSDefaultMallocZone();
            assert!(!zone.is_null());
        }
    }

    /// NSCountFrames returns the current call stack depth.
    #[test]
    fn ns_count_frames() {
        unsafe {
            let depth = NSCountFrames();
            assert!(depth > 0, "should have at least one stack frame");
        }
    }
}

// ==================== IOSurface ====================
// IOSurface functions lack export markers; test statics and types.
#[cfg(all(target_os = "macos", feature = "IOSurface"))]
mod iosurface {
    use apple_sys::CoreFoundation::*;
    use apple_sys::IOSurface::*;

    #[test]
    fn surface_types_layout() {
        assert_eq!(std::mem::size_of::<IOSurfaceID>(), 4);
        assert!(std::mem::size_of::<IOSurface>() > 0);
    }

    /// IOSurface property key statics are non-null CFStrings.
    #[test]
    fn property_key_statics() {
        unsafe {
            assert!(!kIOSurfaceWidth.is_null());
            assert!(!kIOSurfaceHeight.is_null());
            assert!(!kIOSurfaceBytesPerRow.is_null());
            assert!(!kIOSurfaceAllocSize.is_null());
            assert!(CFStringGetLength(kIOSurfaceWidth) > 0);
            assert!(CFStringGetLength(kIOSurfaceHeight) > 0);
        }
    }

    #[test]
    fn lock_options_and_subsampling() {
        let _lo: IOSurfaceLockOptions = 0;
        let _s: IOSurfaceSubsampling = 0;
        assert!(std::mem::size_of::<IOSurfaceLockOptions>() > 0);
    }
}

// ==================== Metal ====================
#[cfg(all(target_os = "macos", feature = "Metal"))]
mod metal {
    use apple_sys::Metal::*;

    /// MTLCreateSystemDefaultDevice returns a non-null device.
    #[test]
    fn create_system_default_device() {
        unsafe {
            let device = MTLCreateSystemDefaultDevice();
            assert!(!device.is_null(), "no Metal-capable GPU found");
        }
    }

    /// MTLCopyAllDevices returns a non-empty device list.
    #[test]
    fn copy_all_devices() {
        unsafe {
            let devices = MTLCopyAllDevices();
            assert!(!devices.0.is_null(), "MTLCopyAllDevices returned null");
        }
    }

    /// MTLIOCompressionContextDefaultChunkSize returns a positive value.
    #[test]
    fn io_compression_default_chunk_size() {
        unsafe {
            let size = MTLIOCompressionContextDefaultChunkSize();
            assert!(size > 0, "default chunk size should be positive");
        }
    }

    /// MTLClearColor has RGBA double fields.
    #[test]
    fn clear_color_layout() {
        let c = MTLClearColor {
            red: 1.0,
            green: 0.5,
            blue: 0.0,
            alpha: 1.0,
        };
        assert_eq!(c.red, 1.0);
        assert_eq!(c.alpha, 1.0);
        assert_eq!(std::mem::size_of::<MTLClearColor>(), 4 * 8); // 4 doubles
    }

    /// MTLSize has width/height/depth fields.
    #[test]
    fn size_layout() {
        let s = MTLSize {
            width: 1920,
            height: 1080,
            depth: 1,
        };
        assert_eq!(s.width, 1920);
        assert_eq!(s.height, 1080);
    }

    /// MTLSamplePosition has x/y float fields.
    #[test]
    fn sample_position_layout() {
        let p = MTLSamplePosition { x: 0.5, y: 0.5 };
        assert_eq!(p.x, 0.5);
    }

    /// Enum types are available.
    #[test]
    fn enum_types() {
        let _pf: MTLPixelFormat = 0;
        let _dt: MTLDataType = 0;
        let _la: MTLLoadAction = 0;
    }

    /// ObjC descriptor classes are properly sized.
    #[test]
    fn descriptor_class_sizes() {
        assert!(std::mem::size_of::<MTLTextureDescriptor>() > 0);
        assert!(std::mem::size_of::<MTLRenderPipelineDescriptor>() > 0);
        assert!(std::mem::size_of::<MTLRenderPassDescriptor>() > 0);
    }
}

// ==================== ApplicationServices ====================
// Sub-framework tests (HIServices, PrintCore, ATS) are in test_sub_frameworks.rs.
#[cfg(all(target_os = "macos", feature = "ApplicationServices"))]
mod application_services {
    use apple_sys::ApplicationServices::*;
    use apple_sys::CoreFoundation::Boolean;

    /// HIServices: AXIsProcessTrusted returns a boolean.
    #[test]
    fn ax_is_process_trusted() {
        unsafe {
            let _trusted: Boolean = AXIsProcessTrusted();
        }
    }

    /// ATS: ATSGetGeneration returns a positive generation number.
    #[test]
    fn ats_get_generation() {
        unsafe {
            let generation = ATSGetGeneration();
            assert!(generation > 0, "ATS generation should be positive");
        }
    }

    /// ATS: font family iterator round-trip.
    #[test]
    fn ats_font_family_iterator() {
        unsafe {
            let mut iter: ATSFontFamilyIterator = std::ptr::null_mut();
            let status = ATSFontFamilyIteratorCreate(
                0, // kATSFontContextLocal
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
                &mut iter,
            );
            assert_eq!(status, 0, "ATSFontFamilyIteratorCreate should succeed");
            assert!(!iter.is_null());

            let mut family: ATSFontFamilyRef = 0;
            let next_status = ATSFontFamilyIteratorNext(iter, &mut family);
            if next_status == 0 {
                assert!(family != 0, "first font family ref should be non-zero");
            }

            ATSFontFamilyIteratorRelease(&mut iter);
        }
    }
}

// ==================== QuartzCore ====================
#[cfg(all(target_os = "macos", feature = "QuartzCore"))]
mod quartz_core {
    use apple_sys::QuartzCore::*;

    /// CACurrentMediaTime returns a positive timestamp.
    #[test]
    fn ca_current_media_time() {
        unsafe {
            let t = CACurrentMediaTime();
            assert!(t > 0.0, "CACurrentMediaTime should return positive value");
        }
    }

    /// Two consecutive calls return non-decreasing timestamps.
    #[test]
    fn ca_media_time_monotonic() {
        unsafe {
            let t1 = CACurrentMediaTime();
            let t2 = CACurrentMediaTime();
            assert!(
                t2 >= t1,
                "timestamps should be monotonically non-decreasing"
            );
        }
    }

    /// CALayer struct is properly sized.
    #[test]
    fn ca_layer_layout() {
        assert!(std::mem::size_of::<CALayer>() > 0);
    }

    /// CATransform3D is a 4x4 matrix (16 doubles = 128 bytes).
    #[test]
    fn ca_transform3d_layout() {
        assert_eq!(std::mem::size_of::<CATransform3D>(), 16 * 8);
        let identity = CATransform3D {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m24: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
            m34: 0.0,
            m41: 0.0,
            m42: 0.0,
            m43: 0.0,
            m44: 1.0,
        };
        assert_eq!(identity.m11, 1.0);
        assert_eq!(identity.m44, 1.0);
    }
}

// ==================== AppKit ====================
#[cfg(all(target_os = "macos", feature = "AppKit"))]
mod appkit {
    use apple_sys::AppKit::*;

    /// NSApp (shared application) layout.
    #[test]
    fn ns_application_layout() {
        assert!(std::mem::size_of::<NSApplication>() > 0);
    }

    /// NSWindow struct is properly sized.
    #[test]
    fn ns_window_layout() {
        assert!(std::mem::size_of::<NSWindow>() > 0);
    }

    /// NSView struct is properly sized.
    #[test]
    fn ns_view_layout() {
        assert!(std::mem::size_of::<NSView>() > 0);
    }

    /// NSViewController struct is properly sized.
    #[test]
    fn ns_view_controller_layout() {
        assert!(std::mem::size_of::<NSViewController>() > 0);
    }

    /// NSBackingStoreType enum values.
    #[test]
    fn ns_backing_store_type() {
        let _t: NSBackingStoreType = 0;
        assert!(std::mem::size_of::<NSBackingStoreType>() > 0);
    }
}

// ==================== CoreVideo ====================
#[cfg(all(target_os = "macos", feature = "CoreVideo"))]
mod core_video {
    use apple_sys::CoreVideo::*;

    /// CVReturn is a 32-bit integer.
    #[test]
    fn cv_return_layout() {
        assert_eq!(std::mem::size_of::<CVReturn>(), 4);
        let _success: CVReturn = 0;
    }

    /// CVBufferRef is pointer-sized.
    #[test]
    fn cv_buffer_ref_layout() {
        assert_eq!(
            std::mem::size_of::<CVBufferRef>(),
            std::mem::size_of::<*mut u8>()
        );
    }
}

// ==================== CoreMedia ====================
#[cfg(all(target_os = "macos", feature = "CoreMedia"))]
mod core_media {
    use apple_sys::CoreMedia::*;

    /// CMTime struct has value, timescale, flags, epoch fields.
    #[test]
    fn cm_time_layout() {
        let t = CMTime {
            value: 0,
            timescale: 1,
            flags: 0,
            epoch: 0,
        };
        // Use local copies to avoid unaligned references on packed structs
        let val = { t.value };
        let ts = { t.timescale };
        assert_eq!(val, 0);
        assert_eq!(ts, 1);
        assert!(std::mem::size_of::<CMTime>() > 0);
    }

    /// CMTimeRange has start and duration.
    #[test]
    fn cm_time_range_layout() {
        let start = CMTime {
            value: 0,
            timescale: 1,
            flags: 0,
            epoch: 0,
        };
        let dur = CMTime {
            value: 30,
            timescale: 1,
            flags: 0,
            epoch: 0,
        };
        let range = CMTimeRange {
            start,
            duration: dur,
        };
        // Use local copies to avoid unaligned references on packed structs
        let start_val = { range.start.value };
        let dur_val = { range.duration.value };
        assert_eq!(start_val, 0);
        assert_eq!(dur_val, 30);
    }
}
