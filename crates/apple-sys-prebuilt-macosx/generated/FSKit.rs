#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use libc::off_t;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: ::std::os::raw::c_long,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSModuleIdentity(pub id);
impl std::ops::Deref for FSModuleIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSModuleIdentity {}
impl FSModuleIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSModuleIdentity").unwrap(), alloc) })
    }
}
impl INSObject for FSModuleIdentity {}
impl PNSObject for FSModuleIdentity {}
impl std::convert::TryFrom<NSObject> for FSModuleIdentity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSModuleIdentity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSModuleIdentity").unwrap()) };
        if is_kind_of {
            Ok(FSModuleIdentity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSModuleIdentity")
        }
    }
}
impl IFSModuleIdentity for FSModuleIdentity {}
pub trait IFSModuleIdentity: Sized + std::ops::Deref {
    unsafe fn bundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIdentifier)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSClient(pub id);
impl std::ops::Deref for FSClient {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSClient {}
impl FSClient {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSClient").unwrap(), alloc) })
    }
}
impl INSObject for FSClient {}
impl PNSObject for FSClient {}
impl std::convert::TryFrom<NSObject> for FSClient {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSClient, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSClient").unwrap()) };
        if is_kind_of {
            Ok(FSClient(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSClient")
        }
    }
}
impl IFSClient for FSClient {}
pub trait IFSClient: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fetchInstalledExtensionsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchInstalledExtensionsWithCompletionHandler : completionHandler)
    }
    unsafe fn sharedInstance() -> FSClient
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSClient").unwrap(), sharedInstance)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSEntityIdentifier(pub id);
impl std::ops::Deref for FSEntityIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSEntityIdentifier {}
impl FSEntityIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSEntityIdentifier").unwrap(), alloc) })
    }
}
impl PNSCopying for FSEntityIdentifier {}
impl PNSSecureCoding for FSEntityIdentifier {}
impl INSObject for FSEntityIdentifier {}
impl PNSObject for FSEntityIdentifier {}
impl std::convert::TryFrom<NSObject> for FSEntityIdentifier {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSEntityIdentifier, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSEntityIdentifier").unwrap()) };
        if is_kind_of {
            Ok(FSEntityIdentifier(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSEntityIdentifier")
        }
    }
}
impl IFSEntityIdentifier for FSEntityIdentifier {}
pub trait IFSEntityIdentifier: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithUUID_(&self, uuid: NSUUID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID : uuid)
    }
    unsafe fn initWithUUID_qualifier_(&self, uuid: NSUUID, qualifier: u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID : uuid, qualifier : qualifier)
    }
    unsafe fn initWithUUID_data_(&self, uuid: NSUUID, qualifierData: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID : uuid, data : qualifierData)
    }
    unsafe fn uuid(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uuid)
    }
    unsafe fn setUuid_(&self, uuid: NSUUID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUuid : uuid)
    }
    unsafe fn qualifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, qualifier)
    }
    unsafe fn setQualifier_(&self, qualifier: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQualifier : qualifier)
    }
}
pub type FSContainerState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSContainerStatus(pub id);
impl std::ops::Deref for FSContainerStatus {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSContainerStatus {}
impl FSContainerStatus {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSContainerStatus").unwrap(), alloc) })
    }
}
impl PNSCopying for FSContainerStatus {}
impl INSObject for FSContainerStatus {}
impl PNSObject for FSContainerStatus {}
impl std::convert::TryFrom<NSObject> for FSContainerStatus {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSContainerStatus, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSContainerStatus").unwrap()) };
        if is_kind_of {
            Ok(FSContainerStatus(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSContainerStatus")
        }
    }
}
impl IFSContainerStatus for FSContainerStatus {}
pub trait IFSContainerStatus: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn state(&self) -> FSContainerState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn status(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn activeWithStatus_(errorStatus: NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSContainerStatus").unwrap(), activeWithStatus : errorStatus)
    }
    unsafe fn blockedWithStatus_(errorStatus: NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSContainerStatus").unwrap(), blockedWithStatus : errorStatus)
    }
    unsafe fn notReadyWithStatus_(errorStatus: NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSContainerStatus").unwrap(), notReadyWithStatus : errorStatus)
    }
    unsafe fn readyWithStatus_(errorStatus: NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSContainerStatus").unwrap(), readyWithStatus : errorStatus)
    }
    unsafe fn active() -> FSContainerStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSContainerStatus").unwrap(), active)
    }
    unsafe fn ready() -> FSContainerStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSContainerStatus").unwrap(), ready)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSContainerIdentifier(pub id);
impl std::ops::Deref for FSContainerIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSContainerIdentifier {}
impl FSContainerIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSContainerIdentifier").unwrap(), alloc) })
    }
}
impl IFSEntityIdentifier for FSContainerIdentifier {}
impl PNSCopying for FSContainerIdentifier {}
impl PNSSecureCoding for FSContainerIdentifier {}
impl From<FSContainerIdentifier> for FSEntityIdentifier {
    fn from(child: FSContainerIdentifier) -> FSEntityIdentifier {
        FSEntityIdentifier(child.0)
    }
}
impl std::convert::TryFrom<FSEntityIdentifier> for FSContainerIdentifier {
    type Error = &'static str;
    fn try_from(parent: FSEntityIdentifier) -> Result<FSContainerIdentifier, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSContainerIdentifier").unwrap()) };
        if is_kind_of {
            Ok(FSContainerIdentifier(parent.0))
        } else {
            Err("This FSEntityIdentifier cannot be downcasted to FSContainerIdentifier")
        }
    }
}
impl INSObject for FSContainerIdentifier {}
impl PNSObject for FSContainerIdentifier {}
impl IFSContainerIdentifier for FSContainerIdentifier {}
pub trait IFSContainerIdentifier: Sized + std::ops::Deref {
    unsafe fn volumeIdentifier(&self) -> FSVolumeIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volumeIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSMutableFileDataBuffer(pub id);
impl std::ops::Deref for FSMutableFileDataBuffer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSMutableFileDataBuffer {}
impl FSMutableFileDataBuffer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSMutableFileDataBuffer").unwrap(), alloc) })
    }
}
impl INSObject for FSMutableFileDataBuffer {}
impl PNSObject for FSMutableFileDataBuffer {}
impl std::convert::TryFrom<NSObject> for FSMutableFileDataBuffer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSMutableFileDataBuffer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSMutableFileDataBuffer").unwrap()) };
        if is_kind_of {
            Ok(FSMutableFileDataBuffer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSMutableFileDataBuffer")
        }
    }
}
impl IFSMutableFileDataBuffer for FSMutableFileDataBuffer {}
pub trait IFSMutableFileDataBuffer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn mutableBytes(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mutableBytes)
    }
    unsafe fn length(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSFileName(pub id);
impl std::ops::Deref for FSFileName {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSFileName {}
impl FSFileName {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSFileName").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for FSFileName {}
impl PNSCopying for FSFileName {}
impl INSObject for FSFileName {}
impl PNSObject for FSFileName {}
impl std::convert::TryFrom<NSObject> for FSFileName {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSFileName, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSFileName").unwrap()) };
        if is_kind_of {
            Ok(FSFileName(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSFileName")
        }
    }
}
impl IFSFileName for FSFileName {}
pub trait IFSFileName: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCString_(&self, name: *const ::std::os::raw::c_char) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCString : name)
    }
    unsafe fn initWithBytes_length_(
        &self,
        bytes: *const ::std::os::raw::c_char,
        length: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBytes : bytes, length : length)
    }
    unsafe fn initWithData_(&self, name: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : name)
    }
    unsafe fn initWithString_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithString : name)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn debugDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, debugDescription)
    }
    unsafe fn nameWithCString_(name: *const ::std::os::raw::c_char) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSFileName").unwrap(), nameWithCString : name)
    }
    unsafe fn nameWithBytes_length_(
        bytes: *const ::std::os::raw::c_char,
        length: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSFileName").unwrap(), nameWithBytes : bytes, length : length)
    }
    unsafe fn nameWithData_(name: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSFileName").unwrap(), nameWithData : name)
    }
    unsafe fn nameWithString_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSFileName").unwrap(), nameWithString : name)
    }
}
pub type FSItemAttribute = NSInteger;
pub type FSItemType = NSInteger;
pub type FSItemID = UInt64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSItem(pub id);
impl std::ops::Deref for FSItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSItem {}
impl FSItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSItem").unwrap(), alloc) })
    }
}
impl INSObject for FSItem {}
impl PNSObject for FSItem {}
impl std::convert::TryFrom<NSObject> for FSItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSItem, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSItem").unwrap()) };
        if is_kind_of {
            Ok(FSItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSItem")
        }
    }
}
impl IFSItem for FSItem {}
pub trait IFSItem: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSItemAttributes(pub id);
impl std::ops::Deref for FSItemAttributes {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSItemAttributes {}
impl FSItemAttributes {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSItemAttributes").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for FSItemAttributes {}
impl INSObject for FSItemAttributes {}
impl PNSObject for FSItemAttributes {}
impl std::convert::TryFrom<NSObject> for FSItemAttributes {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSItemAttributes, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSItemAttributes").unwrap()) };
        if is_kind_of {
            Ok(FSItemAttributes(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSItemAttributes")
        }
    }
}
impl IFSItemAttributes for FSItemAttributes {}
pub trait IFSItemAttributes: Sized + std::ops::Deref {
    unsafe fn invalidateAllProperties(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidateAllProperties)
    }
    unsafe fn isValid_(&self, attribute: FSItemAttribute) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isValid : attribute)
    }
    unsafe fn uid(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uid)
    }
    unsafe fn setUid_(&self, uid: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUid : uid)
    }
    unsafe fn gid(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gid)
    }
    unsafe fn setGid_(&self, gid: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGid : gid)
    }
    unsafe fn mode(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn setMode_(&self, mode: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMode : mode)
    }
    unsafe fn type_(&self) -> FSItemType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: FSItemType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn linkCount(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkCount)
    }
    unsafe fn setLinkCount_(&self, linkCount: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinkCount : linkCount)
    }
    unsafe fn flags(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flags)
    }
    unsafe fn setFlags_(&self, flags: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlags : flags)
    }
    unsafe fn size(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn allocSize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocSize)
    }
    unsafe fn setAllocSize_(&self, allocSize: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllocSize : allocSize)
    }
    unsafe fn fileID(&self) -> FSItemID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileID)
    }
    unsafe fn setFileID_(&self, fileID: FSItemID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFileID : fileID)
    }
    unsafe fn parentID(&self) -> FSItemID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentID)
    }
    unsafe fn setParentID_(&self, parentID: FSItemID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParentID : parentID)
    }
    unsafe fn supportsLimitedXAttrs(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsLimitedXAttrs)
    }
    unsafe fn setSupportsLimitedXAttrs_(&self, supportsLimitedXAttrs: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsLimitedXAttrs : supportsLimitedXAttrs)
    }
    unsafe fn inhibitKernelOffloadedIO(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inhibitKernelOffloadedIO)
    }
    unsafe fn setInhibitKernelOffloadedIO_(&self, inhibitKernelOffloadedIO: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInhibitKernelOffloadedIO : inhibitKernelOffloadedIO)
    }
    unsafe fn modifyTime(&self) -> timespec
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifyTime)
    }
    unsafe fn setModifyTime_(&self, modifyTime: timespec)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModifyTime : modifyTime)
    }
    unsafe fn addedTime(&self) -> timespec
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addedTime)
    }
    unsafe fn setAddedTime_(&self, addedTime: timespec)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddedTime : addedTime)
    }
    unsafe fn changeTime(&self) -> timespec
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeTime)
    }
    unsafe fn setChangeTime_(&self, changeTime: timespec)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChangeTime : changeTime)
    }
    unsafe fn accessTime(&self) -> timespec
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessTime)
    }
    unsafe fn setAccessTime_(&self, accessTime: timespec)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessTime : accessTime)
    }
    unsafe fn birthTime(&self) -> timespec
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthTime)
    }
    unsafe fn setBirthTime_(&self, birthTime: timespec)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBirthTime : birthTime)
    }
    unsafe fn backupTime(&self) -> timespec
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backupTime)
    }
    unsafe fn setBackupTime_(&self, backupTime: timespec)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackupTime : backupTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSItemSetAttributesRequest(pub id);
