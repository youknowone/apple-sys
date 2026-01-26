#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type AAAttributionErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AAAttribution(pub id);
impl std::ops::Deref for AAAttribution {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AAAttribution {}
impl AAAttribution {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AAAttribution").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for AAAttribution {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AAAttribution, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AAAttribution").unwrap()) };
        if is_kind_of {
            Ok(AAAttribution(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AAAttribution")
        }
    }
}
impl IAAAttribution for AAAttribution {}
pub trait IAAAttribution: Sized + std::ops::Deref {
    unsafe fn attributionTokenWithError_(error: *mut NSError) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AAAttribution").unwrap(), attributionTokenWithError : error)
    }
}
unsafe extern "C" {
    pub static AAAttributionErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for AAAttribution {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AAAttribution {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
