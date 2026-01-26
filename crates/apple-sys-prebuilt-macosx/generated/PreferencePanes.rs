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
pub type NSPreferencePaneUnselectReply = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSPreferencePane(pub id);
impl std::ops::Deref for NSPreferencePane {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSPreferencePane {}
impl NSPreferencePane {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSPreferencePane").unwrap(), alloc) })
    }
}
impl INSObject for NSPreferencePane {}
impl PNSObject for NSPreferencePane {}
impl std::convert::TryFrom<NSObject> for NSPreferencePane {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSPreferencePane, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSPreferencePane").unwrap()) };
        if is_kind_of {
            Ok(NSPreferencePane(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSPreferencePane")
        }
    }
}
impl INSPreferencePane for NSPreferencePane {}
pub trait INSPreferencePane: Sized + std::ops::Deref {
    unsafe fn initWithBundle_(&self, bundle: NSBundle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBundle : bundle)
    }
    unsafe fn loadMainView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loadMainView)
    }
    unsafe fn mainViewDidLoad(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainViewDidLoad)
    }
    unsafe fn assignMainView(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assignMainView)
    }
    unsafe fn willSelect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willSelect)
    }
    unsafe fn didSelect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didSelect)
    }
    unsafe fn replyToShouldUnselect_(&self, shouldUnselect: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replyToShouldUnselect : shouldUnselect)
    }
    unsafe fn willUnselect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willUnselect)
    }
    unsafe fn didUnselect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didUnselect)
    }
    unsafe fn updateHelpMenuWithArray_(&self, inArrayOfMenuItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateHelpMenuWithArray : inArrayOfMenuItems)
    }
    unsafe fn bundle(&self) -> NSBundle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundle)
    }
    unsafe fn mainNibName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainNibName)
    }
    unsafe fn shouldUnselect(&self) -> NSPreferencePaneUnselectReply
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldUnselect)
    }
    unsafe fn mainView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainView)
    }
    unsafe fn setMainView_(&self, mainView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMainView : mainView)
    }
    unsafe fn initialKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialKeyView)
    }
    unsafe fn setInitialKeyView_(&self, initialKeyView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialKeyView : initialKeyView)
    }
    unsafe fn firstKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstKeyView)
    }
    unsafe fn setFirstKeyView_(&self, firstKeyView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFirstKeyView : firstKeyView)
    }
    unsafe fn lastKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastKeyView)
    }
    unsafe fn setLastKeyView_(&self, lastKeyView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLastKeyView : lastKeyView)
    }
    unsafe fn autoSaveTextFields(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoSaveTextFields)
    }
    unsafe fn isSelected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSelected)
    }
}
unsafe extern "C" {
    pub static NSPreferencePaneDoUnselectNotification: NSString;
}
unsafe extern "C" {
    pub static NSPreferencePaneCancelUnselectNotification: NSString;
}
unsafe extern "C" {
    pub static NSPreferencePaneSwitchToPaneNotification: NSString;
}
unsafe extern "C" {
    pub static NSPreferencePrefPaneIsAvailableNotification: NSString;
}
unsafe extern "C" {
    pub static NSPreferencePaneUpdateHelpMenuNotification: NSString;
}
unsafe extern "C" {
    pub static NSPrefPaneHelpMenuInfoPListKey: NSString;
}
unsafe extern "C" {
    pub static NSPrefPaneHelpMenuTitleKey: NSString;
}
unsafe extern "C" {
    pub static NSPrefPaneHelpMenuAnchorKey: NSString;
}

unsafe impl objc2::encode::RefEncode for NSPreferencePane {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSPreferencePane {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
