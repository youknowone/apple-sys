#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EXHostViewController(pub id);
impl std::ops::Deref for EXHostViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EXHostViewController {}
impl EXHostViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EXHostViewController").unwrap(), alloc) })
    }
}
impl INSViewController for EXHostViewController {}
impl PNSEditor for EXHostViewController {}
impl PNSSeguePerforming for EXHostViewController {}
impl PNSUserInterfaceItemIdentification for EXHostViewController {}
impl std::convert::TryFrom<NSViewController> for EXHostViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<EXHostViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EXHostViewController").unwrap()) };
        if is_kind_of {
            Ok(EXHostViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to EXHostViewController")
        }
    }
}
impl INSResponder for EXHostViewController {}
impl PNSCoding for EXHostViewController {}
impl IEXHostViewController for EXHostViewController {}
pub trait IEXHostViewController: Sized + std::ops::Deref {
    unsafe fn makeXPCConnectionWithError_(&self, error: *mut NSError) -> NSXPCConnection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeXPCConnectionWithError : error)
    }
    unsafe fn delegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn placeholderView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placeholderView)
    }
    unsafe fn setPlaceholderView_(&self, placeholderView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaceholderView : placeholderView)
    }
}
pub trait PEXHostViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn hostViewControllerDidActivate_(&self, viewController: EXHostViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hostViewControllerDidActivate : viewController)
    }
    unsafe fn hostViewControllerWillDeactivate_error_(
        &self,
        viewController: EXHostViewController,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hostViewControllerWillDeactivate : viewController, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EXAppExtensionBrowserViewController(pub id);
impl std::ops::Deref for EXAppExtensionBrowserViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EXAppExtensionBrowserViewController {}
impl EXAppExtensionBrowserViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EXAppExtensionBrowserViewController").unwrap(), alloc) })
    }
}
impl INSViewController for EXAppExtensionBrowserViewController {}
impl PNSEditor for EXAppExtensionBrowserViewController {}
impl PNSSeguePerforming for EXAppExtensionBrowserViewController {}
impl PNSUserInterfaceItemIdentification for EXAppExtensionBrowserViewController {}
impl std::convert::TryFrom<NSViewController> for EXAppExtensionBrowserViewController {
    type Error = &'static str;
    fn try_from(
        parent: NSViewController,
    ) -> Result<EXAppExtensionBrowserViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EXAppExtensionBrowserViewController").unwrap())
        };
        if is_kind_of {
            Ok(EXAppExtensionBrowserViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to EXAppExtensionBrowserViewController")
        }
    }
}
impl INSResponder for EXAppExtensionBrowserViewController {}
impl PNSCoding for EXAppExtensionBrowserViewController {}
impl IEXAppExtensionBrowserViewController for EXAppExtensionBrowserViewController {}
pub trait IEXAppExtensionBrowserViewController: Sized + std::ops::Deref {}

unsafe impl objc2::encode::RefEncode for EXHostViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EXHostViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EXAppExtensionBrowserViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EXAppExtensionBrowserViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
