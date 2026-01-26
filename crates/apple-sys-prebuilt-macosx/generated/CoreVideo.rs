#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::OpenCL::*;
#[allow(unused_imports)]
use crate::OpenGL::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CVOptionFlags = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVSMPTETime {
    pub subframes: SInt16,
    pub subframeDivisor: SInt16,
    pub counter: UInt32,
    pub type_: UInt32,
    pub flags: UInt32,
    pub hours: SInt16,
    pub minutes: SInt16,
    pub seconds: SInt16,
    pub frames: SInt16,
}
pub type CVSMPTETimeType = u32;
pub type CVSMPTETimeFlags = u32;
pub type CVTimeFlags = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVTime {
    pub timeValue: i64,
    pub timeScale: i32,
    pub flags: i32,
}
pub type CVTimeStampFlags = u64;
pub type CVReturn = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVDisplayLink {
    _unused: [u8; 0],
}
pub type CVDisplayLinkRef = *mut __CVDisplayLink;
pub type CVDisplayLinkOutputCallback = ::std::option::Option<
    unsafe extern "C" fn(
        displayLink: CVDisplayLinkRef,
        inNow: *const CVTimeStamp,
        inOutputTime: *const CVTimeStamp,
        flagsIn: CVOptionFlags,
        flagsOut: *mut CVOptionFlags,
        displayLinkContext: *mut ::std::os::raw::c_void,
    ) -> CVReturn,
