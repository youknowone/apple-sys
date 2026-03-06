#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::CoreML::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SNClassifierIdentifier = NSString;
pub type SNErrorCode = NSInteger;
pub trait PSNRequest: Sized + std::ops::Deref {}
pub trait PSNResult: Sized + std::ops::Deref {}
pub trait PSNResultsObserving: Sized + std::ops::Deref {
    unsafe fn request_didProduceResult_(&self, request: *mut u64, result: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, request : request, didProduceResult : result)
    }
    unsafe fn request_didFailWithError_(&self, request: *mut u64, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, request : request, didFailWithError : error)
    }
    unsafe fn requestDidComplete_(&self, request: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDidComplete : request)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SNAudioStreamAnalyzer(pub id);
impl std::ops::Deref for SNAudioStreamAnalyzer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SNAudioStreamAnalyzer {}
impl SNAudioStreamAnalyzer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SNAudioStreamAnalyzer").unwrap(), alloc) })
    }
}
impl INSObject for SNAudioStreamAnalyzer {}
impl PNSObject for SNAudioStreamAnalyzer {}
impl std::convert::TryFrom<NSObject> for SNAudioStreamAnalyzer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SNAudioStreamAnalyzer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SNAudioStreamAnalyzer").unwrap()) };
        if is_kind_of {
            Ok(SNAudioStreamAnalyzer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SNAudioStreamAnalyzer")
        }
    }
}
impl ISNAudioStreamAnalyzer for SNAudioStreamAnalyzer {}
pub trait ISNAudioStreamAnalyzer: Sized + std::ops::Deref {
    unsafe fn initWithFormat_(&self, format: AVAudioFormat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormat : format)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addRequest_withObserver_error_(
        &self,
        request: *mut u64,
        observer: *mut u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRequest : request, withObserver : observer, error : error)
    }
    unsafe fn removeRequest_(&self, request: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRequest : request)
    }
    unsafe fn removeAllRequests(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllRequests)
    }
    unsafe fn analyzeAudioBuffer_atAudioFramePosition_(
        &self,
        audioBuffer: AVAudioBuffer,
        audioFramePosition: AVAudioFramePosition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, analyzeAudioBuffer : audioBuffer, atAudioFramePosition : audioFramePosition)
    }
    unsafe fn completeAnalysis(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completeAnalysis)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SNAudioFileAnalyzer(pub id);
impl std::ops::Deref for SNAudioFileAnalyzer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SNAudioFileAnalyzer {}
impl SNAudioFileAnalyzer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SNAudioFileAnalyzer").unwrap(), alloc) })
    }
}
impl INSObject for SNAudioFileAnalyzer {}
impl PNSObject for SNAudioFileAnalyzer {}
impl std::convert::TryFrom<NSObject> for SNAudioFileAnalyzer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SNAudioFileAnalyzer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SNAudioFileAnalyzer").unwrap()) };
        if is_kind_of {
            Ok(SNAudioFileAnalyzer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SNAudioFileAnalyzer")
        }
    }
}
impl ISNAudioFileAnalyzer for SNAudioFileAnalyzer {}
pub trait ISNAudioFileAnalyzer: Sized + std::ops::Deref {
    unsafe fn initWithURL_error_(&self, url: NSURL, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, error : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addRequest_withObserver_error_(
        &self,
        request: *mut u64,
        observer: *mut u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRequest : request, withObserver : observer, error : error)
    }
    unsafe fn removeRequest_(&self, request: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRequest : request)
    }
    unsafe fn removeAllRequests(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllRequests)
    }
    unsafe fn analyze(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, analyze)
    }
    unsafe fn analyzeWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, analyzeWithCompletionHandler : completionHandler)
    }
    unsafe fn cancelAnalysis(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelAnalysis)
    }
}
pub type SNTimeDurationConstraintType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SNTimeDurationConstraint(pub id);
impl std::ops::Deref for SNTimeDurationConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SNTimeDurationConstraint {}
impl SNTimeDurationConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SNTimeDurationConstraint").unwrap(), alloc) })
    }
}
impl INSObject for SNTimeDurationConstraint {}
impl PNSObject for SNTimeDurationConstraint {}
impl std::convert::TryFrom<NSObject> for SNTimeDurationConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SNTimeDurationConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SNTimeDurationConstraint").unwrap()) };
        if is_kind_of {
            Ok(SNTimeDurationConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SNTimeDurationConstraint")
        }
    }
}
impl ISNTimeDurationConstraint for SNTimeDurationConstraint {}
pub trait ISNTimeDurationConstraint: Sized + std::ops::Deref {
    unsafe fn initWithEnumeratedDurations_(&self, enumeratedDurations: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEnumeratedDurations : enumeratedDurations)
    }
    unsafe fn initWithDurationRange_(&self, durationRange: CMTimeRange) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDurationRange : durationRange)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn type_(&self) -> SNTimeDurationConstraintType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn enumeratedDurations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enumeratedDurations)
    }
    unsafe fn durationRange(&self) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, durationRange)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SNTimeDurationConstraint").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SNClassifySoundRequest(pub id);
