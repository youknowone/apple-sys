#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::HealthKit::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HKActivityRingView(pub id);
impl std::ops::Deref for HKActivityRingView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for HKActivityRingView {}
impl HKActivityRingView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"HKActivityRingView").unwrap(), alloc) })
    }
}
impl IUIView for HKActivityRingView {}
impl PUIAppearance for HKActivityRingView {}
impl PUIAppearanceContainer for HKActivityRingView {}
impl PUIDynamicItem for HKActivityRingView {}
impl PUITraitEnvironment for HKActivityRingView {}
impl PUICoordinateSpace for HKActivityRingView {}
impl PUIFocusItem for HKActivityRingView {}
impl PUIFocusItemContainer for HKActivityRingView {}
impl std::convert::TryFrom<UIView> for HKActivityRingView {
    type Error = &'static str;
    fn try_from(parent: UIView) -> Result<HKActivityRingView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"HKActivityRingView").unwrap()) };
        if is_kind_of {
            Ok(HKActivityRingView(parent.0))
        } else {
            Err("This UIView cannot be downcasted to HKActivityRingView")
        }
    }
}
impl IUIResponder for HKActivityRingView {}
impl PUIResponderStandardEditActions for HKActivityRingView {}
impl IHKActivityRingView for HKActivityRingView {}
pub trait IHKActivityRingView: Sized + std::ops::Deref {
    unsafe fn setActivitySummary_animated_(
        &self,
        activitySummary: HKActivitySummary,
        animated: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivitySummary : activitySummary, animated : animated)
    }
    unsafe fn activitySummary(&self) -> HKActivitySummary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activitySummary)
    }
    unsafe fn setActivitySummary_(&self, activitySummary: HKActivitySummary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivitySummary : activitySummary)
    }
}
pub trait HKHealthStore_UIKit: Sized + std::ops::Deref {
}

unsafe impl objc2::encode::RefEncode for HKActivityRingView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HKActivityRingView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
