#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASIdentifierManager(pub id);
impl std::ops::Deref for ASIdentifierManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASIdentifierManager {}
impl ASIdentifierManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASIdentifierManager").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ASIdentifierManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASIdentifierManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASIdentifierManager").unwrap()) };
        if is_kind_of {
            Ok(ASIdentifierManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASIdentifierManager")
        }
    }
}
impl IASIdentifierManager for ASIdentifierManager {}
pub trait IASIdentifierManager: Sized + std::ops::Deref {
    unsafe fn advertisingIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, advertisingIdentifier)
    }
    unsafe fn isAdvertisingTrackingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAdvertisingTrackingEnabled)
    }
    unsafe fn sharedManager() -> ASIdentifierManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASIdentifierManager").unwrap(), sharedManager)
    }
}

unsafe impl objc2::encode::RefEncode for ASIdentifierManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASIdentifierManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
