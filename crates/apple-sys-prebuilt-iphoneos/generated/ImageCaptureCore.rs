#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use libc::off_t;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ICDeviceType = NSUInteger;
pub type ICDeviceLocationType = NSUInteger;
pub type ICDeviceTypeMask = NSUInteger;
pub type ICDeviceLocationTypeMask = NSUInteger;
pub type ICDeviceTransport = NSString;
pub type ICDeviceStatus = NSString;
pub type ICDeviceCapability = NSString;
pub type ICSessionOptions = NSString;
pub type ICDeviceLocationOptions = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICDevice(pub id);
impl std::ops::Deref for ICDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICDevice {}
impl ICDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICDevice").unwrap(), alloc) })
    }
}
impl INSObject for ICDevice {}
impl PNSObject for ICDevice {}
impl std::convert::TryFrom<NSObject> for ICDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ICDevice, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICDevice").unwrap()) };
        if is_kind_of {
            Ok(ICDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ICDevice")
        }
    }
}
impl IICDevice for ICDevice {}
pub trait IICDevice: Sized + std::ops::Deref {
    unsafe fn requestOpenSession(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestOpenSession)
    }
    unsafe fn requestCloseSession(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestCloseSession)
    }
    unsafe fn requestEject(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestEject)
    }
    unsafe fn requestOpenSessionWithOptions_completion_(
        &self,
        options: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestOpenSessionWithOptions : options, completion : completion)
    }
    unsafe fn requestCloseSessionWithOptions_completion_(
        &self,
        options: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestCloseSessionWithOptions : options, completion : completion)
    }
    unsafe fn requestEjectWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestEjectWithCompletion : completion)
    }
    unsafe fn requestEjectOrDisconnect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestEjectOrDisconnect)
    }
    unsafe fn requestYield(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestYield)
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
    unsafe fn type_(&self) -> ICDeviceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn capabilities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capabilities)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn productKind(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productKind)
    }
    unsafe fn icon(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, icon)
    }
    unsafe fn systemSymbolName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemSymbolName)
    }
    unsafe fn transportType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transportType)
    }
    unsafe fn UUIDString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUIDString)
    }
    unsafe fn locationDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationDescription)
    }
    unsafe fn hasOpenSession(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasOpenSession)
    }
    unsafe fn userData(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userData)
    }
    unsafe fn modulePath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modulePath)
    }
    unsafe fn moduleVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, moduleVersion)
    }
    unsafe fn serialNumberString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serialNumberString)
    }
    unsafe fn usbLocationID(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usbLocationID)
    }
    unsafe fn usbProductID(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usbProductID)
    }
    unsafe fn usbVendorID(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usbVendorID)
    }
    unsafe fn autolaunchApplicationPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autolaunchApplicationPath)
    }
    unsafe fn setAutolaunchApplicationPath_(&self, autolaunchApplicationPath: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutolaunchApplicationPath : autolaunchApplicationPath)
    }
    unsafe fn isRemote(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRemote)
    }
    unsafe fn persistentIDString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentIDString)
    }
    unsafe fn moduleExecutableArchitecture(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, moduleExecutableArchitecture)
    }
}
pub trait PICDeviceDelegate: Sized + std::ops::Deref {
    unsafe fn device_didCloseSessionWithError_(&self, device: ICDevice, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, device : device, didCloseSessionWithError : error)
    }
    unsafe fn didRemoveDevice_(&self, device: ICDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didRemoveDevice : device)
    }
    unsafe fn device_didOpenSessionWithError_(&self, device: ICDevice, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, device : device, didOpenSessionWithError : error)
    }
    unsafe fn deviceDidBecomeReady_(&self, device: ICDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceDidBecomeReady : device)
    }
    unsafe fn deviceDidChangeName_(&self, device: ICDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceDidChangeName : device)
    }
    unsafe fn device_didReceiveStatusInformation_(&self, device: ICDevice, status: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, device : device, didReceiveStatusInformation : status)
    }
    unsafe fn device_didEncounterError_(&self, device: ICDevice, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, device : device, didEncounterError : error)
    }
    unsafe fn device_didEjectWithError_(&self, device: ICDevice, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, device : device, didEjectWithError : error)
    }
    unsafe fn deviceDidChangeSharingState_(&self, device: ICDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceDidChangeSharingState : device)
    }
}
pub type ICEXIFOrientationType = NSUInteger;
pub type ICReturnThumbnailErrorCode = NSInteger;
pub type ICReturnMetadataErrorCode = NSInteger;
pub type ICReturnConnectionErrorCode = NSInteger;
pub type ICReturnPTPDeviceErrorCode = NSInteger;
pub type ICReturnDownloadErrorCode = NSInteger;
pub type ICLegacyReturnCode = NSInteger;
pub type ICReturnCode = NSInteger;
pub type ICReturnObjectErrorCode = NSInteger;
pub type ICCameraItemThumbnailOption = NSString;
pub type ICDownloadOption = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICCameraItem(pub id);
impl std::ops::Deref for ICCameraItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICCameraItem {}
impl ICCameraItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICCameraItem").unwrap(), alloc) })
    }
}
impl INSObject for ICCameraItem {}
impl PNSObject for ICCameraItem {}
impl std::convert::TryFrom<NSObject> for ICCameraItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ICCameraItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICCameraItem").unwrap()) };
        if is_kind_of {
            Ok(ICCameraItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ICCameraItem")
        }
    }
}
impl IICCameraItem for ICCameraItem {}
pub trait IICCameraItem: Sized + std::ops::Deref {
    unsafe fn requestThumbnail(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestThumbnail)
    }
    unsafe fn requestMetadata(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestMetadata)
    }
    unsafe fn flushThumbnailCache(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flushThumbnailCache)
    }
    unsafe fn flushMetadataCache(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flushMetadataCache)
    }
    unsafe fn device(&self) -> ICCameraDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn parentFolder(&self) -> ICCameraFolder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentFolder)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn UTI(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UTI)
    }
    unsafe fn fileSystemPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileSystemPath)
    }
    unsafe fn isLocked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocked)
    }
    unsafe fn isRaw(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRaw)
    }
    unsafe fn isInTemporaryStore(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInTemporaryStore)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn modificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modificationDate)
    }
    unsafe fn thumbnail(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbnail)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn userData(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userData)
    }
    unsafe fn ptpObjectHandle(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ptpObjectHandle)
    }
    unsafe fn wasAddedAfterContentCatalogCompleted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wasAddedAfterContentCatalogCompleted)
    }
    unsafe fn thumbnailIfAvailable(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbnailIfAvailable)
    }
    unsafe fn largeThumbnailIfAvailable(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, largeThumbnailIfAvailable)
    }
    unsafe fn metadataIfAvailable(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadataIfAvailable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICCameraFile(pub id);
impl std::ops::Deref for ICCameraFile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICCameraFile {}
impl ICCameraFile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICCameraFile").unwrap(), alloc) })
    }
}
impl IICCameraItem for ICCameraFile {}
impl From<ICCameraFile> for ICCameraItem {
    fn from(child: ICCameraFile) -> ICCameraItem {
        ICCameraItem(child.0)
    }
}
impl std::convert::TryFrom<ICCameraItem> for ICCameraFile {
    type Error = &'static str;
    fn try_from(parent: ICCameraItem) -> Result<ICCameraFile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICCameraFile").unwrap()) };
        if is_kind_of {
            Ok(ICCameraFile(parent.0))
        } else {
            Err("This ICCameraItem cannot be downcasted to ICCameraFile")
        }
    }
}
impl INSObject for ICCameraFile {}
impl PNSObject for ICCameraFile {}
impl IICCameraFile for ICCameraFile {}
pub trait IICCameraFile: Sized + std::ops::Deref {
    unsafe fn requestThumbnailDataWithOptions_completion_(
        &self,
        options: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestThumbnailDataWithOptions : options, completion : completion)
    }
    unsafe fn requestMetadataDictionaryWithOptions_completion_(
        &self,
        options: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestMetadataDictionaryWithOptions : options, completion : completion)
    }
    unsafe fn requestDownloadWithOptions_completion_(
        &self,
        options: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDownloadWithOptions : options, completion : completion)
    }
    unsafe fn requestReadDataAtOffset_length_completion_(
        &self,
        offset: off_t,
        length: off_t,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestReadDataAtOffset : offset, length : length, completion : completion)
    }
    unsafe fn requestSecurityScopedURLWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestSecurityScopedURLWithCompletion : completion)
    }
    unsafe fn requestFingerprintWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestFingerprintWithCompletion : completion)
    }
    unsafe fn width(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn originalFilename(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalFilename)
    }
    unsafe fn createdFilename(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createdFilename)
    }
    unsafe fn fileSize(&self) -> off_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileSize)
    }
    unsafe fn orientation(&self) -> ICEXIFOrientationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
    unsafe fn setOrientation_(&self, orientation: ICEXIFOrientationType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrientation : orientation)
    }
    unsafe fn duration(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn highFramerate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highFramerate)
    }
    unsafe fn timeLapse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeLapse)
    }
    unsafe fn firstPicked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstPicked)
    }
    unsafe fn originatingAssetID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originatingAssetID)
    }
    unsafe fn groupUUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupUUID)
    }
    unsafe fn gpsString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpsString)
    }
    unsafe fn relatedUUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relatedUUID)
    }
    unsafe fn burstUUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, burstUUID)
    }
    unsafe fn burstFavorite(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, burstFavorite)
    }
    unsafe fn burstPicked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, burstPicked)
    }
    unsafe fn sidecarFiles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sidecarFiles)
    }
    unsafe fn pairedRawImage(&self) -> ICCameraFile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pairedRawImage)
    }
    unsafe fn fileCreationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileCreationDate)
    }
    unsafe fn fileModificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileModificationDate)
    }
    unsafe fn exifCreationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exifCreationDate)
    }
    unsafe fn exifModificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exifModificationDate)
    }
    unsafe fn fingerprint(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fingerprint)
    }
    unsafe fn fingerprintForFileAtURL_(url: NSURL) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ICCameraFile").unwrap(), fingerprintForFileAtURL : url)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICCameraFolder(pub id);
