#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CNCinematicErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNAssetInfo(pub id);
impl std::ops::Deref for CNAssetInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNAssetInfo {}
impl CNAssetInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNAssetInfo").unwrap(), alloc) })
    }
}
impl INSObject for CNAssetInfo {}
impl PNSObject for CNAssetInfo {}
impl std::convert::TryFrom<NSObject> for CNAssetInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNAssetInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNAssetInfo").unwrap()) };
        if is_kind_of {
            Ok(CNAssetInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNAssetInfo")
        }
    }
}
impl ICNAssetInfo for CNAssetInfo {}
pub trait ICNAssetInfo: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn asset(&self) -> AVAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asset)
    }
    unsafe fn allCinematicTracks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allCinematicTracks)
    }
    unsafe fn cinematicVideoTrack(&self) -> AVAssetTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cinematicVideoTrack)
    }
    unsafe fn cinematicDisparityTrack(&self) -> AVAssetTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cinematicDisparityTrack)
    }
    unsafe fn cinematicMetadataTrack(&self) -> AVAssetTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cinematicMetadataTrack)
    }
    unsafe fn timeRange(&self) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRange)
    }
    unsafe fn naturalSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, naturalSize)
    }
    unsafe fn preferredSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredSize)
    }
    unsafe fn preferredTransform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredTransform)
    }
    unsafe fn checkIfCinematic_completionHandler_(
        asset: AVAsset,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNAssetInfo").unwrap(), checkIfCinematic : asset, completionHandler : completionHandler)
    }
    unsafe fn loadFromAsset_completionHandler_(
        asset: AVAsset,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNAssetInfo").unwrap(), loadFromAsset : asset, completionHandler : completionHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNAssetInfo").unwrap(), new)
    }
}
impl CNAssetInfo_AbstractTracks for CNAssetInfo {}
pub trait CNAssetInfo_AbstractTracks: Sized + std::ops::Deref {
    unsafe fn frameTimingTrack(&self) -> AVAssetTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameTimingTrack)
    }
    unsafe fn videoCompositionTracks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoCompositionTracks)
    }
    unsafe fn videoCompositionTrackIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoCompositionTrackIDs)
    }
    unsafe fn sampleDataTrackIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleDataTrackIDs)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNCompositionInfo(pub id);
