#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SMAppServiceStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SMAppService(pub id);
impl std::ops::Deref for SMAppService {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SMAppService {}
impl SMAppService {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SMAppService").unwrap(), alloc) })
    }
}
impl INSObject for SMAppService {}
impl PNSObject for SMAppService {}
impl std::convert::TryFrom<NSObject> for SMAppService {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SMAppService, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SMAppService").unwrap()) };
        if is_kind_of {
            Ok(SMAppService(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SMAppService")
        }
    }
}
impl ISMAppService for SMAppService {}
pub trait ISMAppService: Sized + std::ops::Deref {
    unsafe fn registerAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerAndReturnError : error)
    }
    unsafe fn unregisterAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterAndReturnError : error)
    }
    unsafe fn unregisterWithCompletionHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterWithCompletionHandler : handler)
    }
    unsafe fn status(&self) -> SMAppServiceStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn loginItemServiceWithIdentifier_(identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SMAppService").unwrap(), loginItemServiceWithIdentifier : identifier)
    }
    unsafe fn agentServiceWithPlistName_(plistName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SMAppService").unwrap(), agentServiceWithPlistName : plistName)
    }
    unsafe fn daemonServiceWithPlistName_(plistName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SMAppService").unwrap(), daemonServiceWithPlistName : plistName)
    }
    unsafe fn statusForLegacyURL_(url: NSURL) -> SMAppServiceStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SMAppService").unwrap(), statusForLegacyURL : url)
    }
    unsafe fn openSystemSettingsLoginItems()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SMAppService").unwrap(), openSystemSettingsLoginItems)
    }
    unsafe fn mainAppService() -> SMAppService
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SMAppService").unwrap(), mainAppService)
    }
}
unsafe extern "C" {
    pub static kSMErrorDomainIPC: CFStringRef;
}
unsafe extern "C" {
    pub static kSMErrorDomainFramework: CFStringRef;
}
unsafe extern "C" {
    pub static kSMErrorDomainLaunchd: CFStringRef;
}
unsafe extern "C" {
    pub static SMAppServiceErrorDomain: NSString;
}
unsafe extern "C" {
    pub fn SMLoginItemSetEnabled(identifier: CFStringRef, enabled: Boolean) -> Boolean;
}
unsafe extern "C" {
    pub static kSMDomainSystemLaunchd: CFStringRef;
}
unsafe extern "C" {
    pub static kSMDomainUserLaunchd: CFStringRef;
}
unsafe extern "C" {
    pub fn SMJobCopyDictionary(domain: CFStringRef, jobLabel: CFStringRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SMCopyAllJobDictionaries(domain: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SMJobSubmit(
        domain: CFStringRef,
        job: CFDictionaryRef,
        auth: AuthorizationRef,
        outError: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SMJobRemove(
        domain: CFStringRef,
        jobLabel: CFStringRef,
        auth: AuthorizationRef,
        wait: Boolean,
        outError: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SMJobBless(
        domain: CFStringRef,
        executableLabel: CFStringRef,
        auth: AuthorizationRef,
        outError: *mut CFErrorRef,
    ) -> Boolean;
}

unsafe impl objc2::encode::RefEncode for SMAppService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SMAppService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
