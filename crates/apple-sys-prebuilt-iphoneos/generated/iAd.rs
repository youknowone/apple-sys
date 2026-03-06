#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ADClientError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ADClient(pub id);
impl std::ops::Deref for ADClient {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ADClient {}
impl ADClient {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ADClient").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ADClient {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ADClient, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ADClient").unwrap()) };
        if is_kind_of {
            Ok(ADClient(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ADClient")
        }
    }
}
impl IADClient for ADClient {}
pub trait IADClient: Sized + std::ops::Deref {
    unsafe fn requestAttributionDetailsWithBlock_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAttributionDetailsWithBlock : completionHandler)
    }
    unsafe fn sharedClient() -> ADClient
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ADClient").unwrap(), sharedClient)
    }
}

unsafe impl objc2::encode::RefEncode for ADClient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ADClient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
