#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DDDevicePickerViewController(pub id);
impl std::ops::Deref for DDDevicePickerViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DDDevicePickerViewController {}
impl DDDevicePickerViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DDDevicePickerViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for DDDevicePickerViewController {}
impl INSObject for DDDevicePickerViewController {}
impl PNSObject for DDDevicePickerViewController {}
impl std::convert::TryFrom<NSObject> for DDDevicePickerViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DDDevicePickerViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DDDevicePickerViewController").unwrap()) };
        if is_kind_of {
            Ok(DDDevicePickerViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DDDevicePickerViewController")
        }
    }
}
impl IDDDevicePickerViewController for DDDevicePickerViewController {}
pub trait IDDDevicePickerViewController: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithNibName_bundle_(
        &self,
        nibNameOrNil: NSString,
        nibBundleOrNil: NSBundle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNibName : nibNameOrNil, bundle : nibBundleOrNil)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DDDevicePickerViewController").unwrap(), new)
    }
}
pub trait PUIAppearanceContainer: Sized + std::ops::Deref {}
impl PUIAppearanceContainer for DDDevicePickerViewController {}

unsafe impl objc2::encode::RefEncode for DDDevicePickerViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DDDevicePickerViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
