#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTInt32Point {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTInt32Size {
    pub width: i32,
    pub height: i32,
}
pub type VTDecodeFrameFlags = u32;
pub type VTDecodeInfoFlags = UInt32;
pub type VTEncodeInfoFlags = UInt32;
pub type VTSessionRef = CFTypeRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVTCompressionSession {
    _unused: [u8; 0],
}
pub type VTCompressionSessionRef = *mut OpaqueVTCompressionSession;
pub type VTCompressionOutputCallback = ::std::option::Option<
    unsafe extern "C" fn(
        outputCallbackRefCon: *mut ::std::os::raw::c_void,
        sourceFrameRefCon: *mut ::std::os::raw::c_void,
        status: OSStatus,
        infoFlags: VTEncodeInfoFlags,
        sampleBuffer: CMSampleBufferRef,
    ),
>;
pub type VTCompressionOutputHandler = *mut ::std::os::raw::c_void;
pub type VTCompressionSessionOptionFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVTDecompressionSession {
    _unused: [u8; 0],
}
pub type VTDecompressionSessionRef = *mut OpaqueVTDecompressionSession;
pub type VTDecompressionOutputCallback = ::std::option::Option<
    unsafe extern "C" fn(
        decompressionOutputRefCon: *mut ::std::os::raw::c_void,
        sourceFrameRefCon: *mut ::std::os::raw::c_void,
        status: OSStatus,
        infoFlags: VTDecodeInfoFlags,
        imageBuffer: CVImageBufferRef,
        presentationTimeStamp: CMTime,
        presentationDuration: CMTime,
    ),
>;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct VTDecompressionOutputCallbackRecord {
    pub decompressionOutputCallback: VTDecompressionOutputCallback,
    pub decompressionOutputRefCon: *mut ::std::os::raw::c_void,
}
pub type VTDecompressionOutputHandler = *mut ::std::os::raw::c_void;
pub type VTDecompressionOutputMultiImageCallback = ::std::option::Option<
    unsafe extern "C" fn(
        decompressionOutputMultiImageRefCon: *mut ::std::os::raw::c_void,
        sourceFrameRefCon: *mut ::std::os::raw::c_void,
        status: OSStatus,
        infoFlags: VTDecodeInfoFlags,
        taggedBufferGroup: CMTaggedBufferGroupRef,
        presentationTimeStamp: CMTime,
        presentationDuration: CMTime,
    ),
>;
pub type VTDecompressionMultiImageCapableOutputHandler = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVTFrameSilo {
    _unused: [u8; 0],
}
pub type VTFrameSiloRef = *mut OpaqueVTFrameSilo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVTMultiPassStorage {
    _unused: [u8; 0],
}
pub type VTMultiPassStorageRef = *mut OpaqueVTMultiPassStorage;
pub type VTExtensionPropertiesKey = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVTPixelTransferSession {
    _unused: [u8; 0],
}
pub type VTPixelTransferSessionRef = *mut OpaqueVTPixelTransferSession;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVTPixelRotationSession {
    _unused: [u8; 0],
}
pub type VTPixelRotationSessionRef = *mut OpaqueVTPixelRotationSession;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVTRAWProcessingSession {
    _unused: [u8; 0],
}
pub type VTRAWProcessingSessionRef = *mut OpaqueVTRAWProcessingSession;
pub type VTRAWProcessingParameterChangeHandler = *mut ::std::os::raw::c_void;
pub type VTRAWProcessingOutputHandler = *mut ::std::os::raw::c_void;
pub type VTHDRPerFrameMetadataGenerationHDRFormatType = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVTHDRPerFrameMetadataGenerationSession {
    _unused: [u8; 0],
}
pub type VTHDRPerFrameMetadataGenerationSessionRef =
    *mut OpaqueVTHDRPerFrameMetadataGenerationSession;
pub type VTMotionEstimationFrameFlags = u32;
pub type VTMotionEstimationInfoFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVTMotionEstimationSession {
    _unused: [u8; 0],
}
pub type VTMotionEstimationSessionRef = *mut OpaqueVTMotionEstimationSession;
pub type VTMotionEstimationOutputHandler = *mut ::std::os::raw::c_void;
pub trait PVTFrameProcessorConfiguration: Sized + std::ops::Deref {
    unsafe fn frameSupportedPixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameSupportedPixelFormats)
    }
    unsafe fn sourcePixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePixelBufferAttributes)
    }
    unsafe fn destinationPixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPixelBufferAttributes)
    }
    unsafe fn nextFrameCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextFrameCount)
    }
    unsafe fn previousFrameCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousFrameCount)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameProcessorConfiguration").unwrap(), isSupported)
    }
    unsafe fn maximumDimensions() -> CMVideoDimensions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameProcessorConfiguration").unwrap(), maximumDimensions)
    }
    unsafe fn minimumDimensions() -> CMVideoDimensions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameProcessorConfiguration").unwrap(), minimumDimensions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTFrameProcessorFrame(pub id);
impl std::ops::Deref for VTFrameProcessorFrame {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTFrameProcessorFrame {}
impl VTFrameProcessorFrame {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameProcessorFrame").unwrap(), alloc) })
    }
}
impl INSObject for VTFrameProcessorFrame {}
impl PNSObject for VTFrameProcessorFrame {}
impl std::convert::TryFrom<NSObject> for VTFrameProcessorFrame {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTFrameProcessorFrame, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTFrameProcessorFrame").unwrap()) };
        if is_kind_of {
            Ok(VTFrameProcessorFrame(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTFrameProcessorFrame")
        }
    }
}
impl IVTFrameProcessorFrame for VTFrameProcessorFrame {}
pub trait IVTFrameProcessorFrame: Sized + std::ops::Deref {
    unsafe fn initWithBuffer_presentationTimeStamp_(
        &self,
        buffer: CVPixelBufferRef,
        presentationTimeStamp: CMTime,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBuffer : buffer, presentationTimeStamp : presentationTimeStamp)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn buffer(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buffer)
    }
    unsafe fn presentationTimeStamp(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationTimeStamp)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameProcessorFrame").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTFrameProcessorOpticalFlow(pub id);
impl std::ops::Deref for VTFrameProcessorOpticalFlow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTFrameProcessorOpticalFlow {}
impl VTFrameProcessorOpticalFlow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameProcessorOpticalFlow").unwrap(), alloc) })
    }
}
impl INSObject for VTFrameProcessorOpticalFlow {}
impl PNSObject for VTFrameProcessorOpticalFlow {}
impl std::convert::TryFrom<NSObject> for VTFrameProcessorOpticalFlow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTFrameProcessorOpticalFlow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTFrameProcessorOpticalFlow").unwrap()) };
        if is_kind_of {
            Ok(VTFrameProcessorOpticalFlow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTFrameProcessorOpticalFlow")
        }
    }
}
impl IVTFrameProcessorOpticalFlow for VTFrameProcessorOpticalFlow {}
pub trait IVTFrameProcessorOpticalFlow: Sized + std::ops::Deref {
    unsafe fn initWithForwardFlow_backwardFlow_(
        &self,
        forwardFlow: CVPixelBufferRef,
        backwardFlow: CVPixelBufferRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithForwardFlow : forwardFlow, backwardFlow : backwardFlow)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn forwardFlow(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forwardFlow)
    }
    unsafe fn backwardFlow(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backwardFlow)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameProcessorOpticalFlow").unwrap(), new)
    }
}
pub trait PVTFrameProcessorParameters: Sized + std::ops::Deref {
    unsafe fn sourceFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn destinationFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationFrame)
    }
    unsafe fn destinationFrames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationFrames)
    }
}
pub type VTFrameProcessorError = NSInteger;
pub type VTMotionBlurConfigurationQualityPrioritization = NSInteger;
pub type VTMotionBlurConfigurationRevision = NSInteger;
pub type VTMotionBlurParametersSubmissionMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTMotionBlurConfiguration(pub id);
impl std::ops::Deref for VTMotionBlurConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTMotionBlurConfiguration {}
impl VTMotionBlurConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTMotionBlurConfiguration").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorConfiguration for VTMotionBlurConfiguration {}
impl INSObject for VTMotionBlurConfiguration {}
impl PNSObject for VTMotionBlurConfiguration {}
impl std::convert::TryFrom<NSObject> for VTMotionBlurConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTMotionBlurConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTMotionBlurConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VTMotionBlurConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTMotionBlurConfiguration")
        }
    }
}
impl IVTMotionBlurConfiguration for VTMotionBlurConfiguration {}
pub trait IVTMotionBlurConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithFrameWidth_frameHeight_usePrecomputedFlow_qualityPrioritization_revision_(
        &self,
        frameWidth: NSInteger,
        frameHeight: NSInteger,
        usePrecomputedFlow: BOOL,
        qualityPrioritization: VTMotionBlurConfigurationQualityPrioritization,
        revision: VTMotionBlurConfigurationRevision,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameWidth : frameWidth, frameHeight : frameHeight, usePrecomputedFlow : usePrecomputedFlow, qualityPrioritization : qualityPrioritization, revision : revision)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn frameWidth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameWidth)
    }
    unsafe fn frameHeight(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameHeight)
    }
    unsafe fn usePrecomputedFlow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usePrecomputedFlow)
    }
    unsafe fn qualityPrioritization(&self) -> VTMotionBlurConfigurationQualityPrioritization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, qualityPrioritization)
    }
    unsafe fn revision(&self) -> VTMotionBlurConfigurationRevision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revision)
    }
    unsafe fn frameSupportedPixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameSupportedPixelFormats)
    }
    unsafe fn sourcePixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePixelBufferAttributes)
    }
    unsafe fn destinationPixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPixelBufferAttributes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTMotionBlurConfiguration").unwrap(), new)
    }
    unsafe fn supportedRevisions() -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTMotionBlurConfiguration").unwrap(), supportedRevisions)
    }
    unsafe fn defaultRevision() -> VTMotionBlurConfigurationRevision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTMotionBlurConfiguration").unwrap(), defaultRevision)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTMotionBlurConfiguration").unwrap(), isSupported)
    }
    unsafe fn processorSupported() -> Boolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTMotionBlurConfiguration").unwrap(), processorSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTMotionBlurParameters(pub id);
