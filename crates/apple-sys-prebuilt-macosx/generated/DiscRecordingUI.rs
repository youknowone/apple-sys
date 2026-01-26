#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::DiscRecording::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DRBurnSession {
    _unused: [u8; 0],
}
pub type DRBurnSessionRef = *mut __DRBurnSession;
pub type DRBurnSessionSetupDialogOptionFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DRBurnSessionSetupDialogOptions {
    pub version: UInt32,
    pub dialogOptionFlags: DRBurnSessionSetupDialogOptionFlags,
    pub defaultButtonTitle: CFStringRef,
}
pub type DRBurnSessionDeviceCheckProcPtr = ::std::option::Option<
    unsafe extern "C" fn(burnSession: DRBurnSessionRef, device: DRDeviceRef) -> Boolean,
>;
pub type DRBurnSessionMediaCheckProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        burnSession: DRBurnSessionRef,
        device: DRDeviceRef,
        prompt: *mut CFStringRef,
    ) -> Boolean,
>;
pub type DRBurnSessionDeviceSelectionNotificationProcPtr =
    ::std::option::Option<unsafe extern "C" fn(burnSession: DRBurnSessionRef, device: DRDeviceRef)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DRBurnSessionSetupCallbacks {
    pub version: UInt32,
    pub deviceShouldBeTarget: DRBurnSessionDeviceCheckProcPtr,
    pub containsSuitableMedia: DRBurnSessionMediaCheckProcPtr,
    pub deviceSelectionChanged: DRBurnSessionDeviceSelectionNotificationProcPtr,
}
pub type DRBurnSessionProgressBeginNotificationProcPtr =
    ::std::option::Option<unsafe extern "C" fn(burnSession: DRBurnSessionRef)>;
pub type DRBurnSessionProgressFinishNotificationProcPtr =
    ::std::option::Option<unsafe extern "C" fn(burnSession: DRBurnSessionRef)>;