impl std::ops::Deref for FSItemSetAttributesRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSItemSetAttributesRequest {}
impl FSItemSetAttributesRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSItemSetAttributesRequest").unwrap(), alloc) })
    }
}
impl IFSItemAttributes for FSItemSetAttributesRequest {}
impl PNSSecureCoding for FSItemSetAttributesRequest {}
impl From<FSItemSetAttributesRequest> for FSItemAttributes {
    fn from(child: FSItemSetAttributesRequest) -> FSItemAttributes {
        FSItemAttributes(child.0)
    }
}
impl std::convert::TryFrom<FSItemAttributes> for FSItemSetAttributesRequest {
    type Error = &'static str;
    fn try_from(parent: FSItemAttributes) -> Result<FSItemSetAttributesRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSItemSetAttributesRequest").unwrap()) };
        if is_kind_of {
            Ok(FSItemSetAttributesRequest(parent.0))
        } else {
            Err("This FSItemAttributes cannot be downcasted to FSItemSetAttributesRequest")
        }
    }
}
impl INSObject for FSItemSetAttributesRequest {}
impl PNSObject for FSItemSetAttributesRequest {}
impl IFSItemSetAttributesRequest for FSItemSetAttributesRequest {}
pub trait IFSItemSetAttributesRequest: Sized + std::ops::Deref {
    unsafe fn wasAttributeConsumed_(&self, attribute: FSItemAttribute) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, wasAttributeConsumed : attribute)
    }
    unsafe fn consumedAttributes(&self) -> FSItemAttribute
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, consumedAttributes)
    }
    unsafe fn setConsumedAttributes_(&self, consumedAttributes: FSItemAttribute)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConsumedAttributes : consumedAttributes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSItemGetAttributesRequest(pub id);