impl std::ops::Deref for VTMotionBlurParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTMotionBlurParameters {}
impl VTMotionBlurParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTMotionBlurParameters").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorParameters for VTMotionBlurParameters {}
impl INSObject for VTMotionBlurParameters {}
impl PNSObject for VTMotionBlurParameters {}
impl std::convert::TryFrom<NSObject> for VTMotionBlurParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTMotionBlurParameters, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTMotionBlurParameters").unwrap()) };
        if is_kind_of {
            Ok(VTMotionBlurParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTMotionBlurParameters")
        }
    }
}
impl IVTMotionBlurParameters for VTMotionBlurParameters {}
pub trait IVTMotionBlurParameters: Sized + std::ops::Deref {
    unsafe fn initWithSourceFrame_nextFrame_previousFrame_nextOpticalFlow_previousOpticalFlow_motionBlurStrength_submissionMode_destinationFrame_(
        &self,
        sourceFrame: VTFrameProcessorFrame,
        nextFrame: VTFrameProcessorFrame,
        previousFrame: VTFrameProcessorFrame,
        nextOpticalFlow: VTFrameProcessorOpticalFlow,
        previousOpticalFlow: VTFrameProcessorOpticalFlow,
        motionBlurStrength: NSInteger,
        submissionMode: VTMotionBlurParametersSubmissionMode,
        destinationFrame: VTFrameProcessorFrame,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSourceFrame : sourceFrame, nextFrame : nextFrame, previousFrame : previousFrame, nextOpticalFlow : nextOpticalFlow, previousOpticalFlow : previousOpticalFlow, motionBlurStrength : motionBlurStrength, submissionMode : submissionMode, destinationFrame : destinationFrame)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sourceFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn nextFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextFrame)
    }
    unsafe fn previousFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousFrame)
    }
    unsafe fn nextOpticalFlow(&self) -> VTFrameProcessorOpticalFlow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextOpticalFlow)
    }
    unsafe fn previousOpticalFlow(&self) -> VTFrameProcessorOpticalFlow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousOpticalFlow)
    }
    unsafe fn motionBlurStrength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionBlurStrength)
    }
    unsafe fn submissionMode(&self) -> VTMotionBlurParametersSubmissionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, submissionMode)
    }
    unsafe fn destinationFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationFrame)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTMotionBlurParameters").unwrap(), new)
    }
}
pub type VTFrameRateConversionConfigurationQualityPrioritization = NSInteger;
pub type VTFrameRateConversionConfigurationRevision = NSInteger;
pub type VTFrameRateConversionParametersSubmissionMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTFrameRateConversionConfiguration(pub id);
impl std::ops::Deref for VTFrameRateConversionConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTFrameRateConversionConfiguration {}
impl VTFrameRateConversionConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameRateConversionConfiguration").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorConfiguration for VTFrameRateConversionConfiguration {}
impl INSObject for VTFrameRateConversionConfiguration {}
impl PNSObject for VTFrameRateConversionConfiguration {}
impl std::convert::TryFrom<NSObject> for VTFrameRateConversionConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTFrameRateConversionConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTFrameRateConversionConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VTFrameRateConversionConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTFrameRateConversionConfiguration")
        }
    }
}
impl IVTFrameRateConversionConfiguration for VTFrameRateConversionConfiguration {}
pub trait IVTFrameRateConversionConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithFrameWidth_frameHeight_usePrecomputedFlow_qualityPrioritization_revision_(
        &self,
        frameWidth: NSInteger,
        frameHeight: NSInteger,
        usePrecomputedFlow: BOOL,
        qualityPrioritization: VTFrameRateConversionConfigurationQualityPrioritization,
        revision: VTFrameRateConversionConfigurationRevision,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameWidth : frameWidth, frameHeight : frameHeight, usePrecomputedFlow : usePrecomputedFlow, qualityPrioritization : qualityPrioritization, revision : revision)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn frameWidth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameWidth)
    }
    unsafe fn frameHeight(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameHeight)
    }
    unsafe fn usePrecomputedFlow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usePrecomputedFlow)
    }
    unsafe fn qualityPrioritization(
        &self,
    ) -> VTFrameRateConversionConfigurationQualityPrioritization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, qualityPrioritization)
    }
    unsafe fn revision(&self) -> VTFrameRateConversionConfigurationRevision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revision)
    }
    unsafe fn frameSupportedPixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameSupportedPixelFormats)
    }
    unsafe fn sourcePixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePixelBufferAttributes)
    }
    unsafe fn destinationPixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPixelBufferAttributes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameRateConversionConfiguration").unwrap(), new)
    }
    unsafe fn supportedRevisions() -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameRateConversionConfiguration").unwrap(), supportedRevisions)
    }
    unsafe fn defaultRevision() -> VTFrameRateConversionConfigurationRevision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameRateConversionConfiguration").unwrap(), defaultRevision)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameRateConversionConfiguration").unwrap(), isSupported)
    }
    unsafe fn processorSupported() -> Boolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameRateConversionConfiguration").unwrap(), processorSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTFrameRateConversionParameters(pub id);
impl std::ops::Deref for VTFrameRateConversionParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTFrameRateConversionParameters {}
impl VTFrameRateConversionParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameRateConversionParameters").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorParameters for VTFrameRateConversionParameters {}
impl INSObject for VTFrameRateConversionParameters {}
impl PNSObject for VTFrameRateConversionParameters {}
impl std::convert::TryFrom<NSObject> for VTFrameRateConversionParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTFrameRateConversionParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTFrameRateConversionParameters").unwrap())
        };
        if is_kind_of {
            Ok(VTFrameRateConversionParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTFrameRateConversionParameters")
        }
    }
}
impl IVTFrameRateConversionParameters for VTFrameRateConversionParameters {}
pub trait IVTFrameRateConversionParameters: Sized + std::ops::Deref {
    unsafe fn initWithSourceFrame_nextFrame_opticalFlow_interpolationPhase_submissionMode_destinationFrames_(
        &self,
        sourceFrame: VTFrameProcessorFrame,
        nextFrame: VTFrameProcessorFrame,
        opticalFlow: VTFrameProcessorOpticalFlow,
        interpolationPhase: NSArray,
        submissionMode: VTFrameRateConversionParametersSubmissionMode,
        destinationFrame: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSourceFrame : sourceFrame, nextFrame : nextFrame, opticalFlow : opticalFlow, interpolationPhase : interpolationPhase, submissionMode : submissionMode, destinationFrames : destinationFrame)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sourceFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn nextFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextFrame)
    }
    unsafe fn opticalFlow(&self) -> VTFrameProcessorOpticalFlow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opticalFlow)
    }
    unsafe fn interpolationPhase(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interpolationPhase)
    }
    unsafe fn submissionMode(&self) -> VTFrameRateConversionParametersSubmissionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, submissionMode)
    }
    unsafe fn destinationFrames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationFrames)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameRateConversionParameters").unwrap(), new)
    }
}
pub type VTOpticalFlowConfigurationQualityPrioritization = NSInteger;
pub type VTOpticalFlowConfigurationRevision = NSInteger;
pub type VTOpticalFlowParametersSubmissionMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTOpticalFlowConfiguration(pub id);
impl std::ops::Deref for VTOpticalFlowConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTOpticalFlowConfiguration {}
impl VTOpticalFlowConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTOpticalFlowConfiguration").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorConfiguration for VTOpticalFlowConfiguration {}
impl INSObject for VTOpticalFlowConfiguration {}
impl PNSObject for VTOpticalFlowConfiguration {}
impl std::convert::TryFrom<NSObject> for VTOpticalFlowConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTOpticalFlowConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTOpticalFlowConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VTOpticalFlowConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTOpticalFlowConfiguration")
        }
    }
}
impl IVTOpticalFlowConfiguration for VTOpticalFlowConfiguration {}
pub trait IVTOpticalFlowConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithFrameWidth_frameHeight_qualityPrioritization_revision_(
        &self,
        frameWidth: NSInteger,
        frameHeight: NSInteger,
        qualityPrioritization: VTOpticalFlowConfigurationQualityPrioritization,
        revision: VTOpticalFlowConfigurationRevision,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameWidth : frameWidth, frameHeight : frameHeight, qualityPrioritization : qualityPrioritization, revision : revision)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn frameWidth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameWidth)
    }
    unsafe fn frameHeight(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameHeight)
    }
    unsafe fn qualityPrioritization(&self) -> VTOpticalFlowConfigurationQualityPrioritization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, qualityPrioritization)
    }
    unsafe fn revision(&self) -> VTOpticalFlowConfigurationRevision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revision)
    }
    unsafe fn frameSupportedPixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameSupportedPixelFormats)
    }
    unsafe fn sourcePixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePixelBufferAttributes)
    }
    unsafe fn destinationPixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPixelBufferAttributes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTOpticalFlowConfiguration").unwrap(), new)
    }
    unsafe fn supportedRevisions() -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTOpticalFlowConfiguration").unwrap(), supportedRevisions)
    }
    unsafe fn defaultRevision() -> VTOpticalFlowConfigurationRevision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTOpticalFlowConfiguration").unwrap(), defaultRevision)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTOpticalFlowConfiguration").unwrap(), isSupported)
    }
    unsafe fn processorSupported() -> Boolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTOpticalFlowConfiguration").unwrap(), processorSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTOpticalFlowParameters(pub id);