impl std::ops::Deref for ICCameraFolder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICCameraFolder {}
impl ICCameraFolder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICCameraFolder").unwrap(), alloc) })
    }
}
impl IICCameraItem for ICCameraFolder {}
impl std::convert::TryFrom<ICCameraItem> for ICCameraFolder {
    type Error = &'static str;
    fn try_from(parent: ICCameraItem) -> Result<ICCameraFolder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICCameraFolder").unwrap()) };
        if is_kind_of {
            Ok(ICCameraFolder(parent.0))
        } else {
            Err("This ICCameraItem cannot be downcasted to ICCameraFolder")
        }
    }
}
impl INSObject for ICCameraFolder {}
impl PNSObject for ICCameraFolder {}
impl IICCameraFolder for ICCameraFolder {}
pub trait IICCameraFolder: Sized + std::ops::Deref {
    unsafe fn contents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
}
pub type ICDeleteResult = NSString;
pub type ICDeleteError = NSString;
pub type ICMediaPresentation = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICCameraDevice(pub id);
impl std::ops::Deref for ICCameraDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICCameraDevice {}
impl ICCameraDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICCameraDevice").unwrap(), alloc) })
    }
}
impl IICDevice for ICCameraDevice {}
impl From<ICCameraDevice> for ICDevice {
    fn from(child: ICCameraDevice) -> ICDevice {
        ICDevice(child.0)
    }
}
impl std::convert::TryFrom<ICDevice> for ICCameraDevice {
    type Error = &'static str;
    fn try_from(parent: ICDevice) -> Result<ICCameraDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICCameraDevice").unwrap()) };
        if is_kind_of {
            Ok(ICCameraDevice(parent.0))
        } else {
            Err("This ICDevice cannot be downcasted to ICCameraDevice")
        }
    }
}
impl INSObject for ICCameraDevice {}
impl PNSObject for ICCameraDevice {}
impl IICCameraDevice for ICCameraDevice {}
pub trait IICCameraDevice: Sized + std::ops::Deref {
    unsafe fn filesOfType_(&self, fileUTType: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, filesOfType : fileUTType)
    }
    unsafe fn cancelDownload(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelDownload)
    }
    unsafe fn requestDeleteFiles_(&self, files: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDeleteFiles : files)
    }
    unsafe fn requestDeleteFiles_deleteFailed_completion_(
        &self,
        files: NSArray,
        deleteFailed: *mut ::std::os::raw::c_void,
        completion: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDeleteFiles : files, deleteFailed : deleteFailed, completion : completion)
    }
    unsafe fn cancelDelete(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelDelete)
    }
    unsafe fn requestSyncClock(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestSyncClock)
    }
    unsafe fn requestTakePicture(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestTakePicture)
    }
    unsafe fn requestEnableTethering(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestEnableTethering)
    }
    unsafe fn requestDisableTethering(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestDisableTethering)
    }
    unsafe fn requestSendPTPCommand_outData_completion_(
        &self,
        ptpCommand: NSData,
        ptpData: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestSendPTPCommand : ptpCommand, outData : ptpData, completion : completion)
    }
    unsafe fn contentCatalogPercentCompleted(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentCatalogPercentCompleted)
    }
    unsafe fn contents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn mediaFiles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaFiles)
    }
    unsafe fn isEjectable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEjectable)
    }
    unsafe fn isLocked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocked)
    }
    unsafe fn isAccessRestrictedAppleDevice(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAccessRestrictedAppleDevice)
    }
    unsafe fn iCloudPhotosEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iCloudPhotosEnabled)
    }
    unsafe fn mountPoint(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mountPoint)
    }
    unsafe fn mediaPresentation(&self) -> ICMediaPresentation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaPresentation)
    }
    unsafe fn setMediaPresentation_(&self, mediaPresentation: ICMediaPresentation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaPresentation : mediaPresentation)
    }
    unsafe fn timeOffset(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeOffset)
    }
    unsafe fn batteryLevelAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, batteryLevelAvailable)
    }
    unsafe fn batteryLevel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, batteryLevel)
    }
    unsafe fn tetheredCaptureEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tetheredCaptureEnabled)
    }
    unsafe fn ptpEventHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ptpEventHandler)
    }
    unsafe fn setPtpEventHandler_(&self, ptpEventHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPtpEventHandler : ptpEventHandler)
    }
}
pub trait PICCameraDeviceDelegate: Sized + std::ops::Deref {
    unsafe fn cameraDevice_didAddItems_(&self, camera: ICCameraDevice, items: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didAddItems : items)
    }
    unsafe fn cameraDevice_didRemoveItems_(&self, camera: ICCameraDevice, items: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didRemoveItems : items)
    }
    unsafe fn cameraDevice_didReceiveThumbnail_forItem_error_(
        &self,
        camera: ICCameraDevice,
        thumbnail: CGImageRef,
        item: ICCameraItem,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didReceiveThumbnail : thumbnail, forItem : item, error : error)
    }
    unsafe fn cameraDevice_didReceiveMetadata_forItem_error_(
        &self,
        camera: ICCameraDevice,
        metadata: NSDictionary,
        item: ICCameraItem,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didReceiveMetadata : metadata, forItem : item, error : error)
    }
    unsafe fn cameraDevice_didRenameItems_(&self, camera: ICCameraDevice, items: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didRenameItems : items)
    }
    unsafe fn cameraDeviceDidChangeCapability_(&self, camera: ICCameraDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDeviceDidChangeCapability : camera)
    }
    unsafe fn cameraDevice_didReceivePTPEvent_(&self, camera: ICCameraDevice, eventData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didReceivePTPEvent : eventData)
    }
    unsafe fn deviceDidBecomeReadyWithCompleteContentCatalog_(&self, device: ICCameraDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceDidBecomeReadyWithCompleteContentCatalog : device)
    }
    unsafe fn cameraDeviceDidRemoveAccessRestriction_(&self, device: ICDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDeviceDidRemoveAccessRestriction : device)
    }
    unsafe fn cameraDeviceDidEnableAccessRestriction_(&self, device: ICDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDeviceDidEnableAccessRestriction : device)
    }
    unsafe fn cameraDevice_shouldGetThumbnailOfItem_(
        &self,
        cameraDevice: ICCameraDevice,
        item: ICCameraItem,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : cameraDevice, shouldGetThumbnailOfItem : item)
    }
    unsafe fn cameraDevice_shouldGetMetadataOfItem_(
        &self,
        cameraDevice: ICCameraDevice,
        item: ICCameraItem,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : cameraDevice, shouldGetMetadataOfItem : item)
    }
    unsafe fn cameraDevice_didCompleteDeleteFilesWithError_(
        &self,
        camera: ICCameraDevice,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didCompleteDeleteFilesWithError : error)
    }
    unsafe fn cameraDevice_didAddItem_(&self, camera: ICCameraDevice, item: ICCameraItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didAddItem : item)
    }
    unsafe fn cameraDevice_didRemoveItem_(&self, camera: ICCameraDevice, item: ICCameraItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didRemoveItem : item)
    }
    unsafe fn cameraDevice_didReceiveThumbnailForItem_(
        &self,
        camera: ICCameraDevice,
        item: ICCameraItem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didReceiveThumbnailForItem : item)
    }
    unsafe fn cameraDevice_didReceiveMetadataForItem_(
        &self,
        camera: ICCameraDevice,
        item: ICCameraItem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDevice : camera, didReceiveMetadataForItem : item)
    }
}
pub trait PICCameraDeviceDownloadDelegate: Sized + std::ops::Deref {
    unsafe fn didDownloadFile_error_options_contextInfo_(
        &self,
        file: ICCameraFile,
        error: NSError,
        options: NSDictionary,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didDownloadFile : file, error : error, options : options, contextInfo : contextInfo)
    }
    unsafe fn didReceiveDownloadProgressForFile_downloadedBytes_maxBytes_(
        &self,
        file: ICCameraFile,
        downloadedBytes: off_t,
        maxBytes: off_t,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveDownloadProgressForFile : file, downloadedBytes : downloadedBytes, maxBytes : maxBytes)
    }
}
pub type ICAuthorizationStatus = NSString;
pub trait PICDeviceBrowserDelegate: Sized + std::ops::Deref {
    unsafe fn deviceBrowser_didAddDevice_moreComing_(
        &self,
        browser: ICDeviceBrowser,
        device: ICDevice,
        moreComing: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowser : browser, didAddDevice : device, moreComing : moreComing)
    }
    unsafe fn deviceBrowser_didRemoveDevice_moreGoing_(
        &self,
        browser: ICDeviceBrowser,
        device: ICDevice,
        moreGoing: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowser : browser, didRemoveDevice : device, moreGoing : moreGoing)
    }
    unsafe fn deviceBrowser_deviceDidChangeName_(&self, browser: ICDeviceBrowser, device: ICDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowser : browser, deviceDidChangeName : device)
    }
    unsafe fn deviceBrowser_deviceDidChangeSharingState_(
        &self,
        browser: ICDeviceBrowser,
        device: ICDevice,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowser : browser, deviceDidChangeSharingState : device)
    }
    unsafe fn deviceBrowser_requestsSelectDevice_(&self, browser: ICDeviceBrowser, device: ICDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowser : browser, requestsSelectDevice : device)
    }
    unsafe fn deviceBrowserDidEnumerateLocalDevices_(&self, browser: ICDeviceBrowser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowserDidEnumerateLocalDevices : browser)
    }
    unsafe fn deviceBrowserWillSuspendOperations_(&self, browser: ICDeviceBrowser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowserWillSuspendOperations : browser)
    }
    unsafe fn deviceBrowserDidSuspendOperations_(&self, browser: ICDeviceBrowser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowserDidSuspendOperations : browser)
    }
    unsafe fn deviceBrowserDidCancelSuspendOperations_(&self, browser: ICDeviceBrowser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowserDidCancelSuspendOperations : browser)
    }
    unsafe fn deviceBrowserDidResumeOperations_(&self, browser: ICDeviceBrowser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowserDidResumeOperations : browser)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICDeviceBrowser(pub id);