impl std::ops::Deref for CNCompositionInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNCompositionInfo {}
impl CNCompositionInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNCompositionInfo").unwrap(), alloc) })
    }
}
impl ICNAssetInfo for CNCompositionInfo {}
impl From<CNCompositionInfo> for CNAssetInfo {
    fn from(child: CNCompositionInfo) -> CNAssetInfo {
        CNAssetInfo(child.0)
    }
}
impl std::convert::TryFrom<CNAssetInfo> for CNCompositionInfo {
    type Error = &'static str;
    fn try_from(parent: CNAssetInfo) -> Result<CNCompositionInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNCompositionInfo").unwrap()) };
        if is_kind_of {
            Ok(CNCompositionInfo(parent.0))
        } else {
            Err("This CNAssetInfo cannot be downcasted to CNCompositionInfo")
        }
    }
}
impl INSObject for CNCompositionInfo {}
impl PNSObject for CNCompositionInfo {}
impl ICNCompositionInfo for CNCompositionInfo {}
pub trait ICNCompositionInfo: Sized + std::ops::Deref {
    unsafe fn insertTimeRange_ofCinematicAssetInfo_atTime_error_(
        &self,
        timeRange: CMTimeRange,
        assetInfo: CNAssetInfo,
        startTime: CMTime,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertTimeRange : timeRange, ofCinematicAssetInfo : assetInfo, atTime : startTime, error : outError)
    }
}
pub trait AVMutableComposition_CNComposition: Sized + std::ops::Deref {
    unsafe fn addTracksForCinematicAssetInfo_preferredStartingTrackID_(
        &self,
        assetInfo: CNAssetInfo,
        preferredStartingTrackID: CMPersistentTrackID,
    ) -> CNCompositionInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTracksForCinematicAssetInfo : assetInfo, preferredStartingTrackID : preferredStartingTrackID)
    }
}
pub type CNSpatialAudioRenderingStyle = NSInteger;
pub type CNSpatialAudioContentType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNAssetSpatialAudioInfo(pub id);
impl std::ops::Deref for CNAssetSpatialAudioInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNAssetSpatialAudioInfo {}
impl CNAssetSpatialAudioInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNAssetSpatialAudioInfo").unwrap(), alloc) })
    }
}
impl INSObject for CNAssetSpatialAudioInfo {}
impl PNSObject for CNAssetSpatialAudioInfo {}
impl std::convert::TryFrom<NSObject> for CNAssetSpatialAudioInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNAssetSpatialAudioInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNAssetSpatialAudioInfo").unwrap()) };
        if is_kind_of {
            Ok(CNAssetSpatialAudioInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNAssetSpatialAudioInfo")
        }
    }
}
impl ICNAssetSpatialAudioInfo for CNAssetSpatialAudioInfo {}
pub trait ICNAssetSpatialAudioInfo: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNAssetSpatialAudioInfo").unwrap(), new)
    }
    unsafe fn checkIfContainsSpatialAudio_completionHandler_(
        asset: AVAsset,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNAssetSpatialAudioInfo").unwrap(), checkIfContainsSpatialAudio : asset, completionHandler : completionHandler)
    }
    unsafe fn loadFromAsset_completionHandler_(
        asset: AVAsset,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNAssetSpatialAudioInfo").unwrap(), loadFromAsset : asset, completionHandler : completionHandler)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNAssetSpatialAudioInfo").unwrap(), isSupported)
    }
}
impl CNAssetSpatialAudioInfo_Properties for CNAssetSpatialAudioInfo {}
pub trait CNAssetSpatialAudioInfo_Properties: Sized + std::ops::Deref {
    unsafe fn defaultSpatialAudioTrack(&self) -> AVAssetTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultSpatialAudioTrack)
    }
    unsafe fn defaultEffectIntensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultEffectIntensity)
    }
    unsafe fn defaultRenderingStyle(&self) -> CNSpatialAudioRenderingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultRenderingStyle)
    }
    unsafe fn spatialAudioMixMetadata(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spatialAudioMixMetadata)
    }
}
impl CNAssetSpatialAudioInfo_SynthesizeAVFoundationObjects for CNAssetSpatialAudioInfo {}
pub trait CNAssetSpatialAudioInfo_SynthesizeAVFoundationObjects: Sized + std::ops::Deref {
    unsafe fn audioMixWithEffectIntensity_renderingStyle_(
        &self,
        effectIntensity: f32,
        renderingStyle: CNSpatialAudioRenderingStyle,
    ) -> AVAudioMix
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, audioMixWithEffectIntensity : effectIntensity, renderingStyle : renderingStyle)
    }
    unsafe fn assetReaderOutputSettingsForContentType_(
        &self,
        contentType: CNSpatialAudioContentType,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assetReaderOutputSettingsForContentType : contentType)
    }
    unsafe fn assetWriterInputSettingsForContentType_(
        &self,
        contentType: CNSpatialAudioContentType,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assetWriterInputSettingsForContentType : contentType)
    }
}
pub type CNRenderingQuality = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNRenderingSessionAttributes(pub id);
impl std::ops::Deref for CNRenderingSessionAttributes {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNRenderingSessionAttributes {}
impl CNRenderingSessionAttributes {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNRenderingSessionAttributes").unwrap(), alloc) })
    }
}
impl INSObject for CNRenderingSessionAttributes {}
impl PNSObject for CNRenderingSessionAttributes {}
impl std::convert::TryFrom<NSObject> for CNRenderingSessionAttributes {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNRenderingSessionAttributes, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNRenderingSessionAttributes").unwrap()) };
        if is_kind_of {
            Ok(CNRenderingSessionAttributes(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNRenderingSessionAttributes")
        }
    }
}
impl ICNRenderingSessionAttributes for CNRenderingSessionAttributes {}
pub trait ICNRenderingSessionAttributes: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn renderingVersion(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderingVersion)
    }
    unsafe fn loadFromAsset_completionHandler_(
        asset: AVAsset,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNRenderingSessionAttributes").unwrap(), loadFromAsset : asset, completionHandler : completionHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNRenderingSessionAttributes").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNRenderingSessionFrameAttributes(pub id);