impl std::ops::Deref for VTOpticalFlowParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTOpticalFlowParameters {}
impl VTOpticalFlowParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTOpticalFlowParameters").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorParameters for VTOpticalFlowParameters {}
impl INSObject for VTOpticalFlowParameters {}
impl PNSObject for VTOpticalFlowParameters {}
impl std::convert::TryFrom<NSObject> for VTOpticalFlowParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTOpticalFlowParameters, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTOpticalFlowParameters").unwrap()) };
        if is_kind_of {
            Ok(VTOpticalFlowParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTOpticalFlowParameters")
        }
    }
}
impl IVTOpticalFlowParameters for VTOpticalFlowParameters {}
pub trait IVTOpticalFlowParameters: Sized + std::ops::Deref {
    unsafe fn initWithSourceFrame_nextFrame_submissionMode_destinationOpticalFlow_(
        &self,
        sourceFrame: VTFrameProcessorFrame,
        nextFrame: VTFrameProcessorFrame,
        submissionMode: VTOpticalFlowParametersSubmissionMode,
        destinationOpticalFlow: VTFrameProcessorOpticalFlow,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSourceFrame : sourceFrame, nextFrame : nextFrame, submissionMode : submissionMode, destinationOpticalFlow : destinationOpticalFlow)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sourceFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn nextFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextFrame)
    }
    unsafe fn submissionMode(&self) -> VTOpticalFlowParametersSubmissionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, submissionMode)
    }
    unsafe fn destinationOpticalFlow(&self) -> VTFrameProcessorOpticalFlow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationOpticalFlow)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTOpticalFlowParameters").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTTemporalNoiseFilterConfiguration(pub id);
impl std::ops::Deref for VTTemporalNoiseFilterConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTTemporalNoiseFilterConfiguration {}
impl VTTemporalNoiseFilterConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterConfiguration").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorConfiguration for VTTemporalNoiseFilterConfiguration {}
impl INSObject for VTTemporalNoiseFilterConfiguration {}
impl PNSObject for VTTemporalNoiseFilterConfiguration {}
impl std::convert::TryFrom<NSObject> for VTTemporalNoiseFilterConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTTemporalNoiseFilterConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VTTemporalNoiseFilterConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTTemporalNoiseFilterConfiguration")
        }
    }
}
impl IVTTemporalNoiseFilterConfiguration for VTTemporalNoiseFilterConfiguration {}
pub trait IVTTemporalNoiseFilterConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithFrameWidth_frameHeight_sourcePixelFormat_(
        &self,
        frameWidth: NSInteger,
        frameHeight: NSInteger,
        sourcePixelFormat: OSType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameWidth : frameWidth, frameHeight : frameHeight, sourcePixelFormat : sourcePixelFormat)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn frameWidth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameWidth)
    }
    unsafe fn frameHeight(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameHeight)
    }
    unsafe fn frameSupportedPixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameSupportedPixelFormats)
    }
    unsafe fn sourcePixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePixelBufferAttributes)
    }
    unsafe fn destinationPixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPixelBufferAttributes)
    }
    unsafe fn nextFrameCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextFrameCount)
    }
    unsafe fn previousFrameCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousFrameCount)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterConfiguration").unwrap(), new)
    }
    unsafe fn supportedSourcePixelFormats() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterConfiguration").unwrap(), supportedSourcePixelFormats)
    }
    unsafe fn maximumDimensions() -> CMVideoDimensions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterConfiguration").unwrap(), maximumDimensions)
    }
    unsafe fn minimumDimensions() -> CMVideoDimensions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterConfiguration").unwrap(), minimumDimensions)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterConfiguration").unwrap(), isSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTTemporalNoiseFilterParameters(pub id);
impl std::ops::Deref for VTTemporalNoiseFilterParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTTemporalNoiseFilterParameters {}
impl VTTemporalNoiseFilterParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterParameters").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorParameters for VTTemporalNoiseFilterParameters {}
impl INSObject for VTTemporalNoiseFilterParameters {}
impl PNSObject for VTTemporalNoiseFilterParameters {}
impl std::convert::TryFrom<NSObject> for VTTemporalNoiseFilterParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTTemporalNoiseFilterParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterParameters").unwrap())
        };
        if is_kind_of {
            Ok(VTTemporalNoiseFilterParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTTemporalNoiseFilterParameters")
        }
    }
}
impl IVTTemporalNoiseFilterParameters for VTTemporalNoiseFilterParameters {}
pub trait IVTTemporalNoiseFilterParameters: Sized + std::ops::Deref {
    unsafe fn initWithSourceFrame_nextFrames_previousFrames_destinationFrame_filterStrength_hasDiscontinuity_(
        &self,
        sourceFrame: VTFrameProcessorFrame,
        nextFrames: NSArray,
        previousFrames: NSArray,
        destinationFrame: VTFrameProcessorFrame,
        filterStrength: f32,
        hasDiscontinuity: Boolean,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSourceFrame : sourceFrame, nextFrames : nextFrames, previousFrames : previousFrames, destinationFrame : destinationFrame, filterStrength : filterStrength, hasDiscontinuity : hasDiscontinuity)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sourceFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn nextFrames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextFrames)
    }
    unsafe fn previousFrames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousFrames)
    }
    unsafe fn filterStrength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterStrength)
    }
    unsafe fn setFilterStrength_(&self, filterStrength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterStrength : filterStrength)
    }
    unsafe fn hasDiscontinuity(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasDiscontinuity)
    }
    unsafe fn setHasDiscontinuity_(&self, hasDiscontinuity: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasDiscontinuity : hasDiscontinuity)
    }
    unsafe fn destinationFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationFrame)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTTemporalNoiseFilterParameters").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTFrameProcessor(pub id);