impl std::ops::Deref for ICDeviceBrowser {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICDeviceBrowser {}
impl ICDeviceBrowser {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICDeviceBrowser").unwrap(), alloc) })
    }
}
impl INSObject for ICDeviceBrowser {}
impl PNSObject for ICDeviceBrowser {}
impl std::convert::TryFrom<NSObject> for ICDeviceBrowser {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ICDeviceBrowser, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICDeviceBrowser").unwrap()) };
        if is_kind_of {
            Ok(ICDeviceBrowser(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ICDeviceBrowser")
        }
    }
}
impl IICDeviceBrowser for ICDeviceBrowser {}
pub trait IICDeviceBrowser: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn requestContentsAuthorizationWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestContentsAuthorizationWithCompletion : completion)
    }
    unsafe fn requestControlAuthorizationWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestControlAuthorizationWithCompletion : completion)
    }
    unsafe fn resetContentsAuthorizationWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetContentsAuthorizationWithCompletion : completion)
    }
    unsafe fn resetControlAuthorizationWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetControlAuthorizationWithCompletion : completion)
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
    unsafe fn isBrowsing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBrowsing)
    }
    unsafe fn isSuspended(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSuspended)
    }
    unsafe fn browsedDeviceTypeMask(&self) -> ICDeviceTypeMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browsedDeviceTypeMask)
    }
    unsafe fn setBrowsedDeviceTypeMask_(&self, browsedDeviceTypeMask: ICDeviceTypeMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrowsedDeviceTypeMask : browsedDeviceTypeMask)
    }
    unsafe fn devices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, devices)
    }
    unsafe fn preferredDevice(&self) -> ICDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDevice)
    }
    unsafe fn contentsAuthorizationStatus(&self) -> ICAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsAuthorizationStatus)
    }
    unsafe fn controlAuthorizationStatus(&self) -> ICAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlAuthorizationStatus)
    }
}
pub type ICScannerFunctionalUnitType = NSUInteger;
pub type ICScannerMeasurementUnit = NSUInteger;
pub type ICScannerBitDepth = NSUInteger;
pub type ICScannerColorDataFormatType = NSUInteger;
pub type ICScannerPixelDataType = NSUInteger;
pub type ICScannerDocumentType = NSUInteger;
pub type ICScannerFunctionalUnitState = NSUInteger;
pub type ICScannerFeatureType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFeature(pub id);
impl std::ops::Deref for ICScannerFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFeature {}
impl ICScannerFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFeature").unwrap(), alloc) })
    }
}
impl INSObject for ICScannerFeature {}
impl PNSObject for ICScannerFeature {}
impl std::convert::TryFrom<NSObject> for ICScannerFeature {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ICScannerFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFeature").unwrap()) };
        if is_kind_of {
            Ok(ICScannerFeature(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ICScannerFeature")
        }
    }
}
impl IICScannerFeature for ICScannerFeature {}
pub trait IICScannerFeature: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> ICScannerFeatureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn internalName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, internalName)
    }
    unsafe fn humanReadableName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, humanReadableName)
    }
    unsafe fn tooltip(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tooltip)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFeatureEnumeration(pub id);
