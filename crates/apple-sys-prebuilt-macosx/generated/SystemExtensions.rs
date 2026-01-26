#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type OSSystemExtensionErrorCode = NSInteger;
pub type OSSystemExtensionReplacementAction = NSInteger;
pub type OSSystemExtensionRequestResult = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSSystemExtensionRequest(pub id);
impl std::ops::Deref for OSSystemExtensionRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSSystemExtensionRequest {}
impl OSSystemExtensionRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionRequest").unwrap(), alloc) })
    }
}
impl INSObject for OSSystemExtensionRequest {}
impl PNSObject for OSSystemExtensionRequest {}
impl std::convert::TryFrom<NSObject> for OSSystemExtensionRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSSystemExtensionRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSSystemExtensionRequest").unwrap()) };
        if is_kind_of {
            Ok(OSSystemExtensionRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSSystemExtensionRequest")
        }
    }
}
impl IOSSystemExtensionRequest for OSSystemExtensionRequest {}
pub trait IOSSystemExtensionRequest: Sized + std::ops::Deref {
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
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn activationRequestForExtension_queue_(
        identifier: NSString,
        queue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionRequest").unwrap(), activationRequestForExtension : identifier, queue : queue)
    }
    unsafe fn deactivationRequestForExtension_queue_(
        identifier: NSString,
        queue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionRequest").unwrap(), deactivationRequestForExtension : identifier, queue : queue)
    }
    unsafe fn propertiesRequestForExtension_queue_(
        identifier: NSString,
        queue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionRequest").unwrap(), propertiesRequestForExtension : identifier, queue : queue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSSystemExtensionProperties(pub id);
impl std::ops::Deref for OSSystemExtensionProperties {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSSystemExtensionProperties {}
impl OSSystemExtensionProperties {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionProperties").unwrap(), alloc) })
    }
}
impl INSObject for OSSystemExtensionProperties {}
impl PNSObject for OSSystemExtensionProperties {}
impl std::convert::TryFrom<NSObject> for OSSystemExtensionProperties {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSSystemExtensionProperties, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSSystemExtensionProperties").unwrap()) };
        if is_kind_of {
            Ok(OSSystemExtensionProperties(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSSystemExtensionProperties")
        }
    }
}
impl IOSSystemExtensionProperties for OSSystemExtensionProperties {}
pub trait IOSSystemExtensionProperties: Sized + std::ops::Deref {
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
    unsafe fn bundleVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleVersion)
    }
    unsafe fn bundleShortVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleShortVersion)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn isAwaitingUserApproval(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAwaitingUserApproval)
    }
    unsafe fn isUninstalling(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUninstalling)
    }
}
pub trait POSSystemExtensionRequestDelegate: Sized + std::ops::Deref {
    unsafe fn request_actionForReplacingExtension_withExtension_(
        &self,
        request: OSSystemExtensionRequest,
        existing: OSSystemExtensionProperties,
        ext: OSSystemExtensionProperties,
    ) -> OSSystemExtensionReplacementAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, request : request, actionForReplacingExtension : existing, withExtension : ext)
    }
    unsafe fn requestNeedsUserApproval_(&self, request: OSSystemExtensionRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestNeedsUserApproval : request)
    }
    unsafe fn request_didFinishWithResult_(
        &self,
        request: OSSystemExtensionRequest,
        result: OSSystemExtensionRequestResult,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, request : request, didFinishWithResult : result)
    }
    unsafe fn request_didFailWithError_(&self, request: OSSystemExtensionRequest, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, request : request, didFailWithError : error)
    }
    unsafe fn request_foundProperties_(
        &self,
        request: OSSystemExtensionRequest,
        properties: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, request : request, foundProperties : properties)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSSystemExtensionManager(pub id);
impl std::ops::Deref for OSSystemExtensionManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSSystemExtensionManager {}
impl OSSystemExtensionManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionManager").unwrap(), alloc) })
    }
}
impl INSObject for OSSystemExtensionManager {}
impl PNSObject for OSSystemExtensionManager {}
impl std::convert::TryFrom<NSObject> for OSSystemExtensionManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSSystemExtensionManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSSystemExtensionManager").unwrap()) };
        if is_kind_of {
            Ok(OSSystemExtensionManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSSystemExtensionManager")
        }
    }
}
impl IOSSystemExtensionManager for OSSystemExtensionManager {}
pub trait IOSSystemExtensionManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn submitRequest_(&self, request: OSSystemExtensionRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, submitRequest : request)
    }
    unsafe fn sharedManager() -> OSSystemExtensionManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionManager").unwrap(), sharedManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSSystemExtensionInfo(pub id);