impl std::ops::Deref for CNRenderingSessionFrameAttributes {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNRenderingSessionFrameAttributes {}
impl CNRenderingSessionFrameAttributes {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNRenderingSessionFrameAttributes").unwrap(), alloc) })
    }
}
impl PNSCopying for CNRenderingSessionFrameAttributes {}
impl PNSMutableCopying for CNRenderingSessionFrameAttributes {}
impl INSObject for CNRenderingSessionFrameAttributes {}
impl PNSObject for CNRenderingSessionFrameAttributes {}
impl std::convert::TryFrom<NSObject> for CNRenderingSessionFrameAttributes {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNRenderingSessionFrameAttributes, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNRenderingSessionFrameAttributes").unwrap())
        };
        if is_kind_of {
            Ok(CNRenderingSessionFrameAttributes(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNRenderingSessionFrameAttributes")
        }
    }
}
impl ICNRenderingSessionFrameAttributes for CNRenderingSessionFrameAttributes {}
pub trait ICNRenderingSessionFrameAttributes: Sized + std::ops::Deref {
    unsafe fn initWithSampleBuffer_sessionAttributes_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        sessionAttributes: CNRenderingSessionAttributes,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSampleBuffer : sampleBuffer, sessionAttributes : sessionAttributes)
    }
    unsafe fn initWithTimedMetadataGroup_sessionAttributes_(
        &self,
        metadataGroup: AVTimedMetadataGroup,
        sessionAttributes: CNRenderingSessionAttributes,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTimedMetadataGroup : metadataGroup, sessionAttributes : sessionAttributes)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn focusDisparity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusDisparity)
    }
    unsafe fn setFocusDisparity_(&self, focusDisparity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusDisparity : focusDisparity)
    }
    unsafe fn fNumber(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fNumber)
    }
    unsafe fn setFNumber_(&self, fNumber: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFNumber : fNumber)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNRenderingSessionFrameAttributes").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNRenderingSession(pub id);