impl std::ops::Deref for ICScannerFeatureEnumeration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFeatureEnumeration {}
impl ICScannerFeatureEnumeration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFeatureEnumeration").unwrap(), alloc) })
    }
}
impl IICScannerFeature for ICScannerFeatureEnumeration {}
impl From<ICScannerFeatureEnumeration> for ICScannerFeature {
    fn from(child: ICScannerFeatureEnumeration) -> ICScannerFeature {
        ICScannerFeature(child.0)
    }
}
impl std::convert::TryFrom<ICScannerFeature> for ICScannerFeatureEnumeration {
    type Error = &'static str;
    fn try_from(parent: ICScannerFeature) -> Result<ICScannerFeatureEnumeration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFeatureEnumeration").unwrap()) };
        if is_kind_of {
            Ok(ICScannerFeatureEnumeration(parent.0))
        } else {
            Err("This ICScannerFeature cannot be downcasted to ICScannerFeatureEnumeration")
        }
    }
}
impl INSObject for ICScannerFeatureEnumeration {}
impl PNSObject for ICScannerFeatureEnumeration {}
impl IICScannerFeatureEnumeration for ICScannerFeatureEnumeration {}
pub trait IICScannerFeatureEnumeration: Sized + std::ops::Deref {
    unsafe fn currentValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentValue)
    }
    unsafe fn setCurrentValue_(&self, currentValue: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentValue : currentValue)
    }
    unsafe fn defaultValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultValue)
    }
    unsafe fn values(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, values)
    }
    unsafe fn menuItemLabels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menuItemLabels)
    }
    unsafe fn menuItemLabelsTooltips(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menuItemLabelsTooltips)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFeatureRange(pub id);
