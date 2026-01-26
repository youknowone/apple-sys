#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::ApplicationServices::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type DRTypeRef = CFTypeRef;
pub type DRRefConRetainCallback = ::std::option::Option<
    unsafe extern "C" fn(refCon: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
>;
pub type DRRefConReleaseCallback =
    ::std::option::Option<unsafe extern "C" fn(refCon: *const ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DRRefConCallbacks {
    pub version: ::std::os::raw::c_ulong,
    pub retain: DRRefConRetainCallback,
    pub release: DRRefConReleaseCallback,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DRDevice {
    _unused: [u8; 0],
}
pub type DRDeviceRef = *mut __DRDevice;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DRTrack {
    _unused: [u8; 0],
}
pub type DRTrackRef = *mut __DRTrack;
pub type DRTrackMessage = UInt32;
pub type DRTrackCallbackProc = ::std::option::Option<
    unsafe extern "C" fn(
        track: DRTrackRef,
        message: DRTrackMessage,
        ioParam: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DRTrackProductionInfo {
    pub buffer: *mut ::std::os::raw::c_void,
    pub reqCount: UInt32,
    pub actCount: UInt32,
    pub flags: UInt32,
    pub blockSize: UInt32,
    pub requestedAddress: UInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DRBurn {
    _unused: [u8; 0],
}
pub type DRBurnRef = *mut __DRBurn;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DRCDTextBlock {
    _unused: [u8; 0],
}
pub type DRCDTextBlockRef = *const __DRCDTextBlock;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DRErase {
    _unused: [u8; 0],
}
pub type DREraseRef = *mut __DRErase;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DRNotificationCenter {
    _unused: [u8; 0],
}
pub type DRNotificationCenterRef = *mut __DRNotificationCenter;
pub type DRNotificationCallback = ::std::option::Option<
    unsafe extern "C" fn(
        center: DRNotificationCenterRef,
        observer: *mut ::std::os::raw::c_void,
        name: CFStringRef,
        object: DRTypeRef,
        info: CFDictionaryRef,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DRFile {
    _unused: [u8; 0],
}
pub type DRFileRef = *mut __DRFile;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DRFolder {
    _unused: [u8; 0],
}
pub type DRFolderRef = *mut __DRFolder;
pub type DRFSObjectRef = DRTypeRef;
pub type DRFilesystemMask = UInt32;
pub type DRLinkType = UInt32;
pub type DRFileMessage = UInt32;
pub type DRFileForkIndex = UInt32;
pub type DRFileForkSizeQuery = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DRFileForkSizeInfo {
    pub fork: DRFileForkIndex,
    pub query: DRFileForkSizeQuery,
    pub size: UInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DRFileProductionInfo {
    pub requestedAddress: UInt64,
    pub buffer: *mut ::std::os::raw::c_void,
    pub reqCount: UInt32,
    pub actCount: UInt32,
    pub blockSize: UInt32,
    pub fork: DRFileForkIndex,
}
pub type DRFileProc = ::std::option::Option<
    unsafe extern "C" fn(
        refCon: *mut ::std::os::raw::c_void,
        file: DRFileRef,
        message: DRFileMessage,
        ioParam: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type DRFilesystemTrackRef = DRTrackRef;
pub type DRAudioTrackRef = DRTrackRef;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRMSF(pub id);
impl std::ops::Deref for DRMSF {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRMSF {}
impl DRMSF {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRMSF").unwrap(), alloc) })
    }
}
impl INSNumber for DRMSF {}
impl std::convert::TryFrom<NSNumber> for DRMSF {
    type Error = &'static str;
    fn try_from(parent: NSNumber) -> Result<DRMSF, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRMSF").unwrap()) };
        if is_kind_of {
            Ok(DRMSF(parent.0))
        } else {
            Err("This NSNumber cannot be downcasted to DRMSF")
        }
    }
}
impl INSValue for DRMSF {}
impl PNSCopying for DRMSF {}
impl PNSSecureCoding for DRMSF {}
impl INSObject for DRMSF {}
impl PNSObject for DRMSF {}
impl IDRMSF for DRMSF {}
pub trait IDRMSF: Sized + std::ops::Deref {
    unsafe fn initWithFrames_(&self, frames: UInt32) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrames : frames)
    }
    unsafe fn initWithString_(&self, string: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithString : string)
    }
    unsafe fn minutes(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minutes)
    }
    unsafe fn seconds(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seconds)
    }
    unsafe fn frames(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frames)
    }
    unsafe fn sectors(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sectors)
    }
    unsafe fn msfByAdding_(&self, msf: DRMSF) -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, msfByAdding : msf)
    }
    unsafe fn msfBySubtracting_(&self, msf: DRMSF) -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, msfBySubtracting : msf)
    }
    unsafe fn description(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, description)
    }
    unsafe fn descriptionWithFormat_(&self, format: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, descriptionWithFormat : format)
    }
    unsafe fn isEqualToMSF_(&self, otherDRMSF: DRMSF) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToMSF : otherDRMSF)
    }
    unsafe fn msf() -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRMSF").unwrap(), msf)
    }
    unsafe fn msfWithFrames_(frames: UInt32) -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRMSF").unwrap(), msfWithFrames : frames)
    }
    unsafe fn msfWithString_(string: NSString) -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRMSF").unwrap(), msfWithString : string)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRDevice(pub id);
impl std::ops::Deref for DRDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRDevice {}
impl DRDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRDevice").unwrap(), alloc) })
    }
}
impl INSObject for DRDevice {}
impl PNSObject for DRDevice {}
impl std::convert::TryFrom<NSObject> for DRDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DRDevice, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRDevice").unwrap()) };
        if is_kind_of {
            Ok(DRDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DRDevice")
        }
    }
}
impl IDRDevice for DRDevice {}
pub trait IDRDevice: Sized + std::ops::Deref {
    unsafe fn isValid(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isValid)
    }
    unsafe fn info(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, info)
    }
    unsafe fn status(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn openTray(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, openTray)
    }
    unsafe fn closeTray(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeTray)
    }
    unsafe fn ejectMedia(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ejectMedia)
    }
    unsafe fn acquireExclusiveAccess(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acquireExclusiveAccess)
    }
    unsafe fn releaseExclusiveAccess(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseExclusiveAccess)
    }
    unsafe fn acquireMediaReservation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acquireMediaReservation)
    }
    unsafe fn releaseMediaReservation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseMediaReservation)
    }
    unsafe fn isEqualToDevice_(&self, otherDevice: DRDevice) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToDevice : otherDevice)
    }
    unsafe fn devices() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRDevice").unwrap(), devices)
    }
    unsafe fn deviceForBSDName_(bsdName: NSString) -> DRDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRDevice").unwrap(), deviceForBSDName : bsdName)
    }
    unsafe fn deviceForIORegistryEntryPath_(path: NSString) -> DRDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRDevice").unwrap(), deviceForIORegistryEntryPath : path)
    }
}
impl DRDevice_InfoConvenience for DRDevice {}
pub trait DRDevice_InfoConvenience: Sized + std::ops::Deref {
    unsafe fn writesCD(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writesCD)
    }
    unsafe fn writesDVD(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writesDVD)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn ioRegistryEntryPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ioRegistryEntryPath)
    }
}
impl DRDevice_StatusConvenience for DRDevice {}
pub trait DRDevice_StatusConvenience: Sized + std::ops::Deref {
    unsafe fn mediaIsPresent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaIsPresent)
    }
    unsafe fn mediaIsTransitioning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaIsTransitioning)
    }
    unsafe fn mediaIsBusy(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaIsBusy)
    }
    unsafe fn mediaType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaType)
    }
    unsafe fn mediaIsBlank(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaIsBlank)
    }
    unsafe fn mediaIsAppendable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaIsAppendable)
    }
    unsafe fn mediaIsOverwritable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaIsOverwritable)
    }
    unsafe fn mediaIsErasable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaIsErasable)
    }
    unsafe fn mediaIsReserved(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaIsReserved)
    }
    unsafe fn mediaSpaceOverwritable(&self) -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaSpaceOverwritable)
    }
    unsafe fn mediaSpaceUsed(&self) -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaSpaceUsed)
    }
    unsafe fn mediaSpaceFree(&self) -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaSpaceFree)
    }
    unsafe fn trayIsOpen(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trayIsOpen)
    }
    unsafe fn bsdName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bsdName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRTrack(pub id);
