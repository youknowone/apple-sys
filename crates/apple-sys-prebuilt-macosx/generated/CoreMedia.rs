#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::AudioToolbox::*;
#[allow(unused_imports)]
use crate::CoreAudio::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFormatListItem {
    pub mASBD: AudioStreamBasicDescription,
    pub mChannelLayoutTag: AudioChannelLayoutTag,
}
pub type CMItemCount = CFIndex;
pub type CMItemIndex = CFIndex;
pub type CMStructVersion = usize;
pub type CMPersistentTrackID = i32;
pub type CMTimeValue = i64;
pub type CMTimeScale = i32;
pub type CMTimeEpoch = i64;
pub type CMTimeFlags = u32;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub timescale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}
pub type CMTimeRoundingMethod = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMTimeRange {
    pub start: CMTime,
    pub duration: CMTime,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMTimeMapping {
    pub source: CMTimeRange,
    pub target: CMTimeRange,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct opaqueCMFormatDescription {
    _unused: [u8; 0],
}
pub type CMFormatDescriptionRef = *const opaqueCMFormatDescription;
pub type CMMediaType = FourCharCode;
pub type CMAudioFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMAudioFormatDescriptionMask = u32;
pub type CMVideoFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMVideoCodecType = FourCharCode;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMVideoDimensions {
    pub width: i32,
    pub height: i32,
}
pub type CMTaggedBufferGroupFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMTaggedBufferGroupFormatType = FourCharCode;
pub type CMMuxedFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMMuxedStreamType = FourCharCode;
pub type CMClosedCaptionFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMTextFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMTextDisplayFlags = u32;
pub type CMTextJustificationValue = i8;
pub type CMTimeCodeFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMTimeCodeFormatType = FourCharCode;
pub type CMMetadataFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMMetadataFormatType = FourCharCode;
pub type CMAttachmentBearerRef = CFTypeRef;
pub type CMAttachmentMode = u32;
pub type CMBlockBufferFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCMBlockBuffer {
    _unused: [u8; 0],
}
pub type CMBlockBufferRef = *mut OpaqueCMBlockBuffer;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMBlockBufferCustomBlockSource {
    pub version: u32,
    pub AllocateBlock: ::std::option::Option<
        unsafe extern "C" fn(
            refcon: *mut ::std::os::raw::c_void,
            sizeInBytes: usize,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub FreeBlock: ::std::option::Option<
        unsafe extern "C" fn(
            refcon: *mut ::std::os::raw::c_void,
            doomedMemoryBlock: *mut ::std::os::raw::c_void,
            sizeInBytes: usize,
        ),
    >,
    pub refCon: *mut ::std::os::raw::c_void,
}
pub type CMImageDescriptionFlavor = CFStringRef;
pub type CMSoundDescriptionFlavor = CFStringRef;
pub type CMTextDescriptionFlavor = CFStringRef;
pub type CMClosedCaptionDescriptionFlavor = CFStringRef;
pub type CMTimeCodeDescriptionFlavor = CFStringRef;
pub type CMMetadataDescriptionFlavor = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct opaqueCMBufferQueue {
    _unused: [u8; 0],
}
pub type CMBufferQueueRef = *mut opaqueCMBufferQueue;
pub type CMBufferRef = CFTypeRef;
pub type CMBufferGetTimeCallback = ::std::option::Option<
    unsafe extern "C" fn(buf: CMBufferRef, refcon: *mut ::std::os::raw::c_void) -> CMTime,
>;
pub type CMBufferGetTimeHandler = *mut ::std::os::raw::c_void;
pub type CMBufferGetBooleanCallback = ::std::option::Option<
    unsafe extern "C" fn(buf: CMBufferRef, refcon: *mut ::std::os::raw::c_void) -> Boolean,
>;
pub type CMBufferGetBooleanHandler = *mut ::std::os::raw::c_void;
pub type CMBufferCompareCallback = ::std::option::Option<
    unsafe extern "C" fn(
        buf1: CMBufferRef,
        buf2: CMBufferRef,
        refcon: *mut ::std::os::raw::c_void,
    ) -> CFComparisonResult,
>;
pub type CMBufferCompareHandler = *mut ::std::os::raw::c_void;
pub type CMBufferGetSizeCallback = ::std::option::Option<
    unsafe extern "C" fn(buf: CMBufferRef, refcon: *mut ::std::os::raw::c_void) -> usize,
>;
pub type CMBufferGetSizeHandler = *mut ::std::os::raw::c_void;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMBufferCallbacks {
    pub version: u32,
    pub refcon: *mut ::std::os::raw::c_void,
    pub getDecodeTimeStamp: CMBufferGetTimeCallback,
    pub getPresentationTimeStamp: CMBufferGetTimeCallback,
    pub getDuration: CMBufferGetTimeCallback,
    pub isDataReady: CMBufferGetBooleanCallback,
    pub compare: CMBufferCompareCallback,
    pub dataBecameReadyNotification: CFStringRef,
    pub getSize: CMBufferGetSizeCallback,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMBufferHandlers {
    pub version: usize,
    pub getDecodeTimeStamp: CMBufferGetTimeHandler,
    pub getPresentationTimeStamp: CMBufferGetTimeHandler,
    pub getDuration: CMBufferGetTimeHandler,
    pub isDataReady: CMBufferGetBooleanHandler,
    pub compare: CMBufferCompareHandler,
    pub dataBecameReadyNotification: CFStringRef,
    pub getSize: CMBufferGetSizeHandler,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct opaqueCMBufferQueueTriggerToken {
    _unused: [u8; 0],
}
pub type CMBufferQueueTriggerToken = *mut opaqueCMBufferQueueTriggerToken;
pub type CMBufferQueueTriggerCallback = ::std::option::Option<
    unsafe extern "C" fn(
        triggerRefcon: *mut ::std::os::raw::c_void,
        triggerToken: CMBufferQueueTriggerToken,
    ),
>;
pub type CMBufferQueueTriggerHandler = *mut ::std::os::raw::c_void;
pub type CMBufferQueueTriggerCondition = i32;
pub type CMBufferValidationCallback = ::std::option::Option<
    unsafe extern "C" fn(
        queue: CMBufferQueueRef,
        buf: CMBufferRef,
        validationRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type CMBufferValidationHandler = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct opaqueCMSampleBuffer {
    _unused: [u8; 0],
}
pub type CMSampleBufferRef = *mut opaqueCMSampleBuffer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMSampleTimingInfo {
    pub duration: CMTime,
    pub presentationTimeStamp: CMTime,
    pub decodeTimeStamp: CMTime,
}
pub type CMSampleBufferMakeDataReadyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        sbuf: CMSampleBufferRef,
        makeDataReadyRefcon: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type CMSampleBufferMakeDataReadyHandler = *mut ::std::os::raw::c_void;
pub type CMSampleBufferInvalidateCallback =
    ::std::option::Option<unsafe extern "C" fn(sbuf: CMSampleBufferRef, invalidateRefCon: u64)>;
pub type CMSampleBufferInvalidateHandler = *mut ::std::os::raw::c_void;
pub type CMTagError = OSStatus;
pub type CMTagCategory = FourCharCode;
pub type CMTagDataType = u32;
pub type CMTagValue = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMTag {
    pub category: CMTagCategory,
    pub dataType: CMTagDataType,
    pub value: CMTagValue,
}
pub type CMStereoViewComponents = u64;
pub type CMStereoViewInterpretationOptions = u64;
pub type CMProjectionType = u64;
pub type CMPackingType = u64;
pub type CMTagCollectionError = OSStatus;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCMTagCollection {
    _unused: [u8; 0],
}
pub type CMTagCollectionRef = *const OpaqueCMTagCollection;
pub type CMMutableTagCollectionRef = *mut OpaqueCMTagCollection;
pub type CMTagCollectionApplierFunction =
    ::std::option::Option<unsafe extern "C" fn(tag: CMTag, context: *mut ::std::os::raw::c_void)>;
pub type CMTagCollectionTagFilterFunction = ::std::option::Option<
    unsafe extern "C" fn(tag: CMTag, context: *mut ::std::os::raw::c_void) -> Boolean,
>;
pub type CMTaggedBufferGroupError = OSStatus;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCMTaggedBufferGroup {
    _unused: [u8; 0],
}
pub type CMTaggedBufferGroupRef = *mut OpaqueCMTaggedBufferGroup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct opaqueCMSimpleQueue {
    _unused: [u8; 0],
}
pub type CMSimpleQueueRef = *mut opaqueCMSimpleQueue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCMMemoryPool {
    _unused: [u8; 0],
}
pub type CMMemoryPoolRef = *mut OpaqueCMMemoryPool;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCMClock {
    _unused: [u8; 0],
}
pub type CMClockRef = *mut OpaqueCMClock;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueCMTimebase {
    _unused: [u8; 0],
}
pub type CMTimebaseRef = *mut OpaqueCMTimebase;
pub type CMClockOrTimebaseRef = CFTypeRef;
unsafe extern "C" {
    pub static kCMTimeInvalid: CMTime;
}
unsafe extern "C" {
    pub static kCMTimeIndefinite: CMTime;
}
unsafe extern "C" {
    pub static kCMTimePositiveInfinity: CMTime;
}
unsafe extern "C" {
    pub static kCMTimeNegativeInfinity: CMTime;
}
unsafe extern "C" {
    pub static kCMTimeZero: CMTime;
}
unsafe extern "C" {
    pub fn CMTimeMake(value: i64, timescale: i32) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeMakeWithEpoch(value: i64, timescale: i32, epoch: i64) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeMakeWithSeconds(seconds: Float64, preferredTimescale: i32) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeGetSeconds(time: CMTime) -> Float64;
}
unsafe extern "C" {
    pub fn CMTimeConvertScale(
        time: CMTime,
        newTimescale: i32,
        method: CMTimeRoundingMethod,
    ) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeAdd(lhs: CMTime, rhs: CMTime) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeSubtract(lhs: CMTime, rhs: CMTime) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeMultiply(time: CMTime, multiplier: i32) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeMultiplyByFloat64(time: CMTime, multiplier: Float64) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeMultiplyByRatio(time: CMTime, multiplier: i32, divisor: i32) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeCompare(time1: CMTime, time2: CMTime) -> i32;
}
unsafe extern "C" {
    pub fn CMTimeMinimum(time1: CMTime, time2: CMTime) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeMaximum(time1: CMTime, time2: CMTime) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeAbsoluteValue(time: CMTime) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeCopyAsDictionary(time: CMTime, allocator: CFAllocatorRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CMTimeMakeFromDictionary(dictionaryRepresentation: CFDictionaryRef) -> CMTime;
}
unsafe extern "C" {
    pub static kCMTimeValueKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTimeScaleKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTimeEpochKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTimeFlagsKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CMTimeCopyDescription(allocator: CFAllocatorRef, time: CMTime) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CMTimeShow(time: CMTime);
}
unsafe extern "C" {
    pub static kCMTimeRangeZero: CMTimeRange;
}
unsafe extern "C" {
    pub static kCMTimeRangeInvalid: CMTimeRange;
}
unsafe extern "C" {
    pub fn CMTimeRangeMake(start: CMTime, duration: CMTime) -> CMTimeRange;
}
unsafe extern "C" {
    pub fn CMTimeRangeGetUnion(range: CMTimeRange, otherRange: CMTimeRange) -> CMTimeRange;
}
unsafe extern "C" {
    pub fn CMTimeRangeGetIntersection(range: CMTimeRange, otherRange: CMTimeRange) -> CMTimeRange;
}
unsafe extern "C" {
    pub fn CMTimeRangeEqual(range1: CMTimeRange, range2: CMTimeRange) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTimeRangeContainsTime(range: CMTimeRange, time: CMTime) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTimeRangeContainsTimeRange(range: CMTimeRange, otherRange: CMTimeRange) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTimeRangeGetEnd(range: CMTimeRange) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeMapTimeFromRangeToRange(
        t: CMTime,
        fromRange: CMTimeRange,
        toRange: CMTimeRange,
    ) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeClampToRange(time: CMTime, range: CMTimeRange) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeMapDurationFromRangeToRange(
        dur: CMTime,
        fromRange: CMTimeRange,
        toRange: CMTimeRange,
    ) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeFoldIntoRange(time: CMTime, foldRange: CMTimeRange) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeRangeFromTimeToTime(start: CMTime, end: CMTime) -> CMTimeRange;
}
unsafe extern "C" {
    pub fn CMTimeRangeCopyAsDictionary(
        range: CMTimeRange,
        allocator: CFAllocatorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CMTimeRangeMakeFromDictionary(dictionaryRepresentation: CFDictionaryRef) -> CMTimeRange;
}
unsafe extern "C" {
    pub static kCMTimeRangeStartKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTimeRangeDurationKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CMTimeRangeCopyDescription(allocator: CFAllocatorRef, range: CMTimeRange)
        -> CFStringRef;
}
unsafe extern "C" {
    pub fn CMTimeRangeShow(range: CMTimeRange);
}
unsafe extern "C" {
    pub static kCMTimeMappingInvalid: CMTimeMapping;
}
unsafe extern "C" {
    pub fn CMTimeMappingMake(source: CMTimeRange, target: CMTimeRange) -> CMTimeMapping;
}
unsafe extern "C" {
    pub fn CMTimeMappingMakeEmpty(target: CMTimeRange) -> CMTimeMapping;
}
unsafe extern "C" {
    pub fn CMTimeMappingCopyAsDictionary(
        mapping: CMTimeMapping,
        allocator: CFAllocatorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CMTimeMappingMakeFromDictionary(
        dictionaryRepresentation: CFDictionaryRef,
    ) -> CMTimeMapping;
}
unsafe extern "C" {
    pub static kCMTimeMappingSourceKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTimeMappingTargetKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CMTimeMappingCopyDescription(
        allocator: CFAllocatorRef,
        mapping: CMTimeMapping,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CMTimeMappingShow(mapping: CMTimeMapping);
}
unsafe extern "C" {
    pub fn CMFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        mediaType: CMMediaType,
        mediaSubType: FourCharCode,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMFormatDescriptionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMFormatDescriptionEqual(
        formatDescription: CMFormatDescriptionRef,
        otherFormatDescription: CMFormatDescriptionRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMFormatDescriptionEqualIgnoringExtensionKeys(
        formatDescription: CMFormatDescriptionRef,
        otherFormatDescription: CMFormatDescriptionRef,
        formatDescriptionExtensionKeysToIgnore: CFTypeRef,
        sampleDescriptionExtensionAtomKeysToIgnore: CFTypeRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMFormatDescriptionGetMediaType(desc: CMFormatDescriptionRef) -> CMMediaType;
}
unsafe extern "C" {
    pub fn CMFormatDescriptionGetMediaSubType(desc: CMFormatDescriptionRef) -> FourCharCode;
}
unsafe extern "C" {
    pub fn CMFormatDescriptionGetExtensions(desc: CMFormatDescriptionRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_OriginalCompressionSettings: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_VerbatimSampleDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_VerbatimISOSampleEntry: CFStringRef;
}
unsafe extern "C" {
    pub fn CMFormatDescriptionGetExtension(
        desc: CMFormatDescriptionRef,
        extensionKey: CFStringRef,
    ) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        asbd: *const AudioStreamBasicDescription,
        layoutSize: usize,
        layout: *const AudioChannelLayout,
        magicCookieSize: usize,
        magicCookie: *const ::std::os::raw::c_void,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMAudioFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionGetStreamBasicDescription(
        desc: CMAudioFormatDescriptionRef,
    ) -> *const AudioStreamBasicDescription;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionGetMagicCookie(
        desc: CMAudioFormatDescriptionRef,
        sizeOut: *mut usize,
    ) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionGetChannelLayout(
        desc: CMAudioFormatDescriptionRef,
        sizeOut: *mut usize,
    ) -> *const AudioChannelLayout;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionGetFormatList(
        desc: CMAudioFormatDescriptionRef,
        sizeOut: *mut usize,
    ) -> *const AudioFormatListItem;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionGetRichestDecodableFormat(
        desc: CMAudioFormatDescriptionRef,
    ) -> *const AudioFormatListItem;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionGetMostCompatibleFormat(
        desc: CMAudioFormatDescriptionRef,
    ) -> *const AudioFormatListItem;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionCreateSummary(
        allocator: CFAllocatorRef,
        formatDescriptionArray: CFArrayRef,
        flags: u32,
        formatDescriptionOut: *mut CMAudioFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionEqual(
        formatDescription: CMAudioFormatDescriptionRef,
        otherFormatDescription: CMAudioFormatDescriptionRef,
        equalityMask: CMAudioFormatDescriptionMask,
        equalityMaskOut: *mut CMAudioFormatDescriptionMask,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_FormatName: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_Depth: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_CleanAperture: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_CleanApertureWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_CleanApertureHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_CleanApertureHorizontalOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_CleanApertureVerticalOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_CleanApertureWidthRational: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_CleanApertureHeightRational: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_CleanApertureHorizontalOffsetRational: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_CleanApertureVerticalOffsetRational: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_FieldCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_FieldDetail: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionFieldDetail_TemporalTopFirst: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionFieldDetail_TemporalBottomFirst: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionFieldDetail_SpatialFirstLineEarly: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionFieldDetail_SpatialFirstLineLate: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_PixelAspectRatio: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_PixelAspectRatioHorizontalSpacing: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionKey_PixelAspectRatioVerticalSpacing: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ColorPrimaries: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionColorPrimaries_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionColorPrimaries_EBU_3213: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionColorPrimaries_SMPTE_C: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionColorPrimaries_DCI_P3: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionColorPrimaries_P3_D65: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionColorPrimaries_ITU_R_2020: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionColorPrimaries_P22: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_TransferFunction: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionTransferFunction_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionTransferFunction_SMPTE_240M_1995: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionTransferFunction_UseGamma: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionTransferFunction_ITU_R_2020: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionTransferFunction_SMPTE_ST_428_1: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionTransferFunction_SMPTE_ST_2084_PQ: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionTransferFunction_ITU_R_2100_HLG: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionTransferFunction_Linear: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionTransferFunction_sRGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_GammaLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_YCbCrMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionYCbCrMatrix_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionYCbCrMatrix_ITU_R_601_4: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionYCbCrMatrix_ITU_R_2020: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_FullRangeVideo: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ICCProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_BytesPerRow: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ChromaLocationTopField: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ChromaLocationBottomField: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionChromaLocation_Left: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionChromaLocation_Center: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionChromaLocation_TopLeft: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionChromaLocation_Top: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionChromaLocation_BottomLeft: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionChromaLocation_Bottom: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionChromaLocation_DV420: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionConformsToMPEG2VideoProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ProtectedContentOriginalFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_TemporalQuality: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_SpatialQuality: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_VerbatimImageDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_Version: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_RevisionLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_Vendor: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionVendor_Apple: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_MasteringDisplayColorVolume: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ContentLightLevelInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ContentColorVolume: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_AlternativeTransferCharacteristics: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_AuxiliaryTypeInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_AlphaChannelMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionAlphaChannelMode_StraightAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionAlphaChannelMode_PremultipliedAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ContainsAlphaChannel: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_BitsPerComponent: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_HorizontalFieldOfView: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_LogTransferFunction: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionLogTransferFunction_AppleLog: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_HeroEye: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionHeroEye_Left: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionHeroEye_Right: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_StereoCameraBaseline: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_HorizontalDisparityAdjustment: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_HasLeftStereoEyeView: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_HasRightStereoEyeView: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_HasAdditionalViews: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ProjectionKind: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionProjectionKind_Rectilinear: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionProjectionKind_Equirectangular: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionProjectionKind_HalfEquirectangular: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionProjectionKind_ParametricImmersive: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionProjectionKind_AppleImmersiveVideo: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ViewPackingKind: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionViewPackingKind_SideBySide: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionViewPackingKind_OverUnder: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_CameraCalibrationDataLensCollection: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_LensAlgorithmKind: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibrationLensAlgorithmKind_ParametricLens: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_LensDomain: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibrationLensDomain_Color: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_LensIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_LensRole: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibrationLensRole_Mono: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibrationLensRole_Left: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibrationLensRole_Right: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_LensDistortions: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_LensFrameAdjustmentsPolynomialX: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_LensFrameAdjustmentsPolynomialY: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_RadialAngleLimit: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_IntrinsicMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_IntrinsicMatrixProjectionOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_IntrinsicMatrixReferenceDimensions:
        CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_ExtrinsicOriginSource: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibrationExtrinsicOriginSource_StereoCameraSystemBaseline:
        CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionCameraCalibration_ExtrinsicOrientationQuaternion: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtension_ConvertedFromExternalSphericalTags: CFStringRef;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        codecType: CMVideoCodecType,
        width: i32,
        height: i32,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionCreateForImageBuffer(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        formatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionCreateFromH264ParameterSets(
        allocator: CFAllocatorRef,
        parameterSetCount: usize,
        parameterSetPointers: *const *const u8,
        parameterSetSizes: *const usize,
        NALUnitHeaderLength: ::std::os::raw::c_int,
        formatDescriptionOut: *mut CMFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionCreateFromHEVCParameterSets(
        allocator: CFAllocatorRef,
        parameterSetCount: usize,
        parameterSetPointers: *const *const u8,
        parameterSetSizes: *const usize,
        NALUnitHeaderLength: ::std::os::raw::c_int,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
        videoDesc: CMFormatDescriptionRef,
        parameterSetIndex: usize,
        parameterSetPointerOut: *mut *const u8,
        parameterSetSizeOut: *mut usize,
        parameterSetCountOut: *mut usize,
        NALUnitHeaderLengthOut: *mut ::std::os::raw::c_int,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
        videoDesc: CMFormatDescriptionRef,
        parameterSetIndex: usize,
        parameterSetPointerOut: *mut *const u8,
        parameterSetSizeOut: *mut usize,
        parameterSetCountOut: *mut usize,
        NALUnitHeaderLengthOut: *mut ::std::os::raw::c_int,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionGetDimensions(
        videoDesc: CMVideoFormatDescriptionRef,
    ) -> CMVideoDimensions;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionGetPresentationDimensions(
        videoDesc: CMVideoFormatDescriptionRef,
        usePixelAspectRatio: Boolean,
        useCleanAperture: Boolean,
    ) -> CGSize;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionGetCleanAperture(
        videoDesc: CMVideoFormatDescriptionRef,
        originIsAtTopLeft: Boolean,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionMatchesImageBuffer(
        desc: CMVideoFormatDescriptionRef,
        imageBuffer: CVImageBufferRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionCopyTagCollectionArray(
        formatDescription: CMVideoFormatDescriptionRef,
        tagCollectionsOut: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMuxedFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        muxType: CMMuxedStreamType,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMMuxedFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionExtension_DisplayFlags: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionExtension_BackgroundColor: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionColor_Red: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionColor_Green: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionColor_Blue: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionColor_Alpha: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionExtension_DefaultTextBox: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionRect_Top: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionRect_Left: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionRect_Bottom: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionRect_Right: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionExtension_DefaultStyle: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionStyle_StartChar: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionStyle_Font: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionStyle_FontFace: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionStyle_ForegroundColor: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionStyle_FontSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionExtension_HorizontalJustification: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionExtension_VerticalJustification: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionStyle_EndChar: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionExtension_FontTable: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionExtension_TextJustification: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionStyle_Height: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionStyle_Ascent: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextFormatDescriptionExtension_DefaultFontName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMFormatDescriptionExtension_AmbientViewingEnvironment: CFStringRef;
}
unsafe extern "C" {
    pub fn CMTextFormatDescriptionGetDisplayFlags(
        desc: CMFormatDescriptionRef,
        displayFlagsOut: *mut CMTextDisplayFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTextFormatDescriptionGetJustification(
        desc: CMFormatDescriptionRef,
        horizontaJustificationlOut: *mut CMTextJustificationValue,
        verticalJustificationOut: *mut CMTextJustificationValue,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTextFormatDescriptionGetDefaultTextBox(
        desc: CMFormatDescriptionRef,
        originIsAtTopLeft: Boolean,
        heightOfTextTrack: CGFloat,
        defaultTextBoxOut: *mut CGRect,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTextFormatDescriptionGetDefaultStyle(
        desc: CMFormatDescriptionRef,
        localFontIDOut: *mut u16,
        boldOut: *mut Boolean,
        italicOut: *mut Boolean,
        underlineOut: *mut Boolean,
        fontSizeOut: *mut CGFloat,
        colorComponentsOut: *mut CGFloat,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTextFormatDescriptionGetFontName(
        desc: CMFormatDescriptionRef,
        localFontID: u16,
        fontNameOut: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimeCodeFormatDescriptionCreate(
        allocator: CFAllocatorRef,
        timeCodeFormatType: CMTimeCodeFormatType,
        frameDuration: CMTime,
        frameQuanta: u32,
        flags: u32,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMTimeCodeFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimeCodeFormatDescriptionGetFrameDuration(
        timeCodeFormatDescription: CMTimeCodeFormatDescriptionRef,
    ) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimeCodeFormatDescriptionGetFrameQuanta(
        timeCodeFormatDescription: CMTimeCodeFormatDescriptionRef,
    ) -> u32;
}
unsafe extern "C" {
    pub fn CMTimeCodeFormatDescriptionGetTimeCodeFlags(desc: CMTimeCodeFormatDescriptionRef)
        -> u32;
}
unsafe extern "C" {
    pub static kCMTimeCodeFormatDescriptionExtension_SourceReferenceName: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTimeCodeFormatDescriptionKey_Value: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTimeCodeFormatDescriptionKey_LangCode: CFStringRef;
}
unsafe extern "C" {
    pub static kCMFormatDescriptionExtensionKey_MetadataKeyTable: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionKey_Namespace: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionKey_Value: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionKey_LocalID: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionKey_DataType: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionKey_DataTypeNamespace: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionKey_ConformingDataTypes: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionKey_LanguageTag: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionKey_StructuralDependency: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionKey_SetupData: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescription_StructuralDependencyKey_DependencyIsInvalidFlag:
        CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_Identifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_DataType: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_ExtendedLanguageTag:
        CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_StructuralDependency:
        CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataFormatDescriptionMetadataSpecificationKey_SetupData: CFStringRef;
}
unsafe extern "C" {
    pub fn CMMetadataFormatDescriptionCreateWithKeys(
        allocator: CFAllocatorRef,
        metadataType: CMMetadataFormatType,
        keys: CFArrayRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataFormatDescriptionCreateWithMetadataSpecifications(
        allocator: CFAllocatorRef,
        metadataType: CMMetadataFormatType,
        metadataSpecifications: CFArrayRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataFormatDescriptionCreateWithMetadataFormatDescriptionAndMetadataSpecifications(
        allocator: CFAllocatorRef,
        sourceDescription: CMMetadataFormatDescriptionRef,
        metadataSpecifications: CFArrayRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataFormatDescriptionCreateByMergingMetadataFormatDescriptions(
        allocator: CFAllocatorRef,
        sourceDescription: CMMetadataFormatDescriptionRef,
        otherSourceDescription: CMMetadataFormatDescriptionRef,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataFormatDescriptionGetKeyWithLocalID(
        desc: CMMetadataFormatDescriptionRef,
        localKeyID: OSType,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CMMetadataFormatDescriptionGetIdentifiers(
        desc: CMMetadataFormatDescriptionRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CMSetAttachment(
        target: CMAttachmentBearerRef,
        key: CFStringRef,
        value: CFTypeRef,
        attachmentMode: CMAttachmentMode,
    );
}
unsafe extern "C" {
    pub fn CMGetAttachment(
        target: CMAttachmentBearerRef,
        key: CFStringRef,
        attachmentModeOut: *mut CMAttachmentMode,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CMRemoveAttachment(target: CMAttachmentBearerRef, key: CFStringRef);
}
unsafe extern "C" {
    pub fn CMRemoveAllAttachments(target: CMAttachmentBearerRef);
}
unsafe extern "C" {
    pub fn CMCopyDictionaryOfAttachments(
        allocator: CFAllocatorRef,
        target: CMAttachmentBearerRef,
        attachmentMode: CMAttachmentMode,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CMSetAttachments(
        target: CMAttachmentBearerRef,
        theAttachments: CFDictionaryRef,
        attachmentMode: CMAttachmentMode,
    );
}
unsafe extern "C" {
    pub fn CMPropagateAttachments(
        source: CMAttachmentBearerRef,
        destination: CMAttachmentBearerRef,
    );
}
unsafe extern "C" {
    pub fn CMBlockBufferCreateEmpty(
        structureAllocator: CFAllocatorRef,
        subBlockCapacity: u32,
        flags: CMBlockBufferFlags,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferCreateWithMemoryBlock(
        structureAllocator: CFAllocatorRef,
        memoryBlock: *mut ::std::os::raw::c_void,
        blockLength: usize,
        blockAllocator: CFAllocatorRef,
        customBlockSource: *const CMBlockBufferCustomBlockSource,
        offsetToData: usize,
        dataLength: usize,
        flags: CMBlockBufferFlags,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferCreateWithBufferReference(
        structureAllocator: CFAllocatorRef,
        bufferReference: CMBlockBufferRef,
        offsetToData: usize,
        dataLength: usize,
        flags: CMBlockBufferFlags,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferCreateContiguous(
        structureAllocator: CFAllocatorRef,
        sourceBuffer: CMBlockBufferRef,
        blockAllocator: CFAllocatorRef,
        customBlockSource: *const CMBlockBufferCustomBlockSource,
        offsetToData: usize,
        dataLength: usize,
        flags: CMBlockBufferFlags,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMBlockBufferAppendMemoryBlock(
        theBuffer: CMBlockBufferRef,
        memoryBlock: *mut ::std::os::raw::c_void,
        blockLength: usize,
        blockAllocator: CFAllocatorRef,
        customBlockSource: *const CMBlockBufferCustomBlockSource,
        offsetToData: usize,
        dataLength: usize,
        flags: CMBlockBufferFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferAppendBufferReference(
        theBuffer: CMBlockBufferRef,
        targetBBuf: CMBlockBufferRef,
        offsetToData: usize,
        dataLength: usize,
        flags: CMBlockBufferFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferAssureBlockMemory(theBuffer: CMBlockBufferRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferAccessDataBytes(
        theBuffer: CMBlockBufferRef,
        offset: usize,
        length: usize,
        temporaryBlock: *mut ::std::os::raw::c_void,
        returnedPointerOut: *mut *mut ::std::os::raw::c_char,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferCopyDataBytes(
        theSourceBuffer: CMBlockBufferRef,
        offsetToData: usize,
        dataLength: usize,
        destination: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferReplaceDataBytes(
        sourceBytes: *const ::std::os::raw::c_void,
        destinationBuffer: CMBlockBufferRef,
        offsetIntoDestination: usize,
        dataLength: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferFillDataBytes(
        fillByte: ::std::os::raw::c_char,
        destinationBuffer: CMBlockBufferRef,
        offsetIntoDestination: usize,
        dataLength: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferGetDataPointer(
        theBuffer: CMBlockBufferRef,
        offset: usize,
        lengthAtOffsetOut: *mut usize,
        totalLengthOut: *mut usize,
        dataPointerOut: *mut *mut ::std::os::raw::c_char,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBlockBufferGetDataLength(theBuffer: CMBlockBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CMBlockBufferIsRangeContiguous(
        theBuffer: CMBlockBufferRef,
        offset: usize,
        length: usize,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMBlockBufferIsEmpty(theBuffer: CMBlockBufferRef) -> Boolean;
}
unsafe extern "C" {
    pub static kCMImageDescriptionFlavor_QuickTimeMovie: CMImageDescriptionFlavor;
}
unsafe extern "C" {
    pub static kCMImageDescriptionFlavor_ISOFamily: CMImageDescriptionFlavor;
}
unsafe extern "C" {
    pub static kCMImageDescriptionFlavor_3GPFamily: CMImageDescriptionFlavor;
}
unsafe extern "C" {
    pub static kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions: CMImageDescriptionFlavor;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData(
        allocator: CFAllocatorRef,
        imageDescriptionData: *const u8,
        size: usize,
        stringEncoding: CFStringEncoding,
        flavor: CMImageDescriptionFlavor,
        formatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        imageDescriptionBlockBuffer: CMBlockBufferRef,
        stringEncoding: CFStringEncoding,
        flavor: CMImageDescriptionFlavor,
        formatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        videoFormatDescription: CMVideoFormatDescriptionRef,
        stringEncoding: CFStringEncoding,
        flavor: CMImageDescriptionFlavor,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapBigEndianImageDescriptionToHost(
        imageDescriptionData: *mut u8,
        imageDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapHostEndianImageDescriptionToBig(
        imageDescriptionData: *mut u8,
        imageDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kCMSoundDescriptionFlavor_QuickTimeMovie: CMSoundDescriptionFlavor;
}
unsafe extern "C" {
    pub static kCMSoundDescriptionFlavor_QuickTimeMovieV2: CMSoundDescriptionFlavor;
}
unsafe extern "C" {
    pub static kCMSoundDescriptionFlavor_ISOFamily: CMSoundDescriptionFlavor;
}
unsafe extern "C" {
    pub static kCMSoundDescriptionFlavor_3GPFamily: CMSoundDescriptionFlavor;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData(
        allocator: CFAllocatorRef,
        soundDescriptionData: *const u8,
        size: usize,
        flavor: CMSoundDescriptionFlavor,
        formatDescriptionOut: *mut CMAudioFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        soundDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CMSoundDescriptionFlavor,
        formatDescriptionOut: *mut CMAudioFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        audioFormatDescription: CMAudioFormatDescriptionRef,
        flavor: CMSoundDescriptionFlavor,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMDoesBigEndianSoundDescriptionRequireLegacyCBRSampleTableLayout(
        soundDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CMSoundDescriptionFlavor,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMSwapBigEndianSoundDescriptionToHost(
        soundDescriptionData: *mut u8,
        soundDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapHostEndianSoundDescriptionToBig(
        soundDescriptionData: *mut u8,
        soundDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTextFormatDescriptionCreateFromBigEndianTextDescriptionData(
        allocator: CFAllocatorRef,
        textDescriptionData: *const u8,
        size: usize,
        flavor: CMTextDescriptionFlavor,
        mediaType: CMMediaType,
        formatDescriptionOut: *mut CMTextFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTextFormatDescriptionCreateFromBigEndianTextDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        textDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CMTextDescriptionFlavor,
        mediaType: CMMediaType,
        formatDescriptionOut: *mut CMTextFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTextFormatDescriptionCopyAsBigEndianTextDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        textFormatDescription: CMTextFormatDescriptionRef,
        flavor: CMTextDescriptionFlavor,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapBigEndianTextDescriptionToHost(
        textDescriptionData: *mut u8,
        textDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapHostEndianTextDescriptionToBig(
        textDescriptionData: *mut u8,
        textDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionData(
        allocator: CFAllocatorRef,
        closedCaptionDescriptionData: *const u8,
        size: usize,
        flavor: CMClosedCaptionDescriptionFlavor,
        formatDescriptionOut: *mut CMClosedCaptionFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMClosedCaptionFormatDescriptionCreateFromBigEndianClosedCaptionDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        closedCaptionDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CMClosedCaptionDescriptionFlavor,
        formatDescriptionOut: *mut CMClosedCaptionFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMClosedCaptionFormatDescriptionCopyAsBigEndianClosedCaptionDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        closedCaptionFormatDescription: CMClosedCaptionFormatDescriptionRef,
        flavor: CMClosedCaptionDescriptionFlavor,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapBigEndianClosedCaptionDescriptionToHost(
        closedCaptionDescriptionData: *mut u8,
        closedCaptionDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapHostEndianClosedCaptionDescriptionToBig(
        closedCaptionDescriptionData: *mut u8,
        closedCaptionDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionData(
        allocator: CFAllocatorRef,
        timeCodeDescriptionData: *const u8,
        size: usize,
        flavor: CMTimeCodeDescriptionFlavor,
        formatDescriptionOut: *mut CMTimeCodeFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimeCodeFormatDescriptionCreateFromBigEndianTimeCodeDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        timeCodeDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CMTimeCodeDescriptionFlavor,
        formatDescriptionOut: *mut CMTimeCodeFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimeCodeFormatDescriptionCopyAsBigEndianTimeCodeDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        timeCodeFormatDescription: CMTimeCodeFormatDescriptionRef,
        flavor: CMTimeCodeDescriptionFlavor,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapBigEndianTimeCodeDescriptionToHost(
        timeCodeDescriptionData: *mut u8,
        timeCodeDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapHostEndianTimeCodeDescriptionToBig(
        timeCodeDescriptionData: *mut u8,
        timeCodeDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionData(
        allocator: CFAllocatorRef,
        metadataDescriptionData: *const u8,
        size: usize,
        flavor: CMMetadataDescriptionFlavor,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataFormatDescriptionCreateFromBigEndianMetadataDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        metadataDescriptionBlockBuffer: CMBlockBufferRef,
        flavor: CMMetadataDescriptionFlavor,
        formatDescriptionOut: *mut CMMetadataFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataFormatDescriptionCopyAsBigEndianMetadataDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        metadataFormatDescription: CMMetadataFormatDescriptionRef,
        flavor: CMMetadataDescriptionFlavor,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapBigEndianMetadataDescriptionToHost(
        metadataDescriptionData: *mut u8,
        metadataDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSwapHostEndianMetadataDescriptionToBig(
        metadataDescriptionData: *mut u8,
        metadataDescriptionSize: usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetCallbacksForUnsortedSampleBuffers() -> *const CMBufferCallbacks;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetCallbacksForSampleBuffersSortedByOutputPTS() -> *const CMBufferCallbacks;
}
unsafe extern "C" {
    pub fn CMBufferQueueCreate(
        allocator: CFAllocatorRef,
        capacity: CMItemCount,
        callbacks: *const CMBufferCallbacks,
        queueOut: *mut CMBufferQueueRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueCreateWithHandlers(
        allocator: CFAllocatorRef,
        capacity: CMItemCount,
        handlers: *const CMBufferHandlers,
        queueOut: *mut CMBufferQueueRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMBufferQueueEnqueue(queue: CMBufferQueueRef, buf: CMBufferRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueDequeueAndRetain(queue: CMBufferQueueRef) -> CMBufferRef;
}
unsafe extern "C" {
    pub fn CMBufferQueueDequeueIfDataReadyAndRetain(queue: CMBufferQueueRef) -> CMBufferRef;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetHead(queue: CMBufferQueueRef) -> CMBufferRef;
}
unsafe extern "C" {
    pub fn CMBufferQueueCopyHead(queue: CMBufferQueueRef) -> CMBufferRef;
}
unsafe extern "C" {
    pub fn CMBufferQueueIsEmpty(queue: CMBufferQueueRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CMBufferQueueMarkEndOfData(queue: CMBufferQueueRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueContainsEndOfData(queue: CMBufferQueueRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CMBufferQueueIsAtEndOfData(queue: CMBufferQueueRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CMBufferQueueReset(queue: CMBufferQueueRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueResetWithCallback(
        queue: CMBufferQueueRef,
        callback: ::std::option::Option<
            unsafe extern "C" fn(buffer: CMBufferRef, refcon: *mut ::std::os::raw::c_void),
        >,
        refcon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetBufferCount(queue: CMBufferQueueRef) -> CMItemCount;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetDuration(queue: CMBufferQueueRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetMinDecodeTimeStamp(queue: CMBufferQueueRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetFirstDecodeTimeStamp(queue: CMBufferQueueRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetMinPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetFirstPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetMaxPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetEndPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMBufferQueueGetTotalSize(queue: CMBufferQueueRef) -> usize;
}
unsafe extern "C" {
    pub fn CMBufferQueueInstallTrigger(
        queue: CMBufferQueueRef,
        callback: CMBufferQueueTriggerCallback,
        refcon: *mut ::std::os::raw::c_void,
        condition: CMBufferQueueTriggerCondition,
        time: CMTime,
        triggerTokenOut: *mut CMBufferQueueTriggerToken,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueInstallTriggerWithIntegerThreshold(
        queue: CMBufferQueueRef,
        callback: CMBufferQueueTriggerCallback,
        refcon: *mut ::std::os::raw::c_void,
        condition: CMBufferQueueTriggerCondition,
        threshold: CMItemCount,
        triggerTokenOut: *mut CMBufferQueueTriggerToken,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueInstallTriggerHandler(
        queue: CMBufferQueueRef,
        condition: CMBufferQueueTriggerCondition,
        time: CMTime,
        triggerTokenOut: *mut CMBufferQueueTriggerToken,
        handler: CMBufferQueueTriggerHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueInstallTriggerHandlerWithIntegerThreshold(
        queue: CMBufferQueueRef,
        condition: CMBufferQueueTriggerCondition,
        threshold: CMItemCount,
        triggerTokenOut: *mut CMBufferQueueTriggerToken,
        handler: CMBufferQueueTriggerHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueRemoveTrigger(
        queue: CMBufferQueueRef,
        triggerToken: CMBufferQueueTriggerToken,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueTestTrigger(
        queue: CMBufferQueueRef,
        triggerToken: CMBufferQueueTriggerToken,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMBufferQueueCallForEachBuffer(
        queue: CMBufferQueueRef,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                buffer: CMBufferRef,
                refcon: *mut ::std::os::raw::c_void,
            ) -> OSStatus,
        >,
        refcon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueSetValidationCallback(
        queue: CMBufferQueueRef,
        callback: CMBufferValidationCallback,
        refcon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMBufferQueueSetValidationHandler(
        queue: CMBufferQueueRef,
        handler: CMBufferValidationHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kCMTimingInfoInvalid: CMSampleTimingInfo;
}
unsafe extern "C" {
    pub fn CMSampleBufferCreate(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: Boolean,
        makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
        makeDataReadyRefcon: *mut ::std::os::raw::c_void,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        numSampleTimingEntries: CMItemCount,
        sampleTimingArray: *const CMSampleTimingInfo,
        numSampleSizeEntries: CMItemCount,
        sampleSizeArray: *const usize,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCreateWithMakeDataReadyHandler(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: Boolean,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        numSampleTimingEntries: CMItemCount,
        sampleTimingArray: *const CMSampleTimingInfo,
        numSampleSizeEntries: CMItemCount,
        sampleSizeArray: *const usize,
        sampleBufferOut: *mut CMSampleBufferRef,
        makeDataReadyHandler: CMSampleBufferMakeDataReadyHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCreateReady(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        numSampleTimingEntries: CMItemCount,
        sampleTimingArray: *const CMSampleTimingInfo,
        numSampleSizeEntries: CMItemCount,
        sampleSizeArray: *const usize,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioSampleBufferCreateWithPacketDescriptions(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: Boolean,
        makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
        makeDataReadyRefcon: *mut ::std::os::raw::c_void,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        presentationTimeStamp: CMTime,
        packetDescriptions: *const AudioStreamPacketDescription,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioSampleBufferCreateWithPacketDescriptionsAndMakeDataReadyHandler(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: Boolean,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        presentationTimeStamp: CMTime,
        packetDescriptions: *const AudioStreamPacketDescription,
        sampleBufferOut: *mut CMSampleBufferRef,
        makeDataReadyHandler: CMSampleBufferMakeDataReadyHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioSampleBufferCreateReadyWithPacketDescriptions(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        presentationTimeStamp: CMTime,
        packetDescriptions: *const AudioStreamPacketDescription,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCreateForImageBuffer(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        dataReady: Boolean,
        makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
        makeDataReadyRefcon: *mut ::std::os::raw::c_void,
        formatDescription: CMVideoFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCreateForImageBufferWithMakeDataReadyHandler(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        dataReady: Boolean,
        formatDescription: CMVideoFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
        makeDataReadyHandler: CMSampleBufferMakeDataReadyHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCreateReadyWithImageBuffer(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        formatDescription: CMVideoFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCreateCopy(
        allocator: CFAllocatorRef,
        sbuf: CMSampleBufferRef,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCreateCopyWithNewTiming(
        allocator: CFAllocatorRef,
        originalSBuf: CMSampleBufferRef,
        numSampleTimingEntries: CMItemCount,
        sampleTimingArray: *const CMSampleTimingInfo,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCopySampleBufferForRange(
        allocator: CFAllocatorRef,
        sbuf: CMSampleBufferRef,
        sampleRange: CFRange,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMSampleBufferSetDataBuffer(
        sbuf: CMSampleBufferRef,
        dataBuffer: CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetDataBuffer(sbuf: CMSampleBufferRef) -> CMBlockBufferRef;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetImageBuffer(sbuf: CMSampleBufferRef) -> CVImageBufferRef;
}
unsafe extern "C" {
    pub fn CMSampleBufferSetDataBufferFromAudioBufferList(
        sbuf: CMSampleBufferRef,
        blockBufferStructureAllocator: CFAllocatorRef,
        blockBufferBlockAllocator: CFAllocatorRef,
        flags: u32,
        bufferList: *const AudioBufferList,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetAudioBufferListWithRetainedBlockBuffer(
        sbuf: CMSampleBufferRef,
        bufferListSizeNeededOut: *mut usize,
        bufferListOut: *mut AudioBufferList,
        bufferListSize: usize,
        blockBufferStructureAllocator: CFAllocatorRef,
        blockBufferBlockAllocator: CFAllocatorRef,
        flags: u32,
        blockBufferOut: *mut CMBlockBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetAudioStreamPacketDescriptions(
        sbuf: CMSampleBufferRef,
        packetDescriptionsSize: usize,
        packetDescriptionsOut: *mut AudioStreamPacketDescription,
        packetDescriptionsSizeNeededOut: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetAudioStreamPacketDescriptionsPtr(
        sbuf: CMSampleBufferRef,
        packetDescriptionsPointerOut: *mut *const AudioStreamPacketDescription,
        packetDescriptionsSizeOut: *mut usize,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCopyPCMDataIntoAudioBufferList(
        sbuf: CMSampleBufferRef,
        frameOffset: i32,
        numFrames: i32,
        bufferList: *mut AudioBufferList,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferSetDataReady(sbuf: CMSampleBufferRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferDataIsReady(sbuf: CMSampleBufferRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CMSampleBufferSetDataFailed(sbuf: CMSampleBufferRef, status: OSStatus) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferHasDataFailed(
        sbuf: CMSampleBufferRef,
        statusOut: *mut OSStatus,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMSampleBufferMakeDataReady(sbuf: CMSampleBufferRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferTrackDataReadiness(
        sbuf: CMSampleBufferRef,
        sampleBufferToTrack: CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferInvalidate(sbuf: CMSampleBufferRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferSetInvalidateCallback(
        sbuf: CMSampleBufferRef,
        invalidateCallback: CMSampleBufferInvalidateCallback,
        invalidateRefCon: u64,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferSetInvalidateHandler(
        sbuf: CMSampleBufferRef,
        invalidateHandler: CMSampleBufferInvalidateHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferIsValid(sbuf: CMSampleBufferRef) -> Boolean;
}
unsafe extern "C" {
    pub static kCMSampleBufferNotification_DataBecameReady: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferNotification_DataFailed: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferNotificationParameter_OSStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferConduitNotification_InhibitOutputUntil: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferConduitNotificationParameter_ResumeTag: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferConduitNotification_ResetOutput: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferConduitNotification_UpcomingOutputPTSRangeChanged: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferConduitNotificationParameter_UpcomingOutputPTSRangeMayOverlapQueuedOutputPTSRange:
        CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferConduitNotificationParameter_MinUpcomingOutputPTS: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferConduitNotificationParameter_MaxUpcomingOutputPTS: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferConsumerNotification_BufferConsumed: CFStringRef;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetNumSamples(sbuf: CMSampleBufferRef) -> CMItemCount;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetDuration(sbuf: CMSampleBufferRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetPresentationTimeStamp(sbuf: CMSampleBufferRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetDecodeTimeStamp(sbuf: CMSampleBufferRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetOutputDuration(sbuf: CMSampleBufferRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetOutputPresentationTimeStamp(sbuf: CMSampleBufferRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMSampleBufferSetOutputPresentationTimeStamp(
        sbuf: CMSampleBufferRef,
        outputPresentationTimeStamp: CMTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetOutputDecodeTimeStamp(sbuf: CMSampleBufferRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetSampleTimingInfoArray(
        sbuf: CMSampleBufferRef,
        numSampleTimingEntries: CMItemCount,
        timingArrayOut: *mut CMSampleTimingInfo,
        timingArrayEntriesNeededOut: *mut CMItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetOutputSampleTimingInfoArray(
        sbuf: CMSampleBufferRef,
        timingArrayEntries: CMItemCount,
        timingArrayOut: *mut CMSampleTimingInfo,
        timingArrayEntriesNeededOut: *mut CMItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetSampleTimingInfo(
        sbuf: CMSampleBufferRef,
        sampleIndex: CMItemIndex,
        timingInfoOut: *mut CMSampleTimingInfo,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetSampleSizeArray(
        sbuf: CMSampleBufferRef,
        sizeArrayEntries: CMItemCount,
        sizeArrayOut: *mut usize,
        sizeArrayEntriesNeededOut: *mut CMItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetSampleSize(sbuf: CMSampleBufferRef, sampleIndex: CMItemIndex) -> usize;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetTotalSampleSize(sbuf: CMSampleBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetFormatDescription(sbuf: CMSampleBufferRef) -> CMFormatDescriptionRef;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetSampleAttachmentsArray(
        sbuf: CMSampleBufferRef,
        createIfNecessary: Boolean,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_NotSync: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_PartialSync: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_HasRedundantCoding: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_IsDependedOnByOthers: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_DependsOnOthers: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_EarlierDisplayTimesAllowed: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_DisplayImmediately: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_DoNotDisplay: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_ResetDecoderBeforeDecoding: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_DrainAfterDecoding: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_PostNotificationWhenConsumed: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_ResumeOutput: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_HEVCTemporalLevelInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCMHEVCTemporalLevelInfoKey_TemporalLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kCMHEVCTemporalLevelInfoKey_ProfileSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kCMHEVCTemporalLevelInfoKey_TierFlag: CFStringRef;
}
unsafe extern "C" {
    pub static kCMHEVCTemporalLevelInfoKey_ProfileIndex: CFStringRef;
}
unsafe extern "C" {
    pub static kCMHEVCTemporalLevelInfoKey_ProfileCompatibilityFlags: CFStringRef;
}
unsafe extern "C" {
    pub static kCMHEVCTemporalLevelInfoKey_ConstraintIndicatorFlags: CFStringRef;
}
unsafe extern "C" {
    pub static kCMHEVCTemporalLevelInfoKey_LevelIndex: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_HEVCTemporalSubLayerAccess: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_HEVCStepwiseTemporalSubLayerAccess: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_HEVCSyncSampleNALUnitType: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_AudioIndependentSampleDecoderRefreshCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_TransitionID: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_TrimDurationAtStart: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_TrimDurationAtEnd: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_SpeedMultiplier: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_Reverse: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_FillDiscontinuitiesWithSilence: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_EmptyMedia: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_PermanentEmptyMedia: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_DisplayEmptyMediaImmediately: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_EndsPreviousSampleDuration: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_SampleReferenceURL: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_SampleReferenceByteOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_GradualDecoderRefresh: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_DroppedFrameReason: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferDroppedFrameReason_FrameWasLate: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferDroppedFrameReason_OutOfBuffers: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferDroppedFrameReason_Discontinuity: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_DroppedFrameReasonInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferDroppedFrameReasonInfo_CameraModeSwitch: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_StillImageLensStabilizationInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferLensStabilizationInfo_Active: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferLensStabilizationInfo_OutOfRange: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferLensStabilizationInfo_Unavailable: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferLensStabilizationInfo_Off: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_CameraIntrinsicMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleBufferAttachmentKey_ForceKeyFrame: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_CryptorSubsampleAuxiliaryData: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_HDR10PlusPerFrameData: CFStringRef;
}
unsafe extern "C" {
    pub static kCMSampleAttachmentKey_PostDecodeProcessingMetadata: CFStringRef;
}
unsafe extern "C" {
    pub fn CMSampleBufferCallForEachSample(
        sbuf: CMSampleBufferRef,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                sampleBuffer: CMSampleBufferRef,
                index: CMItemCount,
                refcon: *mut ::std::os::raw::c_void,
            ) -> OSStatus,
        >,
        refcon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferCallBlockForEachSample(
        sbuf: CMSampleBufferRef,
        handler: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagGetValueDataType(tag: CMTag) -> CMTagDataType;
}
unsafe extern "C" {
    pub static kCMTagInvalid: CMTag;
}
unsafe extern "C" {
    pub static kCMTagMediaTypeVideo: CMTag;
}
unsafe extern "C" {
    pub static kCMTagMediaSubTypeMebx: CMTag;
}
unsafe extern "C" {
    pub static kCMTagMediaTypeAudio: CMTag;
}
unsafe extern "C" {
    pub static kCMTagMediaTypeMetadata: CMTag;
}
unsafe extern "C" {
    pub static kCMTagStereoLeftEye: CMTag;
}
unsafe extern "C" {
    pub static kCMTagStereoRightEye: CMTag;
}
unsafe extern "C" {
    pub static kCMTagStereoLeftAndRightEye: CMTag;
}
unsafe extern "C" {
    pub static kCMTagStereoNone: CMTag;
}
unsafe extern "C" {
    pub static kCMTagStereoInterpretationOrderReversed: CMTag;
}
unsafe extern "C" {
    pub static kCMTagProjectionTypeRectangular: CMTag;
}
unsafe extern "C" {
    pub static kCMTagProjectionTypeEquirectangular: CMTag;
}
unsafe extern "C" {
    pub static kCMTagProjectionTypeHalfEquirectangular: CMTag;
}
unsafe extern "C" {
    pub static kCMTagProjectionTypeFisheye: CMTag;
}
unsafe extern "C" {
    pub static kCMTagProjectionTypeParametricImmersive: CMTag;
}
unsafe extern "C" {
    pub static kCMTagPackingTypeNone: CMTag;
}
unsafe extern "C" {
    pub static kCMTagPackingTypeSideBySide: CMTag;
}
unsafe extern "C" {
    pub static kCMTagPackingTypeOverUnder: CMTag;
}
unsafe extern "C" {
    pub fn CMTagHasSInt64Value(tag: CMTag) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagGetSInt64Value(tag: CMTag) -> i64;
}
unsafe extern "C" {
    pub fn CMTagHasFloat64Value(tag: CMTag) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagGetFloat64Value(tag: CMTag) -> Float64;
}
unsafe extern "C" {
    pub fn CMTagHasOSTypeValue(tag: CMTag) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagGetOSTypeValue(tag: CMTag) -> OSType;
}
unsafe extern "C" {
    pub fn CMTagHasFlagsValue(tag: CMTag) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagGetFlagsValue(tag: CMTag) -> u64;
}
unsafe extern "C" {
    pub fn CMTagMakeWithSInt64Value(category: CMTagCategory, value: i64) -> CMTag;
}
unsafe extern "C" {
    pub fn CMTagMakeWithFloat64Value(category: CMTagCategory, value: Float64) -> CMTag;
}
unsafe extern "C" {
    pub fn CMTagMakeWithOSTypeValue(category: CMTagCategory, value: OSType) -> CMTag;
}
unsafe extern "C" {
    pub fn CMTagMakeWithFlagsValue(category: CMTagCategory, flagsForTag: u64) -> CMTag;
}
unsafe extern "C" {
    pub fn CMTagEqualToTag(tag1: CMTag, tag2: CMTag) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagCompare(tag1: CMTag, tag2: CMTag) -> CFComparisonResult;
}
unsafe extern "C" {
    pub fn CMTagHash(tag: CMTag) -> CFHashCode;
}
unsafe extern "C" {
    pub fn CMTagCopyDescription(allocator: CFAllocatorRef, tag: CMTag) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CMTagCopyAsDictionary(tag: CMTag, allocator: CFAllocatorRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CMTagMakeFromDictionary(dict: CFDictionaryRef) -> CMTag;
}
unsafe extern "C" {
    pub static kCMTagValueKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTagCategoryKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTagDataTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CMTagCollectionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreate(
        allocator: CFAllocatorRef,
        tags: *const CMTag,
        tagCount: CMItemCount,
        newCollectionOut: *mut CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        newMutableCollectionOut: *mut CMMutableTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreateCopy(
        tagCollection: CMTagCollectionRef,
        allocator: CFAllocatorRef,
        newCollectionCopyOut: *mut CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreateMutableCopy(
        tagCollection: CMTagCollectionRef,
        allocator: CFAllocatorRef,
        newMutableCollectionCopyOut: *mut CMMutableTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCopyDescription(
        allocator: CFAllocatorRef,
        tagCollection: CMTagCollectionRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CMTagCollectionGetCount(tagCollection: CMTagCollectionRef) -> CMItemCount;
}
unsafe extern "C" {
    pub fn CMTagCollectionContainsTag(tagCollection: CMTagCollectionRef, tag: CMTag) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagCollectionContainsTagsOfCollection(
        tagCollection: CMTagCollectionRef,
        containedTagCollection: CMTagCollectionRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagCollectionContainsSpecifiedTags(
        tagCollection: CMTagCollectionRef,
        containedTags: *const CMTag,
        containedTagCount: CMItemCount,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagCollectionContainsCategory(
        tagCollection: CMTagCollectionRef,
        category: CMTagCategory,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagCollectionGetCountOfCategory(
        tagCollection: CMTagCollectionRef,
        category: CMTagCategory,
    ) -> CMItemCount;
}
unsafe extern "C" {
    pub fn CMTagCollectionGetTags(
        tagCollection: CMTagCollectionRef,
        tagBuffer: *mut CMTag,
        tagBufferCount: CMItemCount,
        numberOfTagsCopied: *mut CMItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionGetTagsWithCategory(
        tagCollection: CMTagCollectionRef,
        category: CMTagCategory,
        tagBuffer: *mut CMTag,
        tagBufferCount: CMItemCount,
        numberOfTagsCopied: *mut CMItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCountTagsWithFilterFunction(
        tagCollection: CMTagCollectionRef,
        filterApplier: CMTagCollectionTagFilterFunction,
        context: *mut ::std::os::raw::c_void,
    ) -> CMItemCount;
}
unsafe extern "C" {
    pub fn CMTagCollectionGetTagsWithFilterFunction(
        tagCollection: CMTagCollectionRef,
        tagBuffer: *mut CMTag,
        tagBufferCount: CMItemCount,
        numberOfTagsCopied: *mut CMItemCount,
        filter: CMTagCollectionTagFilterFunction,
        context: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCopyTagsOfCategories(
        allocator: CFAllocatorRef,
        tagCollection: CMTagCollectionRef,
        categories: *const CMTagCategory,
        categoriesCount: CMItemCount,
        collectionWithTagsOfCategories: *mut CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionApply(
        tagCollection: CMTagCollectionRef,
        applier: CMTagCollectionApplierFunction,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CMTagCollectionApplyUntil(
        tagCollection: CMTagCollectionRef,
        applier: CMTagCollectionTagFilterFunction,
        context: *mut ::std::os::raw::c_void,
    ) -> CMTag;
}
unsafe extern "C" {
    pub fn CMTagCollectionIsEmpty(tagCollection: CMTagCollectionRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreateIntersection(
        tagCollection1: CMTagCollectionRef,
        tagCollection2: CMTagCollectionRef,
        tagCollectionOut: *mut CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreateUnion(
        tagCollection1: CMTagCollectionRef,
        tagCollection2: CMTagCollectionRef,
        tagCollectionOut: *mut CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreateDifference(
        tagCollectionMinuend: CMTagCollectionRef,
        tagCollectionSubtrahend: CMTagCollectionRef,
        tagCollectionOut: *mut CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreateExclusiveOr(
        tagCollection1: CMTagCollectionRef,
        tagCollection2: CMTagCollectionRef,
        tagCollectionOut: *mut CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionAddTag(
        tagCollection: CMMutableTagCollectionRef,
        tagToAdd: CMTag,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionRemoveTag(
        tagCollection: CMMutableTagCollectionRef,
        tagToRemove: CMTag,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionRemoveAllTags(tagCollection: CMMutableTagCollectionRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionRemoveAllTagsOfCategory(
        tagCollection: CMMutableTagCollectionRef,
        category: CMTagCategory,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionAddTagsFromCollection(
        tagCollection: CMMutableTagCollectionRef,
        collectionWithTagsToAdd: CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionAddTagsFromArray(
        tagCollection: CMMutableTagCollectionRef,
        tags: *mut CMTag,
        tagCount: CMItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCopyAsDictionary(
        tagCollection: CMTagCollectionRef,
        allocator: CFAllocatorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreateFromDictionary(
        dict: CFDictionaryRef,
        allocator: CFAllocatorRef,
        newCollectionOut: *mut CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTagCollectionCopyAsData(
        tagCollection: CMTagCollectionRef,
        allocator: CFAllocatorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CMTagCollectionCreateFromData(
        data: CFDataRef,
        allocator: CFAllocatorRef,
        newCollectionOut: *mut CMTagCollectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kCMTagCollectionTagsArrayKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupCreate(
        allocator: CFAllocatorRef,
        tagCollections: CFArrayRef,
        buffers: CFArrayRef,
        groupOut: *mut CMTaggedBufferGroupRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupCreateCombined(
        allocator: CFAllocatorRef,
        taggedBufferGroups: CFArrayRef,
        groupOut: *mut CMTaggedBufferGroupRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetCount(group: CMTaggedBufferGroupRef) -> CMItemCount;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetTagCollectionAtIndex(
        group: CMTaggedBufferGroupRef,
        index: CFIndex,
    ) -> CMTagCollectionRef;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetCVPixelBufferAtIndex(
        group: CMTaggedBufferGroupRef,
        index: CFIndex,
    ) -> CVPixelBufferRef;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetCVPixelBufferForTag(
        group: CMTaggedBufferGroupRef,
        tag: CMTag,
        indexOut: *mut CFIndex,
    ) -> CVPixelBufferRef;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetCVPixelBufferForTagCollection(
        group: CMTaggedBufferGroupRef,
        tagCollection: CMTagCollectionRef,
        indexOut: *mut CFIndex,
    ) -> CVPixelBufferRef;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetCMSampleBufferAtIndex(
        group: CMTaggedBufferGroupRef,
        index: CFIndex,
    ) -> CMSampleBufferRef;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetCMSampleBufferForTag(
        group: CMTaggedBufferGroupRef,
        tag: CMTag,
        indexOut: *mut CFIndex,
    ) -> CMSampleBufferRef;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetCMSampleBufferForTagCollection(
        group: CMTaggedBufferGroupRef,
        tagCollection: CMTagCollectionRef,
        indexOut: *mut CFIndex,
    ) -> CMSampleBufferRef;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupGetNumberOfMatchesForTagCollection(
        group: CMTaggedBufferGroupRef,
        tagCollection: CMTagCollectionRef,
    ) -> CMItemCount;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupFormatDescriptionCreateForTaggedBufferGroup(
        allocator: CFAllocatorRef,
        taggedBufferGroup: CMTaggedBufferGroupRef,
        formatDescriptionOut: *mut CMTaggedBufferGroupFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupFormatDescriptionCreateForTaggedBufferGroupWithExtensions(
        allocator: CFAllocatorRef,
        taggedBufferGroup: CMTaggedBufferGroupRef,
        extensions: CFDictionaryRef,
        formatDescriptionOut: *mut CMTaggedBufferGroupFormatDescriptionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTaggedBufferGroupFormatDescriptionMatchesTaggedBufferGroup(
        desc: CMTaggedBufferGroupFormatDescriptionRef,
        taggedBufferGroup: CMTaggedBufferGroupRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMSampleBufferCreateForTaggedBufferGroup(
        allocator: CFAllocatorRef,
        taggedBufferGroup: CMTaggedBufferGroupRef,
        sbufPTS: CMTime,
        sbufDuration: CMTime,
        formatDescription: CMTaggedBufferGroupFormatDescriptionRef,
        sBufOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSampleBufferGetTaggedBufferGroup(sbuf: CMSampleBufferRef) -> CMTaggedBufferGroupRef;
}
unsafe extern "C" {
    pub fn CMSimpleQueueGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMSimpleQueueCreate(
        allocator: CFAllocatorRef,
        capacity: i32,
        queueOut: *mut CMSimpleQueueRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSimpleQueueEnqueue(
        queue: CMSimpleQueueRef,
        element: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSimpleQueueDequeue(queue: CMSimpleQueueRef) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CMSimpleQueueGetHead(queue: CMSimpleQueueRef) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CMSimpleQueueReset(queue: CMSimpleQueueRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSimpleQueueGetCapacity(queue: CMSimpleQueueRef) -> i32;
}
unsafe extern "C" {
    pub fn CMSimpleQueueGetCount(queue: CMSimpleQueueRef) -> i32;
}
unsafe extern "C" {
    pub fn CMMemoryPoolGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static mut kCMMemoryPoolOption_AgeOutPeriod: CFStringRef;
}
unsafe extern "C" {
    pub fn CMMemoryPoolCreate(options: CFDictionaryRef) -> CMMemoryPoolRef;
}
unsafe extern "C" {
    pub fn CMMemoryPoolGetAllocator(pool: CMMemoryPoolRef) -> CFAllocatorRef;
}
unsafe extern "C" {
    pub fn CMMemoryPoolFlush(pool: CMMemoryPoolRef);
}
unsafe extern "C" {
    pub fn CMMemoryPoolInvalidate(pool: CMMemoryPoolRef);
}
unsafe extern "C" {
    pub fn CMClockGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMClockGetHostTimeClock() -> CMClockRef;
}
unsafe extern "C" {
    pub fn CMClockConvertHostTimeToSystemUnits(hostTime: CMTime) -> u64;
}
unsafe extern "C" {
    pub fn CMClockMakeHostTimeFromSystemUnits(hostTime: u64) -> CMTime;
}
unsafe extern "C" {
    pub fn CMClockGetTime(clock: CMClockRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMClockGetAnchorTime(
        clock: CMClockRef,
        clockTimeOut: *mut CMTime,
        referenceClockTimeOut: *mut CMTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMClockMightDrift(clock: CMClockRef, otherClock: CMClockRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CMClockInvalidate(clock: CMClockRef);
}
unsafe extern "C" {
    pub fn CMTimebaseGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CMTimebaseCreateWithSourceClock(
        allocator: CFAllocatorRef,
        sourceClock: CMClockRef,
        timebaseOut: *mut CMTimebaseRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseCreateWithSourceTimebase(
        allocator: CFAllocatorRef,
        sourceTimebase: CMTimebaseRef,
        timebaseOut: *mut CMTimebaseRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseCopySourceTimebase(timebase: CMTimebaseRef) -> CMTimebaseRef;
}
unsafe extern "C" {
    pub fn CMTimebaseCopySourceClock(timebase: CMTimebaseRef) -> CMClockRef;
}
unsafe extern "C" {
    pub fn CMTimebaseCopySource(timebase: CMTimebaseRef) -> CMClockOrTimebaseRef;
}
unsafe extern "C" {
    pub fn CMTimebaseCopyUltimateSourceClock(timebase: CMTimebaseRef) -> CMClockRef;
}
unsafe extern "C" {
    pub fn CMTimebaseGetMasterTimebase(timebase: CMTimebaseRef) -> CMTimebaseRef;
}
unsafe extern "C" {
    pub fn CMTimebaseGetMasterClock(timebase: CMTimebaseRef) -> CMClockRef;
}
unsafe extern "C" {
    pub fn CMTimebaseGetMaster(timebase: CMTimebaseRef) -> CMClockOrTimebaseRef;
}
unsafe extern "C" {
    pub fn CMTimebaseGetUltimateMasterClock(timebase: CMTimebaseRef) -> CMClockRef;
}
unsafe extern "C" {
    pub fn CMTimebaseSetSourceClock(
        timebase: CMTimebaseRef,
        newSourceClock: CMClockRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseSetSourceTimebase(
        timebase: CMTimebaseRef,
        newSourceTimebase: CMTimebaseRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseGetTime(timebase: CMTimebaseRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimebaseGetTimeWithTimeScale(
        timebase: CMTimebaseRef,
        timescale: CMTimeScale,
        method: CMTimeRoundingMethod,
    ) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimebaseSetTime(timebase: CMTimebaseRef, time: CMTime) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseSetAnchorTime(
        timebase: CMTimebaseRef,
        timebaseTime: CMTime,
        immediateSourceTime: CMTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseGetRate(timebase: CMTimebaseRef) -> Float64;
}
unsafe extern "C" {
    pub fn CMTimebaseGetTimeAndRate(
        timebase: CMTimebaseRef,
        timeOut: *mut CMTime,
        rateOut: *mut Float64,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseSetRate(timebase: CMTimebaseRef, rate: Float64) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseSetRateAndAnchorTime(
        timebase: CMTimebaseRef,
        rate: Float64,
        timebaseTime: CMTime,
        immediateSourceTime: CMTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseGetEffectiveRate(timebase: CMTimebaseRef) -> Float64;
}
unsafe extern "C" {
    pub fn CMTimebaseAddTimer(
        timebase: CMTimebaseRef,
        timer: CFRunLoopTimerRef,
        runloop: CFRunLoopRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseRemoveTimer(timebase: CMTimebaseRef, timer: CFRunLoopTimerRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseSetTimerNextFireTime(
        timebase: CMTimebaseRef,
        timer: CFRunLoopTimerRef,
        fireTime: CMTime,
        flags: u32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseSetTimerToFireImmediately(
        timebase: CMTimebaseRef,
        timer: CFRunLoopTimerRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseAddTimerDispatchSource(
        timebase: CMTimebaseRef,
        timerSource: NSObject,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseRemoveTimerDispatchSource(
        timebase: CMTimebaseRef,
        timerSource: NSObject,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseSetTimerDispatchSourceNextFireTime(
        timebase: CMTimebaseRef,
        timerSource: NSObject,
        fireTime: CMTime,
        flags: u32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMTimebaseSetTimerDispatchSourceToFireImmediately(
        timebase: CMTimebaseRef,
        timerSource: NSObject,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSyncGetRelativeRate(
        ofClockOrTimebase: CMClockOrTimebaseRef,
        relativeToClockOrTimebase: CMClockOrTimebaseRef,
    ) -> Float64;
}
unsafe extern "C" {
    pub fn CMSyncGetRelativeRateAndAnchorTime(
        ofClockOrTimebase: CMClockOrTimebaseRef,
        relativeToClockOrTimebase: CMClockOrTimebaseRef,
        outRelativeRate: *mut Float64,
        outOfClockOrTimebaseAnchorTime: *mut CMTime,
        outRelativeToClockOrTimebaseAnchorTime: *mut CMTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMSyncConvertTime(
        time: CMTime,
        fromClockOrTimebase: CMClockOrTimebaseRef,
        toClockOrTimebase: CMClockOrTimebaseRef,
    ) -> CMTime;
}
unsafe extern "C" {
    pub fn CMSyncMightDrift(
        clockOrTimebase1: CMClockOrTimebaseRef,
        clockOrTimebase2: CMClockOrTimebaseRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMSyncGetTime(clockOrTimebase: CMClockOrTimebaseRef) -> CMTime;
}
unsafe extern "C" {
    pub fn CMTimebaseNotificationBarrier(timebase: CMTimebaseRef) -> OSStatus;
}
unsafe extern "C" {
    pub static kCMTimebaseNotification_EffectiveRateChanged: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTimebaseNotification_TimeJumped: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTimebaseNotificationKey_EventTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_ForegroundColorARGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_BackgroundColorARGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_CharacterBackgroundColorARGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_BoldStyle: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_ItalicStyle: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_UnderlineStyle: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_FontFamilyName: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_FontFamilyNameList: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_GenericFontFamilyName: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_Default: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_Serif: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_SansSerif: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_Monospace: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_ProportionalSerif: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_ProportionalSansSerif: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_MonospaceSerif: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_MonospaceSansSerif: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_Casual: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_Cursive: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_Fantasy: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupGenericFontName_SmallCapital: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_BaseFontSizePercentageRelativeToVideoHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_RelativeFontSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_VerticalLayout: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextVerticalLayout_LeftToRight: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextVerticalLayout_RightToLeft: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_Alignment: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAlignmentType_Start: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAlignmentType_Middle: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAlignmentType_End: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAlignmentType_Left: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAlignmentType_Right: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_TextPositionPercentageRelativeToWritingDirection: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_OrthogonalLinePositionPercentageRelativeToWritingDirection:
        CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_WritingDirectionSizePercentage: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupAttribute_CharacterEdgeStyle: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupCharacterEdgeStyle_None: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupCharacterEdgeStyle_Raised: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupCharacterEdgeStyle_Depressed: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupCharacterEdgeStyle_Uniform: CFStringRef;
}
unsafe extern "C" {
    pub static kCMTextMarkupCharacterEdgeStyle_DropShadow: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataKeySpace_QuickTimeUserData: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataKeySpace_ISOUserData: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataKeySpace_QuickTimeMetadata: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataKeySpace_iTunes: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataKeySpace_ID3: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataKeySpace_Icy: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataKeySpace_HLSDateRange: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataLocation_ISO6709: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataDirection_Facing: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataPreferredAffineTransform: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataVideoOrientation: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataLivePhotoStillImageTransform: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataLivePhotoStillImageTransformReferenceDimensions:
        CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataSegmentIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataSceneIlluminance: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataSpatialAudioMix: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleMono: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleStereoLeft: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataDisplayMaskRectangleStereoRight: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataIdentifier_QuickTimeMetadataPresentationImmersiveMedia: CFStringRef;
}
unsafe extern "C" {
    pub fn CMMetadataCreateIdentifierForKeyAndKeySpace(
        allocator: CFAllocatorRef,
        key: CFTypeRef,
        keySpace: CFStringRef,
        identifierOut: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataCreateKeyFromIdentifier(
        allocator: CFAllocatorRef,
        identifier: CFStringRef,
        keyOut: *mut CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataCreateKeyFromIdentifierAsCFData(
        allocator: CFAllocatorRef,
        identifier: CFStringRef,
        keyOut: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataCreateKeySpaceFromIdentifier(
        allocator: CFAllocatorRef,
        identifier: CFStringRef,
        keySpaceOut: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_RawData: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_UTF8: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_UTF16: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_GIF: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_JPEG: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_PNG: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_BMP: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_Float32: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_Float64: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_SInt8: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_SInt16: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_SInt32: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_SInt64: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_UInt8: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_UInt16: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_UInt32: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_UInt64: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_PointF32: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_DimensionsF32: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_RectF32: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_AffineTransformF64: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_PolygonF32: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_PolylineF32: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_JSON: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_PerspectiveTransformF64: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_RasterRectangleValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataBaseDataType_ExtendedRasterRectangleValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataDataType_QuickTimeMetadataLocation_ISO6709: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataDataType_QuickTimeMetadataDirection: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataDataType_QuickTimeMetadataUUID: CFStringRef;
}
unsafe extern "C" {
    pub static kCMMetadataDataType_QuickTimeMetadataMilliLux: CFStringRef;
}
unsafe extern "C" {
    pub fn CMMetadataDataTypeRegistryRegisterDataType(
        dataType: CFStringRef,
        description: CFStringRef,
        conformingDataTypes: CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMMetadataDataTypeRegistryDataTypeIsRegistered(dataType: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CMMetadataDataTypeRegistryGetDataTypeDescription(dataType: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CMMetadataDataTypeRegistryGetConformingDataTypes(dataType: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CMMetadataDataTypeRegistryDataTypeConformsToDataType(
        dataType: CFStringRef,
        conformsToDataType: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMMetadataDataTypeRegistryGetBaseDataTypes() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CMMetadataDataTypeRegistryDataTypeIsBaseDataType(dataType: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CMMetadataDataTypeRegistryGetBaseDataTypeForConformingDataType(
        dataType: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CMAudioClockCreate(allocator: CFAllocatorRef, clockOut: *mut CMClockRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioDeviceClockCreate(
        allocator: CFAllocatorRef,
        deviceUID: CFStringRef,
        clockOut: *mut CMClockRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioDeviceClockCreateFromAudioDeviceID(
        allocator: CFAllocatorRef,
        deviceID: AudioDeviceID,
        clockOut: *mut CMClockRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioDeviceClockSetAudioDeviceUID(
        clock: CMClockRef,
        deviceUID: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioDeviceClockSetAudioDeviceID(
        clock: CMClockRef,
        deviceID: AudioDeviceID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMAudioDeviceClockGetAudioDevice(
        clock: CMClockRef,
        deviceUIDOut: *mut CFStringRef,
        deviceIDOut: *mut AudioDeviceID,
        trackingDefaultDeviceOut: *mut Boolean,
    ) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for AudioFormatListItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFormatListItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFormatListItem", &[]);
}
unsafe impl objc2::encode::RefEncode for CMTime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMTime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMTime", &[]);
}
unsafe impl objc2::encode::RefEncode for CMTimeRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMTimeRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMTimeRange", &[]);
}
unsafe impl objc2::encode::RefEncode for CMTimeMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMTimeMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMTimeMapping", &[]);
}
unsafe impl objc2::encode::RefEncode for opaqueCMFormatDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for opaqueCMFormatDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("opaqueCMFormatDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for CMVideoDimensions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMVideoDimensions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMVideoDimensions", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCMBlockBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCMBlockBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCMBlockBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for CMBlockBufferCustomBlockSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMBlockBufferCustomBlockSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMBlockBufferCustomBlockSource", &[]);
}
unsafe impl objc2::encode::RefEncode for opaqueCMBufferQueue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for opaqueCMBufferQueue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("opaqueCMBufferQueue", &[]);
}
unsafe impl objc2::encode::RefEncode for CMBufferCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMBufferCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMBufferCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for CMBufferHandlers {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMBufferHandlers {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMBufferHandlers", &[]);
}
unsafe impl objc2::encode::RefEncode for opaqueCMBufferQueueTriggerToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for opaqueCMBufferQueueTriggerToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("opaqueCMBufferQueueTriggerToken", &[]);
}
unsafe impl objc2::encode::RefEncode for opaqueCMSampleBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for opaqueCMSampleBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("opaqueCMSampleBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for CMSampleTimingInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMSampleTimingInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMSampleTimingInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CMTag {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMTag {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMTag", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCMTagCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCMTagCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCMTagCollection", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCMTaggedBufferGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCMTaggedBufferGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCMTaggedBufferGroup", &[]);
}
unsafe impl objc2::encode::RefEncode for opaqueCMSimpleQueue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for opaqueCMSimpleQueue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("opaqueCMSimpleQueue", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCMMemoryPool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCMMemoryPool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCMMemoryPool", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCMClock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCMClock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCMClock", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueCMTimebase {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueCMTimebase {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueCMTimebase", &[]);
}