impl std::ops::Deref for SNClassifySoundRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SNClassifySoundRequest {}
impl SNClassifySoundRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SNClassifySoundRequest").unwrap(), alloc) })
    }
}
impl PSNRequest for SNClassifySoundRequest {}
impl INSObject for SNClassifySoundRequest {}
impl PNSObject for SNClassifySoundRequest {}
impl std::convert::TryFrom<NSObject> for SNClassifySoundRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SNClassifySoundRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SNClassifySoundRequest").unwrap()) };
        if is_kind_of {
            Ok(SNClassifySoundRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SNClassifySoundRequest")
        }
    }
}
impl ISNClassifySoundRequest for SNClassifySoundRequest {}
pub trait ISNClassifySoundRequest: Sized + std::ops::Deref {
    unsafe fn initWithMLModel_error_(&self, mlModel: MLModel, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMLModel : mlModel, error : error)
    }
    unsafe fn initWithClassifierIdentifier_error_(
        &self,
        classifierIdentifier: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithClassifierIdentifier : classifierIdentifier, error : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn overlapFactor(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overlapFactor)
    }
    unsafe fn setOverlapFactor_(&self, overlapFactor: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverlapFactor : overlapFactor)
    }
    unsafe fn windowDuration(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowDuration)
    }
    unsafe fn setWindowDuration_(&self, windowDuration: CMTime)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWindowDuration : windowDuration)
    }
    unsafe fn windowDurationConstraint(&self) -> SNTimeDurationConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowDurationConstraint)
    }
    unsafe fn knownClassifications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, knownClassifications)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SNClassifySoundRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SNClassification(pub id);
impl std::ops::Deref for SNClassification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SNClassification {}
impl SNClassification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SNClassification").unwrap(), alloc) })
    }
}
impl INSObject for SNClassification {}
impl PNSObject for SNClassification {}
impl std::convert::TryFrom<NSObject> for SNClassification {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SNClassification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SNClassification").unwrap()) };
        if is_kind_of {
            Ok(SNClassification(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SNClassification")
        }
    }
}
impl ISNClassification for SNClassification {}
pub trait ISNClassification: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn confidence(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SNClassification").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SNClassificationResult(pub id);
impl std::ops::Deref for SNClassificationResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SNClassificationResult {}
impl SNClassificationResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SNClassificationResult").unwrap(), alloc) })
    }
}
impl PSNResult for SNClassificationResult {}
impl INSObject for SNClassificationResult {}
impl PNSObject for SNClassificationResult {}
impl std::convert::TryFrom<NSObject> for SNClassificationResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SNClassificationResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SNClassificationResult").unwrap()) };
        if is_kind_of {
            Ok(SNClassificationResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SNClassificationResult")
        }
    }
}
impl ISNClassificationResult for SNClassificationResult {}
pub trait ISNClassificationResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn classificationForIdentifier_(&self, identifier: NSString) -> SNClassification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, classificationForIdentifier : identifier)
    }
    unsafe fn classifications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classifications)
    }
    unsafe fn timeRange(&self) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRange)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SNClassificationResult").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static SNClassifierIdentifierVersion1: SNClassifierIdentifier;
}
unsafe extern "C" {
    pub static SNErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for SNAudioStreamAnalyzer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SNAudioStreamAnalyzer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SNAudioFileAnalyzer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SNAudioFileAnalyzer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SNTimeDurationConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SNTimeDurationConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SNClassifySoundRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SNClassifySoundRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SNClassification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SNClassification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SNClassificationResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SNClassificationResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
