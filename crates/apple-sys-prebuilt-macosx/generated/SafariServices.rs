#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use crate::IOSurface::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SFErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFContentBlockerManager(pub id);
impl std::ops::Deref for SFContentBlockerManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFContentBlockerManager {}
impl SFContentBlockerManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFContentBlockerManager").unwrap(), alloc) })
    }
}
impl INSObject for SFContentBlockerManager {}
impl PNSObject for SFContentBlockerManager {}
impl std::convert::TryFrom<NSObject> for SFContentBlockerManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFContentBlockerManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFContentBlockerManager").unwrap()) };
        if is_kind_of {
            Ok(SFContentBlockerManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFContentBlockerManager")
        }
    }
}
impl ISFContentBlockerManager for SFContentBlockerManager {}
pub trait ISFContentBlockerManager: Sized + std::ops::Deref {
    unsafe fn reloadContentBlockerWithIdentifier_completionHandler_(
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFContentBlockerManager").unwrap(), reloadContentBlockerWithIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn getStateOfContentBlockerWithIdentifier_completionHandler_(
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFContentBlockerManager").unwrap(), getStateOfContentBlockerWithIdentifier : identifier, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFContentBlockerState(pub id);
impl std::ops::Deref for SFContentBlockerState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFContentBlockerState {}
impl SFContentBlockerState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFContentBlockerState").unwrap(), alloc) })
    }
}
impl INSObject for SFContentBlockerState {}
impl PNSObject for SFContentBlockerState {}
impl std::convert::TryFrom<NSObject> for SFContentBlockerState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFContentBlockerState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFContentBlockerState").unwrap()) };
        if is_kind_of {
            Ok(SFContentBlockerState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFContentBlockerState")
        }
    }
}
impl ISFContentBlockerState for SFContentBlockerState {}
pub trait ISFContentBlockerState: Sized + std::ops::Deref {
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariApplication(pub id);
impl std::ops::Deref for SFSafariApplication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariApplication {}
impl SFSafariApplication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap(), alloc) })
    }
}
impl INSObject for SFSafariApplication {}
impl PNSObject for SFSafariApplication {}
impl std::convert::TryFrom<NSObject> for SFSafariApplication {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariApplication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap()) };
        if is_kind_of {
            Ok(SFSafariApplication(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariApplication")
        }
    }
}
impl ISFSafariApplication for SFSafariApplication {}
pub trait ISFSafariApplication: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap(), new)
    }
    unsafe fn getActiveWindowWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap(), getActiveWindowWithCompletionHandler : completionHandler)
    }
    unsafe fn getAllWindowsWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap(), getAllWindowsWithCompletionHandler : completionHandler)
    }
    unsafe fn openWindowWithURL_completionHandler_(
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap(), openWindowWithURL : url, completionHandler : completionHandler)
    }
    unsafe fn setToolbarItemsNeedUpdate()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap(), setToolbarItemsNeedUpdate)
    }
    unsafe fn getHostApplicationWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap(), getHostApplicationWithCompletionHandler : completionHandler)
    }
    unsafe fn showPreferencesForExtensionWithIdentifier_completionHandler_(
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap(), showPreferencesForExtensionWithIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn dispatchMessageWithName_toExtensionWithIdentifier_userInfo_completionHandler_(
        messageName: NSString,
        identifier: NSString,
        userInfo: NSDictionary,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariApplication").unwrap(), dispatchMessageWithName : messageName, toExtensionWithIdentifier : identifier, userInfo : userInfo, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariExtension(pub id);
impl std::ops::Deref for SFSafariExtension {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariExtension {}
impl SFSafariExtension {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariExtension").unwrap(), alloc) })
    }
}
impl INSObject for SFSafariExtension {}
impl PNSObject for SFSafariExtension {}
impl std::convert::TryFrom<NSObject> for SFSafariExtension {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariExtension, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariExtension").unwrap()) };
        if is_kind_of {
            Ok(SFSafariExtension(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariExtension")
        }
    }
}
impl ISFSafariExtension for SFSafariExtension {}
pub trait ISFSafariExtension: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariExtension").unwrap(), new)
    }
    unsafe fn getBaseURIWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariExtension").unwrap(), getBaseURIWithCompletionHandler : completionHandler)
    }
}
pub trait PSFSafariExtensionHandling: Sized + std::ops::Deref {
    unsafe fn messageReceivedWithName_fromPage_userInfo_(
        &self,
        messageName: NSString,
        page: SFSafariPage,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, messageReceivedWithName : messageName, fromPage : page, userInfo : userInfo)
    }
    unsafe fn messageReceivedFromContainingAppWithName_userInfo_(
        &self,
        messageName: NSString,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, messageReceivedFromContainingAppWithName : messageName, userInfo : userInfo)
    }
    unsafe fn toolbarItemClickedInWindow_(&self, window: SFSafariWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toolbarItemClickedInWindow : window)
    }
    unsafe fn validateToolbarItemInWindow_validationHandler_(
        &self,
        window: SFSafariWindow,
        validationHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateToolbarItemInWindow : window, validationHandler : validationHandler)
    }
    unsafe fn contextMenuItemSelectedWithCommand_inPage_userInfo_(
        &self,
        command: NSString,
        page: SFSafariPage,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contextMenuItemSelectedWithCommand : command, inPage : page, userInfo : userInfo)
    }
    unsafe fn validateContextMenuItemWithCommand_inPage_userInfo_validationHandler_(
        &self,
        command: NSString,
        page: SFSafariPage,
        userInfo: NSDictionary,
        validationHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateContextMenuItemWithCommand : command, inPage : page, userInfo : userInfo, validationHandler : validationHandler)
    }
    unsafe fn popoverWillShowInWindow_(&self, window: SFSafariWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popoverWillShowInWindow : window)
    }
    unsafe fn popoverDidCloseInWindow_(&self, window: SFSafariWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popoverDidCloseInWindow : window)
    }
    unsafe fn popoverViewController(&self) -> SFSafariExtensionViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, popoverViewController)
    }
    unsafe fn additionalRequestHeadersForURL_completionHandler_(
        &self,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, additionalRequestHeadersForURL : url, completionHandler : completionHandler)
    }
    unsafe fn contentBlockerWithIdentifier_blockedResourcesWithURLs_onPage_(
        &self,
        contentBlockerIdentifier: NSString,
        urls: NSArray,
        page: SFSafariPage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentBlockerWithIdentifier : contentBlockerIdentifier, blockedResourcesWithURLs : urls, onPage : page)
    }
    unsafe fn page_willNavigateToURL_(&self, page: SFSafariPage, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, page : page, willNavigateToURL : url)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariExtensionHandler(pub id);
