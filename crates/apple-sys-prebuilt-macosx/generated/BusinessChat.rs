#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type BCParameterName = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BCChatAction(pub id);
impl std::ops::Deref for BCChatAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BCChatAction {}
impl BCChatAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BCChatAction").unwrap(), alloc) })
    }
}
impl INSObject for BCChatAction {}
impl PNSObject for BCChatAction {}
impl std::convert::TryFrom<NSObject> for BCChatAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BCChatAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BCChatAction").unwrap()) };
        if is_kind_of {
            Ok(BCChatAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BCChatAction")
        }
    }
}
impl IBCChatAction for BCChatAction {}
pub trait IBCChatAction: Sized + std::ops::Deref {
    unsafe fn openTranscript_intentParameters_(
        businessIdentifier: NSString,
        intentParameters: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BCChatAction").unwrap(), openTranscript : businessIdentifier, intentParameters : intentParameters)
    }
}
pub type BCChatButtonStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BCChatButton(pub id);
impl std::ops::Deref for BCChatButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BCChatButton {}
impl BCChatButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BCChatButton").unwrap(), alloc) })
    }
}
impl PNSCoding for BCChatButton {}
impl INSObject for BCChatButton {}
impl PNSObject for BCChatButton {}
impl std::convert::TryFrom<NSObject> for BCChatButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BCChatButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BCChatButton").unwrap()) };
        if is_kind_of {
            Ok(BCChatButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BCChatButton")
        }
    }
}
impl IBCChatButton for BCChatButton {}
pub trait IBCChatButton: Sized + std::ops::Deref {
    unsafe fn initWithStyle_(&self, style: BCChatButtonStyle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStyle : style)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
}
pub trait PNSAccessibilityElement: Sized + std::ops::Deref {
    unsafe fn accessibilityFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityFrame)
    }
    unsafe fn accessibilityParent(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityParent)
    }
    unsafe fn isAccessibilityFocused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAccessibilityFocused)
    }
    unsafe fn accessibilityIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityIdentifier)
    }
}
pub trait PNSAnimatablePropertyContainer: Sized + std::ops::Deref {
    unsafe fn animator(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animator)
    }
    unsafe fn animationForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, animationForKey : key)
    }
    unsafe fn animations(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animations)
    }
    unsafe fn setAnimations_(&self, animations: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimations : animations)
    }
    unsafe fn defaultAnimationForKey_(key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAnimatablePropertyContainer").unwrap(), defaultAnimationForKey : key)
    }
}
impl PNSAnimatablePropertyContainer for BCChatButton {}
impl PNSAccessibilityElement for BCChatButton {}
unsafe extern "C" {
    pub static BCParameterNameIntent: BCParameterName;
}
unsafe extern "C" {
    pub static BCParameterNameGroup: BCParameterName;
}
unsafe extern "C" {
    pub static BCParameterNameBody: BCParameterName;
}

unsafe impl objc2::encode::RefEncode for BCChatAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BCChatAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BCChatButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BCChatButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