>;
pub type CVDisplayLinkOutputHandler = *mut ::std::os::raw::c_void;
pub type CVAttachmentMode = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVBuffer {
    _unused: [u8; 0],
}
pub type CVImageBufferRef = CVBufferRef;
pub type CVPixelBufferLockFlags = CVOptionFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVPlanarComponentInfo {
    pub offset: i32,
    pub rowBytes: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVPlanarPixelBufferInfo {
    pub componentInfo: [CVPlanarComponentInfo; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVPlanarPixelBufferInfo_YCbCrPlanar {
    pub componentInfoY: CVPlanarComponentInfo,
    pub componentInfoCb: CVPlanarComponentInfo,
    pub componentInfoCr: CVPlanarComponentInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVPlanarPixelBufferInfo_YCbCrBiPlanar {
    pub componentInfoY: CVPlanarComponentInfo,
    pub componentInfoCbCr: CVPlanarComponentInfo,
}
pub type CVPixelBufferReleaseBytesCallback = ::std::option::Option<
    unsafe extern "C" fn(
        releaseRefCon: *mut ::std::os::raw::c_void,
        baseAddress: *const ::std::os::raw::c_void,
    ),
>;
pub type CVPixelBufferReleasePlanarBytesCallback = ::std::option::Option<
    unsafe extern "C" fn(
        releaseRefCon: *mut ::std::os::raw::c_void,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: usize,
        numberOfPlanes: usize,
        planeAddresses: *mut *const ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVPixelBufferPool {
    _unused: [u8; 0],
}
pub type CVPixelBufferPoolRef = *mut __CVPixelBufferPool;
pub type CVPixelBufferPoolFlushFlags = CVOptionFlags;
pub type CVOpenGLBufferRef = CVImageBufferRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVOpenGLBufferPool {
    _unused: [u8; 0],
}
pub type CVOpenGLBufferPoolRef = *mut __CVOpenGLBufferPool;
pub type CVOpenGLTextureRef = CVImageBufferRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVOpenGLTextureCache {
    _unused: [u8; 0],
}
pub type CVOpenGLTextureCacheRef = *mut __CVOpenGLTextureCache;
pub type CVMetalTextureRef = CVImageBufferRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVMetalTextureCache {
    _unused: [u8; 0],
}
pub type CVMetalTextureCacheRef = *mut __CVMetalTextureCache;
pub type CVMetalBufferRef = CVBufferRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVMetalBufferCache {
    _unused: [u8; 0],
}
pub type CVMetalBufferCacheRef = *mut __CVMetalBufferCache;
unsafe extern "C" {
    pub static kCVZeroTime: CVTime;
}
unsafe extern "C" {
    pub static kCVIndefiniteTime: CVTime;
}
unsafe extern "C" {
    pub fn CVGetCurrentHostTime() -> u64;
}
unsafe extern "C" {
    pub fn CVGetHostClockFrequency() -> f64;
}
unsafe extern "C" {
    pub fn CVGetHostClockMinimumTimeDelta() -> u32;
}
unsafe extern "C" {
    pub fn CVDisplayLinkGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVDisplayLinkCreateWithCGDisplays(
        displayArray: *mut CGDirectDisplayID,
        count: CFIndex,
        displayLinkOut: *mut CVDisplayLinkRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkCreateWithOpenGLDisplayMask(
        mask: CGOpenGLDisplayMask,
        displayLinkOut: *mut CVDisplayLinkRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkCreateWithCGDisplay(
        displayID: CGDirectDisplayID,
        displayLinkOut: *mut CVDisplayLinkRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkCreateWithActiveCGDisplays(
        displayLinkOut: *mut CVDisplayLinkRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkSetCurrentCGDisplay(
        displayLink: CVDisplayLinkRef,
        displayID: CGDirectDisplayID,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkSetCurrentCGDisplayFromOpenGLContext(
        displayLink: CVDisplayLinkRef,
        cglContext: CGLContextObj,
        cglPixelFormat: CGLPixelFormatObj,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkGetCurrentCGDisplay(displayLink: CVDisplayLinkRef) -> CGDirectDisplayID;
}
unsafe extern "C" {
    pub fn CVDisplayLinkSetOutputCallback(
        displayLink: CVDisplayLinkRef,
        callback: CVDisplayLinkOutputCallback,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkSetOutputHandler(
        displayLink: CVDisplayLinkRef,
        handler: CVDisplayLinkOutputHandler,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkStart(displayLink: CVDisplayLinkRef) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkStop(displayLink: CVDisplayLinkRef) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkGetNominalOutputVideoRefreshPeriod(displayLink: CVDisplayLinkRef)
        -> CVTime;
}
unsafe extern "C" {
    pub fn CVDisplayLinkGetOutputVideoLatency(displayLink: CVDisplayLinkRef) -> CVTime;
}
unsafe extern "C" {
    pub fn CVDisplayLinkGetActualOutputVideoRefreshPeriod(displayLink: CVDisplayLinkRef) -> f64;
}
unsafe extern "C" {
    pub fn CVDisplayLinkIsRunning(displayLink: CVDisplayLinkRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CVDisplayLinkGetCurrentTime(
        displayLink: CVDisplayLinkRef,
        outTime: *mut CVTimeStamp,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkTranslateTime(
        displayLink: CVDisplayLinkRef,
        inTime: *const CVTimeStamp,
        outTime: *mut CVTimeStamp,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVDisplayLinkRetain(displayLink: CVDisplayLinkRef) -> CVDisplayLinkRef;
}
unsafe extern "C" {
    pub fn CVDisplayLinkRelease(displayLink: CVDisplayLinkRef);
}
unsafe extern "C" {
    pub static mut kCVBufferPropagatedAttachmentsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVBufferNonPropagatedAttachmentsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVBufferMovieTimeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVBufferTimeValueKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVBufferTimeScaleKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVBufferRetain(buffer: CVBufferRef) -> CVBufferRef;
}
unsafe extern "C" {
    pub fn CVBufferRelease(buffer: CVBufferRef);
}
unsafe extern "C" {
    pub fn CVBufferSetAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        value: CFTypeRef,
        attachmentMode: CVAttachmentMode,
    );
}
unsafe extern "C" {
    pub fn CVBufferGetAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        attachmentMode: *mut CVAttachmentMode,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CVBufferRemoveAttachment(buffer: CVBufferRef, key: CFStringRef);
}
unsafe extern "C" {
    pub fn CVBufferRemoveAllAttachments(buffer: CVBufferRef);
}
unsafe extern "C" {
    pub fn CVBufferGetAttachments(
        buffer: CVBufferRef,
        attachmentMode: CVAttachmentMode,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVBufferSetAttachments(
        buffer: CVBufferRef,
        theAttachments: CFDictionaryRef,
        attachmentMode: CVAttachmentMode,
    );
}
unsafe extern "C" {
    pub fn CVBufferPropagateAttachments(sourceBuffer: CVBufferRef, destinationBuffer: CVBufferRef);
}
unsafe extern "C" {
    pub fn CVBufferCopyAttachments(
        buffer: CVBufferRef,
        attachmentMode: CVAttachmentMode,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVBufferCopyAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        attachmentMode: *mut CVAttachmentMode,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CVBufferHasAttachment(buffer: CVBufferRef, key: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCGColorSpaceKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureHorizontalOffsetKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureVerticalOffsetKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPreferredCleanApertureKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailTemporalTopFirst: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailTemporalBottomFirst: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailSpatialFirstLineEarly: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailSpatialFirstLineLate: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPixelAspectRatioKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPixelAspectRatioHorizontalSpacingKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPixelAspectRatioVerticalSpacingKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayDimensionsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferGammaLevelKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferICCProfileKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrixKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_ITU_R_601_4: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_DCI_P3: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_P3_D65: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_ITU_R_2020: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimariesKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_EBU_3213: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_SMPTE_C: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_P22: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_DCI_P3: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_P3_D65: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_ITU_R_2020: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunctionKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_SMPTE_240M_1995: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_UseGamma: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_EBU_3213: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_SMPTE_C: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_sRGB: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_ITU_R_2020: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_SMPTE_ST_428_1: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_ITU_R_2100_HLG: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_Linear: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocationTopFieldKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocationBottomFieldKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_Left: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_Center: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_TopLeft: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_Top: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_BottomLeft: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_Bottom: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_DV420: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaSubsamplingKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaSubsampling_420: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaSubsampling_422: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaSubsampling_411: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAlphaChannelIsOpaque: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAlphaChannelModeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAlphaChannelMode_StraightAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAlphaChannelMode_PremultipliedAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPostDecodeProcessingSequenceMetadataKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPostDecodeProcessingFrameMetadataKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVYCbCrMatrixGetIntegerCodePointForString(
        yCbCrMatrixString: CFStringRef,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CVColorPrimariesGetIntegerCodePointForString(
        colorPrimariesString: CFStringRef,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CVTransferFunctionGetIntegerCodePointForString(
        transferFunctionString: CFStringRef,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CVYCbCrMatrixGetStringForIntegerCodePoint(
        yCbCrMatrixCodePoint: ::std::os::raw::c_int,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CVColorPrimariesGetStringForIntegerCodePoint(
        colorPrimariesCodePoint: ::std::os::raw::c_int,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CVTransferFunctionGetStringForIntegerCodePoint(
        transferFunctionCodePoint: ::std::os::raw::c_int,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CVImageBufferGetEncodedSize(imageBuffer: CVImageBufferRef) -> CGSize;
}
unsafe extern "C" {
    pub fn CVImageBufferGetDisplaySize(imageBuffer: CVImageBufferRef) -> CGSize;
}
unsafe extern "C" {
    pub fn CVImageBufferGetCleanRect(imageBuffer: CVImageBufferRef) -> CGRect;
}
unsafe extern "C" {
    pub fn CVImageBufferIsFlipped(imageBuffer: CVImageBufferRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CVImageBufferGetColorSpace(imageBuffer: CVImageBufferRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CVImageBufferCreateColorSpaceFromAttachments(
        attachments: CFDictionaryRef,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferMasteringDisplayColorVolumeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferContentLightLevelInfoKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAmbientViewingEnvironmentKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferSceneIlluminationKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferRegionOfInterestKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferLogTransferFunctionKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferLogTransferFunction_AppleLog: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferLogTransferFunction_AppleLog2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangleKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_ReferenceRasterWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_ReferenceRasterHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RectangleLeftKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RectangleWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RectangleTopKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RectangleHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangleStereoLeftKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangleStereoRightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_LeftEdgePointsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RightEdgePointsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPixelFormatTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferMemoryAllocatorKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferExtendedPixelsLeftKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferExtendedPixelsTopKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferExtendedPixelsRightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferExtendedPixelsBottomKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferBytesPerRowAlignmentKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferCGBitmapContextCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferCGImageCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferOpenGLCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPlaneAlignmentKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfacePropertiesKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferOpenGLESCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferMetalCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferOpenGLTextureCacheCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferOpenGLESTextureCacheCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferVersatileBayerKey_BayerPattern: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_SenselSitingOffsets: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_BlackLevel: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_WhiteLevel: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_WhiteBalanceCCT: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_WhiteBalanceRedFactor: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_WhiteBalanceBlueFactor: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_ColorMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_GainFactor: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_RecommendedCrop: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_MetadataExtension: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfacePurgeableKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVPixelBufferRetain(texture: CVPixelBufferRef) -> CVPixelBufferRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferRelease(texture: CVPixelBufferRef);
}
unsafe extern "C" {
    pub fn CVPixelBufferCreateResolvedAttributesDictionary(
        allocator: CFAllocatorRef,
        attributes: CFArrayRef,
        resolvedDictionaryOut: *mut CFDictionaryRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferCreate(
        allocator: CFAllocatorRef,
        width: usize,
        height: usize,
        pixelFormatType: OSType,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferCreateWithBytes(
        allocator: CFAllocatorRef,
        width: usize,
        height: usize,
        pixelFormatType: OSType,
        baseAddress: *mut ::std::os::raw::c_void,
        bytesPerRow: usize,
        releaseCallback: CVPixelBufferReleaseBytesCallback,
        releaseRefCon: *mut ::std::os::raw::c_void,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferCreateWithPlanarBytes(
        allocator: CFAllocatorRef,
        width: usize,
        height: usize,
        pixelFormatType: OSType,
        dataPtr: *mut ::std::os::raw::c_void,
        dataSize: usize,
        numberOfPlanes: usize,
        planeBaseAddress: *mut *mut ::std::os::raw::c_void,
        planeWidth: *mut usize,
        planeHeight: *mut usize,
        planeBytesPerRow: *mut usize,
        releaseCallback: CVPixelBufferReleasePlanarBytesCallback,
        releaseRefCon: *mut ::std::os::raw::c_void,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferLockBaseAddress(
        pixelBuffer: CVPixelBufferRef,
        lockFlags: CVPixelBufferLockFlags,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferUnlockBaseAddress(
        pixelBuffer: CVPixelBufferRef,
        unlockFlags: CVPixelBufferLockFlags,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetWidth(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetHeight(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetPixelFormatType(pixelBuffer: CVPixelBufferRef) -> OSType;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetBaseAddress(
        pixelBuffer: CVPixelBufferRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetBytesPerRow(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetDataSize(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferIsPlanar(pixelBuffer: CVPixelBufferRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetPlaneCount(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetWidthOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: usize) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetHeightOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: usize)
        -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetBaseAddressOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: usize,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetBytesPerRowOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: usize,
    ) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetExtendedPixels(
        pixelBuffer: CVPixelBufferRef,
        extraColumnsOnLeft: *mut usize,
        extraColumnsOnRight: *mut usize,
        extraRowsOnTop: *mut usize,
        extraRowsOnBottom: *mut usize,
    );
}
unsafe extern "C" {
    pub fn CVPixelBufferFillExtendedPixels(pixelBuffer: CVPixelBufferRef) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferCopyCreationAttributes(pixelBuffer: CVPixelBufferRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferIsCompatibleWithAttributes(
        pixelBuffer: CVPixelBufferRef,
        attributes: CFDictionaryRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceOpenGLESTextureCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceOpenGLESFBOCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetIOSurface(pixelBuffer: CVPixelBufferRef) -> IOSurfaceRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferCreateWithIOSurface(
        allocator: CFAllocatorRef,
        surface: IOSurfaceRef,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPoolMinimumBufferCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPoolMaximumBufferAgeKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolRetain(pixelBufferPool: CVPixelBufferPoolRef) -> CVPixelBufferPoolRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolRelease(pixelBufferPool: CVPixelBufferPoolRef);
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolCreate(
        allocator: CFAllocatorRef,
        poolAttributes: CFDictionaryRef,
        pixelBufferAttributes: CFDictionaryRef,
        poolOut: *mut CVPixelBufferPoolRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolGetAttributes(pool: CVPixelBufferPoolRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolGetPixelBufferAttributes(pool: CVPixelBufferPoolRef)
        -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolCreatePixelBuffer(
        allocator: CFAllocatorRef,
        pixelBufferPool: CVPixelBufferPoolRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
        allocator: CFAllocatorRef,
        pixelBufferPool: CVPixelBufferPoolRef,
        auxAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPoolAllocationThresholdKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPoolFreeBufferNotification: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolFlush(pool: CVPixelBufferPoolRef, options: CVPixelBufferPoolFlushFlags);
}
unsafe extern "C" {
    pub static mut kCVOpenGLBufferWidth: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVOpenGLBufferHeight: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVOpenGLBufferTarget: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVOpenGLBufferInternalFormat: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVOpenGLBufferMaximumMipmapLevel: CFStringRef;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferRetain(buffer: CVOpenGLBufferRef) -> CVOpenGLBufferRef;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferRelease(buffer: CVOpenGLBufferRef);
}
unsafe extern "C" {
    pub fn CVOpenGLBufferCreate(
        allocator: CFAllocatorRef,
        width: usize,
        height: usize,
        attributes: CFDictionaryRef,
        bufferOut: *mut CVOpenGLBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferGetAttributes(openGLBuffer: CVOpenGLBufferRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferAttach(
        openGLBuffer: CVOpenGLBufferRef,
        cglContext: CGLContextObj,
        face: GLenum,
        level: GLint,
        screen: GLint,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub static mut kCVOpenGLBufferPoolMinimumBufferCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVOpenGLBufferPoolMaximumBufferAgeKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferPoolGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferPoolRetain(
        openGLBufferPool: CVOpenGLBufferPoolRef,
    ) -> CVOpenGLBufferPoolRef;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferPoolRelease(openGLBufferPool: CVOpenGLBufferPoolRef);
}
unsafe extern "C" {
    pub fn CVOpenGLBufferPoolCreate(
        allocator: CFAllocatorRef,
        poolAttributes: CFDictionaryRef,
        openGLBufferAttributes: CFDictionaryRef,
        poolOut: *mut CVOpenGLBufferPoolRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferPoolGetAttributes(pool: CVOpenGLBufferPoolRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferPoolGetOpenGLBufferAttributes(
        pool: CVOpenGLBufferPoolRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVOpenGLBufferPoolCreateOpenGLBuffer(
        allocator: CFAllocatorRef,
        openGLBufferPool: CVOpenGLBufferPoolRef,
        openGLBufferOut: *mut CVOpenGLBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureRetain(texture: CVOpenGLTextureRef) -> CVOpenGLTextureRef;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureRelease(texture: CVOpenGLTextureRef);
}
unsafe extern "C" {
    pub fn CVOpenGLTextureGetTarget(image: CVOpenGLTextureRef) -> GLenum;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureGetName(image: CVOpenGLTextureRef) -> GLuint;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureIsFlipped(image: CVOpenGLTextureRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureGetCleanTexCoords(
        image: CVOpenGLTextureRef,
        lowerLeft: *mut GLfloat,
        lowerRight: *mut GLfloat,
        upperRight: *mut GLfloat,
        upperLeft: *mut GLfloat,
    );
}
unsafe extern "C" {
    pub static mut kCVOpenGLTextureCacheChromaSamplingModeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVOpenGLTextureCacheChromaSamplingModeAutomatic: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVOpenGLTextureCacheChromaSamplingModeHighestQuality: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVOpenGLTextureCacheChromaSamplingModeBestPerformance: CFStringRef;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureCacheGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureCacheRetain(
        textureCache: CVOpenGLTextureCacheRef,
    ) -> CVOpenGLTextureCacheRef;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureCacheRelease(textureCache: CVOpenGLTextureCacheRef);
}
unsafe extern "C" {
    pub fn CVOpenGLTextureCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        cglContext: CGLContextObj,
        cglPixelFormat: CGLPixelFormatObj,
        textureAttributes: CFDictionaryRef,
        cacheOut: *mut CVOpenGLTextureCacheRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVOpenGLTextureCacheRef,
        sourceImage: CVImageBufferRef,
        attributes: CFDictionaryRef,
        textureOut: *mut CVOpenGLTextureRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVOpenGLTextureCacheFlush(textureCache: CVOpenGLTextureCacheRef, options: CVOptionFlags);
}
unsafe extern "C" {
    pub static mut kCVPixelFormatName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatConstant: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatCodecType: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatFourCC: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsYCbCr: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsRGB: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsGrayscale: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsSenselArray: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatComponentRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatComponentRange_VideoRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatComponentRange_FullRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatComponentRange_WideRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatPlanes: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlockWidth: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlockHeight: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBitsPerBlock: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBitsPerComponent: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlockHorizontalAlignment: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlockVerticalAlignment: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlackBlock: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatHorizontalSubsampling: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatVerticalSubsampling: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLFormat: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLType: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLInternalFormat: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatCGBitmapInfo: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatQDCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatCGBitmapContextCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatCGImageCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLESCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatFillExtendedPixelsCallback: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelFormatDescriptionCreateWithPixelFormatType(
        allocator: CFAllocatorRef,
        pixelFormat: OSType,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes(
        allocator: CFAllocatorRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType(
        description: CFDictionaryRef,
        pixelFormat: OSType,
    );
}
unsafe extern "C" {
    pub fn CVPixelFormatTypeCopyFourCharCodeString(pixelFormat: OSType) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CVIsCompressedPixelFormatAvailable(pixelFormatType: OSType) -> Boolean;
}
unsafe extern "C" {
    pub fn CVMetalTextureGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVMetalTextureGetTexture(image: CVMetalTextureRef) -> *mut u64;
}
unsafe extern "C" {
    pub fn CVMetalTextureIsFlipped(image: CVMetalTextureRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CVMetalTextureGetCleanTexCoords(
        image: CVMetalTextureRef,
        lowerLeft: *mut f32,
        lowerRight: *mut f32,
        upperRight: *mut f32,
        upperLeft: *mut f32,
    );
}
unsafe extern "C" {
    pub static mut kCVMetalTextureUsage: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVMetalTextureStorageMode: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVMetalTextureCacheMaximumTextureAgeKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVMetalTextureCacheGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVMetalTextureCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        metalDevice: *mut u64,
        textureAttributes: CFDictionaryRef,
        cacheOut: *mut CVMetalTextureCacheRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVMetalTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVMetalTextureCacheRef,
        sourceImage: CVImageBufferRef,
        textureAttributes: CFDictionaryRef,
        pixelFormat: MTLPixelFormat,
        width: usize,
        height: usize,
        planeIndex: usize,
        textureOut: *mut CVMetalTextureRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVMetalTextureCacheFlush(textureCache: CVMetalTextureCacheRef, options: CVOptionFlags);
}
unsafe extern "C" {
    pub fn CVMetalBufferGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVMetalBufferGetBuffer(buffer: CVMetalBufferRef) -> *mut u64;
}
unsafe extern "C" {
    pub static mut kCVMetalBufferCacheMaximumBufferAgeKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVMetalBufferCacheGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVMetalBufferCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        metalDevice: *mut u64,
        cacheOut: *mut CVMetalBufferCacheRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVMetalBufferCacheCreateBufferFromImage(
        allocator: CFAllocatorRef,
        bufferCache: CVMetalBufferCacheRef,
        imageBuffer: CVImageBufferRef,
        bufferOut: *mut CVMetalBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVMetalBufferCacheFlush(bufferCache: CVMetalBufferCacheRef, options: CVOptionFlags);
}

unsafe impl objc2::encode::RefEncode for CVSMPTETime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVSMPTETime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVSMPTETime", &[]);
}
unsafe impl objc2::encode::RefEncode for CVTime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVTime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVTime", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVDisplayLink {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVDisplayLink {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVDisplayLink", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for CVPlanarComponentInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVPlanarComponentInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVPlanarComponentInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CVPlanarPixelBufferInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVPlanarPixelBufferInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVPlanarPixelBufferInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CVPlanarPixelBufferInfo_YCbCrPlanar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVPlanarPixelBufferInfo_YCbCrPlanar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVPlanarPixelBufferInfo_YCbCrPlanar", &[]);
}
unsafe impl objc2::encode::RefEncode for CVPlanarPixelBufferInfo_YCbCrBiPlanar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVPlanarPixelBufferInfo_YCbCrBiPlanar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVPlanarPixelBufferInfo_YCbCrBiPlanar", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVPixelBufferPool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVPixelBufferPool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVPixelBufferPool", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVOpenGLBufferPool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVOpenGLBufferPool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVOpenGLBufferPool", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVOpenGLTextureCache {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVOpenGLTextureCache {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVOpenGLTextureCache", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVMetalTextureCache {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVMetalTextureCache {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVMetalTextureCache", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVMetalBufferCache {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVMetalBufferCache {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVMetalBufferCache", &[]);
}
