#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSServiceAccount(pub id);
impl std::ops::Deref for MSServiceAccount {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSServiceAccount {}
impl MSServiceAccount {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSServiceAccount").unwrap(), alloc) })
    }
}
impl INSObject for MSServiceAccount {}
impl PNSObject for MSServiceAccount {}
impl std::convert::TryFrom<NSObject> for MSServiceAccount {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MSServiceAccount, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSServiceAccount").unwrap()) };
        if is_kind_of {
            Ok(MSServiceAccount(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MSServiceAccount")
        }
    }
}
impl IMSServiceAccount for MSServiceAccount {}
pub trait IMSServiceAccount: Sized + std::ops::Deref {
    unsafe fn initWithServiceName_accountName_(
        &self,
        serviceName: NSString,
        accountName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceName : serviceName, accountName : accountName)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn serviceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceName)
    }
    unsafe fn accountName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountName)
    }
    unsafe fn clientID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientID)
    }
    unsafe fn setClientID_(&self, clientID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClientID : clientID)
    }
    unsafe fn clientSecret(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientSecret)
    }
    unsafe fn setClientSecret_(&self, clientSecret: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClientSecret : clientSecret)
    }
    unsafe fn configurationURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationURL)
    }
    unsafe fn setConfigurationURL_(&self, configurationURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfigurationURL : configurationURL)
    }
    unsafe fn authorizationTokenURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationTokenURL)
    }
    unsafe fn setAuthorizationTokenURL_(&self, authorizationTokenURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthorizationTokenURL : authorizationTokenURL)
    }
    unsafe fn authorizationScope(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationScope)
    }
    unsafe fn setAuthorizationScope_(&self, authorizationScope: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthorizationScope : authorizationScope)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MSServiceAccount").unwrap(), new)
    }
}
pub type MSPresentationAnchor = UIWindow;
pub trait PMSAuthenticationPresentationContext: Sized + std::ops::Deref {
    unsafe fn presentationAnchor(&self) -> MSPresentationAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationAnchor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSSetupSession(pub id);
impl std::ops::Deref for MSSetupSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSSetupSession {}
impl MSSetupSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSSetupSession").unwrap(), alloc) })
    }
}
impl INSObject for MSSetupSession {}
impl PNSObject for MSSetupSession {}
impl std::convert::TryFrom<NSObject> for MSSetupSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MSSetupSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSSetupSession").unwrap()) };
        if is_kind_of {
            Ok(MSSetupSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MSSetupSession")
        }
    }
}
impl IMSSetupSession for MSSetupSession {}
pub trait IMSSetupSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithServiceAccount_(&self, serviceAccount: MSServiceAccount) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceAccount : serviceAccount)
    }
    unsafe fn startWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWithError : error)
    }
    unsafe fn presentationContext(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationContext)
    }
    unsafe fn setPresentationContext_(&self, presentationContext: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresentationContext : presentationContext)
    }
    unsafe fn account(&self) -> MSServiceAccount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, account)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MSSetupSession").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static mut MediaSetupVersionNumber: f64;
}
unsafe extern "C" {
    pub static MediaSetupVersionString: [::std::os::raw::c_uchar; 0usize];
}

unsafe impl objc2::encode::RefEncode for MSServiceAccount {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSServiceAccount {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSSetupSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSSetupSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
