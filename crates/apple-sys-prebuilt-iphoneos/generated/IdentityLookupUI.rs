#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::IdentityLookup::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILClassificationUIExtensionContext(pub id);
impl std::ops::Deref for ILClassificationUIExtensionContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILClassificationUIExtensionContext {}
impl ILClassificationUIExtensionContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILClassificationUIExtensionContext").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ILClassificationUIExtensionContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILClassificationUIExtensionContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILClassificationUIExtensionContext").unwrap())
        };
        if is_kind_of {
            Ok(ILClassificationUIExtensionContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILClassificationUIExtensionContext")
        }
    }
}
impl IILClassificationUIExtensionContext for ILClassificationUIExtensionContext {}
pub trait IILClassificationUIExtensionContext: Sized + std::ops::Deref {
    unsafe fn isReadyForClassificationResponse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadyForClassificationResponse)
    }
    unsafe fn setReadyForClassificationResponse_(&self, readyForClassificationResponse: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadyForClassificationResponse : readyForClassificationResponse)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILClassificationUIExtensionViewController(pub id);
impl std::ops::Deref for ILClassificationUIExtensionViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILClassificationUIExtensionViewController {}
impl ILClassificationUIExtensionViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILClassificationUIExtensionViewController").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for ILClassificationUIExtensionViewController {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<ILClassificationUIExtensionViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILClassificationUIExtensionViewController").unwrap())
        };
        if is_kind_of {
            Ok(ILClassificationUIExtensionViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILClassificationUIExtensionViewController")
        }
    }
}
impl IILClassificationUIExtensionViewController for ILClassificationUIExtensionViewController {}
pub trait IILClassificationUIExtensionViewController: Sized + std::ops::Deref {
    unsafe fn prepareForClassificationRequest_(&self, request: ILClassificationRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareForClassificationRequest : request)
    }
    unsafe fn classificationResponseForRequest_(
        &self,
        request: ILClassificationRequest,
    ) -> ILClassificationResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, classificationResponseForRequest : request)
    }
    unsafe fn extensionContext(&self) -> ILClassificationUIExtensionContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionContext)
    }
}
pub trait PUIAppearanceContainer: Sized + std::ops::Deref {}
impl PUIAppearanceContainer for ILClassificationUIExtensionViewController {}

unsafe impl objc2::encode::RefEncode for ILClassificationUIExtensionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILClassificationUIExtensionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILClassificationUIExtensionViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILClassificationUIExtensionViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
