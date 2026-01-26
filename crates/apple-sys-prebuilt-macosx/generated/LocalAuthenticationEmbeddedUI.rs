#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::LocalAuthentication::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type LAPresentationContext = NSWindow;
pub trait LARight_UI: Sized + std::ops::Deref {
    unsafe fn authorizeWithLocalizedReason_inPresentationContext_completion_(
        &self,
        localizedReason: NSString,
        presentationContext: NSWindow,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizeWithLocalizedReason : localizedReason, inPresentationContext : presentationContext, completion : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LAAuthenticationView(pub id);
impl std::ops::Deref for LAAuthenticationView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LAAuthenticationView {}
impl LAAuthenticationView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LAAuthenticationView").unwrap(), alloc) })
    }
}
impl INSView for LAAuthenticationView {}
impl PNSAnimatablePropertyContainer for LAAuthenticationView {}
impl PNSUserInterfaceItemIdentification for LAAuthenticationView {}
impl PNSDraggingDestination for LAAuthenticationView {}
impl PNSAppearanceCustomization for LAAuthenticationView {}
impl PNSAccessibilityElement for LAAuthenticationView {}
impl PNSAccessibility for LAAuthenticationView {}
impl std::convert::TryFrom<NSView> for LAAuthenticationView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<LAAuthenticationView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LAAuthenticationView").unwrap()) };
        if is_kind_of {
            Ok(LAAuthenticationView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to LAAuthenticationView")
        }
    }
}
impl INSResponder for LAAuthenticationView {}
impl PNSCoding for LAAuthenticationView {}
impl INSObject for LAAuthenticationView {}
impl PNSObject for LAAuthenticationView {}
impl ILAAuthenticationView for LAAuthenticationView {}
pub trait ILAAuthenticationView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_(&self, frameRect: NSRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frameRect)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
    unsafe fn initWithContext_(&self, context: LAContext) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContext : context)
    }
    unsafe fn initWithContext_controlSize_(
        &self,
        context: LAContext,
        controlSize: NSControlSize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContext : context, controlSize : controlSize)
    }
    unsafe fn context(&self) -> LAContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn controlSize(&self) -> NSControlSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlSize)
    }
}

unsafe impl objc2::encode::RefEncode for LAAuthenticationView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LAAuthenticationView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