impl std::ops::Deref for ICScannerFeatureRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFeatureRange {}
impl ICScannerFeatureRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFeatureRange").unwrap(), alloc) })
    }
}
impl IICScannerFeature for ICScannerFeatureRange {}
impl std::convert::TryFrom<ICScannerFeature> for ICScannerFeatureRange {
    type Error = &'static str;
    fn try_from(parent: ICScannerFeature) -> Result<ICScannerFeatureRange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFeatureRange").unwrap()) };
        if is_kind_of {
            Ok(ICScannerFeatureRange(parent.0))
        } else {
            Err("This ICScannerFeature cannot be downcasted to ICScannerFeatureRange")
        }
    }
}
impl INSObject for ICScannerFeatureRange {}
impl PNSObject for ICScannerFeatureRange {}
impl IICScannerFeatureRange for ICScannerFeatureRange {}
pub trait IICScannerFeatureRange: Sized + std::ops::Deref {
    unsafe fn currentValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentValue)
    }
    unsafe fn setCurrentValue_(&self, currentValue: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentValue : currentValue)
    }
    unsafe fn defaultValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultValue)
    }
    unsafe fn minValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minValue)
    }
    unsafe fn maxValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxValue)
    }
    unsafe fn stepSize(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stepSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFeatureBoolean(pub id);
impl std::ops::Deref for ICScannerFeatureBoolean {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFeatureBoolean {}
impl ICScannerFeatureBoolean {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFeatureBoolean").unwrap(), alloc) })
    }
}
impl IICScannerFeature for ICScannerFeatureBoolean {}
impl std::convert::TryFrom<ICScannerFeature> for ICScannerFeatureBoolean {
    type Error = &'static str;
    fn try_from(parent: ICScannerFeature) -> Result<ICScannerFeatureBoolean, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFeatureBoolean").unwrap()) };
        if is_kind_of {
            Ok(ICScannerFeatureBoolean(parent.0))
        } else {
            Err("This ICScannerFeature cannot be downcasted to ICScannerFeatureBoolean")
        }
    }
}
impl INSObject for ICScannerFeatureBoolean {}
impl PNSObject for ICScannerFeatureBoolean {}
impl IICScannerFeatureBoolean for ICScannerFeatureBoolean {}
pub trait IICScannerFeatureBoolean: Sized + std::ops::Deref {
    unsafe fn value(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFeatureTemplate(pub id);
impl std::ops::Deref for ICScannerFeatureTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFeatureTemplate {}
impl ICScannerFeatureTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFeatureTemplate").unwrap(), alloc) })
    }
}
impl IICScannerFeature for ICScannerFeatureTemplate {}
impl std::convert::TryFrom<ICScannerFeature> for ICScannerFeatureTemplate {
    type Error = &'static str;
    fn try_from(parent: ICScannerFeature) -> Result<ICScannerFeatureTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFeatureTemplate").unwrap()) };
        if is_kind_of {
            Ok(ICScannerFeatureTemplate(parent.0))
        } else {
            Err("This ICScannerFeature cannot be downcasted to ICScannerFeatureTemplate")
        }
    }
}
impl INSObject for ICScannerFeatureTemplate {}
impl PNSObject for ICScannerFeatureTemplate {}
impl IICScannerFeatureTemplate for ICScannerFeatureTemplate {}
pub trait IICScannerFeatureTemplate: Sized + std::ops::Deref {
    unsafe fn targets(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targets)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFunctionalUnit(pub id);
impl std::ops::Deref for ICScannerFunctionalUnit {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFunctionalUnit {}
impl ICScannerFunctionalUnit {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnit").unwrap(), alloc) })
    }
}
impl INSObject for ICScannerFunctionalUnit {}
impl PNSObject for ICScannerFunctionalUnit {}
impl std::convert::TryFrom<NSObject> for ICScannerFunctionalUnit {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ICScannerFunctionalUnit, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnit").unwrap()) };
        if is_kind_of {
            Ok(ICScannerFunctionalUnit(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ICScannerFunctionalUnit")
        }
    }
}
impl IICScannerFunctionalUnit for ICScannerFunctionalUnit {}
pub trait IICScannerFunctionalUnit: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> ICScannerFunctionalUnitType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn pixelDataType(&self) -> ICScannerPixelDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelDataType)
    }
    unsafe fn setPixelDataType_(&self, pixelDataType: ICScannerPixelDataType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelDataType : pixelDataType)
    }
    unsafe fn supportedBitDepths(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedBitDepths)
    }
    unsafe fn bitDepth(&self) -> ICScannerBitDepth
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bitDepth)
    }
    unsafe fn setBitDepth_(&self, bitDepth: ICScannerBitDepth)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBitDepth : bitDepth)
    }
    unsafe fn supportedMeasurementUnits(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedMeasurementUnits)
    }
    unsafe fn measurementUnit(&self) -> ICScannerMeasurementUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, measurementUnit)
    }
    unsafe fn setMeasurementUnit_(&self, measurementUnit: ICScannerMeasurementUnit)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeasurementUnit : measurementUnit)
    }
    unsafe fn supportedResolutions(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedResolutions)
    }
    unsafe fn preferredResolutions(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredResolutions)
    }
    unsafe fn resolution(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolution)
    }
    unsafe fn setResolution_(&self, resolution: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResolution : resolution)
    }
    unsafe fn nativeXResolution(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nativeXResolution)
    }
    unsafe fn nativeYResolution(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nativeYResolution)
    }
    unsafe fn supportedScaleFactors(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedScaleFactors)
    }
    unsafe fn preferredScaleFactors(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredScaleFactors)
    }
    unsafe fn scaleFactor(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn setScaleFactor_(&self, scaleFactor: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleFactor : scaleFactor)
    }
    unsafe fn templates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, templates)
    }
    unsafe fn vendorFeatures(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vendorFeatures)
    }
    unsafe fn physicalSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicalSize)
    }
    unsafe fn scanArea(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scanArea)
    }
    unsafe fn setScanArea_(&self, scanArea: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScanArea : scanArea)
    }
    unsafe fn scanAreaOrientation(&self) -> ICEXIFOrientationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scanAreaOrientation)
    }
    unsafe fn setScanAreaOrientation_(&self, scanAreaOrientation: ICEXIFOrientationType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScanAreaOrientation : scanAreaOrientation)
    }
    unsafe fn acceptsThresholdForBlackAndWhiteScanning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptsThresholdForBlackAndWhiteScanning)
    }
    unsafe fn usesThresholdForBlackAndWhiteScanning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesThresholdForBlackAndWhiteScanning)
    }
    unsafe fn setUsesThresholdForBlackAndWhiteScanning_(
        &self,
        usesThresholdForBlackAndWhiteScanning: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesThresholdForBlackAndWhiteScanning : usesThresholdForBlackAndWhiteScanning)
    }
    unsafe fn defaultThresholdForBlackAndWhiteScanning(&self) -> ::std::os::raw::c_uchar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultThresholdForBlackAndWhiteScanning)
    }
    unsafe fn thresholdForBlackAndWhiteScanning(&self) -> ::std::os::raw::c_uchar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thresholdForBlackAndWhiteScanning)
    }
    unsafe fn setThresholdForBlackAndWhiteScanning_(
        &self,
        thresholdForBlackAndWhiteScanning: ::std::os::raw::c_uchar,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThresholdForBlackAndWhiteScanning : thresholdForBlackAndWhiteScanning)
    }
    unsafe fn state(&self) -> ICScannerFunctionalUnitState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn scanInProgress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scanInProgress)
    }
    unsafe fn scanProgressPercentDone(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scanProgressPercentDone)
    }
    unsafe fn canPerformOverviewScan(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canPerformOverviewScan)
    }
    unsafe fn overviewScanInProgress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overviewScanInProgress)
    }
    unsafe fn overviewImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overviewImage)
    }
    unsafe fn overviewResolution(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overviewResolution)
    }
    unsafe fn setOverviewResolution_(&self, overviewResolution: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverviewResolution : overviewResolution)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFunctionalUnitFlatbed(pub id);
