#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AEAssessmentApplication(pub id);
impl std::ops::Deref for AEAssessmentApplication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AEAssessmentApplication {}
impl AEAssessmentApplication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AEAssessmentApplication").unwrap(), alloc) })
    }
}
impl PNSCopying for AEAssessmentApplication {}
impl INSObject for AEAssessmentApplication {}
impl PNSObject for AEAssessmentApplication {}
impl std::convert::TryFrom<NSObject> for AEAssessmentApplication {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AEAssessmentApplication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AEAssessmentApplication").unwrap()) };
        if is_kind_of {
            Ok(AEAssessmentApplication(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AEAssessmentApplication")
        }
    }
}
impl IAEAssessmentApplication for AEAssessmentApplication {}
pub trait IAEAssessmentApplication: Sized + std::ops::Deref {
    unsafe fn initWithBundleIdentifier_(&self, bundleIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBundleIdentifier : bundleIdentifier)
    }
    unsafe fn initWithBundleIdentifier_teamIdentifier_(
        &self,
        bundleIdentifier: NSString,
        teamIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBundleIdentifier : bundleIdentifier, teamIdentifier : teamIdentifier)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
    unsafe fn teamIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, teamIdentifier)
    }
    unsafe fn requiresSignatureValidation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresSignatureValidation)
    }
    unsafe fn setRequiresSignatureValidation_(&self, requiresSignatureValidation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresSignatureValidation : requiresSignatureValidation)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AEAssessmentApplication").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AEAssessmentConfiguration(pub id);
impl std::ops::Deref for AEAssessmentConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AEAssessmentConfiguration {}
impl AEAssessmentConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AEAssessmentConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for AEAssessmentConfiguration {}
impl INSObject for AEAssessmentConfiguration {}
impl PNSObject for AEAssessmentConfiguration {}
impl std::convert::TryFrom<NSObject> for AEAssessmentConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AEAssessmentConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AEAssessmentConfiguration").unwrap()) };
        if is_kind_of {
            Ok(AEAssessmentConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AEAssessmentConfiguration")
        }
    }
}
impl IAEAssessmentConfiguration for AEAssessmentConfiguration {}
pub trait IAEAssessmentConfiguration: Sized + std::ops::Deref {
    unsafe fn setConfiguration_forApplication_(
        &self,
        configuration: AEAssessmentParticipantConfiguration,
        application: AEAssessmentApplication,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfiguration : configuration, forApplication : application)
    }
    unsafe fn removeApplication_(&self, application: AEAssessmentApplication)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeApplication : application)
    }
    unsafe fn autocorrectMode(&self) -> AEAutocorrectMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autocorrectMode)
    }
    unsafe fn setAutocorrectMode_(&self, autocorrectMode: AEAutocorrectMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutocorrectMode : autocorrectMode)
    }
    unsafe fn allowsSpellCheck(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsSpellCheck)
    }
    unsafe fn setAllowsSpellCheck_(&self, allowsSpellCheck: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsSpellCheck : allowsSpellCheck)
    }
    unsafe fn allowsPredictiveKeyboard(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsPredictiveKeyboard)
    }
    unsafe fn setAllowsPredictiveKeyboard_(&self, allowsPredictiveKeyboard: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsPredictiveKeyboard : allowsPredictiveKeyboard)
    }
    unsafe fn allowsKeyboardShortcuts(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsKeyboardShortcuts)
    }
    unsafe fn setAllowsKeyboardShortcuts_(&self, allowsKeyboardShortcuts: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsKeyboardShortcuts : allowsKeyboardShortcuts)
    }
    unsafe fn allowsActivityContinuation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsActivityContinuation)
    }
    unsafe fn setAllowsActivityContinuation_(&self, allowsActivityContinuation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsActivityContinuation : allowsActivityContinuation)
    }
    unsafe fn allowsDictation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsDictation)
    }
    unsafe fn setAllowsDictation_(&self, allowsDictation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsDictation : allowsDictation)
    }
    unsafe fn allowsAccessibilityKeyboard(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAccessibilityKeyboard)
    }
    unsafe fn setAllowsAccessibilityKeyboard_(&self, allowsAccessibilityKeyboard: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAccessibilityKeyboard : allowsAccessibilityKeyboard)
    }
    unsafe fn allowsAccessibilityLiveCaptions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAccessibilityLiveCaptions)
    }
    unsafe fn setAllowsAccessibilityLiveCaptions_(&self, allowsAccessibilityLiveCaptions: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAccessibilityLiveCaptions : allowsAccessibilityLiveCaptions)
    }
    unsafe fn allowsAccessibilityReader(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAccessibilityReader)
    }
    unsafe fn setAllowsAccessibilityReader_(&self, allowsAccessibilityReader: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAccessibilityReader : allowsAccessibilityReader)
    }
    unsafe fn allowsAccessibilitySpeech(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAccessibilitySpeech)
    }
    unsafe fn setAllowsAccessibilitySpeech_(&self, allowsAccessibilitySpeech: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAccessibilitySpeech : allowsAccessibilitySpeech)
    }
    unsafe fn allowsAccessibilityTypingFeedback(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAccessibilityTypingFeedback)
    }
    unsafe fn setAllowsAccessibilityTypingFeedback_(&self, allowsAccessibilityTypingFeedback: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAccessibilityTypingFeedback : allowsAccessibilityTypingFeedback)
    }
    unsafe fn allowsPasswordAutoFill(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsPasswordAutoFill)
    }
    unsafe fn setAllowsPasswordAutoFill_(&self, allowsPasswordAutoFill: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsPasswordAutoFill : allowsPasswordAutoFill)
    }
    unsafe fn allowsContinuousPathKeyboard(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsContinuousPathKeyboard)
    }
    unsafe fn setAllowsContinuousPathKeyboard_(&self, allowsContinuousPathKeyboard: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsContinuousPathKeyboard : allowsContinuousPathKeyboard)
    }
    unsafe fn allowsScreenshots(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsScreenshots)
    }
    unsafe fn setAllowsScreenshots_(&self, allowsScreenshots: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsScreenshots : allowsScreenshots)
    }
    unsafe fn mainParticipantConfiguration(&self) -> AEAssessmentParticipantConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainParticipantConfiguration)
    }
    unsafe fn configurationsByApplication(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationsByApplication)
    }
}
pub type AEAutocorrectMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AEAssessmentParticipantConfiguration(pub id);
impl std::ops::Deref for AEAssessmentParticipantConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AEAssessmentParticipantConfiguration {}
impl AEAssessmentParticipantConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AEAssessmentParticipantConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for AEAssessmentParticipantConfiguration {}
impl INSObject for AEAssessmentParticipantConfiguration {}
impl PNSObject for AEAssessmentParticipantConfiguration {}
impl std::convert::TryFrom<NSObject> for AEAssessmentParticipantConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AEAssessmentParticipantConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AEAssessmentParticipantConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(AEAssessmentParticipantConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AEAssessmentParticipantConfiguration")
        }
    }
}
impl IAEAssessmentParticipantConfiguration for AEAssessmentParticipantConfiguration {}
pub trait IAEAssessmentParticipantConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn allowsNetworkAccess(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsNetworkAccess)
    }
    unsafe fn setAllowsNetworkAccess_(&self, allowsNetworkAccess: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsNetworkAccess : allowsNetworkAccess)
    }
    unsafe fn isRequired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRequired)
    }
    unsafe fn setRequired_(&self, required: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequired : required)
    }
    unsafe fn configurationInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationInfo)
    }
    unsafe fn setConfigurationInfo_(&self, configurationInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfigurationInfo : configurationInfo)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AEAssessmentParticipantConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AEAssessmentSession(pub id);
