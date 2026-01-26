#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Intents::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type INUIAddVoiceShortcutButtonStyle = NSUInteger;
pub trait PINUIAddVoiceShortcutButtonDelegate: Sized + std::ops::Deref {
    unsafe fn presentAddVoiceShortcutViewController_forAddVoiceShortcutButton_(
        &self,
        addVoiceShortcutViewController: INUIAddVoiceShortcutViewController,
        addVoiceShortcutButton: INUIAddVoiceShortcutButton,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentAddVoiceShortcutViewController : addVoiceShortcutViewController, forAddVoiceShortcutButton : addVoiceShortcutButton)
    }
    unsafe fn presentEditVoiceShortcutViewController_forAddVoiceShortcutButton_(
        &self,
        editVoiceShortcutViewController: INUIEditVoiceShortcutViewController,
        addVoiceShortcutButton: INUIAddVoiceShortcutButton,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentEditVoiceShortcutViewController : editVoiceShortcutViewController, forAddVoiceShortcutButton : addVoiceShortcutButton)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct INUIAddVoiceShortcutButton(pub id);
impl std::ops::Deref for INUIAddVoiceShortcutButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for INUIAddVoiceShortcutButton {}
impl INUIAddVoiceShortcutButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"INUIAddVoiceShortcutButton").unwrap(), alloc) })
    }
}
impl INSButton for INUIAddVoiceShortcutButton {}
impl PNSUserInterfaceValidations for INUIAddVoiceShortcutButton {}
impl PNSAccessibilityButton for INUIAddVoiceShortcutButton {}
impl PNSUserInterfaceCompression for INUIAddVoiceShortcutButton {}
impl std::convert::TryFrom<NSButton> for INUIAddVoiceShortcutButton {
    type Error = &'static str;
    fn try_from(parent: NSButton) -> Result<INUIAddVoiceShortcutButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"INUIAddVoiceShortcutButton").unwrap()) };
        if is_kind_of {
            Ok(INUIAddVoiceShortcutButton(parent.0))
        } else {
            Err("This NSButton cannot be downcasted to INUIAddVoiceShortcutButton")
        }
    }
}
impl INSControl for INUIAddVoiceShortcutButton {}
impl INSView for INUIAddVoiceShortcutButton {}
impl PNSAnimatablePropertyContainer for INUIAddVoiceShortcutButton {}
impl PNSUserInterfaceItemIdentification for INUIAddVoiceShortcutButton {}
impl PNSDraggingDestination for INUIAddVoiceShortcutButton {}
impl PNSAppearanceCustomization for INUIAddVoiceShortcutButton {}
impl PNSAccessibilityElement for INUIAddVoiceShortcutButton {}
impl PNSAccessibility for INUIAddVoiceShortcutButton {}
impl INSResponder for INUIAddVoiceShortcutButton {}
impl PNSCoding for INUIAddVoiceShortcutButton {}
impl INSObject for INUIAddVoiceShortcutButton {}
impl PNSObject for INUIAddVoiceShortcutButton {}
impl IINUIAddVoiceShortcutButton for INUIAddVoiceShortcutButton {}
pub trait IINUIAddVoiceShortcutButton: Sized + std::ops::Deref {
    unsafe fn initWithStyle_(&self, style: INUIAddVoiceShortcutButtonStyle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStyle : style)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFrame_(&self, frame: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame)
    }
    unsafe fn style(&self) -> INUIAddVoiceShortcutButtonStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn setStyle_(&self, style: INUIAddVoiceShortcutButtonStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStyle : style)
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
    unsafe fn shortcut(&self) -> INShortcut
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shortcut)
    }
    unsafe fn setShortcut_(&self, shortcut: INShortcut)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShortcut : shortcut)
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
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct INUIAddVoiceShortcutViewController(pub id);
impl std::ops::Deref for INUIAddVoiceShortcutViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for INUIAddVoiceShortcutViewController {}
impl INUIAddVoiceShortcutViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"INUIAddVoiceShortcutViewController").unwrap(), alloc) })
    }
}
impl INSViewController for INUIAddVoiceShortcutViewController {}
impl PNSEditor for INUIAddVoiceShortcutViewController {}
impl PNSSeguePerforming for INUIAddVoiceShortcutViewController {}
impl PNSUserInterfaceItemIdentification for INUIAddVoiceShortcutViewController {}
impl std::convert::TryFrom<NSViewController> for INUIAddVoiceShortcutViewController {
    type Error = &'static str;
    fn try_from(
        parent: NSViewController,
    ) -> Result<INUIAddVoiceShortcutViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"INUIAddVoiceShortcutViewController").unwrap())
        };
        if is_kind_of {
            Ok(INUIAddVoiceShortcutViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to INUIAddVoiceShortcutViewController")
        }
    }
}
impl INSResponder for INUIAddVoiceShortcutViewController {}
impl PNSCoding for INUIAddVoiceShortcutViewController {}
impl INSObject for INUIAddVoiceShortcutViewController {}
impl PNSObject for INUIAddVoiceShortcutViewController {}
impl IINUIAddVoiceShortcutViewController for INUIAddVoiceShortcutViewController {}
pub trait IINUIAddVoiceShortcutViewController: Sized + std::ops::Deref {
    unsafe fn initWithShortcut_(&self, shortcut: INShortcut) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithShortcut : shortcut)
    }
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
pub trait PINUIAddVoiceShortcutViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn addVoiceShortcutViewController_didFinishWithVoiceShortcut_error_(
        &self,
        controller: INUIAddVoiceShortcutViewController,
        voiceShortcut: INVoiceShortcut,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addVoiceShortcutViewController : controller, didFinishWithVoiceShortcut : voiceShortcut, error : error)
    }
    unsafe fn addVoiceShortcutViewControllerDidCancel_(
        &self,
        controller: INUIAddVoiceShortcutViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addVoiceShortcutViewControllerDidCancel : controller)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct INUIEditVoiceShortcutViewController(pub id);
