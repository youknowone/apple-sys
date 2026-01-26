#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SFSpeechErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSpeechLanguageModelConfiguration(pub id);
impl std::ops::Deref for SFSpeechLanguageModelConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSpeechLanguageModelConfiguration {}
impl SFSpeechLanguageModelConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechLanguageModelConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for SFSpeechLanguageModelConfiguration {}
impl PNSSecureCoding for SFSpeechLanguageModelConfiguration {}
impl INSObject for SFSpeechLanguageModelConfiguration {}
impl PNSObject for SFSpeechLanguageModelConfiguration {}
impl std::convert::TryFrom<NSObject> for SFSpeechLanguageModelConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSpeechLanguageModelConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSpeechLanguageModelConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(SFSpeechLanguageModelConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSpeechLanguageModelConfiguration")
        }
    }
}
impl ISFSpeechLanguageModelConfiguration for SFSpeechLanguageModelConfiguration {}
pub trait ISFSpeechLanguageModelConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithLanguageModel_(&self, languageModel: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLanguageModel : languageModel)
    }
    unsafe fn initWithLanguageModel_vocabulary_(
        &self,
        languageModel: NSURL,
        vocabulary: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLanguageModel : languageModel, vocabulary : vocabulary)
    }
    unsafe fn initWithLanguageModel_vocabulary_weight_(
        &self,
        languageModel: NSURL,
        vocabulary: NSURL,
        weight: NSNumber,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLanguageModel : languageModel, vocabulary : vocabulary, weight : weight)
    }
    unsafe fn languageModel(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageModel)
    }
    unsafe fn vocabulary(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vocabulary)
    }
    unsafe fn weight(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weight)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSpeechLanguageModel(pub id);
