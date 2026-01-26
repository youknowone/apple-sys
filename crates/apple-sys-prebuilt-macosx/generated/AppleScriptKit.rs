#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASKPluginObject(pub id);
impl std::ops::Deref for ASKPluginObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASKPluginObject {}
impl ASKPluginObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASKPluginObject").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ASKPluginObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASKPluginObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASKPluginObject").unwrap()) };
        if is_kind_of {
            Ok(ASKPluginObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASKPluginObject")
        }
    }
}
impl IASKPluginObject for ASKPluginObject {}
pub trait IASKPluginObject: Sized + std::ops::Deref {
    unsafe fn pluginDidLoad_(bundle: NSBundle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASKPluginObject").unwrap(), pluginDidLoad : bundle)
    }
}

unsafe impl objc2::encode::RefEncode for ASKPluginObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASKPluginObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