pub type DRBurnSessionBurnCompleteProcPtr = ::std::option::Option<
    unsafe extern "C" fn(burnSession: DRBurnSessionRef, burn: DRBurnRef) -> Boolean,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DRBurnSessionProgressCallbacks {
    pub version: UInt32,
    pub progressWillBegin: DRBurnSessionProgressBeginNotificationProcPtr,
    pub progressDidFinish: DRBurnSessionProgressFinishNotificationProcPtr,
    pub burnDidFinish: DRBurnSessionBurnCompleteProcPtr,
}
pub type DRBurnSessionProgressDialogOptionFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DRBurnSessionProgressDialogOptions {
    pub version: UInt32,
    pub dialogOptionFlags: DRBurnSessionProgressDialogOptionFlags,
    pub description: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DREraseSession {
    _unused: [u8; 0],
}
pub type DREraseSessionRef = *mut __DREraseSession;
pub type DREraseSessionSetupDialogOptionFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DREraseSessionSetupDialogOptions {
    pub version: UInt32,
    pub dialogOptionFlags: DREraseSessionSetupDialogOptionFlags,
}
pub type DREraseSessionDeviceCheckProcPtr = ::std::option::Option<
    unsafe extern "C" fn(eraseSession: DREraseSessionRef, device: DRDeviceRef) -> Boolean,
>;
pub type DREraseSessionMediaCheckProcPtr = ::std::option::Option<
    unsafe extern "C" fn(
        eraseSession: DREraseSessionRef,
        device: DRDeviceRef,
        prompt: *mut CFStringRef,
    ) -> Boolean,
>;
pub type DREraseSessionDeviceSelectionNotificationProcPtr = ::std::option::Option<
    unsafe extern "C" fn(eraseSession: DREraseSessionRef, selectedDevice: DRDeviceRef),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DREraseSessionSetupCallbacks {
    pub version: UInt32,
    pub deviceShouldBeTarget: DREraseSessionDeviceCheckProcPtr,
    pub containsSuitableMedia: DREraseSessionMediaCheckProcPtr,
    pub deviceSelectionChanged: DREraseSessionDeviceSelectionNotificationProcPtr,
}
pub type DREraseSessionProgressBeginNotificationProcPtr =
    ::std::option::Option<unsafe extern "C" fn(eraseSession: DREraseSessionRef)>;
pub type DREraseSessionProgressFinishNotificationProcPtr =
    ::std::option::Option<unsafe extern "C" fn(eraseSession: DREraseSessionRef)>;
pub type DREraseSessionEraseCompleteProcPtr = ::std::option::Option<
    unsafe extern "C" fn(eraseSession: DREraseSessionRef, erase: DREraseRef) -> Boolean,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DREraseSessionProgressCallbacks {
    pub version: UInt32,
    pub progressWillBegin: DREraseSessionProgressBeginNotificationProcPtr,
    pub progressDidFinish: DREraseSessionProgressFinishNotificationProcPtr,
    pub eraseDidFinish: DREraseSessionEraseCompleteProcPtr,
}
pub type DREraseSessionProgressDialogOptionFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DREraseSessionProgressDialogOptions {
    pub version: UInt32,
    pub dialogOptionFlags: DREraseSessionProgressDialogOptionFlags,
    pub description: CFStringRef,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRSetupPanel(pub id);
impl std::ops::Deref for DRSetupPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRSetupPanel {}
impl DRSetupPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRSetupPanel").unwrap(), alloc) })
    }
}
impl INSPanel for DRSetupPanel {}
impl std::convert::TryFrom<NSPanel> for DRSetupPanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<DRSetupPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRSetupPanel").unwrap()) };
        if is_kind_of {
            Ok(DRSetupPanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to DRSetupPanel")
        }
    }
}
impl INSWindow for DRSetupPanel {}
impl PNSAnimatablePropertyContainer for DRSetupPanel {}
impl PNSMenuItemValidation for DRSetupPanel {}
impl PNSUserInterfaceValidations for DRSetupPanel {}
impl PNSUserInterfaceItemIdentification for DRSetupPanel {}
impl PNSAppearanceCustomization for DRSetupPanel {}
impl PNSAccessibilityElement for DRSetupPanel {}
impl PNSAccessibility for DRSetupPanel {}
impl INSResponder for DRSetupPanel {}
impl PNSCoding for DRSetupPanel {}
impl INSObject for DRSetupPanel {}
impl PNSObject for DRSetupPanel {}
impl IDRSetupPanel for DRSetupPanel {}
pub trait IDRSetupPanel: Sized + std::ops::Deref {
    unsafe fn initWithNibName_(&self, nibName: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNibName : nibName)
    }
    unsafe fn runSetupPanel(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, runSetupPanel)
    }
    unsafe fn beginSetupSheetForWindow_modalDelegate_didEndSelector_contextInfo_(
        &self,
        owner: NSWindow,
        modalDelegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSetupSheetForWindow : owner, modalDelegate : modalDelegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn ok_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ok : sender)
    }
    unsafe fn cancel_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancel : sender)
    }
    unsafe fn eject_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eject : sender)
    }
    unsafe fn open_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, open : sender)
    }
    unsafe fn close_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, close : sender)
    }
    unsafe fn deviceSelectionChanged_(&self, device: DRDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceSelectionChanged : device)
    }
    unsafe fn mediaStateChanged_(&self, status: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mediaStateChanged : status)
    }
    unsafe fn setupForDisplay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setupForDisplay)
    }
}
pub trait NSObject_DRSetupPanelDelegate: Sized + std::ops::Deref {
    unsafe fn setupPanel_deviceCouldBeTarget_(&self, aPanel: DRSetupPanel, device: DRDevice) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setupPanel : aPanel, deviceCouldBeTarget : device)
    }
    unsafe fn setupPanel_determineBestDeviceOfA_orB_(
        &self,
        aPanel: DRSetupPanel,
        deviceA: DRDevice,
        device: DRDevice,
    ) -> DRDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setupPanel : aPanel, determineBestDeviceOfA : deviceA, orB : device)
    }
    unsafe fn setupPanelDeviceSelectionChanged_(&self, aNotification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setupPanelDeviceSelectionChanged : aNotification)
    }
    unsafe fn setupPanelShouldHandleMediaReservations_(&self, aPanel: DRSetupPanel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setupPanelShouldHandleMediaReservations : aPanel)
    }
    unsafe fn setupPanel_deviceContainsSuitableMedia_promptString_(
        &self,
        aPanel: DRSetupPanel,
        device: DRDevice,
        prompt: *mut NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setupPanel : aPanel, deviceContainsSuitableMedia : device, promptString : prompt)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRBurnSetupPanel(pub id);