impl std::ops::Deref for SFSpeechLanguageModel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSpeechLanguageModel {}
impl SFSpeechLanguageModel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechLanguageModel").unwrap(), alloc) })
    }
}
impl INSObject for SFSpeechLanguageModel {}
impl PNSObject for SFSpeechLanguageModel {}
impl std::convert::TryFrom<NSObject> for SFSpeechLanguageModel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSpeechLanguageModel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSpeechLanguageModel").unwrap()) };
        if is_kind_of {
            Ok(SFSpeechLanguageModel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSpeechLanguageModel")
        }
    }
}
impl ISFSpeechLanguageModel for SFSpeechLanguageModel {}
pub trait ISFSpeechLanguageModel: Sized + std::ops::Deref {
    unsafe fn prepareCustomLanguageModelForUrl_clientIdentifier_configuration_completion_(
        asset: NSURL,
        clientIdentifier: NSString,
        configuration: SFSpeechLanguageModelConfiguration,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechLanguageModel").unwrap(), prepareCustomLanguageModelForUrl : asset, clientIdentifier : clientIdentifier, configuration : configuration, completion : completion)
    }
    unsafe fn prepareCustomLanguageModelForUrl_clientIdentifier_configuration_ignoresCache_completion_(
        asset: NSURL,
        clientIdentifier: NSString,
        configuration: SFSpeechLanguageModelConfiguration,
        ignoresCache: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechLanguageModel").unwrap(), prepareCustomLanguageModelForUrl : asset, clientIdentifier : clientIdentifier, configuration : configuration, ignoresCache : ignoresCache, completion : completion)
    }
    unsafe fn prepareCustomLanguageModelForUrl_configuration_completion_(
        asset: NSURL,
        configuration: SFSpeechLanguageModelConfiguration,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechLanguageModel").unwrap(), prepareCustomLanguageModelForUrl : asset, configuration : configuration, completion : completion)
    }
    unsafe fn prepareCustomLanguageModelForUrl_configuration_ignoresCache_completion_(
        asset: NSURL,
        configuration: SFSpeechLanguageModelConfiguration,
        ignoresCache: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechLanguageModel").unwrap(), prepareCustomLanguageModelForUrl : asset, configuration : configuration, ignoresCache : ignoresCache, completion : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSpeechRecognitionMetadata(pub id);
impl std::ops::Deref for SFSpeechRecognitionMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSpeechRecognitionMetadata {}
impl SFSpeechRecognitionMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechRecognitionMetadata").unwrap(), alloc) })
    }
}
impl PNSCopying for SFSpeechRecognitionMetadata {}
impl PNSSecureCoding for SFSpeechRecognitionMetadata {}
impl INSObject for SFSpeechRecognitionMetadata {}
impl PNSObject for SFSpeechRecognitionMetadata {}
impl std::convert::TryFrom<NSObject> for SFSpeechRecognitionMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSpeechRecognitionMetadata, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSpeechRecognitionMetadata").unwrap()) };
        if is_kind_of {
            Ok(SFSpeechRecognitionMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSpeechRecognitionMetadata")
        }
    }
}
impl ISFSpeechRecognitionMetadata for SFSpeechRecognitionMetadata {}
pub trait ISFSpeechRecognitionMetadata: Sized + std::ops::Deref {
    unsafe fn speakingRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speakingRate)
    }
    unsafe fn averagePauseDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averagePauseDuration)
    }
    unsafe fn speechStartTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speechStartTimestamp)
    }
    unsafe fn speechDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speechDuration)
    }
    unsafe fn voiceAnalytics(&self) -> SFVoiceAnalytics
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voiceAnalytics)
    }
}
pub type SFSpeechRecognitionTaskHint = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSpeechRecognitionRequest(pub id);
impl std::ops::Deref for SFSpeechRecognitionRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSpeechRecognitionRequest {}
impl SFSpeechRecognitionRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechRecognitionRequest").unwrap(), alloc) })
    }
}
impl INSObject for SFSpeechRecognitionRequest {}
impl PNSObject for SFSpeechRecognitionRequest {}
impl std::convert::TryFrom<NSObject> for SFSpeechRecognitionRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSpeechRecognitionRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSpeechRecognitionRequest").unwrap()) };
        if is_kind_of {
            Ok(SFSpeechRecognitionRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSpeechRecognitionRequest")
        }
    }
}
impl ISFSpeechRecognitionRequest for SFSpeechRecognitionRequest {}
pub trait ISFSpeechRecognitionRequest: Sized + std::ops::Deref {
    unsafe fn taskHint(&self) -> SFSpeechRecognitionTaskHint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, taskHint)
    }
    unsafe fn setTaskHint_(&self, taskHint: SFSpeechRecognitionTaskHint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTaskHint : taskHint)
    }
    unsafe fn shouldReportPartialResults(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldReportPartialResults)
    }
    unsafe fn setShouldReportPartialResults_(&self, shouldReportPartialResults: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldReportPartialResults : shouldReportPartialResults)
    }
    unsafe fn contextualStrings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contextualStrings)
    }
    unsafe fn setContextualStrings_(&self, contextualStrings: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContextualStrings : contextualStrings)
    }
    unsafe fn interactionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interactionIdentifier)
    }
    unsafe fn setInteractionIdentifier_(&self, interactionIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteractionIdentifier : interactionIdentifier)
    }
    unsafe fn requiresOnDeviceRecognition(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresOnDeviceRecognition)
    }
    unsafe fn setRequiresOnDeviceRecognition_(&self, requiresOnDeviceRecognition: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresOnDeviceRecognition : requiresOnDeviceRecognition)
    }
    unsafe fn addsPunctuation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addsPunctuation)
    }
    unsafe fn setAddsPunctuation_(&self, addsPunctuation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddsPunctuation : addsPunctuation)
    }
    unsafe fn customizedLanguageModel(&self) -> SFSpeechLanguageModelConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customizedLanguageModel)
    }
    unsafe fn setCustomizedLanguageModel_(
        &self,
        customizedLanguageModel: SFSpeechLanguageModelConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomizedLanguageModel : customizedLanguageModel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSpeechURLRecognitionRequest(pub id);
impl std::ops::Deref for SFSpeechURLRecognitionRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSpeechURLRecognitionRequest {}
impl SFSpeechURLRecognitionRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechURLRecognitionRequest").unwrap(), alloc) })
    }
}
impl ISFSpeechRecognitionRequest for SFSpeechURLRecognitionRequest {}
impl From<SFSpeechURLRecognitionRequest> for SFSpeechRecognitionRequest {
    fn from(child: SFSpeechURLRecognitionRequest) -> SFSpeechRecognitionRequest {
        SFSpeechRecognitionRequest(child.0)
    }
}
impl std::convert::TryFrom<SFSpeechRecognitionRequest> for SFSpeechURLRecognitionRequest {
    type Error = &'static str;
    fn try_from(
        parent: SFSpeechRecognitionRequest,
    ) -> Result<SFSpeechURLRecognitionRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSpeechURLRecognitionRequest").unwrap())
        };
        if is_kind_of {
            Ok(SFSpeechURLRecognitionRequest(parent.0))
        } else {
            Err ("This SFSpeechRecognitionRequest cannot be downcasted to SFSpeechURLRecognitionRequest" ,)
        }
    }
}
impl INSObject for SFSpeechURLRecognitionRequest {}
impl PNSObject for SFSpeechURLRecognitionRequest {}
impl ISFSpeechURLRecognitionRequest for SFSpeechURLRecognitionRequest {}
pub trait ISFSpeechURLRecognitionRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithURL_(&self, URL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSpeechAudioBufferRecognitionRequest(pub id);
impl std::ops::Deref for SFSpeechAudioBufferRecognitionRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSpeechAudioBufferRecognitionRequest {}
impl SFSpeechAudioBufferRecognitionRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechAudioBufferRecognitionRequest").unwrap(), alloc) })
    }
}
impl ISFSpeechRecognitionRequest for SFSpeechAudioBufferRecognitionRequest {}
impl std::convert::TryFrom<SFSpeechRecognitionRequest> for SFSpeechAudioBufferRecognitionRequest {
    type Error = &'static str;
    fn try_from(
        parent: SFSpeechRecognitionRequest,
    ) -> Result<SFSpeechAudioBufferRecognitionRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSpeechAudioBufferRecognitionRequest").unwrap())
        };
        if is_kind_of {
            Ok(SFSpeechAudioBufferRecognitionRequest(parent.0))
        } else {
            Err ("This SFSpeechRecognitionRequest cannot be downcasted to SFSpeechAudioBufferRecognitionRequest" ,)
        }
    }
}
impl INSObject for SFSpeechAudioBufferRecognitionRequest {}
impl PNSObject for SFSpeechAudioBufferRecognitionRequest {}
impl ISFSpeechAudioBufferRecognitionRequest for SFSpeechAudioBufferRecognitionRequest {}
pub trait ISFSpeechAudioBufferRecognitionRequest: Sized + std::ops::Deref {
    unsafe fn appendAudioPCMBuffer_(&self, audioPCMBuffer: AVAudioPCMBuffer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendAudioPCMBuffer : audioPCMBuffer)
    }
    unsafe fn appendAudioSampleBuffer_(&self, sampleBuffer: CMSampleBufferRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendAudioSampleBuffer : sampleBuffer)
    }
    unsafe fn endAudio(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endAudio)
    }
    unsafe fn nativeAudioFormat(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nativeAudioFormat)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSpeechRecognitionResult(pub id);