impl std::ops::Deref for FSItemGetAttributesRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSItemGetAttributesRequest {}
impl FSItemGetAttributesRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSItemGetAttributesRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for FSItemGetAttributesRequest {}
impl INSObject for FSItemGetAttributesRequest {}
impl PNSObject for FSItemGetAttributesRequest {}
impl std::convert::TryFrom<NSObject> for FSItemGetAttributesRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSItemGetAttributesRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSItemGetAttributesRequest").unwrap()) };
        if is_kind_of {
            Ok(FSItemGetAttributesRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSItemGetAttributesRequest")
        }
    }
}
impl IFSItemGetAttributesRequest for FSItemGetAttributesRequest {}
pub trait IFSItemGetAttributesRequest: Sized + std::ops::Deref {
    unsafe fn isAttributeWanted_(&self, attribute: FSItemAttribute) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isAttributeWanted : attribute)
    }
    unsafe fn wantedAttributes(&self) -> FSItemAttribute
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantedAttributes)
    }
    unsafe fn setWantedAttributes_(&self, wantedAttributes: FSItemAttribute)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantedAttributes : wantedAttributes)
    }
}
pub type FSErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSTask(pub id);
impl std::ops::Deref for FSTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSTask {}
impl FSTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSTask").unwrap(), alloc) })
    }
}
impl INSObject for FSTask {}
impl PNSObject for FSTask {}
impl std::convert::TryFrom<NSObject> for FSTask {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSTask, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSTask").unwrap()) };
        if is_kind_of {
            Ok(FSTask(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSTask")
        }
    }
}
impl IFSTask for FSTask {}
pub trait IFSTask: Sized + std::ops::Deref {
    unsafe fn logMessage_(&self, str_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logMessage : str_)
    }
    unsafe fn didCompleteWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didCompleteWithError : error)
    }
    unsafe fn cancellationHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancellationHandler)
    }
    unsafe fn setCancellationHandler_(&self, cancellationHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCancellationHandler : cancellationHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSTaskOptions(pub id);
impl std::ops::Deref for FSTaskOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSTaskOptions {}
impl FSTaskOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSTaskOptions").unwrap(), alloc) })
    }
}
impl INSObject for FSTaskOptions {}
impl PNSObject for FSTaskOptions {}
impl std::convert::TryFrom<NSObject> for FSTaskOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSTaskOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSTaskOptions").unwrap()) };
        if is_kind_of {
            Ok(FSTaskOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSTaskOptions")
        }
    }
}
impl IFSTaskOptions for FSTaskOptions {}
pub trait IFSTaskOptions: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn urlForOption_(&self, option: NSString) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, urlForOption : option)
    }
    unsafe fn taskOptions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, taskOptions)
    }
}
pub type FSMatchResult = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSResource(pub id);
impl std::ops::Deref for FSResource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSResource {}
impl FSResource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSResource").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for FSResource {}
impl INSObject for FSResource {}
impl PNSObject for FSResource {}
impl std::convert::TryFrom<NSObject> for FSResource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSResource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSResource").unwrap()) };
        if is_kind_of {
            Ok(FSResource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSResource")
        }
    }
}
impl IFSResource for FSResource {}
pub trait IFSResource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn makeProxy(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, makeProxy)
    }
    unsafe fn revoke(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revoke)
    }
    unsafe fn isRevoked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRevoked)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSMetadataRange(pub id);
impl std::ops::Deref for FSMetadataRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSMetadataRange {}
impl FSMetadataRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSMetadataRange").unwrap(), alloc) })
    }
}
impl INSObject for FSMetadataRange {}
impl PNSObject for FSMetadataRange {}
impl std::convert::TryFrom<NSObject> for FSMetadataRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSMetadataRange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSMetadataRange").unwrap()) };
        if is_kind_of {
            Ok(FSMetadataRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSMetadataRange")
        }
    }
}
impl IFSMetadataRange for FSMetadataRange {}
pub trait IFSMetadataRange: Sized + std::ops::Deref {
    unsafe fn initWithOffset_segmentLength_segmentCount_(
        &self,
        startOffset: off_t,
        segmentLength: u64,
        segmentCount: u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOffset : startOffset, segmentLength : segmentLength, segmentCount : segmentCount)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn startOffset(&self) -> off_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startOffset)
    }
    unsafe fn segmentLength(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentLength)
    }
    unsafe fn segmentCount(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentCount)
    }
    unsafe fn rangeWithOffset_segmentLength_segmentCount_(
        startOffset: off_t,
        segmentLength: u64,
        segmentCount: u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSMetadataRange").unwrap(), rangeWithOffset : startOffset, segmentLength : segmentLength, segmentCount : segmentCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSBlockDeviceResource(pub id);
impl std::ops::Deref for FSBlockDeviceResource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSBlockDeviceResource {}
impl FSBlockDeviceResource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSBlockDeviceResource").unwrap(), alloc) })
    }
}
impl IFSResource for FSBlockDeviceResource {}
impl PNSSecureCoding for FSBlockDeviceResource {}
impl From<FSBlockDeviceResource> for FSResource {
    fn from(child: FSBlockDeviceResource) -> FSResource {
        FSResource(child.0)
    }
}
impl std::convert::TryFrom<FSResource> for FSBlockDeviceResource {
    type Error = &'static str;
    fn try_from(parent: FSResource) -> Result<FSBlockDeviceResource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSBlockDeviceResource").unwrap()) };
        if is_kind_of {
            Ok(FSBlockDeviceResource(parent.0))
        } else {
            Err("This FSResource cannot be downcasted to FSBlockDeviceResource")
        }
    }
}
impl INSObject for FSBlockDeviceResource {}
impl PNSObject for FSBlockDeviceResource {}
impl IFSBlockDeviceResource for FSBlockDeviceResource {}
pub trait IFSBlockDeviceResource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn readInto_startingAt_length_completionHandler_(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        offset: off_t,
        length: usize,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readInto : buffer, startingAt : offset, length : length, completionHandler : completionHandler)
    }
    unsafe fn readInto_startingAt_length_error_(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        offset: off_t,
        length: usize,
        error: *mut NSError,
    ) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readInto : buffer, startingAt : offset, length : length, error : error)
    }
    unsafe fn writeFrom_startingAt_length_completionHandler_(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        offset: off_t,
        length: usize,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeFrom : buffer, startingAt : offset, length : length, completionHandler : completionHandler)
    }
    unsafe fn writeFrom_startingAt_length_error_(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        offset: off_t,
        length: usize,
        error: *mut NSError,
    ) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeFrom : buffer, startingAt : offset, length : length, error : error)
    }
    unsafe fn metadataReadInto_startingAt_length_error_(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        offset: off_t,
        length: usize,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, metadataReadInto : buffer, startingAt : offset, length : length, error : error)
    }
    unsafe fn metadataWriteFrom_startingAt_length_error_(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        offset: off_t,
        length: usize,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, metadataWriteFrom : buffer, startingAt : offset, length : length, error : error)
    }
    unsafe fn delayedMetadataWriteFrom_startingAt_length_error_(
        &self,
        buffer: *mut ::std::os::raw::c_void,
        offset: off_t,
        length: usize,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, delayedMetadataWriteFrom : buffer, startingAt : offset, length : length, error : error)
    }
    unsafe fn metadataFlushWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, metadataFlushWithError : error)
    }
    unsafe fn asynchronousMetadataFlushWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, asynchronousMetadataFlushWithError : error)
    }
    unsafe fn metadataClear_withDelayedWrites_error_(
        &self,
        rangesToClear: NSArray,
        withDelayedWrites: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, metadataClear : rangesToClear, withDelayedWrites : withDelayedWrites, error : error)
    }
    unsafe fn metadataPurge_error_(&self, rangesToPurge: NSArray, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, metadataPurge : rangesToPurge, error : error)
    }
    unsafe fn BSDName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, BSDName)
    }
    unsafe fn isWritable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isWritable)
    }
    unsafe fn blockSize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blockSize)
    }
    unsafe fn blockCount(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blockCount)
    }
    unsafe fn physicalBlockSize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicalBlockSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSGenericURLResource(pub id);