impl std::ops::Deref for OSSystemExtensionInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSSystemExtensionInfo {}
impl OSSystemExtensionInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionInfo").unwrap(), alloc) })
    }
}
impl INSObject for OSSystemExtensionInfo {}
impl PNSObject for OSSystemExtensionInfo {}
impl std::convert::TryFrom<NSObject> for OSSystemExtensionInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSSystemExtensionInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSSystemExtensionInfo").unwrap()) };
        if is_kind_of {
            Ok(OSSystemExtensionInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSSystemExtensionInfo")
        }
    }
}
impl IOSSystemExtensionInfo for OSSystemExtensionInfo {}
pub trait IOSSystemExtensionInfo: Sized + std::ops::Deref {
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
    unsafe fn bundleVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleVersion)
    }
    unsafe fn bundleShortVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleShortVersion)
    }
}
pub trait POSSystemExtensionsWorkspaceObserver: Sized + std::ops::Deref {
    unsafe fn systemExtensionWillBecomeEnabled_(&self, systemExtensionInfo: OSSystemExtensionInfo)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, systemExtensionWillBecomeEnabled : systemExtensionInfo)
    }
    unsafe fn systemExtensionWillBecomeDisabled_(&self, systemExtensionInfo: OSSystemExtensionInfo)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, systemExtensionWillBecomeDisabled : systemExtensionInfo)
    }
    unsafe fn systemExtensionWillBecomeInactive_(&self, systemExtensionInfo: OSSystemExtensionInfo)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, systemExtensionWillBecomeInactive : systemExtensionInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSSystemExtensionsWorkspace(pub id);
impl std::ops::Deref for OSSystemExtensionsWorkspace {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSSystemExtensionsWorkspace {}
impl OSSystemExtensionsWorkspace {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionsWorkspace").unwrap(), alloc) })
    }
}
impl INSObject for OSSystemExtensionsWorkspace {}
impl PNSObject for OSSystemExtensionsWorkspace {}
impl std::convert::TryFrom<NSObject> for OSSystemExtensionsWorkspace {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSSystemExtensionsWorkspace, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSSystemExtensionsWorkspace").unwrap()) };
        if is_kind_of {
            Ok(OSSystemExtensionsWorkspace(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSSystemExtensionsWorkspace")
        }
    }
}
impl IOSSystemExtensionsWorkspace for OSSystemExtensionsWorkspace {}
pub trait IOSSystemExtensionsWorkspace: Sized + std::ops::Deref {
    unsafe fn addObserver_error_(&self, observer: *mut u64, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObserver : observer, error : error)
    }
    unsafe fn removeObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeObserver : observer)
    }
    unsafe fn systemExtensionsForApplicationWithBundleID_error_(
        &self,
        bundleID: NSString,
        out_error: *mut NSError,
    ) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, systemExtensionsForApplicationWithBundleID : bundleID, error : out_error)
    }
    unsafe fn sharedWorkspace() -> OSSystemExtensionsWorkspace
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSSystemExtensionsWorkspace").unwrap(), sharedWorkspace)
    }
}
unsafe extern "C" {
    pub static OSSystemExtensionErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static OSBundleUsageDescriptionKey: NSString;
}
unsafe extern "C" {
    pub static NSSystemExtensionUsageDescriptionKey: NSString;
}
unsafe extern "C" {
    pub static OSRelatedKernelExtensionKey: NSString;
}

unsafe impl objc2::encode::RefEncode for OSSystemExtensionRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSSystemExtensionRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSSystemExtensionProperties {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSSystemExtensionProperties {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSSystemExtensionManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSSystemExtensionManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSSystemExtensionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSSystemExtensionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSSystemExtensionsWorkspace {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSSystemExtensionsWorkspace {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
