#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ScreenSaverView(pub id);
impl std::ops::Deref for ScreenSaverView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ScreenSaverView {}
impl ScreenSaverView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ScreenSaverView").unwrap(), alloc) })
    }
}
impl INSView for ScreenSaverView {}
impl PNSAnimatablePropertyContainer for ScreenSaverView {}
impl PNSUserInterfaceItemIdentification for ScreenSaverView {}
impl PNSDraggingDestination for ScreenSaverView {}
impl PNSAppearanceCustomization for ScreenSaverView {}
impl PNSAccessibilityElement for ScreenSaverView {}
impl PNSAccessibility for ScreenSaverView {}
impl std::convert::TryFrom<NSView> for ScreenSaverView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<ScreenSaverView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ScreenSaverView").unwrap()) };
        if is_kind_of {
            Ok(ScreenSaverView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to ScreenSaverView")
        }
    }
}
impl INSResponder for ScreenSaverView {}
impl PNSCoding for ScreenSaverView {}
impl INSObject for ScreenSaverView {}
impl PNSObject for ScreenSaverView {}
impl IScreenSaverView for ScreenSaverView {}
pub trait IScreenSaverView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_isPreview_(&self, frame: NSRect, isPreview: BOOL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame, isPreview : isPreview)
    }
    unsafe fn startAnimation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startAnimation)
    }
    unsafe fn stopAnimation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopAnimation)
    }
    unsafe fn drawRect_(&self, rect: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawRect : rect)
    }
    unsafe fn animateOneFrame(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animateOneFrame)
    }
    unsafe fn animationTimeInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationTimeInterval)
    }
    unsafe fn setAnimationTimeInterval_(&self, animationTimeInterval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimationTimeInterval : animationTimeInterval)
    }
    unsafe fn isAnimating(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnimating)
    }
    unsafe fn hasConfigureSheet(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasConfigureSheet)
    }
    unsafe fn configureSheet(&self) -> NSWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configureSheet)
    }
    unsafe fn isPreview(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPreview)
    }
    unsafe fn backingStoreType() -> NSBackingStoreType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ScreenSaverView").unwrap(), backingStoreType)
    }
    unsafe fn performGammaFade() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ScreenSaverView").unwrap(), performGammaFade)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ScreenSaverDefaults(pub id);
impl std::ops::Deref for ScreenSaverDefaults {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ScreenSaverDefaults {}
impl ScreenSaverDefaults {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ScreenSaverDefaults").unwrap(), alloc) })
    }
}
impl INSUserDefaults for ScreenSaverDefaults {}
impl std::convert::TryFrom<NSUserDefaults> for ScreenSaverDefaults {
    type Error = &'static str;
    fn try_from(parent: NSUserDefaults) -> Result<ScreenSaverDefaults, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ScreenSaverDefaults").unwrap()) };
        if is_kind_of {
            Ok(ScreenSaverDefaults(parent.0))
        } else {
            Err("This NSUserDefaults cannot be downcasted to ScreenSaverDefaults")
        }
    }
}
impl INSObject for ScreenSaverDefaults {}
impl PNSObject for ScreenSaverDefaults {}
impl IScreenSaverDefaults for ScreenSaverDefaults {}
pub trait IScreenSaverDefaults: Sized + std::ops::Deref {
    unsafe fn defaultsForModuleWithName_(inModuleName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ScreenSaverDefaults").unwrap(), defaultsForModuleWithName : inModuleName)
    }
}

unsafe impl objc2::encode::RefEncode for ScreenSaverView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScreenSaverView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ScreenSaverDefaults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScreenSaverDefaults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