impl std::ops::Deref for CNRenderingSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNRenderingSession {}
impl CNRenderingSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNRenderingSession").unwrap(), alloc) })
    }
}
impl INSObject for CNRenderingSession {}
impl PNSObject for CNRenderingSession {}
impl std::convert::TryFrom<NSObject> for CNRenderingSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNRenderingSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNRenderingSession").unwrap()) };
        if is_kind_of {
            Ok(CNRenderingSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNRenderingSession")
        }
    }
}
impl ICNRenderingSession for CNRenderingSession {}
pub trait ICNRenderingSession: Sized + std::ops::Deref {
    unsafe fn initWithCommandQueue_sessionAttributes_preferredTransform_quality_(
        &self,
        commandQueue: *mut u64,
        sessionAttributes: CNRenderingSessionAttributes,
        preferredTransform: CGAffineTransform,
        quality: CNRenderingQuality,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCommandQueue : commandQueue, sessionAttributes : sessionAttributes, preferredTransform : preferredTransform, quality : quality)
    }
    unsafe fn encodeRenderToCommandBuffer_frameAttributes_sourceImage_sourceDisparity_destinationImage_(
        &self,
        commandBuffer: *mut u64,
        frameAttributes: CNRenderingSessionFrameAttributes,
        sourceImage: CVPixelBufferRef,
        sourceDisparity: CVPixelBufferRef,
        destinationImage: CVPixelBufferRef,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeRenderToCommandBuffer : commandBuffer, frameAttributes : frameAttributes, sourceImage : sourceImage, sourceDisparity : sourceDisparity, destinationImage : destinationImage)
    }
    unsafe fn encodeRenderToCommandBuffer_frameAttributes_sourceImage_sourceDisparity_destinationRGBA_(
        &self,
        commandBuffer: *mut u64,
        frameAttributes: CNRenderingSessionFrameAttributes,
        sourceImage: CVPixelBufferRef,
        sourceDisparity: CVPixelBufferRef,
        destinationRGBA: *mut u64,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeRenderToCommandBuffer : commandBuffer, frameAttributes : frameAttributes, sourceImage : sourceImage, sourceDisparity : sourceDisparity, destinationRGBA : destinationRGBA)
    }
    unsafe fn encodeRenderToCommandBuffer_frameAttributes_sourceImage_sourceDisparity_destinationLuma_destinationChroma_(
        &self,
        commandBuffer: *mut u64,
        frameAttributes: CNRenderingSessionFrameAttributes,
        sourceImage: CVPixelBufferRef,
        sourceDisparity: CVPixelBufferRef,
        destinationLuma: *mut u64,
        destinationChroma: *mut u64,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeRenderToCommandBuffer : commandBuffer, frameAttributes : frameAttributes, sourceImage : sourceImage, sourceDisparity : sourceDisparity, destinationLuma : destinationLuma, destinationChroma : destinationChroma)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn commandQueue(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandQueue)
    }
    unsafe fn sessionAttributes(&self) -> CNRenderingSessionAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionAttributes)
    }
    unsafe fn preferredTransform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredTransform)
    }
    unsafe fn quality(&self) -> CNRenderingQuality
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quality)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNRenderingSession").unwrap(), new)
    }
    unsafe fn sourcePixelFormatTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNRenderingSession").unwrap(), sourcePixelFormatTypes)
    }
    unsafe fn destinationPixelFormatTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNRenderingSession").unwrap(), destinationPixelFormatTypes)
    }
}
pub type CNDetectionID = i64;
pub type CNDetectionGroupID = i64;
pub type CNDetectionType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNDetection(pub id);
impl std::ops::Deref for CNDetection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNDetection {}
impl CNDetection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNDetection").unwrap(), alloc) })
    }
}
impl PNSCopying for CNDetection {}
impl INSObject for CNDetection {}
impl PNSObject for CNDetection {}
impl std::convert::TryFrom<NSObject> for CNDetection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNDetection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNDetection").unwrap()) };
        if is_kind_of {
            Ok(CNDetection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNDetection")
        }
    }
}
impl ICNDetection for CNDetection {}
pub trait ICNDetection: Sized + std::ops::Deref {
    unsafe fn initWithTime_detectionType_normalizedRect_focusDisparity_(
        &self,
        time: CMTime,
        detectionType: CNDetectionType,
        normalizedRect: CGRect,
        focusDisparity: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTime : time, detectionType : detectionType, normalizedRect : normalizedRect, focusDisparity : focusDisparity)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn time(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn detectionType(&self) -> CNDetectionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectionType)
    }
    unsafe fn normalizedRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedRect)
    }
    unsafe fn focusDisparity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusDisparity)
    }
    unsafe fn detectionID(&self) -> CNDetectionID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectionID)
    }
    unsafe fn detectionGroupID(&self) -> CNDetectionGroupID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectionGroupID)
    }
    unsafe fn isValidDetectionID_(detectionID: CNDetectionID) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNDetection").unwrap(), isValidDetectionID : detectionID)
    }
    unsafe fn isValidDetectionGroupID_(detectionGroupID: CNDetectionGroupID) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNDetection").unwrap(), isValidDetectionGroupID : detectionGroupID)
    }
    unsafe fn accessibilityLabelForDetectionType_(detectionType: CNDetectionType) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNDetection").unwrap(), accessibilityLabelForDetectionType : detectionType)
    }
    unsafe fn disparityInNormalizedRect_sourceDisparity_detectionType_priorDisparity_(
        normalizedRect: CGRect,
        sourceDisparity: CVPixelBufferRef,
        detectionType: CNDetectionType,
        priorDisparity: f32,
    ) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNDetection").unwrap(), disparityInNormalizedRect : normalizedRect, sourceDisparity : sourceDisparity, detectionType : detectionType, priorDisparity : priorDisparity)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNDetection").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNDecision(pub id);
