#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type MEError = NSInteger;
pub type MEFileInfoFragmentsStatus = NSInteger;
pub type MEFormatReaderParseAdditionalFragmentsStatus = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEFormatReaderInstantiationOptions(pub id);
impl std::ops::Deref for MEFormatReaderInstantiationOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEFormatReaderInstantiationOptions {}
impl MEFormatReaderInstantiationOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEFormatReaderInstantiationOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for MEFormatReaderInstantiationOptions {}
impl INSObject for MEFormatReaderInstantiationOptions {}
impl PNSObject for MEFormatReaderInstantiationOptions {}
impl std::convert::TryFrom<NSObject> for MEFormatReaderInstantiationOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEFormatReaderInstantiationOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEFormatReaderInstantiationOptions").unwrap())
        };
        if is_kind_of {
            Ok(MEFormatReaderInstantiationOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEFormatReaderInstantiationOptions")
        }
    }
}
impl IMEFormatReaderInstantiationOptions for MEFormatReaderInstantiationOptions {}
pub trait IMEFormatReaderInstantiationOptions: Sized + std::ops::Deref {
    unsafe fn allowIncrementalFragmentParsing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowIncrementalFragmentParsing)
    }
}
pub trait PMEFormatReaderExtension: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn formatReaderWithByteSource_options_error_(
        &self,
        primaryByteSource: MEByteSource,
        options: MEFormatReaderInstantiationOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, formatReaderWithByteSource : primaryByteSource, options : options, error : error)
    }
}
pub trait PMEFormatReader: Sized + std::ops::Deref {
    unsafe fn loadFileInfoWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFileInfoWithCompletionHandler : completionHandler)
    }
    unsafe fn loadMetadataWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadMetadataWithCompletionHandler : completionHandler)
    }
    unsafe fn loadTrackReadersWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadTrackReadersWithCompletionHandler : completionHandler)
    }
    unsafe fn parseAdditionalFragmentsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parseAdditionalFragmentsWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEFileInfo(pub id);
impl std::ops::Deref for MEFileInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEFileInfo {}
impl MEFileInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEFileInfo").unwrap(), alloc) })
    }
}
impl PNSCopying for MEFileInfo {}
impl INSObject for MEFileInfo {}
impl PNSObject for MEFileInfo {}
impl std::convert::TryFrom<NSObject> for MEFileInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEFileInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEFileInfo").unwrap()) };
        if is_kind_of {
            Ok(MEFileInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEFileInfo")
        }
    }
}
impl IMEFileInfo for MEFileInfo {}
pub trait IMEFileInfo: Sized + std::ops::Deref {
    unsafe fn duration(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn setDuration_(&self, duration: CMTime)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDuration : duration)
    }
    unsafe fn fragmentsStatus(&self) -> MEFileInfoFragmentsStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentsStatus)
    }
    unsafe fn setFragmentsStatus_(&self, fragmentsStatus: MEFileInfoFragmentsStatus)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentsStatus : fragmentsStatus)
    }
    unsafe fn sidecarFileName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sidecarFileName)
    }
    unsafe fn setSidecarFileName_(&self, sidecarFileName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSidecarFileName : sidecarFileName)
    }
}
pub trait PMETrackReader: Sized + std::ops::Deref {
    unsafe fn loadTrackInfoWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadTrackInfoWithCompletionHandler : completionHandler)
    }
    unsafe fn generateSampleCursorAtPresentationTimeStamp_completionHandler_(
        &self,
        presentationTimeStamp: CMTime,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateSampleCursorAtPresentationTimeStamp : presentationTimeStamp, completionHandler : completionHandler)
    }
    unsafe fn generateSampleCursorAtFirstSampleInDecodeOrderWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateSampleCursorAtFirstSampleInDecodeOrderWithCompletionHandler : completionHandler)
    }
    unsafe fn generateSampleCursorAtLastSampleInDecodeOrderWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateSampleCursorAtLastSampleInDecodeOrderWithCompletionHandler : completionHandler)
    }
    unsafe fn loadUneditedDurationWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadUneditedDurationWithCompletionHandler : completionHandler)
    }
    unsafe fn loadTotalSampleDataLengthWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadTotalSampleDataLengthWithCompletionHandler : completionHandler)
    }
    unsafe fn loadEstimatedDataRateWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadEstimatedDataRateWithCompletionHandler : completionHandler)
    }
    unsafe fn loadMetadataWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadMetadataWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct METrackInfo(pub id);
