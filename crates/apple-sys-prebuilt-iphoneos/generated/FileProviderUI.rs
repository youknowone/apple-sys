#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::FileProvider::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type FPUIExtensionErrorCode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FPUIActionExtensionContext(pub id);
impl std::ops::Deref for FPUIActionExtensionContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FPUIActionExtensionContext {}
impl FPUIActionExtensionContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FPUIActionExtensionContext").unwrap(), alloc) })
    }
}
impl INSExtensionContext for FPUIActionExtensionContext {}
impl std::convert::TryFrom<NSExtensionContext> for FPUIActionExtensionContext {
    type Error = &'static str;
    fn try_from(parent: NSExtensionContext) -> Result<FPUIActionExtensionContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FPUIActionExtensionContext").unwrap()) };
        if is_kind_of {
            Ok(FPUIActionExtensionContext(parent.0))
        } else {
            Err("This NSExtensionContext cannot be downcasted to FPUIActionExtensionContext")
        }
    }
}
impl INSObject for FPUIActionExtensionContext {}
impl PNSObject for FPUIActionExtensionContext {}
impl IFPUIActionExtensionContext for FPUIActionExtensionContext {}
pub trait IFPUIActionExtensionContext: Sized + std::ops::Deref {
    unsafe fn completeRequest(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completeRequest)
    }
    unsafe fn completeRequestReturningItems_completionHandler_(
        &self,
        items: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeRequestReturningItems : items, completionHandler : completionHandler)
    }
    unsafe fn cancelRequestWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelRequestWithError : error)
    }
    unsafe fn domainIdentifier(&self) -> NSFileProviderDomainIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FPUIActionExtensionViewController(pub id);
impl std::ops::Deref for FPUIActionExtensionViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FPUIActionExtensionViewController {}
impl FPUIActionExtensionViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FPUIActionExtensionViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for FPUIActionExtensionViewController {}
impl INSObject for FPUIActionExtensionViewController {}
impl PNSObject for FPUIActionExtensionViewController {}
impl std::convert::TryFrom<NSObject> for FPUIActionExtensionViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FPUIActionExtensionViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FPUIActionExtensionViewController").unwrap())
        };
        if is_kind_of {
            Ok(FPUIActionExtensionViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FPUIActionExtensionViewController")
        }
    }
}
impl IFPUIActionExtensionViewController for FPUIActionExtensionViewController {}
pub trait IFPUIActionExtensionViewController: Sized + std::ops::Deref {
    unsafe fn prepareForError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareForError : error)
    }
    unsafe fn prepareForActionWithIdentifier_itemIdentifiers_(
        &self,
        actionIdentifier: NSString,
        itemIdentifiers: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareForActionWithIdentifier : actionIdentifier, itemIdentifiers : itemIdentifiers)
    }
    unsafe fn extensionContext(&self) -> FPUIActionExtensionContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionContext)
    }
}
pub trait PNSEditor: Sized + std::ops::Deref {
    unsafe fn discardEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discardEditing)
    }
    unsafe fn commitEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commitEditing)
    }
    unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo_(
        &self,
        delegate: id,
        didCommitSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingWithDelegate : delegate, didCommitSelector : didCommitSelector, contextInfo : contextInfo)
    }
    unsafe fn commitEditingAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingAndReturnError : error)
    }
}
impl PNSEditor for FPUIActionExtensionViewController {}
unsafe extern "C" {
    pub static FPUIErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for FPUIActionExtensionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FPUIActionExtensionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FPUIActionExtensionViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FPUIActionExtensionViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