impl std::ops::Deref for CNDecision {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNDecision {}
impl CNDecision {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNDecision").unwrap(), alloc) })
    }
}
impl PNSCopying for CNDecision {}
impl INSObject for CNDecision {}
impl PNSObject for CNDecision {}
impl std::convert::TryFrom<NSObject> for CNDecision {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNDecision, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNDecision").unwrap()) };
        if is_kind_of {
            Ok(CNDecision(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNDecision")
        }
    }
}
impl ICNDecision for CNDecision {}
pub trait ICNDecision: Sized + std::ops::Deref {
    unsafe fn initWithTime_detectionID_strong_(
        &self,
        time: CMTime,
        detectionID: CNDetectionID,
        isStrong: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTime : time, detectionID : detectionID, strong : isStrong)
    }
    unsafe fn initWithTime_detectionGroupID_strong_(
        &self,
        time: CMTime,
        detectionGroupID: CNDetectionGroupID,
        isStrong: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTime : time, detectionGroupID : detectionGroupID, strong : isStrong)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn time(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn detectionID(&self) -> CNDetectionID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectionID)
    }
    unsafe fn detectionGroupID(&self) -> CNDetectionGroupID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectionGroupID)
    }
    unsafe fn isUserDecision(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserDecision)
    }
    unsafe fn isGroupDecision(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGroupDecision)
    }
    unsafe fn isStrongDecision(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStrongDecision)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNDecision").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNDetectionTrack(pub id);
impl std::ops::Deref for CNDetectionTrack {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNDetectionTrack {}
impl CNDetectionTrack {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNDetectionTrack").unwrap(), alloc) })
    }
}
impl PNSCopying for CNDetectionTrack {}
impl INSObject for CNDetectionTrack {}
impl PNSObject for CNDetectionTrack {}
impl std::convert::TryFrom<NSObject> for CNDetectionTrack {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNDetectionTrack, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNDetectionTrack").unwrap()) };
        if is_kind_of {
            Ok(CNDetectionTrack(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNDetectionTrack")
        }
    }
}
impl ICNDetectionTrack for CNDetectionTrack {}
pub trait ICNDetectionTrack: Sized + std::ops::Deref {
    unsafe fn detectionAtOrBeforeTime_(&self, time: CMTime) -> CNDetection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectionAtOrBeforeTime : time)
    }
    unsafe fn detectionNearestTime_(&self, time: CMTime) -> CNDetection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectionNearestTime : time)
    }
    unsafe fn detectionsInTimeRange_(&self, timeRange: CMTimeRange) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectionsInTimeRange : timeRange)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn detectionType(&self) -> CNDetectionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectionType)
    }
    unsafe fn detectionID(&self) -> CNDetectionID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectionID)
    }
    unsafe fn detectionGroupID(&self) -> CNDetectionGroupID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectionGroupID)
    }
    unsafe fn isUserCreated(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserCreated)
    }
    unsafe fn isDiscrete(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDiscrete)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNDetectionTrack").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNFixedDetectionTrack(pub id);
impl std::ops::Deref for CNFixedDetectionTrack {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNFixedDetectionTrack {}
impl CNFixedDetectionTrack {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNFixedDetectionTrack").unwrap(), alloc) })
    }
}
impl ICNDetectionTrack for CNFixedDetectionTrack {}
impl PNSCopying for CNFixedDetectionTrack {}
impl From<CNFixedDetectionTrack> for CNDetectionTrack {
    fn from(child: CNFixedDetectionTrack) -> CNDetectionTrack {
        CNDetectionTrack(child.0)
    }
}
impl std::convert::TryFrom<CNDetectionTrack> for CNFixedDetectionTrack {
    type Error = &'static str;
    fn try_from(parent: CNDetectionTrack) -> Result<CNFixedDetectionTrack, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNFixedDetectionTrack").unwrap()) };
        if is_kind_of {
            Ok(CNFixedDetectionTrack(parent.0))
        } else {
            Err("This CNDetectionTrack cannot be downcasted to CNFixedDetectionTrack")
        }
    }
}
impl INSObject for CNFixedDetectionTrack {}
impl PNSObject for CNFixedDetectionTrack {}
impl ICNFixedDetectionTrack for CNFixedDetectionTrack {}
pub trait ICNFixedDetectionTrack: Sized + std::ops::Deref {
    unsafe fn initWithFocusDisparity_(&self, focusDisparity: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFocusDisparity : focusDisparity)
    }
    unsafe fn initWithOriginalDetection_(&self, originalDetection: CNDetection) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOriginalDetection : originalDetection)
    }
    unsafe fn focusDisparity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusDisparity)
    }
    unsafe fn originalDetection(&self) -> CNDetection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalDetection)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNCustomDetectionTrack(pub id);
