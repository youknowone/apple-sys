#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreLocation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type APActivationPayloadErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct APActivationPayload(pub id);
impl std::ops::Deref for APActivationPayload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for APActivationPayload {}
impl APActivationPayload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"APActivationPayload").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for APActivationPayload {}
impl PNSCopying for APActivationPayload {}
impl INSObject for APActivationPayload {}
impl PNSObject for APActivationPayload {}
impl std::convert::TryFrom<NSObject> for APActivationPayload {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<APActivationPayload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"APActivationPayload").unwrap()) };
        if is_kind_of {
            Ok(APActivationPayload(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to APActivationPayload")
        }
    }
}
impl IAPActivationPayload for APActivationPayload {}
pub trait IAPActivationPayload: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn confirmAcquiredInRegion_completionHandler_(
        &self,
        region: CLRegion,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, confirmAcquiredInRegion : region, completionHandler : completionHandler)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"APActivationPayload").unwrap(), new)
    }
}
pub trait NSUserActivity_AppClip: Sized + std::ops::Deref {
    unsafe fn appClipActivationPayload(&self) -> APActivationPayload
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appClipActivationPayload)
    }
}
unsafe extern "C" {
    pub static APActivationPayloadErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for APActivationPayload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for APActivationPayload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