impl std::ops::Deref for VTFrameProcessor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTFrameProcessor {}
impl VTFrameProcessor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTFrameProcessor").unwrap(), alloc) })
    }
}
impl INSObject for VTFrameProcessor {}
impl PNSObject for VTFrameProcessor {}
impl std::convert::TryFrom<NSObject> for VTFrameProcessor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTFrameProcessor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTFrameProcessor").unwrap()) };
        if is_kind_of {
            Ok(VTFrameProcessor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTFrameProcessor")
        }
    }
}
impl IVTFrameProcessor for VTFrameProcessor {}
pub trait IVTFrameProcessor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn startSessionWithConfiguration_error_(
        &self,
        configuration: *mut u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startSessionWithConfiguration : configuration, error : error)
    }
    unsafe fn processWithParameters_error_(&self, parameters: *mut u64, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processWithParameters : parameters, error : error)
    }
    unsafe fn processWithParameters_completionHandler_(
        &self,
        parameters: *mut u64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processWithParameters : parameters, completionHandler : completionHandler)
    }
    unsafe fn processWithParameters_frameOutputHandler_(
        &self,
        parameters: *mut u64,
        frameOutputHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processWithParameters : parameters, frameOutputHandler : frameOutputHandler)
    }
    unsafe fn processWithCommandBuffer_parameters_(
        &self,
        commandBuffer: *mut u64,
        parameters: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processWithCommandBuffer : commandBuffer, parameters : parameters)
    }
    unsafe fn endSession(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endSession)
    }
}
pub type VTSuperResolutionScalerConfigurationQualityPrioritization = NSInteger;
pub type VTSuperResolutionScalerConfigurationRevision = NSInteger;
pub type VTSuperResolutionScalerConfigurationInputType = NSInteger;
pub type VTSuperResolutionScalerConfigurationModelStatus = NSInteger;
pub type VTSuperResolutionScalerParametersSubmissionMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTSuperResolutionScalerConfiguration(pub id);
impl std::ops::Deref for VTSuperResolutionScalerConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTSuperResolutionScalerConfiguration {}
impl VTSuperResolutionScalerConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerConfiguration").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorConfiguration for VTSuperResolutionScalerConfiguration {}
impl INSObject for VTSuperResolutionScalerConfiguration {}
impl PNSObject for VTSuperResolutionScalerConfiguration {}
impl std::convert::TryFrom<NSObject> for VTSuperResolutionScalerConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTSuperResolutionScalerConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VTSuperResolutionScalerConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTSuperResolutionScalerConfiguration")
        }
    }
}
impl IVTSuperResolutionScalerConfiguration for VTSuperResolutionScalerConfiguration {}
pub trait IVTSuperResolutionScalerConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithFrameWidth_frameHeight_scaleFactor_inputType_usePrecomputedFlow_qualityPrioritization_revision_(
        &self,
        frameWidth: NSInteger,
        frameHeight: NSInteger,
        scaleFactor: NSInteger,
        inputType: VTSuperResolutionScalerConfigurationInputType,
        usePrecomputedFlow: BOOL,
        qualityPrioritization: VTSuperResolutionScalerConfigurationQualityPrioritization,
        revision: VTSuperResolutionScalerConfigurationRevision,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameWidth : frameWidth, frameHeight : frameHeight, scaleFactor : scaleFactor, inputType : inputType, usePrecomputedFlow : usePrecomputedFlow, qualityPrioritization : qualityPrioritization, revision : revision)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn downloadConfigurationModelWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadConfigurationModelWithCompletionHandler : completionHandler)
    }
    unsafe fn frameWidth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameWidth)
    }
    unsafe fn frameHeight(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameHeight)
    }
    unsafe fn inputType(&self) -> VTSuperResolutionScalerConfigurationInputType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputType)
    }
    unsafe fn usesPrecomputedFlow(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesPrecomputedFlow)
    }
    unsafe fn scaleFactor(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn qualityPrioritization(
        &self,
    ) -> VTSuperResolutionScalerConfigurationQualityPrioritization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, qualityPrioritization)
    }
    unsafe fn revision(&self) -> VTSuperResolutionScalerConfigurationRevision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revision)
    }
    unsafe fn frameSupportedPixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameSupportedPixelFormats)
    }
    unsafe fn sourcePixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePixelBufferAttributes)
    }
    unsafe fn destinationPixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPixelBufferAttributes)
    }
    unsafe fn configurationModelStatus(&self) -> VTSuperResolutionScalerConfigurationModelStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationModelStatus)
    }
    unsafe fn configurationModelPercentageAvailable(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationModelPercentageAvailable)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerConfiguration").unwrap(), new)
    }
    unsafe fn supportedRevisions() -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerConfiguration").unwrap(), supportedRevisions)
    }
    unsafe fn defaultRevision() -> VTSuperResolutionScalerConfigurationRevision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerConfiguration").unwrap(), defaultRevision)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerConfiguration").unwrap(), isSupported)
    }
    unsafe fn supportedScaleFactors() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerConfiguration").unwrap(), supportedScaleFactors)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTSuperResolutionScalerParameters(pub id);
impl std::ops::Deref for VTSuperResolutionScalerParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTSuperResolutionScalerParameters {}
impl VTSuperResolutionScalerParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerParameters").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorParameters for VTSuperResolutionScalerParameters {}
impl INSObject for VTSuperResolutionScalerParameters {}
impl PNSObject for VTSuperResolutionScalerParameters {}
impl std::convert::TryFrom<NSObject> for VTSuperResolutionScalerParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTSuperResolutionScalerParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerParameters").unwrap())
        };
        if is_kind_of {
            Ok(VTSuperResolutionScalerParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTSuperResolutionScalerParameters")
        }
    }
}
impl IVTSuperResolutionScalerParameters for VTSuperResolutionScalerParameters {}
pub trait IVTSuperResolutionScalerParameters: Sized + std::ops::Deref {
    unsafe fn initWithSourceFrame_previousFrame_previousOutputFrame_opticalFlow_submissionMode_destinationFrame_(
        &self,
        sourceFrame: VTFrameProcessorFrame,
        previousFrame: VTFrameProcessorFrame,
        previousOutputFrame: VTFrameProcessorFrame,
        opticalFlow: VTFrameProcessorOpticalFlow,
        submissionMode: VTSuperResolutionScalerParametersSubmissionMode,
        destinationFrame: VTFrameProcessorFrame,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSourceFrame : sourceFrame, previousFrame : previousFrame, previousOutputFrame : previousOutputFrame, opticalFlow : opticalFlow, submissionMode : submissionMode, destinationFrame : destinationFrame)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sourceFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn previousFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousFrame)
    }
    unsafe fn previousOutputFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousOutputFrame)
    }
    unsafe fn opticalFlow(&self) -> VTFrameProcessorOpticalFlow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opticalFlow)
    }
    unsafe fn submissionMode(&self) -> VTSuperResolutionScalerParametersSubmissionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, submissionMode)
    }
    unsafe fn destinationFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationFrame)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTSuperResolutionScalerParameters").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTLowLatencySuperResolutionScalerConfiguration(pub id);
impl std::ops::Deref for VTLowLatencySuperResolutionScalerConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTLowLatencySuperResolutionScalerConfiguration {}
impl VTLowLatencySuperResolutionScalerConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerConfiguration").unwrap(), alloc)
        })
    }
}
impl PVTFrameProcessorConfiguration for VTLowLatencySuperResolutionScalerConfiguration {}
impl INSObject for VTLowLatencySuperResolutionScalerConfiguration {}
impl PNSObject for VTLowLatencySuperResolutionScalerConfiguration {}
impl std::convert::TryFrom<NSObject> for VTLowLatencySuperResolutionScalerConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<VTLowLatencySuperResolutionScalerConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VTLowLatencySuperResolutionScalerConfiguration(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to VTLowLatencySuperResolutionScalerConfiguration" ,)
        }
    }
}
impl IVTLowLatencySuperResolutionScalerConfiguration
    for VTLowLatencySuperResolutionScalerConfiguration
{
}
pub trait IVTLowLatencySuperResolutionScalerConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithFrameWidth_frameHeight_scaleFactor_(
        &self,
        frameWidth: NSInteger,
        frameHeight: NSInteger,
        scaleFactor: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameWidth : frameWidth, frameHeight : frameHeight, scaleFactor : scaleFactor)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn frameWidth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameWidth)
    }
    unsafe fn frameHeight(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameHeight)
    }
    unsafe fn frameSupportedPixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameSupportedPixelFormats)
    }
    unsafe fn sourcePixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePixelBufferAttributes)
    }
    unsafe fn destinationPixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPixelBufferAttributes)
    }
    unsafe fn scaleFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerConfiguration").unwrap(), new)
    }
    unsafe fn supportedScaleFactorsForFrameWidth_frameHeight_(
        frameWidth: NSInteger,
        frameHeight: NSInteger,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerConfiguration").unwrap(), supportedScaleFactorsForFrameWidth : frameWidth, frameHeight : frameHeight)
    }
    unsafe fn maximumDimensions() -> CMVideoDimensions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerConfiguration").unwrap(), maximumDimensions)
    }
    unsafe fn minimumDimensions() -> CMVideoDimensions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerConfiguration").unwrap(), minimumDimensions)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerConfiguration").unwrap(), isSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTLowLatencySuperResolutionScalerParameters(pub id);