impl std::ops::Deref for METrackInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for METrackInfo {}
impl METrackInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"METrackInfo").unwrap(), alloc) })
    }
}
impl PNSCopying for METrackInfo {}
impl INSObject for METrackInfo {}
impl PNSObject for METrackInfo {}
impl std::convert::TryFrom<NSObject> for METrackInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<METrackInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"METrackInfo").unwrap()) };
        if is_kind_of {
            Ok(METrackInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to METrackInfo")
        }
    }
}
impl IMETrackInfo for METrackInfo {}
pub trait IMETrackInfo: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMediaType_trackID_formatDescriptions_(
        &self,
        mediaType: CMMediaType,
        trackID: CMPersistentTrackID,
        formatDescriptions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMediaType : mediaType, trackID : trackID, formatDescriptions : formatDescriptions)
    }
    unsafe fn mediaType(&self) -> CMMediaType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaType)
    }
    unsafe fn trackID(&self) -> CMPersistentTrackID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackID)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn formatDescriptions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formatDescriptions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"METrackInfo").unwrap(), new)
    }
}
impl METrackInfo_OptionalProperties for METrackInfo {}
pub trait METrackInfo_OptionalProperties: Sized + std::ops::Deref {
    unsafe fn naturalTimescale(&self) -> CMTimeScale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, naturalTimescale)
    }
    unsafe fn setNaturalTimescale_(&self, naturalTimescale: CMTimeScale)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNaturalTimescale : naturalTimescale)
    }
    unsafe fn trackEdits(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackEdits)
    }
    unsafe fn setTrackEdits_(&self, trackEdits: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrackEdits : trackEdits)
    }
}
impl METrackInfo_LanguageTagOptionalProperties for METrackInfo {}
pub trait METrackInfo_LanguageTagOptionalProperties: Sized + std::ops::Deref {
    unsafe fn extendedLanguageTag(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extendedLanguageTag)
    }
    unsafe fn setExtendedLanguageTag_(&self, extendedLanguageTag: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtendedLanguageTag : extendedLanguageTag)
    }
}
impl METrackInfo_VideoSpecificOptionalProperties for METrackInfo {}
pub trait METrackInfo_VideoSpecificOptionalProperties: Sized + std::ops::Deref {
    unsafe fn naturalSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, naturalSize)
    }
    unsafe fn setNaturalSize_(&self, naturalSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNaturalSize : naturalSize)
    }
    unsafe fn preferredTransform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredTransform)
    }
    unsafe fn setPreferredTransform_(&self, preferredTransform: CGAffineTransform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredTransform : preferredTransform)
    }
    unsafe fn nominalFrameRate(&self) -> Float32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nominalFrameRate)
    }
    unsafe fn setNominalFrameRate_(&self, nominalFrameRate: Float32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNominalFrameRate : nominalFrameRate)
    }
    unsafe fn requiresFrameReordering(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresFrameReordering)
    }
    unsafe fn setRequiresFrameReordering_(&self, requiresFrameReordering: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresFrameReordering : requiresFrameReordering)
    }
}
pub trait PMESampleCursor: Sized + std::ops::Deref {
    unsafe fn stepInDecodeOrderByCount_completionHandler_(
        &self,
        stepCount: i64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stepInDecodeOrderByCount : stepCount, completionHandler : completionHandler)
    }
    unsafe fn stepInPresentationOrderByCount_completionHandler_(
        &self,
        stepCount: i64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stepInPresentationOrderByCount : stepCount, completionHandler : completionHandler)
    }
    unsafe fn stepByDecodeTime_completionHandler_(
        &self,
        deltaDecodeTime: CMTime,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stepByDecodeTime : deltaDecodeTime, completionHandler : completionHandler)
    }
    unsafe fn stepByPresentationTime_completionHandler_(
        &self,
        deltaPresentationTime: CMTime,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stepByPresentationTime : deltaPresentationTime, completionHandler : completionHandler)
    }
    unsafe fn samplesWithEarlierDTSsMayHaveLaterPTSsThanCursor_(&self, cursor: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, samplesWithEarlierDTSsMayHaveLaterPTSsThanCursor : cursor)
    }
    unsafe fn samplesWithLaterDTSsMayHaveEarlierPTSsThanCursor_(&self, cursor: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, samplesWithLaterDTSsMayHaveEarlierPTSsThanCursor : cursor)
    }
    unsafe fn chunkDetailsReturningError_(&self, error: *mut NSError) -> MESampleCursorChunk
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, chunkDetailsReturningError : error)
    }
    unsafe fn sampleLocationReturningError_(&self, error: *mut NSError) -> MESampleLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleLocationReturningError : error)
    }
    unsafe fn estimatedSampleLocationReturningError_(
        &self,
        error: *mut NSError,
    ) -> MEEstimatedSampleLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, estimatedSampleLocationReturningError : error)
    }
    unsafe fn refineSampleLocation_refinementData_refinementDataLength_refinedLocation_error_(
        &self,
        estimatedSampleLocation: AVSampleCursorStorageRange,
        refinementData: *const u8,
        refinementDataLength: usize,
        refinedLocationOut: *mut AVSampleCursorStorageRange,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, refineSampleLocation : estimatedSampleLocation, refinementData : refinementData, refinementDataLength : refinementDataLength, refinedLocation : refinedLocationOut, error : error)
    }
    unsafe fn loadSampleBufferContainingSamplesToEndCursor_completionHandler_(
        &self,
        endSampleCursor: *mut u64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadSampleBufferContainingSamplesToEndCursor : endSampleCursor, completionHandler : completionHandler)
    }
    unsafe fn loadPostDecodeProcessingMetadataWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadPostDecodeProcessingMetadataWithCompletionHandler : completionHandler)
    }
    unsafe fn presentationTimeStamp(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationTimeStamp)
    }
    unsafe fn decodeTimeStamp(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decodeTimeStamp)
    }
    unsafe fn currentSampleDuration(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentSampleDuration)
    }
    unsafe fn currentSampleFormatDescription(&self) -> CMFormatDescriptionRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentSampleFormatDescription)
    }
    unsafe fn syncInfo(&self) -> AVSampleCursorSyncInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, syncInfo)
    }
    unsafe fn dependencyInfo(&self) -> AVSampleCursorDependencyInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dependencyInfo)
    }
    unsafe fn hevcDependencyInfo(&self) -> MEHEVCDependencyInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hevcDependencyInfo)
    }
    unsafe fn decodeTimeOfLastSampleReachableByForwardSteppingThatIsAlreadyLoadedByByteSource(
        &self,
    ) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decodeTimeOfLastSampleReachableByForwardSteppingThatIsAlreadyLoadedByByteSource)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MESampleCursorChunk(pub id);
