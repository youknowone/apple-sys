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
pub struct FIFinderSyncController(pub id);
impl std::ops::Deref for FIFinderSyncController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FIFinderSyncController {}
impl FIFinderSyncController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FIFinderSyncController").unwrap(), alloc) })
    }
}
impl INSExtensionContext for FIFinderSyncController {}
impl std::convert::TryFrom<NSExtensionContext> for FIFinderSyncController {
    type Error = &'static str;
    fn try_from(parent: NSExtensionContext) -> Result<FIFinderSyncController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FIFinderSyncController").unwrap()) };
        if is_kind_of {
            Ok(FIFinderSyncController(parent.0))
        } else {
            Err("This NSExtensionContext cannot be downcasted to FIFinderSyncController")
        }
    }
}
impl INSObject for FIFinderSyncController {}
impl PNSObject for FIFinderSyncController {}
impl IFIFinderSyncController for FIFinderSyncController {}
pub trait IFIFinderSyncController: Sized + std::ops::Deref {
    unsafe fn setBadgeImage_label_forBadgeIdentifier_(
        &self,
        image: NSImage,
        label: NSString,
        badgeID: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBadgeImage : image, label : label, forBadgeIdentifier : badgeID)
    }
    unsafe fn setBadgeIdentifier_forURL_(&self, badgeID: NSString, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBadgeIdentifier : badgeID, forURL : url)
    }
    unsafe fn targetedURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetedURL)
    }
    unsafe fn selectedItemURLs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedItemURLs)
    }
    unsafe fn lastUsedDateForItemWithURL_(&self, itemURL: NSURL) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lastUsedDateForItemWithURL : itemURL)
    }
    unsafe fn setLastUsedDate_forItemWithURL_completion_(
        &self,
        lastUsedDate: NSDate,
        itemURL: NSURL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLastUsedDate : lastUsedDate, forItemWithURL : itemURL, completion : completion)
    }
    unsafe fn tagDataForItemWithURL_(&self, itemURL: NSURL) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tagDataForItemWithURL : itemURL)
    }
    unsafe fn setTagData_forItemWithURL_completion_(
        &self,
        tagData: NSData,
        itemURL: NSURL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTagData : tagData, forItemWithURL : itemURL, completion : completion)
    }
    unsafe fn directoryURLs(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, directoryURLs)
    }
    unsafe fn setDirectoryURLs_(&self, directoryURLs: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDirectoryURLs : directoryURLs)
    }
    unsafe fn defaultController() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FIFinderSyncController").unwrap(), defaultController)
    }
    unsafe fn showExtensionManagementInterface()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FIFinderSyncController").unwrap(), showExtensionManagementInterface)
    }
    unsafe fn isExtensionEnabled() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FIFinderSyncController").unwrap(), isExtensionEnabled)
    }
}
pub type FIMenuKind = NSUInteger;
pub trait PFIFinderSync: Sized + std::ops::Deref {
    unsafe fn menuForMenuKind_(&self, menu: FIMenuKind) -> NSMenu
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, menuForMenuKind : menu)
    }
    unsafe fn beginObservingDirectoryAtURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginObservingDirectoryAtURL : url)
    }
    unsafe fn endObservingDirectoryAtURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endObservingDirectoryAtURL : url)
    }
    unsafe fn requestBadgeIdentifierForURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestBadgeIdentifierForURL : url)
    }
    unsafe fn supportedServiceNamesForItemWithURL_(&self, itemURL: NSURL) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedServiceNamesForItemWithURL : itemURL)
    }
    unsafe fn makeListenerEndpointForServiceName_itemURL_andReturnError_(
        &self,
        serviceName: NSString,
        itemURL: NSURL,
        error: *mut NSError,
    ) -> NSXPCListenerEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeListenerEndpointForServiceName : serviceName, itemURL : itemURL, andReturnError : error)
    }
    unsafe fn valuesForAttributes_forItemWithURL_completion_(
        &self,
        attributes: NSArray,
        itemURL: NSURL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valuesForAttributes : attributes, forItemWithURL : itemURL, completion : completion)
    }
    unsafe fn toolbarItemName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toolbarItemName)
    }
    unsafe fn toolbarItemImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toolbarItemImage)
    }
    unsafe fn toolbarItemToolTip(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toolbarItemToolTip)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FIFinderSync(pub id);
impl std::ops::Deref for FIFinderSync {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FIFinderSync {}
impl FIFinderSync {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FIFinderSync").unwrap(), alloc) })
    }
}
impl PFIFinderSync for FIFinderSync {}
impl PNSExtensionRequestHandling for FIFinderSync {}
impl INSObject for FIFinderSync {}
impl PNSObject for FIFinderSync {}
impl std::convert::TryFrom<NSObject> for FIFinderSync {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FIFinderSync, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FIFinderSync").unwrap()) };
        if is_kind_of {
            Ok(FIFinderSync(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FIFinderSync")
        }
    }
}
impl IFIFinderSync for FIFinderSync {}
pub trait IFIFinderSync: Sized + std::ops::Deref {}

unsafe impl objc2::encode::RefEncode for FIFinderSyncController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FIFinderSyncController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FIFinderSync {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FIFinderSync {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
