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
pub type InstallerSectionDirection = NSInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InstallerPane_Private {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct InstallerPane(pub id);
impl std::ops::Deref for InstallerPane {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for InstallerPane {}
impl InstallerPane {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"InstallerPane").unwrap(), alloc) })
    }
}
impl INSObject for InstallerPane {}
impl PNSObject for InstallerPane {}
impl std::convert::TryFrom<NSObject> for InstallerPane {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<InstallerPane, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"InstallerPane").unwrap()) };
        if is_kind_of {
            Ok(InstallerPane(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to InstallerPane")
        }
    }
}
impl IInstallerPane for InstallerPane {}
pub trait IInstallerPane: Sized + std::ops::Deref {
    unsafe fn initWithSection_(&self, parent: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSection : parent)
    }
    unsafe fn contentView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentView)
    }
    unsafe fn initialKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialKeyView)
    }
    unsafe fn firstKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstKeyView)
    }
    unsafe fn lastKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastKeyView)
    }
    unsafe fn nextPane(&self) -> InstallerPane
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextPane)
    }
    unsafe fn willEnterPane_(&self, dir: InstallerSectionDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willEnterPane : dir)
    }
    unsafe fn didEnterPane_(&self, dir: InstallerSectionDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didEnterPane : dir)
    }
    unsafe fn shouldExitPane_(&self, dir: InstallerSectionDirection) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldExitPane : dir)
    }
    unsafe fn willExitPane_(&self, dir: InstallerSectionDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willExitPane : dir)
    }
    unsafe fn didExitPane_(&self, dir: InstallerSectionDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didExitPane : dir)
    }
    unsafe fn setContentView_(&self, contentView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentView : contentView)
    }
    unsafe fn setInitialKeyView_(&self, initialKeyView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialKeyView : initialKeyView)
    }
    unsafe fn setFirstKeyView_(&self, firstKeyView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFirstKeyView : firstKeyView)
    }
    unsafe fn setLastKeyView_(&self, lastKeyView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLastKeyView : lastKeyView)
    }
    unsafe fn setNextPane_(&self, nextPane: InstallerPane)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNextPane : nextPane)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn section(&self) -> InstallerSection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, section)
    }
    unsafe fn nextEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextEnabled)
    }
    unsafe fn setNextEnabled_(&self, nextEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNextEnabled : nextEnabled)
    }
    unsafe fn previousEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousEnabled)
    }
    unsafe fn setPreviousEnabled_(&self, previousEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviousEnabled : previousEnabled)
    }
    unsafe fn gotoNextPane(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gotoNextPane)
    }
    unsafe fn gotoPreviousPane(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gotoPreviousPane)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InstallerSection_Private {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct InstallerSection(pub id);
impl std::ops::Deref for InstallerSection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for InstallerSection {}
impl InstallerSection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"InstallerSection").unwrap(), alloc) })
    }
}
impl INSObject for InstallerSection {}
impl PNSObject for InstallerSection {}
impl std::convert::TryFrom<NSObject> for InstallerSection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<InstallerSection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"InstallerSection").unwrap()) };
        if is_kind_of {
            Ok(InstallerSection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to InstallerSection")
        }
    }
}
impl IInstallerSection for InstallerSection {}
pub trait IInstallerSection: Sized + std::ops::Deref {
    unsafe fn willLoadMainNib(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willLoadMainNib)
    }
    unsafe fn didLoadMainNib(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didLoadMainNib)
    }
    unsafe fn sharedDictionary(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharedDictionary)
    }
    unsafe fn gotoPane_(&self, pane: InstallerPane) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gotoPane : pane)
    }
    unsafe fn bundle(&self) -> NSBundle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundle)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn firstPane(&self) -> InstallerPane
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstPane)
    }
    unsafe fn shouldLoad(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldLoad)
    }
    unsafe fn installerState(&self) -> InstallerState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, installerState)
    }
    unsafe fn activePane(&self) -> InstallerPane
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activePane)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InstallerState_Private {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct InstallerState(pub id);
impl std::ops::Deref for InstallerState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for InstallerState {}
impl InstallerState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"InstallerState").unwrap(), alloc) })
    }
}
impl INSObject for InstallerState {}
impl PNSObject for InstallerState {}
impl std::convert::TryFrom<NSObject> for InstallerState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<InstallerState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"InstallerState").unwrap()) };
        if is_kind_of {
            Ok(InstallerState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to InstallerState")
        }
    }
}
impl IInstallerState for InstallerState {}
pub trait IInstallerState: Sized + std::ops::Deref {
    unsafe fn choiceDictionaryForIdentifier_(&self, choiceIdentifier: NSString) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, choiceDictionaryForIdentifier : choiceIdentifier)
    }
    unsafe fn licenseAgreed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, licenseAgreed)
    }
    unsafe fn licenseAgreedLanguage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, licenseAgreedLanguage)
    }
    unsafe fn targetVolumePath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetVolumePath)
    }
    unsafe fn targetPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetPath)
    }
    unsafe fn choiceDictionaries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, choiceDictionaries)
    }
    unsafe fn installStarted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, installStarted)
    }
    unsafe fn installSucceeded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, installSucceeded)
    }
}
unsafe extern "C" {
    pub static InstallerState_Choice_Identifier: NSString;
}
unsafe extern "C" {
    pub static InstallerState_Choice_Installed: NSString;
}
unsafe extern "C" {
    pub static InstallerState_Choice_CustomLocation: NSString;
}

unsafe impl objc2::encode::RefEncode for InstallerPane_Private {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for InstallerPane_Private {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("InstallerPane_Private", &[]);
}
unsafe impl objc2::encode::RefEncode for InstallerPane {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for InstallerPane {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for InstallerSection_Private {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for InstallerSection_Private {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("InstallerSection_Private", &[]);
}
unsafe impl objc2::encode::RefEncode for InstallerSection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for InstallerSection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for InstallerState_Private {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for InstallerState_Private {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("InstallerState_Private", &[]);
}
unsafe impl objc2::encode::RefEncode for InstallerState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for InstallerState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