impl std::ops::Deref for MESampleCursorChunk {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MESampleCursorChunk {}
impl MESampleCursorChunk {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MESampleCursorChunk").unwrap(), alloc) })
    }
}
impl PNSCopying for MESampleCursorChunk {}
impl INSObject for MESampleCursorChunk {}
impl PNSObject for MESampleCursorChunk {}
impl std::convert::TryFrom<NSObject> for MESampleCursorChunk {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MESampleCursorChunk, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MESampleCursorChunk").unwrap()) };
        if is_kind_of {
            Ok(MESampleCursorChunk(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MESampleCursorChunk")
        }
    }
}
impl IMESampleCursorChunk for MESampleCursorChunk {}
pub trait IMESampleCursorChunk: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithByteSource_chunkStorageRange_chunkInfo_sampleIndexWithinChunk_(
        &self,
        byteSource: MEByteSource,
        chunkStorageRange: AVSampleCursorStorageRange,
        chunkInfo: AVSampleCursorChunkInfo,
        sampleIndexWithinChunk: CFIndex,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithByteSource : byteSource, chunkStorageRange : chunkStorageRange, chunkInfo : chunkInfo, sampleIndexWithinChunk : sampleIndexWithinChunk)
    }
    unsafe fn byteSource(&self) -> MEByteSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, byteSource)
    }
    unsafe fn chunkStorageRange(&self) -> AVSampleCursorStorageRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chunkStorageRange)
    }
    unsafe fn chunkInfo(&self) -> AVSampleCursorChunkInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chunkInfo)
    }
    unsafe fn sampleIndexWithinChunk(&self) -> CFIndex
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleIndexWithinChunk)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MESampleCursorChunk").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MESampleLocation(pub id);
impl std::ops::Deref for MESampleLocation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MESampleLocation {}
impl MESampleLocation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MESampleLocation").unwrap(), alloc) })
    }
}
impl PNSCopying for MESampleLocation {}
impl INSObject for MESampleLocation {}
impl PNSObject for MESampleLocation {}
impl std::convert::TryFrom<NSObject> for MESampleLocation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MESampleLocation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MESampleLocation").unwrap()) };
        if is_kind_of {
            Ok(MESampleLocation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MESampleLocation")
        }
    }
}
impl IMESampleLocation for MESampleLocation {}
pub trait IMESampleLocation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithByteSource_sampleLocation_(
        &self,
        byteSource: MEByteSource,
        sampleLocation: AVSampleCursorStorageRange,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithByteSource : byteSource, sampleLocation : sampleLocation)
    }
    unsafe fn sampleLocation(&self) -> AVSampleCursorStorageRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleLocation)
    }
    unsafe fn byteSource(&self) -> MEByteSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, byteSource)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MESampleLocation").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEEstimatedSampleLocation(pub id);
impl std::ops::Deref for MEEstimatedSampleLocation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEEstimatedSampleLocation {}
impl MEEstimatedSampleLocation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEEstimatedSampleLocation").unwrap(), alloc) })
    }
}
impl PNSCopying for MEEstimatedSampleLocation {}
impl INSObject for MEEstimatedSampleLocation {}
impl PNSObject for MEEstimatedSampleLocation {}
impl std::convert::TryFrom<NSObject> for MEEstimatedSampleLocation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEEstimatedSampleLocation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEEstimatedSampleLocation").unwrap()) };
        if is_kind_of {
            Ok(MEEstimatedSampleLocation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEEstimatedSampleLocation")
        }
    }
}
impl IMEEstimatedSampleLocation for MEEstimatedSampleLocation {}
pub trait IMEEstimatedSampleLocation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithByteSource_estimatedSampleLocation_refinementDataLocation_(
        &self,
        byteSource: MEByteSource,
        estimatedSampleLocation: AVSampleCursorStorageRange,
        refinementDataLocation: AVSampleCursorStorageRange,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithByteSource : byteSource, estimatedSampleLocation : estimatedSampleLocation, refinementDataLocation : refinementDataLocation)
    }
    unsafe fn estimatedSampleLocation(&self) -> AVSampleCursorStorageRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, estimatedSampleLocation)
    }
    unsafe fn refinementDataLocation(&self) -> AVSampleCursorStorageRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refinementDataLocation)
    }
    unsafe fn byteSource(&self) -> MEByteSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, byteSource)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEEstimatedSampleLocation").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEHEVCDependencyInfo(pub id);