impl std::ops::Deref for CNCustomDetectionTrack {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNCustomDetectionTrack {}
impl CNCustomDetectionTrack {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNCustomDetectionTrack").unwrap(), alloc) })
    }
}
impl ICNDetectionTrack for CNCustomDetectionTrack {}
impl PNSCopying for CNCustomDetectionTrack {}
impl std::convert::TryFrom<CNDetectionTrack> for CNCustomDetectionTrack {
    type Error = &'static str;
    fn try_from(parent: CNDetectionTrack) -> Result<CNCustomDetectionTrack, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNCustomDetectionTrack").unwrap()) };
        if is_kind_of {
            Ok(CNCustomDetectionTrack(parent.0))
        } else {
            Err("This CNDetectionTrack cannot be downcasted to CNCustomDetectionTrack")
        }
    }
}
impl INSObject for CNCustomDetectionTrack {}
impl PNSObject for CNCustomDetectionTrack {}
impl ICNCustomDetectionTrack for CNCustomDetectionTrack {}
pub trait ICNCustomDetectionTrack: Sized + std::ops::Deref {
    unsafe fn initWithDetections_smooth_(
        &self,
        detections: NSArray,
        applySmoothing: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDetections : detections, smooth : applySmoothing)
    }
    unsafe fn allDetections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allDetections)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNScript(pub id);
impl std::ops::Deref for CNScript {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNScript {}
impl CNScript {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNScript").unwrap(), alloc) })
    }
}
impl INSObject for CNScript {}
impl PNSObject for CNScript {}
impl std::convert::TryFrom<NSObject> for CNScript {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNScript, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNScript").unwrap()) };
        if is_kind_of {
            Ok(CNScript(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNScript")
        }
    }
}
impl ICNScript for CNScript {}
pub trait ICNScript: Sized + std::ops::Deref {
    unsafe fn reloadWithChanges_(&self, changes: CNScriptChanges)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reloadWithChanges : changes)
    }
    unsafe fn changes(&self) -> CNScriptChanges
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changes)
    }
    unsafe fn changesTrimmedByTimeRange_(&self, timeRange: CMTimeRange) -> CNScriptChanges
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changesTrimmedByTimeRange : timeRange)
    }
    unsafe fn frameAtTime_tolerance_(&self, time: CMTime, tolerance: CMTime) -> CNScriptFrame
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, frameAtTime : time, tolerance : tolerance)
    }
    unsafe fn framesInTimeRange_(&self, timeRange: CMTimeRange) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, framesInTimeRange : timeRange)
    }
    unsafe fn decisionAtTime_tolerance_(&self, time: CMTime, tolerance: CMTime) -> CNDecision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, decisionAtTime : time, tolerance : tolerance)
    }
    unsafe fn decisionsInTimeRange_(&self, timeRange: CMTimeRange) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, decisionsInTimeRange : timeRange)
    }
    unsafe fn decisionAfterTime_(&self, time: CMTime) -> CNDecision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, decisionAfterTime : time)
    }
    unsafe fn decisionBeforeTime_(&self, time: CMTime) -> CNDecision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, decisionBeforeTime : time)
    }
    unsafe fn primaryDecisionAtTime_(&self, time: CMTime) -> CNDecision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, primaryDecisionAtTime : time)
    }
    unsafe fn secondaryDecisionAtTime_(&self, time: CMTime) -> CNDecision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, secondaryDecisionAtTime : time)
    }
    unsafe fn timeRangeOfTransitionAfterDecision_(&self, decision: CNDecision) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, timeRangeOfTransitionAfterDecision : decision)
    }
    unsafe fn timeRangeOfTransitionBeforeDecision_(&self, decision: CNDecision) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, timeRangeOfTransitionBeforeDecision : decision)
    }
    unsafe fn userDecisionsInTimeRange_(&self, timeRange: CMTimeRange) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userDecisionsInTimeRange : timeRange)
    }
    unsafe fn baseDecisionsInTimeRange_(&self, timeRange: CMTimeRange) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, baseDecisionsInTimeRange : timeRange)
    }
    unsafe fn detectionTrackForID_(&self, detectionID: CNDetectionID) -> CNDetectionTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectionTrackForID : detectionID)
    }
    unsafe fn detectionTrackForDecision_(&self, decision: CNDecision) -> CNDetectionTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectionTrackForDecision : decision)
    }
    unsafe fn addUserDecision_(&self, decision: CNDecision) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addUserDecision : decision)
    }
    unsafe fn removeUserDecision_(&self, decision: CNDecision) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeUserDecision : decision)
    }
    unsafe fn removeAllUserDecisions(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllUserDecisions)
    }
    unsafe fn addDetectionTrack_(&self, detectionTrack: CNDetectionTrack) -> CNDetectionID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addDetectionTrack : detectionTrack)
    }
    unsafe fn removeDetectionTrack_(&self, detectionTrack: CNDetectionTrack) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeDetectionTrack : detectionTrack)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn timeRange(&self) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRange)
    }
    unsafe fn fNumber(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fNumber)
    }
    unsafe fn setFNumber_(&self, fNumber: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFNumber : fNumber)
    }
    unsafe fn addedDetectionTracks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addedDetectionTracks)
    }
    unsafe fn loadFromAsset_changes_progress_completionHandler_(
        asset: AVAsset,
        changes: CNScriptChanges,
        progress: NSProgress,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNScript").unwrap(), loadFromAsset : asset, changes : changes, progress : progress, completionHandler : completionHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNScript").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNScriptChanges(pub id);
