#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SAAuthorizationStatus = NSInteger;
pub type SACrashDetectionEventResponse = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SACrashDetectionEvent(pub id);
impl std::ops::Deref for SACrashDetectionEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SACrashDetectionEvent {}
impl SACrashDetectionEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SACrashDetectionEvent").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SACrashDetectionEvent {}
impl PNSCopying for SACrashDetectionEvent {}
impl INSObject for SACrashDetectionEvent {}
impl PNSObject for SACrashDetectionEvent {}
impl std::convert::TryFrom<NSObject> for SACrashDetectionEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SACrashDetectionEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SACrashDetectionEvent").unwrap()) };
        if is_kind_of {
            Ok(SACrashDetectionEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SACrashDetectionEvent")
        }
    }
}
impl ISACrashDetectionEvent for SACrashDetectionEvent {}
pub trait ISACrashDetectionEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn response(&self) -> SACrashDetectionEventResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, response)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SACrashDetectionEvent").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SACrashDetectionManager(pub id);
impl std::ops::Deref for SACrashDetectionManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SACrashDetectionManager {}
impl SACrashDetectionManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SACrashDetectionManager").unwrap(), alloc) })
    }
}
impl INSObject for SACrashDetectionManager {}
impl PNSObject for SACrashDetectionManager {}
impl std::convert::TryFrom<NSObject> for SACrashDetectionManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SACrashDetectionManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SACrashDetectionManager").unwrap()) };
        if is_kind_of {
            Ok(SACrashDetectionManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SACrashDetectionManager")
        }
    }
}
impl ISACrashDetectionManager for SACrashDetectionManager {}
pub trait ISACrashDetectionManager: Sized + std::ops::Deref {
    unsafe fn requestAuthorizationWithCompletionHandler_(
        &self,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAuthorizationWithCompletionHandler : handler)
    }
    unsafe fn authorizationStatus(&self) -> SAAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationStatus)
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
    unsafe fn isAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SACrashDetectionManager").unwrap(), isAvailable)
    }
}
pub trait PSACrashDetectionDelegate: Sized + std::ops::Deref {
    unsafe fn crashDetectionManager_didDetectEvent_(
        &self,
        crashDetectionManager: SACrashDetectionManager,
        event: SACrashDetectionEvent,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, crashDetectionManager : crashDetectionManager, didDetectEvent : event)
    }
}
pub type SAEmergencyResponseManagerVoiceCallStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SAEmergencyResponseManager(pub id);
impl std::ops::Deref for SAEmergencyResponseManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SAEmergencyResponseManager {}
impl SAEmergencyResponseManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SAEmergencyResponseManager").unwrap(), alloc) })
    }
}
impl INSObject for SAEmergencyResponseManager {}
impl PNSObject for SAEmergencyResponseManager {}
impl std::convert::TryFrom<NSObject> for SAEmergencyResponseManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SAEmergencyResponseManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SAEmergencyResponseManager").unwrap()) };
        if is_kind_of {
            Ok(SAEmergencyResponseManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SAEmergencyResponseManager")
        }
    }
}
impl ISAEmergencyResponseManager for SAEmergencyResponseManager {}
pub trait ISAEmergencyResponseManager: Sized + std::ops::Deref {
    unsafe fn dialVoiceCallToPhoneNumber_completionHandler_(
        &self,
        phoneNumber: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dialVoiceCallToPhoneNumber : phoneNumber, completionHandler : handler)
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
}
pub trait PSAEmergencyResponseDelegate: Sized + std::ops::Deref {
    unsafe fn emergencyResponseManager_didUpdateVoiceCallStatus_(
        &self,
        emergencyResponseManager: SAEmergencyResponseManager,
        voiceCallStatus: SAEmergencyResponseManagerVoiceCallStatus,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, emergencyResponseManager : emergencyResponseManager, didUpdateVoiceCallStatus : voiceCallStatus)
    }
}
pub type SAErrorCode = NSInteger;
unsafe extern "C" {
    pub static mut SAErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for SACrashDetectionEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SACrashDetectionEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SACrashDetectionManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SACrashDetectionManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SAEmergencyResponseManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SAEmergencyResponseManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
