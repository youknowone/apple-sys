#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Metal::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PGPhysicalMemoryRange_s {
    pub physicalAddress: u64,
    pub physicalLength: u64,
}
pub type PGPhysicalMemoryRange_t = PGPhysicalMemoryRange_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PGTask_s {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PGTraceRange_s {
    _unused: [u8; 0],
}
pub type PGCreateTask = *mut ::std::os::raw::c_void;
pub type PGDestroyTask = *mut ::std::os::raw::c_void;
pub type PGMapMemory = *mut ::std::os::raw::c_void;
pub type PGUnmapMemory = *mut ::std::os::raw::c_void;
pub type PGReadMemory = *mut ::std::os::raw::c_void;
pub type PGRaiseInterrupt = *mut ::std::os::raw::c_void;
pub type PGAddTraceRange = *mut ::std::os::raw::c_void;
pub type PGRemoveTraceRange = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PGDeviceDescriptor(pub id);
impl std::ops::Deref for PGDeviceDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PGDeviceDescriptor {}
impl PGDeviceDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PGDeviceDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for PGDeviceDescriptor {}
impl PNSObject for PGDeviceDescriptor {}
impl std::convert::TryFrom<NSObject> for PGDeviceDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PGDeviceDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PGDeviceDescriptor").unwrap()) };
        if is_kind_of {
            Ok(PGDeviceDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PGDeviceDescriptor")
        }
    }
}
impl IPGDeviceDescriptor for PGDeviceDescriptor {}
pub trait IPGDeviceDescriptor: Sized + std::ops::Deref {
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn setDevice_(&self, device: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDevice : device)
    }
    unsafe fn mmioLength(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mmioLength)
    }
    unsafe fn setMmioLength_(&self, mmioLength: usize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMmioLength : mmioLength)
    }
    unsafe fn createTask(&self) -> PGCreateTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createTask)
    }
    unsafe fn setCreateTask_(&self, createTask: PGCreateTask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCreateTask : createTask)
    }
    unsafe fn destroyTask(&self) -> PGDestroyTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destroyTask)
    }
    unsafe fn setDestroyTask_(&self, destroyTask: PGDestroyTask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestroyTask : destroyTask)
    }
    unsafe fn mapMemory(&self) -> PGMapMemory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapMemory)
    }
    unsafe fn setMapMemory_(&self, mapMemory: PGMapMemory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapMemory : mapMemory)
    }
    unsafe fn unmapMemory(&self) -> PGUnmapMemory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unmapMemory)
    }
    unsafe fn setUnmapMemory_(&self, unmapMemory: PGUnmapMemory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnmapMemory : unmapMemory)
    }
    unsafe fn readMemory(&self) -> PGReadMemory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readMemory)
    }
    unsafe fn setReadMemory_(&self, readMemory: PGReadMemory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadMemory : readMemory)
    }
    unsafe fn raiseInterrupt(&self) -> PGRaiseInterrupt
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, raiseInterrupt)
    }
    unsafe fn setRaiseInterrupt_(&self, raiseInterrupt: PGRaiseInterrupt)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRaiseInterrupt : raiseInterrupt)
    }
    unsafe fn addTraceRange(&self) -> PGAddTraceRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addTraceRange)
    }
    unsafe fn setAddTraceRange_(&self, addTraceRange: PGAddTraceRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddTraceRange : addTraceRange)
    }
    unsafe fn removeTraceRange(&self) -> PGRemoveTraceRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeTraceRange)
    }
    unsafe fn setRemoveTraceRange_(&self, removeTraceRange: PGRemoveTraceRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRemoveTraceRange : removeTraceRange)
    }
    unsafe fn displayPortCount(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayPortCount)
    }
    unsafe fn setDisplayPortCount_(&self, displayPortCount: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayPortCount : displayPortCount)
    }
}
pub trait PPGDevice: Sized + std::ops::Deref {
    unsafe fn mmioReadAtOffset_(&self, offset: usize) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mmioReadAtOffset : offset)
    }
    unsafe fn mmioWriteAtOffset_value_(&self, offset: usize, value: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mmioWriteAtOffset : offset, value : value)
    }
    unsafe fn newDisplayWithDescriptor_port_serialNum_(
        &self,
        descriptor: PGDisplayDescriptor,
        port: NSUInteger,
        serialNum: u32,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newDisplayWithDescriptor : descriptor, port : port, serialNum : serialNum)
    }
    unsafe fn willSuspend(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willSuspend)
    }
    unsafe fn finishSuspend(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finishSuspend)
    }
    unsafe fn willResumeWithSuspendState_error_(
        &self,
        suspendState: NSData,
        error: *mut NSError,
    ) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willResumeWithSuspendState : suspendState, error : error)
    }
    unsafe fn didResume(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didResume)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn unpause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unpause)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
}
pub type PGResumeErrorCode = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PGDisplayCoord_t {
    pub x: u16,
    pub y: u16,
}
pub type PGDisplayModeChangeHandler = *mut ::std::os::raw::c_void;
pub type PGDisplayNewFrameEventHandler = *mut ::std::os::raw::c_void;
pub type PGDisplayCursorGlyphHandler = *mut ::std::os::raw::c_void;
pub type PGDisplayCursorShowHandler = *mut ::std::os::raw::c_void;
pub type PGDisplayCursorMoveHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PGDisplayDescriptor(pub id);
impl std::ops::Deref for PGDisplayDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PGDisplayDescriptor {}
impl PGDisplayDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PGDisplayDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for PGDisplayDescriptor {}
impl PNSObject for PGDisplayDescriptor {}
impl std::convert::TryFrom<NSObject> for PGDisplayDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PGDisplayDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PGDisplayDescriptor").unwrap()) };
        if is_kind_of {
            Ok(PGDisplayDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PGDisplayDescriptor")
        }
    }
}
impl IPGDisplayDescriptor for PGDisplayDescriptor {}
pub trait IPGDisplayDescriptor: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn sizeInMillimeters(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeInMillimeters)
    }
    unsafe fn setSizeInMillimeters_(&self, sizeInMillimeters: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSizeInMillimeters : sizeInMillimeters)
    }
    unsafe fn queue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queue)
    }
    unsafe fn setQueue_(&self, queue: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueue : queue)
    }
    unsafe fn modeChangeHandler(&self) -> PGDisplayModeChangeHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modeChangeHandler)
    }
    unsafe fn setModeChangeHandler_(&self, modeChangeHandler: PGDisplayModeChangeHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModeChangeHandler : modeChangeHandler)
    }
    unsafe fn newFrameEventHandler(&self) -> PGDisplayNewFrameEventHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newFrameEventHandler)
    }
    unsafe fn setNewFrameEventHandler_(&self, newFrameEventHandler: PGDisplayNewFrameEventHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNewFrameEventHandler : newFrameEventHandler)
    }
    unsafe fn cursorGlyphHandler(&self) -> PGDisplayCursorGlyphHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursorGlyphHandler)
    }
    unsafe fn setCursorGlyphHandler_(&self, cursorGlyphHandler: PGDisplayCursorGlyphHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCursorGlyphHandler : cursorGlyphHandler)
    }
    unsafe fn cursorShowHandler(&self) -> PGDisplayCursorShowHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursorShowHandler)
    }
    unsafe fn setCursorShowHandler_(&self, cursorShowHandler: PGDisplayCursorShowHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCursorShowHandler : cursorShowHandler)
    }
    unsafe fn cursorMoveHandler(&self) -> PGDisplayCursorMoveHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursorMoveHandler)
    }
    unsafe fn setCursorMoveHandler_(&self, cursorMoveHandler: PGDisplayCursorMoveHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCursorMoveHandler : cursorMoveHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PGDisplayMode(pub id);
impl std::ops::Deref for PGDisplayMode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PGDisplayMode {}
impl PGDisplayMode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PGDisplayMode").unwrap(), alloc) })
    }
}
impl INSObject for PGDisplayMode {}
impl PNSObject for PGDisplayMode {}
impl std::convert::TryFrom<NSObject> for PGDisplayMode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PGDisplayMode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PGDisplayMode").unwrap()) };
        if is_kind_of {
            Ok(PGDisplayMode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PGDisplayMode")
        }
    }
}
impl IPGDisplayMode for PGDisplayMode {}
pub trait IPGDisplayMode: Sized + std::ops::Deref {
    unsafe fn initWithSizeInPixels_refreshRateInHz_(
        &self,
        sizeInPixels: PGDisplayCoord_t,
        refreshRateInHz: f64,
    ) -> PGDisplayMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSizeInPixels : sizeInPixels, refreshRateInHz : refreshRateInHz)
    }
    unsafe fn sizeInPixels(&self) -> PGDisplayCoord_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeInPixels)
    }
    unsafe fn refreshRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refreshRate)
    }
}
pub trait PPGDisplay: Sized + std::ops::Deref {
    unsafe fn encodeCurrentFrameToCommandBuffer_texture_region_(
        &self,
        commandBuffer: *mut u64,
        texture: *mut u64,
        region: MTLRegion,
    ) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeCurrentFrameToCommandBuffer : commandBuffer, texture : texture, region : region)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn sizeInMillimeters(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeInMillimeters)
    }
    unsafe fn queue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queue)
    }
    unsafe fn modeChangeHandler(&self) -> PGDisplayModeChangeHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modeChangeHandler)
    }
    unsafe fn newFrameEventHandler(&self) -> PGDisplayNewFrameEventHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newFrameEventHandler)
    }
    unsafe fn cursorGlyphHandler(&self) -> PGDisplayCursorGlyphHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursorGlyphHandler)
    }
    unsafe fn cursorShowHandler(&self) -> PGDisplayCursorShowHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursorShowHandler)
    }
    unsafe fn cursorMoveHandler(&self) -> PGDisplayCursorMoveHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursorMoveHandler)
    }
    unsafe fn cursorPosition(&self) -> PGDisplayCoord_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cursorPosition)
    }
    unsafe fn serialNum(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serialNum)
    }
    unsafe fn port(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, port)
    }
    unsafe fn minimumTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumTextureUsage)
    }
    unsafe fn guestPresentCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, guestPresentCount)
    }
    unsafe fn hostPresentCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostPresentCount)
    }
    unsafe fn modeList(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modeList)
    }
    unsafe fn setModeList_(&self, modeList: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModeList : modeList)
    }
}
unsafe extern "C" {
    pub static mut ParavirtualizedGraphicsVersionNumber: f64;
}
unsafe extern "C" {
    pub static ParavirtualizedGraphicsVersionString: [::std::os::raw::c_uchar; 0usize];
}
unsafe extern "C" {
    pub fn PGNewDeviceWithDescriptor(descriptor: PGDeviceDescriptor) -> *mut u64;
}
unsafe extern "C" {
    pub fn PGCreateDeviceWithDescriptor(descriptor: PGDeviceDescriptor) -> *mut u64;
}
unsafe extern "C" {
    pub fn PGMaxDisplayPortCount() -> u32;
}
unsafe extern "C" {
    pub static mut PGResumeErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub fn PGCopyOptionROMURL() -> NSURL;
}

unsafe impl objc2::encode::RefEncode for PGPhysicalMemoryRange_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PGPhysicalMemoryRange_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PGPhysicalMemoryRange_s", &[]);
}
unsafe impl objc2::encode::RefEncode for PGTask_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PGTask_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PGTask_s", &[]);
}
unsafe impl objc2::encode::RefEncode for PGTraceRange_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PGTraceRange_s {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PGTraceRange_s", &[]);
}
unsafe impl objc2::encode::RefEncode for PGDeviceDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PGDeviceDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PGDisplayCoord_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PGDisplayCoord_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("PGDisplayCoord_t", &[]);
}
unsafe impl objc2::encode::RefEncode for PGDisplayDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PGDisplayDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PGDisplayMode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PGDisplayMode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
