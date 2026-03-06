#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DCDevice(pub id);
impl std::ops::Deref for DCDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DCDevice {}
impl DCDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DCDevice").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for DCDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DCDevice, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DCDevice").unwrap()) };
        if is_kind_of {
            Ok(DCDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DCDevice")
        }
    }
}
impl IDCDevice for DCDevice {}
pub trait IDCDevice: Sized + std::ops::Deref {
    unsafe fn generateTokenWithCompletionHandler_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateTokenWithCompletionHandler : completion)
    }
    unsafe fn isSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSupported)
    }
    unsafe fn currentDevice() -> DCDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DCDevice").unwrap(), currentDevice)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DCAppAttestService(pub id);
impl std::ops::Deref for DCAppAttestService {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DCAppAttestService {}
impl DCAppAttestService {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DCAppAttestService").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for DCAppAttestService {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DCAppAttestService, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DCAppAttestService").unwrap()) };
        if is_kind_of {
            Ok(DCAppAttestService(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DCAppAttestService")
        }
    }
}
impl IDCAppAttestService for DCAppAttestService {}
pub trait IDCAppAttestService: Sized + std::ops::Deref {
    unsafe fn generateKeyWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateKeyWithCompletionHandler : completionHandler)
    }
    unsafe fn attestKey_clientDataHash_completionHandler_(
        &self,
        keyId: NSString,
        clientDataHash: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attestKey : keyId, clientDataHash : clientDataHash, completionHandler : completionHandler)
    }
    unsafe fn generateAssertion_clientDataHash_completionHandler_(
        &self,
        keyId: NSString,
        clientDataHash: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateAssertion : keyId, clientDataHash : clientDataHash, completionHandler : completionHandler)
    }
    unsafe fn isSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSupported)
    }
    unsafe fn sharedService() -> DCAppAttestService
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DCAppAttestService").unwrap(), sharedService)
    }
}
pub type DCError = NSInteger;
unsafe extern "C" {
    pub static DCErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for DCDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DCDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DCAppAttestService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DCAppAttestService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