impl std::ops::Deref for DRBurnSetupPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRBurnSetupPanel {}
impl DRBurnSetupPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRBurnSetupPanel").unwrap(), alloc) })
    }
}
impl IDRSetupPanel for DRBurnSetupPanel {}
impl From<DRBurnSetupPanel> for DRSetupPanel {
    fn from(child: DRBurnSetupPanel) -> DRSetupPanel {
        DRSetupPanel(child.0)
    }
}
impl std::convert::TryFrom<DRSetupPanel> for DRBurnSetupPanel {
    type Error = &'static str;
    fn try_from(parent: DRSetupPanel) -> Result<DRBurnSetupPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRBurnSetupPanel").unwrap()) };
        if is_kind_of {
            Ok(DRBurnSetupPanel(parent.0))
        } else {
            Err("This DRSetupPanel cannot be downcasted to DRBurnSetupPanel")
        }
    }
}
impl INSPanel for DRBurnSetupPanel {}
impl INSWindow for DRBurnSetupPanel {}
impl PNSAnimatablePropertyContainer for DRBurnSetupPanel {}
impl PNSMenuItemValidation for DRBurnSetupPanel {}
impl PNSUserInterfaceValidations for DRBurnSetupPanel {}
impl PNSUserInterfaceItemIdentification for DRBurnSetupPanel {}
impl PNSAppearanceCustomization for DRBurnSetupPanel {}
impl PNSAccessibilityElement for DRBurnSetupPanel {}
impl PNSAccessibility for DRBurnSetupPanel {}
impl INSResponder for DRBurnSetupPanel {}
impl PNSCoding for DRBurnSetupPanel {}
impl INSObject for DRBurnSetupPanel {}
impl PNSObject for DRBurnSetupPanel {}
impl IDRBurnSetupPanel for DRBurnSetupPanel {}
pub trait IDRBurnSetupPanel: Sized + std::ops::Deref {
    unsafe fn setDefaultButtonTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultButtonTitle : title)
    }
    unsafe fn setCanSelectTestBurn_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanSelectTestBurn : flag)
    }
    unsafe fn setCanSelectAppendableMedia_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanSelectAppendableMedia : flag)
    }
    unsafe fn burnObject(&self) -> DRBurn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, burnObject)
    }
    unsafe fn expand_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, expand : sender)
    }
    unsafe fn burnSpeed_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, burnSpeed : sender)
    }
    unsafe fn appendable_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendable : sender)
    }
    unsafe fn completionAction_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completionAction : sender)
    }
    unsafe fn testBurn_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, testBurn : sender)
    }
    unsafe fn verifyBurn_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, verifyBurn : sender)
    }
    unsafe fn setupPanel() -> DRBurnSetupPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRBurnSetupPanel").unwrap(), setupPanel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DREraseSetupPanel(pub id);
