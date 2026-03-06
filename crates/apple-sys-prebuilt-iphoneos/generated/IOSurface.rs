#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use libc::{id_t, mach_port_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type task_id_token_t = mach_port_t;
pub type IOSurfaceRef = *mut __IOSurface;
pub type IOSurfaceID = u32;
pub type IOSurfaceLockOptions = u32;
pub type IOSurfacePurgeabilityState = u32;
pub type IOSurfaceComponentName = i32;
pub type IOSurfaceComponentType = i32;
pub type IOSurfaceComponentRange = i32;
pub type IOSurfaceSubsampling = i32;
pub type IOSurfaceMemoryLedgerTags = ::std::os::raw::c_int;
pub type IOSurfaceMemoryLedgerFlags = u32;
pub type IOSurfacePropertyKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOSurface(pub id);
impl std::ops::Deref for IOSurface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOSurface {}
impl IOSurface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOSurface").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for IOSurface {}
impl INSObject for IOSurface {}
impl PNSObject for IOSurface {}
impl std::convert::TryFrom<NSObject> for IOSurface {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOSurface, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOSurface").unwrap()) };
        if is_kind_of {
            Ok(IOSurface(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOSurface")
        }
    }
}
impl IIOSurface for IOSurface {}
pub trait IIOSurface: Sized + std::ops::Deref {
    unsafe fn initWithProperties_(&self, properties: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProperties : properties)
    }
    unsafe fn lockWithOptions_seed_(
        &self,
        options: IOSurfaceLockOptions,
        seed: *mut u32,
    ) -> kern_return_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockWithOptions : options, seed : seed)
    }
    unsafe fn unlockWithOptions_seed_(
        &self,
        options: IOSurfaceLockOptions,
        seed: *mut u32,
    ) -> kern_return_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unlockWithOptions : options, seed : seed)
    }
    unsafe fn widthOfPlaneAtIndex_(&self, planeIndex: NSUInteger) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, widthOfPlaneAtIndex : planeIndex)
    }
    unsafe fn heightOfPlaneAtIndex_(&self, planeIndex: NSUInteger) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, heightOfPlaneAtIndex : planeIndex)
    }
    unsafe fn bytesPerRowOfPlaneAtIndex_(&self, planeIndex: NSUInteger) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bytesPerRowOfPlaneAtIndex : planeIndex)
    }
    unsafe fn bytesPerElementOfPlaneAtIndex_(&self, planeIndex: NSUInteger) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bytesPerElementOfPlaneAtIndex : planeIndex)
    }
    unsafe fn elementWidthOfPlaneAtIndex_(&self, planeIndex: NSUInteger) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementWidthOfPlaneAtIndex : planeIndex)
    }
    unsafe fn elementHeightOfPlaneAtIndex_(&self, planeIndex: NSUInteger) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementHeightOfPlaneAtIndex : planeIndex)
    }
    unsafe fn baseAddressOfPlaneAtIndex_(
        &self,
        planeIndex: NSUInteger,
    ) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, baseAddressOfPlaneAtIndex : planeIndex)
    }
    unsafe fn setAttachment_forKey_(&self, anObject: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachment : anObject, forKey : key)
    }
    unsafe fn attachmentForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attachmentForKey : key)
    }
    unsafe fn removeAttachmentForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAttachmentForKey : key)
    }
    unsafe fn setAllAttachments_(&self, dict: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllAttachments : dict)
    }
    unsafe fn allAttachments(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allAttachments)
    }
    unsafe fn removeAllAttachments(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllAttachments)
    }
    unsafe fn incrementUseCount(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, incrementUseCount)
    }
    unsafe fn decrementUseCount(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decrementUseCount)
    }
    unsafe fn setPurgeable_oldState_(
        &self,
        newState: IOSurfacePurgeabilityState,
        oldState: *mut IOSurfacePurgeabilityState,
    ) -> kern_return_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPurgeable : newState, oldState : oldState)
    }
    unsafe fn allocationSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocationSize)
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
    unsafe fn baseAddress(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseAddress)
    }
    unsafe fn pixelFormat(&self) -> OSType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn bytesPerRow(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesPerRow)
    }
    unsafe fn bytesPerElement(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesPerElement)
    }
    unsafe fn elementWidth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementWidth)
    }
    unsafe fn elementHeight(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementHeight)
    }
    unsafe fn surfaceID(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, surfaceID)
    }
    unsafe fn seed(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seed)
    }
    unsafe fn planeCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, planeCount)
    }
    unsafe fn isInUse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInUse)
    }
    unsafe fn localUseCount(&self) -> i32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localUseCount)
    }
    unsafe fn allowsPixelSizeCasting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsPixelSizeCasting)
    }
}
unsafe extern "C" {
    pub static kIOSurfaceAllocSize: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceBytesPerRow: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceBytesPerElement: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceElementWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceElementHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneBytesPerRow: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneSize: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneBase: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneBitsPerElement: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneBytesPerElement: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneElementWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneElementHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceCacheMode: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceIsGlobal: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePixelFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePixelSizeCastingAllowed: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneComponentBitDepths: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneComponentBitOffsets: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceName: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneComponentNames: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneComponentTypes: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfacePlaneComponentRanges: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceSubsampling: CFStringRef;
}
unsafe extern "C" {
    pub fn IOSurfaceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOSurfaceCreate(properties: CFDictionaryRef) -> IOSurfaceRef;
}
unsafe extern "C" {
    pub fn IOSurfaceLookup(csid: IOSurfaceID) -> IOSurfaceRef;
}
unsafe extern "C" {
    pub fn IOSurfaceGetID(buffer: IOSurfaceRef) -> IOSurfaceID;
}
unsafe extern "C" {
    pub fn IOSurfaceLock(
        buffer: IOSurfaceRef,
        options: IOSurfaceLockOptions,
        seed: *mut u32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOSurfaceUnlock(
        buffer: IOSurfaceRef,
        options: IOSurfaceLockOptions,
        seed: *mut u32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOSurfaceGetAllocSize(buffer: IOSurfaceRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetWidth(buffer: IOSurfaceRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetHeight(buffer: IOSurfaceRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetBytesPerElement(buffer: IOSurfaceRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetBytesPerRow(buffer: IOSurfaceRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetBaseAddress(buffer: IOSurfaceRef) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn IOSurfaceGetElementWidth(buffer: IOSurfaceRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetElementHeight(buffer: IOSurfaceRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetPixelFormat(buffer: IOSurfaceRef) -> OSType;
}
unsafe extern "C" {
    pub fn IOSurfaceGetSeed(buffer: IOSurfaceRef) -> u32;
}
unsafe extern "C" {
    pub fn IOSurfaceGetPlaneCount(buffer: IOSurfaceRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetWidthOfPlane(buffer: IOSurfaceRef, planeIndex: usize) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetHeightOfPlane(buffer: IOSurfaceRef, planeIndex: usize) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetBytesPerElementOfPlane(buffer: IOSurfaceRef, planeIndex: usize) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetBytesPerRowOfPlane(buffer: IOSurfaceRef, planeIndex: usize) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetBaseAddressOfPlane(
        buffer: IOSurfaceRef,
        planeIndex: usize,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn IOSurfaceGetElementWidthOfPlane(buffer: IOSurfaceRef, planeIndex: usize) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetElementHeightOfPlane(buffer: IOSurfaceRef, planeIndex: usize) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetNumberOfComponentsOfPlane(buffer: IOSurfaceRef, planeIndex: usize) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetNameOfComponentOfPlane(
        buffer: IOSurfaceRef,
        planeIndex: usize,
        componentIndex: usize,
    ) -> IOSurfaceComponentName;
}
unsafe extern "C" {
    pub fn IOSurfaceGetTypeOfComponentOfPlane(
        buffer: IOSurfaceRef,
        planeIndex: usize,
        componentIndex: usize,
    ) -> IOSurfaceComponentType;
}
unsafe extern "C" {
    pub fn IOSurfaceGetRangeOfComponentOfPlane(
        buffer: IOSurfaceRef,
        planeIndex: usize,
        componentIndex: usize,
    ) -> IOSurfaceComponentRange;
}
unsafe extern "C" {
    pub fn IOSurfaceGetBitDepthOfComponentOfPlane(
        buffer: IOSurfaceRef,
        planeIndex: usize,
        componentIndex: usize,
    ) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetBitOffsetOfComponentOfPlane(
        buffer: IOSurfaceRef,
        planeIndex: usize,
        componentIndex: usize,
    ) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetSubsampling(buffer: IOSurfaceRef) -> IOSurfaceSubsampling;
}
unsafe extern "C" {
    pub static kIOSurfaceColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceICCProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kIOSurfaceContentHeadroom: CFStringRef;
}
unsafe extern "C" {
    pub fn IOSurfaceSetValue(buffer: IOSurfaceRef, key: CFStringRef, value: CFTypeRef);
}
unsafe extern "C" {
    pub fn IOSurfaceCopyValue(buffer: IOSurfaceRef, key: CFStringRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOSurfaceRemoveValue(buffer: IOSurfaceRef, key: CFStringRef);
}
unsafe extern "C" {
    pub fn IOSurfaceSetValues(buffer: IOSurfaceRef, keysAndValues: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn IOSurfaceCopyAllValues(buffer: IOSurfaceRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn IOSurfaceRemoveAllValues(buffer: IOSurfaceRef);
}
unsafe extern "C" {
    pub fn IOSurfaceCreateMachPort(buffer: IOSurfaceRef) -> mach_port_t;
}
unsafe extern "C" {
    pub fn IOSurfaceLookupFromMachPort(port: mach_port_t) -> IOSurfaceRef;
}
unsafe extern "C" {
    pub fn IOSurfaceGetPropertyMaximum(property: CFStringRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceGetPropertyAlignment(property: CFStringRef) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceAlignProperty(property: CFStringRef, value: usize) -> usize;
}
unsafe extern "C" {
    pub fn IOSurfaceIncrementUseCount(buffer: IOSurfaceRef);
}
unsafe extern "C" {
    pub fn IOSurfaceDecrementUseCount(buffer: IOSurfaceRef);
}
unsafe extern "C" {
    pub fn IOSurfaceGetUseCount(buffer: IOSurfaceRef) -> i32;
}
unsafe extern "C" {
    pub fn IOSurfaceIsInUse(buffer: IOSurfaceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOSurfaceAllowsPixelSizeCasting(buffer: IOSurfaceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOSurfaceSetPurgeable(
        buffer: IOSurfaceRef,
        newState: u32,
        oldState: *mut u32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOSurfaceSetOwnershipIdentity(
        buffer: IOSurfaceRef,
        task_id_token: task_id_token_t,
        newLedgerTag: ::std::os::raw::c_int,
        newLedgerOptions: u32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOSurfaceCreateXPCObject(aSurface: IOSurfaceRef) -> xpc_object_t;
}
unsafe extern "C" {
    pub fn IOSurfaceLookupFromXPCObject(xobj: NSObject) -> IOSurfaceRef;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyAllocSize: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyWidth: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyHeight: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyBytesPerRow: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyBytesPerElement: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyElementWidth: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyElementHeight: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyOffset: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneInfo: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneWidth: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneHeight: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneBytesPerRow: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneOffset: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneSize: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneBase: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneBytesPerElement: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneElementWidth: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPlaneElementHeight: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyCacheMode: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPixelFormat: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyPixelSizeCastingAllowed: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyKeyName: IOSurfacePropertyKey;
}
unsafe extern "C" {
    pub static mut IOSurfacePropertyAllocSizeKey: IOSurfacePropertyKey;
}

unsafe impl objc2::encode::RefEncode for IOSurface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOSurface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