impl std::ops::Deref for CNScriptChanges {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNScriptChanges {}
impl CNScriptChanges {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNScriptChanges").unwrap(), alloc) })
    }
}
impl INSObject for CNScriptChanges {}
impl PNSObject for CNScriptChanges {}
impl std::convert::TryFrom<NSObject> for CNScriptChanges {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNScriptChanges, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNScriptChanges").unwrap()) };
        if is_kind_of {
            Ok(CNScriptChanges(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNScriptChanges")
        }
    }
}
impl ICNScriptChanges for CNScriptChanges {}
pub trait ICNScriptChanges: Sized + std::ops::Deref {
    unsafe fn initWithDataRepresentation_(&self, dataRepresentation: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDataRepresentation : dataRepresentation)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
    unsafe fn fNumber(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fNumber)
    }
    unsafe fn userDecisions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userDecisions)
    }
    unsafe fn addedDetectionTracks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addedDetectionTracks)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNScriptChanges").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNScriptFrame(pub id);
impl std::ops::Deref for CNScriptFrame {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNScriptFrame {}
impl CNScriptFrame {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNScriptFrame").unwrap(), alloc) })
    }
}
impl PNSCopying for CNScriptFrame {}
impl INSObject for CNScriptFrame {}
impl PNSObject for CNScriptFrame {}
impl std::convert::TryFrom<NSObject> for CNScriptFrame {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNScriptFrame, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNScriptFrame").unwrap()) };
        if is_kind_of {
            Ok(CNScriptFrame(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNScriptFrame")
        }
    }
}
impl ICNScriptFrame for CNScriptFrame {}
pub trait ICNScriptFrame: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn time(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn focusDisparity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusDisparity)
    }
    unsafe fn focusDetection(&self) -> CNDetection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusDetection)
    }
    unsafe fn allDetections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allDetections)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNScriptFrame").unwrap(), new)
    }
}
impl CNScriptFrame_CNExtensions for CNScriptFrame {}
pub trait CNScriptFrame_CNExtensions: Sized + std::ops::Deref {
    unsafe fn detectionForID_(&self, detectionID: CNDetectionID) -> CNDetection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detectionForID : detectionID)
    }
    unsafe fn bestDetectionForGroupID_(&self, detectionGroupID: CNDetectionGroupID) -> CNDetection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bestDetectionForGroupID : detectionGroupID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNBoundsPrediction(pub id);