impl std::ops::Deref for SFSafariExtensionHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariExtensionHandler {}
impl SFSafariExtensionHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariExtensionHandler").unwrap(), alloc) })
    }
}
impl PNSExtensionRequestHandling for SFSafariExtensionHandler {}
impl PSFSafariExtensionHandling for SFSafariExtensionHandler {}
impl INSObject for SFSafariExtensionHandler {}
impl PNSObject for SFSafariExtensionHandler {}
impl std::convert::TryFrom<NSObject> for SFSafariExtensionHandler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariExtensionHandler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariExtensionHandler").unwrap()) };
        if is_kind_of {
            Ok(SFSafariExtensionHandler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariExtensionHandler")
        }
    }
}
impl ISFSafariExtensionHandler for SFSafariExtensionHandler {}
pub trait ISFSafariExtensionHandler: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariExtensionManager(pub id);
impl std::ops::Deref for SFSafariExtensionManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariExtensionManager {}
impl SFSafariExtensionManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariExtensionManager").unwrap(), alloc) })
    }
}
impl INSObject for SFSafariExtensionManager {}
impl PNSObject for SFSafariExtensionManager {}
impl std::convert::TryFrom<NSObject> for SFSafariExtensionManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariExtensionManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariExtensionManager").unwrap()) };
        if is_kind_of {
            Ok(SFSafariExtensionManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariExtensionManager")
        }
    }
}
impl ISFSafariExtensionManager for SFSafariExtensionManager {}
pub trait ISFSafariExtensionManager: Sized + std::ops::Deref {
    unsafe fn getStateOfSafariExtensionWithIdentifier_completionHandler_(
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariExtensionManager").unwrap(), getStateOfSafariExtensionWithIdentifier : identifier, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariExtensionState(pub id);
impl std::ops::Deref for SFSafariExtensionState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariExtensionState {}
impl SFSafariExtensionState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariExtensionState").unwrap(), alloc) })
    }
}
impl INSObject for SFSafariExtensionState {}
impl PNSObject for SFSafariExtensionState {}
impl std::convert::TryFrom<NSObject> for SFSafariExtensionState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariExtensionState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariExtensionState").unwrap()) };
        if is_kind_of {
            Ok(SFSafariExtensionState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariExtensionState")
        }
    }
}
impl ISFSafariExtensionState for SFSafariExtensionState {}
pub trait ISFSafariExtensionState: Sized + std::ops::Deref {
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariExtensionViewController(pub id);
impl std::ops::Deref for SFSafariExtensionViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariExtensionViewController {}
impl SFSafariExtensionViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariExtensionViewController").unwrap(), alloc) })
    }
}
impl INSViewController for SFSafariExtensionViewController {}
impl PNSEditor for SFSafariExtensionViewController {}
impl PNSSeguePerforming for SFSafariExtensionViewController {}
impl PNSUserInterfaceItemIdentification for SFSafariExtensionViewController {}
impl std::convert::TryFrom<NSViewController> for SFSafariExtensionViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<SFSafariExtensionViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariExtensionViewController").unwrap())
        };
        if is_kind_of {
            Ok(SFSafariExtensionViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to SFSafariExtensionViewController")
        }
    }
}
impl INSResponder for SFSafariExtensionViewController {}
impl PNSCoding for SFSafariExtensionViewController {}
impl INSObject for SFSafariExtensionViewController {}
impl PNSObject for SFSafariExtensionViewController {}
impl ISFSafariExtensionViewController for SFSafariExtensionViewController {}
pub trait ISFSafariExtensionViewController: Sized + std::ops::Deref {
    unsafe fn dismissPopover(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dismissPopover)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariPage(pub id);
impl std::ops::Deref for SFSafariPage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariPage {}
impl SFSafariPage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariPage").unwrap(), alloc) })
    }
}
impl PNSCopying for SFSafariPage {}
impl PNSSecureCoding for SFSafariPage {}
impl INSObject for SFSafariPage {}
impl PNSObject for SFSafariPage {}
impl std::convert::TryFrom<NSObject> for SFSafariPage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariPage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariPage").unwrap()) };
        if is_kind_of {
            Ok(SFSafariPage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariPage")
        }
    }
}
impl ISFSafariPage for SFSafariPage {}
pub trait ISFSafariPage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn dispatchMessageToScriptWithName_userInfo_(
        &self,
        messageName: NSString,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchMessageToScriptWithName : messageName, userInfo : userInfo)
    }
    unsafe fn reload(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reload)
    }
    unsafe fn getPagePropertiesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPagePropertiesWithCompletionHandler : completionHandler)
    }
    unsafe fn getContainingTabWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getContainingTabWithCompletionHandler : completionHandler)
    }
    unsafe fn getScreenshotOfVisibleAreaWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getScreenshotOfVisibleAreaWithCompletionHandler : completionHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariPage").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariPageProperties(pub id);
