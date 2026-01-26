#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ATTrackingManagerAuthorizationStatus = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ATTrackingManager(pub id);
impl std::ops::Deref for ATTrackingManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ATTrackingManager {}
impl ATTrackingManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ATTrackingManager").unwrap(), alloc) })
    }
}
impl INSObject for ATTrackingManager {}
impl PNSObject for ATTrackingManager {}
impl std::convert::TryFrom<NSObject> for ATTrackingManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ATTrackingManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ATTrackingManager").unwrap()) };
        if is_kind_of {
            Ok(ATTrackingManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ATTrackingManager")
        }
    }
}
impl IATTrackingManager for ATTrackingManager {}
pub trait IATTrackingManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn requestTrackingAuthorizationWithCompletionHandler_(
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ATTrackingManager").unwrap(), requestTrackingAuthorizationWithCompletionHandler : completion)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ATTrackingManager").unwrap(), new)
    }
    unsafe fn trackingAuthorizationStatus() -> ATTrackingManagerAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ATTrackingManager").unwrap(), trackingAuthorizationStatus)
    }
}
unsafe extern "C" {
    pub static mut AppTrackingTransparencyVersionNumber: f64;
}
unsafe extern "C" {
    pub static AppTrackingTransparencyVersionString: [::std::os::raw::c_uchar; 0usize];
}

unsafe impl objc2::encode::RefEncode for ATTrackingManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ATTrackingManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