impl std::ops::Deref for DREraseSetupPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DREraseSetupPanel {}
impl DREraseSetupPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DREraseSetupPanel").unwrap(), alloc) })
    }
}
impl IDRSetupPanel for DREraseSetupPanel {}
impl std::convert::TryFrom<DRSetupPanel> for DREraseSetupPanel {
    type Error = &'static str;
    fn try_from(parent: DRSetupPanel) -> Result<DREraseSetupPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DREraseSetupPanel").unwrap()) };
        if is_kind_of {
            Ok(DREraseSetupPanel(parent.0))
        } else {
            Err("This DRSetupPanel cannot be downcasted to DREraseSetupPanel")
        }
    }
}
impl INSPanel for DREraseSetupPanel {}
impl INSWindow for DREraseSetupPanel {}
impl PNSAnimatablePropertyContainer for DREraseSetupPanel {}
impl PNSMenuItemValidation for DREraseSetupPanel {}
impl PNSUserInterfaceValidations for DREraseSetupPanel {}
impl PNSUserInterfaceItemIdentification for DREraseSetupPanel {}
impl PNSAppearanceCustomization for DREraseSetupPanel {}
impl PNSAccessibilityElement for DREraseSetupPanel {}
impl PNSAccessibility for DREraseSetupPanel {}
impl INSResponder for DREraseSetupPanel {}
impl PNSCoding for DREraseSetupPanel {}
impl INSObject for DREraseSetupPanel {}
impl PNSObject for DREraseSetupPanel {}
impl IDREraseSetupPanel for DREraseSetupPanel {}
pub trait IDREraseSetupPanel: Sized + std::ops::Deref {
    unsafe fn eraseObject(&self) -> DRErase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eraseObject)
    }
    unsafe fn eraseType_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eraseType : sender)
    }
    unsafe fn setupPanel() -> DREraseSetupPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DREraseSetupPanel").unwrap(), setupPanel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRBurnProgressPanel(pub id);
impl std::ops::Deref for DRBurnProgressPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRBurnProgressPanel {}
impl DRBurnProgressPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRBurnProgressPanel").unwrap(), alloc) })
    }
}
impl INSPanel for DRBurnProgressPanel {}
impl std::convert::TryFrom<NSPanel> for DRBurnProgressPanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<DRBurnProgressPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRBurnProgressPanel").unwrap()) };
        if is_kind_of {
            Ok(DRBurnProgressPanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to DRBurnProgressPanel")
        }
    }
}
impl INSWindow for DRBurnProgressPanel {}
impl PNSAnimatablePropertyContainer for DRBurnProgressPanel {}
impl PNSMenuItemValidation for DRBurnProgressPanel {}
impl PNSUserInterfaceValidations for DRBurnProgressPanel {}
impl PNSUserInterfaceItemIdentification for DRBurnProgressPanel {}
impl PNSAppearanceCustomization for DRBurnProgressPanel {}
impl PNSAccessibilityElement for DRBurnProgressPanel {}
impl PNSAccessibility for DRBurnProgressPanel {}
impl INSResponder for DRBurnProgressPanel {}
impl PNSCoding for DRBurnProgressPanel {}
impl INSObject for DRBurnProgressPanel {}
impl PNSObject for DRBurnProgressPanel {}
impl IDRBurnProgressPanel for DRBurnProgressPanel {}
pub trait IDRBurnProgressPanel: Sized + std::ops::Deref {
    unsafe fn beginProgressSheetForBurn_layout_modalForWindow_(
        &self,
        burn: DRBurn,
        layout: id,
        docWindow: NSWindow,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginProgressSheetForBurn : burn, layout : layout, modalForWindow : docWindow)
    }
    unsafe fn beginProgressPanelForBurn_layout_(&self, burn: DRBurn, layout: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginProgressPanelForBurn : burn, layout : layout)
    }
    unsafe fn setDescription_(&self, description: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDescription : description)
    }
    unsafe fn description(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, description)
    }
    unsafe fn setVerboseProgressStatus_(&self, verbose: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVerboseProgressStatus : verbose)
    }
    unsafe fn verboseProgressStatus(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verboseProgressStatus)
    }
    unsafe fn stopBurn_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopBurn : sender)
    }
    unsafe fn progressPanel() -> DRBurnProgressPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRBurnProgressPanel").unwrap(), progressPanel)
    }
}
pub trait NSObject_DRBurnProgressPanelDelegateMethods: Sized + std::ops::Deref {
    unsafe fn burnProgressPanelWillBegin_(&self, aNotification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, burnProgressPanelWillBegin : aNotification)
    }
    unsafe fn burnProgressPanelDidFinish_(&self, aNotification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, burnProgressPanelDidFinish : aNotification)
    }
    unsafe fn burnProgressPanel_burnDidFinish_(
        &self,
        theBurnPanel: DRBurnProgressPanel,
        burn: DRBurn,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, burnProgressPanel : theBurnPanel, burnDidFinish : burn)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DREraseProgressPanel(pub id);
