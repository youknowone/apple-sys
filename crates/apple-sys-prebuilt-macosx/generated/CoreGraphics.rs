#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::ColorSync::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use libc::{id_t, off_t, pid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type boolean_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOSurface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGContext {
    _unused: [u8; 0],
}
pub type CGContextRef = *mut CGContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGColor {
    _unused: [u8; 0],
}
pub type CGColorRef = *mut CGColor;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGColorSpace {
    _unused: [u8; 0],
}
pub type CGColorSpaceRef = *mut CGColorSpace;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGDataProvider {
    _unused: [u8; 0],
}
pub type CGDataProviderRef = *mut CGDataProvider;
pub type CGDataProviderGetBytesCallback = ::std::option::Option<
    unsafe extern "C" fn(
        info: *mut ::std::os::raw::c_void,
        buffer: *mut ::std::os::raw::c_void,
        count: usize,
    ) -> usize,
>;
pub type CGDataProviderSkipForwardCallback = ::std::option::Option<
    unsafe extern "C" fn(info: *mut ::std::os::raw::c_void, count: off_t) -> off_t,
>;
pub type CGDataProviderRewindCallback =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>;
pub type CGDataProviderReleaseInfoCallback =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGDataProviderSequentialCallbacks {
    pub version: ::std::os::raw::c_uint,
    pub getBytes: CGDataProviderGetBytesCallback,
    pub skipForward: CGDataProviderSkipForwardCallback,
    pub rewind: CGDataProviderRewindCallback,
    pub releaseInfo: CGDataProviderReleaseInfoCallback,
}
pub type CGDataProviderGetBytePointerCallback = ::std::option::Option<
    unsafe extern "C" fn(info: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
>;
pub type CGDataProviderReleaseBytePointerCallback = ::std::option::Option<
    unsafe extern "C" fn(info: *mut ::std::os::raw::c_void, pointer: *const ::std::os::raw::c_void),
>;
pub type CGDataProviderGetBytesAtPositionCallback = ::std::option::Option<
    unsafe extern "C" fn(
        info: *mut ::std::os::raw::c_void,
        buffer: *mut ::std::os::raw::c_void,
        pos: off_t,
        cnt: usize,
    ) -> usize,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGDataProviderDirectCallbacks {
    pub version: ::std::os::raw::c_uint,
    pub getBytePointer: CGDataProviderGetBytePointerCallback,
    pub releaseBytePointer: CGDataProviderReleaseBytePointerCallback,
    pub getBytesAtPosition: CGDataProviderGetBytesAtPositionCallback,
    pub releaseInfo: CGDataProviderReleaseInfoCallback,
}
pub type CGDataProviderReleaseDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        info: *mut ::std::os::raw::c_void,
        data: *const ::std::os::raw::c_void,
        size: usize,
    ),
>;
pub type CGColorRenderingIntent = i32;
pub type CGColorSpaceModel = i32;
pub type ColorSyncProfileRef = *const ColorSyncProfile;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPattern {
    _unused: [u8; 0],
}
pub type CGPatternRef = *mut CGPattern;
pub type CGPatternTiling = i32;
pub type CGPatternDrawPatternCallback = ::std::option::Option<
    unsafe extern "C" fn(info: *mut ::std::os::raw::c_void, context: CGContextRef),