impl std::ops::Deref for VTLowLatencySuperResolutionScalerParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTLowLatencySuperResolutionScalerParameters {}
impl VTLowLatencySuperResolutionScalerParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerParameters").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorParameters for VTLowLatencySuperResolutionScalerParameters {}
impl INSObject for VTLowLatencySuperResolutionScalerParameters {}
impl PNSObject for VTLowLatencySuperResolutionScalerParameters {}
impl std::convert::TryFrom<NSObject> for VTLowLatencySuperResolutionScalerParameters {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<VTLowLatencySuperResolutionScalerParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerParameters").unwrap())
        };
        if is_kind_of {
            Ok(VTLowLatencySuperResolutionScalerParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTLowLatencySuperResolutionScalerParameters")
        }
    }
}
impl IVTLowLatencySuperResolutionScalerParameters for VTLowLatencySuperResolutionScalerParameters {}
pub trait IVTLowLatencySuperResolutionScalerParameters: Sized + std::ops::Deref {
    unsafe fn initWithSourceFrame_destinationFrame_(
        &self,
        sourceFrame: VTFrameProcessorFrame,
        destinationFrame: VTFrameProcessorFrame,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSourceFrame : sourceFrame, destinationFrame : destinationFrame)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sourceFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn destinationFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationFrame)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencySuperResolutionScalerParameters").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTLowLatencyFrameInterpolationConfiguration(pub id);
impl std::ops::Deref for VTLowLatencyFrameInterpolationConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTLowLatencyFrameInterpolationConfiguration {}
impl VTLowLatencyFrameInterpolationConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencyFrameInterpolationConfiguration").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorConfiguration for VTLowLatencyFrameInterpolationConfiguration {}
impl INSObject for VTLowLatencyFrameInterpolationConfiguration {}
impl PNSObject for VTLowLatencyFrameInterpolationConfiguration {}
impl std::convert::TryFrom<NSObject> for VTLowLatencyFrameInterpolationConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<VTLowLatencyFrameInterpolationConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTLowLatencyFrameInterpolationConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VTLowLatencyFrameInterpolationConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTLowLatencyFrameInterpolationConfiguration")
        }
    }
}
impl IVTLowLatencyFrameInterpolationConfiguration for VTLowLatencyFrameInterpolationConfiguration {}
pub trait IVTLowLatencyFrameInterpolationConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithFrameWidth_frameHeight_numberOfInterpolatedFrames_(
        &self,
        frameWidth: NSInteger,
        frameHeight: NSInteger,
        numberOfInterpolatedFrames: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameWidth : frameWidth, frameHeight : frameHeight, numberOfInterpolatedFrames : numberOfInterpolatedFrames)
    }
    unsafe fn initWithFrameWidth_frameHeight_spatialScaleFactor_(
        &self,
        frameWidth: NSInteger,
        frameHeight: NSInteger,
        spatialScaleFactor: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameWidth : frameWidth, frameHeight : frameHeight, spatialScaleFactor : spatialScaleFactor)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn frameWidth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameWidth)
    }
    unsafe fn frameHeight(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameHeight)
    }
    unsafe fn spatialScaleFactor(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spatialScaleFactor)
    }
    unsafe fn numberOfInterpolatedFrames(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfInterpolatedFrames)
    }
    unsafe fn frameSupportedPixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameSupportedPixelFormats)
    }
    unsafe fn sourcePixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePixelBufferAttributes)
    }
    unsafe fn destinationPixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPixelBufferAttributes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencyFrameInterpolationConfiguration").unwrap(), new)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencyFrameInterpolationConfiguration").unwrap(), isSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VTLowLatencyFrameInterpolationParameters(pub id);
