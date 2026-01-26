#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Intents::*;
#[allow(unused_imports)]
use crate::Security::*;

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
impl PNSCoding for INUIAddVoiceShortcutButton {}
impl INSObject for INUIAddVoiceShortcutButton {}
impl PNSObject for INUIAddVoiceShortcutButton {}
impl std::convert::TryFrom<NSObject> for INUIAddVoiceShortcutButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<INUIAddVoiceShortcutButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"INUIAddVoiceShortcutButton").unwrap()) };
        if is_kind_of {
            Ok(INUIAddVoiceShortcutButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to INUIAddVoiceShortcutButton")
        }
    }
}
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
impl PNSCoding for INUIAddVoiceShortcutViewController {}
impl INSObject for INUIAddVoiceShortcutViewController {}
impl PNSObject for INUIAddVoiceShortcutViewController {}
impl std::convert::TryFrom<NSObject> for INUIAddVoiceShortcutViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<INUIAddVoiceShortcutViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"INUIAddVoiceShortcutViewController").unwrap())
        };
        if is_kind_of {
            Ok(INUIAddVoiceShortcutViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to INUIAddVoiceShortcutViewController")
        }
    }
}
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
impl PNSCoding for INUIEditVoiceShortcutViewController {}
impl INSObject for INUIEditVoiceShortcutViewController {}
impl PNSObject for INUIEditVoiceShortcutViewController {}
impl std::convert::TryFrom<NSObject> for INUIEditVoiceShortcutViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<INUIEditVoiceShortcutViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"INUIEditVoiceShortcutViewController").unwrap())
        };
        if is_kind_of {
            Ok(INUIEditVoiceShortcutViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to INUIEditVoiceShortcutViewController")
        }
    }
}
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
pub trait PNSAccessibilityButton: Sized + std::ops::Deref {
    unsafe fn accessibilityLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityLabel)
    }
    unsafe fn accessibilityPerformPress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityPerformPress)
    }
}
pub trait PNSUserInterfaceValidations: Sized + std::ops::Deref {
    unsafe fn validateUserInterfaceItem_(&self, item: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateUserInterfaceItem : item)
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
impl PNSUserInterfaceValidations for INUIAddVoiceShortcutButton {}
impl PNSAccessibilityButton for INUIAddVoiceShortcutButton {}
impl PNSAnimatablePropertyContainer for INUIAddVoiceShortcutButton {}
impl PNSAccessibilityElement for INUIAddVoiceShortcutButton {}
impl PNSEditor for INUIAddVoiceShortcutViewController {}
impl PNSEditor for INUIEditVoiceShortcutViewController {}
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