impl std::ops::Deref for FSGenericURLResource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSGenericURLResource {}
impl FSGenericURLResource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSGenericURLResource").unwrap(), alloc) })
    }
}
impl IFSResource for FSGenericURLResource {}
impl PNSSecureCoding for FSGenericURLResource {}
impl std::convert::TryFrom<FSResource> for FSGenericURLResource {
    type Error = &'static str;
    fn try_from(parent: FSResource) -> Result<FSGenericURLResource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSGenericURLResource").unwrap()) };
        if is_kind_of {
            Ok(FSGenericURLResource(parent.0))
        } else {
            Err("This FSResource cannot be downcasted to FSGenericURLResource")
        }
    }
}
impl INSObject for FSGenericURLResource {}
impl PNSObject for FSGenericURLResource {}
impl IFSGenericURLResource for FSGenericURLResource {}
pub trait IFSGenericURLResource: Sized + std::ops::Deref {
    unsafe fn initWithURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSPathURLResource(pub id);
impl std::ops::Deref for FSPathURLResource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSPathURLResource {}
impl FSPathURLResource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSPathURLResource").unwrap(), alloc) })
    }
}
impl IFSResource for FSPathURLResource {}
impl PNSSecureCoding for FSPathURLResource {}
impl std::convert::TryFrom<FSResource> for FSPathURLResource {
    type Error = &'static str;
    fn try_from(parent: FSResource) -> Result<FSPathURLResource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSPathURLResource").unwrap()) };
        if is_kind_of {
            Ok(FSPathURLResource(parent.0))
        } else {
            Err("This FSResource cannot be downcasted to FSPathURLResource")
        }
    }
}
impl INSObject for FSPathURLResource {}
impl PNSObject for FSPathURLResource {}
impl IFSPathURLResource for FSPathURLResource {}
pub trait IFSPathURLResource: Sized + std::ops::Deref {
    unsafe fn initWithURL_writable_(&self, URL: NSURL, writable: BOOL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL, writable : writable)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn isWritable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isWritable)
    }
}
pub trait PFSManageableResourceMaintenanceOperations: Sized + std::ops::Deref {
    unsafe fn startCheckWithTask_options_error_(
        &self,
        task: FSTask,
        options: FSTaskOptions,
        error: *mut NSError,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startCheckWithTask : task, options : options, error : error)
    }
    unsafe fn startFormatWithTask_options_error_(
        &self,
        task: FSTask,
        options: FSTaskOptions,
        error: *mut NSError,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startFormatWithTask : task, options : options, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSProbeResult(pub id);
impl std::ops::Deref for FSProbeResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSProbeResult {}
impl FSProbeResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSProbeResult").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for FSProbeResult {}
impl INSObject for FSProbeResult {}
impl PNSObject for FSProbeResult {}
impl std::convert::TryFrom<NSObject> for FSProbeResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSProbeResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSProbeResult").unwrap()) };
        if is_kind_of {
            Ok(FSProbeResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSProbeResult")
        }
    }
}
impl IFSProbeResult for FSProbeResult {}
pub trait IFSProbeResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn result(&self) -> FSMatchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, result)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn containerID(&self) -> FSContainerIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerID)
    }
    unsafe fn recognizedProbeResultWithName_containerID_(
        name: NSString,
        containerID: FSContainerIdentifier,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSProbeResult").unwrap(), recognizedProbeResultWithName : name, containerID : containerID)
    }
    unsafe fn usableButLimitedProbeResultWithName_containerID_(
        name: NSString,
        containerID: FSContainerIdentifier,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSProbeResult").unwrap(), usableButLimitedProbeResultWithName : name, containerID : containerID)
    }
    unsafe fn usableProbeResultWithName_containerID_(
        name: NSString,
        containerID: FSContainerIdentifier,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSProbeResult").unwrap(), usableProbeResultWithName : name, containerID : containerID)
    }
    unsafe fn notRecognizedProbeResult() -> FSProbeResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSProbeResult").unwrap(), notRecognizedProbeResult)
    }
    unsafe fn usableButLimitedProbeResult() -> FSProbeResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"FSProbeResult").unwrap(), usableButLimitedProbeResult)
    }
}
pub type FSDirectoryCookie = u64;
pub type FSDirectoryVerifier = u64;
pub type FSDeactivateOptions = NSInteger;
pub type FSSyncFlags = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSVolumeIdentifier(pub id);
impl std::ops::Deref for FSVolumeIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSVolumeIdentifier {}
impl FSVolumeIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSVolumeIdentifier").unwrap(), alloc) })
    }
}
impl IFSEntityIdentifier for FSVolumeIdentifier {}
impl PNSCopying for FSVolumeIdentifier {}
impl PNSSecureCoding for FSVolumeIdentifier {}
impl std::convert::TryFrom<FSEntityIdentifier> for FSVolumeIdentifier {
    type Error = &'static str;
    fn try_from(parent: FSEntityIdentifier) -> Result<FSVolumeIdentifier, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSVolumeIdentifier").unwrap()) };
        if is_kind_of {
            Ok(FSVolumeIdentifier(parent.0))
        } else {
            Err("This FSEntityIdentifier cannot be downcasted to FSVolumeIdentifier")
        }
    }
}
impl INSObject for FSVolumeIdentifier {}
impl PNSObject for FSVolumeIdentifier {}
impl IFSVolumeIdentifier for FSVolumeIdentifier {}
pub trait IFSVolumeIdentifier: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSDirectoryEntryPacker(pub id);
impl std::ops::Deref for FSDirectoryEntryPacker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSDirectoryEntryPacker {}
impl FSDirectoryEntryPacker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSDirectoryEntryPacker").unwrap(), alloc) })
    }
}
impl INSObject for FSDirectoryEntryPacker {}
impl PNSObject for FSDirectoryEntryPacker {}
impl std::convert::TryFrom<NSObject> for FSDirectoryEntryPacker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSDirectoryEntryPacker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSDirectoryEntryPacker").unwrap()) };
        if is_kind_of {
            Ok(FSDirectoryEntryPacker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSDirectoryEntryPacker")
        }
    }
}
impl IFSDirectoryEntryPacker for FSDirectoryEntryPacker {}
pub trait IFSDirectoryEntryPacker: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn packEntryWithName_itemType_itemID_nextCookie_attributes_(
        &self,
        name: FSFileName,
        itemType: FSItemType,
        itemID: FSItemID,
        nextCookie: FSDirectoryCookie,
        attributes: FSItemAttributes,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, packEntryWithName : name, itemType : itemType, itemID : itemID, nextCookie : nextCookie, attributes : attributes)
    }
}
pub type FSVolumeCaseFormat = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSVolumeSupportedCapabilities(pub id);
impl std::ops::Deref for FSVolumeSupportedCapabilities {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSVolumeSupportedCapabilities {}
impl FSVolumeSupportedCapabilities {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSVolumeSupportedCapabilities").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for FSVolumeSupportedCapabilities {}
impl INSObject for FSVolumeSupportedCapabilities {}
impl PNSObject for FSVolumeSupportedCapabilities {}
impl std::convert::TryFrom<NSObject> for FSVolumeSupportedCapabilities {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSVolumeSupportedCapabilities, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSVolumeSupportedCapabilities").unwrap())
        };
        if is_kind_of {
            Ok(FSVolumeSupportedCapabilities(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSVolumeSupportedCapabilities")
        }
    }
}
impl IFSVolumeSupportedCapabilities for FSVolumeSupportedCapabilities {}
pub trait IFSVolumeSupportedCapabilities: Sized + std::ops::Deref {
    unsafe fn supportsPersistentObjectIDs(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsPersistentObjectIDs)
    }
    unsafe fn setSupportsPersistentObjectIDs_(&self, supportsPersistentObjectIDs: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsPersistentObjectIDs : supportsPersistentObjectIDs)
    }
    unsafe fn supportsSymbolicLinks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsSymbolicLinks)
    }
    unsafe fn setSupportsSymbolicLinks_(&self, supportsSymbolicLinks: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsSymbolicLinks : supportsSymbolicLinks)
    }
    unsafe fn supportsHardLinks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsHardLinks)
    }
    unsafe fn setSupportsHardLinks_(&self, supportsHardLinks: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsHardLinks : supportsHardLinks)
    }
    unsafe fn supportsJournal(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsJournal)
    }
    unsafe fn setSupportsJournal_(&self, supportsJournal: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsJournal : supportsJournal)
    }
    unsafe fn supportsActiveJournal(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsActiveJournal)
    }
    unsafe fn setSupportsActiveJournal_(&self, supportsActiveJournal: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsActiveJournal : supportsActiveJournal)
    }
    unsafe fn doesNotSupportRootTimes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doesNotSupportRootTimes)
    }
    unsafe fn setDoesNotSupportRootTimes_(&self, doesNotSupportRootTimes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoesNotSupportRootTimes : doesNotSupportRootTimes)
    }
    unsafe fn supportsSparseFiles(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsSparseFiles)
    }
    unsafe fn setSupportsSparseFiles_(&self, supportsSparseFiles: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsSparseFiles : supportsSparseFiles)
    }
    unsafe fn supportsZeroRuns(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsZeroRuns)
    }
    unsafe fn setSupportsZeroRuns_(&self, supportsZeroRuns: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsZeroRuns : supportsZeroRuns)
    }
    unsafe fn supportsFastStatFS(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsFastStatFS)
    }
    unsafe fn setSupportsFastStatFS_(&self, supportsFastStatFS: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsFastStatFS : supportsFastStatFS)
    }
    unsafe fn supports2TBFiles(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supports2TBFiles)
    }
    unsafe fn setSupports2TBFiles_(&self, supports2TBFiles: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupports2TBFiles : supports2TBFiles)
    }
    unsafe fn supportsOpenDenyModes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsOpenDenyModes)
    }
    unsafe fn setSupportsOpenDenyModes_(&self, supportsOpenDenyModes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsOpenDenyModes : supportsOpenDenyModes)
    }
    unsafe fn supportsHiddenFiles(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsHiddenFiles)
    }
    unsafe fn setSupportsHiddenFiles_(&self, supportsHiddenFiles: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsHiddenFiles : supportsHiddenFiles)
    }
    unsafe fn doesNotSupportVolumeSizes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doesNotSupportVolumeSizes)
    }
    unsafe fn setDoesNotSupportVolumeSizes_(&self, doesNotSupportVolumeSizes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoesNotSupportVolumeSizes : doesNotSupportVolumeSizes)
    }
    unsafe fn supports64BitObjectIDs(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supports64BitObjectIDs)
    }
    unsafe fn setSupports64BitObjectIDs_(&self, supports64BitObjectIDs: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupports64BitObjectIDs : supports64BitObjectIDs)
    }
    unsafe fn supportsDocumentID(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsDocumentID)
    }
    unsafe fn setSupportsDocumentID_(&self, supportsDocumentID: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsDocumentID : supportsDocumentID)
    }
    unsafe fn doesNotSupportImmutableFiles(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doesNotSupportImmutableFiles)
    }
    unsafe fn setDoesNotSupportImmutableFiles_(&self, doesNotSupportImmutableFiles: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoesNotSupportImmutableFiles : doesNotSupportImmutableFiles)
    }
    unsafe fn doesNotSupportSettingFilePermissions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doesNotSupportSettingFilePermissions)
    }
    unsafe fn setDoesNotSupportSettingFilePermissions_(
        &self,
        doesNotSupportSettingFilePermissions: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoesNotSupportSettingFilePermissions : doesNotSupportSettingFilePermissions)
    }
    unsafe fn supportsSharedSpace(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsSharedSpace)
    }
    unsafe fn setSupportsSharedSpace_(&self, supportsSharedSpace: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsSharedSpace : supportsSharedSpace)
    }
    unsafe fn supportsVolumeGroups(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsVolumeGroups)
    }
    unsafe fn setSupportsVolumeGroups_(&self, supportsVolumeGroups: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsVolumeGroups : supportsVolumeGroups)
    }
    unsafe fn caseFormat(&self) -> FSVolumeCaseFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, caseFormat)
    }
    unsafe fn setCaseFormat_(&self, caseFormat: FSVolumeCaseFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaseFormat : caseFormat)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSVolume(pub id);