impl std::ops::Deref for SFSpeechRecognitionResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSpeechRecognitionResult {}
impl SFSpeechRecognitionResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechRecognitionResult").unwrap(), alloc) })
    }
}
impl PNSCopying for SFSpeechRecognitionResult {}
impl PNSSecureCoding for SFSpeechRecognitionResult {}
impl INSObject for SFSpeechRecognitionResult {}
impl PNSObject for SFSpeechRecognitionResult {}
impl std::convert::TryFrom<NSObject> for SFSpeechRecognitionResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSpeechRecognitionResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSpeechRecognitionResult").unwrap()) };
        if is_kind_of {
            Ok(SFSpeechRecognitionResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSpeechRecognitionResult")
        }
    }
}
impl ISFSpeechRecognitionResult for SFSpeechRecognitionResult {}
pub trait ISFSpeechRecognitionResult: Sized + std::ops::Deref {
    unsafe fn bestTranscription(&self) -> SFTranscription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bestTranscription)
    }
    unsafe fn transcriptions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transcriptions)
    }
    unsafe fn isFinal(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFinal)
    }
    unsafe fn speechRecognitionMetadata(&self) -> SFSpeechRecognitionMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speechRecognitionMetadata)
    }
}
pub type SFSpeechRecognitionTaskState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSpeechRecognitionTask(pub id);
impl std::ops::Deref for SFSpeechRecognitionTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSpeechRecognitionTask {}
impl SFSpeechRecognitionTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechRecognitionTask").unwrap(), alloc) })
    }
}
impl INSObject for SFSpeechRecognitionTask {}
impl PNSObject for SFSpeechRecognitionTask {}
impl std::convert::TryFrom<NSObject> for SFSpeechRecognitionTask {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSpeechRecognitionTask, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSpeechRecognitionTask").unwrap()) };
        if is_kind_of {
            Ok(SFSpeechRecognitionTask(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSpeechRecognitionTask")
        }
    }
}
impl ISFSpeechRecognitionTask for SFSpeechRecognitionTask {}
pub trait ISFSpeechRecognitionTask: Sized + std::ops::Deref {
    unsafe fn finish(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finish)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn state(&self) -> SFSpeechRecognitionTaskState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn isFinishing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFinishing)
    }
    unsafe fn isCancelled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCancelled)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
}
pub trait PSFSpeechRecognitionTaskDelegate: Sized + std::ops::Deref {
    unsafe fn speechRecognitionDidDetectSpeech_(&self, task: SFSpeechRecognitionTask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechRecognitionDidDetectSpeech : task)
    }
    unsafe fn speechRecognitionTask_didHypothesizeTranscription_(
        &self,
        task: SFSpeechRecognitionTask,
        transcription: SFTranscription,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechRecognitionTask : task, didHypothesizeTranscription : transcription)
    }
    unsafe fn speechRecognitionTask_didFinishRecognition_(
        &self,
        task: SFSpeechRecognitionTask,
        recognitionResult: SFSpeechRecognitionResult,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechRecognitionTask : task, didFinishRecognition : recognitionResult)
    }
    unsafe fn speechRecognitionTaskFinishedReadingAudio_(&self, task: SFSpeechRecognitionTask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechRecognitionTaskFinishedReadingAudio : task)
    }
    unsafe fn speechRecognitionTaskWasCancelled_(&self, task: SFSpeechRecognitionTask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechRecognitionTaskWasCancelled : task)
    }
    unsafe fn speechRecognitionTask_didFinishSuccessfully_(
        &self,
        task: SFSpeechRecognitionTask,
        successfully: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechRecognitionTask : task, didFinishSuccessfully : successfully)
    }
    unsafe fn speechRecognitionTask_didProcessAudioDuration_(
        &self,
        task: SFSpeechRecognitionTask,
        duration: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechRecognitionTask : task, didProcessAudioDuration : duration)
    }
}
pub type SFSpeechRecognizerAuthorizationStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSpeechRecognizer(pub id);
impl std::ops::Deref for SFSpeechRecognizer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSpeechRecognizer {}
impl SFSpeechRecognizer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechRecognizer").unwrap(), alloc) })
    }
}
impl INSObject for SFSpeechRecognizer {}
impl PNSObject for SFSpeechRecognizer {}
impl std::convert::TryFrom<NSObject> for SFSpeechRecognizer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSpeechRecognizer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSpeechRecognizer").unwrap()) };
        if is_kind_of {
            Ok(SFSpeechRecognizer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSpeechRecognizer")
        }
    }
}
impl ISFSpeechRecognizer for SFSpeechRecognizer {}
pub trait ISFSpeechRecognizer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLocale_(&self, locale: NSLocale) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocale : locale)
    }
    unsafe fn recognitionTaskWithRequest_resultHandler_(
        &self,
        request: SFSpeechRecognitionRequest,
        resultHandler: *mut ::std::os::raw::c_void,
    ) -> SFSpeechRecognitionTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognitionTaskWithRequest : request, resultHandler : resultHandler)
    }
    unsafe fn recognitionTaskWithRequest_delegate_(
        &self,
        request: SFSpeechRecognitionRequest,
        delegate: *mut u64,
    ) -> SFSpeechRecognitionTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognitionTaskWithRequest : request, delegate : delegate)
    }
    unsafe fn isAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAvailable)
    }
    unsafe fn locale(&self) -> NSLocale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locale)
    }
    unsafe fn supportsOnDeviceRecognition(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsOnDeviceRecognition)
    }
    unsafe fn setSupportsOnDeviceRecognition_(&self, supportsOnDeviceRecognition: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsOnDeviceRecognition : supportsOnDeviceRecognition)
    }
    unsafe fn delegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn defaultTaskHint(&self) -> SFSpeechRecognitionTaskHint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultTaskHint)
    }
    unsafe fn setDefaultTaskHint_(&self, defaultTaskHint: SFSpeechRecognitionTaskHint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultTaskHint : defaultTaskHint)
    }
    unsafe fn queue(&self) -> NSOperationQueue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queue)
    }
    unsafe fn setQueue_(&self, queue: NSOperationQueue)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueue : queue)
    }
    unsafe fn supportedLocales() -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechRecognizer").unwrap(), supportedLocales)
    }
    unsafe fn authorizationStatus() -> SFSpeechRecognizerAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechRecognizer").unwrap(), authorizationStatus)
    }
    unsafe fn requestAuthorization_(handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSpeechRecognizer").unwrap(), requestAuthorization : handler)
    }
}
pub trait PSFSpeechRecognizerDelegate: Sized + std::ops::Deref {
    unsafe fn speechRecognizer_availabilityDidChange_(
        &self,
        speechRecognizer: SFSpeechRecognizer,
        available: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechRecognizer : speechRecognizer, availabilityDidChange : available)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFTranscription(pub id);
impl std::ops::Deref for SFTranscription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFTranscription {}
impl SFTranscription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFTranscription").unwrap(), alloc) })
    }
}
impl PNSCopying for SFTranscription {}
impl PNSSecureCoding for SFTranscription {}
impl INSObject for SFTranscription {}
impl PNSObject for SFTranscription {}
impl std::convert::TryFrom<NSObject> for SFTranscription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFTranscription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFTranscription").unwrap()) };
        if is_kind_of {
            Ok(SFTranscription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFTranscription")
        }
    }
}
impl ISFTranscription for SFTranscription {}
pub trait ISFTranscription: Sized + std::ops::Deref {
    unsafe fn formattedString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formattedString)
    }
    unsafe fn segments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segments)
    }
    unsafe fn speakingRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speakingRate)
    }
    unsafe fn averagePauseDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, averagePauseDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFTranscriptionSegment(pub id);