impl std::ops::Deref for ICScannerFunctionalUnitFlatbed {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFunctionalUnitFlatbed {}
impl ICScannerFunctionalUnitFlatbed {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnitFlatbed").unwrap(), alloc) })
    }
}
impl IICScannerFunctionalUnit for ICScannerFunctionalUnitFlatbed {}
impl From<ICScannerFunctionalUnitFlatbed> for ICScannerFunctionalUnit {
    fn from(child: ICScannerFunctionalUnitFlatbed) -> ICScannerFunctionalUnit {
        ICScannerFunctionalUnit(child.0)
    }
}
impl std::convert::TryFrom<ICScannerFunctionalUnit> for ICScannerFunctionalUnitFlatbed {
    type Error = &'static str;
    fn try_from(
        parent: ICScannerFunctionalUnit,
    ) -> Result<ICScannerFunctionalUnitFlatbed, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnitFlatbed").unwrap())
        };
        if is_kind_of {
            Ok(ICScannerFunctionalUnitFlatbed(parent.0))
        } else {
            Err ("This ICScannerFunctionalUnit cannot be downcasted to ICScannerFunctionalUnitFlatbed" ,)
        }
    }
}
impl INSObject for ICScannerFunctionalUnitFlatbed {}
impl PNSObject for ICScannerFunctionalUnitFlatbed {}
impl IICScannerFunctionalUnitFlatbed for ICScannerFunctionalUnitFlatbed {}
pub trait IICScannerFunctionalUnitFlatbed: Sized + std::ops::Deref {
    unsafe fn supportedDocumentTypes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedDocumentTypes)
    }
    unsafe fn documentType(&self) -> ICScannerDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentType)
    }
    unsafe fn setDocumentType_(&self, documentType: ICScannerDocumentType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentType : documentType)
    }
    unsafe fn documentSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFunctionalUnitPositiveTransparency(pub id);
impl std::ops::Deref for ICScannerFunctionalUnitPositiveTransparency {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFunctionalUnitPositiveTransparency {}
impl ICScannerFunctionalUnitPositiveTransparency {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnitPositiveTransparency").unwrap(), alloc) })
    }
}
impl IICScannerFunctionalUnit for ICScannerFunctionalUnitPositiveTransparency {}
impl std::convert::TryFrom<ICScannerFunctionalUnit>
    for ICScannerFunctionalUnitPositiveTransparency
{
    type Error = &'static str;
    fn try_from(
        parent: ICScannerFunctionalUnit,
    ) -> Result<ICScannerFunctionalUnitPositiveTransparency, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnitPositiveTransparency").unwrap())
        };
        if is_kind_of {
            Ok(ICScannerFunctionalUnitPositiveTransparency(parent.0))
        } else {
            Err ("This ICScannerFunctionalUnit cannot be downcasted to ICScannerFunctionalUnitPositiveTransparency" ,)
        }
    }
}
impl INSObject for ICScannerFunctionalUnitPositiveTransparency {}
impl PNSObject for ICScannerFunctionalUnitPositiveTransparency {}
impl IICScannerFunctionalUnitPositiveTransparency for ICScannerFunctionalUnitPositiveTransparency {}
pub trait IICScannerFunctionalUnitPositiveTransparency: Sized + std::ops::Deref {
    unsafe fn supportedDocumentTypes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedDocumentTypes)
    }
    unsafe fn documentType(&self) -> ICScannerDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentType)
    }
    unsafe fn setDocumentType_(&self, documentType: ICScannerDocumentType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentType : documentType)
    }
    unsafe fn documentSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFunctionalUnitNegativeTransparency(pub id);
impl std::ops::Deref for ICScannerFunctionalUnitNegativeTransparency {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFunctionalUnitNegativeTransparency {}
impl ICScannerFunctionalUnitNegativeTransparency {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnitNegativeTransparency").unwrap(), alloc) })
    }
}
impl IICScannerFunctionalUnit for ICScannerFunctionalUnitNegativeTransparency {}
impl std::convert::TryFrom<ICScannerFunctionalUnit>
    for ICScannerFunctionalUnitNegativeTransparency
{
    type Error = &'static str;
    fn try_from(
        parent: ICScannerFunctionalUnit,
    ) -> Result<ICScannerFunctionalUnitNegativeTransparency, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnitNegativeTransparency").unwrap())
        };
        if is_kind_of {
            Ok(ICScannerFunctionalUnitNegativeTransparency(parent.0))
        } else {
            Err ("This ICScannerFunctionalUnit cannot be downcasted to ICScannerFunctionalUnitNegativeTransparency" ,)
        }
    }
}
impl INSObject for ICScannerFunctionalUnitNegativeTransparency {}
impl PNSObject for ICScannerFunctionalUnitNegativeTransparency {}
impl IICScannerFunctionalUnitNegativeTransparency for ICScannerFunctionalUnitNegativeTransparency {}
pub trait IICScannerFunctionalUnitNegativeTransparency: Sized + std::ops::Deref {
    unsafe fn supportedDocumentTypes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedDocumentTypes)
    }
    unsafe fn documentType(&self) -> ICScannerDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentType)
    }
    unsafe fn setDocumentType_(&self, documentType: ICScannerDocumentType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentType : documentType)
    }
    unsafe fn documentSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerFunctionalUnitDocumentFeeder(pub id);