impl std::ops::Deref for FSVolume {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSVolume {}
impl FSVolume {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSVolume").unwrap(), alloc) })
    }
}
impl INSObject for FSVolume {}
impl PNSObject for FSVolume {}
impl std::convert::TryFrom<NSObject> for FSVolume {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSVolume, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSVolume").unwrap()) };
        if is_kind_of {
            Ok(FSVolume(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSVolume")
        }
    }
}
impl IFSVolume for FSVolume {}
pub trait IFSVolume: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithVolumeID_volumeName_(
        &self,
        volumeID: FSVolumeIdentifier,
        volumeName: FSFileName,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVolumeID : volumeID, volumeName : volumeName)
    }
    unsafe fn volumeID(&self) -> FSVolumeIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volumeID)
    }
    unsafe fn name(&self) -> FSFileName
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: FSFileName)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
}
pub trait PFSVolumePathConfOperations: Sized + std::ops::Deref {
    unsafe fn maximumLinkCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumLinkCount)
    }
    unsafe fn maximumNameLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumNameLength)
    }
    unsafe fn restrictsOwnershipChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restrictsOwnershipChanges)
    }
    unsafe fn truncatesLongNames(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, truncatesLongNames)
    }
    unsafe fn maximumXattrSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumXattrSize)
    }
    unsafe fn maximumXattrSizeInBits(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumXattrSizeInBits)
    }
    unsafe fn maximumFileSize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumFileSize)
    }
    unsafe fn maximumFileSizeInBits(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumFileSizeInBits)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSStatFSResult(pub id);