impl std::ops::Deref for DRTrack {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRTrack {}
impl DRTrack {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRTrack").unwrap(), alloc) })
    }
}
impl INSObject for DRTrack {}
impl PNSObject for DRTrack {}
impl std::convert::TryFrom<NSObject> for DRTrack {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DRTrack, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRTrack").unwrap()) };
        if is_kind_of {
            Ok(DRTrack(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DRTrack")
        }
    }
}
impl IDRTrack for DRTrack {}
pub trait IDRTrack: Sized + std::ops::Deref {
    unsafe fn initWithProducer_(&self, producer: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProducer : producer)
    }
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setProperties_(&self, properties: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties)
    }
    unsafe fn testProductionSpeedForInterval_(&self, interval: NSTimeInterval) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, testProductionSpeedForInterval : interval)
    }
    unsafe fn testProductionSpeedForLength_(&self, length: u32) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, testProductionSpeedForLength : length)
    }
    unsafe fn estimateLength(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, estimateLength)
    }
}
impl DRTrack_PropertyConvenience for DRTrack {}
pub trait DRTrack_PropertyConvenience: Sized + std::ops::Deref {
    unsafe fn length(&self) -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn preGap(&self) -> DRMSF
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preGap)
    }
    unsafe fn setPreGap_(&self, preGap: DRMSF)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreGap : preGap)
    }
}
pub trait PDRTrackDataProduction: Sized + std::ops::Deref {
    unsafe fn estimateLengthOfTrack_(&self, track: DRTrack) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, estimateLengthOfTrack : track)
    }
    unsafe fn prepareTrack_forBurn_toMedia_(
        &self,
        track: DRTrack,
        burn: DRBurn,
        mediaInfo: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareTrack : track, forBurn : burn, toMedia : mediaInfo)
    }
    unsafe fn cleanupTrackAfterBurn_(&self, track: DRTrack)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cleanupTrackAfterBurn : track)
    }
    unsafe fn producePreGapForTrack_intoBuffer_length_atAddress_blockSize_ioFlags_(
        &self,
        track: DRTrack,
        buffer: *mut ::std::os::raw::c_char,
        bufferLength: u32,
        address: u64,
        blockSize: u32,
        flags: *mut u32,
    ) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, producePreGapForTrack : track, intoBuffer : buffer, length : bufferLength, atAddress : address, blockSize : blockSize, ioFlags : flags)
    }
    unsafe fn produceDataForTrack_intoBuffer_length_atAddress_blockSize_ioFlags_(
        &self,
        track: DRTrack,
        buffer: *mut ::std::os::raw::c_char,
        bufferLength: u32,
        address: u64,
        blockSize: u32,
        flags: *mut u32,
    ) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, produceDataForTrack : track, intoBuffer : buffer, length : bufferLength, atAddress : address, blockSize : blockSize, ioFlags : flags)
    }
    unsafe fn prepareTrackForVerification_(&self, track: DRTrack) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareTrackForVerification : track)
    }
    unsafe fn verifyPreGapForTrack_inBuffer_length_atAddress_blockSize_ioFlags_(
        &self,
        track: DRTrack,
        buffer: *const ::std::os::raw::c_char,
        bufferLength: u32,
        address: u64,
        blockSize: u32,
        flags: *mut u32,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, verifyPreGapForTrack : track, inBuffer : buffer, length : bufferLength, atAddress : address, blockSize : blockSize, ioFlags : flags)
    }
    unsafe fn verifyDataForTrack_inBuffer_length_atAddress_blockSize_ioFlags_(
        &self,
        track: DRTrack,
        buffer: *const ::std::os::raw::c_char,
        bufferLength: u32,
        address: u64,
        blockSize: u32,
        flags: *mut u32,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, verifyDataForTrack : track, inBuffer : buffer, length : bufferLength, atAddress : address, blockSize : blockSize, ioFlags : flags)
    }
    unsafe fn cleanupTrackAfterVerification_(&self, track: DRTrack) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cleanupTrackAfterVerification : track)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRBurn(pub id);
impl std::ops::Deref for DRBurn {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRBurn {}
impl DRBurn {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRBurn").unwrap(), alloc) })
    }
}
impl INSObject for DRBurn {}
impl PNSObject for DRBurn {}
impl std::convert::TryFrom<NSObject> for DRBurn {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DRBurn, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRBurn").unwrap()) };
        if is_kind_of {
            Ok(DRBurn(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DRBurn")
        }
    }
}
impl IDRBurn for DRBurn {}
pub trait IDRBurn: Sized + std::ops::Deref {
    unsafe fn initWithDevice_(&self, device: DRDevice) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDevice : device)
    }
    unsafe fn writeLayout_(&self, layout: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeLayout : layout)
    }
    unsafe fn status(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn abort(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, abort)
    }
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setProperties_(&self, properties: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties)
    }
    unsafe fn device(&self) -> DRDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn burnForDevice_(device: DRDevice) -> DRBurn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRBurn").unwrap(), burnForDevice : device)
    }
}
impl DRBurn_PropertyConvenienceMethods for DRBurn {}
pub trait DRBurn_PropertyConvenienceMethods: Sized + std::ops::Deref {
    unsafe fn requestedBurnSpeed(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestedBurnSpeed)
    }
    unsafe fn setRequestedBurnSpeed_(&self, speed: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequestedBurnSpeed : speed)
    }
    unsafe fn appendable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appendable)
    }
    unsafe fn setAppendable_(&self, appendable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppendable : appendable)
    }
    unsafe fn verifyDisc(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verifyDisc)
    }
    unsafe fn setVerifyDisc_(&self, verify: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVerifyDisc : verify)
    }
    unsafe fn completionAction(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionAction)
    }
    unsafe fn setCompletionAction_(&self, action: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionAction : action)
    }
}
impl DRBurn_ImageContentCreation for DRBurn {}
pub trait DRBurn_ImageContentCreation: Sized + std::ops::Deref {
    unsafe fn layoutForImageFile_(path: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRBurn").unwrap(), layoutForImageFile : path)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRCDTextBlock(pub id);
impl std::ops::Deref for DRCDTextBlock {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRCDTextBlock {}
impl DRCDTextBlock {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRCDTextBlock").unwrap(), alloc) })
    }
}
impl INSObject for DRCDTextBlock {}
impl PNSObject for DRCDTextBlock {}
impl std::convert::TryFrom<NSObject> for DRCDTextBlock {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DRCDTextBlock, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRCDTextBlock").unwrap()) };
        if is_kind_of {
            Ok(DRCDTextBlock(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DRCDTextBlock")
        }
    }
}
impl IDRCDTextBlock for DRCDTextBlock {}
pub trait IDRCDTextBlock: Sized + std::ops::Deref {
    unsafe fn initWithLanguage_encoding_(&self, lang: NSString, enc: NSStringEncoding) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLanguage : lang, encoding : enc)
    }
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setProperties_(&self, properties: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties)
    }
    unsafe fn trackDictionaries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackDictionaries)
    }
    unsafe fn setTrackDictionaries_(&self, tracks: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrackDictionaries : tracks)
    }
    unsafe fn objectForKey_ofTrack_(&self, key: NSString, trackIndex: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKey : key, ofTrack : trackIndex)
    }
    unsafe fn setObject_forKey_ofTrack_(&self, value: id, key: NSString, trackIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : value, forKey : key, ofTrack : trackIndex)
    }
    unsafe fn flatten(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flatten)
    }
    unsafe fn arrayOfCDTextBlocksFromPacks_(packs: NSData) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRCDTextBlock").unwrap(), arrayOfCDTextBlocksFromPacks : packs)
    }
    unsafe fn cdTextBlockWithLanguage_encoding_(
        lang: NSString,
        enc: NSStringEncoding,
    ) -> DRCDTextBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRCDTextBlock").unwrap(), cdTextBlockWithLanguage : lang, encoding : enc)
    }
}
impl DRCDTextBlock_PropertyConvenienceMethods for DRCDTextBlock {}
pub trait DRCDTextBlock_PropertyConvenienceMethods: Sized + std::ops::Deref {
    unsafe fn language(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn encoding(&self) -> NSStringEncoding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encoding)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRErase(pub id);
impl std::ops::Deref for DRErase {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRErase {}
impl DRErase {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRErase").unwrap(), alloc) })
    }
}
impl INSObject for DRErase {}
impl PNSObject for DRErase {}
impl std::convert::TryFrom<NSObject> for DRErase {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DRErase, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRErase").unwrap()) };
        if is_kind_of {
            Ok(DRErase(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DRErase")
        }
    }
}
impl IDRErase for DRErase {}
pub trait IDRErase: Sized + std::ops::Deref {
    unsafe fn initWithDevice_(&self, device: DRDevice) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDevice : device)
    }
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn status(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setProperties_(&self, properties: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties)
    }
    unsafe fn device(&self) -> DRDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn eraseForDevice_(device: DRDevice) -> DRErase
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRErase").unwrap(), eraseForDevice : device)
    }
}
impl DRErase_PropertyConvenienceMethods for DRErase {}
pub trait DRErase_PropertyConvenienceMethods: Sized + std::ops::Deref {
    unsafe fn eraseType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eraseType)
    }
    unsafe fn setEraseType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEraseType : type_)
    }
}
pub type DRFilesystemInclusionMask = UInt32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRFSObject(pub id);
impl std::ops::Deref for DRFSObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRFSObject {}
impl DRFSObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRFSObject").unwrap(), alloc) })
    }
}
impl INSObject for DRFSObject {}
impl PNSObject for DRFSObject {}
impl std::convert::TryFrom<NSObject> for DRFSObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DRFSObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRFSObject").unwrap()) };
        if is_kind_of {
            Ok(DRFSObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DRFSObject")
        }
    }
}
impl IDRFSObject for DRFSObject {}
pub trait IDRFSObject: Sized + std::ops::Deref {
    unsafe fn isVirtual(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVirtual)
    }
    unsafe fn sourcePath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePath)
    }
    unsafe fn parent(&self) -> DRFolder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parent)
    }
    unsafe fn baseName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseName)
    }
    unsafe fn setBaseName_(&self, baseName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaseName : baseName)
    }
    unsafe fn specificNameForFilesystem_(&self, filesystem: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, specificNameForFilesystem : filesystem)
    }
    unsafe fn specificNames(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specificNames)
    }
    unsafe fn setSpecificName_forFilesystem_(&self, name: NSString, filesystem: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecificName : name, forFilesystem : filesystem)
    }
    unsafe fn setSpecificNames_(&self, specificNames: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecificNames : specificNames)
    }
    unsafe fn mangledNameForFilesystem_(&self, filesystem: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mangledNameForFilesystem : filesystem)
    }
    unsafe fn mangledNames(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mangledNames)
    }
    unsafe fn propertyForKey_inFilesystem_mergeWithOtherFilesystems_(
        &self,
        key: NSString,
        filesystem: NSString,
        merge: BOOL,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, propertyForKey : key, inFilesystem : filesystem, mergeWithOtherFilesystems : merge)
    }
    unsafe fn propertiesForFilesystem_mergeWithOtherFilesystems_(
        &self,
        filesystem: NSString,
        merge: BOOL,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, propertiesForFilesystem : filesystem, mergeWithOtherFilesystems : merge)
    }
    unsafe fn setProperty_forKey_inFilesystem_(
        &self,
        property: id,
        key: NSString,
        filesystem: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperty : property, forKey : key, inFilesystem : filesystem)
    }
    unsafe fn setProperties_inFilesystem_(&self, properties: NSDictionary, filesystem: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties, inFilesystem : filesystem)
    }
    unsafe fn explicitFilesystemMask(&self) -> DRFilesystemInclusionMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, explicitFilesystemMask)
    }
    unsafe fn setExplicitFilesystemMask_(&self, mask: DRFilesystemInclusionMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplicitFilesystemMask : mask)
    }
    unsafe fn effectiveFilesystemMask(&self) -> DRFilesystemInclusionMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectiveFilesystemMask)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRFile(pub id);