impl std::ops::Deref for SFTranscriptionSegment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFTranscriptionSegment {}
impl SFTranscriptionSegment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFTranscriptionSegment").unwrap(), alloc) })
    }
}
impl PNSCopying for SFTranscriptionSegment {}
impl PNSSecureCoding for SFTranscriptionSegment {}
impl INSObject for SFTranscriptionSegment {}
impl PNSObject for SFTranscriptionSegment {}
impl std::convert::TryFrom<NSObject> for SFTranscriptionSegment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFTranscriptionSegment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFTranscriptionSegment").unwrap()) };
        if is_kind_of {
            Ok(SFTranscriptionSegment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFTranscriptionSegment")
        }
    }
}
impl ISFTranscriptionSegment for SFTranscriptionSegment {}
pub trait ISFTranscriptionSegment: Sized + std::ops::Deref {
    unsafe fn substring(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, substring)
    }
    unsafe fn substringRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, substringRange)
    }
    unsafe fn timestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn confidence(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
    unsafe fn alternativeSubstrings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternativeSubstrings)
    }
    unsafe fn voiceAnalytics(&self) -> SFVoiceAnalytics
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voiceAnalytics)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFAcousticFeature(pub id);
impl std::ops::Deref for SFAcousticFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFAcousticFeature {}
impl SFAcousticFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFAcousticFeature").unwrap(), alloc) })
    }
}
impl PNSCopying for SFAcousticFeature {}
impl PNSSecureCoding for SFAcousticFeature {}
impl INSObject for SFAcousticFeature {}
impl PNSObject for SFAcousticFeature {}
impl std::convert::TryFrom<NSObject> for SFAcousticFeature {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFAcousticFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFAcousticFeature").unwrap()) };
        if is_kind_of {
            Ok(SFAcousticFeature(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFAcousticFeature")
        }
    }
}
impl ISFAcousticFeature for SFAcousticFeature {}
pub trait ISFAcousticFeature: Sized + std::ops::Deref {
    unsafe fn acousticFeatureValuePerFrame(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acousticFeatureValuePerFrame)
    }
    unsafe fn frameDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFVoiceAnalytics(pub id);
impl std::ops::Deref for SFVoiceAnalytics {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFVoiceAnalytics {}
impl SFVoiceAnalytics {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFVoiceAnalytics").unwrap(), alloc) })
    }
}
impl PNSCopying for SFVoiceAnalytics {}
impl PNSSecureCoding for SFVoiceAnalytics {}
impl INSObject for SFVoiceAnalytics {}
impl PNSObject for SFVoiceAnalytics {}
impl std::convert::TryFrom<NSObject> for SFVoiceAnalytics {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFVoiceAnalytics, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFVoiceAnalytics").unwrap()) };
        if is_kind_of {
            Ok(SFVoiceAnalytics(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFVoiceAnalytics")
        }
    }
}
impl ISFVoiceAnalytics for SFVoiceAnalytics {}
pub trait ISFVoiceAnalytics: Sized + std::ops::Deref {
    unsafe fn jitter(&self) -> SFAcousticFeature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jitter)
    }
    unsafe fn shimmer(&self) -> SFAcousticFeature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shimmer)
    }
    unsafe fn pitch(&self) -> SFAcousticFeature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitch)
    }
    unsafe fn voicing(&self) -> SFAcousticFeature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voicing)
    }
}
unsafe extern "C" {
    pub static SFSpeechErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for SFSpeechLanguageModelConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSpeechLanguageModelConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSpeechLanguageModel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSpeechLanguageModel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSpeechRecognitionMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSpeechRecognitionMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSpeechRecognitionRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSpeechRecognitionRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSpeechURLRecognitionRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSpeechURLRecognitionRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSpeechAudioBufferRecognitionRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSpeechAudioBufferRecognitionRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSpeechRecognitionResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSpeechRecognitionResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSpeechRecognitionTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSpeechRecognitionTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSpeechRecognizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSpeechRecognizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFTranscription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFTranscription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFTranscriptionSegment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFTranscriptionSegment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFAcousticFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFAcousticFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFVoiceAnalytics {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFVoiceAnalytics {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