impl std::ops::Deref for SFSafariPageProperties {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariPageProperties {}
impl SFSafariPageProperties {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariPageProperties").unwrap(), alloc) })
    }
}
impl INSObject for SFSafariPageProperties {}
impl PNSObject for SFSafariPageProperties {}
impl std::convert::TryFrom<NSObject> for SFSafariPageProperties {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariPageProperties, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariPageProperties").unwrap()) };
        if is_kind_of {
            Ok(SFSafariPageProperties(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariPageProperties")
        }
    }
}
impl ISFSafariPageProperties for SFSafariPageProperties {}
pub trait ISFSafariPageProperties: Sized + std::ops::Deref {
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn usesPrivateBrowsing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesPrivateBrowsing)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariTab(pub id);
impl std::ops::Deref for SFSafariTab {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariTab {}
impl SFSafariTab {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariTab").unwrap(), alloc) })
    }
}
impl PNSCopying for SFSafariTab {}
impl PNSSecureCoding for SFSafariTab {}
impl INSObject for SFSafariTab {}
impl PNSObject for SFSafariTab {}
impl std::convert::TryFrom<NSObject> for SFSafariTab {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariTab, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariTab").unwrap()) };
        if is_kind_of {
            Ok(SFSafariTab(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariTab")
        }
    }
}
impl ISFSafariTab for SFSafariTab {}
pub trait ISFSafariTab: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn getActivePageWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getActivePageWithCompletionHandler : completionHandler)
    }
    unsafe fn getPagesWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPagesWithCompletionHandler : completionHandler)
    }
    unsafe fn getContainingWindowWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getContainingWindowWithCompletionHandler : completionHandler)
    }
    unsafe fn activateWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateWithCompletionHandler : completionHandler)
    }
    unsafe fn navigateToURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, navigateToURL : url)
    }
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariTab").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariToolbarItem(pub id);
impl std::ops::Deref for SFSafariToolbarItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariToolbarItem {}
impl SFSafariToolbarItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariToolbarItem").unwrap(), alloc) })
    }
}
impl PNSCopying for SFSafariToolbarItem {}
impl PNSSecureCoding for SFSafariToolbarItem {}
impl INSObject for SFSafariToolbarItem {}
impl PNSObject for SFSafariToolbarItem {}
impl std::convert::TryFrom<NSObject> for SFSafariToolbarItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariToolbarItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariToolbarItem").unwrap()) };
        if is_kind_of {
            Ok(SFSafariToolbarItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariToolbarItem")
        }
    }
}
impl ISFSafariToolbarItem for SFSafariToolbarItem {}
pub trait ISFSafariToolbarItem: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setEnabled_withBadgeText_(&self, enabled: BOOL, badgeText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled, withBadgeText : badgeText)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn setBadgeText_(&self, badgeText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBadgeText : badgeText)
    }
    unsafe fn setImage_(&self, image: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn showPopover(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showPopover)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariToolbarItem").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFSafariWindow(pub id);
impl std::ops::Deref for SFSafariWindow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFSafariWindow {}
impl SFSafariWindow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariWindow").unwrap(), alloc) })
    }
}
impl PNSCopying for SFSafariWindow {}
impl PNSSecureCoding for SFSafariWindow {}
impl INSObject for SFSafariWindow {}
impl PNSObject for SFSafariWindow {}
impl std::convert::TryFrom<NSObject> for SFSafariWindow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFSafariWindow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFSafariWindow").unwrap()) };
        if is_kind_of {
            Ok(SFSafariWindow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFSafariWindow")
        }
    }
}
impl ISFSafariWindow for SFSafariWindow {}
pub trait ISFSafariWindow: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn getActiveTabWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getActiveTabWithCompletionHandler : completionHandler)
    }
    unsafe fn getAllTabsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAllTabsWithCompletionHandler : completionHandler)
    }
    unsafe fn openTabWithURL_makeActiveIfPossible_completionHandler_(
        &self,
        url: NSURL,
        activateTab: BOOL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openTabWithURL : url, makeActiveIfPossible : activateTab, completionHandler : completionHandler)
    }
    unsafe fn getToolbarItemWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getToolbarItemWithCompletionHandler : completionHandler)
    }
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFSafariWindow").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFUniversalLink(pub id);
impl std::ops::Deref for SFUniversalLink {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFUniversalLink {}
impl SFUniversalLink {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFUniversalLink").unwrap(), alloc) })
    }
}
impl INSObject for SFUniversalLink {}
impl PNSObject for SFUniversalLink {}
impl std::convert::TryFrom<NSObject> for SFUniversalLink {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFUniversalLink, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFUniversalLink").unwrap()) };
        if is_kind_of {
            Ok(SFUniversalLink(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFUniversalLink")
        }
    }
}
impl ISFUniversalLink for SFUniversalLink {}
pub trait ISFUniversalLink: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithWebpageURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWebpageURL : url)
    }
    unsafe fn webpageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webpageURL)
    }
    unsafe fn applicationURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationURL)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFUniversalLink").unwrap(), new)
    }
}
pub type SFSafariServicesVersion = NSInteger;
unsafe extern "C" {
    pub static SFErrorDomain: NSString;
}
unsafe extern "C" {
    pub static SFExtensionMessageKey: NSString;
}
unsafe extern "C" {
    pub static SFExtensionProfileKey: NSString;
}
unsafe extern "C" {
    pub static mut _SFSafariServicesAvailable: *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub static mut _SFSafariServicesVersion: SFSafariServicesVersion;
}

unsafe impl objc2::encode::RefEncode for SFContentBlockerManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFContentBlockerManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFContentBlockerState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFContentBlockerState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariApplication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariApplication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariExtension {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariExtension {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariExtensionHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariExtensionHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariExtensionManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariExtensionManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariExtensionState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariExtensionState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariExtensionViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariExtensionViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariPage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariPage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariPageProperties {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariPageProperties {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariTab {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariTab {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariToolbarItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariToolbarItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFSafariWindow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFSafariWindow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFUniversalLink {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFUniversalLink {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