impl std::ops::Deref for MEHEVCDependencyInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEHEVCDependencyInfo {}
impl MEHEVCDependencyInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEHEVCDependencyInfo").unwrap(), alloc) })
    }
}
impl PNSCopying for MEHEVCDependencyInfo {}
impl INSObject for MEHEVCDependencyInfo {}
impl PNSObject for MEHEVCDependencyInfo {}
impl std::convert::TryFrom<NSObject> for MEHEVCDependencyInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEHEVCDependencyInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEHEVCDependencyInfo").unwrap()) };
        if is_kind_of {
            Ok(MEHEVCDependencyInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEHEVCDependencyInfo")
        }
    }
}
impl IMEHEVCDependencyInfo for MEHEVCDependencyInfo {}
pub trait IMEHEVCDependencyInfo: Sized + std::ops::Deref {
    unsafe fn hasTemporalSubLayerAccess(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasTemporalSubLayerAccess)
    }
    unsafe fn setTemporalSubLayerAccess_(&self, temporalSubLayerAccess: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTemporalSubLayerAccess : temporalSubLayerAccess)
    }
    unsafe fn hasStepwiseTemporalSubLayerAccess(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasStepwiseTemporalSubLayerAccess)
    }
    unsafe fn setStepwiseTemporalSubLayerAccess_(&self, stepwiseTemporalSubLayerAccess: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStepwiseTemporalSubLayerAccess : stepwiseTemporalSubLayerAccess)
    }
    unsafe fn syncSampleNALUnitType(&self) -> i16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, syncSampleNALUnitType)
    }
    unsafe fn setSyncSampleNALUnitType_(&self, syncSampleNALUnitType: i16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSyncSampleNALUnitType : syncSampleNALUnitType)
    }
}
impl MEHEVCDependencyInfo_HEVCTemporalLevelInfo for MEHEVCDependencyInfo {}
pub trait MEHEVCDependencyInfo_HEVCTemporalLevelInfo: Sized + std::ops::Deref {
    unsafe fn temporalLevel(&self) -> i16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, temporalLevel)
    }
    unsafe fn setTemporalLevel_(&self, temporalLevel: i16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTemporalLevel : temporalLevel)
    }
    unsafe fn profileSpace(&self) -> i16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileSpace)
    }
    unsafe fn setProfileSpace_(&self, profileSpace: i16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfileSpace : profileSpace)
    }
    unsafe fn tierFlag(&self) -> i16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tierFlag)
    }
    unsafe fn setTierFlag_(&self, tierFlag: i16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTierFlag : tierFlag)
    }
    unsafe fn profileIndex(&self) -> i16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileIndex)
    }
    unsafe fn setProfileIndex_(&self, profileIndex: i16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfileIndex : profileIndex)
    }
    unsafe fn profileCompatibilityFlags(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileCompatibilityFlags)
    }
    unsafe fn setProfileCompatibilityFlags_(&self, profileCompatibilityFlags: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfileCompatibilityFlags : profileCompatibilityFlags)
    }
    unsafe fn constraintIndicatorFlags(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constraintIndicatorFlags)
    }
    unsafe fn setConstraintIndicatorFlags_(&self, constraintIndicatorFlags: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstraintIndicatorFlags : constraintIndicatorFlags)
    }
    unsafe fn levelIndex(&self) -> i16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, levelIndex)
    }
    unsafe fn setLevelIndex_(&self, levelIndex: i16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevelIndex : levelIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEByteSource(pub id);
impl std::ops::Deref for MEByteSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEByteSource {}
impl MEByteSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEByteSource").unwrap(), alloc) })
    }
}
impl INSObject for MEByteSource {}
impl PNSObject for MEByteSource {}
impl std::convert::TryFrom<NSObject> for MEByteSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEByteSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEByteSource").unwrap()) };
        if is_kind_of {
            Ok(MEByteSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEByteSource")
        }
    }
}
impl IMEByteSource for MEByteSource {}
pub trait IMEByteSource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn readDataOfLength_fromOffset_toDestination_completionHandler_(
        &self,
        length: usize,
        offset: i64,
        dest: *mut ::std::os::raw::c_void,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readDataOfLength : length, fromOffset : offset, toDestination : dest, completionHandler : completionHandler)
    }
    unsafe fn readDataOfLength_fromOffset_completionHandler_(
        &self,
        length: usize,
        offset: i64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readDataOfLength : length, fromOffset : offset, completionHandler : completionHandler)
    }
    unsafe fn readDataOfLength_fromOffset_toDestination_bytesRead_error_(
        &self,
        length: usize,
        offset: i64,
        dest: *mut ::std::os::raw::c_void,
        bytesReadOut: *mut usize,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readDataOfLength : length, fromOffset : offset, toDestination : dest, bytesRead : bytesReadOut, error : error)
    }
    unsafe fn availableLengthAtOffset_(&self, offset: i64) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, availableLengthAtOffset : offset)
    }
    unsafe fn byteSourceForRelatedFileName_error_(
        &self,
        fileName: NSString,
        errorOut: *mut NSError,
    ) -> MEByteSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, byteSourceForRelatedFileName : fileName, error : errorOut)
    }
    unsafe fn fileName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileName)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn fileLength(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileLength)
    }
    unsafe fn relatedFileNamesInSameDirectory(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relatedFileNamesInSameDirectory)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEByteSource").unwrap(), new)
    }
}
pub trait PMEVideoDecoderExtension: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn videoDecoderWithCodecType_videoFormatDescription_videoDecoderSpecifications_extensionDecoderPixelBufferManager_error_(
        &self,
        codecType: CMVideoCodecType,
        videoFormatDescription: CMVideoFormatDescriptionRef,
        videoDecoderSpecifications: NSDictionary,
        extensionDecoderPixelBufferManager: MEVideoDecoderPixelBufferManager,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, videoDecoderWithCodecType : codecType, videoFormatDescription : videoFormatDescription, videoDecoderSpecifications : videoDecoderSpecifications, extensionDecoderPixelBufferManager : extensionDecoderPixelBufferManager, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEVideoDecoderPixelBufferManager(pub id);
impl std::ops::Deref for MEVideoDecoderPixelBufferManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEVideoDecoderPixelBufferManager {}
impl MEVideoDecoderPixelBufferManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEVideoDecoderPixelBufferManager").unwrap(), alloc) })
    }
}
impl INSObject for MEVideoDecoderPixelBufferManager {}
impl PNSObject for MEVideoDecoderPixelBufferManager {}
impl std::convert::TryFrom<NSObject> for MEVideoDecoderPixelBufferManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEVideoDecoderPixelBufferManager, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEVideoDecoderPixelBufferManager").unwrap())
        };
        if is_kind_of {
            Ok(MEVideoDecoderPixelBufferManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEVideoDecoderPixelBufferManager")
        }
    }
}
impl IMEVideoDecoderPixelBufferManager for MEVideoDecoderPixelBufferManager {}
pub trait IMEVideoDecoderPixelBufferManager: Sized + std::ops::Deref {
    unsafe fn createPixelBufferAndReturnError_(&self, error: *mut NSError) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createPixelBufferAndReturnError : error)
    }
    unsafe fn registerCustomPixelFormat_(&self, customPixelFormat: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerCustomPixelFormat : customPixelFormat)
    }
    unsafe fn pixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBufferAttributes)
    }
    unsafe fn setPixelBufferAttributes_(&self, pixelBufferAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelBufferAttributes : pixelBufferAttributes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEDecodeFrameOptions(pub id);
impl std::ops::Deref for MEDecodeFrameOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEDecodeFrameOptions {}
impl MEDecodeFrameOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEDecodeFrameOptions").unwrap(), alloc) })
    }
}
impl INSObject for MEDecodeFrameOptions {}
impl PNSObject for MEDecodeFrameOptions {}
impl std::convert::TryFrom<NSObject> for MEDecodeFrameOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEDecodeFrameOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEDecodeFrameOptions").unwrap()) };
        if is_kind_of {
            Ok(MEDecodeFrameOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEDecodeFrameOptions")
        }
    }
}
impl IMEDecodeFrameOptions for MEDecodeFrameOptions {}
pub trait IMEDecodeFrameOptions: Sized + std::ops::Deref {
    unsafe fn doNotOutputFrame(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doNotOutputFrame)
    }
    unsafe fn setDoNotOutputFrame_(&self, doNotOutputFrame: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoNotOutputFrame : doNotOutputFrame)
    }
    unsafe fn realTimePlayback(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, realTimePlayback)
    }
    unsafe fn setRealTimePlayback_(&self, realTimePlayback: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRealTimePlayback : realTimePlayback)
    }
}
pub type MEDecodeFrameStatus = NSUInteger;
pub trait PMEVideoDecoder: Sized + std::ops::Deref {
    unsafe fn decodeFrameFromSampleBuffer_options_completionHandler_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        options: MEDecodeFrameOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, decodeFrameFromSampleBuffer : sampleBuffer, options : options, completionHandler : completionHandler)
    }
    unsafe fn canAcceptFormatDescription_(&self, formatDescription: CMFormatDescriptionRef) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canAcceptFormatDescription : formatDescription)
    }
    unsafe fn producesRAWOutput(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, producesRAWOutput)
    }
    unsafe fn contentHasInterframeDependencies(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentHasInterframeDependencies)
    }
    unsafe fn recommendedThreadCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recommendedThreadCount)
    }
    unsafe fn setRecommendedThreadCount_(&self, recommendedThreadCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecommendedThreadCount : recommendedThreadCount)
    }
    unsafe fn actualThreadCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actualThreadCount)
    }
    unsafe fn supportedPixelFormatsOrderedByQuality(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedPixelFormatsOrderedByQuality)
    }
    unsafe fn reducedResolution(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reducedResolution)
    }
    unsafe fn setReducedResolution_(&self, reducedResolution: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReducedResolution : reducedResolution)
    }
    unsafe fn pixelFormatsWithReducedResolutionDecodeSupport(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormatsWithReducedResolutionDecodeSupport)
    }
    unsafe fn isReadyForMoreMediaData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadyForMoreMediaData)
    }
}
pub trait PMERAWProcessorExtension: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn processorWithFormatDescription_extensionPixelBufferManager_error_(
        &self,
        formatDescription: CMVideoFormatDescriptionRef,
        extensionPixelBufferManager: MERAWProcessorPixelBufferManager,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processorWithFormatDescription : formatDescription, extensionPixelBufferManager : extensionPixelBufferManager, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MERAWProcessorPixelBufferManager(pub id);
impl std::ops::Deref for MERAWProcessorPixelBufferManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MERAWProcessorPixelBufferManager {}
impl MERAWProcessorPixelBufferManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MERAWProcessorPixelBufferManager").unwrap(), alloc) })
    }
}
impl INSObject for MERAWProcessorPixelBufferManager {}
impl PNSObject for MERAWProcessorPixelBufferManager {}
impl std::convert::TryFrom<NSObject> for MERAWProcessorPixelBufferManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MERAWProcessorPixelBufferManager, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MERAWProcessorPixelBufferManager").unwrap())
        };
        if is_kind_of {
            Ok(MERAWProcessorPixelBufferManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MERAWProcessorPixelBufferManager")
        }
    }
}
impl IMERAWProcessorPixelBufferManager for MERAWProcessorPixelBufferManager {}
pub trait IMERAWProcessorPixelBufferManager: Sized + std::ops::Deref {
    unsafe fn createPixelBufferAndReturnError_(&self, error: *mut NSError) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createPixelBufferAndReturnError : error)
    }
    unsafe fn pixelBufferAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBufferAttributes)
    }
    unsafe fn setPixelBufferAttributes_(&self, pixelBufferAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelBufferAttributes : pixelBufferAttributes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MERAWProcessingParameter(pub id);
impl std::ops::Deref for MERAWProcessingParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MERAWProcessingParameter {}
impl MERAWProcessingParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MERAWProcessingParameter").unwrap(), alloc) })
    }
}
impl INSObject for MERAWProcessingParameter {}
impl PNSObject for MERAWProcessingParameter {}
impl std::convert::TryFrom<NSObject> for MERAWProcessingParameter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MERAWProcessingParameter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MERAWProcessingParameter").unwrap()) };
        if is_kind_of {
            Ok(MERAWProcessingParameter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MERAWProcessingParameter")
        }
    }
}
impl IMERAWProcessingParameter for MERAWProcessingParameter {}
pub trait IMERAWProcessingParameter: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn key(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, key)
    }
    unsafe fn longDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, longDescription)
    }
    unsafe fn enabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MERAWProcessingListElementParameter(pub id);