impl std::ops::Deref for FSStatFSResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSStatFSResult {}
impl FSStatFSResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSStatFSResult").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for FSStatFSResult {}
impl INSObject for FSStatFSResult {}
impl PNSObject for FSStatFSResult {}
impl std::convert::TryFrom<NSObject> for FSStatFSResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSStatFSResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSStatFSResult").unwrap()) };
        if is_kind_of {
            Ok(FSStatFSResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSStatFSResult")
        }
    }
}
impl IFSStatFSResult for FSStatFSResult {}
pub trait IFSStatFSResult: Sized + std::ops::Deref {
    unsafe fn initWithFileSystemTypeName_(&self, fileSystemTypeName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileSystemTypeName : fileSystemTypeName)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn blockSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blockSize)
    }
    unsafe fn setBlockSize_(&self, blockSize: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlockSize : blockSize)
    }
    unsafe fn ioSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ioSize)
    }
    unsafe fn setIoSize_(&self, ioSize: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIoSize : ioSize)
    }
    unsafe fn totalBlocks(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalBlocks)
    }
    unsafe fn setTotalBlocks_(&self, totalBlocks: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTotalBlocks : totalBlocks)
    }
    unsafe fn availableBlocks(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableBlocks)
    }
    unsafe fn setAvailableBlocks_(&self, availableBlocks: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAvailableBlocks : availableBlocks)
    }
    unsafe fn freeBlocks(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, freeBlocks)
    }
    unsafe fn setFreeBlocks_(&self, freeBlocks: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFreeBlocks : freeBlocks)
    }
    unsafe fn usedBlocks(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usedBlocks)
    }
    unsafe fn setUsedBlocks_(&self, usedBlocks: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsedBlocks : usedBlocks)
    }
    unsafe fn totalBytes(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalBytes)
    }
    unsafe fn setTotalBytes_(&self, totalBytes: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTotalBytes : totalBytes)
    }
    unsafe fn availableBytes(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableBytes)
    }
    unsafe fn setAvailableBytes_(&self, availableBytes: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAvailableBytes : availableBytes)
    }
    unsafe fn freeBytes(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, freeBytes)
    }
    unsafe fn setFreeBytes_(&self, freeBytes: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFreeBytes : freeBytes)
    }
    unsafe fn usedBytes(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usedBytes)
    }
    unsafe fn setUsedBytes_(&self, usedBytes: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsedBytes : usedBytes)
    }
    unsafe fn totalFiles(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalFiles)
    }
    unsafe fn setTotalFiles_(&self, totalFiles: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTotalFiles : totalFiles)
    }
    unsafe fn freeFiles(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, freeFiles)
    }
    unsafe fn setFreeFiles_(&self, freeFiles: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFreeFiles : freeFiles)
    }
    unsafe fn fileSystemSubType(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileSystemSubType)
    }
    unsafe fn setFileSystemSubType_(&self, fileSystemSubType: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFileSystemSubType : fileSystemSubType)
    }
    unsafe fn fileSystemTypeName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileSystemTypeName)
    }
}
pub trait PFSVolumeOperations: Sized + std::ops::Deref {
    unsafe fn mountWithOptions_replyHandler_(
        &self,
        options: FSTaskOptions,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mountWithOptions : options, replyHandler : reply)
    }
    unsafe fn unmountWithReplyHandler_(&self, reply: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unmountWithReplyHandler : reply)
    }
    unsafe fn synchronizeWithFlags_replyHandler_(
        &self,
        flags: FSSyncFlags,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, synchronizeWithFlags : flags, replyHandler : reply)
    }
    unsafe fn getAttributes_ofItem_replyHandler_(
        &self,
        desiredAttributes: FSItemGetAttributesRequest,
        item: FSItem,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAttributes : desiredAttributes, ofItem : item, replyHandler : reply)
    }
    unsafe fn setAttributes_onItem_replyHandler_(
        &self,
        newAttributes: FSItemSetAttributesRequest,
        item: FSItem,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributes : newAttributes, onItem : item, replyHandler : reply)
    }
    unsafe fn lookupItemNamed_inDirectory_replyHandler_(
        &self,
        name: FSFileName,
        directory: FSItem,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookupItemNamed : name, inDirectory : directory, replyHandler : reply)
    }
    unsafe fn reclaimItem_replyHandler_(&self, item: FSItem, reply: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reclaimItem : item, replyHandler : reply)
    }
    unsafe fn readSymbolicLink_replyHandler_(
        &self,
        item: FSItem,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readSymbolicLink : item, replyHandler : reply)
    }
    unsafe fn createItemNamed_type_inDirectory_attributes_replyHandler_(
        &self,
        name: FSFileName,
        type_: FSItemType,
        directory: FSItem,
        newAttributes: FSItemSetAttributesRequest,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createItemNamed : name, r#type : type_, inDirectory : directory, attributes : newAttributes, replyHandler : reply)
    }
    unsafe fn createSymbolicLinkNamed_inDirectory_attributes_linkContents_replyHandler_(
        &self,
        name: FSFileName,
        directory: FSItem,
        newAttributes: FSItemSetAttributesRequest,
        contents: FSFileName,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createSymbolicLinkNamed : name, inDirectory : directory, attributes : newAttributes, linkContents : contents, replyHandler : reply)
    }
    unsafe fn createLinkToItem_named_inDirectory_replyHandler_(
        &self,
        item: FSItem,
        name: FSFileName,
        directory: FSItem,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createLinkToItem : item, named : name, inDirectory : directory, replyHandler : reply)
    }
    unsafe fn removeItem_named_fromDirectory_replyHandler_(
        &self,
        item: FSItem,
        name: FSFileName,
        directory: FSItem,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeItem : item, named : name, fromDirectory : directory, replyHandler : reply)
    }
    unsafe fn renameItem_inDirectory_named_toNewName_inDirectory_overItem_replyHandler_(
        &self,
        item: FSItem,
        sourceDirectory: FSItem,
        sourceName: FSFileName,
        destinationName: FSFileName,
        destinationDirectory: FSItem,
        overItem: FSItem,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renameItem : item, inDirectory : sourceDirectory, named : sourceName, toNewName : destinationName, inDirectory : destinationDirectory, overItem : overItem, replyHandler : reply)
    }
    unsafe fn enumerateDirectory_startingAtCookie_verifier_providingAttributes_usingPacker_replyHandler_(
        &self,
        directory: FSItem,
        cookie: FSDirectoryCookie,
        verifier: FSDirectoryVerifier,
        attributes: FSItemGetAttributesRequest,
        packer: FSDirectoryEntryPacker,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateDirectory : directory, startingAtCookie : cookie, verifier : verifier, providingAttributes : attributes, usingPacker : packer, replyHandler : reply)
    }
    unsafe fn activateWithOptions_replyHandler_(
        &self,
        options: FSTaskOptions,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateWithOptions : options, replyHandler : reply)
    }
    unsafe fn deactivateWithOptions_replyHandler_(
        &self,
        options: FSDeactivateOptions,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deactivateWithOptions : options, replyHandler : reply)
    }
    unsafe fn supportedVolumeCapabilities(&self) -> FSVolumeSupportedCapabilities
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedVolumeCapabilities)
    }
    unsafe fn volumeStatistics(&self) -> FSStatFSResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volumeStatistics)
    }
    unsafe fn enableOpenUnlinkEmulation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableOpenUnlinkEmulation)
    }
    unsafe fn setEnableOpenUnlinkEmulation_(&self, enableOpenUnlinkEmulation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnableOpenUnlinkEmulation : enableOpenUnlinkEmulation)
    }
}
pub type FSSetXattrPolicy = NSUInteger;
pub trait PFSVolumeXattrOperations: Sized + std::ops::Deref {
    unsafe fn supportedXattrNamesForItem_(&self, item: FSItem) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedXattrNamesForItem : item)
    }
    unsafe fn getXattrNamed_ofItem_replyHandler_(
        &self,
        name: FSFileName,
        item: FSItem,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getXattrNamed : name, ofItem : item, replyHandler : reply)
    }
    unsafe fn setXattrNamed_toData_onItem_policy_replyHandler_(
        &self,
        name: FSFileName,
        value: NSData,
        item: FSItem,
        policy: FSSetXattrPolicy,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXattrNamed : name, toData : value, onItem : item, policy : policy, replyHandler : reply)
    }
    unsafe fn listXattrsOfItem_replyHandler_(
        &self,
        item: FSItem,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, listXattrsOfItem : item, replyHandler : reply)
    }
    unsafe fn xattrOperationsInhibited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xattrOperationsInhibited)
    }
    unsafe fn setXattrOperationsInhibited_(&self, xattrOperationsInhibited: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXattrOperationsInhibited : xattrOperationsInhibited)
    }
}
pub type FSVolumeOpenModes = NSUInteger;
pub trait PFSVolumeOpenCloseOperations: Sized + std::ops::Deref {
    unsafe fn openItem_withModes_replyHandler_(
        &self,
        item: FSItem,
        modes: FSVolumeOpenModes,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openItem : item, withModes : modes, replyHandler : reply)
    }
    unsafe fn closeItem_keepingModes_replyHandler_(
        &self,
        item: FSItem,
        modes: FSVolumeOpenModes,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, closeItem : item, keepingModes : modes, replyHandler : reply)
    }
    unsafe fn isOpenCloseInhibited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpenCloseInhibited)
    }
    unsafe fn setOpenCloseInhibited_(&self, openCloseInhibited: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpenCloseInhibited : openCloseInhibited)
    }
}
pub trait PFSVolumeReadWriteOperations: Sized + std::ops::Deref {
    unsafe fn readFromFile_offset_length_intoBuffer_replyHandler_(
        &self,
        item: FSItem,
        offset: off_t,
        length: usize,
        buffer: FSMutableFileDataBuffer,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readFromFile : item, offset : offset, length : length, intoBuffer : buffer, replyHandler : reply)
    }
    unsafe fn writeContents_toFile_atOffset_replyHandler_(
        &self,
        contents: NSData,
        item: FSItem,
        offset: off_t,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeContents : contents, toFile : item, atOffset : offset, replyHandler : reply)
    }
}
pub type FSAccessMask = NSUInteger;
pub trait PFSVolumeAccessCheckOperations: Sized + std::ops::Deref {
    unsafe fn checkAccessToItem_requestedAccess_replyHandler_(
        &self,
        theItem: FSItem,
        access: FSAccessMask,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkAccessToItem : theItem, requestedAccess : access, replyHandler : reply)
    }
    unsafe fn isAccessCheckInhibited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAccessCheckInhibited)
    }
    unsafe fn setAccessCheckInhibited_(&self, accessCheckInhibited: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessCheckInhibited : accessCheckInhibited)
    }
}
pub trait PFSVolumeRenameOperations: Sized + std::ops::Deref {
    unsafe fn setVolumeName_replyHandler_(
        &self,
        name: FSFileName,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVolumeName : name, replyHandler : reply)
    }
    unsafe fn isVolumeRenameInhibited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVolumeRenameInhibited)
    }
    unsafe fn setVolumeRenameInhibited_(&self, volumeRenameInhibited: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVolumeRenameInhibited : volumeRenameInhibited)
    }
}
pub type FSPreallocateFlags = NSUInteger;
pub trait PFSVolumePreallocateOperations: Sized + std::ops::Deref {
    unsafe fn preallocateSpaceForItem_atOffset_length_flags_replyHandler_(
        &self,
        item: FSItem,
        offset: off_t,
        length: usize,
        flags: FSPreallocateFlags,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preallocateSpaceForItem : item, atOffset : offset, length : length, flags : flags, replyHandler : reply)
    }
    unsafe fn isPreallocateInhibited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPreallocateInhibited)
    }
    unsafe fn setPreallocateInhibited_(&self, preallocateInhibited: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreallocateInhibited : preallocateInhibited)
    }
}
pub type FSItemDeactivationOptions = NSUInteger;
pub trait PFSVolumeItemDeactivation: Sized + std::ops::Deref {
    unsafe fn deactivateItem_replyHandler_(&self, item: FSItem, reply: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deactivateItem : item, replyHandler : reply)
    }
    unsafe fn itemDeactivationPolicy(&self) -> FSItemDeactivationOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemDeactivationPolicy)
    }
}
pub type FSOperationID = NSUInteger;
pub type FSBlockmapFlags = NSUInteger;
pub type FSCompleteIOFlags = NSUInteger;
pub type FSExtentType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSExtentPacker(pub id);
impl std::ops::Deref for FSExtentPacker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSExtentPacker {}
impl FSExtentPacker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSExtentPacker").unwrap(), alloc) })
    }
}
impl INSObject for FSExtentPacker {}
impl PNSObject for FSExtentPacker {}
impl std::convert::TryFrom<NSObject> for FSExtentPacker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSExtentPacker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSExtentPacker").unwrap()) };
        if is_kind_of {
            Ok(FSExtentPacker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSExtentPacker")
        }
    }
}
impl IFSExtentPacker for FSExtentPacker {}
pub trait IFSExtentPacker: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn packExtentWithResource_type_logicalOffset_physicalOffset_length_(
        &self,
        resource: FSBlockDeviceResource,
        type_: FSExtentType,
        logicalOffset: off_t,
        physicalOffset: off_t,
        length: usize,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, packExtentWithResource : resource, r#type : type_, logicalOffset : logicalOffset, physicalOffset : physicalOffset, length : length)
    }
}
pub trait PFSVolumeKernelOffloadedIOOperations: Sized + std::ops::Deref {
    unsafe fn blockmapFile_offset_length_flags_operationID_packer_replyHandler_(
        &self,
        file: FSItem,
        offset: off_t,
        length: usize,
        flags: FSBlockmapFlags,
        operationID: FSOperationID,
        packer: FSExtentPacker,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, blockmapFile : file, offset : offset, length : length, flags : flags, operationID : operationID, packer : packer, replyHandler : reply)
    }
    unsafe fn completeIOForFile_offset_length_status_flags_operationID_replyHandler_(
        &self,
        file: FSItem,
        offset: off_t,
        length: usize,
        status: NSError,
        flags: FSCompleteIOFlags,
        operationID: FSOperationID,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeIOForFile : file, offset : offset, length : length, status : status, flags : flags, operationID : operationID, replyHandler : reply)
    }
    unsafe fn createFileNamed_inDirectory_attributes_packer_replyHandler_(
        &self,
        name: FSFileName,
        directory: FSItem,
        attributes: FSItemSetAttributesRequest,
        packer: FSExtentPacker,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createFileNamed : name, inDirectory : directory, attributes : attributes, packer : packer, replyHandler : reply)
    }
    unsafe fn lookupItemNamed_inDirectory_packer_replyHandler_(
        &self,
        name: FSFileName,
        directory: FSItem,
        packer: FSExtentPacker,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookupItemNamed : name, inDirectory : directory, packer : packer, replyHandler : reply)
    }
    unsafe fn preallocateSpaceForFile_atOffset_length_flags_packer_replyHandler_(
        &self,
        file: FSItem,
        offset: off_t,
        length: usize,
        flags: FSPreallocateFlags,
        packer: FSExtentPacker,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preallocateSpaceForFile : file, atOffset : offset, length : length, flags : flags, packer : packer, replyHandler : reply)
    }
}
pub trait PFSFileSystemBase: Sized + std::ops::Deref {
    unsafe fn wipeResource_completionHandler_(
        &self,
        resource: FSBlockDeviceResource,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, wipeResource : resource, completionHandler : completion)
    }
    unsafe fn containerStatus(&self) -> FSContainerStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerStatus)
    }
    unsafe fn setContainerStatus_(&self, containerStatus: FSContainerStatus)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContainerStatus : containerStatus)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSFileSystem(pub id);
