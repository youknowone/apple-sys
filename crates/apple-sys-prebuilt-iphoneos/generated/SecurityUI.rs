#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFCertificatePresentation(pub id);
impl std::ops::Deref for SFCertificatePresentation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFCertificatePresentation {}
impl SFCertificatePresentation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFCertificatePresentation").unwrap(), alloc) })
    }
}
impl INSObject for SFCertificatePresentation {}
impl PNSObject for SFCertificatePresentation {}
impl std::convert::TryFrom<NSObject> for SFCertificatePresentation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFCertificatePresentation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFCertificatePresentation").unwrap()) };
        if is_kind_of {
            Ok(SFCertificatePresentation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFCertificatePresentation")
        }
    }
}
impl ISFCertificatePresentation for SFCertificatePresentation {}
pub trait ISFCertificatePresentation: Sized + std::ops::Deref {
    unsafe fn initWithTrust_(&self, trust: SecTrustRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTrust : trust)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn presentSheetInWindow_dismissHandler_(
        &self,
        window: NSWindow,
        dismissHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentSheetInWindow : window, dismissHandler : dismissHandler)
    }
    unsafe fn dismissSheet(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dismissSheet)
    }
    unsafe fn trust(&self) -> SecTrustRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trust)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : title)
    }
    unsafe fn message(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn helpURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, helpURL)
    }
    unsafe fn setHelpURL_(&self, helpURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHelpURL : helpURL)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFCertificatePresentation").unwrap(), new)
    }
}

unsafe impl objc2::encode::RefEncode for SFCertificatePresentation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFCertificatePresentation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