impl std::ops::Deref for CNBoundsPrediction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNBoundsPrediction {}
impl CNBoundsPrediction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNBoundsPrediction").unwrap(), alloc) })
    }
}
impl PNSCopying for CNBoundsPrediction {}
impl PNSMutableCopying for CNBoundsPrediction {}
impl INSObject for CNBoundsPrediction {}
impl PNSObject for CNBoundsPrediction {}
impl std::convert::TryFrom<NSObject> for CNBoundsPrediction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNBoundsPrediction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNBoundsPrediction").unwrap()) };
        if is_kind_of {
            Ok(CNBoundsPrediction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNBoundsPrediction")
        }
    }
}
impl ICNBoundsPrediction for CNBoundsPrediction {}
pub trait ICNBoundsPrediction: Sized + std::ops::Deref {
    unsafe fn normalizedBounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedBounds)
    }
    unsafe fn setNormalizedBounds_(&self, normalizedBounds: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalizedBounds : normalizedBounds)
    }
    unsafe fn confidence(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
    unsafe fn setConfidence_(&self, confidence: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfidence : confidence)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNObjectTracker(pub id);
impl std::ops::Deref for CNObjectTracker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNObjectTracker {}
impl CNObjectTracker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNObjectTracker").unwrap(), alloc) })
    }
}
impl INSObject for CNObjectTracker {}
impl PNSObject for CNObjectTracker {}
impl std::convert::TryFrom<NSObject> for CNObjectTracker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNObjectTracker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNObjectTracker").unwrap()) };
        if is_kind_of {
            Ok(CNObjectTracker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNObjectTracker")
        }
    }
}
impl ICNObjectTracker for CNObjectTracker {}
pub trait ICNObjectTracker: Sized + std::ops::Deref {
    unsafe fn initWithCommandQueue_(&self, commandQueue: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCommandQueue : commandQueue)
    }
    unsafe fn findObjectAtPoint_sourceImage_(
        &self,
        point: CGPoint,
        sourceImage: CVPixelBufferRef,
    ) -> CNBoundsPrediction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findObjectAtPoint : point, sourceImage : sourceImage)
    }
    unsafe fn startTrackingAt_within_sourceImage_sourceDisparity_(
        &self,
        time: CMTime,
        normalizedBounds: CGRect,
        sourceImage: CVPixelBufferRef,
        sourceDisparity: CVPixelBufferRef,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startTrackingAt : time, within : normalizedBounds, sourceImage : sourceImage, sourceDisparity : sourceDisparity)
    }
    unsafe fn continueTrackingAt_sourceImage_sourceDisparity_(
        &self,
        time: CMTime,
        sourceImage: CVPixelBufferRef,
        sourceDisparity: CVPixelBufferRef,
    ) -> CNBoundsPrediction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, continueTrackingAt : time, sourceImage : sourceImage, sourceDisparity : sourceDisparity)
    }
    unsafe fn finishDetectionTrack(&self) -> CNDetectionTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finishDetectionTrack)
    }
    unsafe fn resetDetectionTrack(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetDetectionTrack)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNObjectTracker").unwrap(), new)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNObjectTracker").unwrap(), isSupported)
    }
}
unsafe extern "C" {
    pub static CNCinematicErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for CNAssetInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNAssetInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNCompositionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNCompositionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNAssetSpatialAudioInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNAssetSpatialAudioInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNRenderingSessionAttributes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNRenderingSessionAttributes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNRenderingSessionFrameAttributes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNRenderingSessionFrameAttributes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNRenderingSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNRenderingSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNDetection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNDetection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNDecision {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNDecision {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNDetectionTrack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNDetectionTrack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNFixedDetectionTrack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNFixedDetectionTrack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNCustomDetectionTrack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNCustomDetectionTrack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNScript {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNScript {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNScriptChanges {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNScriptChanges {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNScriptFrame {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNScriptFrame {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNBoundsPrediction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNBoundsPrediction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNObjectTracker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNObjectTracker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
