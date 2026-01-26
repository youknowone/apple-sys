#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CLLocationButtonIcon = NSInteger;
pub type CLLocationButtonLabel = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLLocationButton(pub id);
impl std::ops::Deref for CLLocationButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLLocationButton {}
impl CLLocationButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLLocationButton").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CLLocationButton {}
impl PNSCoding for CLLocationButton {}
impl std::convert::TryFrom<NSObject> for CLLocationButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLLocationButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLLocationButton").unwrap()) };
        if is_kind_of {
            Ok(CLLocationButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLLocationButton")
        }
    }
}
impl ICLLocationButton for CLLocationButton {}
pub trait ICLLocationButton: Sized + std::ops::Deref {
    unsafe fn icon(&self) -> CLLocationButtonIcon
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, icon)
    }
    unsafe fn setIcon_(&self, icon: CLLocationButtonIcon)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIcon : icon)
    }
    unsafe fn label(&self) -> CLLocationButtonLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: CLLocationButtonLabel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn fontSize(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontSize)
    }
    unsafe fn setFontSize_(&self, fontSize: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontSize : fontSize)
    }
    unsafe fn cornerRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cornerRadius)
    }
    unsafe fn setCornerRadius_(&self, cornerRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCornerRadius : cornerRadius)
    }
}
pub trait PUICoordinateSpace: Sized + std::ops::Deref {
    unsafe fn convertPoint_toCoordinateSpace_(
        &self,
        point: CGPoint,
        coordinateSpace: *mut u64,
    ) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : point, toCoordinateSpace : coordinateSpace)
    }
    unsafe fn convertPoint_fromCoordinateSpace_(
        &self,
        point: CGPoint,
        coordinateSpace: *mut u64,
    ) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : point, fromCoordinateSpace : coordinateSpace)
    }
    unsafe fn convertRect_toCoordinateSpace_(
        &self,
        rect: CGRect,
        coordinateSpace: *mut u64,
    ) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRect : rect, toCoordinateSpace : coordinateSpace)
    }
    unsafe fn convertRect_fromCoordinateSpace_(
        &self,
        rect: CGRect,
        coordinateSpace: *mut u64,
    ) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRect : rect, fromCoordinateSpace : coordinateSpace)
    }
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
}
pub trait PUIAppearanceContainer: Sized + std::ops::Deref {}
pub trait PUIFocusItemContainer: Sized + std::ops::Deref {
    unsafe fn focusItemsInRect_(&self, rect: CGRect) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, focusItemsInRect : rect)
    }
    unsafe fn coordinateSpace(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinateSpace)
    }
}
impl PUIAppearanceContainer for CLLocationButton {}
impl PUICoordinateSpace for CLLocationButton {}
impl PUIFocusItemContainer for CLLocationButton {}
unsafe extern "C" {
    pub static mut CoreLocationUIVersionNumber: f64;
}
unsafe extern "C" {
    pub static CoreLocationUIVersionString: [::std::os::raw::c_uchar; 0usize];
}

unsafe impl objc2::encode::RefEncode for CLLocationButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLLocationButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