impl std::ops::Deref for ICScannerFunctionalUnitDocumentFeeder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerFunctionalUnitDocumentFeeder {}
impl ICScannerFunctionalUnitDocumentFeeder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnitDocumentFeeder").unwrap(), alloc) })
    }
}
impl IICScannerFunctionalUnit for ICScannerFunctionalUnitDocumentFeeder {}
impl std::convert::TryFrom<ICScannerFunctionalUnit> for ICScannerFunctionalUnitDocumentFeeder {
    type Error = &'static str;
    fn try_from(
        parent: ICScannerFunctionalUnit,
    ) -> Result<ICScannerFunctionalUnitDocumentFeeder, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerFunctionalUnitDocumentFeeder").unwrap())
        };
        if is_kind_of {
            Ok(ICScannerFunctionalUnitDocumentFeeder(parent.0))
        } else {
            Err ("This ICScannerFunctionalUnit cannot be downcasted to ICScannerFunctionalUnitDocumentFeeder" ,)
        }
    }
}
impl INSObject for ICScannerFunctionalUnitDocumentFeeder {}
impl PNSObject for ICScannerFunctionalUnitDocumentFeeder {}
impl IICScannerFunctionalUnitDocumentFeeder for ICScannerFunctionalUnitDocumentFeeder {}
pub trait IICScannerFunctionalUnitDocumentFeeder: Sized + std::ops::Deref {
    unsafe fn supportedDocumentTypes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedDocumentTypes)
    }
    unsafe fn documentType(&self) -> ICScannerDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentType)
    }
    unsafe fn setDocumentType_(&self, documentType: ICScannerDocumentType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentType : documentType)
    }
    unsafe fn documentSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentSize)
    }
    unsafe fn supportsDuplexScanning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsDuplexScanning)
    }
    unsafe fn duplexScanningEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duplexScanningEnabled)
    }
    unsafe fn setDuplexScanningEnabled_(&self, duplexScanningEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDuplexScanningEnabled : duplexScanningEnabled)
    }
    unsafe fn documentLoaded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentLoaded)
    }
    unsafe fn oddPageOrientation(&self) -> ICEXIFOrientationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, oddPageOrientation)
    }
    unsafe fn setOddPageOrientation_(&self, oddPageOrientation: ICEXIFOrientationType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOddPageOrientation : oddPageOrientation)
    }
    unsafe fn evenPageOrientation(&self) -> ICEXIFOrientationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, evenPageOrientation)
    }
    unsafe fn setEvenPageOrientation_(&self, evenPageOrientation: ICEXIFOrientationType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEvenPageOrientation : evenPageOrientation)
    }
    unsafe fn reverseFeederPageOrder(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reverseFeederPageOrder)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerBandData(pub id);