impl std::ops::Deref for DREraseProgressPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DREraseProgressPanel {}
impl DREraseProgressPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DREraseProgressPanel").unwrap(), alloc) })
    }
}
impl INSPanel for DREraseProgressPanel {}
impl std::convert::TryFrom<NSPanel> for DREraseProgressPanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<DREraseProgressPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DREraseProgressPanel").unwrap()) };
        if is_kind_of {
            Ok(DREraseProgressPanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to DREraseProgressPanel")
        }
    }
}
impl INSWindow for DREraseProgressPanel {}
impl PNSAnimatablePropertyContainer for DREraseProgressPanel {}
impl PNSMenuItemValidation for DREraseProgressPanel {}
impl PNSUserInterfaceValidations for DREraseProgressPanel {}
impl PNSUserInterfaceItemIdentification for DREraseProgressPanel {}
impl PNSAppearanceCustomization for DREraseProgressPanel {}
impl PNSAccessibilityElement for DREraseProgressPanel {}
impl PNSAccessibility for DREraseProgressPanel {}
impl INSResponder for DREraseProgressPanel {}
impl PNSCoding for DREraseProgressPanel {}
impl INSObject for DREraseProgressPanel {}
impl PNSObject for DREraseProgressPanel {}
impl IDREraseProgressPanel for DREraseProgressPanel {}
pub trait IDREraseProgressPanel: Sized + std::ops::Deref {
    unsafe fn beginProgressSheetForErase_modalForWindow_(&self, erase: DRErase, docWindow: NSWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginProgressSheetForErase : erase, modalForWindow : docWindow)
    }
    unsafe fn beginProgressPanelForErase_(&self, erase: DRErase)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginProgressPanelForErase : erase)
    }
    unsafe fn setDescription_(&self, description: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDescription : description)
    }
    unsafe fn description(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, description)
    }
    unsafe fn progressPanel() -> DREraseProgressPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DREraseProgressPanel").unwrap(), progressPanel)
    }
}
pub trait NSObject_DREraseProgressPanelDelegateMethods: Sized + std::ops::Deref {
    unsafe fn eraseProgressPanelWillBegin_(&self, aNotification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eraseProgressPanelWillBegin : aNotification)
    }
    unsafe fn eraseProgressPanelDidFinish_(&self, aNotification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eraseProgressPanelDidFinish : aNotification)
    }
    unsafe fn eraseProgressPanel_eraseDidFinish_(
        &self,
        theErasePanel: DREraseProgressPanel,
        erase: DRErase,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eraseProgressPanel : theErasePanel, eraseDidFinish : erase)
    }
}
unsafe extern "C" {
    pub fn DRBurnSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DRBurnSessionCreate() -> DRBurnSessionRef;
}
unsafe extern "C" {
    pub fn DRBurnSessionSetBurn(burnSession: DRBurnSessionRef, burn: DRBurnRef);
}
unsafe extern "C" {
    pub fn DRBurnSessionGetBurn(burnSession: DRBurnSessionRef) -> DRBurnRef;
}
unsafe extern "C" {
    pub fn DRBurnSessionSetupDialog(
        burnSession: DRBurnSessionRef,
        options: *mut DRBurnSessionSetupDialogOptions,
        setupCallbacks: *mut DRBurnSessionSetupCallbacks,
    ) -> SInt8;
}
unsafe extern "C" {
    pub fn DRBurnSessionBeginProgressDialog(
        burnSession: DRBurnSessionRef,
        layout: CFTypeRef,
        options: *mut DRBurnSessionProgressDialogOptions,
        progressCallbacks: *mut DRBurnSessionProgressCallbacks,
    );
}
unsafe extern "C" {
    pub fn DREraseSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DREraseSessionCreate() -> DREraseSessionRef;
}
unsafe extern "C" {
    pub fn DREraseSessionSetErase(eraseSession: DREraseSessionRef, erase: DREraseRef);
}
unsafe extern "C" {
    pub fn DREraseSessionGetErase(eraseSession: DREraseSessionRef) -> DREraseRef;
}
unsafe extern "C" {
    pub fn DREraseSessionSetupDialog(
        eraseSession: DREraseSessionRef,
        options: *mut DREraseSessionSetupDialogOptions,
        setupCallbacks: *mut DREraseSessionSetupCallbacks,
    ) -> SInt8;
}
unsafe extern "C" {
    pub fn DREraseSessionBeginProgressDialog(
        eraseSession: DREraseSessionRef,
        options: *mut DREraseSessionProgressDialogOptions,
        progressCallbacks: *mut DREraseSessionProgressCallbacks,
    );
}
unsafe extern "C" {
    pub static DRSetupPanelDeviceSelectionChangedNotification: NSString;
}
unsafe extern "C" {
    pub static DRSetupPanelSelectedDeviceKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnSetupPanelDefaultButtonDefaultTitle: NSString;
}
unsafe extern "C" {
    pub static DRBurnProgressPanelWillBeginNotification: NSString;
}
unsafe extern "C" {
    pub static DRBurnProgressPanelDidFinishNotification: NSString;
}
unsafe extern "C" {
    pub static DREraseProgressPanelWillBeginNotification: NSString;
}
unsafe extern "C" {
    pub static DREraseProgressPanelDidFinishNotification: NSString;
}
unsafe extern "C" {
    pub static DRBurnIcon: NSString;
}
unsafe extern "C" {
    pub static DREraseIcon: NSString;
}