impl std::ops::Deref for DRFile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRFile {}
impl DRFile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRFile").unwrap(), alloc) })
    }
}
impl IDRFSObject for DRFile {}
impl From<DRFile> for DRFSObject {
    fn from(child: DRFile) -> DRFSObject {
        DRFSObject(child.0)
    }
}
impl std::convert::TryFrom<DRFSObject> for DRFile {
    type Error = &'static str;
    fn try_from(parent: DRFSObject) -> Result<DRFile, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRFile").unwrap()) };
        if is_kind_of {
            Ok(DRFile(parent.0))
        } else {
            Err("This DRFSObject cannot be downcasted to DRFile")
        }
    }
}
impl INSObject for DRFile {}
impl PNSObject for DRFile {}
impl IDRFile for DRFile {}
pub trait IDRFile: Sized + std::ops::Deref {
    unsafe fn initWithPath_(&self, path: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPath : path)
    }
    unsafe fn fileWithPath_(path: NSString) -> DRFile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRFile").unwrap(), fileWithPath : path)
    }
}
impl DRFile_VirtualFiles for DRFile {}
pub trait DRFile_VirtualFiles: Sized + std::ops::Deref {
    unsafe fn initWithName_data_(&self, name: NSString, data: NSData) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, data : data)
    }
    unsafe fn initWithName_dataProducer_(&self, name: NSString, producer: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, dataProducer : producer)
    }
    unsafe fn virtualFileWithName_data_(name: NSString, data: NSData) -> DRFile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRFile").unwrap(), virtualFileWithName : name, data : data)
    }
    unsafe fn virtualFileWithName_dataProducer_(name: NSString, producer: id) -> DRFile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRFile").unwrap(), virtualFileWithName : name, dataProducer : producer)
    }
}
impl DRFile_VirtualLinks for DRFile {}
pub trait DRFile_VirtualLinks: Sized + std::ops::Deref {
    unsafe fn initWithLinkType_pointingTo_inFilesystem_(
        &self,
        linkType: NSString,
        original: DRFSObject,
        filesystem: NSString,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLinkType : linkType, pointingTo : original, inFilesystem : filesystem)
    }
    unsafe fn hardLinkPointingTo_inFilesystem_(original: DRFile, filesystem: NSString) -> DRFile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRFile").unwrap(), hardLinkPointingTo : original, inFilesystem : filesystem)
    }
    unsafe fn symLinkPointingTo_inFilesystem_(original: DRFSObject, filesystem: NSString) -> DRFile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRFile").unwrap(), symLinkPointingTo : original, inFilesystem : filesystem)
    }
    unsafe fn finderAliasPointingTo_inFilesystem_(
        original: DRFSObject,
        filesystem: NSString,
    ) -> DRFile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRFile").unwrap(), finderAliasPointingTo : original, inFilesystem : filesystem)
    }
}
pub type DRFileFork = UInt32;
pub trait PDRFileDataProduction: Sized + std::ops::Deref {
    unsafe fn calculateSizeOfFile_fork_estimating_(
        &self,
        file: DRFile,
        fork: DRFileFork,
        estimate: BOOL,
    ) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateSizeOfFile : file, fork : fork, estimating : estimate)
    }
    unsafe fn prepareFileForBurn_(&self, file: DRFile) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareFileForBurn : file)
    }
    unsafe fn produceFile_fork_intoBuffer_length_atAddress_blockSize_(
        &self,
        file: DRFile,
        fork: DRFileFork,
        buffer: *mut ::std::os::raw::c_char,
        bufferLength: u32,
        address: u64,
        blockSize: u32,
    ) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, produceFile : file, fork : fork, intoBuffer : buffer, length : bufferLength, atAddress : address, blockSize : blockSize)
    }
    unsafe fn prepareFileForVerification_(&self, file: DRFile) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareFileForVerification : file)
    }
    unsafe fn cleanupFileAfterBurn_(&self, file: DRFile)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cleanupFileAfterBurn : file)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRFolder(pub id);
impl std::ops::Deref for DRFolder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRFolder {}
impl DRFolder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRFolder").unwrap(), alloc) })
    }
}
impl IDRFSObject for DRFolder {}
impl std::convert::TryFrom<DRFSObject> for DRFolder {
    type Error = &'static str;
    fn try_from(parent: DRFSObject) -> Result<DRFolder, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRFolder").unwrap()) };
        if is_kind_of {
            Ok(DRFolder(parent.0))
        } else {
            Err("This DRFSObject cannot be downcasted to DRFolder")
        }
    }
}
impl INSObject for DRFolder {}
impl PNSObject for DRFolder {}
impl IDRFolder for DRFolder {}
pub trait IDRFolder: Sized + std::ops::Deref {
    unsafe fn initWithPath_(&self, path: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPath : path)
    }
    unsafe fn folderWithPath_(path: NSString) -> DRFolder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRFolder").unwrap(), folderWithPath : path)
    }
}
impl DRFolder_VirtualFolders for DRFolder {}
pub trait DRFolder_VirtualFolders: Sized + std::ops::Deref {
    unsafe fn initWithName_(&self, name: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name)
    }
    unsafe fn makeVirtual(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, makeVirtual)
    }
    unsafe fn addChild_(&self, child: DRFSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addChild : child)
    }
    unsafe fn removeChild_(&self, child: DRFSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeChild : child)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn children(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, children)
    }
    unsafe fn virtualFolderWithName_(name: NSString) -> DRFolder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRFolder").unwrap(), virtualFolderWithName : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRMSFFormatter(pub id);
