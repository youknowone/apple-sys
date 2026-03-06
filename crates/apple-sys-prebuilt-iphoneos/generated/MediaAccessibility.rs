#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOSurface::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type MACaptionAppearanceDomain = CFIndex;
pub type MACaptionAppearanceDisplayType = CFIndex;
pub type MACaptionAppearanceBehavior = CFIndex;
pub type MACaptionAppearanceFontStyle = CFIndex;
pub type MACaptionAppearanceTextEdgeStyle = CFIndex;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MAFlashingLightsProcessorResult(pub id);
impl std::ops::Deref for MAFlashingLightsProcessorResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MAFlashingLightsProcessorResult {}
impl MAFlashingLightsProcessorResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MAFlashingLightsProcessorResult").unwrap(), alloc) })
    }
}
impl INSObject for MAFlashingLightsProcessorResult {}
impl PNSObject for MAFlashingLightsProcessorResult {}
impl std::convert::TryFrom<NSObject> for MAFlashingLightsProcessorResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MAFlashingLightsProcessorResult, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MAFlashingLightsProcessorResult").unwrap())
        };
        if is_kind_of {
            Ok(MAFlashingLightsProcessorResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MAFlashingLightsProcessorResult")
        }
    }
}
impl IMAFlashingLightsProcessorResult for MAFlashingLightsProcessorResult {}
pub trait IMAFlashingLightsProcessorResult: Sized + std::ops::Deref {
    unsafe fn surfaceProcessed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, surfaceProcessed)
    }
    unsafe fn mitigationLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mitigationLevel)
    }
    unsafe fn intensityLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensityLevel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MAFlashingLightsProcessor(pub id);
impl std::ops::Deref for MAFlashingLightsProcessor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MAFlashingLightsProcessor {}
impl MAFlashingLightsProcessor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MAFlashingLightsProcessor").unwrap(), alloc) })
    }
}
impl INSObject for MAFlashingLightsProcessor {}
impl PNSObject for MAFlashingLightsProcessor {}
impl std::convert::TryFrom<NSObject> for MAFlashingLightsProcessor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MAFlashingLightsProcessor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MAFlashingLightsProcessor").unwrap()) };
        if is_kind_of {
            Ok(MAFlashingLightsProcessor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MAFlashingLightsProcessor")
        }
    }
}
impl IMAFlashingLightsProcessor for MAFlashingLightsProcessor {}
pub trait IMAFlashingLightsProcessor: Sized + std::ops::Deref {
    unsafe fn canProcessSurface_(&self, surface: IOSurfaceRef) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canProcessSurface : surface)
    }
    unsafe fn processSurface_outSurface_timestamp_options_(
        &self,
        inSurface: IOSurfaceRef,
        outSurface: IOSurfaceRef,
        timestamp: CFAbsoluteTime,
        options: NSDictionary,
    ) -> MAFlashingLightsProcessorResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processSurface : inSurface, outSurface : outSurface, timestamp : timestamp, options : options)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MAMusicHapticsManager(pub id);