impl std::ops::Deref for VTLowLatencyFrameInterpolationParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VTLowLatencyFrameInterpolationParameters {}
impl VTLowLatencyFrameInterpolationParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencyFrameInterpolationParameters").unwrap(), alloc) })
    }
}
impl PVTFrameProcessorParameters for VTLowLatencyFrameInterpolationParameters {}
impl INSObject for VTLowLatencyFrameInterpolationParameters {}
impl PNSObject for VTLowLatencyFrameInterpolationParameters {}
impl std::convert::TryFrom<NSObject> for VTLowLatencyFrameInterpolationParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VTLowLatencyFrameInterpolationParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VTLowLatencyFrameInterpolationParameters").unwrap())
        };
        if is_kind_of {
            Ok(VTLowLatencyFrameInterpolationParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VTLowLatencyFrameInterpolationParameters")
        }
    }
}
impl IVTLowLatencyFrameInterpolationParameters for VTLowLatencyFrameInterpolationParameters {}
pub trait IVTLowLatencyFrameInterpolationParameters: Sized + std::ops::Deref {
    unsafe fn initWithSourceFrame_previousFrame_interpolationPhase_destinationFrames_(
        &self,
        sourceFrame: VTFrameProcessorFrame,
        previousFrame: VTFrameProcessorFrame,
        interpolationPhase: NSArray,
        destinationFrames: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSourceFrame : sourceFrame, previousFrame : previousFrame, interpolationPhase : interpolationPhase, destinationFrames : destinationFrames)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sourceFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn previousFrame(&self) -> VTFrameProcessorFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousFrame)
    }
    unsafe fn interpolationPhase(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interpolationPhase)
    }
    unsafe fn destinationFrames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationFrames)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VTLowLatencyFrameInterpolationParameters").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_NumberOfPendingFrames: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_PixelBufferPoolIsShared: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MaxKeyFrameInterval: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_AllowTemporalCompression: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_AllowFrameReordering: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_AllowOpenGOP: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_AverageBitRate: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_DataRateLimits: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_Quality: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_TargetQualityForAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MoreFramesBeforeStart: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MoreFramesAfterEnd: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ConstantBitRate: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_EstimatedAverageBytesPerFrame: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_VariableBitRate: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_VBVMaxBitRate: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_VBVBufferDuration: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_VBVInitialDelayPercentage: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ProfileLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_HEVC_Main_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_HEVC_Main10_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_HEVC_Main42210_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_HEVC_Monochrome_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_HEVC_Monochrome10_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_1_3: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_3_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_3_1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_3_2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_4_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_4_1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_4_2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_5_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_5_1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_5_2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Baseline_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_ConstrainedBaseline_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_3_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_3_1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_3_2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_4_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_4_1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_4_2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_5_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_5_1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_5_2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Main_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Extended_5_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_Extended_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_3_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_3_1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_3_2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_4_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_4_1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_4_2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_5_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_5_1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_5_2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_High_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H264_ConstrainedHigh_AutoLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_Simple_L0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_Simple_L1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_Simple_L2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_Simple_L3: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_Main_L2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_Main_L3: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_Main_L4: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L1: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L2: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L3: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_MP4V_AdvancedSimple_L4: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H263_Profile0_Level10: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H263_Profile0_Level45: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProfileLevel_H263_Profile3_Level45: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_OutputBitDepth: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_HDRMetadataInsertionMode: CFStringRef;
}
unsafe extern "C" {
    pub static kVTHDRMetadataInsertionMode_None: CFStringRef;
}
unsafe extern "C" {
    pub static kVTHDRMetadataInsertionMode_Auto: CFStringRef;
}
unsafe extern "C" {
    pub static kVTHDRMetadataInsertionMode_RequestSDRRangePreservation: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_H264EntropyMode: CFStringRef;
}
unsafe extern "C" {
    pub static kVTH264EntropyMode_CAVLC: CFStringRef;
}
unsafe extern "C" {
    pub static kVTH264EntropyMode_CABAC: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_Depth: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_PreserveAlphaChannel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MaxFrameDelayCount: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MaxH264SliceBytes: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_RealTime: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MaximizePowerEfficiency: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_SourceFrameCount: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ExpectedFrameRate: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MaximumRealTimeFrameRate: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_BaseLayerFrameRateFraction: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_BaseLayerBitRateFraction: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ExpectedDuration: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_BaseLayerFrameRate: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ReferenceBufferCount: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_CalculateMeanSquaredError: CFStringRef;
}
unsafe extern "C" {
    pub static kVTSampleAttachmentKey_QualityMetrics: CFStringRef;
}
unsafe extern "C" {
    pub static kVTSampleAttachmentQualityMetricsKey_LumaMeanSquaredError: CFStringRef;
}
unsafe extern "C" {
    pub static kVTSampleAttachmentQualityMetricsKey_ChromaBlueMeanSquaredError: CFStringRef;
}
unsafe extern "C" {
    pub static kVTSampleAttachmentQualityMetricsKey_ChromaRedMeanSquaredError: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderSpecification_EnableHardwareAcceleratedVideoEncoder: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_UsingHardwareAcceleratedVideoEncoder: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderSpecification_RequiredEncoderGPURegistryID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderSpecification_PreferredEncoderGPURegistryID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_UsingGPURegistryID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_SupportsBaseFrameQP: CFStringRef;
}
unsafe extern "C" {
    pub static kVTEncodeFrameOptionKey_ForceKeyFrame: CFStringRef;
}
unsafe extern "C" {
    pub static kVTEncodeFrameOptionKey_BaseFrameQP: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_CleanAperture: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_PixelAspectRatio: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_FieldCount: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_FieldDetail: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_AspectRatio16x9: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ProgressiveScan: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ColorPrimaries: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_TransferFunction: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_YCbCrMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ICCProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MasteringDisplayColorVolume: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ContentLightLevelInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_GammaLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_AlphaChannelMode: CFStringRef;
}
unsafe extern "C" {
    pub static kVTAlphaChannelMode_StraightAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static kVTAlphaChannelMode_PremultipliedAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_PixelTransferProperties: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MultiPassStorage: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_EncoderID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_RecommendedParallelizationLimit: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumFrameCount:
        CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_RecommendedParallelizedSubdivisionMinimumDuration:
        CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_PreserveDynamicHDRMetadata: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderSpecification_EnableLowLatencyRateControl: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MaxAllowedFrameQP: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MinAllowedFrameQP: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_EnableLTR: CFStringRef;
}
unsafe extern "C" {
    pub static kVTEncodeFrameOptionKey_AcknowledgedLTRTokens: CFStringRef;
}
unsafe extern "C" {
    pub static kVTEncodeFrameOptionKey_ForceLTRRefresh: CFStringRef;
}
unsafe extern "C" {
    pub static kVTSampleAttachmentKey_RequireLTRAcknowledgementToken: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MVHEVCVideoLayerIDs: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MVHEVCViewIDs: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_MVHEVCLeftAndRightViewIDs: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_HeroEye: CFStringRef;
}
unsafe extern "C" {
    pub static kVTHeroEye_Left: CFStringRef;
}
unsafe extern "C" {
    pub static kVTHeroEye_Right: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_StereoCameraBaseline: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_HorizontalDisparityAdjustment: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_HasLeftStereoEyeView: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_HasRightStereoEyeView: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_HorizontalFieldOfView: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ProjectionKind: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProjectionKind_Rectilinear: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProjectionKind_Equirectangular: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProjectionKind_HalfEquirectangular: CFStringRef;
}
unsafe extern "C" {
    pub static kVTProjectionKind_ParametricImmersive: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_ViewPackingKind: CFStringRef;
}
unsafe extern "C" {
    pub static kVTViewPackingKind_SideBySide: CFStringRef;
}
unsafe extern "C" {
    pub static kVTViewPackingKind_OverUnder: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_CameraCalibrationDataLensCollection: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_LensAlgorithmKind: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCameraCalibrationLensAlgorithmKind_ParametricLens: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_LensDomain: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCameraCalibrationLensDomain_Color: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_LensIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_LensRole: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCameraCalibrationLensRole_Mono: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCameraCalibrationLensRole_Left: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCameraCalibrationLensRole_Right: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_LensDistortions: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_RadialAngleLimit: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialX:
        CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_LensFrameAdjustmentsPolynomialY:
        CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixProjectionOffset:
        CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_IntrinsicMatrixReferenceDimensions:
        CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOriginSource: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCameraCalibrationExtrinsicOriginSource_StereoCameraSystemBaseline: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyCameraCalibrationKey_ExtrinsicOrientationQuaternion:
        CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_SuggestedLookAheadFrameCount: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_SpatialAdaptiveQPLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPropertyKey_SupportedPresetDictionaries: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPreset_HighQuality: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPreset_Balanced: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPreset_HighSpeed: CFStringRef;
}
unsafe extern "C" {
    pub static kVTCompressionPreset_VideoConferencing: CFStringRef;
}
unsafe extern "C" {
    pub fn VTSessionCopySupportedPropertyDictionary(
        session: VTSessionRef,
        supportedPropertyDictionaryOut: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kVTPropertyTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertyType_Enumeration: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertyType_Boolean: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertyType_Number: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertyReadWriteStatusKey: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertyReadWriteStatus_ReadOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertyReadWriteStatus_ReadWrite: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertyShouldBeSerializedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertySupportedValueMinimumKey: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertySupportedValueMaximumKey: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertySupportedValueListKey: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPropertyDocumentationKey: CFStringRef;
}
unsafe extern "C" {
    pub fn VTSessionSetProperty(
        session: VTSessionRef,
        propertyKey: CFStringRef,
        propertyValue: CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTSessionCopyProperty(
        session: VTSessionRef,
        propertyKey: CFStringRef,
        allocator: CFAllocatorRef,
        propertyValueOut: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTSessionSetProperties(
        session: VTSessionRef,
        propertyDictionary: CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTSessionCopySerializableProperties(
        session: VTSessionRef,
        allocator: CFAllocatorRef,
        dictionaryOut: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kVTVideoEncoderSpecification_EncoderID: CFStringRef;
}
unsafe extern "C" {
    pub fn VTCompressionSessionCreate(
        allocator: CFAllocatorRef,
        width: i32,
        height: i32,
        codecType: CMVideoCodecType,
        encoderSpecification: CFDictionaryRef,
        sourceImageBufferAttributes: CFDictionaryRef,
        compressedDataAllocator: CFAllocatorRef,
        outputCallback: VTCompressionOutputCallback,
        outputCallbackRefCon: *mut ::std::os::raw::c_void,
        compressionSessionOut: *mut VTCompressionSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCompressionSessionInvalidate(session: VTCompressionSessionRef);
}
unsafe extern "C" {
    pub fn VTCompressionSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn VTCompressionSessionGetPixelBufferPool(
        session: VTCompressionSessionRef,
    ) -> CVPixelBufferPoolRef;
}
unsafe extern "C" {
    pub fn VTCompressionSessionPrepareToEncodeFrames(session: VTCompressionSessionRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCompressionSessionEncodeFrame(
        session: VTCompressionSessionRef,
        imageBuffer: CVImageBufferRef,
        presentationTimeStamp: CMTime,
        duration: CMTime,
        frameProperties: CFDictionaryRef,
        sourceFrameRefcon: *mut ::std::os::raw::c_void,
        infoFlagsOut: *mut VTEncodeInfoFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCompressionSessionEncodeFrameWithOutputHandler(
        session: VTCompressionSessionRef,
        imageBuffer: CVImageBufferRef,
        presentationTimeStamp: CMTime,
        duration: CMTime,
        frameProperties: CFDictionaryRef,
        infoFlagsOut: *mut VTEncodeInfoFlags,
        outputHandler: VTCompressionOutputHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCompressionSessionCompleteFrames(
        session: VTCompressionSessionRef,
        completeUntilPresentationTimeStamp: CMTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTIsStereoMVHEVCEncodeSupported() -> Boolean;
}
unsafe extern "C" {
    pub fn VTCompressionSessionEncodeMultiImageFrame(
        session: VTCompressionSessionRef,
        taggedBufferGroup: CMTaggedBufferGroupRef,
        presentationTimeStamp: CMTime,
        duration: CMTime,
        frameProperties: CFDictionaryRef,
        sourceFrameRefcon: *mut ::std::os::raw::c_void,
        infoFlagsOut: *mut VTEncodeInfoFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCompressionSessionEncodeMultiImageFrameWithOutputHandler(
        session: VTCompressionSessionRef,
        taggedBufferGroup: CMTaggedBufferGroupRef,
        presentationTimeStamp: CMTime,
        duration: CMTime,
        frameProperties: CFDictionaryRef,
        infoFlagsOut: *mut VTEncodeInfoFlags,
        outputHandler: VTCompressionOutputHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCompressionSessionBeginPass(
        session: VTCompressionSessionRef,
        beginPassFlags: VTCompressionSessionOptionFlags,
        reserved: *mut u32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCompressionSessionEndPass(
        session: VTCompressionSessionRef,
        furtherPassesRequestedOut: *mut Boolean,
        reserved: *mut u32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCompressionSessionGetTimeRangesForNextPass(
        session: VTCompressionSessionRef,
        timeRangeCountOut: *mut CMItemCount,
        timeRangeArrayOut: *mut *const CMTimeRange,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_PixelBufferPool: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_PixelBufferPoolIsShared: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded:
        CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded:
        CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_ContentHasInterframeDependencies: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_UsingHardwareAcceleratedVideoDecoder: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_RealTime: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_MaximizePowerEfficiency: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_ThreadCount: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_FieldMode: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_FieldMode_BothFields: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_FieldMode_TopFieldOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_FieldMode_BottomFieldOnly: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_FieldMode_SingleField: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_FieldMode_DeinterlaceFields: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_DeinterlaceMode: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_DeinterlaceMode_VerticalFilter: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_DeinterlaceMode_Temporal: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_ReducedResolutionDecode: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionResolutionKey_Width: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionResolutionKey_Height: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_ReducedCoefficientDecode: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_ReducedFrameDelivery: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_OnlyTheseFrames: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_OnlyTheseFrames_AllFrames: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_OnlyTheseFrames_NonDroppableFrames: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_OnlyTheseFrames_IFrames: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_OnlyTheseFrames_KeyFrames: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionProperty_TemporalLevelLimit: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_SuggestedQualityOfServiceTiers: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByQuality: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_AllowBitstreamToChangeFrameDimensions: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_PixelTransferProperties: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_UsingGPURegistryID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_GeneratePerFrameHDRDisplayMetadata: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_DecoderProducesRAWOutput: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_RequestRAWOutput: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecompressionPropertyKey_RequestedMVHEVCVideoLayerIDs: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecodeFrameOptionKey_ContentAnalyzerRotation: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDecodeFrameOptionKey_ContentAnalyzerCropRectangle: CFStringRef;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionCreate(
        allocator: CFAllocatorRef,
        videoFormatDescription: CMVideoFormatDescriptionRef,
        videoDecoderSpecification: CFDictionaryRef,
        destinationImageBufferAttributes: CFDictionaryRef,
        outputCallback: *const VTDecompressionOutputCallbackRecord,
        decompressionSessionOut: *mut VTDecompressionSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionInvalidate(session: VTDecompressionSessionRef);
}
unsafe extern "C" {
    pub fn VTDecompressionSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionDecodeFrame(
        session: VTDecompressionSessionRef,
        sampleBuffer: CMSampleBufferRef,
        decodeFlags: VTDecodeFrameFlags,
        sourceFrameRefCon: *mut ::std::os::raw::c_void,
        infoFlagsOut: *mut VTDecodeInfoFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionDecodeFrameWithOutputHandler(
        session: VTDecompressionSessionRef,
        sampleBuffer: CMSampleBufferRef,
        decodeFlags: VTDecodeFrameFlags,
        infoFlagsOut: *mut VTDecodeInfoFlags,
        outputHandler: VTDecompressionOutputHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionFinishDelayedFrames(
        session: VTDecompressionSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionCanAcceptFormatDescription(
        session: VTDecompressionSessionRef,
        newFormatDesc: CMFormatDescriptionRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionWaitForAsynchronousFrames(
        session: VTDecompressionSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionCopyBlackPixelBuffer(
        session: VTDecompressionSessionRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTIsHardwareDecodeSupported(codecType: CMVideoCodecType) -> Boolean;
}
unsafe extern "C" {
    pub fn VTIsStereoMVHEVCDecodeSupported() -> Boolean;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionSetMultiImageCallback(
        decompressionSession: VTDecompressionSessionRef,
        outputMultiImageCallback: VTDecompressionOutputMultiImageCallback,
        outputMultiImageRefcon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler(
        session: VTDecompressionSessionRef,
        sampleBuffer: CMSampleBufferRef,
        decodeFlags: VTDecodeFrameFlags,
        infoFlagsOut: *mut VTDecodeInfoFlags,
        multiImageCapableOutputHandler: VTDecompressionMultiImageCapableOutputHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionDecodeFrameWithOptions(
        session: VTDecompressionSessionRef,
        sampleBuffer: CMSampleBufferRef,
        decodeFlags: VTDecodeFrameFlags,
        frameOptions: CFDictionaryRef,
        sourceFrameRefCon: *mut ::std::os::raw::c_void,
        infoFlagsOut: *mut VTDecodeInfoFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTDecompressionSessionDecodeFrameWithOptionsAndOutputHandler(
        session: VTDecompressionSessionRef,
        sampleBuffer: CMSampleBufferRef,
        decodeFlags: VTDecodeFrameFlags,
        frameOptions: CFDictionaryRef,
        infoFlagsOut: *mut VTDecodeInfoFlags,
        outputHandler: VTDecompressionOutputHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTFrameSiloGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn VTFrameSiloCreate(
        allocator: CFAllocatorRef,
        fileURL: CFURLRef,
        timeRange: CMTimeRange,
        options: CFDictionaryRef,
        frameSiloOut: *mut VTFrameSiloRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTFrameSiloAddSampleBuffer(
        silo: VTFrameSiloRef,
        sampleBuffer: CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTFrameSiloSetTimeRangesForNextPass(
        silo: VTFrameSiloRef,
        timeRangeCount: CMItemCount,
        timeRangeArray: *const CMTimeRange,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTFrameSiloGetProgressOfCurrentPass(
        silo: VTFrameSiloRef,
        progressOut: *mut Float32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTFrameSiloCallFunctionForEachSampleBuffer(
        silo: VTFrameSiloRef,
        timeRange: CMTimeRange,
        refcon: *mut ::std::os::raw::c_void,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                refcon: *mut ::std::os::raw::c_void,
                sampleBuffer: CMSampleBufferRef,
            ) -> OSStatus,
        >,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTFrameSiloCallBlockForEachSampleBuffer(
        silo: VTFrameSiloRef,
        timeRange: CMTimeRange,
        handler: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTMultiPassStorageGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn VTMultiPassStorageCreate(
        allocator: CFAllocatorRef,
        fileURL: CFURLRef,
        timeRange: CMTimeRange,
        options: CFDictionaryRef,
        multiPassStorageOut: *mut VTMultiPassStorageRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kVTMultiPassStorageCreationOption_DoNotDelete: CFStringRef;
}
unsafe extern "C" {
    pub fn VTMultiPassStorageClose(multiPassStorage: VTMultiPassStorageRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCopyVideoEncoderList(
        options: CFDictionaryRef,
        listOfVideoEncodersOut: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kVTVideoEncoderListOption_IncludeStandardDefinitionDVEncoders: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_CodecType: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_EncoderID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_CodecName: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_EncoderName: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_DisplayName: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_GPURegistryID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_SupportedSelectionProperties: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_PerformanceRating: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_QualityRating: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_InstanceLimit: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_IsHardwareAccelerated: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_SupportsFrameReordering: CFStringRef;
}
unsafe extern "C" {
    pub static kVTVideoEncoderList_SupportsMultiPass: CFStringRef;
}
unsafe extern "C" {
    pub fn VTCopySupportedPropertyDictionaryForEncoder(
        width: i32,
        height: i32,
        codecType: CMVideoCodecType,
        encoderSpecification: CFDictionaryRef,
        encoderIDOut: *mut CFStringRef,
        supportedPropertiesOut: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCreateCGImageFromCVPixelBuffer(
        pixelBuffer: CVPixelBufferRef,
        options: CFDictionaryRef,
        imageOut: *mut CGImageRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTRegisterSupplementalVideoDecoderIfAvailable(codecType: CMVideoCodecType);
}
unsafe extern "C" {
    pub fn VTCopyVideoDecoderExtensionProperties(
        formatDesc: CMFormatDescriptionRef,
        mediaExtensionPropertiesOut: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTCopyRAWProcessorExtensionProperties(
        formatDesc: CMFormatDescriptionRef,
        mediaExtensionPropertiesOut: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kVTExtensionProperties_ExtensionIdentifierKey: VTExtensionPropertiesKey;
}
unsafe extern "C" {
    pub static kVTExtensionProperties_ExtensionNameKey: VTExtensionPropertiesKey;
}
unsafe extern "C" {
    pub static kVTExtensionProperties_ContainingBundleNameKey: VTExtensionPropertiesKey;
}
unsafe extern "C" {
    pub static kVTExtensionProperties_ExtensionURLKey: VTExtensionPropertiesKey;
}
unsafe extern "C" {
    pub static kVTExtensionProperties_ContainingBundleURLKey: VTExtensionPropertiesKey;
}
unsafe extern "C" {
    pub static kVTExtensionProperties_CodecNameKey: VTExtensionPropertiesKey;
}
unsafe extern "C" {
    pub static kVTPixelTransferPropertyKey_ScalingMode: CFStringRef;
}
unsafe extern "C" {
    pub static kVTScalingMode_Normal: CFStringRef;
}
unsafe extern "C" {
    pub static kVTScalingMode_CropSourceToCleanAperture: CFStringRef;
}
unsafe extern "C" {
    pub static kVTScalingMode_Letterbox: CFStringRef;
}
unsafe extern "C" {
    pub static kVTScalingMode_Trim: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelTransferPropertyKey_DestinationCleanAperture: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelTransferPropertyKey_DestinationPixelAspectRatio: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelTransferPropertyKey_DownsamplingMode: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDownsamplingMode_Decimate: CFStringRef;
}
unsafe extern "C" {
    pub static kVTDownsamplingMode_Average: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelTransferPropertyKey_DestinationColorPrimaries: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelTransferPropertyKey_DestinationTransferFunction: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelTransferPropertyKey_DestinationICCProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelTransferPropertyKey_DestinationYCbCrMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelTransferPropertyKey_RealTime: CFStringRef;
}
unsafe extern "C" {
    pub fn VTPixelTransferSessionCreate(
        allocator: CFAllocatorRef,
        pixelTransferSessionOut: *mut VTPixelTransferSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTPixelTransferSessionInvalidate(session: VTPixelTransferSessionRef);
}
unsafe extern "C" {
    pub fn VTPixelTransferSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn VTPixelTransferSessionTransferImage(
        session: VTPixelTransferSessionRef,
        sourceBuffer: CVPixelBufferRef,
        destinationBuffer: CVPixelBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kVTPixelRotationPropertyKey_Rotation: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRotation_0: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRotation_CW90: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRotation_180: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRotation_CCW90: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelRotationPropertyKey_FlipHorizontalOrientation: CFStringRef;
}
unsafe extern "C" {
    pub static kVTPixelRotationPropertyKey_FlipVerticalOrientation: CFStringRef;
}
unsafe extern "C" {
    pub fn VTPixelRotationSessionCreate(
        allocator: CFAllocatorRef,
        pixelRotationSessionOut: *mut VTPixelRotationSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTPixelRotationSessionInvalidate(session: VTPixelRotationSessionRef);
}
unsafe extern "C" {
    pub fn VTPixelRotationSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn VTPixelRotationSessionRotateImage(
        session: VTPixelRotationSessionRef,
        sourceBuffer: CVPixelBufferRef,
        destinationBuffer: CVPixelBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTRAWProcessingSessionCreate(
        allocator: CFAllocatorRef,
        formatDescription: CMVideoFormatDescriptionRef,
        outputPixelBufferAttributes: CFDictionaryRef,
        processingSessionOptions: CFDictionaryRef,
        processingSessionOut: *mut VTRAWProcessingSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTRAWProcessingSessionInvalidate(session: VTRAWProcessingSessionRef);
}
unsafe extern "C" {
    pub fn VTRAWProcessingSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn VTRAWProcessingSessionSetParameterChangedHandler(
        session: VTRAWProcessingSessionRef,
        parameterChangeHandler: VTRAWProcessingParameterChangeHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTRAWProcessingSessionSetParameterChangedHander(
        session: VTRAWProcessingSessionRef,
        parameterChangeHandler: VTRAWProcessingParameterChangeHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTRAWProcessingSessionProcessFrame(
        session: VTRAWProcessingSessionRef,
        inputPixelBuffer: CVPixelBufferRef,
        frameOptions: CFDictionaryRef,
        outputHandler: VTRAWProcessingOutputHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTRAWProcessingSessionCompleteFrames(session: VTRAWProcessingSessionRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTRAWProcessingSessionCopyProcessingParameters(
        session: VTRAWProcessingSessionRef,
        outParameterArray: *mut CFArrayRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTRAWProcessingSessionSetProcessingParameters(
        session: VTRAWProcessingSessionRef,
        processingParameters: CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_Key: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_Name: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_Description: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_Enabled: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_ValueType: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameterValueType_Boolean: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameterValueType_Integer: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameterValueType_Float: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameterValueType_List: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameterValueType_SubGroup: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_ListArray: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameterListElement_Label: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameterListElement_Description: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameterListElement_ListElementID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_SubGroup: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_MaximumValue: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_MinimumValue: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_InitialValue: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_NeutralValue: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_CameraValue: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingParameter_CurrentValue: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingPropertyKey_MetalDeviceRegistryID: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingPropertyKey_OutputColorAttachments: CFStringRef;
}
unsafe extern "C" {
    pub static kVTRAWProcessingPropertyKey_MetadataForSidecarFile: CFStringRef;
}
unsafe extern "C" {
    pub fn VTRegisterProfessionalVideoWorkflowVideoDecoders();
}
unsafe extern "C" {
    pub fn VTRegisterProfessionalVideoWorkflowVideoEncoders();
}
unsafe extern "C" {
    pub static kVTHDRPerFrameMetadataGenerationHDRFormatType_DolbyVision:
        VTHDRPerFrameMetadataGenerationHDRFormatType;
}
unsafe extern "C" {
    pub static kVTHDRPerFrameMetadataGenerationOptionsKey_HDRFormats: CFStringRef;
}
unsafe extern "C" {
    pub fn VTHDRPerFrameMetadataGenerationSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn VTHDRPerFrameMetadataGenerationSessionCreate(
        allocator: CFAllocatorRef,
        framesPerSecond: f32,
        options: CFDictionaryRef,
        hdrPerFrameMetadataGenerationSessionOut: *mut VTHDRPerFrameMetadataGenerationSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTHDRPerFrameMetadataGenerationSessionAttachMetadata(
        hdrPerFrameMetadataGenerationSession: VTHDRPerFrameMetadataGenerationSessionRef,
        pixelBuffer: CVPixelBufferRef,
        sceneChange: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTMotionEstimationSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn VTMotionEstimationSessionCreate(
        allocator: CFAllocatorRef,
        motionVectorProcessorSelectionOptions: CFDictionaryRef,
        width: u32,
        height: u32,
        motionEstimationSessionOut: *mut VTMotionEstimationSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTMotionEstimationSessionCopySourcePixelBufferAttributes(
        motionEstimationSession: VTMotionEstimationSessionRef,
        attributesOut: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTMotionEstimationSessionInvalidate(session: VTMotionEstimationSessionRef);
}
unsafe extern "C" {
    pub fn VTMotionEstimationSessionEstimateMotionVectors(
        session: VTMotionEstimationSessionRef,
        referenceImage: CVPixelBufferRef,
        currentImage: CVPixelBufferRef,
        motionEstimationFrameFlags: VTMotionEstimationFrameFlags,
        additionalFrameOptions: CFDictionaryRef,
        outputHandler: VTMotionEstimationOutputHandler,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VTMotionEstimationSessionCompleteFrames(
        session: VTMotionEstimationSessionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kVTMotionEstimationSessionCreationOption_MotionVectorSize: CFStringRef;
}
unsafe extern "C" {
    pub static kVTMotionEstimationSessionCreationOption_UseMultiPassSearch: CFStringRef;
}
unsafe extern "C" {
    pub static kVTMotionEstimationSessionCreationOption_DetectTrueMotion: CFStringRef;
}
unsafe extern "C" {
    pub static kVTMotionEstimationSessionCreationOption_Label: CFStringRef;
}
unsafe extern "C" {
    pub static mut VTFrameProcessorErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for VTInt32Point {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTInt32Point {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VTInt32Point", &[]);
}
unsafe impl objc2::encode::RefEncode for VTInt32Size {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTInt32Size {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VTInt32Size", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueVTCompressionSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVTCompressionSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVTCompressionSession", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueVTDecompressionSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVTDecompressionSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVTDecompressionSession", &[]);
}
unsafe impl objc2::encode::RefEncode for VTDecompressionOutputCallbackRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTDecompressionOutputCallbackRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("VTDecompressionOutputCallbackRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueVTFrameSilo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVTFrameSilo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVTFrameSilo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueVTMultiPassStorage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVTMultiPassStorage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVTMultiPassStorage", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueVTPixelTransferSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVTPixelTransferSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVTPixelTransferSession", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueVTPixelRotationSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVTPixelRotationSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVTPixelRotationSession", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueVTRAWProcessingSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVTRAWProcessingSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVTRAWProcessingSession", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueVTHDRPerFrameMetadataGenerationSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVTHDRPerFrameMetadataGenerationSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVTHDRPerFrameMetadataGenerationSession", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueVTMotionEstimationSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVTMotionEstimationSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVTMotionEstimationSession", &[]);
}
unsafe impl objc2::encode::RefEncode for VTFrameProcessorFrame {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTFrameProcessorFrame {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTFrameProcessorOpticalFlow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTFrameProcessorOpticalFlow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTMotionBlurConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTMotionBlurConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTMotionBlurParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTMotionBlurParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTFrameRateConversionConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTFrameRateConversionConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTFrameRateConversionParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTFrameRateConversionParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTOpticalFlowConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTOpticalFlowConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTOpticalFlowParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTOpticalFlowParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTTemporalNoiseFilterConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTTemporalNoiseFilterConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTTemporalNoiseFilterParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTTemporalNoiseFilterParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTFrameProcessor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTFrameProcessor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTSuperResolutionScalerConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTSuperResolutionScalerConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTSuperResolutionScalerParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTSuperResolutionScalerParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTLowLatencySuperResolutionScalerConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTLowLatencySuperResolutionScalerConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTLowLatencySuperResolutionScalerParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTLowLatencySuperResolutionScalerParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTLowLatencyFrameInterpolationConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTLowLatencyFrameInterpolationConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VTLowLatencyFrameInterpolationParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VTLowLatencyFrameInterpolationParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