>;
pub type CGPatternReleaseInfoCallback =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPatternCallbacks {
    pub version: ::std::os::raw::c_uint,
    pub drawPattern: CGPatternDrawPatternCallback,
    pub releaseInfo: CGPatternReleaseInfoCallback,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGFont {
    _unused: [u8; 0],
}
pub type CGFontRef = *mut CGFont;
pub type CGFontIndex = ::std::os::raw::c_ushort;
pub type CGGlyph = CGFontIndex;
pub type CGFontPostScriptFormat = i32;
pub type CGGlyphDeprecatedEnum = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGGradient {
    _unused: [u8; 0],
}
pub type CGGradientRef = *mut CGGradient;
pub type CGGradientDrawingOptions = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGImage {
    _unused: [u8; 0],
}
pub type CGImageRef = *mut CGImage;
pub type CGImageAlphaInfo = u32;
pub type CGImageComponentInfo = u32;
pub type CGImageByteOrderInfo = u32;
pub type CGImagePixelFormatInfo = u32;
pub type CGBitmapInfo = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPath {
    _unused: [u8; 0],
}
pub type CGMutablePathRef = *mut CGPath;
pub type CGPathRef = *const CGPath;
pub type CGLineJoin = i32;
pub type CGLineCap = i32;
pub type CGPathElementType = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPathElement {
    pub type_: CGPathElementType,
    pub points: *mut CGPoint,
}
pub type CGPathApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(info: *mut ::std::os::raw::c_void, element: *const CGPathElement),
>;
pub type CGPathApplyBlock = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFDocument {
    _unused: [u8; 0],
}
pub type CGPDFDocumentRef = *mut CGPDFDocument;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFPage {
    _unused: [u8; 0],
}
pub type CGPDFPageRef = *mut CGPDFPage;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFDictionary {
    _unused: [u8; 0],
}
pub type CGPDFDictionaryRef = *mut CGPDFDictionary;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFArray {
    _unused: [u8; 0],
}
pub type CGPDFArrayRef = *mut CGPDFArray;
pub type CGPDFBoolean = ::std::os::raw::c_uchar;
pub type CGPDFInteger = ::std::os::raw::c_long;
pub type CGPDFReal = CGFloat;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFObject {
    _unused: [u8; 0],
}
pub type CGPDFObjectRef = *mut CGPDFObject;
pub type CGPDFObjectType = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFStream {
    _unused: [u8; 0],
}
pub type CGPDFStreamRef = *mut CGPDFStream;
pub type CGPDFDataFormat = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFString {
    _unused: [u8; 0],
}
pub type CGPDFStringRef = *mut CGPDFString;
pub type CGPDFArrayApplierBlock = *mut ::std::os::raw::c_void;
pub type CGPDFDictionaryApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(
        key: *const ::std::os::raw::c_char,
        value: CGPDFObjectRef,
        info: *mut ::std::os::raw::c_void,
    ),
>;
pub type CGPDFDictionaryApplierBlock = *mut ::std::os::raw::c_void;
pub type CGPDFBox = i32;
pub type CGPDFAccessPermissions = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGShading {
    _unused: [u8; 0],
}
pub type CGShadingRef = *mut CGShading;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGFunction {
    _unused: [u8; 0],
}
pub type CGFunctionRef = *mut CGFunction;
pub type CGFunctionEvaluateCallback = ::std::option::Option<
    unsafe extern "C" fn(info: *mut ::std::os::raw::c_void, in_: *const CGFloat, out: *mut CGFloat),
>;
pub type CGFunctionReleaseInfoCallback =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGFunctionCallbacks {
    pub version: ::std::os::raw::c_uint,
    pub evaluate: CGFunctionEvaluateCallback,
    pub releaseInfo: CGFunctionReleaseInfoCallback,
}
pub type CGToneMapping = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGContentToneMappingInfo {
    pub method: CGToneMapping,
    pub options: CFDictionaryRef,
}
pub type CGPathDrawingMode = i32;
pub type CGTextDrawingMode = i32;
pub type CGTextEncoding = i32;
pub type CGInterpolationQuality = i32;
pub type CGBlendMode = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGRenderingBufferProvider {
    _unused: [u8; 0],
}
pub type CGRenderingBufferProviderRef = *mut CGRenderingBufferProvider;
pub type CGBitmapContextReleaseDataCallback = ::std::option::Option<
    unsafe extern "C" fn(
        releaseInfo: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
    ),
>;
pub type CGColorModel = u32;
pub type CGComponent = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGContentInfo {
    pub deepestImageComponent: CGComponent,
    pub contentColorModels: CGColorModel,
    pub hasWideGamut: bool,
    pub hasTransparency: bool,
    pub largestContentHeadroom: f32,
}
pub type CGBitmapLayout = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGColorConversionInfo {
    _unused: [u8; 0],
}
pub type CGColorConversionInfoRef = *const CGColorConversionInfo;
pub type CGColorConversionInfoTransformType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGColorBufferFormat {
    pub version: u32,
    pub bitmapInfo: CGBitmapInfo,
    pub bitsPerComponent: usize,
    pub bitsPerPixel: usize,
    pub bytesPerRow: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGColorDataFormat {
    pub version: u32,
    pub colorspace_info: CFTypeRef,
    pub bitmap_info: CGBitmapInfo,
    pub bits_per_component: usize,
    pub bytes_per_row: usize,
    pub intent: CGColorRenderingIntent,
    pub decode: *mut CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGDataConsumer {
    _unused: [u8; 0],
}
pub type CGDataConsumerRef = *mut CGDataConsumer;
pub type CGDataConsumerPutBytesCallback = ::std::option::Option<
    unsafe extern "C" fn(
        info: *mut ::std::os::raw::c_void,
        buffer: *const ::std::os::raw::c_void,
        count: usize,
    ) -> usize,
>;
pub type CGDataConsumerReleaseInfoCallback =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGDataConsumerCallbacks {
    pub putBytes: CGDataConsumerPutBytesCallback,
    pub releaseConsumer: CGDataConsumerReleaseInfoCallback,
}
pub type CGError = i32;
pub type CGErrorCallback = ::std::option::Option<unsafe extern "C" fn()>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGLayer {
    _unused: [u8; 0],
}
pub type CGLayerRef = *mut CGLayer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFContentStream {
    _unused: [u8; 0],
}
pub type CGPDFContentStreamRef = *mut CGPDFContentStream;
pub type CGPDFTagType = i32;
pub type CGPDFTagProperty = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFOperatorTable {
    _unused: [u8; 0],
}
pub type CGPDFOperatorTableRef = *mut CGPDFOperatorTable;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPDFScanner {
    _unused: [u8; 0],
}
pub type CGPDFScannerRef = *mut CGPDFScanner;
pub type CGPDFOperatorCallback = ::std::option::Option<
    unsafe extern "C" fn(scanner: CGPDFScannerRef, info: *mut ::std::os::raw::c_void),
>;
pub type CGWindowID = u32;
pub type CGWindowSharingType = u32;
pub type CGWindowBackingType = u32;
pub type CGWindowListOption = u32;
pub type CGWindowImageOption = u32;
pub type CGWindowLevelKey = i32;
pub type CGWindowLevel = i32;
pub type CGDirectDisplayID = u32;
pub type CGOpenGLDisplayMask = u32;
pub type CGRefreshRate = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGDisplayMode {
    _unused: [u8; 0],
}
pub type CGDisplayModeRef = *mut CGDisplayMode;
pub type CGGammaValue = f32;
pub type CGCaptureOptions = u32;
pub type CGDisplayCount = u32;
pub use self::CGError as CGDisplayErr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGDeviceColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CGDisplayConfigRef {
    _unused: [u8; 0],
}
pub type CGDisplayConfigRef = *mut _CGDisplayConfigRef;
pub type CGConfigureOption = u32;
pub type CGDisplayChangeSummaryFlags = u32;
pub type CGDisplayReconfigurationCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        display: CGDirectDisplayID,
        flags: CGDisplayChangeSummaryFlags,
        userInfo: *mut ::std::os::raw::c_void,
    ),
>;
pub type CGDisplayFadeReservationToken = u32;
pub type CGDisplayBlendFraction = f32;
pub type CGDisplayFadeInterval = f32;
pub type CGDisplayReservationInterval = f32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGDisplayStream {
    _unused: [u8; 0],
}
pub type CGDisplayStreamRef = *mut CGDisplayStream;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGDisplayStreamUpdate {
    _unused: [u8; 0],
}
pub type CGDisplayStreamUpdateRef = *const CGDisplayStreamUpdate;
pub type CGDisplayStreamUpdateRectType = i32;
pub type CGDisplayStreamFrameStatus = i32;
pub type CGDisplayStreamFrameAvailableHandler = *mut ::std::os::raw::c_void;
pub use self::CGError as CGEventErr;
pub type CGButtonCount = u32;
pub type CGWheelCount = u32;
pub type CGCharCode = u16;
pub type CGKeyCode = u16;
pub type CGScreenRefreshCallback = ::std::option::Option<
    unsafe extern "C" fn(count: u32, rects: *const CGRect, userInfo: *mut ::std::os::raw::c_void),
>;
pub type CGScreenUpdateOperation = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGScreenUpdateMoveDelta {
    pub dX: i32,
    pub dY: i32,
}
pub type CGScreenUpdateMoveCallback = ::std::option::Option<
    unsafe extern "C" fn(
        delta: CGScreenUpdateMoveDelta,
        count: usize,
        rects: *const CGRect,
        userInfo: *mut ::std::os::raw::c_void,
    ),
>;
pub type CGEventFilterMask = u32;
pub type CGEventSuppressionState = u32;
pub type CGRectCount = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CGEvent {
    _unused: [u8; 0],
}
pub type CGEventRef = *mut __CGEvent;
pub type CGMouseButton = u32;
pub type CGScrollEventUnit = u32;
pub type CGMomentumScrollPhase = u32;
pub type CGScrollPhase = u32;
pub type CGGesturePhase = u32;
pub type CGEventFlags = u64;
pub type CGEventType = u32;
pub type CGEventTimestamp = u64;
pub type CGEventField = u32;
pub type CGEventMouseSubtype = u32;
pub type CGEventTapLocation = u32;
pub type CGEventTapPlacement = u32;
pub type CGEventTapOptions = u32;
pub type CGEventMask = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CGEventTapProxy {
    _unused: [u8; 0],
}
pub type CGEventTapProxy = *mut __CGEventTapProxy;
pub type CGEventTapCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        proxy: CGEventTapProxy,
        type_: CGEventType,
        event: CGEventRef,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> CGEventRef,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CGEventTapInformation {
    pub eventTapID: u32,
    pub tapPoint: CGEventTapLocation,
    pub options: CGEventTapOptions,
    pub eventsOfInterest: CGEventMask,
    pub tappingProcess: pid_t,
    pub processBeingTapped: pid_t,
    pub enabled: bool,
    pub minUsecLatency: f32,
    pub avgUsecLatency: f32,
    pub maxUsecLatency: f32,
}
pub type CGEventTapInformation = __CGEventTapInformation;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CGEventSource {
    _unused: [u8; 0],
}
pub type CGEventSourceRef = *mut __CGEventSource;
pub type CGEventSourceStateID = i32;
pub type CGEventSourceKeyboardType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPSConverter {
    _unused: [u8; 0],
}
pub type CGPSConverterRef = *mut CGPSConverter;
pub type CGPSConverterBeginDocumentCallback =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>;
pub type CGPSConverterEndDocumentCallback =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void, success: bool)>;
pub type CGPSConverterBeginPageCallback = ::std::option::Option<
    unsafe extern "C" fn(
        info: *mut ::std::os::raw::c_void,
        pageNumber: usize,
        pageInfo: CFDictionaryRef,
    ),
>;
pub type CGPSConverterEndPageCallback = ::std::option::Option<
    unsafe extern "C" fn(
        info: *mut ::std::os::raw::c_void,
        pageNumber: usize,
        pageInfo: CFDictionaryRef,
    ),
>;
pub type CGPSConverterProgressCallback =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>;
pub type CGPSConverterMessageCallback = ::std::option::Option<
    unsafe extern "C" fn(info: *mut ::std::os::raw::c_void, message: CFStringRef),
>;
pub type CGPSConverterReleaseInfoCallback =
    ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPSConverterCallbacks {
    pub version: ::std::os::raw::c_uint,
    pub beginDocument: CGPSConverterBeginDocumentCallback,
    pub endDocument: CGPSConverterEndDocumentCallback,
    pub beginPage: CGPSConverterBeginPageCallback,
    pub endPage: CGPSConverterEndPageCallback,
    pub noteProgress: CGPSConverterProgressCallback,
    pub noteMessage: CGPSConverterMessageCallback,
    pub releaseInfo: CGPSConverterReleaseInfoCallback,
}
unsafe extern "C" {
    pub static CGPointZero: CGPoint;
}
unsafe extern "C" {
    pub static CGSizeZero: CGSize;
}
unsafe extern "C" {
    pub static CGRectZero: CGRect;
}
unsafe extern "C" {
    pub static CGRectNull: CGRect;
}
unsafe extern "C" {
    pub static CGRectInfinite: CGRect;
}
unsafe extern "C" {
    pub fn CGRectGetMinX(rect: CGRect) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGRectGetMidX(rect: CGRect) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGRectGetMaxX(rect: CGRect) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGRectGetMinY(rect: CGRect) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGRectGetMidY(rect: CGRect) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGRectGetMaxY(rect: CGRect) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGRectGetWidth(rect: CGRect) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGRectGetHeight(rect: CGRect) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGPointEqualToPoint(point1: CGPoint, point2: CGPoint) -> bool;
}
unsafe extern "C" {
    pub fn CGSizeEqualToSize(size1: CGSize, size2: CGSize) -> bool;
}
unsafe extern "C" {
    pub fn CGRectEqualToRect(rect1: CGRect, rect2: CGRect) -> bool;
}
unsafe extern "C" {
    pub fn CGRectStandardize(rect: CGRect) -> CGRect;
}
unsafe extern "C" {
    pub fn CGRectIsEmpty(rect: CGRect) -> bool;
}
unsafe extern "C" {
    pub fn CGRectIsNull(rect: CGRect) -> bool;
}
unsafe extern "C" {
    pub fn CGRectIsInfinite(rect: CGRect) -> bool;
}
unsafe extern "C" {
    pub fn CGRectInset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
}
unsafe extern "C" {
    pub fn CGRectIntegral(rect: CGRect) -> CGRect;
}
unsafe extern "C" {
    pub fn CGRectUnion(r1: CGRect, r2: CGRect) -> CGRect;
}
unsafe extern "C" {
    pub fn CGRectIntersection(r1: CGRect, r2: CGRect) -> CGRect;
}
unsafe extern "C" {
    pub fn CGRectOffset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
}
unsafe extern "C" {
    pub fn CGRectDivide(
        rect: CGRect,
        slice: *mut CGRect,
        remainder: *mut CGRect,
        amount: CGFloat,
        edge: CGRectEdge,
    );
}
unsafe extern "C" {
    pub fn CGRectContainsPoint(rect: CGRect, point: CGPoint) -> bool;
}
unsafe extern "C" {
    pub fn CGRectContainsRect(rect1: CGRect, rect2: CGRect) -> bool;
}
unsafe extern "C" {
    pub fn CGRectIntersectsRect(rect1: CGRect, rect2: CGRect) -> bool;
}
unsafe extern "C" {
    pub fn CGPointCreateDictionaryRepresentation(point: CGPoint) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGPointMakeWithDictionaryRepresentation(
        dict: CFDictionaryRef,
        point: *mut CGPoint,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGSizeCreateDictionaryRepresentation(size: CGSize) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGSizeMakeWithDictionaryRepresentation(dict: CFDictionaryRef, size: *mut CGSize)
        -> bool;
}
unsafe extern "C" {
    pub fn CGRectCreateDictionaryRepresentation(arg1: CGRect) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGRectMakeWithDictionaryRepresentation(dict: CFDictionaryRef, rect: *mut CGRect)
        -> bool;
}
unsafe extern "C" {
    pub static CGAffineTransformIdentity: CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformMake(
        a: CGFloat,
        b: CGFloat,
        c: CGFloat,
        d: CGFloat,
        tx: CGFloat,
        ty: CGFloat,
    ) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformMakeTranslation(tx: CGFloat, ty: CGFloat) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformMakeScale(sx: CGFloat, sy: CGFloat) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformMakeRotation(angle: CGFloat) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformIsIdentity(t: CGAffineTransform) -> bool;
}
unsafe extern "C" {
    pub fn CGAffineTransformTranslate(
        t: CGAffineTransform,
        tx: CGFloat,
        ty: CGFloat,
    ) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformScale(
        t: CGAffineTransform,
        sx: CGFloat,
        sy: CGFloat,
    ) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformRotate(t: CGAffineTransform, angle: CGFloat) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformInvert(t: CGAffineTransform) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformConcat(
        t1: CGAffineTransform,
        t2: CGAffineTransform,
    ) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGAffineTransformEqualToTransform(t1: CGAffineTransform, t2: CGAffineTransform) -> bool;
}
unsafe extern "C" {
    pub fn CGPointApplyAffineTransform(point: CGPoint, t: CGAffineTransform) -> CGPoint;
}
unsafe extern "C" {
    pub fn CGSizeApplyAffineTransform(size: CGSize, t: CGAffineTransform) -> CGSize;
}
unsafe extern "C" {
    pub fn CGRectApplyAffineTransform(rect: CGRect, t: CGAffineTransform) -> CGRect;
}
unsafe extern "C" {
    pub fn CGAffineTransformDecompose(transform: CGAffineTransform) -> CGAffineTransformComponents;
}
unsafe extern "C" {
    pub fn CGAffineTransformMakeWithComponents(
        components: CGAffineTransformComponents,
    ) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGDataProviderGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGDataProviderCreateSequential(
        info: *mut ::std::os::raw::c_void,
        callbacks: *const CGDataProviderSequentialCallbacks,
    ) -> CGDataProviderRef;
}
unsafe extern "C" {
    pub fn CGDataProviderCreateDirect(
        info: *mut ::std::os::raw::c_void,
        size: off_t,
        callbacks: *const CGDataProviderDirectCallbacks,
    ) -> CGDataProviderRef;
}
unsafe extern "C" {
    pub fn CGDataProviderCreateWithData(
        info: *mut ::std::os::raw::c_void,
        data: *const ::std::os::raw::c_void,
        size: usize,
        releaseData: CGDataProviderReleaseDataCallback,
    ) -> CGDataProviderRef;
}
unsafe extern "C" {
    pub fn CGDataProviderCreateWithCFData(data: CFDataRef) -> CGDataProviderRef;
}
unsafe extern "C" {
    pub fn CGDataProviderCreateWithURL(url: CFURLRef) -> CGDataProviderRef;
}
unsafe extern "C" {
    pub fn CGDataProviderCreateWithFilename(
        filename: *const ::std::os::raw::c_char,
    ) -> CGDataProviderRef;
}
unsafe extern "C" {
    pub fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
}
unsafe extern "C" {
    pub fn CGDataProviderRelease(provider: CGDataProviderRef);
}
unsafe extern "C" {
    pub fn CGDataProviderCopyData(provider: CGDataProviderRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CGDataProviderGetInfo(provider: CGDataProviderRef) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static kCGColorSpaceGenericGray: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceGenericRGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceGenericCMYK: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceDisplayP3: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceGenericRGBLinear: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceAdobeRGB1998: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceSRGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceGenericGrayGamma2_2: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceGenericXYZ: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceGenericLab: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceACESCGLinear: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_709: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_709_PQ: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_709_HLG: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_2020: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_2020_sRGBGamma: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceROMMRGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceDCIP3: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceLinearITUR_2020: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceExtendedITUR_2020: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceExtendedLinearITUR_2020: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceLinearDisplayP3: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceExtendedDisplayP3: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceExtendedLinearDisplayP3: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_2100_PQ: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_2100_HLG: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceDisplayP3_PQ: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceDisplayP3_HLG: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_2020_PQ: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_2020_HLG: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceDisplayP3_PQ_EOTF: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceITUR_2020_PQ_EOTF: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceExtendedSRGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceLinearSRGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceExtendedLinearSRGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceExtendedGray: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceLinearGray: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceExtendedLinearGray: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceCoreMedia709: CFStringRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateDeviceGray() -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateDeviceRGB() -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateDeviceCMYK() -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateCalibratedGray(
        whitePoint: *const CGFloat,
        blackPoint: *const CGFloat,
        gamma: CGFloat,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateCalibratedRGB(
        whitePoint: *const CGFloat,
        blackPoint: *const CGFloat,
        gamma: *const CGFloat,
        matrix: *const CGFloat,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateLab(
        whitePoint: *const CGFloat,
        blackPoint: *const CGFloat,
        range: *const CGFloat,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateWithICCData(data: CFTypeRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateICCBased(
        nComponents: usize,
        range: *const CGFloat,
        profile: CGDataProviderRef,
        alternate: CGColorSpaceRef,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateIndexed(
        baseSpace: CGColorSpaceRef,
        lastIndex: usize,
        colorTable: *const ::std::os::raw::c_uchar,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreatePattern(baseSpace: CGColorSpaceRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub static kCGColorSpaceExtendedRange: CFStringRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateWithColorSyncProfile(
        arg1: ColorSyncProfileRef,
        options: CFDictionaryRef,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateWithName(name: CFStringRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceRetain(space: CGColorSpaceRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceRelease(space: CGColorSpaceRef);
}
unsafe extern "C" {
    pub fn CGColorSpaceGetName(space: CGColorSpaceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCopyName(space: CGColorSpaceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGColorSpaceGetNumberOfComponents(space: CGColorSpaceRef) -> usize;
}
unsafe extern "C" {
    pub fn CGColorSpaceGetModel(space: CGColorSpaceRef) -> CGColorSpaceModel;
}
unsafe extern "C" {
    pub fn CGColorSpaceGetBaseColorSpace(space: CGColorSpaceRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCopyBaseColorSpace(space: CGColorSpaceRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceGetColorTableCount(space: CGColorSpaceRef) -> usize;
}
unsafe extern "C" {
    pub fn CGColorSpaceGetColorTable(space: CGColorSpaceRef, table: *mut u8);
}
unsafe extern "C" {
    pub fn CGColorSpaceCopyICCData(space: CGColorSpaceRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceIsWideGamutRGB(arg1: CGColorSpaceRef) -> bool;
}
unsafe extern "C" {
    pub fn CGColorSpaceIsHDR(arg1: CGColorSpaceRef) -> bool;
}
unsafe extern "C" {
    pub fn CGColorSpaceUsesITUR_2100TF(arg1: CGColorSpaceRef) -> bool;
}
unsafe extern "C" {
    pub fn CGColorSpaceIsPQBased(s: CGColorSpaceRef) -> bool;
}
unsafe extern "C" {
    pub fn CGColorSpaceIsHLGBased(s: CGColorSpaceRef) -> bool;
}
unsafe extern "C" {
    pub fn CGColorSpaceSupportsOutput(space: CGColorSpaceRef) -> bool;
}
unsafe extern "C" {
    pub fn CGColorSpaceCopyPropertyList(space: CGColorSpaceRef) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateWithPropertyList(plist: CFPropertyListRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceUsesExtendedRange(space: CGColorSpaceRef) -> bool;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateLinearized(space: CGColorSpaceRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateExtended(space: CGColorSpaceRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateExtendedLinearized(space: CGColorSpaceRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateCopyWithStandardRange(space: CGColorSpaceRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateWithICCProfile(data: CFDataRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCopyICCProfile(space: CGColorSpaceRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CGColorSpaceCreateWithPlatformColorSpace(
        ref_: *const ::std::os::raw::c_void,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGPatternGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGPatternCreate(
        info: *mut ::std::os::raw::c_void,
        bounds: CGRect,
        matrix: CGAffineTransform,
        xStep: CGFloat,
        yStep: CGFloat,
        tiling: CGPatternTiling,
        isColored: bool,
        callbacks: *const CGPatternCallbacks,
    ) -> CGPatternRef;
}
unsafe extern "C" {
    pub fn CGPatternRetain(pattern: CGPatternRef) -> CGPatternRef;
}
unsafe extern "C" {
    pub fn CGPatternRelease(pattern: CGPatternRef);
}
unsafe extern "C" {
    pub fn CGColorCreate(space: CGColorSpaceRef, components: *const CGFloat) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateGenericGray(gray: CGFloat, alpha: CGFloat) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateGenericRGB(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateGenericCMYK(
        cyan: CGFloat,
        magenta: CGFloat,
        yellow: CGFloat,
        black: CGFloat,
        alpha: CGFloat,
    ) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateGenericGrayGamma2_2(gray: CGFloat, alpha: CGFloat) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateSRGB(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateWithContentHeadroom(
        headroom: f32,
        space: CGColorSpaceRef,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorGetContentHeadroom(color: CGColorRef) -> f32;
}
unsafe extern "C" {
    pub fn CGColorGetConstantColor(colorName: CFStringRef) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateWithPattern(
        space: CGColorSpaceRef,
        pattern: CGPatternRef,
        components: *const CGFloat,
    ) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateCopy(color: CGColorRef) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateCopyWithAlpha(color: CGColorRef, alpha: CGFloat) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorCreateCopyByMatchingToColorSpace(
        arg1: CGColorSpaceRef,
        intent: CGColorRenderingIntent,
        color: CGColorRef,
        options: CFDictionaryRef,
    ) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorRetain(color: CGColorRef) -> CGColorRef;
}
unsafe extern "C" {
    pub fn CGColorRelease(color: CGColorRef);
}
unsafe extern "C" {
    pub fn CGColorEqualToColor(color1: CGColorRef, color2: CGColorRef) -> bool;
}
unsafe extern "C" {
    pub fn CGColorGetNumberOfComponents(color: CGColorRef) -> usize;
}
unsafe extern "C" {
    pub fn CGColorGetComponents(color: CGColorRef) -> *const CGFloat;
}
unsafe extern "C" {
    pub fn CGColorGetAlpha(color: CGColorRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGColorGetColorSpace(color: CGColorRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGColorGetPattern(color: CGColorRef) -> CGPatternRef;
}
unsafe extern "C" {
    pub fn CGColorGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCGColorWhite: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorBlack: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorClear: CFStringRef;
}
unsafe extern "C" {
    pub fn CGFontGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGFontCreateWithPlatformFont(
        platformFontReference: *mut ::std::os::raw::c_void,
    ) -> CGFontRef;
}
unsafe extern "C" {
    pub fn CGFontCreateWithDataProvider(provider: CGDataProviderRef) -> CGFontRef;
}
unsafe extern "C" {
    pub fn CGFontCreateWithFontName(name: CFStringRef) -> CGFontRef;
}
unsafe extern "C" {
    pub fn CGFontCreateCopyWithVariations(
        font: CGFontRef,
        variations: CFDictionaryRef,
    ) -> CGFontRef;
}
unsafe extern "C" {
    pub fn CGFontRetain(font: CGFontRef) -> CGFontRef;
}
unsafe extern "C" {
    pub fn CGFontRelease(font: CGFontRef);
}
unsafe extern "C" {
    pub fn CGFontGetNumberOfGlyphs(font: CGFontRef) -> usize;
}
unsafe extern "C" {
    pub fn CGFontGetUnitsPerEm(font: CGFontRef) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CGFontCopyPostScriptName(font: CGFontRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGFontCopyFullName(font: CGFontRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGFontGetAscent(font: CGFontRef) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CGFontGetDescent(font: CGFontRef) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CGFontGetLeading(font: CGFontRef) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CGFontGetCapHeight(font: CGFontRef) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CGFontGetXHeight(font: CGFontRef) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CGFontGetFontBBox(font: CGFontRef) -> CGRect;
}
unsafe extern "C" {
    pub fn CGFontGetItalicAngle(font: CGFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGFontGetStemV(font: CGFontRef) -> CGFloat;
}
unsafe extern "C" {
    pub fn CGFontCopyVariationAxes(font: CGFontRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGFontCopyVariations(font: CGFontRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGFontGetGlyphAdvances(
        font: CGFontRef,
        glyphs: *const CGGlyph,
        count: usize,
        advances: *mut ::std::os::raw::c_int,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGFontGetGlyphBBoxes(
        font: CGFontRef,
        glyphs: *const CGGlyph,
        count: usize,
        bboxes: *mut CGRect,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGFontGetGlyphWithGlyphName(font: CGFontRef, name: CFStringRef) -> CGGlyph;
}
unsafe extern "C" {
    pub fn CGFontCopyGlyphNameForGlyph(font: CGFontRef, glyph: CGGlyph) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGFontCanCreatePostScriptSubset(font: CGFontRef, format: CGFontPostScriptFormat)
        -> bool;
}
unsafe extern "C" {
    pub fn CGFontCreatePostScriptSubset(
        font: CGFontRef,
        subsetName: CFStringRef,
        format: CGFontPostScriptFormat,
        glyphs: *const CGGlyph,
        count: usize,
        encoding: *const CGGlyph,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CGFontCreatePostScriptEncoding(font: CGFontRef, encoding: *const CGGlyph) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CGFontCopyTableTags(font: CGFontRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGFontCopyTableForTag(font: CGFontRef, tag: u32) -> CFDataRef;
}
unsafe extern "C" {
    pub static kCGFontVariationAxisName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGFontVariationAxisMinValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGFontVariationAxisMaxValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGFontVariationAxisDefaultValue: CFStringRef;
}
unsafe extern "C" {
    pub fn CGGradientGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGGradientCreateWithColorComponents(
        space: CGColorSpaceRef,
        components: *const CGFloat,
        locations: *const CGFloat,
        count: usize,
    ) -> CGGradientRef;
}
unsafe extern "C" {
    pub fn CGGradientCreateWithContentHeadroom(
        headroom: f32,
        space: CGColorSpaceRef,
        components: *const CGFloat,
        locations: *const CGFloat,
        count: usize,
    ) -> CGGradientRef;
}
unsafe extern "C" {
    pub fn CGGradientCreateWithColors(
        space: CGColorSpaceRef,
        colors: CFArrayRef,
        locations: *const CGFloat,
    ) -> CGGradientRef;
}
unsafe extern "C" {
    pub fn CGGradientRetain(gradient: CGGradientRef) -> CGGradientRef;
}
unsafe extern "C" {
    pub fn CGGradientRelease(gradient: CGGradientRef);
}
unsafe extern "C" {
    pub fn CGGradientGetContentHeadroom(gradient: CGGradientRef) -> f32;
}
unsafe extern "C" {
    pub fn CGImageGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGImageCreate(
        width: usize,
        height: usize,
        bitsPerComponent: usize,
        bitsPerPixel: usize,
        bytesPerRow: usize,
        space: CGColorSpaceRef,
        bitmapInfo: CGBitmapInfo,
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageMaskCreate(
        width: usize,
        height: usize,
        bitsPerComponent: usize,
        bitsPerPixel: usize,
        bytesPerRow: usize,
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateCopy(image: CGImageRef) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateWithJPEGDataProvider(
        source: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateWithPNGDataProvider(
        source: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateWithImageInRect(image: CGImageRef, rect: CGRect) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateWithMask(image: CGImageRef, mask: CGImageRef) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateWithMaskingColors(
        image: CGImageRef,
        components: *const CGFloat,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateCopyWithColorSpace(image: CGImageRef, space: CGColorSpaceRef)
        -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateWithContentHeadroom(
        headroom: f32,
        width: usize,
        height: usize,
        bitsPerComponent: usize,
        bitsPerPixel: usize,
        bytesPerRow: usize,
        space: CGColorSpaceRef,
        bitmapInfo: CGBitmapInfo,
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateCopyWithContentHeadroom(headroom: f32, image: CGImageRef) -> CGImageRef;
}
unsafe extern "C" {
    pub static mut kCGDefaultHDRImageContentHeadroom: f32;
}
unsafe extern "C" {
    pub fn CGImageGetContentHeadroom(image: CGImageRef) -> f32;
}
unsafe extern "C" {
    pub fn CGImageCalculateContentHeadroom(image: CGImageRef) -> f32;
}
unsafe extern "C" {
    pub fn CGImageGetContentAverageLightLevel(image: CGImageRef) -> f32;
}
unsafe extern "C" {
    pub fn CGImageCalculateContentAverageLightLevel(image: CGImageRef) -> f32;
}
unsafe extern "C" {
    pub fn CGImageCreateCopyWithContentAverageLightLevel(
        image: CGImageRef,
        avll: f32,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageCreateCopyWithCalculatedHDRStats(image: CGImageRef) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageRetain(image: CGImageRef) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageRelease(image: CGImageRef);
}
unsafe extern "C" {
    pub fn CGImageIsMask(image: CGImageRef) -> bool;
}
unsafe extern "C" {
    pub fn CGImageGetWidth(image: CGImageRef) -> usize;
}
unsafe extern "C" {
    pub fn CGImageGetHeight(image: CGImageRef) -> usize;
}
unsafe extern "C" {
    pub fn CGImageGetBitsPerComponent(image: CGImageRef) -> usize;
}
unsafe extern "C" {
    pub fn CGImageGetBitsPerPixel(image: CGImageRef) -> usize;
}
unsafe extern "C" {
    pub fn CGImageGetBytesPerRow(image: CGImageRef) -> usize;
}
unsafe extern "C" {
    pub fn CGImageGetColorSpace(image: CGImageRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGImageGetAlphaInfo(image: CGImageRef) -> CGImageAlphaInfo;
}
unsafe extern "C" {
    pub fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;
}
unsafe extern "C" {
    pub fn CGImageGetDecode(image: CGImageRef) -> *const CGFloat;
}
unsafe extern "C" {
    pub fn CGImageGetShouldInterpolate(image: CGImageRef) -> bool;
}
unsafe extern "C" {
    pub fn CGImageGetRenderingIntent(image: CGImageRef) -> CGColorRenderingIntent;
}
unsafe extern "C" {
    pub fn CGImageGetBitmapInfo(image: CGImageRef) -> CGBitmapInfo;
}
unsafe extern "C" {
    pub fn CGImageGetByteOrderInfo(image: CGImageRef) -> CGImageByteOrderInfo;
}
unsafe extern "C" {
    pub fn CGImageGetPixelFormatInfo(image: CGImageRef) -> CGImagePixelFormatInfo;
}
unsafe extern "C" {
    pub fn CGImageShouldToneMap(image: CGImageRef) -> bool;
}
unsafe extern "C" {
    pub fn CGImageContainsImageSpecificToneMappingMetadata(image: CGImageRef) -> bool;
}
unsafe extern "C" {
    pub fn CGImageGetUTType(image: CGImageRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGPathGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGPathCreateMutable() -> CGMutablePathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopy(path: CGPathRef) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopyByTransformingPath(
        path: CGPathRef,
        transform: *const CGAffineTransform,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateMutableCopy(path: CGPathRef) -> CGMutablePathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateMutableCopyByTransformingPath(
        path: CGPathRef,
        transform: *const CGAffineTransform,
    ) -> CGMutablePathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateWithRect(rect: CGRect, transform: *const CGAffineTransform) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateWithEllipseInRect(
        rect: CGRect,
        transform: *const CGAffineTransform,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateWithRoundedRect(
        rect: CGRect,
        cornerWidth: CGFloat,
        cornerHeight: CGFloat,
        transform: *const CGAffineTransform,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathAddRoundedRect(
        path: CGMutablePathRef,
        transform: *const CGAffineTransform,
        rect: CGRect,
        cornerWidth: CGFloat,
        cornerHeight: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGPathCreateCopyByDashingPath(
        path: CGPathRef,
        transform: *const CGAffineTransform,
        phase: CGFloat,
        lengths: *const CGFloat,
        count: usize,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopyByStrokingPath(
        path: CGPathRef,
        transform: *const CGAffineTransform,
        lineWidth: CGFloat,
        lineCap: CGLineCap,
        lineJoin: CGLineJoin,
        miterLimit: CGFloat,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathRetain(path: CGPathRef) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathRelease(path: CGPathRef);
}
unsafe extern "C" {
    pub fn CGPathEqualToPath(path1: CGPathRef, path2: CGPathRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPathMoveToPoint(
        path: CGMutablePathRef,
        m: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGPathAddLineToPoint(
        path: CGMutablePathRef,
        m: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGPathAddQuadCurveToPoint(
        path: CGMutablePathRef,
        m: *const CGAffineTransform,
        cpx: CGFloat,
        cpy: CGFloat,
        x: CGFloat,
        y: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGPathAddCurveToPoint(
        path: CGMutablePathRef,
        m: *const CGAffineTransform,
        cp1x: CGFloat,
        cp1y: CGFloat,
        cp2x: CGFloat,
        cp2y: CGFloat,
        x: CGFloat,
        y: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGPathCloseSubpath(path: CGMutablePathRef);
}
unsafe extern "C" {
    pub fn CGPathAddRect(path: CGMutablePathRef, m: *const CGAffineTransform, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGPathAddRects(
        path: CGMutablePathRef,
        m: *const CGAffineTransform,
        rects: *const CGRect,
        count: usize,
    );
}
unsafe extern "C" {
    pub fn CGPathAddLines(
        path: CGMutablePathRef,
        m: *const CGAffineTransform,
        points: *const CGPoint,
        count: usize,
    );
}
unsafe extern "C" {
    pub fn CGPathAddEllipseInRect(
        path: CGMutablePathRef,
        m: *const CGAffineTransform,
        rect: CGRect,
    );
}
unsafe extern "C" {
    pub fn CGPathAddRelativeArc(
        path: CGMutablePathRef,
        matrix: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        startAngle: CGFloat,
        delta: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGPathAddArc(
        path: CGMutablePathRef,
        m: *const CGAffineTransform,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        startAngle: CGFloat,
        endAngle: CGFloat,
        clockwise: bool,
    );
}
unsafe extern "C" {
    pub fn CGPathAddArcToPoint(
        path: CGMutablePathRef,
        m: *const CGAffineTransform,
        x1: CGFloat,
        y1: CGFloat,
        x2: CGFloat,
        y2: CGFloat,
        radius: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGPathAddPath(path1: CGMutablePathRef, m: *const CGAffineTransform, path2: CGPathRef);
}
unsafe extern "C" {
    pub fn CGPathIsEmpty(path: CGPathRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPathIsRect(path: CGPathRef, rect: *mut CGRect) -> bool;
}
unsafe extern "C" {
    pub fn CGPathGetCurrentPoint(path: CGPathRef) -> CGPoint;
}
unsafe extern "C" {
    pub fn CGPathGetBoundingBox(path: CGPathRef) -> CGRect;
}
unsafe extern "C" {
    pub fn CGPathGetPathBoundingBox(path: CGPathRef) -> CGRect;
}
unsafe extern "C" {
    pub fn CGPathContainsPoint(
        path: CGPathRef,
        m: *const CGAffineTransform,
        point: CGPoint,
        eoFill: bool,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPathApply(
        path: CGPathRef,
        info: *mut ::std::os::raw::c_void,
        function: CGPathApplierFunction,
    );
}
unsafe extern "C" {
    pub fn CGPathApplyWithBlock(path: CGPathRef, block: CGPathApplyBlock);
}
unsafe extern "C" {
    pub fn CGPathCreateCopyByNormalizing(path: CGPathRef, evenOddFillRule: bool) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopyByUnioningPath(
        path: CGPathRef,
        maskPath: CGPathRef,
        evenOddFillRule: bool,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopyByIntersectingPath(
        path: CGPathRef,
        maskPath: CGPathRef,
        evenOddFillRule: bool,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopyBySubtractingPath(
        path: CGPathRef,
        maskPath: CGPathRef,
        evenOddFillRule: bool,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopyBySymmetricDifferenceOfPath(
        path: CGPathRef,
        maskPath: CGPathRef,
        evenOddFillRule: bool,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopyOfLineBySubtractingPath(
        path: CGPathRef,
        maskPath: CGPathRef,
        evenOddFillRule: bool,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopyOfLineByIntersectingPath(
        path: CGPathRef,
        maskPath: CGPathRef,
        evenOddFillRule: bool,
    ) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathCreateSeparateComponents(path: CGPathRef, evenOddFillRule: bool) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGPathCreateCopyByFlattening(path: CGPathRef, flatteningThreshold: CGFloat)
        -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGPathIntersectsPath(path1: CGPathRef, path2: CGPathRef, evenOddFillRule: bool) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFObjectGetType(object: CGPDFObjectRef) -> CGPDFObjectType;
}
unsafe extern "C" {
    pub fn CGPDFObjectGetValue(
        object: CGPDFObjectRef,
        type_: CGPDFObjectType,
        value: *mut ::std::os::raw::c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFStreamGetDictionary(stream: CGPDFStreamRef) -> CGPDFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGPDFStreamCopyData(stream: CGPDFStreamRef, format: *mut CGPDFDataFormat) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CGPDFStringGetLength(string: CGPDFStringRef) -> usize;
}
unsafe extern "C" {
    pub fn CGPDFStringGetBytePtr(string: CGPDFStringRef) -> *const ::std::os::raw::c_uchar;
}
unsafe extern "C" {
    pub fn CGPDFStringCopyTextString(string: CGPDFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGPDFStringCopyDate(string: CGPDFStringRef) -> CFDateRef;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetCount(array: CGPDFArrayRef) -> usize;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetObject(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFObjectRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetNull(array: CGPDFArrayRef, index: usize) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetBoolean(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFBoolean,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetInteger(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFInteger,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetNumber(array: CGPDFArrayRef, index: usize, value: *mut CGPDFReal) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetName(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetString(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFStringRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetArray(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFArrayRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetDictionary(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayGetStream(
        array: CGPDFArrayRef,
        index: usize,
        value: *mut CGPDFStreamRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFArrayApplyBlock(
        array: CGPDFArrayRef,
        block: CGPDFArrayApplierBlock,
        info: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetCount(dict: CGPDFDictionaryRef) -> usize;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetObject(
        dict: CGPDFDictionaryRef,
        key: *const ::std::os::raw::c_char,
        value: *mut CGPDFObjectRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetBoolean(
        dict: CGPDFDictionaryRef,
        key: *const ::std::os::raw::c_char,
        value: *mut CGPDFBoolean,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetInteger(
        dict: CGPDFDictionaryRef,
        key: *const ::std::os::raw::c_char,
        value: *mut CGPDFInteger,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetNumber(
        dict: CGPDFDictionaryRef,
        key: *const ::std::os::raw::c_char,
        value: *mut CGPDFReal,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetName(
        dict: CGPDFDictionaryRef,
        key: *const ::std::os::raw::c_char,
        value: *mut *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetString(
        dict: CGPDFDictionaryRef,
        key: *const ::std::os::raw::c_char,
        value: *mut CGPDFStringRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetArray(
        dict: CGPDFDictionaryRef,
        key: *const ::std::os::raw::c_char,
        value: *mut CGPDFArrayRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetDictionary(
        dict: CGPDFDictionaryRef,
        key: *const ::std::os::raw::c_char,
        value: *mut CGPDFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryGetStream(
        dict: CGPDFDictionaryRef,
        key: *const ::std::os::raw::c_char,
        value: *mut CGPDFStreamRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDictionaryApplyFunction(
        dict: CGPDFDictionaryRef,
        function: CGPDFDictionaryApplierFunction,
        info: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CGPDFDictionaryApplyBlock(
        dict: CGPDFDictionaryRef,
        block: CGPDFDictionaryApplierBlock,
        info: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CGPDFPageRetain(page: CGPDFPageRef) -> CGPDFPageRef;
}
unsafe extern "C" {
    pub fn CGPDFPageRelease(page: CGPDFPageRef);
}
unsafe extern "C" {
    pub fn CGPDFPageGetDocument(page: CGPDFPageRef) -> CGPDFDocumentRef;
}
unsafe extern "C" {
    pub fn CGPDFPageGetPageNumber(page: CGPDFPageRef) -> usize;
}
unsafe extern "C" {
    pub fn CGPDFPageGetBoxRect(page: CGPDFPageRef, box_: CGPDFBox) -> CGRect;
}
unsafe extern "C" {
    pub fn CGPDFPageGetRotationAngle(page: CGPDFPageRef) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CGPDFPageGetDrawingTransform(
        page: CGPDFPageRef,
        box_: CGPDFBox,
        rect: CGRect,
        rotate: ::std::os::raw::c_int,
        preserveAspectRatio: bool,
    ) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGPDFPageGetDictionary(page: CGPDFPageRef) -> CGPDFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGPDFPageGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCGPDFOutlineTitle: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFOutlineChildren: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFOutlineDestination: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFOutlineDestinationRect: CFStringRef;
}
unsafe extern "C" {
    pub fn CGPDFDocumentCreateWithProvider(provider: CGDataProviderRef) -> CGPDFDocumentRef;
}
unsafe extern "C" {
    pub fn CGPDFDocumentCreateWithURL(url: CFURLRef) -> CGPDFDocumentRef;
}
unsafe extern "C" {
    pub fn CGPDFDocumentRetain(document: CGPDFDocumentRef) -> CGPDFDocumentRef;
}
unsafe extern "C" {
    pub fn CGPDFDocumentRelease(document: CGPDFDocumentRef);
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetVersion(
        document: CGPDFDocumentRef,
        majorVersion: *mut ::std::os::raw::c_int,
        minorVersion: *mut ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn CGPDFDocumentIsEncrypted(document: CGPDFDocumentRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDocumentUnlockWithPassword(
        document: CGPDFDocumentRef,
        password: *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDocumentIsUnlocked(document: CGPDFDocumentRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDocumentAllowsPrinting(document: CGPDFDocumentRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDocumentAllowsCopying(document: CGPDFDocumentRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetNumberOfPages(document: CGPDFDocumentRef) -> usize;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetPage(document: CGPDFDocumentRef, pageNumber: usize) -> CGPDFPageRef;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetCatalog(document: CGPDFDocumentRef) -> CGPDFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetInfo(document: CGPDFDocumentRef) -> CGPDFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetID(document: CGPDFDocumentRef) -> CGPDFArrayRef;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetOutline(document: CGPDFDocumentRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetAccessPermissions(document: CGPDFDocumentRef) -> CGPDFAccessPermissions;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetMediaBox(
        document: CGPDFDocumentRef,
        page: ::std::os::raw::c_int,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetCropBox(
        document: CGPDFDocumentRef,
        page: ::std::os::raw::c_int,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetBleedBox(
        document: CGPDFDocumentRef,
        page: ::std::os::raw::c_int,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetTrimBox(
        document: CGPDFDocumentRef,
        page: ::std::os::raw::c_int,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetArtBox(
        document: CGPDFDocumentRef,
        page: ::std::os::raw::c_int,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn CGPDFDocumentGetRotationAngle(
        document: CGPDFDocumentRef,
        page: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CGFunctionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGFunctionCreate(
        info: *mut ::std::os::raw::c_void,
        domainDimension: usize,
        domain: *const CGFloat,
        rangeDimension: usize,
        range: *const CGFloat,
        callbacks: *const CGFunctionCallbacks,
    ) -> CGFunctionRef;
}
unsafe extern "C" {
    pub fn CGFunctionRetain(function: CGFunctionRef) -> CGFunctionRef;
}
unsafe extern "C" {
    pub fn CGFunctionRelease(function: CGFunctionRef);
}
unsafe extern "C" {
    pub fn CGShadingGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGShadingCreateAxial(
        space: CGColorSpaceRef,
        start: CGPoint,
        end: CGPoint,
        function: CGFunctionRef,
        extendStart: bool,
        extendEnd: bool,
    ) -> CGShadingRef;
}
unsafe extern "C" {
    pub fn CGShadingCreateAxialWithContentHeadroom(
        headroom: f32,
        space: CGColorSpaceRef,
        start: CGPoint,
        end: CGPoint,
        function: CGFunctionRef,
        extendStart: bool,
        extendEnd: bool,
    ) -> CGShadingRef;
}
unsafe extern "C" {
    pub fn CGShadingCreateRadial(
        space: CGColorSpaceRef,
        start: CGPoint,
        startRadius: CGFloat,
        end: CGPoint,
        endRadius: CGFloat,
        function: CGFunctionRef,
        extendStart: bool,
        extendEnd: bool,
    ) -> CGShadingRef;
}
unsafe extern "C" {
    pub fn CGShadingCreateRadialWithContentHeadroom(
        headroom: f32,
        space: CGColorSpaceRef,
        start: CGPoint,
        startRadius: CGFloat,
        end: CGPoint,
        endRadius: CGFloat,
        function: CGFunctionRef,
        extendStart: bool,
        extendEnd: bool,
    ) -> CGShadingRef;
}
unsafe extern "C" {
    pub fn CGShadingRetain(shading: CGShadingRef) -> CGShadingRef;
}
unsafe extern "C" {
    pub fn CGShadingRelease(shading: CGShadingRef);
}
unsafe extern "C" {
    pub fn CGShadingGetContentHeadroom(shading: CGShadingRef) -> f32;
}
unsafe extern "C" {
    pub static kCGEXRToneMappingGammaDefog: CFStringRef;
}
unsafe extern "C" {
    pub static kCGEXRToneMappingGammaExposure: CFStringRef;
}
unsafe extern "C" {
    pub static kCGEXRToneMappingGammaKneeLow: CFStringRef;
}
unsafe extern "C" {
    pub static kCGEXRToneMappingGammaKneeHigh: CFStringRef;
}
unsafe extern "C" {
    pub fn CGEXRToneMappingGammaGetDefaultOptions() -> CFDictionaryRef;
}
unsafe extern "C" {
    pub static kCGUse100nitsHLGOOTF: CFStringRef;
}
unsafe extern "C" {
    pub static kCGUseBT1886ForCoreVideoGamma: CFStringRef;
}
unsafe extern "C" {
    pub static kCGSkipBoostToHDR: CFStringRef;
}
unsafe extern "C" {
    pub static kCGUseLegacyHDREcosystem: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCGPreferredDynamicRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCGDynamicRangeHigh: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCGDynamicRangeConstrained: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCGDynamicRangeStandard: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCGContentAverageLightLevel: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCGContentAverageLightLevelNits: CFStringRef;
}
unsafe extern "C" {
    pub fn CGContextGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGContextSaveGState(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextRestoreGState(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextScaleCTM(c: CGContextRef, sx: CGFloat, sy: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextTranslateCTM(c: CGContextRef, tx: CGFloat, ty: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextRotateCTM(c: CGContextRef, angle: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextConcatCTM(c: CGContextRef, transform: CGAffineTransform);
}
unsafe extern "C" {
    pub fn CGContextGetCTM(c: CGContextRef) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGContextSetLineWidth(c: CGContextRef, width: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextSetLineCap(c: CGContextRef, cap: CGLineCap);
}
unsafe extern "C" {
    pub fn CGContextSetLineJoin(c: CGContextRef, join: CGLineJoin);
}
unsafe extern "C" {
    pub fn CGContextSetMiterLimit(c: CGContextRef, limit: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextSetLineDash(
        c: CGContextRef,
        phase: CGFloat,
        lengths: *const CGFloat,
        count: usize,
    );
}
unsafe extern "C" {
    pub fn CGContextSetFlatness(c: CGContextRef, flatness: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextSetAlpha(c: CGContextRef, alpha: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextSetBlendMode(c: CGContextRef, mode: CGBlendMode);
}
unsafe extern "C" {
    pub fn CGContextBeginPath(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextMoveToPoint(c: CGContextRef, x: CGFloat, y: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextAddLineToPoint(c: CGContextRef, x: CGFloat, y: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextAddCurveToPoint(
        c: CGContextRef,
        cp1x: CGFloat,
        cp1y: CGFloat,
        cp2x: CGFloat,
        cp2y: CGFloat,
        x: CGFloat,
        y: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextAddQuadCurveToPoint(
        c: CGContextRef,
        cpx: CGFloat,
        cpy: CGFloat,
        x: CGFloat,
        y: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextClosePath(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextAddRect(c: CGContextRef, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGContextAddRects(c: CGContextRef, rects: *const CGRect, count: usize);
}
unsafe extern "C" {
    pub fn CGContextAddLines(c: CGContextRef, points: *const CGPoint, count: usize);
}
unsafe extern "C" {
    pub fn CGContextAddEllipseInRect(c: CGContextRef, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGContextAddArc(
        c: CGContextRef,
        x: CGFloat,
        y: CGFloat,
        radius: CGFloat,
        startAngle: CGFloat,
        endAngle: CGFloat,
        clockwise: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn CGContextAddArcToPoint(
        c: CGContextRef,
        x1: CGFloat,
        y1: CGFloat,
        x2: CGFloat,
        y2: CGFloat,
        radius: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextAddPath(c: CGContextRef, path: CGPathRef);
}
unsafe extern "C" {
    pub fn CGContextReplacePathWithStrokedPath(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextIsPathEmpty(c: CGContextRef) -> bool;
}
unsafe extern "C" {
    pub fn CGContextGetPathCurrentPoint(c: CGContextRef) -> CGPoint;
}
unsafe extern "C" {
    pub fn CGContextGetPathBoundingBox(c: CGContextRef) -> CGRect;
}
unsafe extern "C" {
    pub fn CGContextCopyPath(c: CGContextRef) -> CGPathRef;
}
unsafe extern "C" {
    pub fn CGContextPathContainsPoint(
        c: CGContextRef,
        point: CGPoint,
        mode: CGPathDrawingMode,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGContextDrawPath(c: CGContextRef, mode: CGPathDrawingMode);
}
unsafe extern "C" {
    pub fn CGContextFillPath(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextEOFillPath(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextStrokePath(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextFillRect(c: CGContextRef, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGContextFillRects(c: CGContextRef, rects: *const CGRect, count: usize);
}
unsafe extern "C" {
    pub fn CGContextStrokeRect(c: CGContextRef, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGContextStrokeRectWithWidth(c: CGContextRef, rect: CGRect, width: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextClearRect(c: CGContextRef, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGContextFillEllipseInRect(c: CGContextRef, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGContextStrokeEllipseInRect(c: CGContextRef, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGContextStrokeLineSegments(c: CGContextRef, points: *const CGPoint, count: usize);
}
unsafe extern "C" {
    pub fn CGContextClip(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextEOClip(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextResetClip(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextClipToMask(c: CGContextRef, rect: CGRect, mask: CGImageRef);
}
unsafe extern "C" {
    pub fn CGContextGetClipBoundingBox(c: CGContextRef) -> CGRect;
}
unsafe extern "C" {
    pub fn CGContextClipToRect(c: CGContextRef, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGContextClipToRects(c: CGContextRef, rects: *const CGRect, count: usize);
}
unsafe extern "C" {
    pub fn CGContextSetFillColorWithColor(c: CGContextRef, color: CGColorRef);
}
unsafe extern "C" {
    pub fn CGContextSetStrokeColorWithColor(c: CGContextRef, color: CGColorRef);
}
unsafe extern "C" {
    pub fn CGContextSetFillColorSpace(c: CGContextRef, space: CGColorSpaceRef);
}
unsafe extern "C" {
    pub fn CGContextSetStrokeColorSpace(c: CGContextRef, space: CGColorSpaceRef);
}
unsafe extern "C" {
    pub fn CGContextSetFillColor(c: CGContextRef, components: *const CGFloat);
}
unsafe extern "C" {
    pub fn CGContextSetStrokeColor(c: CGContextRef, components: *const CGFloat);
}
unsafe extern "C" {
    pub fn CGContextSetFillPattern(
        c: CGContextRef,
        pattern: CGPatternRef,
        components: *const CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextSetStrokePattern(
        c: CGContextRef,
        pattern: CGPatternRef,
        components: *const CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextSetPatternPhase(c: CGContextRef, phase: CGSize);
}
unsafe extern "C" {
    pub fn CGContextSetGrayFillColor(c: CGContextRef, gray: CGFloat, alpha: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextSetGrayStrokeColor(c: CGContextRef, gray: CGFloat, alpha: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextSetRGBFillColor(
        c: CGContextRef,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextSetRGBStrokeColor(
        c: CGContextRef,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextSetCMYKFillColor(
        c: CGContextRef,
        cyan: CGFloat,
        magenta: CGFloat,
        yellow: CGFloat,
        black: CGFloat,
        alpha: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextSetCMYKStrokeColor(
        c: CGContextRef,
        cyan: CGFloat,
        magenta: CGFloat,
        yellow: CGFloat,
        black: CGFloat,
        alpha: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextSetRenderingIntent(c: CGContextRef, intent: CGColorRenderingIntent);
}
unsafe extern "C" {
    pub fn CGContextSetEDRTargetHeadroom(c: CGContextRef, headroom: f32) -> bool;
}
unsafe extern "C" {
    pub fn CGContextGetEDRTargetHeadroom(c: CGContextRef) -> f32;
}
unsafe extern "C" {
    pub fn CGContextDrawImage(c: CGContextRef, rect: CGRect, image: CGImageRef);
}
unsafe extern "C" {
    pub fn CGContextDrawTiledImage(c: CGContextRef, rect: CGRect, image: CGImageRef);
}
unsafe extern "C" {
    pub fn CGContextDrawImageApplyingToneMapping(
        c: CGContextRef,
        r: CGRect,
        image: CGImageRef,
        method: CGToneMapping,
        options: CFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGContextGetContentToneMappingInfo(c: CGContextRef) -> CGContentToneMappingInfo;
}
unsafe extern "C" {
    pub fn CGContextSetContentToneMappingInfo(c: CGContextRef, info: CGContentToneMappingInfo);
}
unsafe extern "C" {
    pub fn CGContextGetInterpolationQuality(c: CGContextRef) -> CGInterpolationQuality;
}
unsafe extern "C" {
    pub fn CGContextSetInterpolationQuality(c: CGContextRef, quality: CGInterpolationQuality);
}
unsafe extern "C" {
    pub fn CGContextSetShadowWithColor(
        c: CGContextRef,
        offset: CGSize,
        blur: CGFloat,
        color: CGColorRef,
    );
}
unsafe extern "C" {
    pub fn CGContextSetShadow(c: CGContextRef, offset: CGSize, blur: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextDrawLinearGradient(
        c: CGContextRef,
        gradient: CGGradientRef,
        startPoint: CGPoint,
        endPoint: CGPoint,
        options: CGGradientDrawingOptions,
    );
}
unsafe extern "C" {
    pub fn CGContextDrawRadialGradient(
        c: CGContextRef,
        gradient: CGGradientRef,
        startCenter: CGPoint,
        startRadius: CGFloat,
        endCenter: CGPoint,
        endRadius: CGFloat,
        options: CGGradientDrawingOptions,
    );
}
unsafe extern "C" {
    pub fn CGContextDrawConicGradient(
        c: CGContextRef,
        gradient: CGGradientRef,
        center: CGPoint,
        angle: CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGContextDrawShading(c: CGContextRef, shading: CGShadingRef);
}
unsafe extern "C" {
    pub fn CGContextSetCharacterSpacing(c: CGContextRef, spacing: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextSetTextPosition(c: CGContextRef, x: CGFloat, y: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextGetTextPosition(c: CGContextRef) -> CGPoint;
}
unsafe extern "C" {
    pub fn CGContextSetTextMatrix(c: CGContextRef, t: CGAffineTransform);
}
unsafe extern "C" {
    pub fn CGContextGetTextMatrix(c: CGContextRef) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGContextSetTextDrawingMode(c: CGContextRef, mode: CGTextDrawingMode);
}
unsafe extern "C" {
    pub fn CGContextSetFont(c: CGContextRef, font: CGFontRef);
}
unsafe extern "C" {
    pub fn CGContextSetFontSize(c: CGContextRef, size: CGFloat);
}
unsafe extern "C" {
    pub fn CGContextShowGlyphsAtPositions(
        c: CGContextRef,
        glyphs: *const CGGlyph,
        Lpositions: *const CGPoint,
        count: usize,
    );
}
unsafe extern "C" {
    pub fn CGContextDrawPDFPage(c: CGContextRef, page: CGPDFPageRef);
}
unsafe extern "C" {
    pub fn CGContextBeginPage(c: CGContextRef, mediaBox: *const CGRect);
}
unsafe extern "C" {
    pub fn CGContextEndPage(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextRetain(c: CGContextRef) -> CGContextRef;
}
unsafe extern "C" {
    pub fn CGContextRelease(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextFlush(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextSynchronize(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextSynchronizeAttributes(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextSetShouldAntialias(c: CGContextRef, shouldAntialias: bool);
}
unsafe extern "C" {
    pub fn CGContextSetAllowsAntialiasing(c: CGContextRef, allowsAntialiasing: bool);
}
unsafe extern "C" {
    pub fn CGContextSetShouldSmoothFonts(c: CGContextRef, shouldSmoothFonts: bool);
}
unsafe extern "C" {
    pub fn CGContextSetAllowsFontSmoothing(c: CGContextRef, allowsFontSmoothing: bool);
}
unsafe extern "C" {
    pub fn CGContextSetShouldSubpixelPositionFonts(
        c: CGContextRef,
        shouldSubpixelPositionFonts: bool,
    );
}
unsafe extern "C" {
    pub fn CGContextSetAllowsFontSubpixelPositioning(
        c: CGContextRef,
        allowsFontSubpixelPositioning: bool,
    );
}
unsafe extern "C" {
    pub fn CGContextSetShouldSubpixelQuantizeFonts(
        c: CGContextRef,
        shouldSubpixelQuantizeFonts: bool,
    );
}
unsafe extern "C" {
    pub fn CGContextSetAllowsFontSubpixelQuantization(
        c: CGContextRef,
        allowsFontSubpixelQuantization: bool,
    );
}
unsafe extern "C" {
    pub fn CGContextBeginTransparencyLayer(c: CGContextRef, auxiliaryInfo: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn CGContextBeginTransparencyLayerWithRect(
        c: CGContextRef,
        rect: CGRect,
        auxInfo: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn CGContextEndTransparencyLayer(c: CGContextRef);
}
unsafe extern "C" {
    pub fn CGContextGetUserSpaceToDeviceSpaceTransform(c: CGContextRef) -> CGAffineTransform;
}
unsafe extern "C" {
    pub fn CGContextConvertPointToDeviceSpace(c: CGContextRef, point: CGPoint) -> CGPoint;
}
unsafe extern "C" {
    pub fn CGContextConvertPointToUserSpace(c: CGContextRef, point: CGPoint) -> CGPoint;
}
unsafe extern "C" {
    pub fn CGContextConvertSizeToDeviceSpace(c: CGContextRef, size: CGSize) -> CGSize;
}
unsafe extern "C" {
    pub fn CGContextConvertSizeToUserSpace(c: CGContextRef, size: CGSize) -> CGSize;
}
unsafe extern "C" {
    pub fn CGContextConvertRectToDeviceSpace(c: CGContextRef, rect: CGRect) -> CGRect;
}
unsafe extern "C" {
    pub fn CGContextConvertRectToUserSpace(c: CGContextRef, rect: CGRect) -> CGRect;
}
unsafe extern "C" {
    pub fn CGContextSelectFont(
        c: CGContextRef,
        name: *const ::std::os::raw::c_char,
        size: CGFloat,
        textEncoding: CGTextEncoding,
    );
}
unsafe extern "C" {
    pub fn CGContextShowText(c: CGContextRef, string: *const ::std::os::raw::c_char, length: usize);
}
unsafe extern "C" {
    pub fn CGContextShowTextAtPoint(
        c: CGContextRef,
        x: CGFloat,
        y: CGFloat,
        string: *const ::std::os::raw::c_char,
        length: usize,
    );
}
unsafe extern "C" {
    pub fn CGContextShowGlyphs(c: CGContextRef, g: *const CGGlyph, count: usize);
}
unsafe extern "C" {
    pub fn CGContextShowGlyphsAtPoint(
        c: CGContextRef,
        x: CGFloat,
        y: CGFloat,
        glyphs: *const CGGlyph,
        count: usize,
    );
}
unsafe extern "C" {
    pub fn CGContextShowGlyphsWithAdvances(
        c: CGContextRef,
        glyphs: *const CGGlyph,
        advances: *const CGSize,
        count: usize,
    );
}
unsafe extern "C" {
    pub fn CGContextDrawPDFDocument(
        c: CGContextRef,
        rect: CGRect,
        document: CGPDFDocumentRef,
        page: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn CGRenderingBufferProviderCreate(
        info: *mut ::std::os::raw::c_void,
        size: usize,
        lockPointer: *mut ::std::os::raw::c_void,
        unlockPointer: *mut ::std::os::raw::c_void,
        releaseInfo: *mut ::std::os::raw::c_void,
    ) -> CGRenderingBufferProviderRef;
}
unsafe extern "C" {
    pub fn CGRenderingBufferProviderCreateWithCFData(
        data: CFMutableDataRef,
    ) -> CGRenderingBufferProviderRef;
}
unsafe extern "C" {
    pub fn CGRenderingBufferProviderGetSize(provider: CGRenderingBufferProviderRef) -> usize;
}
unsafe extern "C" {
    pub fn CGRenderingBufferLockBytePtr(
        provider: CGRenderingBufferProviderRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CGRenderingBufferUnlockBytePtr(provider: CGRenderingBufferProviderRef);
}
unsafe extern "C" {
    pub fn CGRenderingBufferProviderGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGBitmapContextCreateWithData(
        data: *mut ::std::os::raw::c_void,
        width: usize,
        height: usize,
        bitsPerComponent: usize,
        bytesPerRow: usize,
        space: CGColorSpaceRef,
        bitmapInfo: CGBitmapInfo,
        releaseCallback: CGBitmapContextReleaseDataCallback,
        releaseInfo: *mut ::std::os::raw::c_void,
    ) -> CGContextRef;
}
unsafe extern "C" {
    pub fn CGBitmapContextCreate(
        data: *mut ::std::os::raw::c_void,
        width: usize,
        height: usize,
        bitsPerComponent: usize,
        bytesPerRow: usize,
        space: CGColorSpaceRef,
        bitmapInfo: CGBitmapInfo,
    ) -> CGContextRef;
}
unsafe extern "C" {
    pub fn CGBitmapContextCreateAdaptive(
        width: usize,
        height: usize,
        auxiliaryInfo: CFDictionaryRef,
        onResolve: *mut ::std::os::raw::c_void,
        onAllocate: *mut ::std::os::raw::c_void,
        onFree: *mut ::std::os::raw::c_void,
        onError: *mut ::std::os::raw::c_void,
    ) -> CGContextRef;
}
unsafe extern "C" {
    pub static kCGAdaptiveMaximumBitDepth: CFStringRef;
}
unsafe extern "C" {
    pub fn CGBitmapContextGetData(context: CGContextRef) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CGBitmapContextGetWidth(context: CGContextRef) -> usize;
}
unsafe extern "C" {
    pub fn CGBitmapContextGetHeight(context: CGContextRef) -> usize;
}
unsafe extern "C" {
    pub fn CGBitmapContextGetBitsPerComponent(context: CGContextRef) -> usize;
}
unsafe extern "C" {
    pub fn CGBitmapContextGetBitsPerPixel(context: CGContextRef) -> usize;
}
unsafe extern "C" {
    pub fn CGBitmapContextGetBytesPerRow(context: CGContextRef) -> usize;
}
unsafe extern "C" {
    pub fn CGBitmapContextGetColorSpace(context: CGContextRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGBitmapContextGetAlphaInfo(context: CGContextRef) -> CGImageAlphaInfo;
}
unsafe extern "C" {
    pub fn CGBitmapContextGetBitmapInfo(context: CGContextRef) -> CGBitmapInfo;
}
unsafe extern "C" {
    pub fn CGBitmapContextCreateImage(context: CGContextRef) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGColorConversionInfoGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGColorConversionInfoCreate(
        src: CGColorSpaceRef,
        dst: CGColorSpaceRef,
    ) -> CGColorConversionInfoRef;
}
unsafe extern "C" {
    pub fn CGColorConversionInfoCreateWithOptions(
        src: CGColorSpaceRef,
        dst: CGColorSpaceRef,
        options: CFDictionaryRef,
    ) -> CGColorConversionInfoRef;
}
unsafe extern "C" {
    pub fn CGColorConversionInfoCreateFromList(
        options: CFDictionaryRef,
        arg1: CGColorSpaceRef,
        arg2: CGColorConversionInfoTransformType,
        arg3: CGColorRenderingIntent,
        ...
    ) -> CGColorConversionInfoRef;
}
unsafe extern "C" {
    pub fn CGColorConversionInfoCreateFromListWithArguments(
        options: CFDictionaryRef,
        arg1: CGColorSpaceRef,
        arg2: CGColorConversionInfoTransformType,
        arg3: CGColorRenderingIntent,
        arg4: va_list,
    ) -> CGColorConversionInfoRef;
}
unsafe extern "C" {
    pub fn CGColorConversionInfoCreateForToneMapping(
        from: CGColorSpaceRef,
        source_headroom: f32,
        to: CGColorSpaceRef,
        target_headroom: f32,
        method: CGToneMapping,
        options: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> CGColorConversionInfoRef;
}
unsafe extern "C" {
    pub fn CGColorConversionInfoConvertData(
        info: CGColorConversionInfoRef,
        width: usize,
        height: usize,
        dst_data: *mut ::std::os::raw::c_void,
        dst_format: CGColorBufferFormat,
        src_data: *const ::std::os::raw::c_void,
        src_format: CGColorBufferFormat,
        options: CFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub static kCGColorConversionBlackPointCompensation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGColorConversionTRCSize: CFStringRef;
}
unsafe extern "C" {
    pub fn CGConvertColorDataWithFormat(
        width: usize,
        height: usize,
        dst_data: *mut ::std::os::raw::c_void,
        dst_format: CGColorDataFormat,
        src_data: *mut ::std::os::raw::c_void,
        src_format: CGColorDataFormat,
        options: CFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGDataConsumerGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGDataConsumerCreate(
        info: *mut ::std::os::raw::c_void,
        cbks: *const CGDataConsumerCallbacks,
    ) -> CGDataConsumerRef;
}
unsafe extern "C" {
    pub fn CGDataConsumerCreateWithURL(url: CFURLRef) -> CGDataConsumerRef;
}
unsafe extern "C" {
    pub fn CGDataConsumerCreateWithCFData(data: CFMutableDataRef) -> CGDataConsumerRef;
}
unsafe extern "C" {
    pub fn CGDataConsumerRetain(consumer: CGDataConsumerRef) -> CGDataConsumerRef;
}
unsafe extern "C" {
    pub fn CGDataConsumerRelease(consumer: CGDataConsumerRef);
}
unsafe extern "C" {
    pub fn CGErrorSetCallback(callback: CGErrorCallback);
}
unsafe extern "C" {
    pub fn CGLayerCreateWithContext(
        context: CGContextRef,
        size: CGSize,
        auxiliaryInfo: CFDictionaryRef,
    ) -> CGLayerRef;
}
unsafe extern "C" {
    pub fn CGLayerRetain(layer: CGLayerRef) -> CGLayerRef;
}
unsafe extern "C" {
    pub fn CGLayerRelease(layer: CGLayerRef);
}
unsafe extern "C" {
    pub fn CGLayerGetSize(layer: CGLayerRef) -> CGSize;
}
unsafe extern "C" {
    pub fn CGLayerGetContext(layer: CGLayerRef) -> CGContextRef;
}
unsafe extern "C" {
    pub fn CGContextDrawLayerInRect(context: CGContextRef, rect: CGRect, layer: CGLayerRef);
}
unsafe extern "C" {
    pub fn CGContextDrawLayerAtPoint(context: CGContextRef, point: CGPoint, layer: CGLayerRef);
}
unsafe extern "C" {
    pub fn CGLayerGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGPDFContentStreamCreateWithPage(page: CGPDFPageRef) -> CGPDFContentStreamRef;
}
unsafe extern "C" {
    pub fn CGPDFContentStreamCreateWithStream(
        stream: CGPDFStreamRef,
        streamResources: CGPDFDictionaryRef,
        parent: CGPDFContentStreamRef,
    ) -> CGPDFContentStreamRef;
}
unsafe extern "C" {
    pub fn CGPDFContentStreamRetain(cs: CGPDFContentStreamRef) -> CGPDFContentStreamRef;
}
unsafe extern "C" {
    pub fn CGPDFContentStreamRelease(cs: CGPDFContentStreamRef);
}
unsafe extern "C" {
    pub fn CGPDFContentStreamGetStreams(cs: CGPDFContentStreamRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGPDFContentStreamGetResource(
        cs: CGPDFContentStreamRef,
        category: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> CGPDFObjectRef;
}
unsafe extern "C" {
    pub fn CGPDFContextCreate(
        consumer: CGDataConsumerRef,
        mediaBox: *const CGRect,
        auxiliaryInfo: CFDictionaryRef,
    ) -> CGContextRef;
}
unsafe extern "C" {
    pub fn CGPDFContextCreateWithURL(
        url: CFURLRef,
        mediaBox: *const CGRect,
        auxiliaryInfo: CFDictionaryRef,
    ) -> CGContextRef;
}
unsafe extern "C" {
    pub fn CGPDFContextClose(context: CGContextRef);
}
unsafe extern "C" {
    pub fn CGPDFContextBeginPage(context: CGContextRef, pageInfo: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn CGPDFContextEndPage(context: CGContextRef);
}
unsafe extern "C" {
    pub fn CGPDFContextAddDocumentMetadata(context: CGContextRef, metadata: CFDataRef);
}
unsafe extern "C" {
    pub fn CGPDFContextSetParentTree(
        context: CGContextRef,
        parentTreeDictionary: CGPDFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn CGPDFContextSetIDTree(context: CGContextRef, IDTreeDictionary: CGPDFDictionaryRef);
}
unsafe extern "C" {
    pub fn CGPDFContextSetPageTagStructureTree(
        context: CGContextRef,
        pageTagStructureTreeDictionary: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn CGPDFContextSetURLForRect(context: CGContextRef, url: CFURLRef, rect: CGRect);
}
unsafe extern "C" {
    pub fn CGPDFContextAddDestinationAtPoint(
        context: CGContextRef,
        name: CFStringRef,
        point: CGPoint,
    );
}
unsafe extern "C" {
    pub fn CGPDFContextSetDestinationForRect(
        context: CGContextRef,
        name: CFStringRef,
        rect: CGRect,
    );
}
unsafe extern "C" {
    pub static kCGPDFContextMediaBox: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextCropBox: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextBleedBox: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextTrimBox: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextArtBox: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextTitle: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextAuthor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextSubject: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextKeywords: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextCreator: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextOwnerPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextUserPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextEncryptionKeyLength: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextAllowsPrinting: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextAllowsCopying: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextOutputIntent: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFXOutputIntentSubtype: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFXOutputConditionIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFXOutputCondition: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFXRegistryName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFXInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFXDestinationOutputProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextOutputIntents: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextAccessPermissions: CFStringRef;
}
unsafe extern "C" {
    pub fn CGPDFContextSetOutline(context: CGContextRef, outline: CFDictionaryRef);
}
unsafe extern "C" {
    pub static kCGPDFContextCreateLinearizedPDF: CFStringRef;
}
unsafe extern "C" {
    pub static kCGPDFContextCreatePDFA: CFStringRef;
}
unsafe extern "C" {
    pub fn CGPDFTagTypeGetName(tagType: CGPDFTagType) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static mut kCGPDFTagPropertyActualText: CGPDFTagProperty;
}
unsafe extern "C" {
    pub static mut kCGPDFTagPropertyAlternativeText: CGPDFTagProperty;
}
unsafe extern "C" {
    pub static mut kCGPDFTagPropertyTitleText: CGPDFTagProperty;
}
unsafe extern "C" {
    pub static mut kCGPDFTagPropertyLanguageText: CGPDFTagProperty;
}
unsafe extern "C" {
    pub fn CGPDFContextBeginTag(
        context: CGContextRef,
        tagType: CGPDFTagType,
        tagProperties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn CGPDFContextEndTag(context: CGContextRef);
}
unsafe extern "C" {
    pub fn CGPDFScannerCreate(
        cs: CGPDFContentStreamRef,
        table: CGPDFOperatorTableRef,
        info: *mut ::std::os::raw::c_void,
    ) -> CGPDFScannerRef;
}
unsafe extern "C" {
    pub fn CGPDFScannerRetain(scanner: CGPDFScannerRef) -> CGPDFScannerRef;
}
unsafe extern "C" {
    pub fn CGPDFScannerRelease(scanner: CGPDFScannerRef);
}
unsafe extern "C" {
    pub fn CGPDFScannerScan(scanner: CGPDFScannerRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerGetContentStream(scanner: CGPDFScannerRef) -> CGPDFContentStreamRef;
}
unsafe extern "C" {
    pub fn CGPDFScannerPopObject(scanner: CGPDFScannerRef, value: *mut CGPDFObjectRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerPopBoolean(scanner: CGPDFScannerRef, value: *mut CGPDFBoolean) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerPopInteger(scanner: CGPDFScannerRef, value: *mut CGPDFInteger) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerPopNumber(scanner: CGPDFScannerRef, value: *mut CGPDFReal) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerPopName(
        scanner: CGPDFScannerRef,
        value: *mut *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerPopString(scanner: CGPDFScannerRef, value: *mut CGPDFStringRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerPopArray(scanner: CGPDFScannerRef, value: *mut CGPDFArrayRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerPopDictionary(
        scanner: CGPDFScannerRef,
        value: *mut CGPDFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerPopStream(scanner: CGPDFScannerRef, value: *mut CGPDFStreamRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPDFScannerStop(s: CGPDFScannerRef);
}
unsafe extern "C" {
    pub fn CGPDFOperatorTableCreate() -> CGPDFOperatorTableRef;
}
unsafe extern "C" {
    pub fn CGPDFOperatorTableRetain(table: CGPDFOperatorTableRef) -> CGPDFOperatorTableRef;
}
unsafe extern "C" {
    pub fn CGPDFOperatorTableRelease(table: CGPDFOperatorTableRef);
}
unsafe extern "C" {
    pub fn CGPDFOperatorTableSetCallback(
        table: CGPDFOperatorTableRef,
        name: *const ::std::os::raw::c_char,
        callback: CGPDFOperatorCallback,
    );
}
unsafe extern "C" {
    pub static kCGWindowNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowStoreType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowLayer: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowBounds: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowSharingState: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowOwnerPID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowMemoryUsage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowWorkspace: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowOwnerName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowIsOnscreen: CFStringRef;
}
unsafe extern "C" {
    pub static kCGWindowBackingLocationVideoMemory: CFStringRef;
}
unsafe extern "C" {
    pub fn CGWindowListCopyWindowInfo(
        option: CGWindowListOption,
        relativeToWindow: CGWindowID,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGWindowListCreate(
        option: CGWindowListOption,
        relativeToWindow: CGWindowID,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGWindowListCreateDescriptionFromArray(windowArray: CFArrayRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGWindowListCreateImage(
        screenBounds: CGRect,
        listOption: CGWindowListOption,
        windowID: CGWindowID,
        imageOption: CGWindowImageOption,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGWindowListCreateImageFromArray(
        screenBounds: CGRect,
        windowArray: CFArrayRef,
        imageOption: CGWindowImageOption,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGPreflightScreenCaptureAccess() -> bool;
}
unsafe extern "C" {
    pub fn CGRequestScreenCaptureAccess() -> bool;
}
unsafe extern "C" {
    pub fn CGWindowLevelForKey(key: CGWindowLevelKey) -> CGWindowLevel;
}
unsafe extern "C" {
    pub fn CGMainDisplayID() -> CGDirectDisplayID;
}
unsafe extern "C" {
    pub fn CGGetDisplaysWithPoint(
        point: CGPoint,
        maxDisplays: u32,
        displays: *mut CGDirectDisplayID,
        matchingDisplayCount: *mut u32,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGGetDisplaysWithRect(
        rect: CGRect,
        maxDisplays: u32,
        displays: *mut CGDirectDisplayID,
        matchingDisplayCount: *mut u32,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGGetDisplaysWithOpenGLDisplayMask(
        mask: CGOpenGLDisplayMask,
        maxDisplays: u32,
        displays: *mut CGDirectDisplayID,
        matchingDisplayCount: *mut u32,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGGetActiveDisplayList(
        maxDisplays: u32,
        activeDisplays: *mut CGDirectDisplayID,
        displayCount: *mut u32,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGGetOnlineDisplayList(
        maxDisplays: u32,
        onlineDisplays: *mut CGDirectDisplayID,
        displayCount: *mut u32,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayIDToOpenGLDisplayMask(display: CGDirectDisplayID) -> CGOpenGLDisplayMask;
}
unsafe extern "C" {
    pub fn CGOpenGLDisplayMaskToDisplayID(mask: CGOpenGLDisplayMask) -> CGDirectDisplayID;
}
unsafe extern "C" {
    pub fn CGDisplayBounds(display: CGDirectDisplayID) -> CGRect;
}
unsafe extern "C" {
    pub fn CGDisplayPixelsWide(display: CGDirectDisplayID) -> usize;
}
unsafe extern "C" {
    pub fn CGDisplayPixelsHigh(display: CGDirectDisplayID) -> usize;
}
unsafe extern "C" {
    pub fn CGDisplayCopyAllDisplayModes(
        display: CGDirectDisplayID,
        options: CFDictionaryRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub static kCGDisplayShowDuplicateLowResolutionModes: CFStringRef;
}
unsafe extern "C" {
    pub fn CGDisplayCopyDisplayMode(display: CGDirectDisplayID) -> CGDisplayModeRef;
}
unsafe extern "C" {
    pub fn CGDisplaySetDisplayMode(
        display: CGDirectDisplayID,
        mode: CGDisplayModeRef,
        options: CFDictionaryRef,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayModeGetWidth(mode: CGDisplayModeRef) -> usize;
}
unsafe extern "C" {
    pub fn CGDisplayModeGetHeight(mode: CGDisplayModeRef) -> usize;
}
unsafe extern "C" {
    pub fn CGDisplayModeCopyPixelEncoding(mode: CGDisplayModeRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGDisplayModeGetRefreshRate(mode: CGDisplayModeRef) -> f64;
}
unsafe extern "C" {
    pub fn CGDisplayModeGetIOFlags(mode: CGDisplayModeRef) -> u32;
}
unsafe extern "C" {
    pub fn CGDisplayModeGetIODisplayModeID(mode: CGDisplayModeRef) -> i32;
}
unsafe extern "C" {
    pub fn CGDisplayModeIsUsableForDesktopGUI(mode: CGDisplayModeRef) -> bool;
}
unsafe extern "C" {
    pub fn CGDisplayModeGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGDisplayModeRetain(mode: CGDisplayModeRef) -> CGDisplayModeRef;
}
unsafe extern "C" {
    pub fn CGDisplayModeRelease(mode: CGDisplayModeRef);
}
unsafe extern "C" {
    pub fn CGDisplayModeGetPixelWidth(mode: CGDisplayModeRef) -> usize;
}
unsafe extern "C" {
    pub fn CGDisplayModeGetPixelHeight(mode: CGDisplayModeRef) -> usize;
}
unsafe extern "C" {
    pub fn CGSetDisplayTransferByFormula(
        display: CGDirectDisplayID,
        redMin: CGGammaValue,
        redMax: CGGammaValue,
        redGamma: CGGammaValue,
        greenMin: CGGammaValue,
        greenMax: CGGammaValue,
        greenGamma: CGGammaValue,
        blueMin: CGGammaValue,
        blueMax: CGGammaValue,
        blueGamma: CGGammaValue,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGGetDisplayTransferByFormula(
        display: CGDirectDisplayID,
        redMin: *mut CGGammaValue,
        redMax: *mut CGGammaValue,
        redGamma: *mut CGGammaValue,
        greenMin: *mut CGGammaValue,
        greenMax: *mut CGGammaValue,
        greenGamma: *mut CGGammaValue,
        blueMin: *mut CGGammaValue,
        blueMax: *mut CGGammaValue,
        blueGamma: *mut CGGammaValue,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayGammaTableCapacity(display: CGDirectDisplayID) -> u32;
}
unsafe extern "C" {
    pub fn CGSetDisplayTransferByTable(
        display: CGDirectDisplayID,
        tableSize: u32,
        redTable: *const CGGammaValue,
        greenTable: *const CGGammaValue,
        blueTable: *const CGGammaValue,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGGetDisplayTransferByTable(
        display: CGDirectDisplayID,
        capacity: u32,
        redTable: *mut CGGammaValue,
        greenTable: *mut CGGammaValue,
        blueTable: *mut CGGammaValue,
        sampleCount: *mut u32,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGSetDisplayTransferByByteTable(
        display: CGDirectDisplayID,
        tableSize: u32,
        redTable: *const u8,
        greenTable: *const u8,
        blueTable: *const u8,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayRestoreColorSyncSettings();
}
unsafe extern "C" {
    pub fn CGDisplayIsCaptured(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayCapture(display: CGDirectDisplayID) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayCaptureWithOptions(
        display: CGDirectDisplayID,
        options: CGCaptureOptions,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayRelease(display: CGDirectDisplayID) -> CGError;
}
unsafe extern "C" {
    pub fn CGCaptureAllDisplays() -> CGError;
}
unsafe extern "C" {
    pub fn CGCaptureAllDisplaysWithOptions(options: CGCaptureOptions) -> CGError;
}
unsafe extern "C" {
    pub fn CGReleaseAllDisplays() -> CGError;
}
unsafe extern "C" {
    pub fn CGShieldingWindowID(display: CGDirectDisplayID) -> CGWindowID;
}
unsafe extern "C" {
    pub fn CGShieldingWindowLevel() -> CGWindowLevel;
}
unsafe extern "C" {
    pub fn CGDisplayCreateImage(displayID: CGDirectDisplayID) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGDisplayCreateImageForRect(display: CGDirectDisplayID, rect: CGRect) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGDisplayHideCursor(display: CGDirectDisplayID) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayShowCursor(display: CGDirectDisplayID) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayMoveCursorToPoint(display: CGDirectDisplayID, point: CGPoint) -> CGError;
}
unsafe extern "C" {
    pub fn CGGetLastMouseDelta(deltaX: *mut i32, deltaY: *mut i32);
}
unsafe extern "C" {
    pub fn CGDisplayGetDrawingContext(display: CGDirectDisplayID) -> CGContextRef;
}
unsafe extern "C" {
    pub fn CGDisplayAvailableModes(dsp: CGDirectDisplayID) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGDisplayBestModeForParameters(
        display: CGDirectDisplayID,
        bitsPerPixel: usize,
        width: usize,
        height: usize,
        exactMatch: *mut boolean_t,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGDisplayBestModeForParametersAndRefreshRate(
        display: CGDirectDisplayID,
        bitsPerPixel: usize,
        width: usize,
        height: usize,
        refreshRate: CGRefreshRate,
        exactMatch: *mut boolean_t,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGDisplayCurrentMode(display: CGDirectDisplayID) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGDisplaySwitchToMode(display: CGDirectDisplayID, mode: CFDictionaryRef) -> CGError;
}
unsafe extern "C" {
    pub fn CGBeginDisplayConfiguration(config: *mut CGDisplayConfigRef) -> CGError;
}
unsafe extern "C" {
    pub fn CGConfigureDisplayOrigin(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        x: i32,
        y: i32,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGConfigureDisplayWithDisplayMode(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        mode: CGDisplayModeRef,
        options: CFDictionaryRef,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGConfigureDisplayStereoOperation(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        stereo: boolean_t,
        forceBlueLine: boolean_t,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGConfigureDisplayMirrorOfDisplay(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        master: CGDirectDisplayID,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGCancelDisplayConfiguration(config: CGDisplayConfigRef) -> CGError;
}
unsafe extern "C" {
    pub fn CGCompleteDisplayConfiguration(
        config: CGDisplayConfigRef,
        option: CGConfigureOption,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGRestorePermanentDisplayConfiguration();
}
unsafe extern "C" {
    pub fn CGDisplayRegisterReconfigurationCallback(
        callback: CGDisplayReconfigurationCallBack,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayRemoveReconfigurationCallback(
        callback: CGDisplayReconfigurationCallBack,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplaySetStereoOperation(
        display: CGDirectDisplayID,
        stereo: boolean_t,
        forceBlueLine: boolean_t,
        option: CGConfigureOption,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayIsActive(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayIsAsleep(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayIsOnline(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayIsMain(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayIsBuiltin(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayIsInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayIsAlwaysInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayIsInHWMirrorSet(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayMirrorsDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
}
unsafe extern "C" {
    pub fn CGDisplayUsesOpenGLAcceleration(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayIsStereo(display: CGDirectDisplayID) -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayPrimaryDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
}
unsafe extern "C" {
    pub fn CGDisplayUnitNumber(display: CGDirectDisplayID) -> u32;
}
unsafe extern "C" {
    pub fn CGDisplayVendorNumber(display: CGDirectDisplayID) -> u32;
}
unsafe extern "C" {
    pub fn CGDisplayModelNumber(display: CGDirectDisplayID) -> u32;
}
unsafe extern "C" {
    pub fn CGDisplaySerialNumber(display: CGDirectDisplayID) -> u32;
}
unsafe extern "C" {
    pub fn CGDisplayIOServicePort(display: CGDirectDisplayID) -> io_service_t;
}
unsafe extern "C" {
    pub fn CGDisplayScreenSize(display: CGDirectDisplayID) -> CGSize;
}
unsafe extern "C" {
    pub fn CGDisplayRotation(display: CGDirectDisplayID) -> f64;
}
unsafe extern "C" {
    pub fn CGDisplayCopyColorSpace(display: CGDirectDisplayID) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CGConfigureDisplayMode(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        mode: CFDictionaryRef,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGConfigureDisplayFadeEffect(
        config: CGDisplayConfigRef,
        fadeOutSeconds: CGDisplayFadeInterval,
        fadeInSeconds: CGDisplayFadeInterval,
        fadeRed: f32,
        fadeGreen: f32,
        fadeBlue: f32,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGAcquireDisplayFadeReservation(
        seconds: CGDisplayReservationInterval,
        token: *mut CGDisplayFadeReservationToken,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGReleaseDisplayFadeReservation(token: CGDisplayFadeReservationToken) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayFade(
        token: CGDisplayFadeReservationToken,
        duration: CGDisplayFadeInterval,
        startBlend: CGDisplayBlendFraction,
        endBlend: CGDisplayBlendFraction,
        redBlend: f32,
        greenBlend: f32,
        blueBlend: f32,
        synchronous: boolean_t,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayFadeOperationInProgress() -> boolean_t;
}
unsafe extern "C" {
    pub fn CGDisplayStreamUpdateGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGDisplayStreamUpdateGetRects(
        updateRef: CGDisplayStreamUpdateRef,
        rectType: CGDisplayStreamUpdateRectType,
        rectCount: *mut usize,
    ) -> *const CGRect;
}
unsafe extern "C" {
    pub fn CGDisplayStreamUpdateCreateMergedUpdate(
        firstUpdate: CGDisplayStreamUpdateRef,
        secondUpdate: CGDisplayStreamUpdateRef,
    ) -> CGDisplayStreamUpdateRef;
}
unsafe extern "C" {
    pub fn CGDisplayStreamUpdateGetMovedRectsDelta(
        updateRef: CGDisplayStreamUpdateRef,
        dx: *mut CGFloat,
        dy: *mut CGFloat,
    );
}
unsafe extern "C" {
    pub fn CGDisplayStreamUpdateGetDropCount(updateRef: CGDisplayStreamUpdateRef) -> usize;
}
unsafe extern "C" {
    pub static kCGDisplayStreamSourceRect: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamDestinationRect: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamPreserveAspectRatio: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamMinimumFrameTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamShowCursor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamQueueDepth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamYCbCrMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamYCbCrMatrix_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamYCbCrMatrix_ITU_R_601_4: CFStringRef;
}
unsafe extern "C" {
    pub static kCGDisplayStreamYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
}
unsafe extern "C" {
    pub fn CGDisplayStreamGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGDisplayStreamCreate(
        display: CGDirectDisplayID,
        outputWidth: usize,
        outputHeight: usize,
        pixelFormat: i32,
        properties: CFDictionaryRef,
        handler: CGDisplayStreamFrameAvailableHandler,
    ) -> CGDisplayStreamRef;
}
unsafe extern "C" {
    pub fn CGDisplayStreamCreateWithDispatchQueue(
        display: CGDirectDisplayID,
        outputWidth: usize,
        outputHeight: usize,
        pixelFormat: i32,
        properties: CFDictionaryRef,
        queue: NSObject,
        handler: CGDisplayStreamFrameAvailableHandler,
    ) -> CGDisplayStreamRef;
}
unsafe extern "C" {
    pub fn CGDisplayStreamStart(displayStream: CGDisplayStreamRef) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayStreamStop(displayStream: CGDisplayStreamRef) -> CGError;
}
unsafe extern "C" {
    pub fn CGDisplayStreamGetRunLoopSource(displayStream: CGDisplayStreamRef)
        -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn CGRegisterScreenRefreshCallback(
        callback: CGScreenRefreshCallback,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGUnregisterScreenRefreshCallback(
        callback: CGScreenRefreshCallback,
        userInfo: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CGWaitForScreenRefreshRects(rects: *mut *mut CGRect, count: *mut u32) -> CGError;
}
unsafe extern "C" {
    pub fn CGScreenRegisterMoveCallback(
        callback: CGScreenUpdateMoveCallback,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGScreenUnregisterMoveCallback(
        callback: CGScreenUpdateMoveCallback,
        userInfo: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CGWaitForScreenUpdateRects(
        requestedOperations: CGScreenUpdateOperation,
        currentOperation: *mut CGScreenUpdateOperation,
        rects: *mut *mut CGRect,
        rectCount: *mut usize,
        delta: *mut CGScreenUpdateMoveDelta,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGReleaseScreenRefreshRects(rects: *mut CGRect);
}
unsafe extern "C" {
    pub fn CGCursorIsVisible() -> boolean_t;
}
unsafe extern "C" {
    pub fn CGCursorIsDrawnInFramebuffer() -> boolean_t;
}
unsafe extern "C" {
    pub fn CGWarpMouseCursorPosition(newCursorPosition: CGPoint) -> CGError;
}
unsafe extern "C" {
    pub fn CGAssociateMouseAndMouseCursorPosition(connected: boolean_t) -> CGError;
}
unsafe extern "C" {
    pub fn CGWindowServerCreateServerPort() -> CFMachPortRef;
}
unsafe extern "C" {
    pub fn CGEnableEventStateCombining(combineState: boolean_t) -> CGError;
}
unsafe extern "C" {
    pub fn CGInhibitLocalEvents(inhibit: boolean_t) -> CGError;
}
unsafe extern "C" {
    pub fn CGPostMouseEvent(
        mouseCursorPosition: CGPoint,
        updateMouseCursorPosition: boolean_t,
        buttonCount: CGButtonCount,
        mouseButtonDown: boolean_t,
        ...
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGPostScrollWheelEvent(wheelCount: CGWheelCount, wheel1: i32, ...) -> CGError;
}
unsafe extern "C" {
    pub fn CGPostKeyboardEvent(
        keyChar: CGCharCode,
        virtualKey: CGKeyCode,
        keyDown: boolean_t,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGSetLocalEventsFilterDuringSuppressionState(
        filter: CGEventFilterMask,
        state: CGEventSuppressionState,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGSetLocalEventsSuppressionInterval(seconds: CFTimeInterval) -> CGError;
}
unsafe extern "C" {
    pub fn CGWindowServerCFMachPort() -> CFMachPortRef;
}
unsafe extern "C" {
    pub fn CGEventGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGEventCreate(source: CGEventSourceRef) -> CGEventRef;
}
unsafe extern "C" {
    pub fn CGEventCreateData(allocator: CFAllocatorRef, event: CGEventRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CGEventCreateFromData(allocator: CFAllocatorRef, data: CFDataRef) -> CGEventRef;
}
unsafe extern "C" {
    pub fn CGEventCreateMouseEvent(
        source: CGEventSourceRef,
        mouseType: CGEventType,
        mouseCursorPosition: CGPoint,
        mouseButton: CGMouseButton,
    ) -> CGEventRef;
}
unsafe extern "C" {
    pub fn CGEventCreateKeyboardEvent(
        source: CGEventSourceRef,
        virtualKey: CGKeyCode,
        keyDown: bool,
    ) -> CGEventRef;
}
unsafe extern "C" {
    pub fn CGEventCreateScrollWheelEvent(
        source: CGEventSourceRef,
        units: CGScrollEventUnit,
        wheelCount: u32,
        wheel1: i32,
        ...
    ) -> CGEventRef;
}
unsafe extern "C" {
    pub fn CGEventCreateScrollWheelEvent2(
        source: CGEventSourceRef,
        units: CGScrollEventUnit,
        wheelCount: u32,
        wheel1: i32,
        wheel2: i32,
        wheel3: i32,
    ) -> CGEventRef;
}
unsafe extern "C" {
    pub fn CGEventCreateCopy(event: CGEventRef) -> CGEventRef;
}
unsafe extern "C" {
    pub fn CGEventCreateSourceFromEvent(event: CGEventRef) -> CGEventSourceRef;
}
unsafe extern "C" {
    pub fn CGEventSetSource(event: CGEventRef, source: CGEventSourceRef);
}
unsafe extern "C" {
    pub fn CGEventGetType(event: CGEventRef) -> CGEventType;
}
unsafe extern "C" {
    pub fn CGEventSetType(event: CGEventRef, type_: CGEventType);
}
unsafe extern "C" {
    pub fn CGEventGetTimestamp(event: CGEventRef) -> CGEventTimestamp;
}
unsafe extern "C" {
    pub fn CGEventSetTimestamp(event: CGEventRef, timestamp: CGEventTimestamp);
}
unsafe extern "C" {
    pub fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
}
unsafe extern "C" {
    pub fn CGEventGetUnflippedLocation(event: CGEventRef) -> CGPoint;
}
unsafe extern "C" {
    pub fn CGEventSetLocation(event: CGEventRef, location: CGPoint);
}
unsafe extern "C" {
    pub fn CGEventGetFlags(event: CGEventRef) -> CGEventFlags;
}
unsafe extern "C" {
    pub fn CGEventSetFlags(event: CGEventRef, flags: CGEventFlags);
}
unsafe extern "C" {
    pub fn CGEventKeyboardGetUnicodeString(
        event: CGEventRef,
        maxStringLength: UniCharCount,
        actualStringLength: *mut UniCharCount,
        unicodeString: *mut UniChar,
    );
}
unsafe extern "C" {
    pub fn CGEventKeyboardSetUnicodeString(
        event: CGEventRef,
        stringLength: UniCharCount,
        unicodeString: *const UniChar,
    );
}
unsafe extern "C" {
    pub fn CGEventGetIntegerValueField(event: CGEventRef, field: CGEventField) -> i64;
}
unsafe extern "C" {
    pub fn CGEventSetIntegerValueField(event: CGEventRef, field: CGEventField, value: i64);
}
unsafe extern "C" {
    pub fn CGEventGetDoubleValueField(event: CGEventRef, field: CGEventField) -> f64;
}
unsafe extern "C" {
    pub fn CGEventSetDoubleValueField(event: CGEventRef, field: CGEventField, value: f64);
}
unsafe extern "C" {
    pub fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: CGEventTapCallBack,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> CFMachPortRef;
}
unsafe extern "C" {
    pub fn CGEventTapCreateForPSN(
        processSerialNumber: *mut ::std::os::raw::c_void,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: CGEventTapCallBack,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> CFMachPortRef;
}
unsafe extern "C" {
    pub fn CGEventTapCreateForPid(
        pid: pid_t,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: CGEventTapCallBack,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> CFMachPortRef;
}
unsafe extern "C" {
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
}
unsafe extern "C" {
    pub fn CGEventTapIsEnabled(tap: CFMachPortRef) -> bool;
}
unsafe extern "C" {
    pub fn CGEventTapPostEvent(proxy: CGEventTapProxy, event: CGEventRef);
}
unsafe extern "C" {
    pub fn CGEventPost(tap: CGEventTapLocation, event: CGEventRef);
}
unsafe extern "C" {
    pub fn CGEventPostToPSN(processSerialNumber: *mut ::std::os::raw::c_void, event: CGEventRef);
}
unsafe extern "C" {
    pub fn CGEventPostToPid(pid: pid_t, event: CGEventRef);
}
unsafe extern "C" {
    pub fn CGGetEventTapList(
        maxNumberOfTaps: u32,
        tapList: *mut CGEventTapInformation,
        eventTapCount: *mut u32,
    ) -> CGError;
}
unsafe extern "C" {
    pub fn CGPreflightListenEventAccess() -> bool;
}
unsafe extern "C" {
    pub fn CGRequestListenEventAccess() -> bool;
}
unsafe extern "C" {
    pub fn CGPreflightPostEventAccess() -> bool;
}
unsafe extern "C" {
    pub fn CGRequestPostEventAccess() -> bool;
}
unsafe extern "C" {
    pub fn CGEventSourceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGEventSourceCreate(stateID: CGEventSourceStateID) -> CGEventSourceRef;
}
unsafe extern "C" {
    pub fn CGEventSourceGetKeyboardType(source: CGEventSourceRef) -> CGEventSourceKeyboardType;
}
unsafe extern "C" {
    pub fn CGEventSourceSetKeyboardType(
        source: CGEventSourceRef,
        keyboardType: CGEventSourceKeyboardType,
    );
}
unsafe extern "C" {
    pub fn CGEventSourceGetPixelsPerLine(source: CGEventSourceRef) -> f64;
}
unsafe extern "C" {
    pub fn CGEventSourceSetPixelsPerLine(source: CGEventSourceRef, pixelsPerLine: f64);
}
unsafe extern "C" {
    pub fn CGEventSourceGetSourceStateID(source: CGEventSourceRef) -> CGEventSourceStateID;
}
unsafe extern "C" {
    pub fn CGEventSourceButtonState(stateID: CGEventSourceStateID, button: CGMouseButton) -> bool;
}
unsafe extern "C" {
    pub fn CGEventSourceKeyState(stateID: CGEventSourceStateID, key: CGKeyCode) -> bool;
}
unsafe extern "C" {
    pub fn CGEventSourceFlagsState(stateID: CGEventSourceStateID) -> CGEventFlags;
}
unsafe extern "C" {
    pub fn CGEventSourceSecondsSinceLastEventType(
        stateID: CGEventSourceStateID,
        eventType: CGEventType,
    ) -> CFTimeInterval;
}
unsafe extern "C" {
    pub fn CGEventSourceCounterForEventType(
        stateID: CGEventSourceStateID,
        eventType: CGEventType,
    ) -> u32;
}
unsafe extern "C" {
    pub fn CGEventSourceSetUserData(source: CGEventSourceRef, userData: i64);
}
unsafe extern "C" {
    pub fn CGEventSourceGetUserData(source: CGEventSourceRef) -> i64;
}
unsafe extern "C" {
    pub fn CGEventSourceSetLocalEventsFilterDuringSuppressionState(
        source: CGEventSourceRef,
        filter: CGEventFilterMask,
        state: CGEventSuppressionState,
    );
}
unsafe extern "C" {
    pub fn CGEventSourceGetLocalEventsFilterDuringSuppressionState(
        source: CGEventSourceRef,
        state: CGEventSuppressionState,
    ) -> CGEventFilterMask;
}
unsafe extern "C" {
    pub fn CGEventSourceSetLocalEventsSuppressionInterval(
        source: CGEventSourceRef,
        seconds: CFTimeInterval,
    );
}
unsafe extern "C" {
    pub fn CGEventSourceGetLocalEventsSuppressionInterval(
        source: CGEventSourceRef,
    ) -> CFTimeInterval;
}
unsafe extern "C" {
    pub fn CGPSConverterCreate(
        info: *mut ::std::os::raw::c_void,
        callbacks: *const CGPSConverterCallbacks,
        options: CFDictionaryRef,
    ) -> CGPSConverterRef;
}
unsafe extern "C" {
    pub fn CGPSConverterConvert(
        converter: CGPSConverterRef,
        provider: CGDataProviderRef,
        consumer: CGDataConsumerRef,
        options: CFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGPSConverterAbort(converter: CGPSConverterRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPSConverterIsConverting(converter: CGPSConverterRef) -> bool;
}
unsafe extern "C" {
    pub fn CGPSConverterGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGSessionCopyCurrentDictionary() -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGDirectDisplayCopyCurrentMetalDevice(display: CGDirectDisplayID) -> *mut u64;
}

unsafe impl objc2::encode::RefEncode for __IOSurface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOSurface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOSurface", &[]);
}
unsafe impl objc2::encode::RefEncode for CGContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGContext", &[]);
}
unsafe impl objc2::encode::RefEncode for CGColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGColor", &[]);
}
unsafe impl objc2::encode::RefEncode for CGColorSpace {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGColorSpace {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGColorSpace", &[]);
}
unsafe impl objc2::encode::RefEncode for CGDataProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGDataProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGDataProvider", &[]);
}
unsafe impl objc2::encode::RefEncode for CGDataProviderSequentialCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGDataProviderSequentialCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGDataProviderSequentialCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for CGDataProviderDirectCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGDataProviderDirectCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGDataProviderDirectCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPattern {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPattern {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPattern", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPatternCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPatternCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPatternCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for CGFont {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGFont {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGFont", &[]);
}
unsafe impl objc2::encode::RefEncode for CGGradient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGGradient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGGradient", &[]);
}
unsafe impl objc2::encode::RefEncode for CGImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGImage", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPath {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPath {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPath", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPathElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPathElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPathElement", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFDocument {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFDocument {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFDocument", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFPage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFPage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFPage", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFDictionary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFDictionary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFDictionary", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFArray", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFObject", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFStream", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFString {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFString {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFString", &[]);
}
unsafe impl objc2::encode::RefEncode for CGShading {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGShading {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGShading", &[]);
}
unsafe impl objc2::encode::RefEncode for CGFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGFunction", &[]);
}
unsafe impl objc2::encode::RefEncode for CGFunctionCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGFunctionCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGFunctionCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for CGContentToneMappingInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGContentToneMappingInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGContentToneMappingInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CGRenderingBufferProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGRenderingBufferProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGRenderingBufferProvider", &[]);
}
unsafe impl objc2::encode::RefEncode for CGContentInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGContentInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGContentInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CGColorConversionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGColorConversionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGColorConversionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CGColorBufferFormat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGColorBufferFormat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGColorBufferFormat", &[]);
}
unsafe impl objc2::encode::RefEncode for CGColorDataFormat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGColorDataFormat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGColorDataFormat", &[]);
}
unsafe impl objc2::encode::RefEncode for CGDataConsumer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGDataConsumer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGDataConsumer", &[]);
}
unsafe impl objc2::encode::RefEncode for CGDataConsumerCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGDataConsumerCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGDataConsumerCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for CGLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGLayer", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFContentStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFContentStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFContentStream", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFOperatorTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFOperatorTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFOperatorTable", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPDFScanner {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPDFScanner {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPDFScanner", &[]);
}
unsafe impl objc2::encode::RefEncode for CGDisplayMode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGDisplayMode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGDisplayMode", &[]);
}
unsafe impl objc2::encode::RefEncode for CGDeviceColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGDeviceColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGDeviceColor", &[]);
}
unsafe impl objc2::encode::RefEncode for _CGDisplayConfigRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CGDisplayConfigRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CGDisplayConfigRef", &[]);
}
unsafe impl objc2::encode::RefEncode for CGDisplayStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGDisplayStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGDisplayStream", &[]);
}
unsafe impl objc2::encode::RefEncode for CGDisplayStreamUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGDisplayStreamUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGDisplayStreamUpdate", &[]);
}
unsafe impl objc2::encode::RefEncode for CGScreenUpdateMoveDelta {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGScreenUpdateMoveDelta {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGScreenUpdateMoveDelta", &[]);
}
unsafe impl objc2::encode::RefEncode for __CGEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CGEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CGEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for __CGEventTapProxy {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CGEventTapProxy {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CGEventTapProxy", &[]);
}
unsafe impl objc2::encode::RefEncode for __CGEventTapInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CGEventTapInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CGEventTapInformation", &[]);
}
unsafe impl objc2::encode::RefEncode for __CGEventSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CGEventSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CGEventSource", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPSConverter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPSConverter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPSConverter", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPSConverterCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPSConverterCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPSConverterCallbacks", &[]);
}
