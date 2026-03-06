#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::VideoToolbox::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCSensitivityAnalysis(pub id);
impl std::ops::Deref for SCSensitivityAnalysis {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCSensitivityAnalysis {}
impl SCSensitivityAnalysis {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCSensitivityAnalysis").unwrap(), alloc) })
    }
}
impl INSObject for SCSensitivityAnalysis {}
impl PNSObject for SCSensitivityAnalysis {}
impl std::convert::TryFrom<NSObject> for SCSensitivityAnalysis {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCSensitivityAnalysis, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCSensitivityAnalysis").unwrap()) };
        if is_kind_of {
            Ok(SCSensitivityAnalysis(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCSensitivityAnalysis")
        }
    }
}
impl ISCSensitivityAnalysis for SCSensitivityAnalysis {}
pub trait ISCSensitivityAnalysis: Sized + std::ops::Deref {
    unsafe fn isSensitive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSensitive)
    }
}
pub type SCSensitivityAnalysisPolicy = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCSensitivityAnalyzer(pub id);
impl std::ops::Deref for SCSensitivityAnalyzer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCSensitivityAnalyzer {}
impl SCSensitivityAnalyzer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCSensitivityAnalyzer").unwrap(), alloc) })
    }
}
impl INSObject for SCSensitivityAnalyzer {}
impl PNSObject for SCSensitivityAnalyzer {}
impl std::convert::TryFrom<NSObject> for SCSensitivityAnalyzer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCSensitivityAnalyzer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCSensitivityAnalyzer").unwrap()) };
        if is_kind_of {
            Ok(SCSensitivityAnalyzer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCSensitivityAnalyzer")
        }
    }
}
impl ISCSensitivityAnalyzer for SCSensitivityAnalyzer {}
pub trait ISCSensitivityAnalyzer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn analyzeImageFile_completionHandler_(
        &self,
        fileURL: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, analyzeImageFile : fileURL, completionHandler : completionHandler)
    }
    unsafe fn analyzeCGImage_completionHandler_(
        &self,
        image: CGImageRef,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, analyzeCGImage : image, completionHandler : completionHandler)
    }
    unsafe fn analyzeVideoFile_completionHandler_(
        &self,
        fileURL: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, analyzeVideoFile : fileURL, completionHandler : completionHandler)
    }
    unsafe fn analysisPolicy(&self) -> SCSensitivityAnalysisPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, analysisPolicy)
    }
}
pub type SCVideoStreamAnalyzerStreamDirection = NSInteger;
pub type SCVideoStreamAnalysisChangeHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SCVideoStreamAnalyzer(pub id);
impl std::ops::Deref for SCVideoStreamAnalyzer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SCVideoStreamAnalyzer {}
impl SCVideoStreamAnalyzer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SCVideoStreamAnalyzer").unwrap(), alloc) })
    }
}
impl INSObject for SCVideoStreamAnalyzer {}
impl PNSObject for SCVideoStreamAnalyzer {}
impl std::convert::TryFrom<NSObject> for SCVideoStreamAnalyzer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SCVideoStreamAnalyzer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SCVideoStreamAnalyzer").unwrap()) };
        if is_kind_of {
            Ok(SCVideoStreamAnalyzer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SCVideoStreamAnalyzer")
        }
    }
}
impl ISCVideoStreamAnalyzer for SCVideoStreamAnalyzer {}
pub trait ISCVideoStreamAnalyzer: Sized + std::ops::Deref {
    unsafe fn initWithParticipantUUID_streamDirection_error_(
        &self,
        participantUUID: NSString,
        streamDirection: SCVideoStreamAnalyzerStreamDirection,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithParticipantUUID : participantUUID, streamDirection : streamDirection, error : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn analyzePixelBuffer_(&self, pixelBuffer: CVPixelBufferRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, analyzePixelBuffer : pixelBuffer)
    }
    unsafe fn analysis(&self) -> SCSensitivityAnalysis
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, analysis)
    }
    unsafe fn analysisChangedHandler(&self) -> SCVideoStreamAnalysisChangeHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, analysisChangedHandler)
    }
    unsafe fn setAnalysisChangedHandler_(
        &self,
        analysisChangedHandler: SCVideoStreamAnalysisChangeHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnalysisChangedHandler : analysisChangedHandler)
    }
}
impl SCVideoStreamAnalyzer_SessionManagement for SCVideoStreamAnalyzer {}
pub trait SCVideoStreamAnalyzer_SessionManagement: Sized + std::ops::Deref {
    unsafe fn beginAnalysisOfDecompressionSession_error_(
        &self,
        decompressionSession: VTDecompressionSessionRef,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginAnalysisOfDecompressionSession : decompressionSession, error : error)
    }
    unsafe fn beginAnalysisOfCaptureDeviceInput_error_(
        &self,
        captureDeviceInput: AVCaptureDeviceInput,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginAnalysisOfCaptureDeviceInput : captureDeviceInput, error : error)
    }
    unsafe fn endAnalysis(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endAnalysis)
    }
    unsafe fn continueStream(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, continueStream)
    }
}
impl SCSensitivityAnalysis_VideoStreamAnalysis for SCSensitivityAnalysis {}
pub trait SCSensitivityAnalysis_VideoStreamAnalysis: Sized + std::ops::Deref {
    unsafe fn shouldInterruptVideo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldInterruptVideo)
    }
    unsafe fn shouldIndicateSensitivity(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldIndicateSensitivity)
    }
    unsafe fn shouldMuteAudio(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldMuteAudio)
    }
}

unsafe impl objc2::encode::RefEncode for SCSensitivityAnalysis {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCSensitivityAnalysis {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCSensitivityAnalyzer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCSensitivityAnalyzer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SCVideoStreamAnalyzer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCVideoStreamAnalyzer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
