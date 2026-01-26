#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::Contacts::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContactPicker(pub id);
impl std::ops::Deref for CNContactPicker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContactPicker {}
impl CNContactPicker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactPicker").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for CNContactPicker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNContactPicker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContactPicker").unwrap()) };
        if is_kind_of {
            Ok(CNContactPicker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNContactPicker")
        }
    }
}
impl ICNContactPicker for CNContactPicker {}
pub trait ICNContactPicker: Sized + std::ops::Deref {
    unsafe fn showRelativeToRect_ofView_preferredEdge_(
        &self,
        positioningRect: NSRect,
        positioningView: NSView,
        preferredEdge: NSRectEdge,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showRelativeToRect : positioningRect, ofView : positioningView, preferredEdge : preferredEdge)
    }
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn displayedKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayedKeys)
    }
    unsafe fn setDisplayedKeys_(&self, displayedKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayedKeys : displayedKeys)
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
}
pub trait PCNContactPickerDelegate: Sized + std::ops::Deref {
    unsafe fn contactPicker_didSelectContact_(&self, picker: CNContactPicker, contact: CNContact)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contactPicker : picker, didSelectContact : contact)
    }
    unsafe fn contactPicker_didSelectContactProperty_(
        &self,
        picker: CNContactPicker,
        contactProperty: CNContactProperty,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contactPicker : picker, didSelectContactProperty : contactProperty)
    }
    unsafe fn contactPickerWillClose_(&self, picker: CNContactPicker)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contactPickerWillClose : picker)
    }
    unsafe fn contactPickerDidClose_(&self, picker: CNContactPicker)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contactPickerDidClose : picker)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContactViewController(pub id);
impl std::ops::Deref for CNContactViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContactViewController {}
impl CNContactViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactViewController").unwrap(), alloc) })
    }
}
impl INSViewController for CNContactViewController {}
impl PNSEditor for CNContactViewController {}
impl PNSSeguePerforming for CNContactViewController {}
impl PNSUserInterfaceItemIdentification for CNContactViewController {}
impl std::convert::TryFrom<NSViewController> for CNContactViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<CNContactViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContactViewController").unwrap()) };
        if is_kind_of {
            Ok(CNContactViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to CNContactViewController")
        }
    }
}
impl INSResponder for CNContactViewController {}
impl PNSCoding for CNContactViewController {}
impl ICNContactViewController for CNContactViewController {}
pub trait ICNContactViewController: Sized + std::ops::Deref {
    unsafe fn contact(&self) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contact)
    }
    unsafe fn setContact_(&self, contact: CNContact)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContact : contact)
    }
    unsafe fn descriptorForRequiredKeys() -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactViewController").unwrap(), descriptorForRequiredKeys)
    }
}

unsafe impl objc2::encode::RefEncode for CNContactPicker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContactPicker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContactViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContactViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