impl std::ops::Deref for MAMusicHapticsManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MAMusicHapticsManager {}
impl MAMusicHapticsManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MAMusicHapticsManager").unwrap(), alloc) })
    }
}
impl INSObject for MAMusicHapticsManager {}
impl PNSObject for MAMusicHapticsManager {}
impl std::convert::TryFrom<NSObject> for MAMusicHapticsManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MAMusicHapticsManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MAMusicHapticsManager").unwrap()) };
        if is_kind_of {
            Ok(MAMusicHapticsManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MAMusicHapticsManager")
        }
    }
}
impl IMAMusicHapticsManager for MAMusicHapticsManager {}
pub trait IMAMusicHapticsManager: Sized + std::ops::Deref {
    unsafe fn checkHapticTrackAvailabilityForMediaMatchingCode_completionHandler_(
        &self,
        internationalStandardRecordingCode: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkHapticTrackAvailabilityForMediaMatchingCode : internationalStandardRecordingCode, completionHandler : completionHandler)
    }
    unsafe fn addStatusObserver_(&self, statusHandler: *mut ::std::os::raw::c_void) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addStatusObserver : statusHandler)
    }
    unsafe fn removeStatusObserver_(&self, registrationToken: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeStatusObserver : registrationToken)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MAMusicHapticsManager").unwrap(), new)
    }
    unsafe fn sharedManager() -> MAMusicHapticsManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MAMusicHapticsManager").unwrap(), sharedManager)
    }
}
unsafe extern "C" {
    pub static kMACaptionAppearanceSettingsChangedNotification: CFStringRef;
}
unsafe extern "C" {
    pub static MAMediaCharacteristicDescribesMusicAndSoundForAccessibility: CFStringRef;
}
unsafe extern "C" {
    pub static MAMediaCharacteristicTranscribesSpokenDialogForAccessibility: CFStringRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceDidDisplayCaptions(strings: CFArrayRef);
}
unsafe extern "C" {
    pub fn MACaptionAppearanceAddSelectedLanguage(
        domain: MACaptionAppearanceDomain,
        language: CFStringRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceCopySelectedLanguages(
        domain: MACaptionAppearanceDomain,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceGetDisplayType(
        domain: MACaptionAppearanceDomain,
    ) -> MACaptionAppearanceDisplayType;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceSetDisplayType(
        domain: MACaptionAppearanceDomain,
        displayType: MACaptionAppearanceDisplayType,
    );
}
unsafe extern "C" {
    pub fn MACaptionAppearanceCopyPreferredCaptioningMediaCharacteristics(
        domain: MACaptionAppearanceDomain,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceIsCustomized(domain: MACaptionAppearanceDomain) -> bool;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceCopyForegroundColor(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
    ) -> CGColorRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceCopyBackgroundColor(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
    ) -> CGColorRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceCopyWindowColor(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
    ) -> CGColorRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceGetForegroundOpacity(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
    ) -> CGFloat;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceGetBackgroundOpacity(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
    ) -> CGFloat;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceGetWindowOpacity(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
    ) -> CGFloat;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceGetWindowRoundedCornerRadius(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
    ) -> CGFloat;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceCopyFontDescriptorForStyle(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
        fontStyle: MACaptionAppearanceFontStyle,
    ) -> CTFontDescriptorRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceGetRelativeCharacterSize(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
    ) -> CGFloat;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceGetTextEdgeStyle(
        domain: MACaptionAppearanceDomain,
        behavior: *mut MACaptionAppearanceBehavior,
    ) -> MACaptionAppearanceTextEdgeStyle;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceCopyProfileIDs() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceSetActiveProfileID(profileID: CFStringRef);
}
unsafe extern "C" {
    pub fn MACaptionAppearanceCopyActiveProfileID() -> CFStringRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceCopyProfileName(profileID: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn MACaptionAppearanceExecuteBlockForProfileID(
        profileID: CFStringRef,
        aBlock: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub static kMAAudibleMediaSettingsChangedNotification: CFStringRef;
}
unsafe extern "C" {
    pub static MAMediaCharacteristicDescribesVideoForAccessibility: CFStringRef;
}
unsafe extern "C" {
    pub fn MAAudibleMediaCopyPreferredCharacteristics() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn MAImageCaptioningCopyCaption(url: CFURLRef, error: *mut CFErrorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn MAImageCaptioningSetCaption(
        url: CFURLRef,
        string: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn MAImageCaptioningCopyMetadataTagPath() -> CFStringRef;
}
unsafe extern "C" {
    pub fn MADimFlashingLightsEnabled() -> bool;
}
unsafe extern "C" {
    pub static kMADimFlashingLightsChangedNotification: CFStringRef;
}
unsafe extern "C" {
    pub static MAMusicHapticsManagerActiveStatusDidChangeNotification: NSNotificationName;
}

unsafe impl objc2::encode::RefEncode for MAFlashingLightsProcessorResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MAFlashingLightsProcessorResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MAFlashingLightsProcessor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MAFlashingLightsProcessor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MAMusicHapticsManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MAMusicHapticsManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