impl std::ops::Deref for DRMSFFormatter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRMSFFormatter {}
impl DRMSFFormatter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRMSFFormatter").unwrap(), alloc) })
    }
}
impl INSFormatter for DRMSFFormatter {}
impl PNSCopying for DRMSFFormatter {}
impl PNSCoding for DRMSFFormatter {}
impl std::convert::TryFrom<NSFormatter> for DRMSFFormatter {
    type Error = &'static str;
    fn try_from(parent: NSFormatter) -> Result<DRMSFFormatter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRMSFFormatter").unwrap()) };
        if is_kind_of {
            Ok(DRMSFFormatter(parent.0))
        } else {
            Err("This NSFormatter cannot be downcasted to DRMSFFormatter")
        }
    }
}
impl INSObject for DRMSFFormatter {}
impl PNSObject for DRMSFFormatter {}
impl IDRMSFFormatter for DRMSFFormatter {}
pub trait IDRMSFFormatter: Sized + std::ops::Deref {
    unsafe fn initWithFormat_(&self, format: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormat : format)
    }
    unsafe fn format(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn setFormat_(&self, format: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFormat : format)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DRNotificationCenter(pub id);
impl std::ops::Deref for DRNotificationCenter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DRNotificationCenter {}
impl DRNotificationCenter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DRNotificationCenter").unwrap(), alloc) })
    }
}
impl INSObject for DRNotificationCenter {}
impl PNSObject for DRNotificationCenter {}
impl std::convert::TryFrom<NSObject> for DRNotificationCenter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DRNotificationCenter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DRNotificationCenter").unwrap()) };
        if is_kind_of {
            Ok(DRNotificationCenter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DRNotificationCenter")
        }
    }
}
impl IDRNotificationCenter for DRNotificationCenter {}
pub trait IDRNotificationCenter: Sized + std::ops::Deref {
    unsafe fn addObserver_selector_name_object_(
        &self,
        observer: id,
        aSelector: objc2::runtime::Sel,
        notificationName: NSString,
        anObject: id,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObserver : observer, selector : aSelector, name : notificationName, object : anObject)
    }
    unsafe fn removeObserver_name_object_(&self, observer: id, aName: NSString, anObject: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeObserver : observer, name : aName, object : anObject)
    }
    unsafe fn currentRunLoopCenter() -> DRNotificationCenter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRNotificationCenter").unwrap(), currentRunLoopCenter)
    }
}
impl DRTrack_AudioContentCreation for DRTrack {}
pub trait DRTrack_AudioContentCreation: Sized + std::ops::Deref {
    unsafe fn trackForAudioOfLength_producer_(length: DRMSF, producer: id) -> DRTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRTrack").unwrap(), trackForAudioOfLength : length, producer : producer)
    }
    unsafe fn trackForAudioFile_(path: NSString) -> DRTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRTrack").unwrap(), trackForAudioFile : path)
    }
}
impl DRTrack_DataContentCreation for DRTrack {}
pub trait DRTrack_DataContentCreation: Sized + std::ops::Deref {
    unsafe fn trackForRootFolder_(rootFolder: DRFolder) -> DRTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"DRTrack").unwrap(), trackForRootFolder : rootFolder)
    }
}
unsafe extern "C" {
    pub static kDRRefConCFTypeCallbacks: DRRefConCallbacks;
}
unsafe extern "C" {
    pub fn DRSetRefCon(
        ref_: DRTypeRef,
        refCon: *mut ::std::os::raw::c_void,
        callbacks: *const DRRefConCallbacks,
    );
}
unsafe extern "C" {
    pub fn DRGetRefCon(ref_: DRTypeRef) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn DRCopyLocalizedStringForValue(value: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn DRDeviceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DRCopyDeviceArray() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn DRDeviceCopyDeviceForBSDName(name: CFStringRef) -> DRDeviceRef;
}
unsafe extern "C" {
    pub fn DRDeviceCopyDeviceForIORegistryEntryPath(path: CFStringRef) -> DRDeviceRef;
}
unsafe extern "C" {
    pub fn DRDeviceIsValid(device: DRDeviceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn DRDeviceOpenTray(device: DRDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DRDeviceCloseTray(device: DRDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DRDeviceEjectMedia(device: DRDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DRDeviceAcquireMediaReservation(device: DRDeviceRef);
}
unsafe extern "C" {
    pub fn DRDeviceReleaseMediaReservation(device: DRDeviceRef);
}
unsafe extern "C" {
    pub fn DRDeviceAcquireExclusiveAccess(device: DRDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DRDeviceReleaseExclusiveAccess(device: DRDeviceRef);
}
unsafe extern "C" {
    pub fn DRDeviceCopyInfo(device: DRDeviceRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DRDeviceCopyStatus(device: DRDeviceRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub static kDRDeviceAppearedNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceDisappearedNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceStatusChangedNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceSupportLevelKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceIORegistryEntryPathKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceVendorNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceProductNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceFirmwareRevisionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectLocationKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceWriteCapabilitiesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceLoadingMechanismCanEjectKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceLoadingMechanismCanInjectKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceLoadingMechanismCanOpenKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceWriteBufferSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceSupportLevelNone: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceSupportLevelUnsupported: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceSupportLevelVendorSupported: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceSupportLevelAppleSupported: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceSupportLevelAppleShipping: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectATAPI: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectFibreChannel: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectFireWire: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectUSB: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectSCSI: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectLocationInternal: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectLocationExternal: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDevicePhysicalInterconnectLocationUnknown: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteCDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteCDRKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteCDRWKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDRKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDRDualLayerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDRWKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDRWDualLayerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDRAMKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDPlusRKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDPlusRDoubleLayerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDPlusRWKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDPlusRWDoubleLayerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteBDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteBDRKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteBDREKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteHDDVDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteHDDVDRKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteHDDVDRDualLayerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteHDDVDRAMKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteHDDVDRWKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteHDDVDRWDualLayerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteCDTextKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteIndexPointsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteISRCKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteCDTAOKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteCDSAOKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteCDRawKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanWriteDVDDAOKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanTestWriteCDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanTestWriteDVDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanUnderrunProtectCDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCanUnderrunProtectDVDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceIsBusyKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceIsTrayOpenKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMaximumWriteSpeedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceCurrentWriteSpeedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaStateKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaInfoKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceBurnSpeedsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceTrackRefsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceTrackInfoKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaStateMediaPresent: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaStateInTransition: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaStateNone: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaBSDNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaIsBlankKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaIsAppendableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaIsOverwritableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaIsErasableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaIsReservedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaBlocksOverwritableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaBlocksFreeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaBlocksUsedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaDoubleLayerL0DataZoneBlocksKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTrackCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaSessionCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaClassKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaClassCD: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaClassDVD: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaClassBD: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaClassHDDVD: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaClassUnknown: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeCDROM: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeCDR: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeCDRW: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDROM: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDRAM: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDR: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDRDualLayer: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDRW: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDRWDualLayer: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDPlusR: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDPlusRDoubleLayer: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDPlusRW: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeDVDPlusRWDoubleLayer: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeBDR: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeBDRE: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeBDROM: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeHDDVDROM: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeHDDVDR: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeHDDVDRDualLayer: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeHDDVDRAM: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeHDDVDRW: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeHDDVDRWDualLayer: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceMediaTypeUnknown: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDeviceBurnSpeedCD1x: f32;
}
unsafe extern "C" {
    pub static kDRDeviceBurnSpeedDVD1x: f32;
}
unsafe extern "C" {
    pub static kDRDeviceBurnSpeedBD1x: f32;
}
unsafe extern "C" {
    pub static kDRDeviceBurnSpeedHDDVD1x: f32;
}
unsafe extern "C" {
    pub static kDRDeviceBurnSpeedMax: f32;
}
unsafe extern "C" {
    pub fn DRDeviceKPSForXFactor(deviceOrMediaType: DRTypeRef, xfactor: f32) -> f32;
}
unsafe extern "C" {
    pub fn DRDeviceXFactorForKPS(deviceOrMediaType: DRTypeRef, kps: f32) -> f32;
}
unsafe extern "C" {
    pub fn DRTrackGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DRTrackCreate(properties: CFDictionaryRef, callback: DRTrackCallbackProc) -> DRTrackRef;
}
unsafe extern "C" {
    pub fn DRTrackSetProperties(track: DRTrackRef, properties: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn DRTrackGetProperties(track: DRTrackRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DRTrackSpeedTest(
        track: DRTrackRef,
        howManyMilliseconds: UInt32,
        howManyBytes: UInt32,
    ) -> f32;
}
unsafe extern "C" {
    pub fn DRTrackEstimateLength(track: DRTrackRef) -> UInt64;
}
unsafe extern "C" {
    pub static kDRTrackLengthKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBlockSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBlockTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDataFormKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSessionFormatKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackModeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVerificationTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDVDCopyrightInfoKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDVDTimestampKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBufferZone1DataKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMaxBurnSpeedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRPreGapLengthKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRPreGapIsRequiredKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackISRCKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRIndexPointsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRAudioPreEmphasisKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRAudioFourChannelKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSerialCopyManagementStateKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVerificationTypeNone: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVerificationTypeProduceAgain: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVerificationTypeReceiveData: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVerificationTypeChecksum: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSCMSCopyrightFree: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSCMSCopyrightProtectedOriginal: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSCMSCopyrightProtectedCopy: CFStringRef;
}
unsafe extern "C" {
    pub static kDRNextWritableAddressKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackStartAddressKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRFreeBlocksKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackNumberKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSessionNumberKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackIsEmptyKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackPacketTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackPacketSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackTypeInvisible: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackTypeIncomplete: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackTypeReserved: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackTypeClosed: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackPacketTypeFixed: CFStringRef;
}
unsafe extern "C" {
    pub static kDRTrackPacketTypeVariable: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSubchannelDataFormKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSubchannelDataFormNone: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSubchannelDataFormPack: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSubchannelDataFormRaw: CFStringRef;
}
unsafe extern "C" {
    pub fn DRBurnGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DRBurnCreate(device: DRDeviceRef) -> DRBurnRef;
}
unsafe extern "C" {
    pub fn DRBurnWriteLayout(burn: DRBurnRef, layout: CFTypeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DRBurnAbort(burn: DRBurnRef);
}
unsafe extern "C" {
    pub fn DRBurnCopyStatus(burn: DRBurnRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DRBurnGetDevice(burn: DRBurnRef) -> DRDeviceRef;
}
unsafe extern "C" {
    pub fn DRBurnSetProperties(burn: DRBurnRef, properties: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn DRBurnGetProperties(burn: DRBurnRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub static kDRBurnStatusChangedNotification: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnRequestedSpeedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnAppendableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnOverwriteDiscKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnVerifyDiscKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnCompletionActionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnUnderrunProtectionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnTestingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSynchronousBehaviorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnFailureActionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMediaCatalogNumberKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnDoubleLayerL0DataZoneBlocksKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnStrategyKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnStrategyIsRequiredKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnCompletionActionEject: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnCompletionActionMount: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnFailureActionEject: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnFailureActionNone: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnStrategyCDTAO: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnStrategyCDSAO: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnStrategyDVDDAO: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBurnStrategyBDDAO: CFStringRef;
}
unsafe extern "C" {
    pub fn DRCDTextBlockCreateArrayFromPackList(packs: CFDataRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn DRCDTextBlockGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DRCDTextBlockCreate(
        language: CFStringRef,
        encoding: CFStringEncoding,
    ) -> DRCDTextBlockRef;
}
unsafe extern "C" {
    pub fn DRCDTextBlockGetProperties(block: DRCDTextBlockRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DRCDTextBlockSetProperties(block: DRCDTextBlockRef, properties: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn DRCDTextBlockGetTrackDictionaries(block: DRCDTextBlockRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn DRCDTextBlockSetTrackDictionaries(block: DRCDTextBlockRef, array: CFArrayRef);
}
unsafe extern "C" {
    pub fn DRCDTextBlockGetValue(
        block: DRCDTextBlockRef,
        trackIndex: CFIndex,
        key: CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn DRCDTextBlockSetValue(
        block: DRCDTextBlockRef,
        trackIndex: CFIndex,
        key: CFStringRef,
        value: CFTypeRef,
    );
}
unsafe extern "C" {
    pub fn DRCDTextBlockFlatten(block: DRCDTextBlockRef) -> UInt32;
}
unsafe extern "C" {
    pub static kDRCDTextLanguageKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextCharacterCodeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextCFStringEncodingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextCopyrightAssertedForSpecialMessagesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextCopyrightAssertedForNamesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextCopyrightAssertedForTitlesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextTitleKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextPerformerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextSongwriterKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextComposerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextArrangerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextSpecialMessageKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextDiscIdentKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextGenreKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextGenreCodeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextClosedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextMCNISRCKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextTOCKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextTOC2Key: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCDTextSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub fn DREraseGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DREraseCreate(device: DRDeviceRef) -> DREraseRef;
}
unsafe extern "C" {
    pub fn DREraseStart(erase: DREraseRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DREraseCopyStatus(erase: DREraseRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub static kDREraseStatusChangedNotification: CFStringRef;
}
unsafe extern "C" {
    pub fn DREraseGetDevice(erase: DREraseRef) -> DRDeviceRef;
}
unsafe extern "C" {
    pub fn DREraseSetProperties(erase: DREraseRef, properties: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn DREraseGetProperties(erase: DREraseRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub static kDREraseTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDREraseTypeQuick: CFStringRef;
}
unsafe extern "C" {
    pub static kDREraseTypeComplete: CFStringRef;
}
unsafe extern "C" {
    pub fn DRCopyLocalizedStringForDiscRecordingError(osError: OSStatus) -> CFStringRef;
}
unsafe extern "C" {
    pub fn DRCopyLocalizedStringForSenseCode(senseCode: UInt8) -> CFStringRef;
}
unsafe extern "C" {
    pub fn DRCopyLocalizedStringForAdditionalSense(ASC: UInt8, ASCQ: UInt8) -> CFStringRef;
}
unsafe extern "C" {
    pub static kDRErrorStatusKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRErrorStatusErrorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRErrorStatusErrorStringKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRErrorStatusErrorInfoStringKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRErrorStatusSenseKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRErrorStatusSenseCodeStringKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRErrorStatusAdditionalSenseStringKey: CFStringRef;
}
unsafe extern "C" {
    pub fn DRNotificationCenterGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DRNotificationCenterCreate() -> DRNotificationCenterRef;
}
unsafe extern "C" {
    pub fn DRNotificationCenterCreateRunLoopSource(
        center: DRNotificationCenterRef,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn DRNotificationCenterAddObserver(
        center: DRNotificationCenterRef,
        observer: *const ::std::os::raw::c_void,
        callback: DRNotificationCallback,
        name: CFStringRef,
        object: DRTypeRef,
    );
}
unsafe extern "C" {
    pub fn DRNotificationCenterRemoveObserver(
        center: DRNotificationCenterRef,
        observer: *const ::std::os::raw::c_void,
        name: CFStringRef,
        object: DRTypeRef,
    );
}
unsafe extern "C" {
    pub static kDRStatusStateKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusPercentCompleteKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusCurrentSpeedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusCurrentSessionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusCurrentTrackKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusTotalSessionsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusTotalTracksKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusEraseTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateNone: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStatePreparing: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateVerifying: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateDone: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateFailed: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateSessionOpen: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateTrackOpen: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateTrackWrite: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateTrackClose: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateSessionClose: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateFinishing: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusStateErasing: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusProgressInfoKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusProgressCurrentKPS: CFStringRef;
}
unsafe extern "C" {
    pub static kDRStatusProgressCurrentXFactor: CFStringRef;
}
unsafe extern "C" {
    pub fn DRFSObjectIsVirtual(object: DRFSObjectRef) -> Boolean;
}
unsafe extern "C" {
    pub fn DRFSObjectGetRealFSRef(object: DRFSObjectRef, fsRef: *mut FSRef);
}
unsafe extern "C" {
    pub fn DRFSObjectCopyRealURL(object: DRFSObjectRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn DRFSObjectGetParent(object: DRFSObjectRef) -> DRFolderRef;
}
unsafe extern "C" {
    pub fn DRFSObjectCopyBaseName(object: DRFSObjectRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn DRFSObjectCopySpecificName(object: DRFSObjectRef, fsKey: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn DRFSObjectCopySpecificNames(object: DRFSObjectRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DRFSObjectCopyMangledName(object: DRFSObjectRef, fsKey: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn DRFSObjectCopyMangledNames(object: DRFSObjectRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DRFSObjectCopyFilesystemProperty(
        object: DRFSObjectRef,
        fsKey: CFStringRef,
        propertyKey: CFStringRef,
        coalesce: Boolean,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn DRFSObjectCopyFilesystemProperties(
        object: DRFSObjectRef,
        fsKey: CFStringRef,
        coalesce: Boolean,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DRFSObjectGetFilesystemMask(
        object: DRFSObjectRef,
        explicitMask: *mut DRFilesystemMask,
        effectiveMask: *mut DRFilesystemMask,
    ) -> DRFilesystemMask;
}
unsafe extern "C" {
    pub fn DRFSObjectSetBaseName(object: DRFSObjectRef, baseName: CFStringRef);
}
unsafe extern "C" {
    pub fn DRFSObjectSetSpecificName(
        object: DRFSObjectRef,
        fsKey: CFStringRef,
        specificName: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn DRFSObjectSetSpecificNames(object: DRFSObjectRef, specificNames: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn DRFSObjectSetFilesystemProperty(
        object: DRFSObjectRef,
        fsKey: CFStringRef,
        propertyKey: CFStringRef,
        value: CFTypeRef,
    );
}
unsafe extern "C" {
    pub fn DRFSObjectSetFilesystemProperties(
        object: DRFSObjectRef,
        fsKey: CFStringRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn DRFSObjectSetFilesystemMask(object: DRFSObjectRef, newMask: DRFilesystemMask);
}
unsafe extern "C" {
    pub fn DRFolderGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DRFolderCreateReal(fsRef: *const FSRef) -> DRFolderRef;
}
unsafe extern "C" {
    pub fn DRFolderCreateRealWithURL(urlRef: CFURLRef) -> DRFolderRef;
}
unsafe extern "C" {
    pub fn DRFolderCreateVirtual(baseName: CFStringRef) -> DRFolderRef;
}
unsafe extern "C" {
    pub fn DRFolderConvertRealToVirtual(realFolder: DRFolderRef);
}
unsafe extern "C" {
    pub fn DRFolderAddChild(parent: DRFolderRef, newChild: DRFSObjectRef);
}
unsafe extern "C" {
    pub fn DRFolderRemoveChild(parent: DRFolderRef, child: DRFSObjectRef);
}
unsafe extern "C" {
    pub fn DRFolderCountChildren(folder: DRFolderRef) -> UInt32;
}
unsafe extern "C" {
    pub fn DRFolderCopyChildren(folder: DRFolderRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn DRFileGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DRFileCreateReal(fsRef: *const FSRef) -> DRFileRef;
}
unsafe extern "C" {
    pub fn DRFileCreateRealWithURL(urlRef: CFURLRef) -> DRFileRef;
}
unsafe extern "C" {
    pub fn DRFileCreateVirtualWithData(
        baseName: CFStringRef,
        fileData: *mut ::std::os::raw::c_void,
        fileDataLength: UInt32,
    ) -> DRFileRef;
}
unsafe extern "C" {
    pub fn DRFileCreateVirtualWithCallback(
        baseName: CFStringRef,
        fileProc: DRFileProc,
        fileProcRefCon: *mut ::std::os::raw::c_void,
    ) -> DRFileRef;
}
unsafe extern "C" {
    pub fn DRFileCreateVirtualLink(
        original: DRFSObjectRef,
        linkType: DRLinkType,
        fsKey: CFStringRef,
    ) -> DRFileRef;
}
unsafe extern "C" {
    pub static kDRISOLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVolumeSet: CFStringRef;
}
unsafe extern "C" {
    pub static kDRPublisher: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDataPreparer: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSystemIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kDRApplicationIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCopyrightFile: CFStringRef;
}
unsafe extern "C" {
    pub static kDRAbstractFile: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBibliographicFile: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBlockSize: CFStringRef;
}
unsafe extern "C" {
    pub static kDRDefaultDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVolumeCreationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVolumeModificationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVolumeCheckedDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVolumeExpirationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRVolumeEffectiveDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRISOMacExtensions: CFStringRef;
}
unsafe extern "C" {
    pub static kDRISORockRidgeExtensions: CFStringRef;
}
unsafe extern "C" {
    pub static kDRSuppressMacSpecificFiles: CFStringRef;
}
unsafe extern "C" {
    pub static kDRAllFilesystems: CFStringRef;
}
unsafe extern "C" {
    pub static kDRISO9660: CFStringRef;
}
unsafe extern "C" {
    pub static kDRISO9660LevelOne: CFStringRef;
}
unsafe extern "C" {
    pub static kDRISO9660LevelTwo: CFStringRef;
}
unsafe extern "C" {
    pub static kDRJoliet: CFStringRef;
}
unsafe extern "C" {
    pub static kDRHFSPlus: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDF: CFStringRef;
}
unsafe extern "C" {
    pub static kDRISO9660VersionNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kDRInvisible: CFStringRef;
}
unsafe extern "C" {
    pub static kDRCreationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRContentModificationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRAttributeModificationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRAccessDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRBackupDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRRecordingDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDREffectiveDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRExpirationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kDRPosixFileMode: CFStringRef;
}
unsafe extern "C" {
    pub static kDRPosixUID: CFStringRef;
}
unsafe extern "C" {
    pub static kDRPosixGID: CFStringRef;
}
unsafe extern "C" {
    pub static kDRHFSPlusTextEncodingHint: CFStringRef;
}
unsafe extern "C" {
    pub static kDRHFSPlusCatalogNodeID: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMacFileType: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMacFileCreator: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMacWindowBounds: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMacIconLocation: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMacScrollPosition: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMacWindowView: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMacFinderFlags: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMacExtendedFinderFlags: CFStringRef;
}
unsafe extern "C" {
    pub static kDRMacFinderHideExtension: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFWriteVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFVersion102: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFVersion150: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFPrimaryVolumeDescriptorNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFVolumeSequenceNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFMaxVolumeSequenceNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFInterchangeLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFMaxInterchangeLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFApplicationIdentifierSuffix: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFVolumeSetIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFVolumeSetTimestamp: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFVolumeSetImplementationUse: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFRealTimeFile: CFStringRef;
}
unsafe extern "C" {
    pub static kDRUDFExtendedFilePermissions: CFStringRef;
}
unsafe extern "C" {
    pub fn DRFilesystemTrackCreate(rootFolder: DRFolderRef) -> DRFilesystemTrackRef;
}
unsafe extern "C" {
    pub fn DRFilesystemTrackEstimateOverhead(
        numBlocks: UInt64,
        blockSize: UInt32,
        fsMask: DRFilesystemMask,
    ) -> UInt64;
}
unsafe extern "C" {
    pub fn DRAudioTrackCreate(audioFile: *const FSRef) -> DRAudioTrackRef;
}
unsafe extern "C" {
    pub fn DRAudioTrackCreateWithURL(audioFileURL: CFURLRef) -> DRAudioTrackRef;
}
unsafe extern "C" {
    pub static DRDeviceBurnSpeedCD1x: f32;
}
unsafe extern "C" {
    pub static DRDeviceBurnSpeedDVD1x: f32;
}
unsafe extern "C" {
    pub static DRDeviceBurnSpeedBD1x: f32;
}
unsafe extern "C" {
    pub static DRDeviceBurnSpeedHDDVD1x: f32;
}
unsafe extern "C" {
    pub static DRDeviceBurnSpeedMax: f32;
}
unsafe extern "C" {
    pub static DRDeviceAppearedNotification: NSString;
}
unsafe extern "C" {
    pub static DRDeviceDisappearedNotification: NSString;
}
unsafe extern "C" {
    pub static DRDeviceStatusChangedNotification: NSString;
}
unsafe extern "C" {
    pub static DRDeviceSupportLevelKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceIORegistryEntryPathKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceWriteCapabilitiesKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceVendorNameKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceProductNameKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceFirmwareRevisionKey: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectKey: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectLocationKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceLoadingMechanismCanEjectKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceLoadingMechanismCanInjectKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceLoadingMechanismCanOpenKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceWriteBufferSizeKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceSupportLevelNone: NSString;
}
unsafe extern "C" {
    pub static DRDeviceSupportLevelUnsupported: NSString;
}
unsafe extern "C" {
    pub static DRDeviceSupportLevelVendorSupported: NSString;
}
unsafe extern "C" {
    pub static DRDeviceSupportLevelAppleSupported: NSString;
}
unsafe extern "C" {
    pub static DRDeviceSupportLevelAppleShipping: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectATAPI: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectFibreChannel: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectFireWire: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectSCSI: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectUSB: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectLocationInternal: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectLocationExternal: NSString;
}
unsafe extern "C" {
    pub static DRDevicePhysicalInterconnectLocationUnknown: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteCDKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteCDRKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteCDRWKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDRKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDRDualLayerKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDRWKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDRWDualLayerKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDRAMKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDPlusRKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDPlusRDoubleLayerKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDPlusRWKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDPlusRWDoubleLayerKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteBDKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteBDRKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteBDREKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteHDDVDKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteHDDVDRKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteHDDVDRDualLayerKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteHDDVDRAMKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteHDDVDRWKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteHDDVDRWDualLayerKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteCDTextKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteIndexPointsKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteISRCKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteCDTAOKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteCDSAOKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteCDRawKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanWriteDVDDAOKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanTestWriteCDKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanTestWriteDVDKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanUnderrunProtectCDKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCanUnderrunProtectDVDKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceIsBusyKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceIsTrayOpenKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMaximumWriteSpeedKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceCurrentWriteSpeedKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaStateKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaInfoKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceBurnSpeedsKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceTrackRefsKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceTrackInfoKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaStateMediaPresent: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaStateInTransition: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaStateNone: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaBSDNameKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaIsBlankKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaIsAppendableKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaIsOverwritableKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaIsErasableKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaIsReservedKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaOverwritableSpaceKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaFreeSpaceKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaUsedSpaceKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaBlocksOverwritableKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaBlocksFreeKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaBlocksUsedKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaDoubleLayerL0DataZoneBlocksKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTrackCountKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaSessionCountKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaClassKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeKey: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaClassCD: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaClassDVD: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaClassBD: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaClassHDDVD: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaClassUnknown: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeCDROM: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeCDR: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeCDRW: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDROM: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDRAM: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDR: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDRDualLayer: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDRW: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDRWDualLayer: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDPlusR: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDPlusRDoubleLayer: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDPlusRW: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeDVDPlusRWDoubleLayer: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeBDR: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeBDRE: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeBDROM: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeHDDVDROM: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeHDDVDR: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeHDDVDRDualLayer: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeHDDVDRAM: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeHDDVDRW: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeHDDVDRWDualLayer: NSString;
}
unsafe extern "C" {
    pub static DRDeviceMediaTypeUnknown: NSString;
}
unsafe extern "C" {
    pub static DRTrackLengthKey: NSString;
}
unsafe extern "C" {
    pub static DRBlockSizeKey: NSString;
}
unsafe extern "C" {
    pub static DRBlockTypeKey: NSString;
}
unsafe extern "C" {
    pub static DRDataFormKey: NSString;
}
unsafe extern "C" {
    pub static DRSessionFormatKey: NSString;
}
unsafe extern "C" {
    pub static DRTrackModeKey: NSString;
}
unsafe extern "C" {
    pub static DRVerificationTypeKey: NSString;
}
unsafe extern "C" {
    pub static DRMaxBurnSpeedKey: NSString;
}
unsafe extern "C" {
    pub static DRPreGapLengthKey: NSString;
}
unsafe extern "C" {
    pub static DRPreGapIsRequiredKey: NSString;
}
unsafe extern "C" {
    pub static DRDVDTimestampKey: NSString;
}
unsafe extern "C" {
    pub static DRDVDCopyrightInfoKey: NSString;
}
unsafe extern "C" {
    pub static DRTrackISRCKey: NSString;
}
unsafe extern "C" {
    pub static DRIndexPointsKey: NSString;
}
unsafe extern "C" {
    pub static DRAudioPreEmphasisKey: NSString;
}
unsafe extern "C" {
    pub static DRAudioFourChannelKey: NSString;
}
unsafe extern "C" {
    pub static DRSerialCopyManagementStateKey: NSString;
}
unsafe extern "C" {
    pub static DRVerificationTypeProduceAgain: NSString;
}
unsafe extern "C" {
    pub static DRVerificationTypeReceiveData: NSString;
}
unsafe extern "C" {
    pub static DRVerificationTypeChecksum: NSString;
}
unsafe extern "C" {
    pub static DRVerificationTypeNone: NSString;
}
unsafe extern "C" {
    pub static DRSCMSCopyrightFree: NSString;
}
unsafe extern "C" {
    pub static DRSCMSCopyrightProtectedOriginal: NSString;
}
unsafe extern "C" {
    pub static DRSCMSCopyrightProtectedCopy: NSString;
}
unsafe extern "C" {
    pub static DRNextWritableAddressKey: NSString;
}
unsafe extern "C" {
    pub static DRTrackStartAddressKey: NSString;
}
unsafe extern "C" {
    pub static DRFreeBlocksKey: NSString;
}
unsafe extern "C" {
    pub static DRTrackNumberKey: NSString;
}
unsafe extern "C" {
    pub static DRSessionNumberKey: NSString;
}
unsafe extern "C" {
    pub static DRTrackTypeKey: NSString;
}
unsafe extern "C" {
    pub static DRTrackIsEmptyKey: NSString;
}
unsafe extern "C" {
    pub static DRTrackPacketTypeKey: NSString;
}
unsafe extern "C" {
    pub static DRTrackPacketSizeKey: NSString;
}
unsafe extern "C" {
    pub static DRTrackTypeInvisible: NSString;
}
unsafe extern "C" {
    pub static DRTrackTypeIncomplete: NSString;
}
unsafe extern "C" {
    pub static DRTrackTypeReserved: NSString;
}
unsafe extern "C" {
    pub static DRTrackTypeClosed: NSString;
}
unsafe extern "C" {
    pub static DRTrackPacketTypeFixed: NSString;
}
unsafe extern "C" {
    pub static DRTrackPacketTypeVariable: NSString;
}
unsafe extern "C" {
    pub static DRISOLevel: NSString;
}
unsafe extern "C" {
    pub static DRVolumeSet: NSString;
}
unsafe extern "C" {
    pub static DRPublisher: NSString;
}
unsafe extern "C" {
    pub static DRDataPreparer: NSString;
}
unsafe extern "C" {
    pub static DRApplicationIdentifier: NSString;
}
unsafe extern "C" {
    pub static DRSystemIdentifier: NSString;
}
unsafe extern "C" {
    pub static DRCopyrightFile: NSString;
}
unsafe extern "C" {
    pub static DRAbstractFile: NSString;
}
unsafe extern "C" {
    pub static DRBibliographicFile: NSString;
}
unsafe extern "C" {
    pub static DRBlockSize: NSString;
}
unsafe extern "C" {
    pub static DRDefaultDate: NSString;
}
unsafe extern "C" {
    pub static DRVolumeCreationDate: NSString;
}
unsafe extern "C" {
    pub static DRVolumeModificationDate: NSString;
}
unsafe extern "C" {
    pub static DRVolumeCheckedDate: NSString;
}
unsafe extern "C" {
    pub static DRVolumeExpirationDate: NSString;
}
unsafe extern "C" {
    pub static DRVolumeEffectiveDate: NSString;
}
unsafe extern "C" {
    pub static DRISOMacExtensions: NSString;
}
unsafe extern "C" {
    pub static DRISORockRidgeExtensions: NSString;
}
unsafe extern "C" {
    pub static DRSuppressMacSpecificFiles: NSString;
}
unsafe extern "C" {
    pub static DRSubchannelDataFormKey: NSString;
}
unsafe extern "C" {
    pub static DRSubchannelDataFormNone: NSString;
}
unsafe extern "C" {
    pub static DRSubchannelDataFormPack: NSString;
}
unsafe extern "C" {
    pub static DRSubchannelDataFormRaw: NSString;
}
unsafe extern "C" {
    pub static DRBurnRequestedSpeedKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnAppendableKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnOverwriteDiscKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnVerifyDiscKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnCompletionActionKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnUnderrunProtectionKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnTestingKey: NSString;
}
unsafe extern "C" {
    pub static DRSynchronousBehaviorKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnFailureActionKey: NSString;
}
unsafe extern "C" {
    pub static DRMediaCatalogNumberKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnDoubleLayerL0DataZoneBlocksKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnStrategyKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnStrategyIsRequiredKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextKey: NSString;
}
unsafe extern "C" {
    pub static DRBurnCompletionActionEject: NSString;
}
unsafe extern "C" {
    pub static DRBurnCompletionActionMount: NSString;
}
unsafe extern "C" {
    pub static DRBurnFailureActionEject: NSString;
}
unsafe extern "C" {
    pub static DRBurnFailureActionNone: NSString;
}
unsafe extern "C" {
    pub static DRBurnStrategyCDTAO: NSString;
}
unsafe extern "C" {
    pub static DRBurnStrategyCDSAO: NSString;
}
unsafe extern "C" {
    pub static DRBurnStrategyDVDDAO: NSString;
}
unsafe extern "C" {
    pub static DRBurnStrategyBDDAO: NSString;
}
unsafe extern "C" {
    pub static DRBurnStatusChangedNotification: NSString;
}
unsafe extern "C" {
    pub static DRCDTextLanguageKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextCharacterCodeKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextNSStringEncodingKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextCopyrightAssertedForSpecialMessagesKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextCopyrightAssertedForNamesKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextCopyrightAssertedForTitlesKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextTitleKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextPerformerKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextSongwriterKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextComposerKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextArrangerKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextSpecialMessageKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextDiscIdentKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextGenreKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextGenreCodeKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextClosedKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextMCNISRCKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextTOCKey: NSString;
}
unsafe extern "C" {
    pub static DRCDTextTOC2Key: NSString;
}
unsafe extern "C" {
    pub static DRCDTextSizeKey: NSString;
}
unsafe extern "C" {
    pub static DREraseTypeKey: NSString;
}
unsafe extern "C" {
    pub static DREraseTypeQuick: NSString;
}
unsafe extern "C" {
    pub static DREraseTypeComplete: NSString;
}
unsafe extern "C" {
    pub static DREraseStatusChangedNotification: NSString;
}
unsafe extern "C" {
    pub static DRAllFilesystems: NSString;
}
unsafe extern "C" {
    pub static DRISO9660: NSString;
}
unsafe extern "C" {
    pub static DRISO9660LevelOne: NSString;
}
unsafe extern "C" {
    pub static DRISO9660LevelTwo: NSString;
}
unsafe extern "C" {
    pub static DRJoliet: NSString;
}
unsafe extern "C" {
    pub static DRHFSPlus: NSString;
}
unsafe extern "C" {
    pub static DRUDF: NSString;
}
unsafe extern "C" {
    pub static DRISO9660VersionNumber: NSString;
}
unsafe extern "C" {
    pub static DRInvisible: NSString;
}
unsafe extern "C" {
    pub static DRCreationDate: NSString;
}
unsafe extern "C" {
    pub static DRContentModificationDate: NSString;
}
unsafe extern "C" {
    pub static DRAttributeModificationDate: NSString;
}
unsafe extern "C" {
    pub static DRAccessDate: NSString;
}
unsafe extern "C" {
    pub static DRBackupDate: NSString;
}
unsafe extern "C" {
    pub static DREffectiveDate: NSString;
}
unsafe extern "C" {
    pub static DRExpirationDate: NSString;
}
unsafe extern "C" {
    pub static DRRecordingDate: NSString;
}
unsafe extern "C" {
    pub static DRPosixFileMode: NSString;
}
unsafe extern "C" {
    pub static DRPosixUID: NSString;
}
unsafe extern "C" {
    pub static DRPosixGID: NSString;
}
unsafe extern "C" {
    pub static DRHFSPlusTextEncodingHint: NSString;
}
unsafe extern "C" {
    pub static DRHFSPlusCatalogNodeID: NSString;
}
unsafe extern "C" {
    pub static DRMacFileType: NSString;
}
unsafe extern "C" {
    pub static DRMacFileCreator: NSString;
}
unsafe extern "C" {
    pub static DRMacWindowBounds: NSString;
}
unsafe extern "C" {
    pub static DRMacIconLocation: NSString;
}
unsafe extern "C" {
    pub static DRMacScrollPosition: NSString;
}
unsafe extern "C" {
    pub static DRMacWindowView: NSString;
}
unsafe extern "C" {
    pub static DRMacFinderFlags: NSString;
}
unsafe extern "C" {
    pub static DRMacExtendedFinderFlags: NSString;
}
unsafe extern "C" {
    pub static DRMacFinderHideExtension: NSString;
}
unsafe extern "C" {
    pub static DRUDFWriteVersion: NSString;
}
unsafe extern "C" {
    pub static DRUDFVersion102: NSString;
}
unsafe extern "C" {
    pub static DRUDFVersion150: NSString;
}
unsafe extern "C" {
    pub static DRUDFPrimaryVolumeDescriptorNumber: NSString;
}
unsafe extern "C" {
    pub static DRUDFVolumeSequenceNumber: NSString;
}
unsafe extern "C" {
    pub static DRUDFMaxVolumeSequenceNumber: NSString;
}
unsafe extern "C" {
    pub static DRUDFInterchangeLevel: NSString;
}
unsafe extern "C" {
    pub static DRUDFMaxInterchangeLevel: NSString;
}
unsafe extern "C" {
    pub static DRUDFApplicationIdentifierSuffix: NSString;
}
unsafe extern "C" {
    pub static DRUDFVolumeSetIdentifier: NSString;
}
unsafe extern "C" {
    pub static DRUDFVolumeSetTimestamp: NSString;
}
unsafe extern "C" {
    pub static DRUDFVolumeSetImplementationUse: NSString;
}
unsafe extern "C" {
    pub static DRUDFRealTimeFile: NSString;
}
unsafe extern "C" {
    pub static DRUDFExtendedFilePermissions: NSString;
}
unsafe extern "C" {
    pub static DRLinkTypeHardLink: NSString;
}
unsafe extern "C" {
    pub static DRLinkTypeSymbolicLink: NSString;
}
unsafe extern "C" {
    pub static DRLinkTypeFinderAlias: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateKey: NSString;
}
unsafe extern "C" {
    pub static DRStatusPercentCompleteKey: NSString;
}
unsafe extern "C" {
    pub static DRStatusCurrentSessionKey: NSString;
}
unsafe extern "C" {
    pub static DRStatusCurrentTrackKey: NSString;
}
unsafe extern "C" {
    pub static DRStatusTotalSessionsKey: NSString;
}
unsafe extern "C" {
    pub static DRStatusTotalTracksKey: NSString;
}
unsafe extern "C" {
    pub static DRStatusCurrentSpeedKey: NSString;
}
unsafe extern "C" {
    pub static DRStatusEraseTypeKey: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateNone: NSString;
}
unsafe extern "C" {
    pub static DRStatusStatePreparing: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateVerifying: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateDone: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateFailed: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateSessionOpen: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateTrackOpen: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateTrackWrite: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateTrackClose: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateSessionClose: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateFinishing: NSString;
}
unsafe extern "C" {
    pub static DRStatusStateErasing: NSString;
}
unsafe extern "C" {
    pub static DRStatusProgressInfoKey: NSString;
}
unsafe extern "C" {
    pub static DRStatusProgressCurrentKPS: NSString;
}
unsafe extern "C" {
    pub static DRStatusProgressCurrentXFactor: NSString;
}
unsafe extern "C" {
    pub static DRErrorStatusKey: NSString;
}
unsafe extern "C" {
    pub static DRErrorStatusErrorKey: NSString;
}
unsafe extern "C" {
    pub static DRErrorStatusErrorStringKey: NSString;
}
unsafe extern "C" {
    pub static DRErrorStatusErrorInfoStringKey: NSString;
}
unsafe extern "C" {
    pub static DRErrorStatusSenseKey: NSString;
}
unsafe extern "C" {
    pub static DRErrorStatusSenseCodeStringKey: NSString;
}
unsafe extern "C" {
    pub static DRErrorStatusAdditionalSenseStringKey: NSString;
}
unsafe extern "C" {
    pub fn DRGetVersion() -> NumVersion;
}

unsafe impl objc2::encode::RefEncode for DRRefConCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRRefConCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DRRefConCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for __DRDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DRDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DRDevice", &[]);
}
unsafe impl objc2::encode::RefEncode for __DRTrack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DRTrack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DRTrack", &[]);
}
unsafe impl objc2::encode::RefEncode for DRTrackProductionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRTrackProductionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DRTrackProductionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for __DRBurn {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DRBurn {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DRBurn", &[]);
}
unsafe impl objc2::encode::RefEncode for __DRCDTextBlock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DRCDTextBlock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DRCDTextBlock", &[]);
}
unsafe impl objc2::encode::RefEncode for __DRErase {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DRErase {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DRErase", &[]);
}
unsafe impl objc2::encode::RefEncode for __DRNotificationCenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DRNotificationCenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DRNotificationCenter", &[]);
}
unsafe impl objc2::encode::RefEncode for __DRFile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DRFile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DRFile", &[]);
}
unsafe impl objc2::encode::RefEncode for __DRFolder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DRFolder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DRFolder", &[]);
}
unsafe impl objc2::encode::RefEncode for DRFileForkSizeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRFileForkSizeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DRFileForkSizeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for DRFileProductionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRFileProductionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("DRFileProductionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for DRMSF {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRMSF {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRTrack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRTrack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRBurn {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRBurn {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRCDTextBlock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRCDTextBlock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRErase {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRErase {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRFSObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRFSObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRFile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRFile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRFolder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRFolder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRMSFFormatter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRMSFFormatter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DRNotificationCenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DRNotificationCenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