impl std::ops::Deref for ICScannerBandData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerBandData {}
impl ICScannerBandData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerBandData").unwrap(), alloc) })
    }
}
impl INSObject for ICScannerBandData {}
impl PNSObject for ICScannerBandData {}
impl std::convert::TryFrom<NSObject> for ICScannerBandData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ICScannerBandData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerBandData").unwrap()) };
        if is_kind_of {
            Ok(ICScannerBandData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ICScannerBandData")
        }
    }
}
impl IICScannerBandData for ICScannerBandData {}
pub trait IICScannerBandData: Sized + std::ops::Deref {
    unsafe fn fullImageWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullImageWidth)
    }
    unsafe fn fullImageHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullImageHeight)
    }
    unsafe fn bitsPerPixel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bitsPerPixel)
    }
    unsafe fn bitsPerComponent(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bitsPerComponent)
    }
    unsafe fn numComponents(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numComponents)
    }
    unsafe fn isBigEndian(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBigEndian)
    }
    unsafe fn pixelDataType(&self) -> ICScannerPixelDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelDataType)
    }
    unsafe fn colorSyncProfilePath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSyncProfilePath)
    }
    unsafe fn bytesPerRow(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesPerRow)
    }
    unsafe fn dataStartRow(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataStartRow)
    }
    unsafe fn dataNumRows(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataNumRows)
    }
    unsafe fn dataSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSize)
    }
    unsafe fn dataBuffer(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataBuffer)
    }
}
pub type ICScannerTransferMode = NSUInteger;
pub trait PICScannerDeviceDelegate: Sized + std::ops::Deref {
    unsafe fn scannerDeviceDidBecomeAvailable_(&self, scanner: ICScannerDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDeviceDidBecomeAvailable : scanner)
    }
    unsafe fn scannerDevice_didSelectFunctionalUnit_error_(
        &self,
        scanner: ICScannerDevice,
        functionalUnit: ICScannerFunctionalUnit,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDevice : scanner, didSelectFunctionalUnit : functionalUnit, error : error)
    }
    unsafe fn scannerDevice_didScanToURL_data_(
        &self,
        scanner: ICScannerDevice,
        url: NSURL,
        data: NSData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDevice : scanner, didScanToURL : url, data : data)
    }
    unsafe fn scannerDevice_didScanToURL_(&self, scanner: ICScannerDevice, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDevice : scanner, didScanToURL : url)
    }
    unsafe fn scannerDevice_didScanToBandData_(
        &self,
        scanner: ICScannerDevice,
        data: ICScannerBandData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDevice : scanner, didScanToBandData : data)
    }
    unsafe fn scannerDevice_didCompleteOverviewScanWithError_(
        &self,
        scanner: ICScannerDevice,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDevice : scanner, didCompleteOverviewScanWithError : error)
    }
    unsafe fn scannerDevice_didCompleteScanWithError_(
        &self,
        scanner: ICScannerDevice,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDevice : scanner, didCompleteScanWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ICScannerDevice(pub id);
impl std::ops::Deref for ICScannerDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ICScannerDevice {}
impl ICScannerDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ICScannerDevice").unwrap(), alloc) })
    }
}
impl IICDevice for ICScannerDevice {}
impl std::convert::TryFrom<ICDevice> for ICScannerDevice {
    type Error = &'static str;
    fn try_from(parent: ICDevice) -> Result<ICScannerDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ICScannerDevice").unwrap()) };
        if is_kind_of {
            Ok(ICScannerDevice(parent.0))
        } else {
            Err("This ICDevice cannot be downcasted to ICScannerDevice")
        }
    }
}
impl INSObject for ICScannerDevice {}
impl PNSObject for ICScannerDevice {}
impl IICScannerDevice for ICScannerDevice {}
pub trait IICScannerDevice: Sized + std::ops::Deref {
    unsafe fn requestOpenSessionWithCredentials_password_(
        &self,
        username: NSString,
        password: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestOpenSessionWithCredentials : username, password : password)
    }
    unsafe fn requestSelectFunctionalUnit_(&self, type_: ICScannerFunctionalUnitType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestSelectFunctionalUnit : type_)
    }
    unsafe fn requestOverviewScan(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestOverviewScan)
    }
    unsafe fn requestScan(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestScan)
    }
    unsafe fn cancelScan(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelScan)
    }
    unsafe fn availableFunctionalUnitTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableFunctionalUnitTypes)
    }
    unsafe fn selectedFunctionalUnit(&self) -> ICScannerFunctionalUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedFunctionalUnit)
    }
    unsafe fn transferMode(&self) -> ICScannerTransferMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transferMode)
    }
    unsafe fn setTransferMode_(&self, transferMode: ICScannerTransferMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransferMode : transferMode)
    }
    unsafe fn maxMemoryBandSize(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxMemoryBandSize)
    }
    unsafe fn setMaxMemoryBandSize_(&self, maxMemoryBandSize: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxMemoryBandSize : maxMemoryBandSize)
    }
    unsafe fn downloadsDirectory(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadsDirectory)
    }
    unsafe fn setDownloadsDirectory_(&self, downloadsDirectory: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownloadsDirectory : downloadsDirectory)
    }
    unsafe fn documentName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentName)
    }
    unsafe fn setDocumentName_(&self, documentName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentName : documentName)
    }
    unsafe fn documentUTI(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentUTI)
    }
    unsafe fn setDocumentUTI_(&self, documentUTI: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentUTI : documentUTI)
    }
    unsafe fn defaultUsername(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultUsername)
    }
    unsafe fn setDefaultUsername_(&self, defaultUsername: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultUsername : defaultUsername)
    }
}
unsafe extern "C" {
    pub static ICTransportTypeUSB: ICDeviceTransport;
}
unsafe extern "C" {
    pub static ICTransportTypeMassStorage: ICDeviceTransport;
}
unsafe extern "C" {
    pub static ICTransportTypeExFAT: ICDeviceTransport;
}
unsafe extern "C" {
    pub static ICTransportTypeTCPIP: ICDeviceTransport;
}
unsafe extern "C" {
    pub static ICTransportTypeProximity: ICDeviceTransport;
}
unsafe extern "C" {
    pub static ICStatusNotificationKey: ICDeviceStatus;
}
unsafe extern "C" {
    pub static ICDeviceCanEjectOrDisconnect: ICDeviceCapability;
}
unsafe extern "C" {
    pub static ICEnumerationChronologicalOrder: ICSessionOptions;
}
unsafe extern "C" {
    pub static ICDeviceLocationDescriptionUSB: ICDeviceLocationOptions;
}
unsafe extern "C" {
    pub static ICDeviceLocationDescriptionFireWire: ICDeviceLocationOptions;
}
unsafe extern "C" {
    pub static ICDeviceLocationDescriptionBluetooth: ICDeviceLocationOptions;
}
unsafe extern "C" {
    pub static ICDeviceLocationDescriptionMassStorage: ICDeviceLocationOptions;
}
unsafe extern "C" {
    pub static ICErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static ICImageSourceThumbnailMaxPixelSize: ICCameraItemThumbnailOption;
}
unsafe extern "C" {
    pub static ICImageSourceShouldCache: ICCameraItemThumbnailOption;
}
unsafe extern "C" {
    pub static ICDownloadsDirectoryURL: ICDownloadOption;
}
unsafe extern "C" {
    pub static ICSaveAsFilename: ICDownloadOption;
}
unsafe extern "C" {
    pub static ICSavedFilename: ICDownloadOption;
}
unsafe extern "C" {
    pub static ICSavedAncillaryFiles: ICDownloadOption;
}
unsafe extern "C" {
    pub static ICOverwrite: ICDownloadOption;
}
unsafe extern "C" {
    pub static ICDeleteAfterSuccessfulDownload: ICDownloadOption;
}
unsafe extern "C" {
    pub static ICDownloadSidecarFiles: ICDownloadOption;
}
unsafe extern "C" {
    pub static ICTruncateAfterSuccessfulDownload: ICDownloadOption;
}
unsafe extern "C" {
    pub static ICCameraDeviceCanTakePicture: ICDeviceCapability;
}
unsafe extern "C" {
    pub static ICCameraDeviceCanTakePictureUsingShutterReleaseOnCamera: ICDeviceCapability;
}
unsafe extern "C" {
    pub static ICCameraDeviceCanDeleteOneFile: ICDeviceCapability;
}
unsafe extern "C" {
    pub static ICCameraDeviceCanDeleteAllFiles: ICDeviceCapability;
}
unsafe extern "C" {
    pub static ICCameraDeviceCanSyncClock: ICDeviceCapability;
}
unsafe extern "C" {
    pub static ICCameraDeviceCanReceiveFile: ICDeviceCapability;
}
unsafe extern "C" {
    pub static ICCameraDeviceCanAcceptPTPCommands: ICDeviceCapability;
}
unsafe extern "C" {
    pub static ICCameraDeviceSupportsHEIF: ICDeviceCapability;
}
unsafe extern "C" {
    pub static ICDeleteSuccessful: ICDeleteResult;
}
unsafe extern "C" {
    pub static ICDeleteCanceled: ICDeleteResult;
}
unsafe extern "C" {
    pub static ICDeleteFailed: ICDeleteResult;
}
unsafe extern "C" {
    pub static ICDeleteErrorReadOnly: ICDeleteError;
}
unsafe extern "C" {
    pub static ICDeleteErrorFileMissing: ICDeleteError;
}
unsafe extern "C" {
    pub static ICDeleteErrorDeviceMissing: ICDeleteError;
}
unsafe extern "C" {
    pub static ICDeleteErrorCanceled: ICDeleteError;
}
unsafe extern "C" {
    pub static ICAuthorizationStatusNotDetermined: ICAuthorizationStatus;
}
unsafe extern "C" {
    pub static ICAuthorizationStatusRestricted: ICAuthorizationStatus;
}
unsafe extern "C" {
    pub static ICAuthorizationStatusDenied: ICAuthorizationStatus;
}
unsafe extern "C" {
    pub static ICAuthorizationStatusAuthorized: ICAuthorizationStatus;
}

unsafe impl objc2::encode::RefEncode for ICDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICCameraItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICCameraItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICCameraFile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICCameraFile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICCameraFolder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICCameraFolder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICCameraDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICCameraDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICDeviceBrowser {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICDeviceBrowser {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFeatureEnumeration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFeatureEnumeration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFeatureRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFeatureRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFeatureBoolean {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFeatureBoolean {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFeatureTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFeatureTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFunctionalUnit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFunctionalUnit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFunctionalUnitFlatbed {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFunctionalUnitFlatbed {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFunctionalUnitPositiveTransparency {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFunctionalUnitPositiveTransparency {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFunctionalUnitNegativeTransparency {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFunctionalUnitNegativeTransparency {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerFunctionalUnitDocumentFeeder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerFunctionalUnitDocumentFeeder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerBandData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerBandData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ICScannerDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICScannerDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