impl std::ops::Deref for AEAssessmentSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AEAssessmentSession {}
impl AEAssessmentSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AEAssessmentSession").unwrap(), alloc) })
    }
}
impl INSObject for AEAssessmentSession {}
impl PNSObject for AEAssessmentSession {}
impl std::convert::TryFrom<NSObject> for AEAssessmentSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AEAssessmentSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AEAssessmentSession").unwrap()) };
        if is_kind_of {
            Ok(AEAssessmentSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AEAssessmentSession")
        }
    }
}
impl IAEAssessmentSession for AEAssessmentSession {}
pub trait IAEAssessmentSession: Sized + std::ops::Deref {
    unsafe fn initWithConfiguration_(
        &self,
        configuration: AEAssessmentConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn begin(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, begin)
    }
    unsafe fn end(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, end)
    }
    unsafe fn updateToConfiguration_(&self, configuration: AEAssessmentConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateToConfiguration : configuration)
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
    unsafe fn configuration(&self) -> AEAssessmentConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AEAssessmentSession").unwrap(), new)
    }
    unsafe fn supportsMultipleParticipants() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AEAssessmentSession").unwrap(), supportsMultipleParticipants)
    }
    unsafe fn supportsConfigurationUpdates() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AEAssessmentSession").unwrap(), supportsConfigurationUpdates)
    }
}
pub trait PAEAssessmentSessionDelegate: Sized + std::ops::Deref {
    unsafe fn assessmentSessionDidBegin_(&self, session: AEAssessmentSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assessmentSessionDidBegin : session)
    }
    unsafe fn assessmentSession_failedToBeginWithError_(
        &self,
        session: AEAssessmentSession,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assessmentSession : session, failedToBeginWithError : error)
    }
    unsafe fn assessmentSession_wasInterruptedWithError_(
        &self,
        session: AEAssessmentSession,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assessmentSession : session, wasInterruptedWithError : error)
    }
    unsafe fn assessmentSessionDidEnd_(&self, session: AEAssessmentSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assessmentSessionDidEnd : session)
    }
    unsafe fn assessmentSessionDidUpdate_(&self, session: AEAssessmentSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assessmentSessionDidUpdate : session)
    }
    unsafe fn assessmentSession_failedToUpdateToConfiguration_error_(
        &self,
        session: AEAssessmentSession,
        configuration: AEAssessmentConfiguration,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assessmentSession : session, failedToUpdateToConfiguration : configuration, error : error)
    }
}
pub type AEAssessmentErrorCode = NSInteger;
unsafe extern "C" {
    pub static AEAssessmentErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static mut AENotInstalledParticipantsKey: NSString;
}
unsafe extern "C" {
    pub static mut AERestrictedSystemParticipantsKey: NSString;
}

unsafe impl objc2::encode::RefEncode for AEAssessmentApplication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AEAssessmentApplication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AEAssessmentConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AEAssessmentConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AEAssessmentParticipantConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AEAssessmentParticipantConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AEAssessmentSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AEAssessmentSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