impl std::ops::Deref for FSFileSystem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSFileSystem {}
impl FSFileSystem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSFileSystem").unwrap(), alloc) })
    }
}
impl PFSFileSystemBase for FSFileSystem {}
impl INSObject for FSFileSystem {}
impl PNSObject for FSFileSystem {}
impl std::convert::TryFrom<NSObject> for FSFileSystem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSFileSystem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSFileSystem").unwrap()) };
        if is_kind_of {
            Ok(FSFileSystem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSFileSystem")
        }
    }
}
impl IFSFileSystem for FSFileSystem {}
pub trait IFSFileSystem: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FSUnaryFileSystem(pub id);
impl std::ops::Deref for FSUnaryFileSystem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for FSUnaryFileSystem {}
impl FSUnaryFileSystem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"FSUnaryFileSystem").unwrap(), alloc) })
    }
}
impl PFSFileSystemBase for FSUnaryFileSystem {}
impl INSObject for FSUnaryFileSystem {}
impl PNSObject for FSUnaryFileSystem {}
impl std::convert::TryFrom<NSObject> for FSUnaryFileSystem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<FSUnaryFileSystem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"FSUnaryFileSystem").unwrap()) };
        if is_kind_of {
            Ok(FSUnaryFileSystem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to FSUnaryFileSystem")
        }
    }
}
impl IFSUnaryFileSystem for FSUnaryFileSystem {}
pub trait IFSUnaryFileSystem: Sized + std::ops::Deref {}
pub trait PFSUnaryFileSystemOperations: Sized + std::ops::Deref {
    unsafe fn probeResource_replyHandler_(
        &self,
        resource: FSResource,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, probeResource : resource, replyHandler : reply)
    }
    unsafe fn loadResource_options_replyHandler_(
        &self,
        resource: FSResource,
        options: FSTaskOptions,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadResource : resource, options : options, replyHandler : reply)
    }
    unsafe fn unloadResource_options_replyHandler_(
        &self,
        resource: FSResource,
        options: FSTaskOptions,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unloadResource : resource, options : options, replyHandler : reply)
    }
    unsafe fn didFinishLoading(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didFinishLoading)
    }
}
unsafe extern "C" {
    pub fn fs_errorForPOSIXError(arg1: ::std::os::raw::c_int) -> NSError;
}
unsafe extern "C" {
    pub fn fs_errorForMachError(errorCode: ::std::os::raw::c_int) -> NSError;
}
unsafe extern "C" {
    pub fn fs_errorForCocoaError(errorCode: ::std::os::raw::c_int) -> NSError;
}
unsafe extern "C" {
    pub static mut FSKitVersionNumber: f64;
}
unsafe extern "C" {
    pub static FSKitVersionString: [::std::os::raw::c_uchar; 0usize];
}
unsafe extern "C" {
    pub static FSKitErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static FSDirectoryCookieInitial: FSDirectoryCookie;
}
unsafe extern "C" {
    pub static FSDirectoryVerifierInitial: FSDirectoryVerifier;
}
unsafe extern "C" {
    pub static FSOperationIDUnspecified: FSOperationID;
}

unsafe impl objc2::encode::RefEncode for timespec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for timespec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("timespec", &[]);
}
unsafe impl objc2::encode::RefEncode for FSModuleIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSModuleIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSClient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSClient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSEntityIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSEntityIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSContainerStatus {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSContainerStatus {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSContainerIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSContainerIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSMutableFileDataBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSMutableFileDataBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSFileName {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSFileName {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSItemAttributes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSItemAttributes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSItemSetAttributesRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSItemSetAttributesRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSItemGetAttributesRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSItemGetAttributesRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSTaskOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSTaskOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSMetadataRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSMetadataRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSBlockDeviceResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSBlockDeviceResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSGenericURLResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSGenericURLResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSPathURLResource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSPathURLResource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSProbeResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSProbeResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSVolumeIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSVolumeIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSDirectoryEntryPacker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSDirectoryEntryPacker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSVolumeSupportedCapabilities {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSVolumeSupportedCapabilities {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSVolume {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSVolume {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSStatFSResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSStatFSResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSExtentPacker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSExtentPacker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSFileSystem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSFileSystem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for FSUnaryFileSystem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSUnaryFileSystem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