impl std::ops::Deref for MERAWProcessingListElementParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MERAWProcessingListElementParameter {}
impl MERAWProcessingListElementParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MERAWProcessingListElementParameter").unwrap(), alloc) })
    }
}
impl IMERAWProcessingParameter for MERAWProcessingListElementParameter {}
impl From<MERAWProcessingListElementParameter> for MERAWProcessingParameter {
    fn from(child: MERAWProcessingListElementParameter) -> MERAWProcessingParameter {
        MERAWProcessingParameter(child.0)
    }
}
impl std::convert::TryFrom<MERAWProcessingParameter> for MERAWProcessingListElementParameter {
    type Error = &'static str;
    fn try_from(
        parent: MERAWProcessingParameter,
    ) -> Result<MERAWProcessingListElementParameter, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MERAWProcessingListElementParameter").unwrap())
        };
        if is_kind_of {
            Ok(MERAWProcessingListElementParameter(parent.0))
        } else {
            Err ("This MERAWProcessingParameter cannot be downcasted to MERAWProcessingListElementParameter" ,)
        }
    }
}
impl INSObject for MERAWProcessingListElementParameter {}
impl PNSObject for MERAWProcessingListElementParameter {}
impl IMERAWProcessingListElementParameter for MERAWProcessingListElementParameter {}
pub trait IMERAWProcessingListElementParameter: Sized + std::ops::Deref {
    unsafe fn initWithName_description_elementID_(
        &self,
        name: NSString,
        description: NSString,
        elementID: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, description : description, elementID : elementID)
    }
    unsafe fn listElementID(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listElementID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MERAWProcessingBooleanParameter(pub id);
impl std::ops::Deref for MERAWProcessingBooleanParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MERAWProcessingBooleanParameter {}
impl MERAWProcessingBooleanParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MERAWProcessingBooleanParameter").unwrap(), alloc) })
    }
}
impl IMERAWProcessingParameter for MERAWProcessingBooleanParameter {}
impl std::convert::TryFrom<MERAWProcessingParameter> for MERAWProcessingBooleanParameter {
    type Error = &'static str;
    fn try_from(
        parent: MERAWProcessingParameter,
    ) -> Result<MERAWProcessingBooleanParameter, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MERAWProcessingBooleanParameter").unwrap())
        };
        if is_kind_of {
            Ok(MERAWProcessingBooleanParameter(parent.0))
        } else {
            Err ("This MERAWProcessingParameter cannot be downcasted to MERAWProcessingBooleanParameter" ,)
        }
    }
}
impl INSObject for MERAWProcessingBooleanParameter {}
impl PNSObject for MERAWProcessingBooleanParameter {}
impl IMERAWProcessingBooleanParameter for MERAWProcessingBooleanParameter {}
pub trait IMERAWProcessingBooleanParameter: Sized + std::ops::Deref {
    unsafe fn initWithName_key_description_initialValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue)
    }
    unsafe fn initWithName_key_description_initialValue_neutralValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: BOOL,
        neutralValue: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, neutralValue : neutralValue)
    }
    unsafe fn initWithName_key_description_initialValue_cameraValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: BOOL,
        cameraValue: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, cameraValue : cameraValue)
    }
    unsafe fn initWithName_key_description_initialValue_neutralValue_cameraValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: BOOL,
        neutralValue: BOOL,
        cameraValue: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, neutralValue : neutralValue, cameraValue : cameraValue)
    }
    unsafe fn hasNeutralValue_(&self, outNeutralValue: *mut BOOL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasNeutralValue : outNeutralValue)
    }
    unsafe fn hasCameraValue_(&self, outCameraValue: *mut BOOL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasCameraValue : outCameraValue)
    }
    unsafe fn initialValue(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialValue)
    }
    unsafe fn currentValue(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentValue)
    }
    unsafe fn setCurrentValue_(&self, currentValue: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentValue : currentValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MERAWProcessingIntegerParameter(pub id);
impl std::ops::Deref for MERAWProcessingIntegerParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MERAWProcessingIntegerParameter {}
impl MERAWProcessingIntegerParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MERAWProcessingIntegerParameter").unwrap(), alloc) })
    }
}
impl IMERAWProcessingParameter for MERAWProcessingIntegerParameter {}
impl std::convert::TryFrom<MERAWProcessingParameter> for MERAWProcessingIntegerParameter {
    type Error = &'static str;
    fn try_from(
        parent: MERAWProcessingParameter,
    ) -> Result<MERAWProcessingIntegerParameter, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MERAWProcessingIntegerParameter").unwrap())
        };
        if is_kind_of {
            Ok(MERAWProcessingIntegerParameter(parent.0))
        } else {
            Err ("This MERAWProcessingParameter cannot be downcasted to MERAWProcessingIntegerParameter" ,)
        }
    }
}
impl INSObject for MERAWProcessingIntegerParameter {}
impl PNSObject for MERAWProcessingIntegerParameter {}
impl IMERAWProcessingIntegerParameter for MERAWProcessingIntegerParameter {}
pub trait IMERAWProcessingIntegerParameter: Sized + std::ops::Deref {
    unsafe fn initWithName_key_description_initialValue_maximum_minimum_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: NSInteger,
        maximum: NSInteger,
        minimum: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, maximum : maximum, minimum : minimum)
    }
    unsafe fn initWithName_key_description_initialValue_maximum_minimum_neutralValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: NSInteger,
        maximum: NSInteger,
        minimum: NSInteger,
        neutralValue: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, maximum : maximum, minimum : minimum, neutralValue : neutralValue)
    }
    unsafe fn initWithName_key_description_initialValue_maximum_minimum_cameraValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: NSInteger,
        maximum: NSInteger,
        minimum: NSInteger,
        cameraValue: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, maximum : maximum, minimum : minimum, cameraValue : cameraValue)
    }
    unsafe fn initWithName_key_description_initialValue_maximum_minimum_neutralValue_cameraValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: NSInteger,
        maximum: NSInteger,
        minimum: NSInteger,
        neutralValue: NSInteger,
        cameraValue: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, maximum : maximum, minimum : minimum, neutralValue : neutralValue, cameraValue : cameraValue)
    }
    unsafe fn hasNeutralValue_(&self, outNeutralValue: *mut NSInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasNeutralValue : outNeutralValue)
    }
    unsafe fn hasCameraValue_(&self, outCameraValue: *mut NSInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasCameraValue : outCameraValue)
    }
    unsafe fn maximumValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumValue)
    }
    unsafe fn minimumValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumValue)
    }
    unsafe fn initialValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialValue)
    }
    unsafe fn currentValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentValue)
    }
    unsafe fn setCurrentValue_(&self, currentValue: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentValue : currentValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MERAWProcessingFloatParameter(pub id);
impl std::ops::Deref for MERAWProcessingFloatParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MERAWProcessingFloatParameter {}
impl MERAWProcessingFloatParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MERAWProcessingFloatParameter").unwrap(), alloc) })
    }
}
impl IMERAWProcessingParameter for MERAWProcessingFloatParameter {}
impl std::convert::TryFrom<MERAWProcessingParameter> for MERAWProcessingFloatParameter {
    type Error = &'static str;
    fn try_from(
        parent: MERAWProcessingParameter,
    ) -> Result<MERAWProcessingFloatParameter, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MERAWProcessingFloatParameter").unwrap())
        };
        if is_kind_of {
            Ok(MERAWProcessingFloatParameter(parent.0))
        } else {
            Err ("This MERAWProcessingParameter cannot be downcasted to MERAWProcessingFloatParameter" ,)
        }
    }
}
impl INSObject for MERAWProcessingFloatParameter {}
impl PNSObject for MERAWProcessingFloatParameter {}
impl IMERAWProcessingFloatParameter for MERAWProcessingFloatParameter {}
pub trait IMERAWProcessingFloatParameter: Sized + std::ops::Deref {
    unsafe fn initWithName_key_description_initialValue_maximum_minimum_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: f32,
        maximum: f32,
        minimum: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, maximum : maximum, minimum : minimum)
    }
    unsafe fn initWithName_key_description_initialValue_maximum_minimum_neutralValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: f32,
        maximum: f32,
        minimum: f32,
        neutralValue: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, maximum : maximum, minimum : minimum, neutralValue : neutralValue)
    }
    unsafe fn initWithName_key_description_initialValue_maximum_minimum_cameraValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: f32,
        maximum: f32,
        minimum: f32,
        cameraValue: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, maximum : maximum, minimum : minimum, cameraValue : cameraValue)
    }
    unsafe fn initWithName_key_description_initialValue_maximum_minimum_neutralValue_cameraValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        initialValue: f32,
        maximum: f32,
        minimum: f32,
        neutralValue: f32,
        cameraValue: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, initialValue : initialValue, maximum : maximum, minimum : minimum, neutralValue : neutralValue, cameraValue : cameraValue)
    }
    unsafe fn hasNeutralValue_(&self, outNeutralValue: *mut f32) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasNeutralValue : outNeutralValue)
    }
    unsafe fn hasCameraValue_(&self, outCameraValue: *mut f32) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasCameraValue : outCameraValue)
    }
    unsafe fn maximumValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumValue)
    }
    unsafe fn minimumValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumValue)
    }
    unsafe fn initialValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialValue)
    }
    unsafe fn currentValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentValue)
    }
    unsafe fn setCurrentValue_(&self, currentValue: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentValue : currentValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MERAWProcessingListParameter(pub id);
impl std::ops::Deref for MERAWProcessingListParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MERAWProcessingListParameter {}
impl MERAWProcessingListParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MERAWProcessingListParameter").unwrap(), alloc) })
    }
}
impl IMERAWProcessingParameter for MERAWProcessingListParameter {}
impl std::convert::TryFrom<MERAWProcessingParameter> for MERAWProcessingListParameter {
    type Error = &'static str;
    fn try_from(
        parent: MERAWProcessingParameter,
    ) -> Result<MERAWProcessingListParameter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MERAWProcessingListParameter").unwrap()) };
        if is_kind_of {
            Ok(MERAWProcessingListParameter(parent.0))
        } else {
            Err ("This MERAWProcessingParameter cannot be downcasted to MERAWProcessingListParameter" ,)
        }
    }
}
impl INSObject for MERAWProcessingListParameter {}
impl PNSObject for MERAWProcessingListParameter {}
impl IMERAWProcessingListParameter for MERAWProcessingListParameter {}
pub trait IMERAWProcessingListParameter: Sized + std::ops::Deref {
    unsafe fn initWithName_key_description_list_initialValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        listElements: NSArray,
        initialValue: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, list : listElements, initialValue : initialValue)
    }
    unsafe fn initWithName_key_description_list_initialValue_neutralValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        listElements: NSArray,
        initialValue: NSInteger,
        neutralValue: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, list : listElements, initialValue : initialValue, neutralValue : neutralValue)
    }
    unsafe fn initWithName_key_description_list_initialValue_cameraValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        listElements: NSArray,
        initialValue: NSInteger,
        cameraValue: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, list : listElements, initialValue : initialValue, cameraValue : cameraValue)
    }
    unsafe fn initWithName_key_description_list_initialValue_neutralValue_cameraValue_(
        &self,
        name: NSString,
        key: NSString,
        description: NSString,
        listElements: NSArray,
        initialValue: NSInteger,
        neutralValue: NSInteger,
        cameraValue: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, key : key, description : description, list : listElements, initialValue : initialValue, neutralValue : neutralValue, cameraValue : cameraValue)
    }
    unsafe fn hasNeutralValue_(&self, outNeutralValue: *mut NSInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasNeutralValue : outNeutralValue)
    }
    unsafe fn hasCameraValue_(&self, outCameraValue: *mut NSInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasCameraValue : outCameraValue)
    }
    unsafe fn listElements(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listElements)
    }
    unsafe fn initialValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialValue)
    }
    unsafe fn currentValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentValue)
    }
    unsafe fn setCurrentValue_(&self, currentValue: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentValue : currentValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MERAWProcessingSubGroupParameter(pub id);
impl std::ops::Deref for MERAWProcessingSubGroupParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MERAWProcessingSubGroupParameter {}
impl MERAWProcessingSubGroupParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MERAWProcessingSubGroupParameter").unwrap(), alloc) })
    }
}
impl IMERAWProcessingParameter for MERAWProcessingSubGroupParameter {}
impl std::convert::TryFrom<MERAWProcessingParameter> for MERAWProcessingSubGroupParameter {
    type Error = &'static str;
    fn try_from(
        parent: MERAWProcessingParameter,
    ) -> Result<MERAWProcessingSubGroupParameter, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MERAWProcessingSubGroupParameter").unwrap())
        };
        if is_kind_of {
            Ok(MERAWProcessingSubGroupParameter(parent.0))
        } else {
            Err ("This MERAWProcessingParameter cannot be downcasted to MERAWProcessingSubGroupParameter" ,)
        }
    }
}
impl INSObject for MERAWProcessingSubGroupParameter {}
impl PNSObject for MERAWProcessingSubGroupParameter {}
impl IMERAWProcessingSubGroupParameter for MERAWProcessingSubGroupParameter {}
pub trait IMERAWProcessingSubGroupParameter: Sized + std::ops::Deref {
    unsafe fn initWithName_description_parameters_(
        &self,
        name: NSString,
        description: NSString,
        parameters: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, description : description, parameters : parameters)
    }
    unsafe fn subGroupParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subGroupParameters)
    }
}
pub trait PMERAWProcessor: Sized + std::ops::Deref {
    unsafe fn processFrameFromImageBuffer_completionHandler_(
        &self,
        inputFrame: CVPixelBufferRef,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processFrameFromImageBuffer : inputFrame, completionHandler : completionHandler)
    }
    unsafe fn metalDeviceRegistryID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalDeviceRegistryID)
    }
    unsafe fn setMetalDeviceRegistryID_(&self, metalDeviceRegistryID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMetalDeviceRegistryID : metalDeviceRegistryID)
    }
    unsafe fn outputColorAttachments(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputColorAttachments)
    }
    unsafe fn metadataForSidecarFile(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadataForSidecarFile)
    }
    unsafe fn processingParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, processingParameters)
    }
    unsafe fn isReadyForMoreMediaData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadyForMoreMediaData)
    }
}
unsafe extern "C" {
    pub static MediaExtensionErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MEVideoDecoderReadyForMoreMediaDataDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MERAWProcessorValuesDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MERAWProcessorReadyForMoreMediaDataDidChangeNotification: NSNotificationName;
}

unsafe impl objc2::encode::RefEncode for MEFormatReaderInstantiationOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEFormatReaderInstantiationOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEFileInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEFileInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for METrackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for METrackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MESampleCursorChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MESampleCursorChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MESampleLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MESampleLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEEstimatedSampleLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEEstimatedSampleLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEHEVCDependencyInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEHEVCDependencyInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEByteSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEByteSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEVideoDecoderPixelBufferManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEVideoDecoderPixelBufferManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEDecodeFrameOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEDecodeFrameOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MERAWProcessorPixelBufferManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MERAWProcessorPixelBufferManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MERAWProcessingParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MERAWProcessingParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MERAWProcessingListElementParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MERAWProcessingListElementParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MERAWProcessingBooleanParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MERAWProcessingBooleanParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MERAWProcessingIntegerParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MERAWProcessingIntegerParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MERAWProcessingFloatParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MERAWProcessingFloatParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MERAWProcessingListParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MERAWProcessingListParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MERAWProcessingSubGroupParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MERAWProcessingSubGroupParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