unsafe impl objc2::encode::RefEncode for __DRBurnSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DRBurnSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DRBurnSession", &[]);
}
unsafe impl objc2::encode::RefEncode for DRBurnSessionSetupDialogOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRBurnSessionSetupDialogOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DRBurnSessionSetupDialogOptions", &[]);
}
unsafe impl objc2::encode::RefEncode for DRBurnSessionSetupCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRBurnSessionSetupCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DRBurnSessionSetupCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for DRBurnSessionProgressCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRBurnSessionProgressCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DRBurnSessionProgressCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for DRBurnSessionProgressDialogOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRBurnSessionProgressDialogOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DRBurnSessionProgressDialogOptions", &[]);
}
unsafe impl objc2::encode::RefEncode for __DREraseSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DREraseSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DREraseSession", &[]);
}
unsafe impl objc2::encode::RefEncode for DREraseSessionSetupDialogOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DREraseSessionSetupDialogOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DREraseSessionSetupDialogOptions", &[]);
}
unsafe impl objc2::encode::RefEncode for DREraseSessionSetupCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DREraseSessionSetupCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DREraseSessionSetupCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for DREraseSessionProgressCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DREraseSessionProgressCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DREraseSessionProgressCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for DREraseSessionProgressDialogOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DREraseSessionProgressDialogOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DREraseSessionProgressDialogOptions", &[]);
}
unsafe impl objc2::encode::RefEncode for DRSetupPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRSetupPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRBurnSetupPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRBurnSetupPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DREraseSetupPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DREraseSetupPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRBurnProgressPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRBurnProgressPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DREraseProgressPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DREraseProgressPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