impl std::ops::Deref for INUIEditVoiceShortcutViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for INUIEditVoiceShortcutViewController {}
impl INUIEditVoiceShortcutViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"INUIEditVoiceShortcutViewController").unwrap(), alloc) })
    }
}
impl INSViewController for INUIEditVoiceShortcutViewController {}
impl PNSEditor for INUIEditVoiceShortcutViewController {}
impl PNSSeguePerforming for INUIEditVoiceShortcutViewController {}
impl PNSUserInterfaceItemIdentification for INUIEditVoiceShortcutViewController {}
impl std::convert::TryFrom<NSViewController> for INUIEditVoiceShortcutViewController {
    type Error = &'static str;
    fn try_from(
        parent: NSViewController,
    ) -> Result<INUIEditVoiceShortcutViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"INUIEditVoiceShortcutViewController").unwrap())
        };
        if is_kind_of {
            Ok(INUIEditVoiceShortcutViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to INUIEditVoiceShortcutViewController")
        }
    }
}
impl INSResponder for INUIEditVoiceShortcutViewController {}
impl PNSCoding for INUIEditVoiceShortcutViewController {}
impl INSObject for INUIEditVoiceShortcutViewController {}
impl PNSObject for INUIEditVoiceShortcutViewController {}
impl IINUIEditVoiceShortcutViewController for INUIEditVoiceShortcutViewController {}
pub trait IINUIEditVoiceShortcutViewController: Sized + std::ops::Deref {
    unsafe fn initWithVoiceShortcut_(&self, voiceShortcut: INVoiceShortcut) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVoiceShortcut : voiceShortcut)
    }
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
pub trait PINUIEditVoiceShortcutViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn editVoiceShortcutViewController_didUpdateVoiceShortcut_error_(
        &self,
        controller: INUIEditVoiceShortcutViewController,
        voiceShortcut: INVoiceShortcut,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, editVoiceShortcutViewController : controller, didUpdateVoiceShortcut : voiceShortcut, error : error)
    }
    unsafe fn editVoiceShortcutViewController_didDeleteVoiceShortcutWithIdentifier_(
        &self,
        controller: INUIEditVoiceShortcutViewController,
        deletedVoiceShortcutIdentifier: NSUUID,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, editVoiceShortcutViewController : controller, didDeleteVoiceShortcutWithIdentifier : deletedVoiceShortcutIdentifier)
    }
    unsafe fn editVoiceShortcutViewControllerDidCancel_(
        &self,
        controller: INUIEditVoiceShortcutViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, editVoiceShortcutViewControllerDidCancel : controller)
    }
}
pub trait INImage_IntentsUI: Sized + std::ops::Deref {
    unsafe fn imageWithNSImage_(image: NSImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"INImage").unwrap(), imageWithNSImage : image)
    }
}
unsafe extern "C" {
    pub static mut IntentsUIVersionNumber: f64;
}
unsafe extern "C" {
    pub static IntentsUIVersionString: [::std::os::raw::c_uchar; 0usize];
}

unsafe impl objc2::encode::RefEncode for INUIAddVoiceShortcutButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for INUIAddVoiceShortcutButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for INUIAddVoiceShortcutViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for INUIAddVoiceShortcutViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for INUIEditVoiceShortcutViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for INUIEditVoiceShortcutViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
