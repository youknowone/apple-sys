#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOSurface::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use libc::id_t;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLOrigin {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub z: NSUInteger,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLSize {
    pub width: NSUInteger,
    pub height: NSUInteger,
    pub depth: NSUInteger,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRegion {
    pub origin: MTLOrigin,
    pub size: MTLSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLSamplePosition {
    pub x: f32,
    pub y: f32,
}
pub type MTLCoordinate2D = MTLSamplePosition;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLResourceID {
    pub _impl: u64,
}
pub type MTLResourceUsage = NSUInteger;
pub type MTLBarrierScope = NSUInteger;
pub type MTLStages = NSUInteger;
pub trait PMTLCommandEncoder: Sized + std::ops::Deref {
    unsafe fn endEncoding(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endEncoding)
    }
    unsafe fn barrierAfterQueueStages_beforeStages_(
        &self,
        afterQueueStages: MTLStages,
        beforeStages: MTLStages,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, barrierAfterQueueStages : afterQueueStages, beforeStages : beforeStages)
    }
    unsafe fn insertDebugSignpost_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertDebugSignpost : string)
    }
    unsafe fn pushDebugGroup_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushDebugGroup : string)
    }
    unsafe fn popDebugGroup(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, popDebugGroup)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub trait PMTLAllocation: Sized + std::ops::Deref {
    unsafe fn allocatedSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocatedSize)
    }
}
pub type MTLPurgeableState = NSUInteger;
pub type MTLCPUCacheMode = NSUInteger;
pub type MTLStorageMode = NSUInteger;
pub type MTLHazardTrackingMode = NSUInteger;
pub type MTLResourceOptions = NSUInteger;
pub type MTLSparsePageSize = NSInteger;
pub type MTLBufferSparseTier = NSInteger;
pub type MTLTextureSparseTier = NSInteger;
pub trait PMTLResource: Sized + std::ops::Deref {
    unsafe fn setPurgeableState_(&self, state: MTLPurgeableState) -> MTLPurgeableState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPurgeableState : state)
    }
    unsafe fn makeAliasable(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, makeAliasable)
    }
    unsafe fn isAliasable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAliasable)
    }
    unsafe fn setOwnerWithIdentity_(&self, task_id_token: task_id_token_t) -> kern_return_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOwnerWithIdentity : task_id_token)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn cpuCacheMode(&self) -> MTLCPUCacheMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cpuCacheMode)
    }
    unsafe fn storageMode(&self) -> MTLStorageMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storageMode)
    }
    unsafe fn hazardTrackingMode(&self) -> MTLHazardTrackingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hazardTrackingMode)
    }
    unsafe fn resourceOptions(&self) -> MTLResourceOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceOptions)
    }
    unsafe fn heap(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heap)
    }
    unsafe fn heapOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heapOffset)
    }
    unsafe fn allocatedSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocatedSize)
    }
}
pub type MTLPixelFormat = NSUInteger;
pub type MTLDataType = NSUInteger;
pub type MTLTensorDataType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTensorExtents(pub id);
impl std::ops::Deref for MTLTensorExtents {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLTensorExtents {}
impl MTLTensorExtents {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTensorExtents").unwrap(), alloc) })
    }
}
impl INSObject for MTLTensorExtents {}
impl PNSObject for MTLTensorExtents {}
impl std::convert::TryFrom<NSObject> for MTLTensorExtents {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLTensorExtents, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLTensorExtents").unwrap()) };
        if is_kind_of {
            Ok(MTLTensorExtents(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLTensorExtents")
        }
    }
}
impl IMTLTensorExtents for MTLTensorExtents {}
pub trait IMTLTensorExtents: Sized + std::ops::Deref {
    unsafe fn initWithRank_values_(
        &self,
        rank: NSUInteger,
        values: *const NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRank : rank, values : values)
    }
    unsafe fn extentAtDimensionIndex_(&self, dimensionIndex: NSUInteger) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extentAtDimensionIndex : dimensionIndex)
    }
    unsafe fn rank(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rank)
    }
}
pub type MTLTensorError = NSInteger;
pub type MTLTensorUsage = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTensorDescriptor(pub id);
impl std::ops::Deref for MTLTensorDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLTensorDescriptor {}
impl MTLTensorDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTensorDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLTensorDescriptor {}
impl INSObject for MTLTensorDescriptor {}
impl PNSObject for MTLTensorDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLTensorDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLTensorDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLTensorDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLTensorDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLTensorDescriptor")
        }
    }
}
impl IMTLTensorDescriptor for MTLTensorDescriptor {}
pub trait IMTLTensorDescriptor: Sized + std::ops::Deref {
    unsafe fn dimensions(&self) -> MTLTensorExtents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensions)
    }
    unsafe fn setDimensions_(&self, dimensions: MTLTensorExtents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDimensions : dimensions)
    }
    unsafe fn strides(&self) -> MTLTensorExtents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strides)
    }
    unsafe fn setStrides_(&self, strides: MTLTensorExtents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrides : strides)
    }
    unsafe fn dataType(&self) -> MTLTensorDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn setDataType_(&self, dataType: MTLTensorDataType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataType : dataType)
    }
    unsafe fn usage(&self) -> MTLTensorUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usage)
    }
    unsafe fn setUsage_(&self, usage: MTLTensorUsage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsage : usage)
    }
    unsafe fn resourceOptions(&self) -> MTLResourceOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceOptions)
    }
    unsafe fn setResourceOptions_(&self, resourceOptions: MTLResourceOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResourceOptions : resourceOptions)
    }
    unsafe fn cpuCacheMode(&self) -> MTLCPUCacheMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cpuCacheMode)
    }
    unsafe fn setCpuCacheMode_(&self, cpuCacheMode: MTLCPUCacheMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCpuCacheMode : cpuCacheMode)
    }
    unsafe fn storageMode(&self) -> MTLStorageMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storageMode)
    }
    unsafe fn setStorageMode_(&self, storageMode: MTLStorageMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStorageMode : storageMode)
    }
    unsafe fn hazardTrackingMode(&self) -> MTLHazardTrackingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hazardTrackingMode)
    }
    unsafe fn setHazardTrackingMode_(&self, hazardTrackingMode: MTLHazardTrackingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHazardTrackingMode : hazardTrackingMode)
    }
}
pub trait PMTLTensor: Sized + std::ops::Deref {
    unsafe fn replaceSliceOrigin_sliceDimensions_withBytes_strides_(
        &self,
        sliceOrigin: MTLTensorExtents,
        sliceDimensions: MTLTensorExtents,
        bytes: *const ::std::os::raw::c_void,
        strides: MTLTensorExtents,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceSliceOrigin : sliceOrigin, sliceDimensions : sliceDimensions, withBytes : bytes, strides : strides)
    }
    unsafe fn getBytes_strides_fromSliceOrigin_sliceDimensions_(
        &self,
        bytes: *mut ::std::os::raw::c_void,
        strides: MTLTensorExtents,
        sliceOrigin: MTLTensorExtents,
        sliceDimensions: MTLTensorExtents,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getBytes : bytes, strides : strides, fromSliceOrigin : sliceOrigin, sliceDimensions : sliceDimensions)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
    unsafe fn buffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buffer)
    }
    unsafe fn bufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferOffset)
    }
    unsafe fn strides(&self) -> MTLTensorExtents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strides)
    }
    unsafe fn dimensions(&self) -> MTLTensorExtents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensions)
    }
    unsafe fn dataType(&self) -> MTLTensorDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn usage(&self) -> MTLTensorUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usage)
    }
}
pub type MTLGPUAddress = u64;
pub type MTLTextureType = NSUInteger;
pub type MTLTextureSwizzle = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTextureSwizzleChannels {
    pub red: MTLTextureSwizzle,
    pub green: MTLTextureSwizzle,
    pub blue: MTLTextureSwizzle,
    pub alpha: MTLTextureSwizzle,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLSharedTextureHandle(pub id);
impl std::ops::Deref for MTLSharedTextureHandle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLSharedTextureHandle {}
impl MTLSharedTextureHandle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLSharedTextureHandle").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MTLSharedTextureHandle {}
impl INSObject for MTLSharedTextureHandle {}
impl PNSObject for MTLSharedTextureHandle {}
impl std::convert::TryFrom<NSObject> for MTLSharedTextureHandle {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLSharedTextureHandle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLSharedTextureHandle").unwrap()) };
        if is_kind_of {
            Ok(MTLSharedTextureHandle(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLSharedTextureHandle")
        }
    }
}
impl IMTLSharedTextureHandle for MTLSharedTextureHandle {}
pub trait IMTLSharedTextureHandle: Sized + std::ops::Deref {
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
}
pub type MTLTextureUsage = NSUInteger;
pub type MTLTextureCompressionType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTextureDescriptor(pub id);
impl std::ops::Deref for MTLTextureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLTextureDescriptor {}
impl MTLTextureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTextureDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLTextureDescriptor {}
impl INSObject for MTLTextureDescriptor {}
impl PNSObject for MTLTextureDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLTextureDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLTextureDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLTextureDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLTextureDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLTextureDescriptor")
        }
    }
}
impl IMTLTextureDescriptor for MTLTextureDescriptor {}
pub trait IMTLTextureDescriptor: Sized + std::ops::Deref {
    unsafe fn textureType(&self) -> MTLTextureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureType)
    }
    unsafe fn setTextureType_(&self, textureType: MTLTextureType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureType : textureType)
    }
    unsafe fn pixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn setPixelFormat_(&self, pixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelFormat : pixelFormat)
    }
    unsafe fn width(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn depth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depth)
    }
    unsafe fn setDepth_(&self, depth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepth : depth)
    }
    unsafe fn mipmapLevelCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mipmapLevelCount)
    }
    unsafe fn setMipmapLevelCount_(&self, mipmapLevelCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMipmapLevelCount : mipmapLevelCount)
    }
    unsafe fn sampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
    unsafe fn setSampleCount_(&self, sampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleCount : sampleCount)
    }
    unsafe fn arrayLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrayLength)
    }
    unsafe fn setArrayLength_(&self, arrayLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArrayLength : arrayLength)
    }
    unsafe fn resourceOptions(&self) -> MTLResourceOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceOptions)
    }
    unsafe fn setResourceOptions_(&self, resourceOptions: MTLResourceOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResourceOptions : resourceOptions)
    }
    unsafe fn cpuCacheMode(&self) -> MTLCPUCacheMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cpuCacheMode)
    }
    unsafe fn setCpuCacheMode_(&self, cpuCacheMode: MTLCPUCacheMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCpuCacheMode : cpuCacheMode)
    }
    unsafe fn storageMode(&self) -> MTLStorageMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storageMode)
    }
    unsafe fn setStorageMode_(&self, storageMode: MTLStorageMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStorageMode : storageMode)
    }
    unsafe fn hazardTrackingMode(&self) -> MTLHazardTrackingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hazardTrackingMode)
    }
    unsafe fn setHazardTrackingMode_(&self, hazardTrackingMode: MTLHazardTrackingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHazardTrackingMode : hazardTrackingMode)
    }
    unsafe fn usage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usage)
    }
    unsafe fn setUsage_(&self, usage: MTLTextureUsage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsage : usage)
    }
    unsafe fn allowGPUOptimizedContents(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowGPUOptimizedContents)
    }
    unsafe fn setAllowGPUOptimizedContents_(&self, allowGPUOptimizedContents: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowGPUOptimizedContents : allowGPUOptimizedContents)
    }
    unsafe fn compressionType(&self) -> MTLTextureCompressionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compressionType)
    }
    unsafe fn setCompressionType_(&self, compressionType: MTLTextureCompressionType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompressionType : compressionType)
    }
    unsafe fn swizzle(&self) -> MTLTextureSwizzleChannels
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, swizzle)
    }
    unsafe fn setSwizzle_(&self, swizzle: MTLTextureSwizzleChannels)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSwizzle : swizzle)
    }
    unsafe fn placementSparsePageSize(&self) -> MTLSparsePageSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placementSparsePageSize)
    }
    unsafe fn setPlacementSparsePageSize_(&self, placementSparsePageSize: MTLSparsePageSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlacementSparsePageSize : placementSparsePageSize)
    }
    unsafe fn texture2DDescriptorWithPixelFormat_width_height_mipmapped_(
        pixelFormat: MTLPixelFormat,
        width: NSUInteger,
        height: NSUInteger,
        mipmapped: BOOL,
    ) -> MTLTextureDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTextureDescriptor").unwrap(), texture2DDescriptorWithPixelFormat : pixelFormat, width : width, height : height, mipmapped : mipmapped)
    }
    unsafe fn textureCubeDescriptorWithPixelFormat_size_mipmapped_(
        pixelFormat: MTLPixelFormat,
        size: NSUInteger,
        mipmapped: BOOL,
    ) -> MTLTextureDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTextureDescriptor").unwrap(), textureCubeDescriptorWithPixelFormat : pixelFormat, size : size, mipmapped : mipmapped)
    }
    unsafe fn textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage_(
        pixelFormat: MTLPixelFormat,
        width: NSUInteger,
        resourceOptions: MTLResourceOptions,
        usage: MTLTextureUsage,
    ) -> MTLTextureDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTextureDescriptor").unwrap(), textureBufferDescriptorWithPixelFormat : pixelFormat, width : width, resourceOptions : resourceOptions, usage : usage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTextureViewDescriptor(pub id);
impl std::ops::Deref for MTLTextureViewDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLTextureViewDescriptor {}
impl MTLTextureViewDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTextureViewDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLTextureViewDescriptor {}
impl INSObject for MTLTextureViewDescriptor {}
impl PNSObject for MTLTextureViewDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLTextureViewDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLTextureViewDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLTextureViewDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLTextureViewDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLTextureViewDescriptor")
        }
    }
}
impl IMTLTextureViewDescriptor for MTLTextureViewDescriptor {}
pub trait IMTLTextureViewDescriptor: Sized + std::ops::Deref {
    unsafe fn pixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn setPixelFormat_(&self, pixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelFormat : pixelFormat)
    }
    unsafe fn textureType(&self) -> MTLTextureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureType)
    }
    unsafe fn setTextureType_(&self, textureType: MTLTextureType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureType : textureType)
    }
    unsafe fn levelRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, levelRange)
    }
    unsafe fn setLevelRange_(&self, levelRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevelRange : levelRange)
    }
    unsafe fn sliceRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sliceRange)
    }
    unsafe fn setSliceRange_(&self, sliceRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSliceRange : sliceRange)
    }
    unsafe fn swizzle(&self) -> MTLTextureSwizzleChannels
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, swizzle)
    }
    unsafe fn setSwizzle_(&self, swizzle: MTLTextureSwizzleChannels)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSwizzle : swizzle)
    }
}
pub type MTLIndexType = NSUInteger;
pub type MTLBindingType = NSInteger;
pub type MTLArgumentType = NSUInteger;
pub type MTLBindingAccess = NSUInteger;
pub use self::MTLBindingAccess as MTLArgumentAccess;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLType(pub id);
impl std::ops::Deref for MTLType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLType {}
impl MTLType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLType").unwrap(), alloc) })
    }
}
impl INSObject for MTLType {}
impl PNSObject for MTLType {}
impl std::convert::TryFrom<NSObject> for MTLType {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLType, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLType").unwrap()) };
        if is_kind_of {
            Ok(MTLType(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLType")
        }
    }
}
impl IMTLType for MTLType {}
pub trait IMTLType: Sized + std::ops::Deref {
    unsafe fn dataType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLStructMember(pub id);
impl std::ops::Deref for MTLStructMember {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLStructMember {}
impl MTLStructMember {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLStructMember").unwrap(), alloc) })
    }
}
impl INSObject for MTLStructMember {}
impl PNSObject for MTLStructMember {}
impl std::convert::TryFrom<NSObject> for MTLStructMember {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLStructMember, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLStructMember").unwrap()) };
        if is_kind_of {
            Ok(MTLStructMember(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLStructMember")
        }
    }
}
impl IMTLStructMember for MTLStructMember {}
pub trait IMTLStructMember: Sized + std::ops::Deref {
    unsafe fn structType(&self) -> MTLStructType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, structType)
    }
    unsafe fn arrayType(&self) -> MTLArrayType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrayType)
    }
    unsafe fn textureReferenceType(&self) -> MTLTextureReferenceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureReferenceType)
    }
    unsafe fn pointerType(&self) -> MTLPointerType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointerType)
    }
    unsafe fn tensorReferenceType(&self) -> MTLTensorReferenceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tensorReferenceType)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn offset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn dataType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn argumentIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLStructType(pub id);
impl std::ops::Deref for MTLStructType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLStructType {}
impl MTLStructType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLStructType").unwrap(), alloc) })
    }
}
impl IMTLType for MTLStructType {}
impl From<MTLStructType> for MTLType {
    fn from(child: MTLStructType) -> MTLType {
        MTLType(child.0)
    }
}
impl std::convert::TryFrom<MTLType> for MTLStructType {
    type Error = &'static str;
    fn try_from(parent: MTLType) -> Result<MTLStructType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLStructType").unwrap()) };
        if is_kind_of {
            Ok(MTLStructType(parent.0))
        } else {
            Err("This MTLType cannot be downcasted to MTLStructType")
        }
    }
}
impl INSObject for MTLStructType {}
impl PNSObject for MTLStructType {}
impl IMTLStructType for MTLStructType {}
pub trait IMTLStructType: Sized + std::ops::Deref {
    unsafe fn memberByName_(&self, name: NSString) -> MTLStructMember
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, memberByName : name)
    }
    unsafe fn members(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, members)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLArrayType(pub id);
impl std::ops::Deref for MTLArrayType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLArrayType {}
impl MTLArrayType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLArrayType").unwrap(), alloc) })
    }
}
impl IMTLType for MTLArrayType {}
impl std::convert::TryFrom<MTLType> for MTLArrayType {
    type Error = &'static str;
    fn try_from(parent: MTLType) -> Result<MTLArrayType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLArrayType").unwrap()) };
        if is_kind_of {
            Ok(MTLArrayType(parent.0))
        } else {
            Err("This MTLType cannot be downcasted to MTLArrayType")
        }
    }
}
impl INSObject for MTLArrayType {}
impl PNSObject for MTLArrayType {}
impl IMTLArrayType for MTLArrayType {}
pub trait IMTLArrayType: Sized + std::ops::Deref {
    unsafe fn elementStructType(&self) -> MTLStructType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementStructType)
    }
    unsafe fn elementArrayType(&self) -> MTLArrayType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementArrayType)
    }
    unsafe fn elementTextureReferenceType(&self) -> MTLTextureReferenceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementTextureReferenceType)
    }
    unsafe fn elementPointerType(&self) -> MTLPointerType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementPointerType)
    }
    unsafe fn elementTensorReferenceType(&self) -> MTLTensorReferenceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementTensorReferenceType)
    }
    unsafe fn elementType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementType)
    }
    unsafe fn arrayLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrayLength)
    }
    unsafe fn stride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stride)
    }
    unsafe fn argumentIndexStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentIndexStride)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLPointerType(pub id);
impl std::ops::Deref for MTLPointerType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLPointerType {}
impl MTLPointerType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLPointerType").unwrap(), alloc) })
    }
}
impl IMTLType for MTLPointerType {}
impl std::convert::TryFrom<MTLType> for MTLPointerType {
    type Error = &'static str;
    fn try_from(parent: MTLType) -> Result<MTLPointerType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLPointerType").unwrap()) };
        if is_kind_of {
            Ok(MTLPointerType(parent.0))
        } else {
            Err("This MTLType cannot be downcasted to MTLPointerType")
        }
    }
}
impl INSObject for MTLPointerType {}
impl PNSObject for MTLPointerType {}
impl IMTLPointerType for MTLPointerType {}
pub trait IMTLPointerType: Sized + std::ops::Deref {
    unsafe fn elementStructType(&self) -> MTLStructType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementStructType)
    }
    unsafe fn elementArrayType(&self) -> MTLArrayType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementArrayType)
    }
    unsafe fn elementType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementType)
    }
    unsafe fn access(&self) -> MTLBindingAccess
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, access)
    }
    unsafe fn alignment(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignment)
    }
    unsafe fn dataSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSize)
    }
    unsafe fn elementIsArgumentBuffer(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementIsArgumentBuffer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTextureReferenceType(pub id);
impl std::ops::Deref for MTLTextureReferenceType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLTextureReferenceType {}
impl MTLTextureReferenceType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTextureReferenceType").unwrap(), alloc) })
    }
}
impl IMTLType for MTLTextureReferenceType {}
impl std::convert::TryFrom<MTLType> for MTLTextureReferenceType {
    type Error = &'static str;
    fn try_from(parent: MTLType) -> Result<MTLTextureReferenceType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLTextureReferenceType").unwrap()) };
        if is_kind_of {
            Ok(MTLTextureReferenceType(parent.0))
        } else {
            Err("This MTLType cannot be downcasted to MTLTextureReferenceType")
        }
    }
}
impl INSObject for MTLTextureReferenceType {}
impl PNSObject for MTLTextureReferenceType {}
impl IMTLTextureReferenceType for MTLTextureReferenceType {}
pub trait IMTLTextureReferenceType: Sized + std::ops::Deref {
    unsafe fn textureDataType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureDataType)
    }
    unsafe fn textureType(&self) -> MTLTextureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureType)
    }
    unsafe fn access(&self) -> MTLBindingAccess
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, access)
    }
    unsafe fn isDepthTexture(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDepthTexture)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTensorReferenceType(pub id);
impl std::ops::Deref for MTLTensorReferenceType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLTensorReferenceType {}
impl MTLTensorReferenceType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTensorReferenceType").unwrap(), alloc) })
    }
}
impl IMTLType for MTLTensorReferenceType {}
impl std::convert::TryFrom<MTLType> for MTLTensorReferenceType {
    type Error = &'static str;
    fn try_from(parent: MTLType) -> Result<MTLTensorReferenceType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLTensorReferenceType").unwrap()) };
        if is_kind_of {
            Ok(MTLTensorReferenceType(parent.0))
        } else {
            Err("This MTLType cannot be downcasted to MTLTensorReferenceType")
        }
    }
}
impl INSObject for MTLTensorReferenceType {}
impl PNSObject for MTLTensorReferenceType {}
impl IMTLTensorReferenceType for MTLTensorReferenceType {}
pub trait IMTLTensorReferenceType: Sized + std::ops::Deref {
    unsafe fn tensorDataType(&self) -> MTLTensorDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tensorDataType)
    }
    unsafe fn indexType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn dimensions(&self) -> MTLTensorExtents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensions)
    }
    unsafe fn access(&self) -> MTLBindingAccess
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, access)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLArgument(pub id);
impl std::ops::Deref for MTLArgument {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLArgument {}
impl MTLArgument {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLArgument").unwrap(), alloc) })
    }
}
impl INSObject for MTLArgument {}
impl PNSObject for MTLArgument {}
impl std::convert::TryFrom<NSObject> for MTLArgument {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLArgument, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLArgument").unwrap()) };
        if is_kind_of {
            Ok(MTLArgument(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLArgument")
        }
    }
}
impl IMTLArgument for MTLArgument {}
pub trait IMTLArgument: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn type_(&self) -> MTLArgumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn access(&self) -> MTLBindingAccess
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, access)
    }
    unsafe fn index(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, index)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn bufferAlignment(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferAlignment)
    }
    unsafe fn bufferDataSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferDataSize)
    }
    unsafe fn bufferDataType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferDataType)
    }
    unsafe fn bufferStructType(&self) -> MTLStructType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferStructType)
    }
    unsafe fn bufferPointerType(&self) -> MTLPointerType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferPointerType)
    }
    unsafe fn threadgroupMemoryAlignment(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadgroupMemoryAlignment)
    }
    unsafe fn threadgroupMemoryDataSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadgroupMemoryDataSize)
    }
    unsafe fn textureType(&self) -> MTLTextureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureType)
    }
    unsafe fn textureDataType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureDataType)
    }
    unsafe fn isDepthTexture(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDepthTexture)
    }
    unsafe fn arrayLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrayLength)
    }
}
pub trait PMTLBinding: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn type_(&self) -> MTLBindingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn access(&self) -> MTLBindingAccess
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, access)
    }
    unsafe fn index(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, index)
    }
    unsafe fn isUsed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUsed)
    }
    unsafe fn isArgument(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isArgument)
    }
}
pub trait PMTLBufferBinding: Sized + std::ops::Deref {
    unsafe fn bufferAlignment(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferAlignment)
    }
    unsafe fn bufferDataSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferDataSize)
    }
    unsafe fn bufferDataType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferDataType)
    }
    unsafe fn bufferStructType(&self) -> MTLStructType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferStructType)
    }
    unsafe fn bufferPointerType(&self) -> MTLPointerType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferPointerType)
    }
}
pub trait PMTLThreadgroupBinding: Sized + std::ops::Deref {
    unsafe fn threadgroupMemoryAlignment(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadgroupMemoryAlignment)
    }
    unsafe fn threadgroupMemoryDataSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadgroupMemoryDataSize)
    }
}
pub trait PMTLTextureBinding: Sized + std::ops::Deref {
    unsafe fn textureType(&self) -> MTLTextureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureType)
    }
    unsafe fn textureDataType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureDataType)
    }
    unsafe fn isDepthTexture(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDepthTexture)
    }
    unsafe fn arrayLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrayLength)
    }
}
pub trait PMTLObjectPayloadBinding: Sized + std::ops::Deref {
    unsafe fn objectPayloadAlignment(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectPayloadAlignment)
    }
    unsafe fn objectPayloadDataSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectPayloadDataSize)
    }
}
pub trait PMTLTensorBinding: Sized + std::ops::Deref {
    unsafe fn tensorDataType(&self) -> MTLTensorDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tensorDataType)
    }
    unsafe fn indexType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn dimensions(&self) -> MTLTensorExtents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFunctionConstantValues(pub id);
impl std::ops::Deref for MTLFunctionConstantValues {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFunctionConstantValues {}
impl MTLFunctionConstantValues {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFunctionConstantValues").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLFunctionConstantValues {}
impl INSObject for MTLFunctionConstantValues {}
impl PNSObject for MTLFunctionConstantValues {}
impl std::convert::TryFrom<NSObject> for MTLFunctionConstantValues {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFunctionConstantValues, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFunctionConstantValues").unwrap()) };
        if is_kind_of {
            Ok(MTLFunctionConstantValues(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFunctionConstantValues")
        }
    }
}
impl IMTLFunctionConstantValues for MTLFunctionConstantValues {}
pub trait IMTLFunctionConstantValues: Sized + std::ops::Deref {
    unsafe fn setConstantValue_type_atIndex_(
        &self,
        value: *const ::std::os::raw::c_void,
        type_: MTLDataType,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstantValue : value, r#type : type_, atIndex : index)
    }
    unsafe fn setConstantValues_type_withRange_(
        &self,
        values: *const ::std::os::raw::c_void,
        type_: MTLDataType,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstantValues : values, r#type : type_, withRange : range)
    }
    unsafe fn setConstantValue_type_withName_(
        &self,
        value: *const ::std::os::raw::c_void,
        type_: MTLDataType,
        name: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstantValue : value, r#type : type_, withName : name)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
}
pub type MTLFunctionOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFunctionDescriptor(pub id);
impl std::ops::Deref for MTLFunctionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFunctionDescriptor {}
impl MTLFunctionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFunctionDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLFunctionDescriptor {}
impl INSObject for MTLFunctionDescriptor {}
impl PNSObject for MTLFunctionDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLFunctionDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFunctionDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFunctionDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLFunctionDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFunctionDescriptor")
        }
    }
}
impl IMTLFunctionDescriptor for MTLFunctionDescriptor {}
pub trait IMTLFunctionDescriptor: Sized + std::ops::Deref {
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
    unsafe fn specializedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specializedName)
    }
    unsafe fn setSpecializedName_(&self, specializedName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecializedName : specializedName)
    }
    unsafe fn constantValues(&self) -> MTLFunctionConstantValues
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constantValues)
    }
    unsafe fn setConstantValues_(&self, constantValues: MTLFunctionConstantValues)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstantValues : constantValues)
    }
    unsafe fn options(&self) -> MTLFunctionOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: MTLFunctionOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
    unsafe fn binaryArchives(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, binaryArchives)
    }
    unsafe fn setBinaryArchives_(&self, binaryArchives: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBinaryArchives : binaryArchives)
    }
    unsafe fn functionDescriptor() -> MTLFunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFunctionDescriptor").unwrap(), functionDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLIntersectionFunctionDescriptor(pub id);
impl std::ops::Deref for MTLIntersectionFunctionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLIntersectionFunctionDescriptor {}
impl MTLIntersectionFunctionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLIntersectionFunctionDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLIntersectionFunctionDescriptor {}
impl IMTLFunctionDescriptor for MTLIntersectionFunctionDescriptor {}
impl From<MTLIntersectionFunctionDescriptor> for MTLFunctionDescriptor {
    fn from(child: MTLIntersectionFunctionDescriptor) -> MTLFunctionDescriptor {
        MTLFunctionDescriptor(child.0)
    }
}
impl std::convert::TryFrom<MTLFunctionDescriptor> for MTLIntersectionFunctionDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MTLFunctionDescriptor,
    ) -> Result<MTLIntersectionFunctionDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLIntersectionFunctionDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLIntersectionFunctionDescriptor(parent.0))
        } else {
            Err ("This MTLFunctionDescriptor cannot be downcasted to MTLIntersectionFunctionDescriptor" ,)
        }
    }
}
impl INSObject for MTLIntersectionFunctionDescriptor {}
impl PNSObject for MTLIntersectionFunctionDescriptor {}
impl IMTLIntersectionFunctionDescriptor for MTLIntersectionFunctionDescriptor {}
pub trait IMTLIntersectionFunctionDescriptor: Sized + std::ops::Deref {}
pub type MTLNewDynamicLibraryCompletionHandler = *mut ::std::os::raw::c_void;
pub type MTLPatchType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLVertexAttribute(pub id);
impl std::ops::Deref for MTLVertexAttribute {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLVertexAttribute {}
impl MTLVertexAttribute {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLVertexAttribute").unwrap(), alloc) })
    }
}
impl INSObject for MTLVertexAttribute {}
impl PNSObject for MTLVertexAttribute {}
impl std::convert::TryFrom<NSObject> for MTLVertexAttribute {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLVertexAttribute, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLVertexAttribute").unwrap()) };
        if is_kind_of {
            Ok(MTLVertexAttribute(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLVertexAttribute")
        }
    }
}
impl IMTLVertexAttribute for MTLVertexAttribute {}
pub trait IMTLVertexAttribute: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn attributeIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeIndex)
    }
    unsafe fn attributeType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeType)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn isPatchData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPatchData)
    }
    unsafe fn isPatchControlPointData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPatchControlPointData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAttribute(pub id);
impl std::ops::Deref for MTLAttribute {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAttribute {}
impl MTLAttribute {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAttribute").unwrap(), alloc) })
    }
}
impl INSObject for MTLAttribute {}
impl PNSObject for MTLAttribute {}
impl std::convert::TryFrom<NSObject> for MTLAttribute {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLAttribute, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAttribute").unwrap()) };
        if is_kind_of {
            Ok(MTLAttribute(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLAttribute")
        }
    }
}
impl IMTLAttribute for MTLAttribute {}
pub trait IMTLAttribute: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn attributeIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeIndex)
    }
    unsafe fn attributeType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeType)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn isPatchData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPatchData)
    }
    unsafe fn isPatchControlPointData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPatchControlPointData)
    }
}
pub type MTLFunctionType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFunctionConstant(pub id);
impl std::ops::Deref for MTLFunctionConstant {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFunctionConstant {}
impl MTLFunctionConstant {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFunctionConstant").unwrap(), alloc) })
    }
}
impl INSObject for MTLFunctionConstant {}
impl PNSObject for MTLFunctionConstant {}
impl std::convert::TryFrom<NSObject> for MTLFunctionConstant {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFunctionConstant, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFunctionConstant").unwrap()) };
        if is_kind_of {
            Ok(MTLFunctionConstant(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFunctionConstant")
        }
    }
}
impl IMTLFunctionConstant for MTLFunctionConstant {}
pub trait IMTLFunctionConstant: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn type_(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn index(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, index)
    }
    unsafe fn required(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, required)
    }
}
pub trait PMTLFunction: Sized + std::ops::Deref {
    unsafe fn newArgumentEncoderWithBufferIndex_(&self, bufferIndex: NSUInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newArgumentEncoderWithBufferIndex : bufferIndex)
    }
    unsafe fn newArgumentEncoderWithBufferIndex_reflection_(
        &self,
        bufferIndex: NSUInteger,
        reflection: *mut MTLArgument,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newArgumentEncoderWithBufferIndex : bufferIndex, reflection : reflection)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn functionType(&self) -> MTLFunctionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionType)
    }
    unsafe fn patchType(&self) -> MTLPatchType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, patchType)
    }
    unsafe fn patchControlPointCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, patchControlPointCount)
    }
    unsafe fn vertexAttributes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexAttributes)
    }
    unsafe fn stageInputAttributes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stageInputAttributes)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn functionConstantsDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionConstantsDictionary)
    }
    unsafe fn options(&self) -> MTLFunctionOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
}
pub type MTLLanguageVersion = NSUInteger;
pub type MTLLibraryType = NSInteger;
pub type MTLLibraryOptimizationLevel = NSInteger;
pub type MTLCompileSymbolVisibility = NSInteger;
pub type MTLMathMode = NSInteger;
pub type MTLMathFloatingPointFunctions = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCompileOptions(pub id);
impl std::ops::Deref for MTLCompileOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLCompileOptions {}
impl MTLCompileOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLCompileOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLCompileOptions {}
impl INSObject for MTLCompileOptions {}
impl PNSObject for MTLCompileOptions {}
impl std::convert::TryFrom<NSObject> for MTLCompileOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLCompileOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLCompileOptions").unwrap()) };
        if is_kind_of {
            Ok(MTLCompileOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLCompileOptions")
        }
    }
}
impl IMTLCompileOptions for MTLCompileOptions {}
pub trait IMTLCompileOptions: Sized + std::ops::Deref {
    unsafe fn preprocessorMacros(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preprocessorMacros)
    }
    unsafe fn setPreprocessorMacros_(&self, preprocessorMacros: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreprocessorMacros : preprocessorMacros)
    }
    unsafe fn fastMathEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fastMathEnabled)
    }
    unsafe fn setFastMathEnabled_(&self, fastMathEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFastMathEnabled : fastMathEnabled)
    }
    unsafe fn mathMode(&self) -> MTLMathMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mathMode)
    }
    unsafe fn setMathMode_(&self, mathMode: MTLMathMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMathMode : mathMode)
    }
    unsafe fn mathFloatingPointFunctions(&self) -> MTLMathFloatingPointFunctions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mathFloatingPointFunctions)
    }
    unsafe fn setMathFloatingPointFunctions_(
        &self,
        mathFloatingPointFunctions: MTLMathFloatingPointFunctions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMathFloatingPointFunctions : mathFloatingPointFunctions)
    }
    unsafe fn languageVersion(&self) -> MTLLanguageVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageVersion)
    }
    unsafe fn setLanguageVersion_(&self, languageVersion: MTLLanguageVersion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanguageVersion : languageVersion)
    }
    unsafe fn libraryType(&self) -> MTLLibraryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, libraryType)
    }
    unsafe fn setLibraryType_(&self, libraryType: MTLLibraryType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLibraryType : libraryType)
    }
    unsafe fn installName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, installName)
    }
    unsafe fn setInstallName_(&self, installName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstallName : installName)
    }
    unsafe fn libraries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, libraries)
    }
    unsafe fn setLibraries_(&self, libraries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLibraries : libraries)
    }
    unsafe fn preserveInvariance(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preserveInvariance)
    }
    unsafe fn setPreserveInvariance_(&self, preserveInvariance: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreserveInvariance : preserveInvariance)
    }
    unsafe fn optimizationLevel(&self) -> MTLLibraryOptimizationLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optimizationLevel)
    }
    unsafe fn setOptimizationLevel_(&self, optimizationLevel: MTLLibraryOptimizationLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptimizationLevel : optimizationLevel)
    }
    unsafe fn compileSymbolVisibility(&self) -> MTLCompileSymbolVisibility
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compileSymbolVisibility)
    }
    unsafe fn setCompileSymbolVisibility_(
        &self,
        compileSymbolVisibility: MTLCompileSymbolVisibility,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompileSymbolVisibility : compileSymbolVisibility)
    }
    unsafe fn allowReferencingUndefinedSymbols(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowReferencingUndefinedSymbols)
    }
    unsafe fn setAllowReferencingUndefinedSymbols_(&self, allowReferencingUndefinedSymbols: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowReferencingUndefinedSymbols : allowReferencingUndefinedSymbols)
    }
    unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerThreadgroup)
    }
    unsafe fn setMaxTotalThreadsPerThreadgroup_(&self, maxTotalThreadsPerThreadgroup: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadsPerThreadgroup : maxTotalThreadsPerThreadgroup)
    }
    unsafe fn requiredThreadsPerThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerThreadgroup)
    }
    unsafe fn setRequiredThreadsPerThreadgroup_(&self, requiredThreadsPerThreadgroup: MTLSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredThreadsPerThreadgroup : requiredThreadsPerThreadgroup)
    }
    unsafe fn enableLogging(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableLogging)
    }
    unsafe fn setEnableLogging_(&self, enableLogging: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnableLogging : enableLogging)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFunctionReflection(pub id);
impl std::ops::Deref for MTLFunctionReflection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFunctionReflection {}
impl MTLFunctionReflection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFunctionReflection").unwrap(), alloc) })
    }
}
impl INSObject for MTLFunctionReflection {}
impl PNSObject for MTLFunctionReflection {}
impl std::convert::TryFrom<NSObject> for MTLFunctionReflection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFunctionReflection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFunctionReflection").unwrap()) };
        if is_kind_of {
            Ok(MTLFunctionReflection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFunctionReflection")
        }
    }
}
impl IMTLFunctionReflection for MTLFunctionReflection {}
pub trait IMTLFunctionReflection: Sized + std::ops::Deref {
    unsafe fn bindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bindings)
    }
    unsafe fn userAnnotation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userAnnotation)
    }
}
pub type MTLLibraryError = NSUInteger;
pub trait PMTLLibrary: Sized + std::ops::Deref {
    unsafe fn newFunctionWithName_(&self, functionName: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newFunctionWithName : functionName)
    }
    unsafe fn newFunctionWithName_constantValues_error_(
        &self,
        name: NSString,
        constantValues: MTLFunctionConstantValues,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newFunctionWithName : name, constantValues : constantValues, error : error)
    }
    unsafe fn newFunctionWithName_constantValues_completionHandler_(
        &self,
        name: NSString,
        constantValues: MTLFunctionConstantValues,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newFunctionWithName : name, constantValues : constantValues, completionHandler : completionHandler)
    }
    unsafe fn reflectionForFunctionWithName_(&self, functionName: NSString) -> MTLFunctionReflection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reflectionForFunctionWithName : functionName)
    }
    unsafe fn newFunctionWithDescriptor_completionHandler_(
        &self,
        descriptor: MTLFunctionDescriptor,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newFunctionWithDescriptor : descriptor, completionHandler : completionHandler)
    }
    unsafe fn newFunctionWithDescriptor_error_(
        &self,
        descriptor: MTLFunctionDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newFunctionWithDescriptor : descriptor, error : error)
    }
    unsafe fn newIntersectionFunctionWithDescriptor_completionHandler_(
        &self,
        descriptor: MTLIntersectionFunctionDescriptor,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIntersectionFunctionWithDescriptor : descriptor, completionHandler : completionHandler)
    }
    unsafe fn newIntersectionFunctionWithDescriptor_error_(
        &self,
        descriptor: MTLIntersectionFunctionDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIntersectionFunctionWithDescriptor : descriptor, error : error)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn functionNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionNames)
    }
    unsafe fn type_(&self) -> MTLLibraryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn installName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, installName)
    }
}
pub type MTLCommonCounter = NSString;
pub type MTLCommonCounterSet = NSString;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCounterResultTimestamp {
    pub timestamp: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCounterResultStageUtilization {
    pub totalCycles: u64,
    pub vertexCycles: u64,
    pub tessellationCycles: u64,
    pub postTessellationVertexCycles: u64,
    pub fragmentCycles: u64,
    pub renderTargetCycles: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCounterResultStatistic {
    pub tessellationInputPatches: u64,
    pub vertexInvocations: u64,
    pub postTessellationVertexInvocations: u64,
    pub clipperInvocations: u64,
    pub clipperPrimitivesOut: u64,
    pub fragmentInvocations: u64,
    pub fragmentsPassed: u64,
    pub computeKernelInvocations: u64,
}
pub trait PMTLCounter: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
}
pub trait PMTLCounterSet: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn counters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, counters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCounterSampleBufferDescriptor(pub id);
impl std::ops::Deref for MTLCounterSampleBufferDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLCounterSampleBufferDescriptor {}
impl MTLCounterSampleBufferDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLCounterSampleBufferDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLCounterSampleBufferDescriptor {}
impl INSObject for MTLCounterSampleBufferDescriptor {}
impl PNSObject for MTLCounterSampleBufferDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLCounterSampleBufferDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLCounterSampleBufferDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLCounterSampleBufferDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLCounterSampleBufferDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLCounterSampleBufferDescriptor")
        }
    }
}
impl IMTLCounterSampleBufferDescriptor for MTLCounterSampleBufferDescriptor {}
pub trait IMTLCounterSampleBufferDescriptor: Sized + std::ops::Deref {
    unsafe fn counterSet(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, counterSet)
    }
    unsafe fn setCounterSet_(&self, counterSet: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCounterSet : counterSet)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn storageMode(&self) -> MTLStorageMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storageMode)
    }
    unsafe fn setStorageMode_(&self, storageMode: MTLStorageMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStorageMode : storageMode)
    }
    unsafe fn sampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
    unsafe fn setSampleCount_(&self, sampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleCount : sampleCount)
    }
}
pub trait PMTLCounterSampleBuffer: Sized + std::ops::Deref {
    unsafe fn resolveCounterRange_(&self, range: NSRange) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveCounterRange : range)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn sampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
}
pub type MTLCounterSampleBufferError = NSInteger;
pub type MTL4CompilerTaskStatus = NSInteger;
pub trait PMTL4CompilerTask: Sized + std::ops::Deref {
    unsafe fn waitUntilCompleted(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, waitUntilCompleted)
    }
    unsafe fn compiler(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compiler)
    }
    unsafe fn status(&self) -> MTL4CompilerTaskStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4CompilerDescriptor(pub id);
impl std::ops::Deref for MTL4CompilerDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4CompilerDescriptor {}
impl MTL4CompilerDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4CompilerDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4CompilerDescriptor {}
impl INSObject for MTL4CompilerDescriptor {}
impl PNSObject for MTL4CompilerDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4CompilerDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4CompilerDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4CompilerDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4CompilerDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4CompilerDescriptor")
        }
    }
}
impl IMTL4CompilerDescriptor for MTL4CompilerDescriptor {}
pub trait IMTL4CompilerDescriptor: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn pipelineDataSetSerializer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pipelineDataSetSerializer)
    }
    unsafe fn setPipelineDataSetSerializer_(&self, pipelineDataSetSerializer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPipelineDataSetSerializer : pipelineDataSetSerializer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4CompilerTaskOptions(pub id);
impl std::ops::Deref for MTL4CompilerTaskOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4CompilerTaskOptions {}
impl MTL4CompilerTaskOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4CompilerTaskOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4CompilerTaskOptions {}
impl INSObject for MTL4CompilerTaskOptions {}
impl PNSObject for MTL4CompilerTaskOptions {}
impl std::convert::TryFrom<NSObject> for MTL4CompilerTaskOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4CompilerTaskOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4CompilerTaskOptions").unwrap()) };
        if is_kind_of {
            Ok(MTL4CompilerTaskOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4CompilerTaskOptions")
        }
    }
}
impl IMTL4CompilerTaskOptions for MTL4CompilerTaskOptions {}
pub trait IMTL4CompilerTaskOptions: Sized + std::ops::Deref {
    unsafe fn lookupArchives(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lookupArchives)
    }
    unsafe fn setLookupArchives_(&self, lookupArchives: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLookupArchives : lookupArchives)
    }
}
pub type MTL4NewBinaryFunctionCompletionHandler = *mut ::std::os::raw::c_void;
pub type MTL4NewMachineLearningPipelineStateCompletionHandler = *mut ::std::os::raw::c_void;
pub trait PMTL4Compiler: Sized + std::ops::Deref {
    unsafe fn newLibraryWithDescriptor_error_(
        &self,
        descriptor: MTL4LibraryDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLibraryWithDescriptor : descriptor, error : error)
    }
    unsafe fn newDynamicLibrary_error_(&self, library: *mut u64, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newDynamicLibrary : library, error : error)
    }
    unsafe fn newDynamicLibraryWithURL_error_(&self, url: NSURL, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newDynamicLibraryWithURL : url, error : error)
    }
    unsafe fn newComputePipelineStateWithDescriptor_compilerTaskOptions_error_(
        &self,
        descriptor: MTL4ComputePipelineDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithDescriptor : descriptor, compilerTaskOptions : compilerTaskOptions, error : error)
    }
    unsafe fn newComputePipelineStateWithDescriptor_dynamicLinkingDescriptor_compilerTaskOptions_error_(
        &self,
        descriptor: MTL4ComputePipelineDescriptor,
        dynamicLinkingDescriptor: MTL4PipelineStageDynamicLinkingDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithDescriptor : descriptor, dynamicLinkingDescriptor : dynamicLinkingDescriptor, compilerTaskOptions : compilerTaskOptions, error : error)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_compilerTaskOptions_error_(
        &self,
        descriptor: MTL4PipelineDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, compilerTaskOptions : compilerTaskOptions, error : error)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_dynamicLinkingDescriptor_compilerTaskOptions_error_(
        &self,
        descriptor: MTL4PipelineDescriptor,
        dynamicLinkingDescriptor: MTL4RenderPipelineDynamicLinkingDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, dynamicLinkingDescriptor : dynamicLinkingDescriptor, compilerTaskOptions : compilerTaskOptions, error : error)
    }
    unsafe fn newRenderPipelineStateBySpecializationWithDescriptor_pipeline_error_(
        &self,
        descriptor: MTL4PipelineDescriptor,
        pipeline: *mut u64,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateBySpecializationWithDescriptor : descriptor, pipeline : pipeline, error : error)
    }
    unsafe fn newBinaryFunctionWithDescriptor_compilerTaskOptions_error_(
        &self,
        descriptor: MTL4BinaryFunctionDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBinaryFunctionWithDescriptor : descriptor, compilerTaskOptions : compilerTaskOptions, error : error)
    }
    unsafe fn newLibraryWithDescriptor_completionHandler_(
        &self,
        descriptor: MTL4LibraryDescriptor,
        completionHandler: MTLNewLibraryCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLibraryWithDescriptor : descriptor, completionHandler : completionHandler)
    }
    unsafe fn newDynamicLibrary_completionHandler_(
        &self,
        library: *mut u64,
        completionHandler: MTLNewDynamicLibraryCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newDynamicLibrary : library, completionHandler : completionHandler)
    }
    unsafe fn newDynamicLibraryWithURL_completionHandler_(
        &self,
        url: NSURL,
        completionHandler: MTLNewDynamicLibraryCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newDynamicLibraryWithURL : url, completionHandler : completionHandler)
    }
    unsafe fn newComputePipelineStateWithDescriptor_compilerTaskOptions_completionHandler_(
        &self,
        descriptor: MTL4ComputePipelineDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        completionHandler: MTLNewComputePipelineStateCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithDescriptor : descriptor, compilerTaskOptions : compilerTaskOptions, completionHandler : completionHandler)
    }
    unsafe fn newComputePipelineStateWithDescriptor_dynamicLinkingDescriptor_compilerTaskOptions_completionHandler_(
        &self,
        descriptor: MTL4ComputePipelineDescriptor,
        dynamicLinkingDescriptor: MTL4PipelineStageDynamicLinkingDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        completionHandler: MTLNewComputePipelineStateCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithDescriptor : descriptor, dynamicLinkingDescriptor : dynamicLinkingDescriptor, compilerTaskOptions : compilerTaskOptions, completionHandler : completionHandler)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_compilerTaskOptions_completionHandler_(
        &self,
        descriptor: MTL4PipelineDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        completionHandler: MTLNewRenderPipelineStateCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, compilerTaskOptions : compilerTaskOptions, completionHandler : completionHandler)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_dynamicLinkingDescriptor_compilerTaskOptions_completionHandler_(
        &self,
        descriptor: MTL4PipelineDescriptor,
        dynamicLinkingDescriptor: MTL4RenderPipelineDynamicLinkingDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        completionHandler: MTLNewRenderPipelineStateCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, dynamicLinkingDescriptor : dynamicLinkingDescriptor, compilerTaskOptions : compilerTaskOptions, completionHandler : completionHandler)
    }
    unsafe fn newRenderPipelineStateBySpecializationWithDescriptor_pipeline_completionHandler_(
        &self,
        descriptor: MTL4PipelineDescriptor,
        pipeline: *mut u64,
        completionHandler: MTLNewRenderPipelineStateCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateBySpecializationWithDescriptor : descriptor, pipeline : pipeline, completionHandler : completionHandler)
    }
    unsafe fn newBinaryFunctionWithDescriptor_compilerTaskOptions_completionHandler_(
        &self,
        descriptor: MTL4BinaryFunctionDescriptor,
        compilerTaskOptions: MTL4CompilerTaskOptions,
        completionHandler: MTL4NewBinaryFunctionCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBinaryFunctionWithDescriptor : descriptor, compilerTaskOptions : compilerTaskOptions, completionHandler : completionHandler)
    }
    unsafe fn newMachineLearningPipelineStateWithDescriptor_error_(
        &self,
        descriptor: MTL4MachineLearningPipelineDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newMachineLearningPipelineStateWithDescriptor : descriptor, error : error)
    }
    unsafe fn newMachineLearningPipelineStateWithDescriptor_completionHandler_(
        &self,
        descriptor: MTL4MachineLearningPipelineDescriptor,
        completionHandler: MTL4NewMachineLearningPipelineStateCompletionHandler,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newMachineLearningPipelineStateWithDescriptor : descriptor, completionHandler : completionHandler)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn pipelineDataSetSerializer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pipelineDataSetSerializer)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4TimestampHeapEntry {
    pub timestamp: u64,
}
pub type MTL4CounterHeapType = NSInteger;
pub type MTL4TimestampGranularity = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4CounterHeapDescriptor(pub id);
impl std::ops::Deref for MTL4CounterHeapDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4CounterHeapDescriptor {}
impl MTL4CounterHeapDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4CounterHeapDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4CounterHeapDescriptor {}
impl INSObject for MTL4CounterHeapDescriptor {}
impl PNSObject for MTL4CounterHeapDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4CounterHeapDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4CounterHeapDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4CounterHeapDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4CounterHeapDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4CounterHeapDescriptor")
        }
    }
}
impl IMTL4CounterHeapDescriptor for MTL4CounterHeapDescriptor {}
pub trait IMTL4CounterHeapDescriptor: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> MTL4CounterHeapType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: MTL4CounterHeapType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn setCount_(&self, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCount : count)
    }
}
pub trait PMTL4CounterHeap: Sized + std::ops::Deref {
    unsafe fn resolveCounterRange_(&self, range: NSRange) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveCounterRange : range)
    }
    unsafe fn invalidateCounterRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateCounterRange : range)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn type_(&self) -> MTL4CounterHeapType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
pub type MTLIOCompressionMethod = NSInteger;
pub type MTLFeatureSet = NSUInteger;
pub type MTLGPUFamily = NSInteger;
pub type MTLPipelineOption = NSUInteger;
pub type MTLReadWriteTextureTier = NSUInteger;
pub type MTLArgumentBuffersTier = NSUInteger;
pub type MTLSparseTextureRegionAlignmentMode = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureSizes {
    pub accelerationStructureSize: NSUInteger,
    pub buildScratchBufferSize: NSUInteger,
    pub refitScratchBufferSize: NSUInteger,
}
pub type MTLCounterSamplingPoint = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLSizeAndAlign {
    pub size: NSUInteger,
    pub align: NSUInteger,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLArgumentDescriptor(pub id);
impl std::ops::Deref for MTLArgumentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLArgumentDescriptor {}
impl MTLArgumentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLArgumentDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLArgumentDescriptor {}
impl INSObject for MTLArgumentDescriptor {}
impl PNSObject for MTLArgumentDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLArgumentDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLArgumentDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLArgumentDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLArgumentDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLArgumentDescriptor")
        }
    }
}
impl IMTLArgumentDescriptor for MTLArgumentDescriptor {}
pub trait IMTLArgumentDescriptor: Sized + std::ops::Deref {
    unsafe fn dataType(&self) -> MTLDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn setDataType_(&self, dataType: MTLDataType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataType : dataType)
    }
    unsafe fn index(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, index)
    }
    unsafe fn setIndex_(&self, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndex : index)
    }
    unsafe fn arrayLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrayLength)
    }
    unsafe fn setArrayLength_(&self, arrayLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArrayLength : arrayLength)
    }
    unsafe fn access(&self) -> MTLBindingAccess
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, access)
    }
    unsafe fn setAccess_(&self, access: MTLBindingAccess)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccess : access)
    }
    unsafe fn textureType(&self) -> MTLTextureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureType)
    }
    unsafe fn setTextureType_(&self, textureType: MTLTextureType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureType : textureType)
    }
    unsafe fn constantBlockAlignment(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constantBlockAlignment)
    }
    unsafe fn setConstantBlockAlignment_(&self, constantBlockAlignment: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstantBlockAlignment : constantBlockAlignment)
    }
    unsafe fn argumentDescriptor() -> MTLArgumentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLArgumentDescriptor").unwrap(), argumentDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLArchitecture(pub id);
impl std::ops::Deref for MTLArchitecture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLArchitecture {}
impl MTLArchitecture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLArchitecture").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLArchitecture {}
impl INSObject for MTLArchitecture {}
impl PNSObject for MTLArchitecture {}
impl std::convert::TryFrom<NSObject> for MTLArchitecture {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLArchitecture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLArchitecture").unwrap()) };
        if is_kind_of {
            Ok(MTLArchitecture(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLArchitecture")
        }
    }
}
impl IMTLArchitecture for MTLArchitecture {}
pub trait IMTLArchitecture: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
}
pub type MTLTimestamp = u64;
pub trait PMTLFence: Sized + std::ops::Deref {
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLResourceStatePassSampleBufferAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLResourceStatePassSampleBufferAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLResourceStatePassSampleBufferAttachmentDescriptor {}
impl MTLResourceStatePassSampleBufferAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLResourceStatePassSampleBufferAttachmentDescriptor").unwrap(), alloc)
        })
    }
}
impl PNSCopying for MTLResourceStatePassSampleBufferAttachmentDescriptor {}
impl INSObject for MTLResourceStatePassSampleBufferAttachmentDescriptor {}
impl PNSObject for MTLResourceStatePassSampleBufferAttachmentDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLResourceStatePassSampleBufferAttachmentDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLResourceStatePassSampleBufferAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLResourceStatePassSampleBufferAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLResourceStatePassSampleBufferAttachmentDescriptor(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to MTLResourceStatePassSampleBufferAttachmentDescriptor" ,)
        }
    }
}
impl IMTLResourceStatePassSampleBufferAttachmentDescriptor
    for MTLResourceStatePassSampleBufferAttachmentDescriptor
{
}
pub trait IMTLResourceStatePassSampleBufferAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn sampleBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBuffer)
    }
    unsafe fn setSampleBuffer_(&self, sampleBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleBuffer : sampleBuffer)
    }
    unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startOfEncoderSampleIndex)
    }
    unsafe fn setStartOfEncoderSampleIndex_(&self, startOfEncoderSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartOfEncoderSampleIndex : startOfEncoderSampleIndex)
    }
    unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endOfEncoderSampleIndex)
    }
    unsafe fn setEndOfEncoderSampleIndex_(&self, endOfEncoderSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndOfEncoderSampleIndex : endOfEncoderSampleIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLResourceStatePassSampleBufferAttachmentDescriptorArray(pub id);
impl std::ops::Deref for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {}
impl MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLResourceStatePassSampleBufferAttachmentDescriptorArray").unwrap(), alloc)
        })
    }
}
impl INSObject for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {}
impl PNSObject for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLResourceStatePassSampleBufferAttachmentDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLResourceStatePassSampleBufferAttachmentDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLResourceStatePassSampleBufferAttachmentDescriptorArray(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to MTLResourceStatePassSampleBufferAttachmentDescriptorArray" ,)
        }
    }
}
impl IMTLResourceStatePassSampleBufferAttachmentDescriptorArray
    for MTLResourceStatePassSampleBufferAttachmentDescriptorArray
{
}
pub trait IMTLResourceStatePassSampleBufferAttachmentDescriptorArray:
    Sized + std::ops::Deref
{
    unsafe fn objectAtIndexedSubscript_(
        &self,
        attachmentIndex: NSUInteger,
    ) -> MTLResourceStatePassSampleBufferAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : attachmentIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attachment: MTLResourceStatePassSampleBufferAttachmentDescriptor,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attachment, atIndexedSubscript : attachmentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLResourceStatePassDescriptor(pub id);
impl std::ops::Deref for MTLResourceStatePassDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLResourceStatePassDescriptor {}
impl MTLResourceStatePassDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLResourceStatePassDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLResourceStatePassDescriptor {}
impl INSObject for MTLResourceStatePassDescriptor {}
impl PNSObject for MTLResourceStatePassDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLResourceStatePassDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLResourceStatePassDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLResourceStatePassDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLResourceStatePassDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLResourceStatePassDescriptor")
        }
    }
}
impl IMTLResourceStatePassDescriptor for MTLResourceStatePassDescriptor {}
pub trait IMTLResourceStatePassDescriptor: Sized + std::ops::Deref {
    unsafe fn sampleBufferAttachments(
        &self,
    ) -> MTLResourceStatePassSampleBufferAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBufferAttachments)
    }
    unsafe fn resourceStatePassDescriptor() -> MTLResourceStatePassDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLResourceStatePassDescriptor").unwrap(), resourceStatePassDescriptor)
    }
}
pub type MTLSparseTextureMappingMode = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLMapIndirectArguments {
    pub regionOriginX: u32,
    pub regionOriginY: u32,
    pub regionOriginZ: u32,
    pub regionSizeWidth: u32,
    pub regionSizeHeight: u32,
    pub regionSizeDepth: u32,
    pub mipMapLevel: u32,
    pub sliceId: u32,
}
pub trait PMTLResourceStateCommandEncoder: Sized + std::ops::Deref {
    unsafe fn updateTextureMappings_mode_regions_mipLevels_slices_numRegions_(
        &self,
        texture: *mut u64,
        mode: MTLSparseTextureMappingMode,
        regions: *const MTLRegion,
        mipLevels: *const NSUInteger,
        slices: *const NSUInteger,
        numRegions: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTextureMappings : texture, mode : mode, regions : regions, mipLevels : mipLevels, slices : slices, numRegions : numRegions)
    }
    unsafe fn updateTextureMapping_mode_region_mipLevel_slice_(
        &self,
        texture: *mut u64,
        mode: MTLSparseTextureMappingMode,
        region: MTLRegion,
        mipLevel: NSUInteger,
        slice: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTextureMapping : texture, mode : mode, region : region, mipLevel : mipLevel, slice : slice)
    }
    unsafe fn updateTextureMapping_mode_indirectBuffer_indirectBufferOffset_(
        &self,
        texture: *mut u64,
        mode: MTLSparseTextureMappingMode,
        indirectBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTextureMapping : texture, mode : mode, indirectBuffer : indirectBuffer, indirectBufferOffset : indirectBufferOffset)
    }
    unsafe fn updateFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateFence : fence)
    }
    unsafe fn waitForFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForFence : fence)
    }
    unsafe fn moveTextureMappingsFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_(
        &self,
        sourceTexture: *mut u64,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        sourceOrigin: MTLOrigin,
        sourceSize: MTLSize,
        destinationTexture: *mut u64,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        destinationOrigin: MTLOrigin,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveTextureMappingsFromTexture : sourceTexture, sourceSlice : sourceSlice, sourceLevel : sourceLevel, sourceOrigin : sourceOrigin, sourceSize : sourceSize, toTexture : destinationTexture, destinationSlice : destinationSlice, destinationLevel : destinationLevel, destinationOrigin : destinationOrigin)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}
pub type MTLLoadAction = NSUInteger;
pub type MTLStoreAction = NSUInteger;
pub type MTLStoreActionOptions = NSUInteger;
pub type MTLVisibilityResultType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPassAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLRenderPassAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPassAttachmentDescriptor {}
impl MTLRenderPassAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPassAttachmentDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLRenderPassAttachmentDescriptor {}
impl INSObject for MTLRenderPassAttachmentDescriptor {}
impl PNSObject for MTLRenderPassAttachmentDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLRenderPassAttachmentDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLRenderPassAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPassAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPassAttachmentDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRenderPassAttachmentDescriptor")
        }
    }
}
impl IMTLRenderPassAttachmentDescriptor for MTLRenderPassAttachmentDescriptor {}
pub trait IMTLRenderPassAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn texture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texture)
    }
    unsafe fn setTexture_(&self, texture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTexture : texture)
    }
    unsafe fn level(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, level)
    }
    unsafe fn setLevel_(&self, level: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevel : level)
    }
    unsafe fn slice(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, slice)
    }
    unsafe fn setSlice_(&self, slice: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSlice : slice)
    }
    unsafe fn depthPlane(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthPlane)
    }
    unsafe fn setDepthPlane_(&self, depthPlane: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthPlane : depthPlane)
    }
    unsafe fn resolveTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolveTexture)
    }
    unsafe fn setResolveTexture_(&self, resolveTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResolveTexture : resolveTexture)
    }
    unsafe fn resolveLevel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolveLevel)
    }
    unsafe fn setResolveLevel_(&self, resolveLevel: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResolveLevel : resolveLevel)
    }
    unsafe fn resolveSlice(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolveSlice)
    }
    unsafe fn setResolveSlice_(&self, resolveSlice: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResolveSlice : resolveSlice)
    }
    unsafe fn resolveDepthPlane(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolveDepthPlane)
    }
    unsafe fn setResolveDepthPlane_(&self, resolveDepthPlane: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResolveDepthPlane : resolveDepthPlane)
    }
    unsafe fn loadAction(&self) -> MTLLoadAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loadAction)
    }
    unsafe fn setLoadAction_(&self, loadAction: MTLLoadAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoadAction : loadAction)
    }
    unsafe fn storeAction(&self) -> MTLStoreAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storeAction)
    }
    unsafe fn setStoreAction_(&self, storeAction: MTLStoreAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStoreAction : storeAction)
    }
    unsafe fn storeActionOptions(&self) -> MTLStoreActionOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storeActionOptions)
    }
    unsafe fn setStoreActionOptions_(&self, storeActionOptions: MTLStoreActionOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStoreActionOptions : storeActionOptions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPassColorAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLRenderPassColorAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPassColorAttachmentDescriptor {}
impl MTLRenderPassColorAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPassColorAttachmentDescriptor").unwrap(), alloc) })
    }
}
impl IMTLRenderPassAttachmentDescriptor for MTLRenderPassColorAttachmentDescriptor {}
impl PNSCopying for MTLRenderPassColorAttachmentDescriptor {}
impl From<MTLRenderPassColorAttachmentDescriptor> for MTLRenderPassAttachmentDescriptor {
    fn from(child: MTLRenderPassColorAttachmentDescriptor) -> MTLRenderPassAttachmentDescriptor {
        MTLRenderPassAttachmentDescriptor(child.0)
    }
}
impl std::convert::TryFrom<MTLRenderPassAttachmentDescriptor>
    for MTLRenderPassColorAttachmentDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLRenderPassAttachmentDescriptor,
    ) -> Result<MTLRenderPassColorAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPassColorAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPassColorAttachmentDescriptor(parent.0))
        } else {
            Err ("This MTLRenderPassAttachmentDescriptor cannot be downcasted to MTLRenderPassColorAttachmentDescriptor" ,)
        }
    }
}
impl INSObject for MTLRenderPassColorAttachmentDescriptor {}
impl PNSObject for MTLRenderPassColorAttachmentDescriptor {}
impl IMTLRenderPassColorAttachmentDescriptor for MTLRenderPassColorAttachmentDescriptor {}
pub trait IMTLRenderPassColorAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn clearColor(&self) -> MTLClearColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearColor)
    }
    unsafe fn setClearColor_(&self, clearColor: MTLClearColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClearColor : clearColor)
    }
}
pub type MTLMultisampleDepthResolveFilter = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPassDepthAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLRenderPassDepthAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPassDepthAttachmentDescriptor {}
impl MTLRenderPassDepthAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPassDepthAttachmentDescriptor").unwrap(), alloc) })
    }
}
impl IMTLRenderPassAttachmentDescriptor for MTLRenderPassDepthAttachmentDescriptor {}
impl PNSCopying for MTLRenderPassDepthAttachmentDescriptor {}
impl std::convert::TryFrom<MTLRenderPassAttachmentDescriptor>
    for MTLRenderPassDepthAttachmentDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLRenderPassAttachmentDescriptor,
    ) -> Result<MTLRenderPassDepthAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPassDepthAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPassDepthAttachmentDescriptor(parent.0))
        } else {
            Err ("This MTLRenderPassAttachmentDescriptor cannot be downcasted to MTLRenderPassDepthAttachmentDescriptor" ,)
        }
    }
}
impl INSObject for MTLRenderPassDepthAttachmentDescriptor {}
impl PNSObject for MTLRenderPassDepthAttachmentDescriptor {}
impl IMTLRenderPassDepthAttachmentDescriptor for MTLRenderPassDepthAttachmentDescriptor {}
pub trait IMTLRenderPassDepthAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn clearDepth(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearDepth)
    }
    unsafe fn setClearDepth_(&self, clearDepth: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClearDepth : clearDepth)
    }
    unsafe fn depthResolveFilter(&self) -> MTLMultisampleDepthResolveFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthResolveFilter)
    }
    unsafe fn setDepthResolveFilter_(&self, depthResolveFilter: MTLMultisampleDepthResolveFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthResolveFilter : depthResolveFilter)
    }
}
pub type MTLMultisampleStencilResolveFilter = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPassStencilAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLRenderPassStencilAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPassStencilAttachmentDescriptor {}
impl MTLRenderPassStencilAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPassStencilAttachmentDescriptor").unwrap(), alloc) })
    }
}
impl IMTLRenderPassAttachmentDescriptor for MTLRenderPassStencilAttachmentDescriptor {}
impl PNSCopying for MTLRenderPassStencilAttachmentDescriptor {}
impl std::convert::TryFrom<MTLRenderPassAttachmentDescriptor>
    for MTLRenderPassStencilAttachmentDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLRenderPassAttachmentDescriptor,
    ) -> Result<MTLRenderPassStencilAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPassStencilAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPassStencilAttachmentDescriptor(parent.0))
        } else {
            Err ("This MTLRenderPassAttachmentDescriptor cannot be downcasted to MTLRenderPassStencilAttachmentDescriptor" ,)
        }
    }
}
impl INSObject for MTLRenderPassStencilAttachmentDescriptor {}
impl PNSObject for MTLRenderPassStencilAttachmentDescriptor {}
impl IMTLRenderPassStencilAttachmentDescriptor for MTLRenderPassStencilAttachmentDescriptor {}
pub trait IMTLRenderPassStencilAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn clearStencil(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearStencil)
    }
    unsafe fn setClearStencil_(&self, clearStencil: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClearStencil : clearStencil)
    }
    unsafe fn stencilResolveFilter(&self) -> MTLMultisampleStencilResolveFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stencilResolveFilter)
    }
    unsafe fn setStencilResolveFilter_(
        &self,
        stencilResolveFilter: MTLMultisampleStencilResolveFilter,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilResolveFilter : stencilResolveFilter)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPassColorAttachmentDescriptorArray(pub id);
impl std::ops::Deref for MTLRenderPassColorAttachmentDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPassColorAttachmentDescriptorArray {}
impl MTLRenderPassColorAttachmentDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPassColorAttachmentDescriptorArray").unwrap(), alloc) })
    }
}
impl INSObject for MTLRenderPassColorAttachmentDescriptorArray {}
impl PNSObject for MTLRenderPassColorAttachmentDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLRenderPassColorAttachmentDescriptorArray {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLRenderPassColorAttachmentDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPassColorAttachmentDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPassColorAttachmentDescriptorArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRenderPassColorAttachmentDescriptorArray")
        }
    }
}
impl IMTLRenderPassColorAttachmentDescriptorArray for MTLRenderPassColorAttachmentDescriptorArray {}
pub trait IMTLRenderPassColorAttachmentDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(
        &self,
        attachmentIndex: NSUInteger,
    ) -> MTLRenderPassColorAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : attachmentIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attachment: MTLRenderPassColorAttachmentDescriptor,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attachment, atIndexedSubscript : attachmentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPassSampleBufferAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLRenderPassSampleBufferAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPassSampleBufferAttachmentDescriptor {}
impl MTLRenderPassSampleBufferAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPassSampleBufferAttachmentDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLRenderPassSampleBufferAttachmentDescriptor {}
impl INSObject for MTLRenderPassSampleBufferAttachmentDescriptor {}
impl PNSObject for MTLRenderPassSampleBufferAttachmentDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLRenderPassSampleBufferAttachmentDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLRenderPassSampleBufferAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPassSampleBufferAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPassSampleBufferAttachmentDescriptor(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MTLRenderPassSampleBufferAttachmentDescriptor" ,)
        }
    }
}
impl IMTLRenderPassSampleBufferAttachmentDescriptor
    for MTLRenderPassSampleBufferAttachmentDescriptor
{
}
pub trait IMTLRenderPassSampleBufferAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn sampleBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBuffer)
    }
    unsafe fn setSampleBuffer_(&self, sampleBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleBuffer : sampleBuffer)
    }
    unsafe fn startOfVertexSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startOfVertexSampleIndex)
    }
    unsafe fn setStartOfVertexSampleIndex_(&self, startOfVertexSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartOfVertexSampleIndex : startOfVertexSampleIndex)
    }
    unsafe fn endOfVertexSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endOfVertexSampleIndex)
    }
    unsafe fn setEndOfVertexSampleIndex_(&self, endOfVertexSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndOfVertexSampleIndex : endOfVertexSampleIndex)
    }
    unsafe fn startOfFragmentSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startOfFragmentSampleIndex)
    }
    unsafe fn setStartOfFragmentSampleIndex_(&self, startOfFragmentSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartOfFragmentSampleIndex : startOfFragmentSampleIndex)
    }
    unsafe fn endOfFragmentSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endOfFragmentSampleIndex)
    }
    unsafe fn setEndOfFragmentSampleIndex_(&self, endOfFragmentSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndOfFragmentSampleIndex : endOfFragmentSampleIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPassSampleBufferAttachmentDescriptorArray(pub id);
impl std::ops::Deref for MTLRenderPassSampleBufferAttachmentDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPassSampleBufferAttachmentDescriptorArray {}
impl MTLRenderPassSampleBufferAttachmentDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPassSampleBufferAttachmentDescriptorArray").unwrap(), alloc)
        })
    }
}
impl INSObject for MTLRenderPassSampleBufferAttachmentDescriptorArray {}
impl PNSObject for MTLRenderPassSampleBufferAttachmentDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLRenderPassSampleBufferAttachmentDescriptorArray {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLRenderPassSampleBufferAttachmentDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPassSampleBufferAttachmentDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPassSampleBufferAttachmentDescriptorArray(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MTLRenderPassSampleBufferAttachmentDescriptorArray" ,)
        }
    }
}
impl IMTLRenderPassSampleBufferAttachmentDescriptorArray
    for MTLRenderPassSampleBufferAttachmentDescriptorArray
{
}
pub trait IMTLRenderPassSampleBufferAttachmentDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(
        &self,
        attachmentIndex: NSUInteger,
    ) -> MTLRenderPassSampleBufferAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : attachmentIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attachment: MTLRenderPassSampleBufferAttachmentDescriptor,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attachment, atIndexedSubscript : attachmentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPassDescriptor(pub id);
impl std::ops::Deref for MTLRenderPassDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPassDescriptor {}
impl MTLRenderPassDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPassDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLRenderPassDescriptor {}
impl INSObject for MTLRenderPassDescriptor {}
impl PNSObject for MTLRenderPassDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLRenderPassDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLRenderPassDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPassDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLRenderPassDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRenderPassDescriptor")
        }
    }
}
impl IMTLRenderPassDescriptor for MTLRenderPassDescriptor {}
pub trait IMTLRenderPassDescriptor: Sized + std::ops::Deref {
    unsafe fn setSamplePositions_count_(
        &self,
        positions: *const MTLSamplePosition,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplePositions : positions, count : count)
    }
    unsafe fn getSamplePositions_count_(
        &self,
        positions: *mut MTLSamplePosition,
        count: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSamplePositions : positions, count : count)
    }
    unsafe fn colorAttachments(&self) -> MTLRenderPassColorAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachments)
    }
    unsafe fn depthAttachment(&self) -> MTLRenderPassDepthAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthAttachment)
    }
    unsafe fn setDepthAttachment_(&self, depthAttachment: MTLRenderPassDepthAttachmentDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthAttachment : depthAttachment)
    }
    unsafe fn stencilAttachment(&self) -> MTLRenderPassStencilAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stencilAttachment)
    }
    unsafe fn setStencilAttachment_(
        &self,
        stencilAttachment: MTLRenderPassStencilAttachmentDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilAttachment : stencilAttachment)
    }
    unsafe fn visibilityResultBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibilityResultBuffer)
    }
    unsafe fn setVisibilityResultBuffer_(&self, visibilityResultBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibilityResultBuffer : visibilityResultBuffer)
    }
    unsafe fn renderTargetArrayLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderTargetArrayLength)
    }
    unsafe fn setRenderTargetArrayLength_(&self, renderTargetArrayLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderTargetArrayLength : renderTargetArrayLength)
    }
    unsafe fn imageblockSampleLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageblockSampleLength)
    }
    unsafe fn setImageblockSampleLength_(&self, imageblockSampleLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageblockSampleLength : imageblockSampleLength)
    }
    unsafe fn threadgroupMemoryLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadgroupMemoryLength)
    }
    unsafe fn setThreadgroupMemoryLength_(&self, threadgroupMemoryLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadgroupMemoryLength : threadgroupMemoryLength)
    }
    unsafe fn tileWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileWidth)
    }
    unsafe fn setTileWidth_(&self, tileWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileWidth : tileWidth)
    }
    unsafe fn tileHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileHeight)
    }
    unsafe fn setTileHeight_(&self, tileHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileHeight : tileHeight)
    }
    unsafe fn defaultRasterSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultRasterSampleCount)
    }
    unsafe fn setDefaultRasterSampleCount_(&self, defaultRasterSampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultRasterSampleCount : defaultRasterSampleCount)
    }
    unsafe fn renderTargetWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderTargetWidth)
    }
    unsafe fn setRenderTargetWidth_(&self, renderTargetWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderTargetWidth : renderTargetWidth)
    }
    unsafe fn renderTargetHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderTargetHeight)
    }
    unsafe fn setRenderTargetHeight_(&self, renderTargetHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderTargetHeight : renderTargetHeight)
    }
    unsafe fn rasterizationRateMap(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rasterizationRateMap)
    }
    unsafe fn setRasterizationRateMap_(&self, rasterizationRateMap: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterizationRateMap : rasterizationRateMap)
    }
    unsafe fn sampleBufferAttachments(&self) -> MTLRenderPassSampleBufferAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBufferAttachments)
    }
    unsafe fn visibilityResultType(&self) -> MTLVisibilityResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibilityResultType)
    }
    unsafe fn setVisibilityResultType_(&self, visibilityResultType: MTLVisibilityResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibilityResultType : visibilityResultType)
    }
    unsafe fn supportColorAttachmentMapping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportColorAttachmentMapping)
    }
    unsafe fn setSupportColorAttachmentMapping_(&self, supportColorAttachmentMapping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportColorAttachmentMapping : supportColorAttachmentMapping)
    }
    unsafe fn renderPassDescriptor() -> MTLRenderPassDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPassDescriptor").unwrap(), renderPassDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLBlitPassSampleBufferAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLBlitPassSampleBufferAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLBlitPassSampleBufferAttachmentDescriptor {}
impl MTLBlitPassSampleBufferAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLBlitPassSampleBufferAttachmentDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLBlitPassSampleBufferAttachmentDescriptor {}
impl INSObject for MTLBlitPassSampleBufferAttachmentDescriptor {}
impl PNSObject for MTLBlitPassSampleBufferAttachmentDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLBlitPassSampleBufferAttachmentDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLBlitPassSampleBufferAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLBlitPassSampleBufferAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLBlitPassSampleBufferAttachmentDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLBlitPassSampleBufferAttachmentDescriptor")
        }
    }
}
impl IMTLBlitPassSampleBufferAttachmentDescriptor for MTLBlitPassSampleBufferAttachmentDescriptor {}
pub trait IMTLBlitPassSampleBufferAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn sampleBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBuffer)
    }
    unsafe fn setSampleBuffer_(&self, sampleBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleBuffer : sampleBuffer)
    }
    unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startOfEncoderSampleIndex)
    }
    unsafe fn setStartOfEncoderSampleIndex_(&self, startOfEncoderSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartOfEncoderSampleIndex : startOfEncoderSampleIndex)
    }
    unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endOfEncoderSampleIndex)
    }
    unsafe fn setEndOfEncoderSampleIndex_(&self, endOfEncoderSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndOfEncoderSampleIndex : endOfEncoderSampleIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLBlitPassSampleBufferAttachmentDescriptorArray(pub id);
impl std::ops::Deref for MTLBlitPassSampleBufferAttachmentDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLBlitPassSampleBufferAttachmentDescriptorArray {}
impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLBlitPassSampleBufferAttachmentDescriptorArray").unwrap(), alloc)
        })
    }
}
impl INSObject for MTLBlitPassSampleBufferAttachmentDescriptorArray {}
impl PNSObject for MTLBlitPassSampleBufferAttachmentDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLBlitPassSampleBufferAttachmentDescriptorArray {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLBlitPassSampleBufferAttachmentDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLBlitPassSampleBufferAttachmentDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLBlitPassSampleBufferAttachmentDescriptorArray(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MTLBlitPassSampleBufferAttachmentDescriptorArray" ,)
        }
    }
}
impl IMTLBlitPassSampleBufferAttachmentDescriptorArray
    for MTLBlitPassSampleBufferAttachmentDescriptorArray
{
}
pub trait IMTLBlitPassSampleBufferAttachmentDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(
        &self,
        attachmentIndex: NSUInteger,
    ) -> MTLBlitPassSampleBufferAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : attachmentIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attachment: MTLBlitPassSampleBufferAttachmentDescriptor,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attachment, atIndexedSubscript : attachmentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLBlitPassDescriptor(pub id);
impl std::ops::Deref for MTLBlitPassDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLBlitPassDescriptor {}
impl MTLBlitPassDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLBlitPassDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLBlitPassDescriptor {}
impl INSObject for MTLBlitPassDescriptor {}
impl PNSObject for MTLBlitPassDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLBlitPassDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLBlitPassDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLBlitPassDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLBlitPassDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLBlitPassDescriptor")
        }
    }
}
impl IMTLBlitPassDescriptor for MTLBlitPassDescriptor {}
pub trait IMTLBlitPassDescriptor: Sized + std::ops::Deref {
    unsafe fn sampleBufferAttachments(&self) -> MTLBlitPassSampleBufferAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBufferAttachments)
    }
    unsafe fn blitPassDescriptor() -> MTLBlitPassDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLBlitPassDescriptor").unwrap(), blitPassDescriptor)
    }
}
pub type MTLBlitOption = NSUInteger;
pub trait PMTLBlitCommandEncoder: Sized + std::ops::Deref {
    unsafe fn synchronizeResource_(&self, resource: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, synchronizeResource : resource)
    }
    unsafe fn synchronizeTexture_slice_level_(
        &self,
        texture: *mut u64,
        slice: NSUInteger,
        level: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, synchronizeTexture : texture, slice : slice, level : level)
    }
    unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_(
        &self,
        sourceTexture: *mut u64,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        sourceOrigin: MTLOrigin,
        sourceSize: MTLSize,
        destinationTexture: *mut u64,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        destinationOrigin: MTLOrigin,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, sourceSlice : sourceSlice, sourceLevel : sourceLevel, sourceOrigin : sourceOrigin, sourceSize : sourceSize, toTexture : destinationTexture, destinationSlice : destinationSlice, destinationLevel : destinationLevel, destinationOrigin : destinationOrigin)
    }
    unsafe fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_(
        &self,
        sourceBuffer: *mut u64,
        sourceOffset: NSUInteger,
        sourceBytesPerRow: NSUInteger,
        sourceBytesPerImage: NSUInteger,
        sourceSize: MTLSize,
        destinationTexture: *mut u64,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        destinationOrigin: MTLOrigin,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromBuffer : sourceBuffer, sourceOffset : sourceOffset, sourceBytesPerRow : sourceBytesPerRow, sourceBytesPerImage : sourceBytesPerImage, sourceSize : sourceSize, toTexture : destinationTexture, destinationSlice : destinationSlice, destinationLevel : destinationLevel, destinationOrigin : destinationOrigin)
    }
    unsafe fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options_(
        &self,
        sourceBuffer: *mut u64,
        sourceOffset: NSUInteger,
        sourceBytesPerRow: NSUInteger,
        sourceBytesPerImage: NSUInteger,
        sourceSize: MTLSize,
        destinationTexture: *mut u64,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        destinationOrigin: MTLOrigin,
        options: MTLBlitOption,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromBuffer : sourceBuffer, sourceOffset : sourceOffset, sourceBytesPerRow : sourceBytesPerRow, sourceBytesPerImage : sourceBytesPerImage, sourceSize : sourceSize, toTexture : destinationTexture, destinationSlice : destinationSlice, destinationLevel : destinationLevel, destinationOrigin : destinationOrigin, options : options)
    }
    unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_(
        &self,
        sourceTexture: *mut u64,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        sourceOrigin: MTLOrigin,
        sourceSize: MTLSize,
        destinationBuffer: *mut u64,
        destinationOffset: NSUInteger,
        destinationBytesPerRow: NSUInteger,
        destinationBytesPerImage: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, sourceSlice : sourceSlice, sourceLevel : sourceLevel, sourceOrigin : sourceOrigin, sourceSize : sourceSize, toBuffer : destinationBuffer, destinationOffset : destinationOffset, destinationBytesPerRow : destinationBytesPerRow, destinationBytesPerImage : destinationBytesPerImage)
    }
    unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options_(
        &self,
        sourceTexture: *mut u64,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        sourceOrigin: MTLOrigin,
        sourceSize: MTLSize,
        destinationBuffer: *mut u64,
        destinationOffset: NSUInteger,
        destinationBytesPerRow: NSUInteger,
        destinationBytesPerImage: NSUInteger,
        options: MTLBlitOption,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, sourceSlice : sourceSlice, sourceLevel : sourceLevel, sourceOrigin : sourceOrigin, sourceSize : sourceSize, toBuffer : destinationBuffer, destinationOffset : destinationOffset, destinationBytesPerRow : destinationBytesPerRow, destinationBytesPerImage : destinationBytesPerImage, options : options)
    }
    unsafe fn generateMipmapsForTexture_(&self, texture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateMipmapsForTexture : texture)
    }
    unsafe fn fillBuffer_range_value_(&self, buffer: *mut u64, range: NSRange, value: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fillBuffer : buffer, range : range, value : value)
    }
    unsafe fn copyFromTexture_sourceSlice_sourceLevel_toTexture_destinationSlice_destinationLevel_sliceCount_levelCount_(
        &self,
        sourceTexture: *mut u64,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        destinationTexture: *mut u64,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        sliceCount: NSUInteger,
        levelCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, sourceSlice : sourceSlice, sourceLevel : sourceLevel, toTexture : destinationTexture, destinationSlice : destinationSlice, destinationLevel : destinationLevel, sliceCount : sliceCount, levelCount : levelCount)
    }
    unsafe fn copyFromTexture_toTexture_(
        &self,
        sourceTexture: *mut u64,
        destinationTexture: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, toTexture : destinationTexture)
    }
    unsafe fn copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size_(
        &self,
        sourceBuffer: *mut u64,
        sourceOffset: NSUInteger,
        destinationBuffer: *mut u64,
        destinationOffset: NSUInteger,
        size: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromBuffer : sourceBuffer, sourceOffset : sourceOffset, toBuffer : destinationBuffer, destinationOffset : destinationOffset, size : size)
    }
    unsafe fn updateFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateFence : fence)
    }
    unsafe fn waitForFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForFence : fence)
    }
    unsafe fn getTextureAccessCounters_region_mipLevel_slice_resetCounters_countersBuffer_countersBufferOffset_(
        &self,
        texture: *mut u64,
        region: MTLRegion,
        mipLevel: NSUInteger,
        slice: NSUInteger,
        resetCounters: BOOL,
        countersBuffer: *mut u64,
        countersBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getTextureAccessCounters : texture, region : region, mipLevel : mipLevel, slice : slice, resetCounters : resetCounters, countersBuffer : countersBuffer, countersBufferOffset : countersBufferOffset)
    }
    unsafe fn resetTextureAccessCounters_region_mipLevel_slice_(
        &self,
        texture: *mut u64,
        region: MTLRegion,
        mipLevel: NSUInteger,
        slice: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetTextureAccessCounters : texture, region : region, mipLevel : mipLevel, slice : slice)
    }
    unsafe fn optimizeContentsForGPUAccess_(&self, texture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeContentsForGPUAccess : texture)
    }
    unsafe fn optimizeContentsForGPUAccess_slice_level_(
        &self,
        texture: *mut u64,
        slice: NSUInteger,
        level: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeContentsForGPUAccess : texture, slice : slice, level : level)
    }
    unsafe fn optimizeContentsForCPUAccess_(&self, texture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeContentsForCPUAccess : texture)
    }
    unsafe fn optimizeContentsForCPUAccess_slice_level_(
        &self,
        texture: *mut u64,
        slice: NSUInteger,
        level: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeContentsForCPUAccess : texture, slice : slice, level : level)
    }
    unsafe fn resetCommandsInBuffer_withRange_(&self, buffer: *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetCommandsInBuffer : buffer, withRange : range)
    }
    unsafe fn copyIndirectCommandBuffer_sourceRange_destination_destinationIndex_(
        &self,
        source: *mut u64,
        sourceRange: NSRange,
        destination: *mut u64,
        destinationIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyIndirectCommandBuffer : source, sourceRange : sourceRange, destination : destination, destinationIndex : destinationIndex)
    }
    unsafe fn optimizeIndirectCommandBuffer_withRange_(
        &self,
        indirectCommandBuffer: *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeIndirectCommandBuffer : indirectCommandBuffer, withRange : range)
    }
    unsafe fn sampleCountersInBuffer_atSampleIndex_withBarrier_(
        &self,
        sampleBuffer: *mut u64,
        sampleIndex: NSUInteger,
        barrier: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleCountersInBuffer : sampleBuffer, atSampleIndex : sampleIndex, withBarrier : barrier)
    }
    unsafe fn resolveCounters_inRange_destinationBuffer_destinationOffset_(
        &self,
        sampleBuffer: *mut u64,
        range: NSRange,
        destinationBuffer: *mut u64,
        destinationOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveCounters : sampleBuffer, inRange : range, destinationBuffer : destinationBuffer, destinationOffset : destinationOffset)
    }
    unsafe fn copyFromTensor_sourceOrigin_sourceDimensions_toTensor_destinationOrigin_destinationDimensions_(
        &self,
        sourceTensor: *mut u64,
        sourceOrigin: MTLTensorExtents,
        sourceDimensions: MTLTensorExtents,
        destinationTensor: *mut u64,
        destinationOrigin: MTLTensorExtents,
        destinationDimensions: MTLTensorExtents,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTensor : sourceTensor, sourceOrigin : sourceOrigin, sourceDimensions : sourceDimensions, toTensor : destinationTensor, destinationOrigin : destinationOrigin, destinationDimensions : destinationDimensions)
    }
}
pub type MTLCommandBufferStatus = NSUInteger;
pub type MTLCommandBufferError = NSUInteger;
pub type MTLCommandBufferErrorOption = NSUInteger;
pub type MTLCommandEncoderErrorState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCommandBufferDescriptor(pub id);
impl std::ops::Deref for MTLCommandBufferDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLCommandBufferDescriptor {}
impl MTLCommandBufferDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLCommandBufferDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLCommandBufferDescriptor {}
impl INSObject for MTLCommandBufferDescriptor {}
impl PNSObject for MTLCommandBufferDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLCommandBufferDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLCommandBufferDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLCommandBufferDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLCommandBufferDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLCommandBufferDescriptor")
        }
    }
}
impl IMTLCommandBufferDescriptor for MTLCommandBufferDescriptor {}
pub trait IMTLCommandBufferDescriptor: Sized + std::ops::Deref {
    unsafe fn retainedReferences(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retainedReferences)
    }
    unsafe fn setRetainedReferences_(&self, retainedReferences: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRetainedReferences : retainedReferences)
    }
    unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorOptions)
    }
    unsafe fn setErrorOptions_(&self, errorOptions: MTLCommandBufferErrorOption)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setErrorOptions : errorOptions)
    }
    unsafe fn logState(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, logState)
    }
    unsafe fn setLogState_(&self, logState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLogState : logState)
    }
}
pub trait PMTLCommandBufferEncoderInfo: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn debugSignposts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, debugSignposts)
    }
    unsafe fn errorState(&self) -> MTLCommandEncoderErrorState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorState)
    }
}
pub type MTLDispatchType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLComputePassSampleBufferAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLComputePassSampleBufferAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLComputePassSampleBufferAttachmentDescriptor {}
impl MTLComputePassSampleBufferAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLComputePassSampleBufferAttachmentDescriptor").unwrap(), alloc)
        })
    }
}
impl PNSCopying for MTLComputePassSampleBufferAttachmentDescriptor {}
impl INSObject for MTLComputePassSampleBufferAttachmentDescriptor {}
impl PNSObject for MTLComputePassSampleBufferAttachmentDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLComputePassSampleBufferAttachmentDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLComputePassSampleBufferAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLComputePassSampleBufferAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLComputePassSampleBufferAttachmentDescriptor(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MTLComputePassSampleBufferAttachmentDescriptor" ,)
        }
    }
}
impl IMTLComputePassSampleBufferAttachmentDescriptor
    for MTLComputePassSampleBufferAttachmentDescriptor
{
}
pub trait IMTLComputePassSampleBufferAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn sampleBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBuffer)
    }
    unsafe fn setSampleBuffer_(&self, sampleBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleBuffer : sampleBuffer)
    }
    unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startOfEncoderSampleIndex)
    }
    unsafe fn setStartOfEncoderSampleIndex_(&self, startOfEncoderSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartOfEncoderSampleIndex : startOfEncoderSampleIndex)
    }
    unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endOfEncoderSampleIndex)
    }
    unsafe fn setEndOfEncoderSampleIndex_(&self, endOfEncoderSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndOfEncoderSampleIndex : endOfEncoderSampleIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLComputePassSampleBufferAttachmentDescriptorArray(pub id);
impl std::ops::Deref for MTLComputePassSampleBufferAttachmentDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLComputePassSampleBufferAttachmentDescriptorArray {}
impl MTLComputePassSampleBufferAttachmentDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLComputePassSampleBufferAttachmentDescriptorArray").unwrap(), alloc)
        })
    }
}
impl INSObject for MTLComputePassSampleBufferAttachmentDescriptorArray {}
impl PNSObject for MTLComputePassSampleBufferAttachmentDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLComputePassSampleBufferAttachmentDescriptorArray {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLComputePassSampleBufferAttachmentDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLComputePassSampleBufferAttachmentDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLComputePassSampleBufferAttachmentDescriptorArray(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to MTLComputePassSampleBufferAttachmentDescriptorArray" ,)
        }
    }
}
impl IMTLComputePassSampleBufferAttachmentDescriptorArray
    for MTLComputePassSampleBufferAttachmentDescriptorArray
{
}
pub trait IMTLComputePassSampleBufferAttachmentDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(
        &self,
        attachmentIndex: NSUInteger,
    ) -> MTLComputePassSampleBufferAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : attachmentIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attachment: MTLComputePassSampleBufferAttachmentDescriptor,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attachment, atIndexedSubscript : attachmentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLComputePassDescriptor(pub id);
impl std::ops::Deref for MTLComputePassDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLComputePassDescriptor {}
impl MTLComputePassDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLComputePassDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLComputePassDescriptor {}
impl INSObject for MTLComputePassDescriptor {}
impl PNSObject for MTLComputePassDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLComputePassDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLComputePassDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLComputePassDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLComputePassDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLComputePassDescriptor")
        }
    }
}
impl IMTLComputePassDescriptor for MTLComputePassDescriptor {}
pub trait IMTLComputePassDescriptor: Sized + std::ops::Deref {
    unsafe fn dispatchType(&self) -> MTLDispatchType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dispatchType)
    }
    unsafe fn setDispatchType_(&self, dispatchType: MTLDispatchType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDispatchType : dispatchType)
    }
    unsafe fn sampleBufferAttachments(&self) -> MTLComputePassSampleBufferAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBufferAttachments)
    }
    unsafe fn computePassDescriptor() -> MTLComputePassDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLComputePassDescriptor").unwrap(), computePassDescriptor)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLDispatchThreadgroupsIndirectArguments {
    pub threadgroupsPerGrid: [u32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLDispatchThreadsIndirectArguments {
    pub threadsPerGrid: [u32; 3usize],
    pub threadsPerThreadgroup: [u32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLStageInRegionIndirectArguments {
    pub stageInOrigin: [u32; 3usize],
    pub stageInSize: [u32; 3usize],
}
pub trait PMTLComputeCommandEncoder: Sized + std::ops::Deref {
    unsafe fn setComputePipelineState_(&self, state: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputePipelineState : state)
    }
    unsafe fn setBytes_length_atIndex_(
        &self,
        bytes: *const ::std::os::raw::c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBytes : bytes, length : length, atIndex : index)
    }
    unsafe fn setBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setBufferOffset_atIndex_(&self, offset: NSUInteger, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBufferOffset : offset, atIndex : index)
    }
    unsafe fn setBuffers_offsets_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBuffers : buffers, offsets : offsets, withRange : range)
    }
    unsafe fn setBuffer_offset_attributeStride_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        stride: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBuffer : buffer, offset : offset, attributeStride : stride, atIndex : index)
    }
    unsafe fn setBuffers_offsets_attributeStrides_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        strides: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBuffers : buffers, offsets : offsets, attributeStrides : strides, withRange : range)
    }
    unsafe fn setBufferOffset_attributeStride_atIndex_(
        &self,
        offset: NSUInteger,
        stride: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBufferOffset : offset, attributeStride : stride, atIndex : index)
    }
    unsafe fn setBytes_length_attributeStride_atIndex_(
        &self,
        bytes: *const ::std::os::raw::c_void,
        length: NSUInteger,
        stride: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBytes : bytes, length : length, attributeStride : stride, atIndex : index)
    }
    unsafe fn setVisibleFunctionTable_atBufferIndex_(
        &self,
        visibleFunctionTable: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibleFunctionTable : visibleFunctionTable, atBufferIndex : bufferIndex)
    }
    unsafe fn setVisibleFunctionTables_withBufferRange_(
        &self,
        visibleFunctionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibleFunctionTables : visibleFunctionTables, withBufferRange : range)
    }
    unsafe fn setIntersectionFunctionTable_atBufferIndex_(
        &self,
        intersectionFunctionTable: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntersectionFunctionTable : intersectionFunctionTable, atBufferIndex : bufferIndex)
    }
    unsafe fn setIntersectionFunctionTables_withBufferRange_(
        &self,
        intersectionFunctionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntersectionFunctionTables : intersectionFunctionTables, withBufferRange : range)
    }
    unsafe fn setAccelerationStructure_atBufferIndex_(
        &self,
        accelerationStructure: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccelerationStructure : accelerationStructure, atBufferIndex : bufferIndex)
    }
    unsafe fn setTexture_atIndex_(&self, texture: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTexture : texture, atIndex : index)
    }
    unsafe fn setTextures_withRange_(&self, textures: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextures : textures, withRange : range)
    }
    unsafe fn setSamplerState_atIndex_(&self, sampler: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplerState : sampler, atIndex : index)
    }
    unsafe fn setSamplerStates_withRange_(&self, samplers: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplerStates : samplers, withRange : range)
    }
    unsafe fn setSamplerState_lodMinClamp_lodMaxClamp_atIndex_(
        &self,
        sampler: *mut u64,
        lodMinClamp: f32,
        lodMaxClamp: f32,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplerState : sampler, lodMinClamp : lodMinClamp, lodMaxClamp : lodMaxClamp, atIndex : index)
    }
    unsafe fn setSamplerStates_lodMinClamps_lodMaxClamps_withRange_(
        &self,
        samplers: *const *mut u64,
        lodMinClamps: *const f32,
        lodMaxClamps: *const f32,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplerStates : samplers, lodMinClamps : lodMinClamps, lodMaxClamps : lodMaxClamps, withRange : range)
    }
    unsafe fn setThreadgroupMemoryLength_atIndex_(&self, length: NSUInteger, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadgroupMemoryLength : length, atIndex : index)
    }
    unsafe fn setImageblockWidth_height_(&self, width: NSUInteger, height: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageblockWidth : width, height : height)
    }
    unsafe fn setStageInRegion_(&self, region: MTLRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStageInRegion : region)
    }
    unsafe fn setStageInRegionWithIndirectBuffer_indirectBufferOffset_(
        &self,
        indirectBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStageInRegionWithIndirectBuffer : indirectBuffer, indirectBufferOffset : indirectBufferOffset)
    }
    unsafe fn dispatchThreadgroups_threadsPerThreadgroup_(
        &self,
        threadgroupsPerGrid: MTLSize,
        threadsPerThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchThreadgroups : threadgroupsPerGrid, threadsPerThreadgroup : threadsPerThreadgroup)
    }
    unsafe fn dispatchThreadgroupsWithIndirectBuffer_indirectBufferOffset_threadsPerThreadgroup_(
        &self,
        indirectBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
        threadsPerThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchThreadgroupsWithIndirectBuffer : indirectBuffer, indirectBufferOffset : indirectBufferOffset, threadsPerThreadgroup : threadsPerThreadgroup)
    }
    unsafe fn dispatchThreads_threadsPerThreadgroup_(
        &self,
        threadsPerGrid: MTLSize,
        threadsPerThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchThreads : threadsPerGrid, threadsPerThreadgroup : threadsPerThreadgroup)
    }
    unsafe fn updateFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateFence : fence)
    }
    unsafe fn waitForFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForFence : fence)
    }
    unsafe fn useResource_usage_(&self, resource: *mut u64, usage: MTLResourceUsage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResource : resource, usage : usage)
    }
    unsafe fn useResources_count_usage_(
        &self,
        resources: *const *mut u64,
        count: NSUInteger,
        usage: MTLResourceUsage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResources : resources, count : count, usage : usage)
    }
    unsafe fn useHeap_(&self, heap: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useHeap : heap)
    }
    unsafe fn useHeaps_count_(&self, heaps: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useHeaps : heaps, count : count)
    }
    unsafe fn executeCommandsInBuffer_withRange_(
        &self,
        indirectCommandBuffer: *mut u64,
        executionRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeCommandsInBuffer : indirectCommandBuffer, withRange : executionRange)
    }
    unsafe fn executeCommandsInBuffer_indirectBuffer_indirectBufferOffset_(
        &self,
        indirectCommandbuffer: *mut u64,
        indirectRangeBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeCommandsInBuffer : indirectCommandbuffer, indirectBuffer : indirectRangeBuffer, indirectBufferOffset : indirectBufferOffset)
    }
    unsafe fn memoryBarrierWithScope_(&self, scope: MTLBarrierScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, memoryBarrierWithScope : scope)
    }
    unsafe fn memoryBarrierWithResources_count_(
        &self,
        resources: *const *mut u64,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, memoryBarrierWithResources : resources, count : count)
    }
    unsafe fn sampleCountersInBuffer_atSampleIndex_withBarrier_(
        &self,
        sampleBuffer: *mut u64,
        sampleIndex: NSUInteger,
        barrier: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleCountersInBuffer : sampleBuffer, atSampleIndex : sampleIndex, withBarrier : barrier)
    }
    unsafe fn dispatchType(&self) -> MTLDispatchType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dispatchType)
    }
}
pub trait PMTLCommandQueue: Sized + std::ops::Deref {
    unsafe fn commandBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandBuffer)
    }
    unsafe fn commandBufferWithDescriptor_(
        &self,
        descriptor: MTLCommandBufferDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commandBufferWithDescriptor : descriptor)
    }
    unsafe fn commandBufferWithUnretainedReferences(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandBufferWithUnretainedReferences)
    }
    unsafe fn insertDebugCaptureBoundary(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insertDebugCaptureBoundary)
    }
    unsafe fn addResidencySet_(&self, residencySet: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addResidencySet : residencySet)
    }
    unsafe fn addResidencySets_count_(&self, residencySets: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addResidencySets : residencySets, count : count)
    }
    unsafe fn removeResidencySet_(&self, residencySet: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeResidencySet : residencySet)
    }
    unsafe fn removeResidencySets_count_(&self, residencySets: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeResidencySets : residencySets, count : count)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCommandQueueDescriptor(pub id);
impl std::ops::Deref for MTLCommandQueueDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLCommandQueueDescriptor {}
impl MTLCommandQueueDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLCommandQueueDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLCommandQueueDescriptor {}
impl INSObject for MTLCommandQueueDescriptor {}
impl PNSObject for MTLCommandQueueDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLCommandQueueDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLCommandQueueDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLCommandQueueDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLCommandQueueDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLCommandQueueDescriptor")
        }
    }
}
impl IMTLCommandQueueDescriptor for MTLCommandQueueDescriptor {}
pub trait IMTLCommandQueueDescriptor: Sized + std::ops::Deref {
    unsafe fn maxCommandBufferCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxCommandBufferCount)
    }
    unsafe fn setMaxCommandBufferCount_(&self, maxCommandBufferCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxCommandBufferCount : maxCommandBufferCount)
    }
    unsafe fn logState(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, logState)
    }
    unsafe fn setLogState_(&self, logState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLogState : logState)
    }
}
pub type MTLCompareFunction = NSUInteger;
pub type MTLStencilOperation = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLStencilDescriptor(pub id);
impl std::ops::Deref for MTLStencilDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLStencilDescriptor {}
impl MTLStencilDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLStencilDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLStencilDescriptor {}
impl INSObject for MTLStencilDescriptor {}
impl PNSObject for MTLStencilDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLStencilDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLStencilDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLStencilDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLStencilDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLStencilDescriptor")
        }
    }
}
impl IMTLStencilDescriptor for MTLStencilDescriptor {}
pub trait IMTLStencilDescriptor: Sized + std::ops::Deref {
    unsafe fn stencilCompareFunction(&self) -> MTLCompareFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stencilCompareFunction)
    }
    unsafe fn setStencilCompareFunction_(&self, stencilCompareFunction: MTLCompareFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilCompareFunction : stencilCompareFunction)
    }
    unsafe fn stencilFailureOperation(&self) -> MTLStencilOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stencilFailureOperation)
    }
    unsafe fn setStencilFailureOperation_(&self, stencilFailureOperation: MTLStencilOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilFailureOperation : stencilFailureOperation)
    }
    unsafe fn depthFailureOperation(&self) -> MTLStencilOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthFailureOperation)
    }
    unsafe fn setDepthFailureOperation_(&self, depthFailureOperation: MTLStencilOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthFailureOperation : depthFailureOperation)
    }
    unsafe fn depthStencilPassOperation(&self) -> MTLStencilOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthStencilPassOperation)
    }
    unsafe fn setDepthStencilPassOperation_(&self, depthStencilPassOperation: MTLStencilOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStencilPassOperation : depthStencilPassOperation)
    }
    unsafe fn readMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readMask)
    }
    unsafe fn setReadMask_(&self, readMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadMask : readMask)
    }
    unsafe fn writeMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writeMask)
    }
    unsafe fn setWriteMask_(&self, writeMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWriteMask : writeMask)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLDepthStencilDescriptor(pub id);
impl std::ops::Deref for MTLDepthStencilDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLDepthStencilDescriptor {}
impl MTLDepthStencilDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLDepthStencilDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLDepthStencilDescriptor {}
impl INSObject for MTLDepthStencilDescriptor {}
impl PNSObject for MTLDepthStencilDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLDepthStencilDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLDepthStencilDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLDepthStencilDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLDepthStencilDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLDepthStencilDescriptor")
        }
    }
}
impl IMTLDepthStencilDescriptor for MTLDepthStencilDescriptor {}
pub trait IMTLDepthStencilDescriptor: Sized + std::ops::Deref {
    unsafe fn depthCompareFunction(&self) -> MTLCompareFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthCompareFunction)
    }
    unsafe fn setDepthCompareFunction_(&self, depthCompareFunction: MTLCompareFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthCompareFunction : depthCompareFunction)
    }
    unsafe fn isDepthWriteEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDepthWriteEnabled)
    }
    unsafe fn setDepthWriteEnabled_(&self, depthWriteEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthWriteEnabled : depthWriteEnabled)
    }
    unsafe fn frontFaceStencil(&self) -> MTLStencilDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frontFaceStencil)
    }
    unsafe fn setFrontFaceStencil_(&self, frontFaceStencil: MTLStencilDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrontFaceStencil : frontFaceStencil)
    }
    unsafe fn backFaceStencil(&self) -> MTLStencilDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backFaceStencil)
    }
    unsafe fn setBackFaceStencil_(&self, backFaceStencil: MTLStencilDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackFaceStencil : backFaceStencil)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub trait PMTLDepthStencilState: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
}
pub type MTLDrawablePresentedHandler = *mut ::std::os::raw::c_void;
pub trait PMTLDrawable: Sized + std::ops::Deref {
    unsafe fn present(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, present)
    }
    unsafe fn presentAtTime_(&self, presentationTime: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentAtTime : presentationTime)
    }
    unsafe fn presentAfterMinimumDuration_(&self, duration: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentAfterMinimumDuration : duration)
    }
    unsafe fn addPresentedHandler_(&self, block: MTLDrawablePresentedHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPresentedHandler : block)
    }
    unsafe fn presentedTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentedTime)
    }
    unsafe fn drawableID(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawableID)
    }
}
pub type MTLVertexFormat = NSUInteger;
pub type MTLVertexStepFunction = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLVertexBufferLayoutDescriptor(pub id);
impl std::ops::Deref for MTLVertexBufferLayoutDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLVertexBufferLayoutDescriptor {}
impl MTLVertexBufferLayoutDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLVertexBufferLayoutDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLVertexBufferLayoutDescriptor {}
impl INSObject for MTLVertexBufferLayoutDescriptor {}
impl PNSObject for MTLVertexBufferLayoutDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLVertexBufferLayoutDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLVertexBufferLayoutDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLVertexBufferLayoutDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLVertexBufferLayoutDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLVertexBufferLayoutDescriptor")
        }
    }
}
impl IMTLVertexBufferLayoutDescriptor for MTLVertexBufferLayoutDescriptor {}
pub trait IMTLVertexBufferLayoutDescriptor: Sized + std::ops::Deref {
    unsafe fn stride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stride)
    }
    unsafe fn setStride_(&self, stride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStride : stride)
    }
    unsafe fn stepFunction(&self) -> MTLVertexStepFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stepFunction)
    }
    unsafe fn setStepFunction_(&self, stepFunction: MTLVertexStepFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStepFunction : stepFunction)
    }
    unsafe fn stepRate(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stepRate)
    }
    unsafe fn setStepRate_(&self, stepRate: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStepRate : stepRate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLVertexBufferLayoutDescriptorArray(pub id);
impl std::ops::Deref for MTLVertexBufferLayoutDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLVertexBufferLayoutDescriptorArray {}
impl MTLVertexBufferLayoutDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLVertexBufferLayoutDescriptorArray").unwrap(), alloc) })
    }
}
impl INSObject for MTLVertexBufferLayoutDescriptorArray {}
impl PNSObject for MTLVertexBufferLayoutDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLVertexBufferLayoutDescriptorArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLVertexBufferLayoutDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLVertexBufferLayoutDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLVertexBufferLayoutDescriptorArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLVertexBufferLayoutDescriptorArray")
        }
    }
}
impl IMTLVertexBufferLayoutDescriptorArray for MTLVertexBufferLayoutDescriptorArray {}
pub trait IMTLVertexBufferLayoutDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(&self, index: NSUInteger) -> MTLVertexBufferLayoutDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : index)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        bufferDesc: MTLVertexBufferLayoutDescriptor,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : bufferDesc, atIndexedSubscript : index)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLVertexAttributeDescriptor(pub id);
impl std::ops::Deref for MTLVertexAttributeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLVertexAttributeDescriptor {}
impl MTLVertexAttributeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLVertexAttributeDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLVertexAttributeDescriptor {}
impl INSObject for MTLVertexAttributeDescriptor {}
impl PNSObject for MTLVertexAttributeDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLVertexAttributeDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLVertexAttributeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLVertexAttributeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLVertexAttributeDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLVertexAttributeDescriptor")
        }
    }
}
impl IMTLVertexAttributeDescriptor for MTLVertexAttributeDescriptor {}
pub trait IMTLVertexAttributeDescriptor: Sized + std::ops::Deref {
    unsafe fn format(&self) -> MTLVertexFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn setFormat_(&self, format: MTLVertexFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFormat : format)
    }
    unsafe fn offset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn bufferIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferIndex)
    }
    unsafe fn setBufferIndex_(&self, bufferIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBufferIndex : bufferIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLVertexAttributeDescriptorArray(pub id);
impl std::ops::Deref for MTLVertexAttributeDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLVertexAttributeDescriptorArray {}
impl MTLVertexAttributeDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLVertexAttributeDescriptorArray").unwrap(), alloc) })
    }
}
impl INSObject for MTLVertexAttributeDescriptorArray {}
impl PNSObject for MTLVertexAttributeDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLVertexAttributeDescriptorArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLVertexAttributeDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLVertexAttributeDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLVertexAttributeDescriptorArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLVertexAttributeDescriptorArray")
        }
    }
}
impl IMTLVertexAttributeDescriptorArray for MTLVertexAttributeDescriptorArray {}
pub trait IMTLVertexAttributeDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(&self, index: NSUInteger) -> MTLVertexAttributeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : index)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attributeDesc: MTLVertexAttributeDescriptor,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attributeDesc, atIndexedSubscript : index)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLVertexDescriptor(pub id);
impl std::ops::Deref for MTLVertexDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLVertexDescriptor {}
impl MTLVertexDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLVertexDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLVertexDescriptor {}
impl INSObject for MTLVertexDescriptor {}
impl PNSObject for MTLVertexDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLVertexDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLVertexDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLVertexDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLVertexDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLVertexDescriptor")
        }
    }
}
impl IMTLVertexDescriptor for MTLVertexDescriptor {}
pub trait IMTLVertexDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn layouts(&self) -> MTLVertexBufferLayoutDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layouts)
    }
    unsafe fn attributes(&self) -> MTLVertexAttributeDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn vertexDescriptor() -> MTLVertexDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLVertexDescriptor").unwrap(), vertexDescriptor)
    }
}
pub type MTLAttributeFormat = NSUInteger;
pub type MTLStepFunction = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLBufferLayoutDescriptor(pub id);
impl std::ops::Deref for MTLBufferLayoutDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLBufferLayoutDescriptor {}
impl MTLBufferLayoutDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLBufferLayoutDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLBufferLayoutDescriptor {}
impl INSObject for MTLBufferLayoutDescriptor {}
impl PNSObject for MTLBufferLayoutDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLBufferLayoutDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLBufferLayoutDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLBufferLayoutDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLBufferLayoutDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLBufferLayoutDescriptor")
        }
    }
}
impl IMTLBufferLayoutDescriptor for MTLBufferLayoutDescriptor {}
pub trait IMTLBufferLayoutDescriptor: Sized + std::ops::Deref {
    unsafe fn stride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stride)
    }
    unsafe fn setStride_(&self, stride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStride : stride)
    }
    unsafe fn stepFunction(&self) -> MTLStepFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stepFunction)
    }
    unsafe fn setStepFunction_(&self, stepFunction: MTLStepFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStepFunction : stepFunction)
    }
    unsafe fn stepRate(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stepRate)
    }
    unsafe fn setStepRate_(&self, stepRate: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStepRate : stepRate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLBufferLayoutDescriptorArray(pub id);
impl std::ops::Deref for MTLBufferLayoutDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLBufferLayoutDescriptorArray {}
impl MTLBufferLayoutDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLBufferLayoutDescriptorArray").unwrap(), alloc) })
    }
}
impl INSObject for MTLBufferLayoutDescriptorArray {}
impl PNSObject for MTLBufferLayoutDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLBufferLayoutDescriptorArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLBufferLayoutDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLBufferLayoutDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLBufferLayoutDescriptorArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLBufferLayoutDescriptorArray")
        }
    }
}
impl IMTLBufferLayoutDescriptorArray for MTLBufferLayoutDescriptorArray {}
pub trait IMTLBufferLayoutDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(&self, index: NSUInteger) -> MTLBufferLayoutDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : index)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        bufferDesc: MTLBufferLayoutDescriptor,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : bufferDesc, atIndexedSubscript : index)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAttributeDescriptor(pub id);
impl std::ops::Deref for MTLAttributeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAttributeDescriptor {}
impl MTLAttributeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAttributeDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLAttributeDescriptor {}
impl INSObject for MTLAttributeDescriptor {}
impl PNSObject for MTLAttributeDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLAttributeDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLAttributeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAttributeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLAttributeDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLAttributeDescriptor")
        }
    }
}
impl IMTLAttributeDescriptor for MTLAttributeDescriptor {}
pub trait IMTLAttributeDescriptor: Sized + std::ops::Deref {
    unsafe fn format(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn setFormat_(&self, format: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFormat : format)
    }
    unsafe fn offset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn bufferIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferIndex)
    }
    unsafe fn setBufferIndex_(&self, bufferIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBufferIndex : bufferIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAttributeDescriptorArray(pub id);
impl std::ops::Deref for MTLAttributeDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAttributeDescriptorArray {}
impl MTLAttributeDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAttributeDescriptorArray").unwrap(), alloc) })
    }
}
impl INSObject for MTLAttributeDescriptorArray {}
impl PNSObject for MTLAttributeDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLAttributeDescriptorArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLAttributeDescriptorArray, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAttributeDescriptorArray").unwrap()) };
        if is_kind_of {
            Ok(MTLAttributeDescriptorArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLAttributeDescriptorArray")
        }
    }
}
impl IMTLAttributeDescriptorArray for MTLAttributeDescriptorArray {}
pub trait IMTLAttributeDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(&self, index: NSUInteger) -> MTLAttributeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : index)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attributeDesc: MTLAttributeDescriptor,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attributeDesc, atIndexedSubscript : index)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLStageInputOutputDescriptor(pub id);
impl std::ops::Deref for MTLStageInputOutputDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLStageInputOutputDescriptor {}
impl MTLStageInputOutputDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLStageInputOutputDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLStageInputOutputDescriptor {}
impl INSObject for MTLStageInputOutputDescriptor {}
impl PNSObject for MTLStageInputOutputDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLStageInputOutputDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLStageInputOutputDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLStageInputOutputDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLStageInputOutputDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLStageInputOutputDescriptor")
        }
    }
}
impl IMTLStageInputOutputDescriptor for MTLStageInputOutputDescriptor {}
pub trait IMTLStageInputOutputDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn layouts(&self) -> MTLBufferLayoutDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layouts)
    }
    unsafe fn attributes(&self) -> MTLAttributeDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn setIndexType_(&self, indexType: MTLIndexType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexType : indexType)
    }
    unsafe fn indexBufferIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBufferIndex)
    }
    unsafe fn setIndexBufferIndex_(&self, indexBufferIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBufferIndex : indexBufferIndex)
    }
    unsafe fn stageInputOutputDescriptor() -> MTLStageInputOutputDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLStageInputOutputDescriptor").unwrap(), stageInputOutputDescriptor)
    }
}
pub type MTLMutability = NSUInteger;
pub type MTLShaderValidation = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLPipelineBufferDescriptor(pub id);
impl std::ops::Deref for MTLPipelineBufferDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLPipelineBufferDescriptor {}
impl MTLPipelineBufferDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLPipelineBufferDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLPipelineBufferDescriptor {}
impl INSObject for MTLPipelineBufferDescriptor {}
impl PNSObject for MTLPipelineBufferDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLPipelineBufferDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLPipelineBufferDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLPipelineBufferDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLPipelineBufferDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLPipelineBufferDescriptor")
        }
    }
}
impl IMTLPipelineBufferDescriptor for MTLPipelineBufferDescriptor {}
pub trait IMTLPipelineBufferDescriptor: Sized + std::ops::Deref {
    unsafe fn mutability(&self) -> MTLMutability
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mutability)
    }
    unsafe fn setMutability_(&self, mutability: MTLMutability)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMutability : mutability)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLPipelineBufferDescriptorArray(pub id);
impl std::ops::Deref for MTLPipelineBufferDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLPipelineBufferDescriptorArray {}
impl MTLPipelineBufferDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLPipelineBufferDescriptorArray").unwrap(), alloc) })
    }
}
impl INSObject for MTLPipelineBufferDescriptorArray {}
impl PNSObject for MTLPipelineBufferDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLPipelineBufferDescriptorArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLPipelineBufferDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLPipelineBufferDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLPipelineBufferDescriptorArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLPipelineBufferDescriptorArray")
        }
    }
}
impl IMTLPipelineBufferDescriptorArray for MTLPipelineBufferDescriptorArray {}
pub trait IMTLPipelineBufferDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(
        &self,
        bufferIndex: NSUInteger,
    ) -> MTLPipelineBufferDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : bufferIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        buffer: MTLPipelineBufferDescriptor,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : buffer, atIndexedSubscript : bufferIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLLinkedFunctions(pub id);
impl std::ops::Deref for MTLLinkedFunctions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLLinkedFunctions {}
impl MTLLinkedFunctions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLLinkedFunctions").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLLinkedFunctions {}
impl INSObject for MTLLinkedFunctions {}
impl PNSObject for MTLLinkedFunctions {}
impl std::convert::TryFrom<NSObject> for MTLLinkedFunctions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLLinkedFunctions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLLinkedFunctions").unwrap()) };
        if is_kind_of {
            Ok(MTLLinkedFunctions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLLinkedFunctions")
        }
    }
}
impl IMTLLinkedFunctions for MTLLinkedFunctions {}
pub trait IMTLLinkedFunctions: Sized + std::ops::Deref {
    unsafe fn functions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functions)
    }
    unsafe fn setFunctions_(&self, functions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctions : functions)
    }
    unsafe fn binaryFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, binaryFunctions)
    }
    unsafe fn setBinaryFunctions_(&self, binaryFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBinaryFunctions : binaryFunctions)
    }
    unsafe fn groups(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groups)
    }
    unsafe fn setGroups_(&self, groups: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroups : groups)
    }
    unsafe fn privateFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, privateFunctions)
    }
    unsafe fn setPrivateFunctions_(&self, privateFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrivateFunctions : privateFunctions)
    }
    unsafe fn linkedFunctions() -> MTLLinkedFunctions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLLinkedFunctions").unwrap(), linkedFunctions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLComputePipelineReflection(pub id);
impl std::ops::Deref for MTLComputePipelineReflection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLComputePipelineReflection {}
impl MTLComputePipelineReflection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLComputePipelineReflection").unwrap(), alloc) })
    }
}
impl INSObject for MTLComputePipelineReflection {}
impl PNSObject for MTLComputePipelineReflection {}
impl std::convert::TryFrom<NSObject> for MTLComputePipelineReflection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLComputePipelineReflection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLComputePipelineReflection").unwrap()) };
        if is_kind_of {
            Ok(MTLComputePipelineReflection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLComputePipelineReflection")
        }
    }
}
impl IMTLComputePipelineReflection for MTLComputePipelineReflection {}
pub trait IMTLComputePipelineReflection: Sized + std::ops::Deref {
    unsafe fn bindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bindings)
    }
    unsafe fn arguments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arguments)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLComputePipelineDescriptor(pub id);
impl std::ops::Deref for MTLComputePipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLComputePipelineDescriptor {}
impl MTLComputePipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLComputePipelineDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLComputePipelineDescriptor {}
impl INSObject for MTLComputePipelineDescriptor {}
impl PNSObject for MTLComputePipelineDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLComputePipelineDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLComputePipelineDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLComputePipelineDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLComputePipelineDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLComputePipelineDescriptor")
        }
    }
}
impl IMTLComputePipelineDescriptor for MTLComputePipelineDescriptor {}
pub trait IMTLComputePipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn computeFunction(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, computeFunction)
    }
    unsafe fn setComputeFunction_(&self, computeFunction: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputeFunction : computeFunction)
    }
    unsafe fn threadGroupSizeIsMultipleOfThreadExecutionWidth(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadGroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn setThreadGroupSizeIsMultipleOfThreadExecutionWidth_(
        &self,
        threadGroupSizeIsMultipleOfThreadExecutionWidth: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadGroupSizeIsMultipleOfThreadExecutionWidth : threadGroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerThreadgroup)
    }
    unsafe fn setMaxTotalThreadsPerThreadgroup_(&self, maxTotalThreadsPerThreadgroup: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadsPerThreadgroup : maxTotalThreadsPerThreadgroup)
    }
    unsafe fn stageInputDescriptor(&self) -> MTLStageInputOutputDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stageInputDescriptor)
    }
    unsafe fn setStageInputDescriptor_(&self, stageInputDescriptor: MTLStageInputOutputDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStageInputDescriptor : stageInputDescriptor)
    }
    unsafe fn buffers(&self) -> MTLPipelineBufferDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buffers)
    }
    unsafe fn supportIndirectCommandBuffers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportIndirectCommandBuffers)
    }
    unsafe fn setSupportIndirectCommandBuffers_(&self, supportIndirectCommandBuffers: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportIndirectCommandBuffers : supportIndirectCommandBuffers)
    }
    unsafe fn insertLibraries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insertLibraries)
    }
    unsafe fn setInsertLibraries_(&self, insertLibraries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInsertLibraries : insertLibraries)
    }
    unsafe fn preloadedLibraries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preloadedLibraries)
    }
    unsafe fn setPreloadedLibraries_(&self, preloadedLibraries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreloadedLibraries : preloadedLibraries)
    }
    unsafe fn binaryArchives(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, binaryArchives)
    }
    unsafe fn setBinaryArchives_(&self, binaryArchives: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBinaryArchives : binaryArchives)
    }
    unsafe fn linkedFunctions(&self) -> MTLLinkedFunctions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkedFunctions)
    }
    unsafe fn setLinkedFunctions_(&self, linkedFunctions: MTLLinkedFunctions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinkedFunctions : linkedFunctions)
    }
    unsafe fn supportAddingBinaryFunctions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportAddingBinaryFunctions)
    }
    unsafe fn setSupportAddingBinaryFunctions_(&self, supportAddingBinaryFunctions: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportAddingBinaryFunctions : supportAddingBinaryFunctions)
    }
    unsafe fn maxCallStackDepth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxCallStackDepth)
    }
    unsafe fn setMaxCallStackDepth_(&self, maxCallStackDepth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxCallStackDepth : maxCallStackDepth)
    }
    unsafe fn shaderValidation(&self) -> MTLShaderValidation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shaderValidation)
    }
    unsafe fn setShaderValidation_(&self, shaderValidation: MTLShaderValidation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShaderValidation : shaderValidation)
    }
    unsafe fn requiredThreadsPerThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerThreadgroup)
    }
    unsafe fn setRequiredThreadsPerThreadgroup_(&self, requiredThreadsPerThreadgroup: MTLSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredThreadsPerThreadgroup : requiredThreadsPerThreadgroup)
    }
}
pub trait PMTLComputePipelineState: Sized + std::ops::Deref {
    unsafe fn functionHandleWithName_(&self, name: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, functionHandleWithName : name)
    }
    unsafe fn functionHandleWithBinaryFunction_(&self, function: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, functionHandleWithBinaryFunction : function)
    }
    unsafe fn newComputePipelineStateWithBinaryFunctions_error_(
        &self,
        additionalBinaryFunctions: NSArray,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithBinaryFunctions : additionalBinaryFunctions, error : error)
    }
    unsafe fn imageblockMemoryLengthForDimensions_(
        &self,
        imageblockDimensions: MTLSize,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageblockMemoryLengthForDimensions : imageblockDimensions)
    }
    unsafe fn functionHandleWithFunction_(&self, function: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, functionHandleWithFunction : function)
    }
    unsafe fn newComputePipelineStateWithAdditionalBinaryFunctions_error_(
        &self,
        functions: NSArray,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithAdditionalBinaryFunctions : functions, error : error)
    }
    unsafe fn newVisibleFunctionTableWithDescriptor_(
        &self,
        descriptor: MTLVisibleFunctionTableDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newVisibleFunctionTableWithDescriptor : descriptor)
    }
    unsafe fn newIntersectionFunctionTableWithDescriptor_(
        &self,
        descriptor: MTLIntersectionFunctionTableDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIntersectionFunctionTableWithDescriptor : descriptor)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn reflection(&self) -> MTLComputePipelineReflection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflection)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerThreadgroup)
    }
    unsafe fn threadExecutionWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadExecutionWidth)
    }
    unsafe fn staticThreadgroupMemoryLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, staticThreadgroupMemoryLength)
    }
    unsafe fn supportIndirectCommandBuffers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportIndirectCommandBuffers)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
    unsafe fn shaderValidation(&self) -> MTLShaderValidation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shaderValidation)
    }
    unsafe fn requiredThreadsPerThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerThreadgroup)
    }
}
pub type MTLPrimitiveType = NSUInteger;
pub type MTLVisibilityResultMode = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLScissorRect {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub width: NSUInteger,
    pub height: NSUInteger,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLViewport {
    pub originX: f64,
    pub originY: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}
pub type MTLCullMode = NSUInteger;
pub type MTLWinding = NSUInteger;
pub type MTLDepthClipMode = NSUInteger;
pub type MTLTriangleFillMode = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLDrawPrimitivesIndirectArguments {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub vertexStart: u32,
    pub baseInstance: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLDrawIndexedPrimitivesIndirectArguments {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub indexStart: u32,
    pub baseVertex: i32,
    pub baseInstance: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLVertexAmplificationViewMapping {
    pub viewportArrayIndexOffset: u32,
    pub renderTargetArrayIndexOffset: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLDrawPatchIndirectArguments {
    pub patchCount: u32,
    pub instanceCount: u32,
    pub patchStart: u32,
    pub baseInstance: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLQuadTessellationFactorsHalf {
    pub edgeTessellationFactor: [u16; 4usize],
    pub insideTessellationFactor: [u16; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTriangleTessellationFactorsHalf {
    pub edgeTessellationFactor: [u16; 3usize],
    pub insideTessellationFactor: u16,
}
pub type MTLRenderStages = NSUInteger;
pub trait PMTLRenderCommandEncoder: Sized + std::ops::Deref {
    unsafe fn setRenderPipelineState_(&self, pipelineState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderPipelineState : pipelineState)
    }
    unsafe fn setVertexBytes_length_atIndex_(
        &self,
        bytes: *const ::std::os::raw::c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBytes : bytes, length : length, atIndex : index)
    }
    unsafe fn setVertexBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setVertexBufferOffset_atIndex_(&self, offset: NSUInteger, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBufferOffset : offset, atIndex : index)
    }
    unsafe fn setVertexBuffers_offsets_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffers : buffers, offsets : offsets, withRange : range)
    }
    unsafe fn setVertexBuffer_offset_attributeStride_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        stride: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffer : buffer, offset : offset, attributeStride : stride, atIndex : index)
    }
    unsafe fn setVertexBuffers_offsets_attributeStrides_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        strides: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffers : buffers, offsets : offsets, attributeStrides : strides, withRange : range)
    }
    unsafe fn setVertexBufferOffset_attributeStride_atIndex_(
        &self,
        offset: NSUInteger,
        stride: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBufferOffset : offset, attributeStride : stride, atIndex : index)
    }
    unsafe fn setVertexBytes_length_attributeStride_atIndex_(
        &self,
        bytes: *const ::std::os::raw::c_void,
        length: NSUInteger,
        stride: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBytes : bytes, length : length, attributeStride : stride, atIndex : index)
    }
    unsafe fn setVertexTexture_atIndex_(&self, texture: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexTexture : texture, atIndex : index)
    }
    unsafe fn setVertexTextures_withRange_(&self, textures: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexTextures : textures, withRange : range)
    }
    unsafe fn setVertexSamplerState_atIndex_(&self, sampler: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexSamplerState : sampler, atIndex : index)
    }
    unsafe fn setVertexSamplerStates_withRange_(&self, samplers: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexSamplerStates : samplers, withRange : range)
    }
    unsafe fn setVertexSamplerState_lodMinClamp_lodMaxClamp_atIndex_(
        &self,
        sampler: *mut u64,
        lodMinClamp: f32,
        lodMaxClamp: f32,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexSamplerState : sampler, lodMinClamp : lodMinClamp, lodMaxClamp : lodMaxClamp, atIndex : index)
    }
    unsafe fn setVertexSamplerStates_lodMinClamps_lodMaxClamps_withRange_(
        &self,
        samplers: *const *mut u64,
        lodMinClamps: *const f32,
        lodMaxClamps: *const f32,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexSamplerStates : samplers, lodMinClamps : lodMinClamps, lodMaxClamps : lodMaxClamps, withRange : range)
    }
    unsafe fn setVertexVisibleFunctionTable_atBufferIndex_(
        &self,
        functionTable: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexVisibleFunctionTable : functionTable, atBufferIndex : bufferIndex)
    }
    unsafe fn setVertexVisibleFunctionTables_withBufferRange_(
        &self,
        functionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexVisibleFunctionTables : functionTables, withBufferRange : range)
    }
    unsafe fn setVertexIntersectionFunctionTable_atBufferIndex_(
        &self,
        intersectionFunctionTable: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexIntersectionFunctionTable : intersectionFunctionTable, atBufferIndex : bufferIndex)
    }
    unsafe fn setVertexIntersectionFunctionTables_withBufferRange_(
        &self,
        intersectionFunctionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexIntersectionFunctionTables : intersectionFunctionTables, withBufferRange : range)
    }
    unsafe fn setVertexAccelerationStructure_atBufferIndex_(
        &self,
        accelerationStructure: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexAccelerationStructure : accelerationStructure, atBufferIndex : bufferIndex)
    }
    unsafe fn setViewport_(&self, viewport: MTLViewport)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setViewport : viewport)
    }
    unsafe fn setViewports_count_(&self, viewports: *const MTLViewport, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setViewports : viewports, count : count)
    }
    unsafe fn setFrontFacingWinding_(&self, frontFacingWinding: MTLWinding)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrontFacingWinding : frontFacingWinding)
    }
    unsafe fn setVertexAmplificationCount_viewMappings_(
        &self,
        count: NSUInteger,
        viewMappings: *const MTLVertexAmplificationViewMapping,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexAmplificationCount : count, viewMappings : viewMappings)
    }
    unsafe fn setCullMode_(&self, cullMode: MTLCullMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCullMode : cullMode)
    }
    unsafe fn setDepthClipMode_(&self, depthClipMode: MTLDepthClipMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthClipMode : depthClipMode)
    }
    unsafe fn setDepthBias_slopeScale_clamp_(&self, depthBias: f32, slopeScale: f32, clamp: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthBias : depthBias, slopeScale : slopeScale, clamp : clamp)
    }
    unsafe fn setDepthTestMinBound_maxBound_(&self, minBound: f32, maxBound: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthTestMinBound : minBound, maxBound : maxBound)
    }
    unsafe fn setScissorRect_(&self, rect: MTLScissorRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScissorRect : rect)
    }
    unsafe fn setScissorRects_count_(&self, scissorRects: *const MTLScissorRect, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScissorRects : scissorRects, count : count)
    }
    unsafe fn setTriangleFillMode_(&self, fillMode: MTLTriangleFillMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTriangleFillMode : fillMode)
    }
    unsafe fn setFragmentBytes_length_atIndex_(
        &self,
        bytes: *const ::std::os::raw::c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentBytes : bytes, length : length, atIndex : index)
    }
    unsafe fn setFragmentBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setFragmentBufferOffset_atIndex_(&self, offset: NSUInteger, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentBufferOffset : offset, atIndex : index)
    }
    unsafe fn setFragmentBuffers_offsets_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentBuffers : buffers, offsets : offsets, withRange : range)
    }
    unsafe fn setFragmentTexture_atIndex_(&self, texture: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentTexture : texture, atIndex : index)
    }
    unsafe fn setFragmentTextures_withRange_(&self, textures: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentTextures : textures, withRange : range)
    }
    unsafe fn setFragmentSamplerState_atIndex_(&self, sampler: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentSamplerState : sampler, atIndex : index)
    }
    unsafe fn setFragmentSamplerStates_withRange_(&self, samplers: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentSamplerStates : samplers, withRange : range)
    }
    unsafe fn setFragmentSamplerState_lodMinClamp_lodMaxClamp_atIndex_(
        &self,
        sampler: *mut u64,
        lodMinClamp: f32,
        lodMaxClamp: f32,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentSamplerState : sampler, lodMinClamp : lodMinClamp, lodMaxClamp : lodMaxClamp, atIndex : index)
    }
    unsafe fn setFragmentSamplerStates_lodMinClamps_lodMaxClamps_withRange_(
        &self,
        samplers: *const *mut u64,
        lodMinClamps: *const f32,
        lodMaxClamps: *const f32,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentSamplerStates : samplers, lodMinClamps : lodMinClamps, lodMaxClamps : lodMaxClamps, withRange : range)
    }
    unsafe fn setFragmentVisibleFunctionTable_atBufferIndex_(
        &self,
        functionTable: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentVisibleFunctionTable : functionTable, atBufferIndex : bufferIndex)
    }
    unsafe fn setFragmentVisibleFunctionTables_withBufferRange_(
        &self,
        functionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentVisibleFunctionTables : functionTables, withBufferRange : range)
    }
    unsafe fn setFragmentIntersectionFunctionTable_atBufferIndex_(
        &self,
        intersectionFunctionTable: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentIntersectionFunctionTable : intersectionFunctionTable, atBufferIndex : bufferIndex)
    }
    unsafe fn setFragmentIntersectionFunctionTables_withBufferRange_(
        &self,
        intersectionFunctionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentIntersectionFunctionTables : intersectionFunctionTables, withBufferRange : range)
    }
    unsafe fn setFragmentAccelerationStructure_atBufferIndex_(
        &self,
        accelerationStructure: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentAccelerationStructure : accelerationStructure, atBufferIndex : bufferIndex)
    }
    unsafe fn setBlendColorRed_green_blue_alpha_(&self, red: f32, green: f32, blue: f32, alpha: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendColorRed : red, green : green, blue : blue, alpha : alpha)
    }
    unsafe fn setDepthStencilState_(&self, depthStencilState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStencilState : depthStencilState)
    }
    unsafe fn setStencilReferenceValue_(&self, referenceValue: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilReferenceValue : referenceValue)
    }
    unsafe fn setStencilFrontReferenceValue_backReferenceValue_(
        &self,
        frontReferenceValue: u32,
        backReferenceValue: u32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilFrontReferenceValue : frontReferenceValue, backReferenceValue : backReferenceValue)
    }
    unsafe fn setVisibilityResultMode_offset_(
        &self,
        mode: MTLVisibilityResultMode,
        offset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibilityResultMode : mode, offset : offset)
    }
    unsafe fn setColorStoreAction_atIndex_(
        &self,
        storeAction: MTLStoreAction,
        colorAttachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorStoreAction : storeAction, atIndex : colorAttachmentIndex)
    }
    unsafe fn setDepthStoreAction_(&self, storeAction: MTLStoreAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStoreAction : storeAction)
    }
    unsafe fn setStencilStoreAction_(&self, storeAction: MTLStoreAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilStoreAction : storeAction)
    }
    unsafe fn setColorStoreActionOptions_atIndex_(
        &self,
        storeActionOptions: MTLStoreActionOptions,
        colorAttachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorStoreActionOptions : storeActionOptions, atIndex : colorAttachmentIndex)
    }
    unsafe fn setDepthStoreActionOptions_(&self, storeActionOptions: MTLStoreActionOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStoreActionOptions : storeActionOptions)
    }
    unsafe fn setStencilStoreActionOptions_(&self, storeActionOptions: MTLStoreActionOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilStoreActionOptions : storeActionOptions)
    }
    unsafe fn setObjectBytes_length_atIndex_(
        &self,
        bytes: *const ::std::os::raw::c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectBytes : bytes, length : length, atIndex : index)
    }
    unsafe fn setObjectBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setObjectBufferOffset_atIndex_(&self, offset: NSUInteger, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectBufferOffset : offset, atIndex : index)
    }
    unsafe fn setObjectBuffers_offsets_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectBuffers : buffers, offsets : offsets, withRange : range)
    }
    unsafe fn setObjectTexture_atIndex_(&self, texture: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectTexture : texture, atIndex : index)
    }
    unsafe fn setObjectTextures_withRange_(&self, textures: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectTextures : textures, withRange : range)
    }
    unsafe fn setObjectSamplerState_atIndex_(&self, sampler: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectSamplerState : sampler, atIndex : index)
    }
    unsafe fn setObjectSamplerStates_withRange_(&self, samplers: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectSamplerStates : samplers, withRange : range)
    }
    unsafe fn setObjectSamplerState_lodMinClamp_lodMaxClamp_atIndex_(
        &self,
        sampler: *mut u64,
        lodMinClamp: f32,
        lodMaxClamp: f32,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectSamplerState : sampler, lodMinClamp : lodMinClamp, lodMaxClamp : lodMaxClamp, atIndex : index)
    }
    unsafe fn setObjectSamplerStates_lodMinClamps_lodMaxClamps_withRange_(
        &self,
        samplers: *const *mut u64,
        lodMinClamps: *const f32,
        lodMaxClamps: *const f32,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectSamplerStates : samplers, lodMinClamps : lodMinClamps, lodMaxClamps : lodMaxClamps, withRange : range)
    }
    unsafe fn setObjectThreadgroupMemoryLength_atIndex_(
        &self,
        length: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectThreadgroupMemoryLength : length, atIndex : index)
    }
    unsafe fn setMeshBytes_length_atIndex_(
        &self,
        bytes: *const ::std::os::raw::c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshBytes : bytes, length : length, atIndex : index)
    }
    unsafe fn setMeshBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setMeshBufferOffset_atIndex_(&self, offset: NSUInteger, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshBufferOffset : offset, atIndex : index)
    }
    unsafe fn setMeshBuffers_offsets_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshBuffers : buffers, offsets : offsets, withRange : range)
    }
    unsafe fn setMeshTexture_atIndex_(&self, texture: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshTexture : texture, atIndex : index)
    }
    unsafe fn setMeshTextures_withRange_(&self, textures: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshTextures : textures, withRange : range)
    }
    unsafe fn setMeshSamplerState_atIndex_(&self, sampler: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshSamplerState : sampler, atIndex : index)
    }
    unsafe fn setMeshSamplerStates_withRange_(&self, samplers: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshSamplerStates : samplers, withRange : range)
    }
    unsafe fn setMeshSamplerState_lodMinClamp_lodMaxClamp_atIndex_(
        &self,
        sampler: *mut u64,
        lodMinClamp: f32,
        lodMaxClamp: f32,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshSamplerState : sampler, lodMinClamp : lodMinClamp, lodMaxClamp : lodMaxClamp, atIndex : index)
    }
    unsafe fn setMeshSamplerStates_lodMinClamps_lodMaxClamps_withRange_(
        &self,
        samplers: *const *mut u64,
        lodMinClamps: *const f32,
        lodMaxClamps: *const f32,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshSamplerStates : samplers, lodMinClamps : lodMinClamps, lodMaxClamps : lodMaxClamps, withRange : range)
    }
    unsafe fn drawMeshThreadgroups_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup_(
        &self,
        threadgroupsPerGrid: MTLSize,
        threadsPerObjectThreadgroup: MTLSize,
        threadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawMeshThreadgroups : threadgroupsPerGrid, threadsPerObjectThreadgroup : threadsPerObjectThreadgroup, threadsPerMeshThreadgroup : threadsPerMeshThreadgroup)
    }
    unsafe fn drawMeshThreads_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup_(
        &self,
        threadsPerGrid: MTLSize,
        threadsPerObjectThreadgroup: MTLSize,
        threadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawMeshThreads : threadsPerGrid, threadsPerObjectThreadgroup : threadsPerObjectThreadgroup, threadsPerMeshThreadgroup : threadsPerMeshThreadgroup)
    }
    unsafe fn drawMeshThreadgroupsWithIndirectBuffer_indirectBufferOffset_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup_(
        &self,
        indirectBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
        threadsPerObjectThreadgroup: MTLSize,
        threadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawMeshThreadgroupsWithIndirectBuffer : indirectBuffer, indirectBufferOffset : indirectBufferOffset, threadsPerObjectThreadgroup : threadsPerObjectThreadgroup, threadsPerMeshThreadgroup : threadsPerMeshThreadgroup)
    }
    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount_(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
        instanceCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPrimitives : primitiveType, vertexStart : vertexStart, vertexCount : vertexCount, instanceCount : instanceCount)
    }
    unsafe fn drawPrimitives_vertexStart_vertexCount_(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPrimitives : primitiveType, vertexStart : vertexStart, vertexCount : vertexCount)
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: *mut u64,
        indexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPrimitives : primitiveType, indexCount : indexCount, indexType : indexType, indexBuffer : indexBuffer, indexBufferOffset : indexBufferOffset, instanceCount : instanceCount)
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: *mut u64,
        indexBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPrimitives : primitiveType, indexCount : indexCount, indexType : indexType, indexBuffer : indexBuffer, indexBufferOffset : indexBufferOffset)
    }
    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance_(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPrimitives : primitiveType, vertexStart : vertexStart, vertexCount : vertexCount, instanceCount : instanceCount, baseInstance : baseInstance)
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance_(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: *mut u64,
        indexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
        baseVertex: NSInteger,
        baseInstance: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPrimitives : primitiveType, indexCount : indexCount, indexType : indexType, indexBuffer : indexBuffer, indexBufferOffset : indexBufferOffset, instanceCount : instanceCount, baseVertex : baseVertex, baseInstance : baseInstance)
    }
    unsafe fn drawPrimitives_indirectBuffer_indirectBufferOffset_(
        &self,
        primitiveType: MTLPrimitiveType,
        indirectBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPrimitives : primitiveType, indirectBuffer : indirectBuffer, indirectBufferOffset : indirectBufferOffset)
    }
    unsafe fn drawIndexedPrimitives_indexType_indexBuffer_indexBufferOffset_indirectBuffer_indirectBufferOffset_(
        &self,
        primitiveType: MTLPrimitiveType,
        indexType: MTLIndexType,
        indexBuffer: *mut u64,
        indexBufferOffset: NSUInteger,
        indirectBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPrimitives : primitiveType, indexType : indexType, indexBuffer : indexBuffer, indexBufferOffset : indexBufferOffset, indirectBuffer : indirectBuffer, indirectBufferOffset : indirectBufferOffset)
    }
    unsafe fn textureBarrier(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureBarrier)
    }
    unsafe fn updateFence_afterStages_(&self, fence: *mut u64, stages: MTLRenderStages)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateFence : fence, afterStages : stages)
    }
    unsafe fn waitForFence_beforeStages_(&self, fence: *mut u64, stages: MTLRenderStages)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForFence : fence, beforeStages : stages)
    }
    unsafe fn setTessellationFactorBuffer_offset_instanceStride_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        instanceStride: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationFactorBuffer : buffer, offset : offset, instanceStride : instanceStride)
    }
    unsafe fn setTessellationFactorScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationFactorScale : scale)
    }
    unsafe fn drawPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_instanceCount_baseInstance_(
        &self,
        numberOfPatchControlPoints: NSUInteger,
        patchStart: NSUInteger,
        patchCount: NSUInteger,
        patchIndexBuffer: *mut u64,
        patchIndexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPatches : numberOfPatchControlPoints, patchStart : patchStart, patchCount : patchCount, patchIndexBuffer : patchIndexBuffer, patchIndexBufferOffset : patchIndexBufferOffset, instanceCount : instanceCount, baseInstance : baseInstance)
    }
    unsafe fn drawPatches_patchIndexBuffer_patchIndexBufferOffset_indirectBuffer_indirectBufferOffset_(
        &self,
        numberOfPatchControlPoints: NSUInteger,
        patchIndexBuffer: *mut u64,
        patchIndexBufferOffset: NSUInteger,
        indirectBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPatches : numberOfPatchControlPoints, patchIndexBuffer : patchIndexBuffer, patchIndexBufferOffset : patchIndexBufferOffset, indirectBuffer : indirectBuffer, indirectBufferOffset : indirectBufferOffset)
    }
    unsafe fn drawIndexedPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_controlPointIndexBuffer_controlPointIndexBufferOffset_instanceCount_baseInstance_(
        &self,
        numberOfPatchControlPoints: NSUInteger,
        patchStart: NSUInteger,
        patchCount: NSUInteger,
        patchIndexBuffer: *mut u64,
        patchIndexBufferOffset: NSUInteger,
        controlPointIndexBuffer: *mut u64,
        controlPointIndexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPatches : numberOfPatchControlPoints, patchStart : patchStart, patchCount : patchCount, patchIndexBuffer : patchIndexBuffer, patchIndexBufferOffset : patchIndexBufferOffset, controlPointIndexBuffer : controlPointIndexBuffer, controlPointIndexBufferOffset : controlPointIndexBufferOffset, instanceCount : instanceCount, baseInstance : baseInstance)
    }
    unsafe fn drawIndexedPatches_patchIndexBuffer_patchIndexBufferOffset_controlPointIndexBuffer_controlPointIndexBufferOffset_indirectBuffer_indirectBufferOffset_(
        &self,
        numberOfPatchControlPoints: NSUInteger,
        patchIndexBuffer: *mut u64,
        patchIndexBufferOffset: NSUInteger,
        controlPointIndexBuffer: *mut u64,
        controlPointIndexBufferOffset: NSUInteger,
        indirectBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPatches : numberOfPatchControlPoints, patchIndexBuffer : patchIndexBuffer, patchIndexBufferOffset : patchIndexBufferOffset, controlPointIndexBuffer : controlPointIndexBuffer, controlPointIndexBufferOffset : controlPointIndexBufferOffset, indirectBuffer : indirectBuffer, indirectBufferOffset : indirectBufferOffset)
    }
    unsafe fn setTileBytes_length_atIndex_(
        &self,
        bytes: *const ::std::os::raw::c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileBytes : bytes, length : length, atIndex : index)
    }
    unsafe fn setTileBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setTileBufferOffset_atIndex_(&self, offset: NSUInteger, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileBufferOffset : offset, atIndex : index)
    }
    unsafe fn setTileBuffers_offsets_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileBuffers : buffers, offsets : offsets, withRange : range)
    }
    unsafe fn setTileTexture_atIndex_(&self, texture: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileTexture : texture, atIndex : index)
    }
    unsafe fn setTileTextures_withRange_(&self, textures: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileTextures : textures, withRange : range)
    }
    unsafe fn setTileSamplerState_atIndex_(&self, sampler: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileSamplerState : sampler, atIndex : index)
    }
    unsafe fn setTileSamplerStates_withRange_(&self, samplers: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileSamplerStates : samplers, withRange : range)
    }
    unsafe fn setTileSamplerState_lodMinClamp_lodMaxClamp_atIndex_(
        &self,
        sampler: *mut u64,
        lodMinClamp: f32,
        lodMaxClamp: f32,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileSamplerState : sampler, lodMinClamp : lodMinClamp, lodMaxClamp : lodMaxClamp, atIndex : index)
    }
    unsafe fn setTileSamplerStates_lodMinClamps_lodMaxClamps_withRange_(
        &self,
        samplers: *const *mut u64,
        lodMinClamps: *const f32,
        lodMaxClamps: *const f32,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileSamplerStates : samplers, lodMinClamps : lodMinClamps, lodMaxClamps : lodMaxClamps, withRange : range)
    }
    unsafe fn setTileVisibleFunctionTable_atBufferIndex_(
        &self,
        functionTable: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileVisibleFunctionTable : functionTable, atBufferIndex : bufferIndex)
    }
    unsafe fn setTileVisibleFunctionTables_withBufferRange_(
        &self,
        functionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileVisibleFunctionTables : functionTables, withBufferRange : range)
    }
    unsafe fn setTileIntersectionFunctionTable_atBufferIndex_(
        &self,
        intersectionFunctionTable: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileIntersectionFunctionTable : intersectionFunctionTable, atBufferIndex : bufferIndex)
    }
    unsafe fn setTileIntersectionFunctionTables_withBufferRange_(
        &self,
        intersectionFunctionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileIntersectionFunctionTables : intersectionFunctionTables, withBufferRange : range)
    }
    unsafe fn setTileAccelerationStructure_atBufferIndex_(
        &self,
        accelerationStructure: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileAccelerationStructure : accelerationStructure, atBufferIndex : bufferIndex)
    }
    unsafe fn dispatchThreadsPerTile_(&self, threadsPerTile: MTLSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchThreadsPerTile : threadsPerTile)
    }
    unsafe fn setThreadgroupMemoryLength_offset_atIndex_(
        &self,
        length: NSUInteger,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadgroupMemoryLength : length, offset : offset, atIndex : index)
    }
    unsafe fn useResource_usage_(&self, resource: *mut u64, usage: MTLResourceUsage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResource : resource, usage : usage)
    }
    unsafe fn useResources_count_usage_(
        &self,
        resources: *const *mut u64,
        count: NSUInteger,
        usage: MTLResourceUsage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResources : resources, count : count, usage : usage)
    }
    unsafe fn useResource_usage_stages_(
        &self,
        resource: *mut u64,
        usage: MTLResourceUsage,
        stages: MTLRenderStages,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResource : resource, usage : usage, stages : stages)
    }
    unsafe fn useResources_count_usage_stages_(
        &self,
        resources: *const *mut u64,
        count: NSUInteger,
        usage: MTLResourceUsage,
        stages: MTLRenderStages,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResources : resources, count : count, usage : usage, stages : stages)
    }
    unsafe fn useHeap_(&self, heap: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useHeap : heap)
    }
    unsafe fn useHeaps_count_(&self, heaps: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useHeaps : heaps, count : count)
    }
    unsafe fn useHeap_stages_(&self, heap: *mut u64, stages: MTLRenderStages)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useHeap : heap, stages : stages)
    }
    unsafe fn useHeaps_count_stages_(
        &self,
        heaps: *const *mut u64,
        count: NSUInteger,
        stages: MTLRenderStages,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useHeaps : heaps, count : count, stages : stages)
    }
    unsafe fn executeCommandsInBuffer_withRange_(
        &self,
        indirectCommandBuffer: *mut u64,
        executionRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeCommandsInBuffer : indirectCommandBuffer, withRange : executionRange)
    }
    unsafe fn executeCommandsInBuffer_indirectBuffer_indirectBufferOffset_(
        &self,
        indirectCommandbuffer: *mut u64,
        indirectRangeBuffer: *mut u64,
        indirectBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeCommandsInBuffer : indirectCommandbuffer, indirectBuffer : indirectRangeBuffer, indirectBufferOffset : indirectBufferOffset)
    }
    unsafe fn memoryBarrierWithScope_afterStages_beforeStages_(
        &self,
        scope: MTLBarrierScope,
        after: MTLRenderStages,
        before: MTLRenderStages,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, memoryBarrierWithScope : scope, afterStages : after, beforeStages : before)
    }
    unsafe fn memoryBarrierWithResources_count_afterStages_beforeStages_(
        &self,
        resources: *const *mut u64,
        count: NSUInteger,
        after: MTLRenderStages,
        before: MTLRenderStages,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, memoryBarrierWithResources : resources, count : count, afterStages : after, beforeStages : before)
    }
    unsafe fn sampleCountersInBuffer_atSampleIndex_withBarrier_(
        &self,
        sampleBuffer: *mut u64,
        sampleIndex: NSUInteger,
        barrier: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleCountersInBuffer : sampleBuffer, atSampleIndex : sampleIndex, withBarrier : barrier)
    }
    unsafe fn setColorAttachmentMap_(&self, mapping: MTLLogicalToPhysicalColorAttachmentMap)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorAttachmentMap : mapping)
    }
    unsafe fn tileWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileWidth)
    }
    unsafe fn tileHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileHeight)
    }
}
pub trait PMTLFunctionHandle: Sized + std::ops::Deref {
    unsafe fn functionType(&self) -> MTLFunctionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionType)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLVisibleFunctionTableDescriptor(pub id);
impl std::ops::Deref for MTLVisibleFunctionTableDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLVisibleFunctionTableDescriptor {}
impl MTLVisibleFunctionTableDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLVisibleFunctionTableDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLVisibleFunctionTableDescriptor {}
impl INSObject for MTLVisibleFunctionTableDescriptor {}
impl PNSObject for MTLVisibleFunctionTableDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLVisibleFunctionTableDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLVisibleFunctionTableDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLVisibleFunctionTableDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLVisibleFunctionTableDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLVisibleFunctionTableDescriptor")
        }
    }
}
impl IMTLVisibleFunctionTableDescriptor for MTLVisibleFunctionTableDescriptor {}
pub trait IMTLVisibleFunctionTableDescriptor: Sized + std::ops::Deref {
    unsafe fn functionCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionCount)
    }
    unsafe fn setFunctionCount_(&self, functionCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionCount : functionCount)
    }
    unsafe fn visibleFunctionTableDescriptor() -> MTLVisibleFunctionTableDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLVisibleFunctionTableDescriptor").unwrap(), visibleFunctionTableDescriptor)
    }
}
pub trait PMTLVisibleFunctionTable: Sized + std::ops::Deref {
    unsafe fn setFunction_atIndex_(&self, function: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunction : function, atIndex : index)
    }
    unsafe fn setFunctions_withRange_(&self, functions: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctions : functions, withRange : range)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
}
pub type MTLBlendFactor = NSUInteger;
pub type MTLBlendOperation = NSUInteger;
pub type MTLColorWriteMask = NSUInteger;
pub type MTLPrimitiveTopologyClass = NSUInteger;
pub type MTLTessellationPartitionMode = NSUInteger;
pub type MTLTessellationFactorStepFunction = NSUInteger;
pub type MTLTessellationFactorFormat = NSUInteger;
pub type MTLTessellationControlPointIndexType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPipelineColorAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLRenderPipelineColorAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPipelineColorAttachmentDescriptor {}
impl MTLRenderPipelineColorAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPipelineColorAttachmentDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLRenderPipelineColorAttachmentDescriptor {}
impl INSObject for MTLRenderPipelineColorAttachmentDescriptor {}
impl PNSObject for MTLRenderPipelineColorAttachmentDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLRenderPipelineColorAttachmentDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLRenderPipelineColorAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPipelineColorAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPipelineColorAttachmentDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRenderPipelineColorAttachmentDescriptor")
        }
    }
}
impl IMTLRenderPipelineColorAttachmentDescriptor for MTLRenderPipelineColorAttachmentDescriptor {}
pub trait IMTLRenderPipelineColorAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn pixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn setPixelFormat_(&self, pixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelFormat : pixelFormat)
    }
    unsafe fn isBlendingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBlendingEnabled)
    }
    unsafe fn setBlendingEnabled_(&self, blendingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendingEnabled : blendingEnabled)
    }
    unsafe fn sourceRGBBlendFactor(&self) -> MTLBlendFactor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceRGBBlendFactor)
    }
    unsafe fn setSourceRGBBlendFactor_(&self, sourceRGBBlendFactor: MTLBlendFactor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceRGBBlendFactor : sourceRGBBlendFactor)
    }
    unsafe fn destinationRGBBlendFactor(&self) -> MTLBlendFactor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationRGBBlendFactor)
    }
    unsafe fn setDestinationRGBBlendFactor_(&self, destinationRGBBlendFactor: MTLBlendFactor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationRGBBlendFactor : destinationRGBBlendFactor)
    }
    unsafe fn rgbBlendOperation(&self) -> MTLBlendOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rgbBlendOperation)
    }
    unsafe fn setRgbBlendOperation_(&self, rgbBlendOperation: MTLBlendOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRgbBlendOperation : rgbBlendOperation)
    }
    unsafe fn sourceAlphaBlendFactor(&self) -> MTLBlendFactor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAlphaBlendFactor)
    }
    unsafe fn setSourceAlphaBlendFactor_(&self, sourceAlphaBlendFactor: MTLBlendFactor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceAlphaBlendFactor : sourceAlphaBlendFactor)
    }
    unsafe fn destinationAlphaBlendFactor(&self) -> MTLBlendFactor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationAlphaBlendFactor)
    }
    unsafe fn setDestinationAlphaBlendFactor_(&self, destinationAlphaBlendFactor: MTLBlendFactor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationAlphaBlendFactor : destinationAlphaBlendFactor)
    }
    unsafe fn alphaBlendOperation(&self) -> MTLBlendOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaBlendOperation)
    }
    unsafe fn setAlphaBlendOperation_(&self, alphaBlendOperation: MTLBlendOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaBlendOperation : alphaBlendOperation)
    }
    unsafe fn writeMask(&self) -> MTLColorWriteMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writeMask)
    }
    unsafe fn setWriteMask_(&self, writeMask: MTLColorWriteMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWriteMask : writeMask)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLLogicalToPhysicalColorAttachmentMap(pub id);
impl std::ops::Deref for MTLLogicalToPhysicalColorAttachmentMap {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLLogicalToPhysicalColorAttachmentMap {}
impl MTLLogicalToPhysicalColorAttachmentMap {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLLogicalToPhysicalColorAttachmentMap").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLLogicalToPhysicalColorAttachmentMap {}
impl INSObject for MTLLogicalToPhysicalColorAttachmentMap {}
impl PNSObject for MTLLogicalToPhysicalColorAttachmentMap {}
impl std::convert::TryFrom<NSObject> for MTLLogicalToPhysicalColorAttachmentMap {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLLogicalToPhysicalColorAttachmentMap, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLLogicalToPhysicalColorAttachmentMap").unwrap())
        };
        if is_kind_of {
            Ok(MTLLogicalToPhysicalColorAttachmentMap(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLLogicalToPhysicalColorAttachmentMap")
        }
    }
}
impl IMTLLogicalToPhysicalColorAttachmentMap for MTLLogicalToPhysicalColorAttachmentMap {}
pub trait IMTLLogicalToPhysicalColorAttachmentMap: Sized + std::ops::Deref {
    unsafe fn setPhysicalIndex_forLogicalIndex_(
        &self,
        physicalIndex: NSUInteger,
        logicalIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhysicalIndex : physicalIndex, forLogicalIndex : logicalIndex)
    }
    unsafe fn getPhysicalIndexForLogicalIndex_(&self, logicalIndex: NSUInteger) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPhysicalIndexForLogicalIndex : logicalIndex)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPipelineReflection(pub id);
impl std::ops::Deref for MTLRenderPipelineReflection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPipelineReflection {}
impl MTLRenderPipelineReflection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPipelineReflection").unwrap(), alloc) })
    }
}
impl INSObject for MTLRenderPipelineReflection {}
impl PNSObject for MTLRenderPipelineReflection {}
impl std::convert::TryFrom<NSObject> for MTLRenderPipelineReflection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLRenderPipelineReflection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPipelineReflection").unwrap()) };
        if is_kind_of {
            Ok(MTLRenderPipelineReflection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRenderPipelineReflection")
        }
    }
}
impl IMTLRenderPipelineReflection for MTLRenderPipelineReflection {}
pub trait IMTLRenderPipelineReflection: Sized + std::ops::Deref {
    unsafe fn vertexBindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexBindings)
    }
    unsafe fn fragmentBindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentBindings)
    }
    unsafe fn tileBindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileBindings)
    }
    unsafe fn objectBindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectBindings)
    }
    unsafe fn meshBindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshBindings)
    }
    unsafe fn vertexArguments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexArguments)
    }
    unsafe fn fragmentArguments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentArguments)
    }
    unsafe fn tileArguments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileArguments)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPipelineDescriptor(pub id);
impl std::ops::Deref for MTLRenderPipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPipelineDescriptor {}
impl MTLRenderPipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPipelineDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLRenderPipelineDescriptor {}
impl INSObject for MTLRenderPipelineDescriptor {}
impl PNSObject for MTLRenderPipelineDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLRenderPipelineDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLRenderPipelineDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPipelineDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLRenderPipelineDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRenderPipelineDescriptor")
        }
    }
}
impl IMTLRenderPipelineDescriptor for MTLRenderPipelineDescriptor {}
pub trait IMTLRenderPipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn vertexFunction(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexFunction)
    }
    unsafe fn setVertexFunction_(&self, vertexFunction: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexFunction : vertexFunction)
    }
    unsafe fn fragmentFunction(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentFunction)
    }
    unsafe fn setFragmentFunction_(&self, fragmentFunction: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentFunction : fragmentFunction)
    }
    unsafe fn vertexDescriptor(&self) -> MTLVertexDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexDescriptor)
    }
    unsafe fn setVertexDescriptor_(&self, vertexDescriptor: MTLVertexDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexDescriptor : vertexDescriptor)
    }
    unsafe fn sampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
    unsafe fn setSampleCount_(&self, sampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleCount : sampleCount)
    }
    unsafe fn rasterSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rasterSampleCount)
    }
    unsafe fn setRasterSampleCount_(&self, rasterSampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterSampleCount : rasterSampleCount)
    }
    unsafe fn isAlphaToCoverageEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAlphaToCoverageEnabled)
    }
    unsafe fn setAlphaToCoverageEnabled_(&self, alphaToCoverageEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaToCoverageEnabled : alphaToCoverageEnabled)
    }
    unsafe fn isAlphaToOneEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAlphaToOneEnabled)
    }
    unsafe fn setAlphaToOneEnabled_(&self, alphaToOneEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaToOneEnabled : alphaToOneEnabled)
    }
    unsafe fn isRasterizationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRasterizationEnabled)
    }
    unsafe fn setRasterizationEnabled_(&self, rasterizationEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterizationEnabled : rasterizationEnabled)
    }
    unsafe fn maxVertexAmplificationCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxVertexAmplificationCount)
    }
    unsafe fn setMaxVertexAmplificationCount_(&self, maxVertexAmplificationCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxVertexAmplificationCount : maxVertexAmplificationCount)
    }
    unsafe fn colorAttachments(&self) -> MTLRenderPipelineColorAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachments)
    }
    unsafe fn depthAttachmentPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthAttachmentPixelFormat)
    }
    unsafe fn setDepthAttachmentPixelFormat_(&self, depthAttachmentPixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthAttachmentPixelFormat : depthAttachmentPixelFormat)
    }
    unsafe fn stencilAttachmentPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stencilAttachmentPixelFormat)
    }
    unsafe fn setStencilAttachmentPixelFormat_(&self, stencilAttachmentPixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilAttachmentPixelFormat : stencilAttachmentPixelFormat)
    }
    unsafe fn inputPrimitiveTopology(&self) -> MTLPrimitiveTopologyClass
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputPrimitiveTopology)
    }
    unsafe fn setInputPrimitiveTopology_(&self, inputPrimitiveTopology: MTLPrimitiveTopologyClass)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputPrimitiveTopology : inputPrimitiveTopology)
    }
    unsafe fn tessellationPartitionMode(&self) -> MTLTessellationPartitionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellationPartitionMode)
    }
    unsafe fn setTessellationPartitionMode_(
        &self,
        tessellationPartitionMode: MTLTessellationPartitionMode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationPartitionMode : tessellationPartitionMode)
    }
    unsafe fn maxTessellationFactor(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTessellationFactor)
    }
    unsafe fn setMaxTessellationFactor_(&self, maxTessellationFactor: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTessellationFactor : maxTessellationFactor)
    }
    unsafe fn isTessellationFactorScaleEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTessellationFactorScaleEnabled)
    }
    unsafe fn setTessellationFactorScaleEnabled_(&self, tessellationFactorScaleEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationFactorScaleEnabled : tessellationFactorScaleEnabled)
    }
    unsafe fn tessellationFactorFormat(&self) -> MTLTessellationFactorFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellationFactorFormat)
    }
    unsafe fn setTessellationFactorFormat_(
        &self,
        tessellationFactorFormat: MTLTessellationFactorFormat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationFactorFormat : tessellationFactorFormat)
    }
    unsafe fn tessellationControlPointIndexType(&self) -> MTLTessellationControlPointIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellationControlPointIndexType)
    }
    unsafe fn setTessellationControlPointIndexType_(
        &self,
        tessellationControlPointIndexType: MTLTessellationControlPointIndexType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationControlPointIndexType : tessellationControlPointIndexType)
    }
    unsafe fn tessellationFactorStepFunction(&self) -> MTLTessellationFactorStepFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellationFactorStepFunction)
    }
    unsafe fn setTessellationFactorStepFunction_(
        &self,
        tessellationFactorStepFunction: MTLTessellationFactorStepFunction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationFactorStepFunction : tessellationFactorStepFunction)
    }
    unsafe fn tessellationOutputWindingOrder(&self) -> MTLWinding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tessellationOutputWindingOrder)
    }
    unsafe fn setTessellationOutputWindingOrder_(&self, tessellationOutputWindingOrder: MTLWinding)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTessellationOutputWindingOrder : tessellationOutputWindingOrder)
    }
    unsafe fn vertexBuffers(&self) -> MTLPipelineBufferDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexBuffers)
    }
    unsafe fn fragmentBuffers(&self) -> MTLPipelineBufferDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentBuffers)
    }
    unsafe fn supportIndirectCommandBuffers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportIndirectCommandBuffers)
    }
    unsafe fn setSupportIndirectCommandBuffers_(&self, supportIndirectCommandBuffers: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportIndirectCommandBuffers : supportIndirectCommandBuffers)
    }
    unsafe fn binaryArchives(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, binaryArchives)
    }
    unsafe fn setBinaryArchives_(&self, binaryArchives: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBinaryArchives : binaryArchives)
    }
    unsafe fn vertexPreloadedLibraries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexPreloadedLibraries)
    }
    unsafe fn setVertexPreloadedLibraries_(&self, vertexPreloadedLibraries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexPreloadedLibraries : vertexPreloadedLibraries)
    }
    unsafe fn fragmentPreloadedLibraries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentPreloadedLibraries)
    }
    unsafe fn setFragmentPreloadedLibraries_(&self, fragmentPreloadedLibraries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentPreloadedLibraries : fragmentPreloadedLibraries)
    }
    unsafe fn vertexLinkedFunctions(&self) -> MTLLinkedFunctions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexLinkedFunctions)
    }
    unsafe fn setVertexLinkedFunctions_(&self, vertexLinkedFunctions: MTLLinkedFunctions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexLinkedFunctions : vertexLinkedFunctions)
    }
    unsafe fn fragmentLinkedFunctions(&self) -> MTLLinkedFunctions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentLinkedFunctions)
    }
    unsafe fn setFragmentLinkedFunctions_(&self, fragmentLinkedFunctions: MTLLinkedFunctions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentLinkedFunctions : fragmentLinkedFunctions)
    }
    unsafe fn supportAddingVertexBinaryFunctions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportAddingVertexBinaryFunctions)
    }
    unsafe fn setSupportAddingVertexBinaryFunctions_(
        &self,
        supportAddingVertexBinaryFunctions: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportAddingVertexBinaryFunctions : supportAddingVertexBinaryFunctions)
    }
    unsafe fn supportAddingFragmentBinaryFunctions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportAddingFragmentBinaryFunctions)
    }
    unsafe fn setSupportAddingFragmentBinaryFunctions_(
        &self,
        supportAddingFragmentBinaryFunctions: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportAddingFragmentBinaryFunctions : supportAddingFragmentBinaryFunctions)
    }
    unsafe fn maxVertexCallStackDepth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxVertexCallStackDepth)
    }
    unsafe fn setMaxVertexCallStackDepth_(&self, maxVertexCallStackDepth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxVertexCallStackDepth : maxVertexCallStackDepth)
    }
    unsafe fn maxFragmentCallStackDepth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxFragmentCallStackDepth)
    }
    unsafe fn setMaxFragmentCallStackDepth_(&self, maxFragmentCallStackDepth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxFragmentCallStackDepth : maxFragmentCallStackDepth)
    }
    unsafe fn shaderValidation(&self) -> MTLShaderValidation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shaderValidation)
    }
    unsafe fn setShaderValidation_(&self, shaderValidation: MTLShaderValidation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShaderValidation : shaderValidation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPipelineFunctionsDescriptor(pub id);
impl std::ops::Deref for MTLRenderPipelineFunctionsDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPipelineFunctionsDescriptor {}
impl MTLRenderPipelineFunctionsDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPipelineFunctionsDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLRenderPipelineFunctionsDescriptor {}
impl INSObject for MTLRenderPipelineFunctionsDescriptor {}
impl PNSObject for MTLRenderPipelineFunctionsDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLRenderPipelineFunctionsDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLRenderPipelineFunctionsDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPipelineFunctionsDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPipelineFunctionsDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRenderPipelineFunctionsDescriptor")
        }
    }
}
impl IMTLRenderPipelineFunctionsDescriptor for MTLRenderPipelineFunctionsDescriptor {}
pub trait IMTLRenderPipelineFunctionsDescriptor: Sized + std::ops::Deref {
    unsafe fn vertexAdditionalBinaryFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexAdditionalBinaryFunctions)
    }
    unsafe fn setVertexAdditionalBinaryFunctions_(&self, vertexAdditionalBinaryFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexAdditionalBinaryFunctions : vertexAdditionalBinaryFunctions)
    }
    unsafe fn fragmentAdditionalBinaryFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentAdditionalBinaryFunctions)
    }
    unsafe fn setFragmentAdditionalBinaryFunctions_(
        &self,
        fragmentAdditionalBinaryFunctions: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentAdditionalBinaryFunctions : fragmentAdditionalBinaryFunctions)
    }
    unsafe fn tileAdditionalBinaryFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileAdditionalBinaryFunctions)
    }
    unsafe fn setTileAdditionalBinaryFunctions_(&self, tileAdditionalBinaryFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileAdditionalBinaryFunctions : tileAdditionalBinaryFunctions)
    }
}
pub trait PMTLRenderPipelineState: Sized + std::ops::Deref {
    unsafe fn functionHandleWithName_stage_(
        &self,
        name: NSString,
        stage: MTLRenderStages,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, functionHandleWithName : name, stage : stage)
    }
    unsafe fn functionHandleWithBinaryFunction_stage_(
        &self,
        function: *mut u64,
        stage: MTLRenderStages,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, functionHandleWithBinaryFunction : function, stage : stage)
    }
    unsafe fn newRenderPipelineStateWithBinaryFunctions_error_(
        &self,
        binaryFunctionsDescriptor: MTL4RenderPipelineBinaryFunctionsDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithBinaryFunctions : binaryFunctionsDescriptor, error : error)
    }
    unsafe fn newRenderPipelineDescriptorForSpecialization(&self) -> MTL4PipelineDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newRenderPipelineDescriptorForSpecialization)
    }
    unsafe fn imageblockMemoryLengthForDimensions_(
        &self,
        imageblockDimensions: MTLSize,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageblockMemoryLengthForDimensions : imageblockDimensions)
    }
    unsafe fn functionHandleWithFunction_stage_(
        &self,
        function: *mut u64,
        stage: MTLRenderStages,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, functionHandleWithFunction : function, stage : stage)
    }
    unsafe fn newVisibleFunctionTableWithDescriptor_stage_(
        &self,
        descriptor: MTLVisibleFunctionTableDescriptor,
        stage: MTLRenderStages,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newVisibleFunctionTableWithDescriptor : descriptor, stage : stage)
    }
    unsafe fn newIntersectionFunctionTableWithDescriptor_stage_(
        &self,
        descriptor: MTLIntersectionFunctionTableDescriptor,
        stage: MTLRenderStages,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIntersectionFunctionTableWithDescriptor : descriptor, stage : stage)
    }
    unsafe fn newRenderPipelineStateWithAdditionalBinaryFunctions_error_(
        &self,
        additionalBinaryFunctions: MTLRenderPipelineFunctionsDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithAdditionalBinaryFunctions : additionalBinaryFunctions, error : error)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn reflection(&self) -> MTLRenderPipelineReflection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflection)
    }
    unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerThreadgroup)
    }
    unsafe fn threadgroupSizeMatchesTileSize(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadgroupSizeMatchesTileSize)
    }
    unsafe fn imageblockSampleLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageblockSampleLength)
    }
    unsafe fn supportIndirectCommandBuffers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportIndirectCommandBuffers)
    }
    unsafe fn maxTotalThreadsPerObjectThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerObjectThreadgroup)
    }
    unsafe fn maxTotalThreadsPerMeshThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerMeshThreadgroup)
    }
    unsafe fn objectThreadExecutionWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectThreadExecutionWidth)
    }
    unsafe fn meshThreadExecutionWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshThreadExecutionWidth)
    }
    unsafe fn maxTotalThreadgroupsPerMeshGrid(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadgroupsPerMeshGrid)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
    unsafe fn shaderValidation(&self) -> MTLShaderValidation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shaderValidation)
    }
    unsafe fn requiredThreadsPerTileThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerTileThreadgroup)
    }
    unsafe fn requiredThreadsPerObjectThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerObjectThreadgroup)
    }
    unsafe fn requiredThreadsPerMeshThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerMeshThreadgroup)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRenderPipelineColorAttachmentDescriptorArray(pub id);
impl std::ops::Deref for MTLRenderPipelineColorAttachmentDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRenderPipelineColorAttachmentDescriptorArray {}
impl MTLRenderPipelineColorAttachmentDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRenderPipelineColorAttachmentDescriptorArray").unwrap(), alloc)
        })
    }
}
impl INSObject for MTLRenderPipelineColorAttachmentDescriptorArray {}
impl PNSObject for MTLRenderPipelineColorAttachmentDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLRenderPipelineColorAttachmentDescriptorArray {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLRenderPipelineColorAttachmentDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRenderPipelineColorAttachmentDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLRenderPipelineColorAttachmentDescriptorArray(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MTLRenderPipelineColorAttachmentDescriptorArray" ,)
        }
    }
}
impl IMTLRenderPipelineColorAttachmentDescriptorArray
    for MTLRenderPipelineColorAttachmentDescriptorArray
{
}
pub trait IMTLRenderPipelineColorAttachmentDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(
        &self,
        attachmentIndex: NSUInteger,
    ) -> MTLRenderPipelineColorAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : attachmentIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attachment: MTLRenderPipelineColorAttachmentDescriptor,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attachment, atIndexedSubscript : attachmentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTileRenderPipelineColorAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLTileRenderPipelineColorAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLTileRenderPipelineColorAttachmentDescriptor {}
impl MTLTileRenderPipelineColorAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTileRenderPipelineColorAttachmentDescriptor").unwrap(), alloc)
        })
    }
}
impl PNSCopying for MTLTileRenderPipelineColorAttachmentDescriptor {}
impl INSObject for MTLTileRenderPipelineColorAttachmentDescriptor {}
impl PNSObject for MTLTileRenderPipelineColorAttachmentDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLTileRenderPipelineColorAttachmentDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLTileRenderPipelineColorAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLTileRenderPipelineColorAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLTileRenderPipelineColorAttachmentDescriptor(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MTLTileRenderPipelineColorAttachmentDescriptor" ,)
        }
    }
}
impl IMTLTileRenderPipelineColorAttachmentDescriptor
    for MTLTileRenderPipelineColorAttachmentDescriptor
{
}
pub trait IMTLTileRenderPipelineColorAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn pixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn setPixelFormat_(&self, pixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelFormat : pixelFormat)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTileRenderPipelineColorAttachmentDescriptorArray(pub id);
impl std::ops::Deref for MTLTileRenderPipelineColorAttachmentDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLTileRenderPipelineColorAttachmentDescriptorArray {}
impl MTLTileRenderPipelineColorAttachmentDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTileRenderPipelineColorAttachmentDescriptorArray").unwrap(), alloc)
        })
    }
}
impl INSObject for MTLTileRenderPipelineColorAttachmentDescriptorArray {}
impl PNSObject for MTLTileRenderPipelineColorAttachmentDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTLTileRenderPipelineColorAttachmentDescriptorArray {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLTileRenderPipelineColorAttachmentDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLTileRenderPipelineColorAttachmentDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLTileRenderPipelineColorAttachmentDescriptorArray(
                parent.0,
            ))
        } else {
            Err ("This NSObject cannot be downcasted to MTLTileRenderPipelineColorAttachmentDescriptorArray" ,)
        }
    }
}
impl IMTLTileRenderPipelineColorAttachmentDescriptorArray
    for MTLTileRenderPipelineColorAttachmentDescriptorArray
{
}
pub trait IMTLTileRenderPipelineColorAttachmentDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(
        &self,
        attachmentIndex: NSUInteger,
    ) -> MTLTileRenderPipelineColorAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : attachmentIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attachment: MTLTileRenderPipelineColorAttachmentDescriptor,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attachment, atIndexedSubscript : attachmentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLTileRenderPipelineDescriptor(pub id);
impl std::ops::Deref for MTLTileRenderPipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLTileRenderPipelineDescriptor {}
impl MTLTileRenderPipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLTileRenderPipelineDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLTileRenderPipelineDescriptor {}
impl INSObject for MTLTileRenderPipelineDescriptor {}
impl PNSObject for MTLTileRenderPipelineDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLTileRenderPipelineDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLTileRenderPipelineDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLTileRenderPipelineDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLTileRenderPipelineDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLTileRenderPipelineDescriptor")
        }
    }
}
impl IMTLTileRenderPipelineDescriptor for MTLTileRenderPipelineDescriptor {}
pub trait IMTLTileRenderPipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn tileFunction(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileFunction)
    }
    unsafe fn setTileFunction_(&self, tileFunction: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileFunction : tileFunction)
    }
    unsafe fn rasterSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rasterSampleCount)
    }
    unsafe fn setRasterSampleCount_(&self, rasterSampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterSampleCount : rasterSampleCount)
    }
    unsafe fn colorAttachments(&self) -> MTLTileRenderPipelineColorAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachments)
    }
    unsafe fn threadgroupSizeMatchesTileSize(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadgroupSizeMatchesTileSize)
    }
    unsafe fn setThreadgroupSizeMatchesTileSize_(&self, threadgroupSizeMatchesTileSize: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadgroupSizeMatchesTileSize : threadgroupSizeMatchesTileSize)
    }
    unsafe fn tileBuffers(&self) -> MTLPipelineBufferDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileBuffers)
    }
    unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerThreadgroup)
    }
    unsafe fn setMaxTotalThreadsPerThreadgroup_(&self, maxTotalThreadsPerThreadgroup: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadsPerThreadgroup : maxTotalThreadsPerThreadgroup)
    }
    unsafe fn binaryArchives(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, binaryArchives)
    }
    unsafe fn setBinaryArchives_(&self, binaryArchives: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBinaryArchives : binaryArchives)
    }
    unsafe fn preloadedLibraries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preloadedLibraries)
    }
    unsafe fn setPreloadedLibraries_(&self, preloadedLibraries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreloadedLibraries : preloadedLibraries)
    }
    unsafe fn linkedFunctions(&self) -> MTLLinkedFunctions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkedFunctions)
    }
    unsafe fn setLinkedFunctions_(&self, linkedFunctions: MTLLinkedFunctions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinkedFunctions : linkedFunctions)
    }
    unsafe fn supportAddingBinaryFunctions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportAddingBinaryFunctions)
    }
    unsafe fn setSupportAddingBinaryFunctions_(&self, supportAddingBinaryFunctions: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportAddingBinaryFunctions : supportAddingBinaryFunctions)
    }
    unsafe fn maxCallStackDepth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxCallStackDepth)
    }
    unsafe fn setMaxCallStackDepth_(&self, maxCallStackDepth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxCallStackDepth : maxCallStackDepth)
    }
    unsafe fn shaderValidation(&self) -> MTLShaderValidation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shaderValidation)
    }
    unsafe fn setShaderValidation_(&self, shaderValidation: MTLShaderValidation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShaderValidation : shaderValidation)
    }
    unsafe fn requiredThreadsPerThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerThreadgroup)
    }
    unsafe fn setRequiredThreadsPerThreadgroup_(&self, requiredThreadsPerThreadgroup: MTLSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredThreadsPerThreadgroup : requiredThreadsPerThreadgroup)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLMeshRenderPipelineDescriptor(pub id);
impl std::ops::Deref for MTLMeshRenderPipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLMeshRenderPipelineDescriptor {}
impl MTLMeshRenderPipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLMeshRenderPipelineDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLMeshRenderPipelineDescriptor {}
impl INSObject for MTLMeshRenderPipelineDescriptor {}
impl PNSObject for MTLMeshRenderPipelineDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLMeshRenderPipelineDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLMeshRenderPipelineDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLMeshRenderPipelineDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLMeshRenderPipelineDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLMeshRenderPipelineDescriptor")
        }
    }
}
impl IMTLMeshRenderPipelineDescriptor for MTLMeshRenderPipelineDescriptor {}
pub trait IMTLMeshRenderPipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn objectFunction(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectFunction)
    }
    unsafe fn setObjectFunction_(&self, objectFunction: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectFunction : objectFunction)
    }
    unsafe fn meshFunction(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshFunction)
    }
    unsafe fn setMeshFunction_(&self, meshFunction: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshFunction : meshFunction)
    }
    unsafe fn fragmentFunction(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentFunction)
    }
    unsafe fn setFragmentFunction_(&self, fragmentFunction: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentFunction : fragmentFunction)
    }
    unsafe fn maxTotalThreadsPerObjectThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerObjectThreadgroup)
    }
    unsafe fn setMaxTotalThreadsPerObjectThreadgroup_(
        &self,
        maxTotalThreadsPerObjectThreadgroup: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadsPerObjectThreadgroup : maxTotalThreadsPerObjectThreadgroup)
    }
    unsafe fn maxTotalThreadsPerMeshThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerMeshThreadgroup)
    }
    unsafe fn setMaxTotalThreadsPerMeshThreadgroup_(
        &self,
        maxTotalThreadsPerMeshThreadgroup: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadsPerMeshThreadgroup : maxTotalThreadsPerMeshThreadgroup)
    }
    unsafe fn objectThreadgroupSizeIsMultipleOfThreadExecutionWidth(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectThreadgroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth_(
        &self,
        objectThreadgroupSizeIsMultipleOfThreadExecutionWidth: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth : objectThreadgroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn meshThreadgroupSizeIsMultipleOfThreadExecutionWidth(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshThreadgroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth_(
        &self,
        meshThreadgroupSizeIsMultipleOfThreadExecutionWidth: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth : meshThreadgroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn payloadMemoryLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, payloadMemoryLength)
    }
    unsafe fn setPayloadMemoryLength_(&self, payloadMemoryLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPayloadMemoryLength : payloadMemoryLength)
    }
    unsafe fn maxTotalThreadgroupsPerMeshGrid(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadgroupsPerMeshGrid)
    }
    unsafe fn setMaxTotalThreadgroupsPerMeshGrid_(
        &self,
        maxTotalThreadgroupsPerMeshGrid: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadgroupsPerMeshGrid : maxTotalThreadgroupsPerMeshGrid)
    }
    unsafe fn objectBuffers(&self) -> MTLPipelineBufferDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectBuffers)
    }
    unsafe fn meshBuffers(&self) -> MTLPipelineBufferDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshBuffers)
    }
    unsafe fn fragmentBuffers(&self) -> MTLPipelineBufferDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentBuffers)
    }
    unsafe fn rasterSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rasterSampleCount)
    }
    unsafe fn setRasterSampleCount_(&self, rasterSampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterSampleCount : rasterSampleCount)
    }
    unsafe fn isAlphaToCoverageEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAlphaToCoverageEnabled)
    }
    unsafe fn setAlphaToCoverageEnabled_(&self, alphaToCoverageEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaToCoverageEnabled : alphaToCoverageEnabled)
    }
    unsafe fn isAlphaToOneEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAlphaToOneEnabled)
    }
    unsafe fn setAlphaToOneEnabled_(&self, alphaToOneEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaToOneEnabled : alphaToOneEnabled)
    }
    unsafe fn isRasterizationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRasterizationEnabled)
    }
    unsafe fn setRasterizationEnabled_(&self, rasterizationEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterizationEnabled : rasterizationEnabled)
    }
    unsafe fn maxVertexAmplificationCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxVertexAmplificationCount)
    }
    unsafe fn setMaxVertexAmplificationCount_(&self, maxVertexAmplificationCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxVertexAmplificationCount : maxVertexAmplificationCount)
    }
    unsafe fn colorAttachments(&self) -> MTLRenderPipelineColorAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachments)
    }
    unsafe fn depthAttachmentPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthAttachmentPixelFormat)
    }
    unsafe fn setDepthAttachmentPixelFormat_(&self, depthAttachmentPixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthAttachmentPixelFormat : depthAttachmentPixelFormat)
    }
    unsafe fn stencilAttachmentPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stencilAttachmentPixelFormat)
    }
    unsafe fn setStencilAttachmentPixelFormat_(&self, stencilAttachmentPixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilAttachmentPixelFormat : stencilAttachmentPixelFormat)
    }
    unsafe fn supportIndirectCommandBuffers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportIndirectCommandBuffers)
    }
    unsafe fn setSupportIndirectCommandBuffers_(&self, supportIndirectCommandBuffers: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportIndirectCommandBuffers : supportIndirectCommandBuffers)
    }
    unsafe fn binaryArchives(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, binaryArchives)
    }
    unsafe fn setBinaryArchives_(&self, binaryArchives: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBinaryArchives : binaryArchives)
    }
    unsafe fn objectLinkedFunctions(&self) -> MTLLinkedFunctions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectLinkedFunctions)
    }
    unsafe fn setObjectLinkedFunctions_(&self, objectLinkedFunctions: MTLLinkedFunctions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectLinkedFunctions : objectLinkedFunctions)
    }
    unsafe fn meshLinkedFunctions(&self) -> MTLLinkedFunctions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshLinkedFunctions)
    }
    unsafe fn setMeshLinkedFunctions_(&self, meshLinkedFunctions: MTLLinkedFunctions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshLinkedFunctions : meshLinkedFunctions)
    }
    unsafe fn fragmentLinkedFunctions(&self) -> MTLLinkedFunctions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentLinkedFunctions)
    }
    unsafe fn setFragmentLinkedFunctions_(&self, fragmentLinkedFunctions: MTLLinkedFunctions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentLinkedFunctions : fragmentLinkedFunctions)
    }
    unsafe fn shaderValidation(&self) -> MTLShaderValidation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shaderValidation)
    }
    unsafe fn setShaderValidation_(&self, shaderValidation: MTLShaderValidation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShaderValidation : shaderValidation)
    }
    unsafe fn requiredThreadsPerObjectThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerObjectThreadgroup)
    }
    unsafe fn setRequiredThreadsPerObjectThreadgroup_(
        &self,
        requiredThreadsPerObjectThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredThreadsPerObjectThreadgroup : requiredThreadsPerObjectThreadgroup)
    }
    unsafe fn requiredThreadsPerMeshThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerMeshThreadgroup)
    }
    unsafe fn setRequiredThreadsPerMeshThreadgroup_(
        &self,
        requiredThreadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredThreadsPerMeshThreadgroup : requiredThreadsPerMeshThreadgroup)
    }
}
pub trait PMTLParallelRenderCommandEncoder: Sized + std::ops::Deref {
    unsafe fn renderCommandEncoder(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderCommandEncoder)
    }
    unsafe fn setColorStoreAction_atIndex_(
        &self,
        storeAction: MTLStoreAction,
        colorAttachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorStoreAction : storeAction, atIndex : colorAttachmentIndex)
    }
    unsafe fn setDepthStoreAction_(&self, storeAction: MTLStoreAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStoreAction : storeAction)
    }
    unsafe fn setStencilStoreAction_(&self, storeAction: MTLStoreAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilStoreAction : storeAction)
    }
    unsafe fn setColorStoreActionOptions_atIndex_(
        &self,
        storeActionOptions: MTLStoreActionOptions,
        colorAttachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorStoreActionOptions : storeActionOptions, atIndex : colorAttachmentIndex)
    }
    unsafe fn setDepthStoreActionOptions_(&self, storeActionOptions: MTLStoreActionOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStoreActionOptions : storeActionOptions)
    }
    unsafe fn setStencilStoreActionOptions_(&self, storeActionOptions: MTLStoreActionOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilStoreActionOptions : storeActionOptions)
    }
}
pub type MTLSamplerMinMagFilter = NSUInteger;
pub type MTLSamplerMipFilter = NSUInteger;
pub type MTLSamplerAddressMode = NSUInteger;
pub type MTLSamplerBorderColor = NSUInteger;
pub type MTLSamplerReductionMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLSamplerDescriptor(pub id);
impl std::ops::Deref for MTLSamplerDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLSamplerDescriptor {}
impl MTLSamplerDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLSamplerDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLSamplerDescriptor {}
impl INSObject for MTLSamplerDescriptor {}
impl PNSObject for MTLSamplerDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLSamplerDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLSamplerDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLSamplerDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLSamplerDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLSamplerDescriptor")
        }
    }
}
impl IMTLSamplerDescriptor for MTLSamplerDescriptor {}
pub trait IMTLSamplerDescriptor: Sized + std::ops::Deref {
    unsafe fn minFilter(&self) -> MTLSamplerMinMagFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minFilter)
    }
    unsafe fn setMinFilter_(&self, minFilter: MTLSamplerMinMagFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinFilter : minFilter)
    }
    unsafe fn magFilter(&self) -> MTLSamplerMinMagFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magFilter)
    }
    unsafe fn setMagFilter_(&self, magFilter: MTLSamplerMinMagFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagFilter : magFilter)
    }
    unsafe fn mipFilter(&self) -> MTLSamplerMipFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mipFilter)
    }
    unsafe fn setMipFilter_(&self, mipFilter: MTLSamplerMipFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMipFilter : mipFilter)
    }
    unsafe fn maxAnisotropy(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxAnisotropy)
    }
    unsafe fn setMaxAnisotropy_(&self, maxAnisotropy: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxAnisotropy : maxAnisotropy)
    }
    unsafe fn sAddressMode(&self) -> MTLSamplerAddressMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sAddressMode)
    }
    unsafe fn setSAddressMode_(&self, sAddressMode: MTLSamplerAddressMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSAddressMode : sAddressMode)
    }
    unsafe fn tAddressMode(&self) -> MTLSamplerAddressMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tAddressMode)
    }
    unsafe fn setTAddressMode_(&self, tAddressMode: MTLSamplerAddressMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTAddressMode : tAddressMode)
    }
    unsafe fn rAddressMode(&self) -> MTLSamplerAddressMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rAddressMode)
    }
    unsafe fn setRAddressMode_(&self, rAddressMode: MTLSamplerAddressMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRAddressMode : rAddressMode)
    }
    unsafe fn borderColor(&self) -> MTLSamplerBorderColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderColor)
    }
    unsafe fn setBorderColor_(&self, borderColor: MTLSamplerBorderColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderColor : borderColor)
    }
    unsafe fn reductionMode(&self) -> MTLSamplerReductionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reductionMode)
    }
    unsafe fn setReductionMode_(&self, reductionMode: MTLSamplerReductionMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReductionMode : reductionMode)
    }
    unsafe fn normalizedCoordinates(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedCoordinates)
    }
    unsafe fn setNormalizedCoordinates_(&self, normalizedCoordinates: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalizedCoordinates : normalizedCoordinates)
    }
    unsafe fn lodMinClamp(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lodMinClamp)
    }
    unsafe fn setLodMinClamp_(&self, lodMinClamp: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLodMinClamp : lodMinClamp)
    }
    unsafe fn lodMaxClamp(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lodMaxClamp)
    }
    unsafe fn setLodMaxClamp_(&self, lodMaxClamp: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLodMaxClamp : lodMaxClamp)
    }
    unsafe fn lodAverage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lodAverage)
    }
    unsafe fn setLodAverage_(&self, lodAverage: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLodAverage : lodAverage)
    }
    unsafe fn lodBias(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lodBias)
    }
    unsafe fn setLodBias_(&self, lodBias: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLodBias : lodBias)
    }
    unsafe fn compareFunction(&self) -> MTLCompareFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compareFunction)
    }
    unsafe fn setCompareFunction_(&self, compareFunction: MTLCompareFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompareFunction : compareFunction)
    }
    unsafe fn supportArgumentBuffers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportArgumentBuffers)
    }
    unsafe fn setSupportArgumentBuffers_(&self, supportArgumentBuffers: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportArgumentBuffers : supportArgumentBuffers)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub trait PMTLSamplerState: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4BufferRange {
    pub bufferAddress: MTLGPUAddress,
    pub length: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _MTLPackedFloat3 {
    pub __bindgen_anon_1: _MTLPackedFloat3__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _MTLPackedFloat3__bindgen_ty_1 {
    pub __bindgen_anon_1: _MTLPackedFloat3__bindgen_ty_1__bindgen_ty_1,
    pub elements: [f32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MTLPackedFloat3__bindgen_ty_1__bindgen_ty_1 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub type MTLPackedFloat3 = _MTLPackedFloat3;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLPackedFloatQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _MTLPackedFloat4x3 {
    pub columns: [MTLPackedFloat3; 4usize],
}
pub type MTLPackedFloat4x3 = _MTLPackedFloat4x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _MTLAxisAlignedBoundingBox {
    pub min: MTLPackedFloat3,
    pub max: MTLPackedFloat3,
}
pub type MTLAxisAlignedBoundingBox = _MTLAxisAlignedBoundingBox;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLComponentTransform {
    pub scale: MTLPackedFloat3,
    pub shear: MTLPackedFloat3,
    pub pivot: MTLPackedFloat3,
    pub rotation: MTLPackedFloatQuaternion,
    pub translation: MTLPackedFloat3,
}
pub type MTLAccelerationStructureRefitOptions = NSUInteger;
pub type MTLAccelerationStructureUsage = NSUInteger;
pub type MTLAccelerationStructureInstanceOptions = u32;
pub type MTLMatrixLayout = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructureDescriptor {}
impl MTLAccelerationStructureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLAccelerationStructureDescriptor {}
impl INSObject for MTLAccelerationStructureDescriptor {}
impl PNSObject for MTLAccelerationStructureDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLAccelerationStructureDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLAccelerationStructureDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructureDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructureDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLAccelerationStructureDescriptor")
        }
    }
}
impl IMTLAccelerationStructureDescriptor for MTLAccelerationStructureDescriptor {}
pub trait IMTLAccelerationStructureDescriptor: Sized + std::ops::Deref {
    unsafe fn usage(&self) -> MTLAccelerationStructureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usage)
    }
    unsafe fn setUsage_(&self, usage: MTLAccelerationStructureUsage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsage : usage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureGeometryDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructureGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructureGeometryDescriptor {}
impl MTLAccelerationStructureGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureGeometryDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLAccelerationStructureGeometryDescriptor {}
impl INSObject for MTLAccelerationStructureGeometryDescriptor {}
impl PNSObject for MTLAccelerationStructureGeometryDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLAccelerationStructureGeometryDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLAccelerationStructureGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructureGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructureGeometryDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLAccelerationStructureGeometryDescriptor")
        }
    }
}
impl IMTLAccelerationStructureGeometryDescriptor for MTLAccelerationStructureGeometryDescriptor {}
pub trait IMTLAccelerationStructureGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn intersectionFunctionTableOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intersectionFunctionTableOffset)
    }
    unsafe fn setIntersectionFunctionTableOffset_(
        &self,
        intersectionFunctionTableOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntersectionFunctionTableOffset : intersectionFunctionTableOffset)
    }
    unsafe fn opaque(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opaque)
    }
    unsafe fn setOpaque_(&self, opaque: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpaque : opaque)
    }
    unsafe fn allowDuplicateIntersectionFunctionInvocation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowDuplicateIntersectionFunctionInvocation)
    }
    unsafe fn setAllowDuplicateIntersectionFunctionInvocation_(
        &self,
        allowDuplicateIntersectionFunctionInvocation: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowDuplicateIntersectionFunctionInvocation : allowDuplicateIntersectionFunctionInvocation)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn primitiveDataBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveDataBuffer)
    }
    unsafe fn setPrimitiveDataBuffer_(&self, primitiveDataBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimitiveDataBuffer : primitiveDataBuffer)
    }
    unsafe fn primitiveDataBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveDataBufferOffset)
    }
    unsafe fn setPrimitiveDataBufferOffset_(&self, primitiveDataBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimitiveDataBufferOffset : primitiveDataBufferOffset)
    }
    unsafe fn primitiveDataStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveDataStride)
    }
    unsafe fn setPrimitiveDataStride_(&self, primitiveDataStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimitiveDataStride : primitiveDataStride)
    }
    unsafe fn primitiveDataElementSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveDataElementSize)
    }
    unsafe fn setPrimitiveDataElementSize_(&self, primitiveDataElementSize: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimitiveDataElementSize : primitiveDataElementSize)
    }
}
pub type MTLMotionBorderMode = u32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLPrimitiveAccelerationStructureDescriptor(pub id);
impl std::ops::Deref for MTLPrimitiveAccelerationStructureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLPrimitiveAccelerationStructureDescriptor {}
impl MTLPrimitiveAccelerationStructureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLPrimitiveAccelerationStructureDescriptor").unwrap(), alloc) })
    }
}
impl IMTLAccelerationStructureDescriptor for MTLPrimitiveAccelerationStructureDescriptor {}
impl PNSCopying for MTLPrimitiveAccelerationStructureDescriptor {}
impl From<MTLPrimitiveAccelerationStructureDescriptor> for MTLAccelerationStructureDescriptor {
    fn from(
        child: MTLPrimitiveAccelerationStructureDescriptor,
    ) -> MTLAccelerationStructureDescriptor {
        MTLAccelerationStructureDescriptor(child.0)
    }
}
impl std::convert::TryFrom<MTLAccelerationStructureDescriptor>
    for MTLPrimitiveAccelerationStructureDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureDescriptor,
    ) -> Result<MTLPrimitiveAccelerationStructureDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLPrimitiveAccelerationStructureDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLPrimitiveAccelerationStructureDescriptor(parent.0))
        } else {
            Err ("This MTLAccelerationStructureDescriptor cannot be downcasted to MTLPrimitiveAccelerationStructureDescriptor" ,)
        }
    }
}
impl INSObject for MTLPrimitiveAccelerationStructureDescriptor {}
impl PNSObject for MTLPrimitiveAccelerationStructureDescriptor {}
impl IMTLPrimitiveAccelerationStructureDescriptor for MTLPrimitiveAccelerationStructureDescriptor {}
pub trait IMTLPrimitiveAccelerationStructureDescriptor: Sized + std::ops::Deref {
    unsafe fn geometryDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometryDescriptors)
    }
    unsafe fn setGeometryDescriptors_(&self, geometryDescriptors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeometryDescriptors : geometryDescriptors)
    }
    unsafe fn motionStartBorderMode(&self) -> MTLMotionBorderMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionStartBorderMode)
    }
    unsafe fn setMotionStartBorderMode_(&self, motionStartBorderMode: MTLMotionBorderMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionStartBorderMode : motionStartBorderMode)
    }
    unsafe fn motionEndBorderMode(&self) -> MTLMotionBorderMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionEndBorderMode)
    }
    unsafe fn setMotionEndBorderMode_(&self, motionEndBorderMode: MTLMotionBorderMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionEndBorderMode : motionEndBorderMode)
    }
    unsafe fn motionStartTime(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionStartTime)
    }
    unsafe fn setMotionStartTime_(&self, motionStartTime: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionStartTime : motionStartTime)
    }
    unsafe fn motionEndTime(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionEndTime)
    }
    unsafe fn setMotionEndTime_(&self, motionEndTime: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionEndTime : motionEndTime)
    }
    unsafe fn motionKeyframeCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionKeyframeCount)
    }
    unsafe fn setMotionKeyframeCount_(&self, motionKeyframeCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionKeyframeCount : motionKeyframeCount)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLPrimitiveAccelerationStructureDescriptor").unwrap(), descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureTriangleGeometryDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructureTriangleGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructureTriangleGeometryDescriptor {}
impl MTLAccelerationStructureTriangleGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureTriangleGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTLAccelerationStructureGeometryDescriptor
    for MTLAccelerationStructureTriangleGeometryDescriptor
{
}
impl PNSCopying for MTLAccelerationStructureTriangleGeometryDescriptor {}
impl From<MTLAccelerationStructureTriangleGeometryDescriptor>
    for MTLAccelerationStructureGeometryDescriptor
{
    fn from(
        child: MTLAccelerationStructureTriangleGeometryDescriptor,
    ) -> MTLAccelerationStructureGeometryDescriptor {
        MTLAccelerationStructureGeometryDescriptor(child.0)
    }
}
impl std::convert::TryFrom<MTLAccelerationStructureGeometryDescriptor>
    for MTLAccelerationStructureTriangleGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureGeometryDescriptor,
    ) -> Result<MTLAccelerationStructureTriangleGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructureTriangleGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructureTriangleGeometryDescriptor(parent.0))
        } else {
            Err ("This MTLAccelerationStructureGeometryDescriptor cannot be downcasted to MTLAccelerationStructureTriangleGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTLAccelerationStructureTriangleGeometryDescriptor {}
impl PNSObject for MTLAccelerationStructureTriangleGeometryDescriptor {}
impl IMTLAccelerationStructureTriangleGeometryDescriptor
    for MTLAccelerationStructureTriangleGeometryDescriptor
{
}
pub trait IMTLAccelerationStructureTriangleGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn vertexBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexBuffer)
    }
    unsafe fn setVertexBuffer_(&self, vertexBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffer : vertexBuffer)
    }
    unsafe fn vertexBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexBufferOffset)
    }
    unsafe fn setVertexBufferOffset_(&self, vertexBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBufferOffset : vertexBufferOffset)
    }
    unsafe fn vertexFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexFormat)
    }
    unsafe fn setVertexFormat_(&self, vertexFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexFormat : vertexFormat)
    }
    unsafe fn vertexStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexStride)
    }
    unsafe fn setVertexStride_(&self, vertexStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexStride : vertexStride)
    }
    unsafe fn indexBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn setIndexBuffer_(&self, indexBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBuffer : indexBuffer)
    }
    unsafe fn indexBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBufferOffset)
    }
    unsafe fn setIndexBufferOffset_(&self, indexBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBufferOffset : indexBufferOffset)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn setIndexType_(&self, indexType: MTLIndexType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexType : indexType)
    }
    unsafe fn triangleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triangleCount)
    }
    unsafe fn setTriangleCount_(&self, triangleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTriangleCount : triangleCount)
    }
    unsafe fn transformationMatrixBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixBuffer)
    }
    unsafe fn setTransformationMatrixBuffer_(&self, transformationMatrixBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixBuffer : transformationMatrixBuffer)
    }
    unsafe fn transformationMatrixBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixBufferOffset)
    }
    unsafe fn setTransformationMatrixBufferOffset_(
        &self,
        transformationMatrixBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixBufferOffset : transformationMatrixBufferOffset)
    }
    unsafe fn transformationMatrixLayout(&self) -> MTLMatrixLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixLayout)
    }
    unsafe fn setTransformationMatrixLayout_(&self, transformationMatrixLayout: MTLMatrixLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixLayout : transformationMatrixLayout)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureTriangleGeometryDescriptor").unwrap(), descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureBoundingBoxGeometryDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructureBoundingBoxGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructureBoundingBoxGeometryDescriptor {}
impl MTLAccelerationStructureBoundingBoxGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureBoundingBoxGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTLAccelerationStructureGeometryDescriptor
    for MTLAccelerationStructureBoundingBoxGeometryDescriptor
{
}
impl PNSCopying for MTLAccelerationStructureBoundingBoxGeometryDescriptor {}
impl std::convert::TryFrom<MTLAccelerationStructureGeometryDescriptor>
    for MTLAccelerationStructureBoundingBoxGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureGeometryDescriptor,
    ) -> Result<MTLAccelerationStructureBoundingBoxGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructureBoundingBoxGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructureBoundingBoxGeometryDescriptor(
                parent.0,
            ))
        } else {
            Err ("This MTLAccelerationStructureGeometryDescriptor cannot be downcasted to MTLAccelerationStructureBoundingBoxGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTLAccelerationStructureBoundingBoxGeometryDescriptor {}
impl PNSObject for MTLAccelerationStructureBoundingBoxGeometryDescriptor {}
impl IMTLAccelerationStructureBoundingBoxGeometryDescriptor
    for MTLAccelerationStructureBoundingBoxGeometryDescriptor
{
}
pub trait IMTLAccelerationStructureBoundingBoxGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn boundingBoxBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxBuffer)
    }
    unsafe fn setBoundingBoxBuffer_(&self, boundingBoxBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxBuffer : boundingBoxBuffer)
    }
    unsafe fn boundingBoxBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxBufferOffset)
    }
    unsafe fn setBoundingBoxBufferOffset_(&self, boundingBoxBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxBufferOffset : boundingBoxBufferOffset)
    }
    unsafe fn boundingBoxStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxStride)
    }
    unsafe fn setBoundingBoxStride_(&self, boundingBoxStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxStride : boundingBoxStride)
    }
    unsafe fn boundingBoxCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxCount)
    }
    unsafe fn setBoundingBoxCount_(&self, boundingBoxCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxCount : boundingBoxCount)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureBoundingBoxGeometryDescriptor").unwrap(), descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLMotionKeyframeData(pub id);
impl std::ops::Deref for MTLMotionKeyframeData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLMotionKeyframeData {}
impl MTLMotionKeyframeData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLMotionKeyframeData").unwrap(), alloc) })
    }
}
impl INSObject for MTLMotionKeyframeData {}
impl PNSObject for MTLMotionKeyframeData {}
impl std::convert::TryFrom<NSObject> for MTLMotionKeyframeData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLMotionKeyframeData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLMotionKeyframeData").unwrap()) };
        if is_kind_of {
            Ok(MTLMotionKeyframeData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLMotionKeyframeData")
        }
    }
}
impl IMTLMotionKeyframeData for MTLMotionKeyframeData {}
pub trait IMTLMotionKeyframeData: Sized + std::ops::Deref {
    unsafe fn buffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buffer)
    }
    unsafe fn setBuffer_(&self, buffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBuffer : buffer)
    }
    unsafe fn offset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn setOffset_(&self, offset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffset : offset)
    }
    unsafe fn data() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLMotionKeyframeData").unwrap(), data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureMotionTriangleGeometryDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructureMotionTriangleGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructureMotionTriangleGeometryDescriptor {}
impl MTLAccelerationStructureMotionTriangleGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureMotionTriangleGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTLAccelerationStructureGeometryDescriptor
    for MTLAccelerationStructureMotionTriangleGeometryDescriptor
{
}
impl PNSCopying for MTLAccelerationStructureMotionTriangleGeometryDescriptor {}
impl std::convert::TryFrom<MTLAccelerationStructureGeometryDescriptor>
    for MTLAccelerationStructureMotionTriangleGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureGeometryDescriptor,
    ) -> Result<MTLAccelerationStructureMotionTriangleGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructureMotionTriangleGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructureMotionTriangleGeometryDescriptor(
                parent.0,
            ))
        } else {
            Err ("This MTLAccelerationStructureGeometryDescriptor cannot be downcasted to MTLAccelerationStructureMotionTriangleGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTLAccelerationStructureMotionTriangleGeometryDescriptor {}
impl PNSObject for MTLAccelerationStructureMotionTriangleGeometryDescriptor {}
impl IMTLAccelerationStructureMotionTriangleGeometryDescriptor
    for MTLAccelerationStructureMotionTriangleGeometryDescriptor
{
}
pub trait IMTLAccelerationStructureMotionTriangleGeometryDescriptor:
    Sized + std::ops::Deref
{
    unsafe fn vertexBuffers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexBuffers)
    }
    unsafe fn setVertexBuffers_(&self, vertexBuffers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffers : vertexBuffers)
    }
    unsafe fn vertexFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexFormat)
    }
    unsafe fn setVertexFormat_(&self, vertexFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexFormat : vertexFormat)
    }
    unsafe fn vertexStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexStride)
    }
    unsafe fn setVertexStride_(&self, vertexStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexStride : vertexStride)
    }
    unsafe fn indexBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn setIndexBuffer_(&self, indexBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBuffer : indexBuffer)
    }
    unsafe fn indexBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBufferOffset)
    }
    unsafe fn setIndexBufferOffset_(&self, indexBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBufferOffset : indexBufferOffset)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn setIndexType_(&self, indexType: MTLIndexType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexType : indexType)
    }
    unsafe fn triangleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triangleCount)
    }
    unsafe fn setTriangleCount_(&self, triangleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTriangleCount : triangleCount)
    }
    unsafe fn transformationMatrixBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixBuffer)
    }
    unsafe fn setTransformationMatrixBuffer_(&self, transformationMatrixBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixBuffer : transformationMatrixBuffer)
    }
    unsafe fn transformationMatrixBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixBufferOffset)
    }
    unsafe fn setTransformationMatrixBufferOffset_(
        &self,
        transformationMatrixBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixBufferOffset : transformationMatrixBufferOffset)
    }
    unsafe fn transformationMatrixLayout(&self) -> MTLMatrixLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixLayout)
    }
    unsafe fn setTransformationMatrixLayout_(&self, transformationMatrixLayout: MTLMatrixLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixLayout : transformationMatrixLayout)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureMotionTriangleGeometryDescriptor").unwrap(), descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {}
impl MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTLAccelerationStructureGeometryDescriptor
    for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor
{
}
impl PNSCopying for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {}
impl std::convert::TryFrom<MTLAccelerationStructureGeometryDescriptor>
    for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureGeometryDescriptor,
    ) -> Result<MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor(
                parent.0,
            ))
        } else {
            Err ("This MTLAccelerationStructureGeometryDescriptor cannot be downcasted to MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {}
impl PNSObject for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {}
impl IMTLAccelerationStructureMotionBoundingBoxGeometryDescriptor
    for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor
{
}
pub trait IMTLAccelerationStructureMotionBoundingBoxGeometryDescriptor:
    Sized + std::ops::Deref
{
    unsafe fn boundingBoxBuffers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxBuffers)
    }
    unsafe fn setBoundingBoxBuffers_(&self, boundingBoxBuffers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxBuffers : boundingBoxBuffers)
    }
    unsafe fn boundingBoxStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxStride)
    }
    unsafe fn setBoundingBoxStride_(&self, boundingBoxStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxStride : boundingBoxStride)
    }
    unsafe fn boundingBoxCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxCount)
    }
    unsafe fn setBoundingBoxCount_(&self, boundingBoxCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxCount : boundingBoxCount)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor").unwrap(), descriptor)
    }
}
pub type MTLCurveType = NSInteger;
pub type MTLCurveBasis = NSInteger;
pub type MTLCurveEndCaps = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureCurveGeometryDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructureCurveGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructureCurveGeometryDescriptor {}
impl MTLAccelerationStructureCurveGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureCurveGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTLAccelerationStructureGeometryDescriptor
    for MTLAccelerationStructureCurveGeometryDescriptor
{
}
impl PNSCopying for MTLAccelerationStructureCurveGeometryDescriptor {}
impl std::convert::TryFrom<MTLAccelerationStructureGeometryDescriptor>
    for MTLAccelerationStructureCurveGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureGeometryDescriptor,
    ) -> Result<MTLAccelerationStructureCurveGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructureCurveGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructureCurveGeometryDescriptor(parent.0))
        } else {
            Err ("This MTLAccelerationStructureGeometryDescriptor cannot be downcasted to MTLAccelerationStructureCurveGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTLAccelerationStructureCurveGeometryDescriptor {}
impl PNSObject for MTLAccelerationStructureCurveGeometryDescriptor {}
impl IMTLAccelerationStructureCurveGeometryDescriptor
    for MTLAccelerationStructureCurveGeometryDescriptor
{
}
pub trait IMTLAccelerationStructureCurveGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn controlPointBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointBuffer)
    }
    unsafe fn setControlPointBuffer_(&self, controlPointBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointBuffer : controlPointBuffer)
    }
    unsafe fn controlPointBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointBufferOffset)
    }
    unsafe fn setControlPointBufferOffset_(&self, controlPointBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointBufferOffset : controlPointBufferOffset)
    }
    unsafe fn controlPointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointCount)
    }
    unsafe fn setControlPointCount_(&self, controlPointCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointCount : controlPointCount)
    }
    unsafe fn controlPointStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointStride)
    }
    unsafe fn setControlPointStride_(&self, controlPointStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointStride : controlPointStride)
    }
    unsafe fn controlPointFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointFormat)
    }
    unsafe fn setControlPointFormat_(&self, controlPointFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointFormat : controlPointFormat)
    }
    unsafe fn radiusBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusBuffer)
    }
    unsafe fn setRadiusBuffer_(&self, radiusBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusBuffer : radiusBuffer)
    }
    unsafe fn radiusBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusBufferOffset)
    }
    unsafe fn setRadiusBufferOffset_(&self, radiusBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusBufferOffset : radiusBufferOffset)
    }
    unsafe fn radiusFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusFormat)
    }
    unsafe fn setRadiusFormat_(&self, radiusFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusFormat : radiusFormat)
    }
    unsafe fn radiusStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusStride)
    }
    unsafe fn setRadiusStride_(&self, radiusStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusStride : radiusStride)
    }
    unsafe fn indexBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn setIndexBuffer_(&self, indexBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBuffer : indexBuffer)
    }
    unsafe fn indexBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBufferOffset)
    }
    unsafe fn setIndexBufferOffset_(&self, indexBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBufferOffset : indexBufferOffset)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn setIndexType_(&self, indexType: MTLIndexType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexType : indexType)
    }
    unsafe fn segmentCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentCount)
    }
    unsafe fn setSegmentCount_(&self, segmentCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSegmentCount : segmentCount)
    }
    unsafe fn segmentControlPointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentControlPointCount)
    }
    unsafe fn setSegmentControlPointCount_(&self, segmentControlPointCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSegmentControlPointCount : segmentControlPointCount)
    }
    unsafe fn curveType(&self) -> MTLCurveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveType)
    }
    unsafe fn setCurveType_(&self, curveType: MTLCurveType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveType : curveType)
    }
    unsafe fn curveBasis(&self) -> MTLCurveBasis
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveBasis)
    }
    unsafe fn setCurveBasis_(&self, curveBasis: MTLCurveBasis)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveBasis : curveBasis)
    }
    unsafe fn curveEndCaps(&self) -> MTLCurveEndCaps
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveEndCaps)
    }
    unsafe fn setCurveEndCaps_(&self, curveEndCaps: MTLCurveEndCaps)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveEndCaps : curveEndCaps)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureCurveGeometryDescriptor").unwrap(), descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureMotionCurveGeometryDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructureMotionCurveGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructureMotionCurveGeometryDescriptor {}
impl MTLAccelerationStructureMotionCurveGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureMotionCurveGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTLAccelerationStructureGeometryDescriptor
    for MTLAccelerationStructureMotionCurveGeometryDescriptor
{
}
impl PNSCopying for MTLAccelerationStructureMotionCurveGeometryDescriptor {}
impl std::convert::TryFrom<MTLAccelerationStructureGeometryDescriptor>
    for MTLAccelerationStructureMotionCurveGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureGeometryDescriptor,
    ) -> Result<MTLAccelerationStructureMotionCurveGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructureMotionCurveGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructureMotionCurveGeometryDescriptor(
                parent.0,
            ))
        } else {
            Err ("This MTLAccelerationStructureGeometryDescriptor cannot be downcasted to MTLAccelerationStructureMotionCurveGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTLAccelerationStructureMotionCurveGeometryDescriptor {}
impl PNSObject for MTLAccelerationStructureMotionCurveGeometryDescriptor {}
impl IMTLAccelerationStructureMotionCurveGeometryDescriptor
    for MTLAccelerationStructureMotionCurveGeometryDescriptor
{
}
pub trait IMTLAccelerationStructureMotionCurveGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn controlPointBuffers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointBuffers)
    }
    unsafe fn setControlPointBuffers_(&self, controlPointBuffers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointBuffers : controlPointBuffers)
    }
    unsafe fn controlPointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointCount)
    }
    unsafe fn setControlPointCount_(&self, controlPointCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointCount : controlPointCount)
    }
    unsafe fn controlPointStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointStride)
    }
    unsafe fn setControlPointStride_(&self, controlPointStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointStride : controlPointStride)
    }
    unsafe fn controlPointFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointFormat)
    }
    unsafe fn setControlPointFormat_(&self, controlPointFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointFormat : controlPointFormat)
    }
    unsafe fn radiusBuffers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusBuffers)
    }
    unsafe fn setRadiusBuffers_(&self, radiusBuffers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusBuffers : radiusBuffers)
    }
    unsafe fn radiusFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusFormat)
    }
    unsafe fn setRadiusFormat_(&self, radiusFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusFormat : radiusFormat)
    }
    unsafe fn radiusStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusStride)
    }
    unsafe fn setRadiusStride_(&self, radiusStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusStride : radiusStride)
    }
    unsafe fn indexBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn setIndexBuffer_(&self, indexBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBuffer : indexBuffer)
    }
    unsafe fn indexBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBufferOffset)
    }
    unsafe fn setIndexBufferOffset_(&self, indexBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBufferOffset : indexBufferOffset)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn setIndexType_(&self, indexType: MTLIndexType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexType : indexType)
    }
    unsafe fn segmentCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentCount)
    }
    unsafe fn setSegmentCount_(&self, segmentCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSegmentCount : segmentCount)
    }
    unsafe fn segmentControlPointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentControlPointCount)
    }
    unsafe fn setSegmentControlPointCount_(&self, segmentControlPointCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSegmentControlPointCount : segmentControlPointCount)
    }
    unsafe fn curveType(&self) -> MTLCurveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveType)
    }
    unsafe fn setCurveType_(&self, curveType: MTLCurveType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveType : curveType)
    }
    unsafe fn curveBasis(&self) -> MTLCurveBasis
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveBasis)
    }
    unsafe fn setCurveBasis_(&self, curveBasis: MTLCurveBasis)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveBasis : curveBasis)
    }
    unsafe fn curveEndCaps(&self) -> MTLCurveEndCaps
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveEndCaps)
    }
    unsafe fn setCurveEndCaps_(&self, curveEndCaps: MTLCurveEndCaps)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveEndCaps : curveEndCaps)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructureMotionCurveGeometryDescriptor").unwrap(), descriptor)
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLAccelerationStructureInstanceDescriptor {
    pub transformationMatrix: MTLPackedFloat4x3,
    pub options: MTLAccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersectionFunctionTableOffset: u32,
    pub accelerationStructureIndex: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLAccelerationStructureUserIDInstanceDescriptor {
    pub transformationMatrix: MTLPackedFloat4x3,
    pub options: MTLAccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersectionFunctionTableOffset: u32,
    pub accelerationStructureIndex: u32,
    pub userID: u32,
}
pub type MTLAccelerationStructureInstanceDescriptorType = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructureMotionInstanceDescriptor {
    pub options: MTLAccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersectionFunctionTableOffset: u32,
    pub accelerationStructureIndex: u32,
    pub userID: u32,
    pub motionTransformsStartIndex: u32,
    pub motionTransformsCount: u32,
    pub motionStartBorderMode: MTLMotionBorderMode,
    pub motionEndBorderMode: MTLMotionBorderMode,
    pub motionStartTime: f32,
    pub motionEndTime: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLIndirectAccelerationStructureInstanceDescriptor {
    pub transformationMatrix: MTLPackedFloat4x3,
    pub options: MTLAccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersectionFunctionTableOffset: u32,
    pub userID: u32,
    pub accelerationStructureID: MTLResourceID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLIndirectAccelerationStructureMotionInstanceDescriptor {
    pub options: MTLAccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersectionFunctionTableOffset: u32,
    pub userID: u32,
    pub accelerationStructureID: MTLResourceID,
    pub motionTransformsStartIndex: u32,
    pub motionTransformsCount: u32,
    pub motionStartBorderMode: MTLMotionBorderMode,
    pub motionEndBorderMode: MTLMotionBorderMode,
    pub motionStartTime: f32,
    pub motionEndTime: f32,
}
pub type MTLTransformType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLInstanceAccelerationStructureDescriptor(pub id);
impl std::ops::Deref for MTLInstanceAccelerationStructureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLInstanceAccelerationStructureDescriptor {}
impl MTLInstanceAccelerationStructureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLInstanceAccelerationStructureDescriptor").unwrap(), alloc) })
    }
}
impl IMTLAccelerationStructureDescriptor for MTLInstanceAccelerationStructureDescriptor {}
impl PNSCopying for MTLInstanceAccelerationStructureDescriptor {}
impl std::convert::TryFrom<MTLAccelerationStructureDescriptor>
    for MTLInstanceAccelerationStructureDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureDescriptor,
    ) -> Result<MTLInstanceAccelerationStructureDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLInstanceAccelerationStructureDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLInstanceAccelerationStructureDescriptor(parent.0))
        } else {
            Err ("This MTLAccelerationStructureDescriptor cannot be downcasted to MTLInstanceAccelerationStructureDescriptor" ,)
        }
    }
}
impl INSObject for MTLInstanceAccelerationStructureDescriptor {}
impl PNSObject for MTLInstanceAccelerationStructureDescriptor {}
impl IMTLInstanceAccelerationStructureDescriptor for MTLInstanceAccelerationStructureDescriptor {}
pub trait IMTLInstanceAccelerationStructureDescriptor: Sized + std::ops::Deref {
    unsafe fn instanceDescriptorBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorBuffer)
    }
    unsafe fn setInstanceDescriptorBuffer_(&self, instanceDescriptorBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorBuffer : instanceDescriptorBuffer)
    }
    unsafe fn instanceDescriptorBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorBufferOffset)
    }
    unsafe fn setInstanceDescriptorBufferOffset_(&self, instanceDescriptorBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorBufferOffset : instanceDescriptorBufferOffset)
    }
    unsafe fn instanceDescriptorStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorStride)
    }
    unsafe fn setInstanceDescriptorStride_(&self, instanceDescriptorStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorStride : instanceDescriptorStride)
    }
    unsafe fn instanceCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceCount)
    }
    unsafe fn setInstanceCount_(&self, instanceCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceCount : instanceCount)
    }
    unsafe fn instancedAccelerationStructures(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instancedAccelerationStructures)
    }
    unsafe fn setInstancedAccelerationStructures_(&self, instancedAccelerationStructures: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstancedAccelerationStructures : instancedAccelerationStructures)
    }
    unsafe fn instanceDescriptorType(&self) -> MTLAccelerationStructureInstanceDescriptorType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorType)
    }
    unsafe fn setInstanceDescriptorType_(
        &self,
        instanceDescriptorType: MTLAccelerationStructureInstanceDescriptorType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorType : instanceDescriptorType)
    }
    unsafe fn motionTransformBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformBuffer)
    }
    unsafe fn setMotionTransformBuffer_(&self, motionTransformBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformBuffer : motionTransformBuffer)
    }
    unsafe fn motionTransformBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformBufferOffset)
    }
    unsafe fn setMotionTransformBufferOffset_(&self, motionTransformBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformBufferOffset : motionTransformBufferOffset)
    }
    unsafe fn motionTransformCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformCount)
    }
    unsafe fn setMotionTransformCount_(&self, motionTransformCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformCount : motionTransformCount)
    }
    unsafe fn instanceTransformationMatrixLayout(&self) -> MTLMatrixLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceTransformationMatrixLayout)
    }
    unsafe fn setInstanceTransformationMatrixLayout_(
        &self,
        instanceTransformationMatrixLayout: MTLMatrixLayout,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceTransformationMatrixLayout : instanceTransformationMatrixLayout)
    }
    unsafe fn motionTransformType(&self) -> MTLTransformType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformType)
    }
    unsafe fn setMotionTransformType_(&self, motionTransformType: MTLTransformType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformType : motionTransformType)
    }
    unsafe fn motionTransformStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformStride)
    }
    unsafe fn setMotionTransformStride_(&self, motionTransformStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformStride : motionTransformStride)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLInstanceAccelerationStructureDescriptor").unwrap(), descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLIndirectInstanceAccelerationStructureDescriptor(pub id);
impl std::ops::Deref for MTLIndirectInstanceAccelerationStructureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLIndirectInstanceAccelerationStructureDescriptor {}
impl MTLIndirectInstanceAccelerationStructureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLIndirectInstanceAccelerationStructureDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTLAccelerationStructureDescriptor for MTLIndirectInstanceAccelerationStructureDescriptor {}
impl PNSCopying for MTLIndirectInstanceAccelerationStructureDescriptor {}
impl std::convert::TryFrom<MTLAccelerationStructureDescriptor>
    for MTLIndirectInstanceAccelerationStructureDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureDescriptor,
    ) -> Result<MTLIndirectInstanceAccelerationStructureDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLIndirectInstanceAccelerationStructureDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLIndirectInstanceAccelerationStructureDescriptor(parent.0))
        } else {
            Err ("This MTLAccelerationStructureDescriptor cannot be downcasted to MTLIndirectInstanceAccelerationStructureDescriptor" ,)
        }
    }
}
impl INSObject for MTLIndirectInstanceAccelerationStructureDescriptor {}
impl PNSObject for MTLIndirectInstanceAccelerationStructureDescriptor {}
impl IMTLIndirectInstanceAccelerationStructureDescriptor
    for MTLIndirectInstanceAccelerationStructureDescriptor
{
}
pub trait IMTLIndirectInstanceAccelerationStructureDescriptor: Sized + std::ops::Deref {
    unsafe fn instanceDescriptorBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorBuffer)
    }
    unsafe fn setInstanceDescriptorBuffer_(&self, instanceDescriptorBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorBuffer : instanceDescriptorBuffer)
    }
    unsafe fn instanceDescriptorBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorBufferOffset)
    }
    unsafe fn setInstanceDescriptorBufferOffset_(&self, instanceDescriptorBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorBufferOffset : instanceDescriptorBufferOffset)
    }
    unsafe fn instanceDescriptorStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorStride)
    }
    unsafe fn setInstanceDescriptorStride_(&self, instanceDescriptorStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorStride : instanceDescriptorStride)
    }
    unsafe fn maxInstanceCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxInstanceCount)
    }
    unsafe fn setMaxInstanceCount_(&self, maxInstanceCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxInstanceCount : maxInstanceCount)
    }
    unsafe fn instanceCountBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceCountBuffer)
    }
    unsafe fn setInstanceCountBuffer_(&self, instanceCountBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceCountBuffer : instanceCountBuffer)
    }
    unsafe fn instanceCountBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceCountBufferOffset)
    }
    unsafe fn setInstanceCountBufferOffset_(&self, instanceCountBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceCountBufferOffset : instanceCountBufferOffset)
    }
    unsafe fn instanceDescriptorType(&self) -> MTLAccelerationStructureInstanceDescriptorType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorType)
    }
    unsafe fn setInstanceDescriptorType_(
        &self,
        instanceDescriptorType: MTLAccelerationStructureInstanceDescriptorType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorType : instanceDescriptorType)
    }
    unsafe fn motionTransformBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformBuffer)
    }
    unsafe fn setMotionTransformBuffer_(&self, motionTransformBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformBuffer : motionTransformBuffer)
    }
    unsafe fn motionTransformBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformBufferOffset)
    }
    unsafe fn setMotionTransformBufferOffset_(&self, motionTransformBufferOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformBufferOffset : motionTransformBufferOffset)
    }
    unsafe fn maxMotionTransformCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxMotionTransformCount)
    }
    unsafe fn setMaxMotionTransformCount_(&self, maxMotionTransformCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxMotionTransformCount : maxMotionTransformCount)
    }
    unsafe fn motionTransformCountBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformCountBuffer)
    }
    unsafe fn setMotionTransformCountBuffer_(&self, motionTransformCountBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformCountBuffer : motionTransformCountBuffer)
    }
    unsafe fn motionTransformCountBufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformCountBufferOffset)
    }
    unsafe fn setMotionTransformCountBufferOffset_(
        &self,
        motionTransformCountBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformCountBufferOffset : motionTransformCountBufferOffset)
    }
    unsafe fn instanceTransformationMatrixLayout(&self) -> MTLMatrixLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceTransformationMatrixLayout)
    }
    unsafe fn setInstanceTransformationMatrixLayout_(
        &self,
        instanceTransformationMatrixLayout: MTLMatrixLayout,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceTransformationMatrixLayout : instanceTransformationMatrixLayout)
    }
    unsafe fn motionTransformType(&self) -> MTLTransformType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformType)
    }
    unsafe fn setMotionTransformType_(&self, motionTransformType: MTLTransformType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformType : motionTransformType)
    }
    unsafe fn motionTransformStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformStride)
    }
    unsafe fn setMotionTransformStride_(&self, motionTransformStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformStride : motionTransformStride)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLIndirectInstanceAccelerationStructureDescriptor").unwrap(), descriptor)
    }
}
pub trait PMTLAccelerationStructure: Sized + std::ops::Deref {
    unsafe fn size(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
}
pub type MTLHeapType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLHeapDescriptor(pub id);
impl std::ops::Deref for MTLHeapDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLHeapDescriptor {}
impl MTLHeapDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLHeapDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLHeapDescriptor {}
impl INSObject for MTLHeapDescriptor {}
impl PNSObject for MTLHeapDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLHeapDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLHeapDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLHeapDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLHeapDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLHeapDescriptor")
        }
    }
}
impl IMTLHeapDescriptor for MTLHeapDescriptor {}
pub trait IMTLHeapDescriptor: Sized + std::ops::Deref {
    unsafe fn size(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn storageMode(&self) -> MTLStorageMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storageMode)
    }
    unsafe fn setStorageMode_(&self, storageMode: MTLStorageMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStorageMode : storageMode)
    }
    unsafe fn cpuCacheMode(&self) -> MTLCPUCacheMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cpuCacheMode)
    }
    unsafe fn setCpuCacheMode_(&self, cpuCacheMode: MTLCPUCacheMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCpuCacheMode : cpuCacheMode)
    }
    unsafe fn sparsePageSize(&self) -> MTLSparsePageSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sparsePageSize)
    }
    unsafe fn setSparsePageSize_(&self, sparsePageSize: MTLSparsePageSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSparsePageSize : sparsePageSize)
    }
    unsafe fn hazardTrackingMode(&self) -> MTLHazardTrackingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hazardTrackingMode)
    }
    unsafe fn setHazardTrackingMode_(&self, hazardTrackingMode: MTLHazardTrackingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHazardTrackingMode : hazardTrackingMode)
    }
    unsafe fn resourceOptions(&self) -> MTLResourceOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceOptions)
    }
    unsafe fn setResourceOptions_(&self, resourceOptions: MTLResourceOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResourceOptions : resourceOptions)
    }
    unsafe fn type_(&self) -> MTLHeapType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: MTLHeapType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn maxCompatiblePlacementSparsePageSize(&self) -> MTLSparsePageSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxCompatiblePlacementSparsePageSize)
    }
    unsafe fn setMaxCompatiblePlacementSparsePageSize_(
        &self,
        maxCompatiblePlacementSparsePageSize: MTLSparsePageSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxCompatiblePlacementSparsePageSize : maxCompatiblePlacementSparsePageSize)
    }
}
pub trait PMTLHeap: Sized + std::ops::Deref {
    unsafe fn maxAvailableSizeWithAlignment_(&self, alignment: NSUInteger) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxAvailableSizeWithAlignment : alignment)
    }
    unsafe fn newBufferWithLength_options_(
        &self,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBufferWithLength : length, options : options)
    }
    unsafe fn newTextureWithDescriptor_(&self, descriptor: MTLTextureDescriptor) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithDescriptor : descriptor)
    }
    unsafe fn setPurgeableState_(&self, state: MTLPurgeableState) -> MTLPurgeableState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPurgeableState : state)
    }
    unsafe fn newBufferWithLength_options_offset_(
        &self,
        length: NSUInteger,
        options: MTLResourceOptions,
        offset: NSUInteger,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBufferWithLength : length, options : options, offset : offset)
    }
    unsafe fn newTextureWithDescriptor_offset_(
        &self,
        descriptor: MTLTextureDescriptor,
        offset: NSUInteger,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithDescriptor : descriptor, offset : offset)
    }
    unsafe fn newAccelerationStructureWithSize_(&self, size: NSUInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newAccelerationStructureWithSize : size)
    }
    unsafe fn newAccelerationStructureWithDescriptor_(
        &self,
        descriptor: MTLAccelerationStructureDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newAccelerationStructureWithDescriptor : descriptor)
    }
    unsafe fn newAccelerationStructureWithSize_offset_(
        &self,
        size: NSUInteger,
        offset: NSUInteger,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newAccelerationStructureWithSize : size, offset : offset)
    }
    unsafe fn newAccelerationStructureWithDescriptor_offset_(
        &self,
        descriptor: MTLAccelerationStructureDescriptor,
        offset: NSUInteger,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newAccelerationStructureWithDescriptor : descriptor, offset : offset)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn storageMode(&self) -> MTLStorageMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storageMode)
    }
    unsafe fn cpuCacheMode(&self) -> MTLCPUCacheMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cpuCacheMode)
    }
    unsafe fn hazardTrackingMode(&self) -> MTLHazardTrackingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hazardTrackingMode)
    }
    unsafe fn resourceOptions(&self) -> MTLResourceOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceOptions)
    }
    unsafe fn size(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn usedSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usedSize)
    }
    unsafe fn currentAllocatedSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentAllocatedSize)
    }
    unsafe fn type_(&self) -> MTLHeapType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
pub trait PMTLArgumentEncoder: Sized + std::ops::Deref {
    unsafe fn setArgumentBuffer_offset_(&self, argumentBuffer: *mut u64, offset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArgumentBuffer : argumentBuffer, offset : offset)
    }
    unsafe fn setArgumentBuffer_startOffset_arrayElement_(
        &self,
        argumentBuffer: *mut u64,
        startOffset: NSUInteger,
        arrayElement: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArgumentBuffer : argumentBuffer, startOffset : startOffset, arrayElement : arrayElement)
    }
    unsafe fn setBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setBuffers_offsets_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBuffers : buffers, offsets : offsets, withRange : range)
    }
    unsafe fn setTexture_atIndex_(&self, texture: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTexture : texture, atIndex : index)
    }
    unsafe fn setTextures_withRange_(&self, textures: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextures : textures, withRange : range)
    }
    unsafe fn setSamplerState_atIndex_(&self, sampler: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplerState : sampler, atIndex : index)
    }
    unsafe fn setSamplerStates_withRange_(&self, samplers: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplerStates : samplers, withRange : range)
    }
    unsafe fn constantDataAtIndex_(&self, index: NSUInteger) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constantDataAtIndex : index)
    }
    unsafe fn setRenderPipelineState_atIndex_(&self, pipeline: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderPipelineState : pipeline, atIndex : index)
    }
    unsafe fn setRenderPipelineStates_withRange_(&self, pipelines: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderPipelineStates : pipelines, withRange : range)
    }
    unsafe fn setComputePipelineState_atIndex_(&self, pipeline: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputePipelineState : pipeline, atIndex : index)
    }
    unsafe fn setComputePipelineStates_withRange_(&self, pipelines: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputePipelineStates : pipelines, withRange : range)
    }
    unsafe fn setIndirectCommandBuffer_atIndex_(
        &self,
        indirectCommandBuffer: *mut u64,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndirectCommandBuffer : indirectCommandBuffer, atIndex : index)
    }
    unsafe fn setIndirectCommandBuffers_withRange_(&self, buffers: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndirectCommandBuffers : buffers, withRange : range)
    }
    unsafe fn setAccelerationStructure_atIndex_(
        &self,
        accelerationStructure: *mut u64,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccelerationStructure : accelerationStructure, atIndex : index)
    }
    unsafe fn newArgumentEncoderForBufferAtIndex_(&self, index: NSUInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newArgumentEncoderForBufferAtIndex : index)
    }
    unsafe fn setVisibleFunctionTable_atIndex_(
        &self,
        visibleFunctionTable: *mut u64,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibleFunctionTable : visibleFunctionTable, atIndex : index)
    }
    unsafe fn setVisibleFunctionTables_withRange_(
        &self,
        visibleFunctionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibleFunctionTables : visibleFunctionTables, withRange : range)
    }
    unsafe fn setIntersectionFunctionTable_atIndex_(
        &self,
        intersectionFunctionTable: *mut u64,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntersectionFunctionTable : intersectionFunctionTable, atIndex : index)
    }
    unsafe fn setIntersectionFunctionTables_withRange_(
        &self,
        intersectionFunctionTables: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntersectionFunctionTables : intersectionFunctionTables, withRange : range)
    }
    unsafe fn setDepthStencilState_atIndex_(&self, depthStencilState: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStencilState : depthStencilState, atIndex : index)
    }
    unsafe fn setDepthStencilStates_withRange_(
        &self,
        depthStencilStates: *const *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStencilStates : depthStencilStates, withRange : range)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn encodedLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encodedLength)
    }
    unsafe fn alignment(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignment)
    }
}
pub type MTL4VisibilityOptions = NSUInteger;
pub trait PMTL4CommandEncoder: Sized + std::ops::Deref {
    unsafe fn barrierAfterQueueStages_beforeStages_visibilityOptions_(
        &self,
        afterQueueStages: MTLStages,
        beforeStages: MTLStages,
        visibilityOptions: MTL4VisibilityOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, barrierAfterQueueStages : afterQueueStages, beforeStages : beforeStages, visibilityOptions : visibilityOptions)
    }
    unsafe fn barrierAfterStages_beforeQueueStages_visibilityOptions_(
        &self,
        afterStages: MTLStages,
        beforeQueueStages: MTLStages,
        visibilityOptions: MTL4VisibilityOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, barrierAfterStages : afterStages, beforeQueueStages : beforeQueueStages, visibilityOptions : visibilityOptions)
    }
    unsafe fn barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions_(
        &self,
        afterEncoderStages: MTLStages,
        beforeEncoderStages: MTLStages,
        visibilityOptions: MTL4VisibilityOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, barrierAfterEncoderStages : afterEncoderStages, beforeEncoderStages : beforeEncoderStages, visibilityOptions : visibilityOptions)
    }
    unsafe fn updateFence_afterEncoderStages_(&self, fence: *mut u64, afterEncoderStages: MTLStages)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateFence : fence, afterEncoderStages : afterEncoderStages)
    }
    unsafe fn waitForFence_beforeEncoderStages_(
        &self,
        fence: *mut u64,
        beforeEncoderStages: MTLStages,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForFence : fence, beforeEncoderStages : beforeEncoderStages)
    }
    unsafe fn insertDebugSignpost_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertDebugSignpost : string)
    }
    unsafe fn pushDebugGroup_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushDebugGroup : string)
    }
    unsafe fn popDebugGroup(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, popDebugGroup)
    }
    unsafe fn endEncoding(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endEncoding)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn commandBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandBuffer)
    }
}
pub type MTL4RenderEncoderOptions = NSUInteger;
pub trait PMTL4RenderCommandEncoder: Sized + std::ops::Deref {
    unsafe fn setColorAttachmentMap_(&self, mapping: MTLLogicalToPhysicalColorAttachmentMap)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorAttachmentMap : mapping)
    }
    unsafe fn setRenderPipelineState_(&self, pipelineState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderPipelineState : pipelineState)
    }
    unsafe fn setViewport_(&self, viewport: MTLViewport)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setViewport : viewport)
    }
    unsafe fn setViewports_count_(&self, viewports: *const MTLViewport, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setViewports : viewports, count : count)
    }
    unsafe fn setVertexAmplificationCount_viewMappings_(
        &self,
        count: NSUInteger,
        viewMappings: *const MTLVertexAmplificationViewMapping,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexAmplificationCount : count, viewMappings : viewMappings)
    }
    unsafe fn setCullMode_(&self, cullMode: MTLCullMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCullMode : cullMode)
    }
    unsafe fn setDepthClipMode_(&self, depthClipMode: MTLDepthClipMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthClipMode : depthClipMode)
    }
    unsafe fn setDepthBias_slopeScale_clamp_(&self, depthBias: f32, slopeScale: f32, clamp: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthBias : depthBias, slopeScale : slopeScale, clamp : clamp)
    }
    unsafe fn setDepthTestMinBound_maxBound_(&self, minBound: f32, maxBound: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthTestMinBound : minBound, maxBound : maxBound)
    }
    unsafe fn setScissorRect_(&self, rect: MTLScissorRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScissorRect : rect)
    }
    unsafe fn setScissorRects_count_(&self, scissorRects: *const MTLScissorRect, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScissorRects : scissorRects, count : count)
    }
    unsafe fn setTriangleFillMode_(&self, fillMode: MTLTriangleFillMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTriangleFillMode : fillMode)
    }
    unsafe fn setBlendColorRed_green_blue_alpha_(&self, red: f32, green: f32, blue: f32, alpha: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendColorRed : red, green : green, blue : blue, alpha : alpha)
    }
    unsafe fn setDepthStencilState_(&self, depthStencilState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStencilState : depthStencilState)
    }
    unsafe fn setStencilReferenceValue_(&self, referenceValue: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilReferenceValue : referenceValue)
    }
    unsafe fn setStencilFrontReferenceValue_backReferenceValue_(
        &self,
        frontReferenceValue: u32,
        backReferenceValue: u32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilFrontReferenceValue : frontReferenceValue, backReferenceValue : backReferenceValue)
    }
    unsafe fn setVisibilityResultMode_offset_(
        &self,
        mode: MTLVisibilityResultMode,
        offset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibilityResultMode : mode, offset : offset)
    }
    unsafe fn setColorStoreAction_atIndex_(
        &self,
        storeAction: MTLStoreAction,
        colorAttachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorStoreAction : storeAction, atIndex : colorAttachmentIndex)
    }
    unsafe fn setDepthStoreAction_(&self, storeAction: MTLStoreAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStoreAction : storeAction)
    }
    unsafe fn setStencilStoreAction_(&self, storeAction: MTLStoreAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilStoreAction : storeAction)
    }
    unsafe fn drawPrimitives_vertexStart_vertexCount_(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPrimitives : primitiveType, vertexStart : vertexStart, vertexCount : vertexCount)
    }
    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount_(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
        instanceCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPrimitives : primitiveType, vertexStart : vertexStart, vertexCount : vertexCount, instanceCount : instanceCount)
    }
    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance_(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPrimitives : primitiveType, vertexStart : vertexStart, vertexCount : vertexCount, instanceCount : instanceCount, baseInstance : baseInstance)
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferLength_(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: MTLGPUAddress,
        indexBufferLength: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPrimitives : primitiveType, indexCount : indexCount, indexType : indexType, indexBuffer : indexBuffer, indexBufferLength : indexBufferLength)
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferLength_instanceCount_(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: MTLGPUAddress,
        indexBufferLength: NSUInteger,
        instanceCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPrimitives : primitiveType, indexCount : indexCount, indexType : indexType, indexBuffer : indexBuffer, indexBufferLength : indexBufferLength, instanceCount : instanceCount)
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferLength_instanceCount_baseVertex_baseInstance_(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: MTLGPUAddress,
        indexBufferLength: NSUInteger,
        instanceCount: NSUInteger,
        baseVertex: NSInteger,
        baseInstance: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPrimitives : primitiveType, indexCount : indexCount, indexType : indexType, indexBuffer : indexBuffer, indexBufferLength : indexBufferLength, instanceCount : instanceCount, baseVertex : baseVertex, baseInstance : baseInstance)
    }
    unsafe fn drawPrimitives_indirectBuffer_(
        &self,
        primitiveType: MTLPrimitiveType,
        indirectBuffer: MTLGPUAddress,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPrimitives : primitiveType, indirectBuffer : indirectBuffer)
    }
    unsafe fn drawIndexedPrimitives_indexType_indexBuffer_indexBufferLength_indirectBuffer_(
        &self,
        primitiveType: MTLPrimitiveType,
        indexType: MTLIndexType,
        indexBuffer: MTLGPUAddress,
        indexBufferLength: NSUInteger,
        indirectBuffer: MTLGPUAddress,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPrimitives : primitiveType, indexType : indexType, indexBuffer : indexBuffer, indexBufferLength : indexBufferLength, indirectBuffer : indirectBuffer)
    }
    unsafe fn executeCommandsInBuffer_withRange_(
        &self,
        indirectCommandBuffer: *mut u64,
        executionRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeCommandsInBuffer : indirectCommandBuffer, withRange : executionRange)
    }
    unsafe fn executeCommandsInBuffer_indirectBuffer_(
        &self,
        indirectCommandBuffer: *mut u64,
        indirectRangeBuffer: MTLGPUAddress,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeCommandsInBuffer : indirectCommandBuffer, indirectBuffer : indirectRangeBuffer)
    }
    unsafe fn setObjectThreadgroupMemoryLength_atIndex_(
        &self,
        length: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectThreadgroupMemoryLength : length, atIndex : index)
    }
    unsafe fn drawMeshThreadgroups_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup_(
        &self,
        threadgroupsPerGrid: MTLSize,
        threadsPerObjectThreadgroup: MTLSize,
        threadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawMeshThreadgroups : threadgroupsPerGrid, threadsPerObjectThreadgroup : threadsPerObjectThreadgroup, threadsPerMeshThreadgroup : threadsPerMeshThreadgroup)
    }
    unsafe fn drawMeshThreads_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup_(
        &self,
        threadsPerGrid: MTLSize,
        threadsPerObjectThreadgroup: MTLSize,
        threadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawMeshThreads : threadsPerGrid, threadsPerObjectThreadgroup : threadsPerObjectThreadgroup, threadsPerMeshThreadgroup : threadsPerMeshThreadgroup)
    }
    unsafe fn drawMeshThreadgroupsWithIndirectBuffer_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup_(
        &self,
        indirectBuffer: MTLGPUAddress,
        threadsPerObjectThreadgroup: MTLSize,
        threadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawMeshThreadgroupsWithIndirectBuffer : indirectBuffer, threadsPerObjectThreadgroup : threadsPerObjectThreadgroup, threadsPerMeshThreadgroup : threadsPerMeshThreadgroup)
    }
    unsafe fn dispatchThreadsPerTile_(&self, threadsPerTile: MTLSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchThreadsPerTile : threadsPerTile)
    }
    unsafe fn setThreadgroupMemoryLength_offset_atIndex_(
        &self,
        length: NSUInteger,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadgroupMemoryLength : length, offset : offset, atIndex : index)
    }
    unsafe fn setArgumentTable_atStages_(&self, argumentTable: *mut u64, stages: MTLRenderStages)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArgumentTable : argumentTable, atStages : stages)
    }
    unsafe fn setFrontFacingWinding_(&self, frontFacingWinding: MTLWinding)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrontFacingWinding : frontFacingWinding)
    }
    unsafe fn writeTimestampWithGranularity_afterStage_intoHeap_atIndex_(
        &self,
        granularity: MTL4TimestampGranularity,
        stage: MTLRenderStages,
        counterHeap: *mut u64,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeTimestampWithGranularity : granularity, afterStage : stage, intoHeap : counterHeap, atIndex : index)
    }
    unsafe fn tileWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileWidth)
    }
    unsafe fn tileHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileHeight)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4CommandBufferOptions(pub id);
impl std::ops::Deref for MTL4CommandBufferOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4CommandBufferOptions {}
impl MTL4CommandBufferOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4CommandBufferOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4CommandBufferOptions {}
impl INSObject for MTL4CommandBufferOptions {}
impl PNSObject for MTL4CommandBufferOptions {}
impl std::convert::TryFrom<NSObject> for MTL4CommandBufferOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4CommandBufferOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4CommandBufferOptions").unwrap()) };
        if is_kind_of {
            Ok(MTL4CommandBufferOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4CommandBufferOptions")
        }
    }
}
impl IMTL4CommandBufferOptions for MTL4CommandBufferOptions {}
pub trait IMTL4CommandBufferOptions: Sized + std::ops::Deref {
    unsafe fn logState(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, logState)
    }
    unsafe fn setLogState_(&self, logState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLogState : logState)
    }
}
pub trait PMTL4CommandBuffer: Sized + std::ops::Deref {
    unsafe fn beginCommandBufferWithAllocator_(&self, allocator: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginCommandBufferWithAllocator : allocator)
    }
    unsafe fn beginCommandBufferWithAllocator_options_(
        &self,
        allocator: *mut u64,
        options: MTL4CommandBufferOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginCommandBufferWithAllocator : allocator, options : options)
    }
    unsafe fn endCommandBuffer(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endCommandBuffer)
    }
    unsafe fn renderCommandEncoderWithDescriptor_(
        &self,
        descriptor: MTL4RenderPassDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderCommandEncoderWithDescriptor : descriptor)
    }
    unsafe fn renderCommandEncoderWithDescriptor_options_(
        &self,
        descriptor: MTL4RenderPassDescriptor,
        options: MTL4RenderEncoderOptions,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderCommandEncoderWithDescriptor : descriptor, options : options)
    }
    unsafe fn computeCommandEncoder(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, computeCommandEncoder)
    }
    unsafe fn machineLearningCommandEncoder(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, machineLearningCommandEncoder)
    }
    unsafe fn useResidencySet_(&self, residencySet: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResidencySet : residencySet)
    }
    unsafe fn useResidencySets_count_(&self, residencySets: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResidencySets : residencySets, count : count)
    }
    unsafe fn pushDebugGroup_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushDebugGroup : string)
    }
    unsafe fn popDebugGroup(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, popDebugGroup)
    }
    unsafe fn writeTimestampIntoHeap_atIndex_(&self, counterHeap: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeTimestampIntoHeap : counterHeap, atIndex : index)
    }
    unsafe fn resolveCounterHeap_withRange_intoBuffer_waitFence_updateFence_(
        &self,
        counterHeap: *mut u64,
        range: NSRange,
        bufferRange: MTL4BufferRange,
        fenceToWait: *mut u64,
        fenceToUpdate: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveCounterHeap : counterHeap, withRange : range, intoBuffer : bufferRange, waitFence : fenceToWait, updateFence : fenceToUpdate)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub trait PMTLEvent: Sized + std::ops::Deref {
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLSharedEventListener(pub id);
impl std::ops::Deref for MTLSharedEventListener {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLSharedEventListener {}
impl MTLSharedEventListener {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLSharedEventListener").unwrap(), alloc) })
    }
}
impl INSObject for MTLSharedEventListener {}
impl PNSObject for MTLSharedEventListener {}
impl std::convert::TryFrom<NSObject> for MTLSharedEventListener {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLSharedEventListener, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLSharedEventListener").unwrap()) };
        if is_kind_of {
            Ok(MTLSharedEventListener(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLSharedEventListener")
        }
    }
}
impl IMTLSharedEventListener for MTLSharedEventListener {}
pub trait IMTLSharedEventListener: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDispatchQueue_(&self, dispatchQueue: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDispatchQueue : dispatchQueue)
    }
    unsafe fn dispatchQueue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dispatchQueue)
    }
    unsafe fn sharedListener() -> MTLSharedEventListener
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLSharedEventListener").unwrap(), sharedListener)
    }
}
pub type MTLSharedEventNotificationBlock = *mut ::std::os::raw::c_void;
pub trait PMTLSharedEvent: Sized + std::ops::Deref {
    unsafe fn notifyListener_atValue_block_(
        &self,
        listener: MTLSharedEventListener,
        value: u64,
        block: MTLSharedEventNotificationBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notifyListener : listener, atValue : value, block : block)
    }
    unsafe fn newSharedEventHandle(&self) -> MTLSharedEventHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newSharedEventHandle)
    }
    unsafe fn waitUntilSignaledValue_timeoutMS_(&self, value: u64, milliseconds: u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitUntilSignaledValue : value, timeoutMS : milliseconds)
    }
    unsafe fn signaledValue(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signaledValue)
    }
    unsafe fn setSignaledValue_(&self, signaledValue: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSignaledValue : signaledValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLSharedEventHandle(pub id);
impl std::ops::Deref for MTLSharedEventHandle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLSharedEventHandle {}
impl MTLSharedEventHandle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLSharedEventHandle").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MTLSharedEventHandle {}
impl INSObject for MTLSharedEventHandle {}
impl PNSObject for MTLSharedEventHandle {}
impl std::convert::TryFrom<NSObject> for MTLSharedEventHandle {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLSharedEventHandle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLSharedEventHandle").unwrap()) };
        if is_kind_of {
            Ok(MTLSharedEventHandle(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLSharedEventHandle")
        }
    }
}
impl IMTLSharedEventHandle for MTLSharedEventHandle {}
pub trait IMTLSharedEventHandle: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
}
pub trait PMTL4CommitFeedback: Sized + std::ops::Deref {
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
    unsafe fn GPUStartTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPUStartTime)
    }
    unsafe fn GPUEndTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPUEndTime)
    }
}
pub type MTL4CommandQueueError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4CommitOptions(pub id);
impl std::ops::Deref for MTL4CommitOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4CommitOptions {}
impl MTL4CommitOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4CommitOptions").unwrap(), alloc) })
    }
}
impl INSObject for MTL4CommitOptions {}
impl PNSObject for MTL4CommitOptions {}
impl std::convert::TryFrom<NSObject> for MTL4CommitOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4CommitOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4CommitOptions").unwrap()) };
        if is_kind_of {
            Ok(MTL4CommitOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4CommitOptions")
        }
    }
}
impl IMTL4CommitOptions for MTL4CommitOptions {}
pub trait IMTL4CommitOptions: Sized + std::ops::Deref {
    unsafe fn addFeedbackHandler_(&self, block: MTL4CommitFeedbackHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addFeedbackHandler : block)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4CommandQueueDescriptor(pub id);
impl std::ops::Deref for MTL4CommandQueueDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4CommandQueueDescriptor {}
impl MTL4CommandQueueDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4CommandQueueDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4CommandQueueDescriptor {}
impl INSObject for MTL4CommandQueueDescriptor {}
impl PNSObject for MTL4CommandQueueDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4CommandQueueDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4CommandQueueDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4CommandQueueDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4CommandQueueDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4CommandQueueDescriptor")
        }
    }
}
impl IMTL4CommandQueueDescriptor for MTL4CommandQueueDescriptor {}
pub trait IMTL4CommandQueueDescriptor: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn feedbackQueue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, feedbackQueue)
    }
    unsafe fn setFeedbackQueue_(&self, feedbackQueue: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFeedbackQueue : feedbackQueue)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4UpdateSparseTextureMappingOperation {
    pub mode: MTLSparseTextureMappingMode,
    pub textureRegion: MTLRegion,
    pub textureLevel: NSUInteger,
    pub textureSlice: NSUInteger,
    pub heapOffset: NSUInteger,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4CopySparseTextureMappingOperation {
    pub sourceRegion: MTLRegion,
    pub sourceLevel: NSUInteger,
    pub sourceSlice: NSUInteger,
    pub destinationOrigin: MTLOrigin,
    pub destinationLevel: NSUInteger,
    pub destinationSlice: NSUInteger,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4UpdateSparseBufferMappingOperation {
    pub mode: MTLSparseTextureMappingMode,
    pub bufferRange: NSRange,
    pub heapOffset: NSUInteger,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4CopySparseBufferMappingOperation {
    pub sourceRange: NSRange,
    pub destinationOffset: NSUInteger,
}
pub trait PMTL4CommandQueue: Sized + std::ops::Deref {
    unsafe fn commit_count_(&self, commandBuffers: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commit : commandBuffers, count : count)
    }
    unsafe fn commit_count_options_(
        &self,
        commandBuffers: *const *mut u64,
        count: NSUInteger,
        options: MTL4CommitOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commit : commandBuffers, count : count, options : options)
    }
    unsafe fn signalEvent_value_(&self, event: *mut u64, value: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signalEvent : event, value : value)
    }
    unsafe fn waitForEvent_value_(&self, event: *mut u64, value: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForEvent : event, value : value)
    }
    unsafe fn signalDrawable_(&self, drawable: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signalDrawable : drawable)
    }
    unsafe fn waitForDrawable_(&self, drawable: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForDrawable : drawable)
    }
    unsafe fn addResidencySet_(&self, residencySet: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addResidencySet : residencySet)
    }
    unsafe fn addResidencySets_count_(&self, residencySets: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addResidencySets : residencySets, count : count)
    }
    unsafe fn removeResidencySet_(&self, residencySet: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeResidencySet : residencySet)
    }
    unsafe fn removeResidencySets_count_(&self, residencySets: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeResidencySets : residencySets, count : count)
    }
    unsafe fn updateTextureMappings_heap_operations_count_(
        &self,
        texture: *mut u64,
        heap: *mut u64,
        operations: *const MTL4UpdateSparseTextureMappingOperation,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTextureMappings : texture, heap : heap, operations : operations, count : count)
    }
    unsafe fn copyTextureMappingsFromTexture_toTexture_operations_count_(
        &self,
        sourceTexture: *mut u64,
        destinationTexture: *mut u64,
        operations: *const MTL4CopySparseTextureMappingOperation,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyTextureMappingsFromTexture : sourceTexture, toTexture : destinationTexture, operations : operations, count : count)
    }
    unsafe fn updateBufferMappings_heap_operations_count_(
        &self,
        buffer: *mut u64,
        heap: *mut u64,
        operations: *const MTL4UpdateSparseBufferMappingOperation,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateBufferMappings : buffer, heap : heap, operations : operations, count : count)
    }
    unsafe fn copyBufferMappingsFromBuffer_toBuffer_operations_count_(
        &self,
        sourceBuffer: *mut u64,
        destinationBuffer: *mut u64,
        operations: *const MTL4CopySparseBufferMappingOperation,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyBufferMappingsFromBuffer : sourceBuffer, toBuffer : destinationBuffer, operations : operations, count : count)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
}
pub type MTLCaptureError = NSInteger;
pub type MTLCaptureDestination = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCaptureDescriptor(pub id);
impl std::ops::Deref for MTLCaptureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLCaptureDescriptor {}
impl MTLCaptureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLCaptureDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLCaptureDescriptor {}
impl INSObject for MTLCaptureDescriptor {}
impl PNSObject for MTLCaptureDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLCaptureDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLCaptureDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLCaptureDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLCaptureDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLCaptureDescriptor")
        }
    }
}
impl IMTLCaptureDescriptor for MTLCaptureDescriptor {}
pub trait IMTLCaptureDescriptor: Sized + std::ops::Deref {
    unsafe fn captureObject(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, captureObject)
    }
    unsafe fn setCaptureObject_(&self, captureObject: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaptureObject : captureObject)
    }
    unsafe fn destination(&self) -> MTLCaptureDestination
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn setDestination_(&self, destination: MTLCaptureDestination)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestination : destination)
    }
    unsafe fn outputURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputURL)
    }
    unsafe fn setOutputURL_(&self, outputURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputURL : outputURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCaptureManager(pub id);
impl std::ops::Deref for MTLCaptureManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLCaptureManager {}
impl MTLCaptureManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLCaptureManager").unwrap(), alloc) })
    }
}
impl INSObject for MTLCaptureManager {}
impl PNSObject for MTLCaptureManager {}
impl std::convert::TryFrom<NSObject> for MTLCaptureManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLCaptureManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLCaptureManager").unwrap()) };
        if is_kind_of {
            Ok(MTLCaptureManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLCaptureManager")
        }
    }
}
impl IMTLCaptureManager for MTLCaptureManager {}
pub trait IMTLCaptureManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn newCaptureScopeWithDevice_(&self, device: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCaptureScopeWithDevice : device)
    }
    unsafe fn newCaptureScopeWithCommandQueue_(&self, commandQueue: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCaptureScopeWithCommandQueue : commandQueue)
    }
    unsafe fn newCaptureScopeWithMTL4CommandQueue_(&self, commandQueue: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCaptureScopeWithMTL4CommandQueue : commandQueue)
    }
    unsafe fn supportsDestination_(&self, destination: MTLCaptureDestination) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsDestination : destination)
    }
    unsafe fn startCaptureWithDescriptor_error_(
        &self,
        descriptor: MTLCaptureDescriptor,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startCaptureWithDescriptor : descriptor, error : error)
    }
    unsafe fn startCaptureWithDevice_(&self, device: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startCaptureWithDevice : device)
    }
    unsafe fn startCaptureWithCommandQueue_(&self, commandQueue: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startCaptureWithCommandQueue : commandQueue)
    }
    unsafe fn startCaptureWithScope_(&self, captureScope: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startCaptureWithScope : captureScope)
    }
    unsafe fn stopCapture(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopCapture)
    }
    unsafe fn defaultCaptureScope(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultCaptureScope)
    }
    unsafe fn setDefaultCaptureScope_(&self, defaultCaptureScope: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultCaptureScope : defaultCaptureScope)
    }
    unsafe fn isCapturing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCapturing)
    }
    unsafe fn sharedCaptureManager() -> MTLCaptureManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLCaptureManager").unwrap(), sharedCaptureManager)
    }
}
pub trait PMTLCaptureScope: Sized + std::ops::Deref {
    unsafe fn beginScope(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginScope)
    }
    unsafe fn endScope(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endScope)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn commandQueue(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandQueue)
    }
    unsafe fn mtl4CommandQueue(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mtl4CommandQueue)
    }
}
pub trait PMTLIndirectRenderCommand: Sized + std::ops::Deref {
    unsafe fn setRenderPipelineState_(&self, pipelineState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderPipelineState : pipelineState)
    }
    unsafe fn setVertexBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setFragmentBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setVertexBuffer_offset_attributeStride_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        stride: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffer : buffer, offset : offset, attributeStride : stride, atIndex : index)
    }
    unsafe fn drawPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_instanceCount_baseInstance_tessellationFactorBuffer_tessellationFactorBufferOffset_tessellationFactorBufferInstanceStride_(
        &self,
        numberOfPatchControlPoints: NSUInteger,
        patchStart: NSUInteger,
        patchCount: NSUInteger,
        patchIndexBuffer: *mut u64,
        patchIndexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
        buffer: *mut u64,
        offset: NSUInteger,
        instanceStride: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPatches : numberOfPatchControlPoints, patchStart : patchStart, patchCount : patchCount, patchIndexBuffer : patchIndexBuffer, patchIndexBufferOffset : patchIndexBufferOffset, instanceCount : instanceCount, baseInstance : baseInstance, tessellationFactorBuffer : buffer, tessellationFactorBufferOffset : offset, tessellationFactorBufferInstanceStride : instanceStride)
    }
    unsafe fn drawIndexedPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_controlPointIndexBuffer_controlPointIndexBufferOffset_instanceCount_baseInstance_tessellationFactorBuffer_tessellationFactorBufferOffset_tessellationFactorBufferInstanceStride_(
        &self,
        numberOfPatchControlPoints: NSUInteger,
        patchStart: NSUInteger,
        patchCount: NSUInteger,
        patchIndexBuffer: *mut u64,
        patchIndexBufferOffset: NSUInteger,
        controlPointIndexBuffer: *mut u64,
        controlPointIndexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
        buffer: *mut u64,
        offset: NSUInteger,
        instanceStride: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPatches : numberOfPatchControlPoints, patchStart : patchStart, patchCount : patchCount, patchIndexBuffer : patchIndexBuffer, patchIndexBufferOffset : patchIndexBufferOffset, controlPointIndexBuffer : controlPointIndexBuffer, controlPointIndexBufferOffset : controlPointIndexBufferOffset, instanceCount : instanceCount, baseInstance : baseInstance, tessellationFactorBuffer : buffer, tessellationFactorBufferOffset : offset, tessellationFactorBufferInstanceStride : instanceStride)
    }
    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance_(
        &self,
        primitiveType: MTLPrimitiveType,
        vertexStart: NSUInteger,
        vertexCount: NSUInteger,
        instanceCount: NSUInteger,
        baseInstance: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPrimitives : primitiveType, vertexStart : vertexStart, vertexCount : vertexCount, instanceCount : instanceCount, baseInstance : baseInstance)
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance_(
        &self,
        primitiveType: MTLPrimitiveType,
        indexCount: NSUInteger,
        indexType: MTLIndexType,
        indexBuffer: *mut u64,
        indexBufferOffset: NSUInteger,
        instanceCount: NSUInteger,
        baseVertex: NSInteger,
        baseInstance: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawIndexedPrimitives : primitiveType, indexCount : indexCount, indexType : indexType, indexBuffer : indexBuffer, indexBufferOffset : indexBufferOffset, instanceCount : instanceCount, baseVertex : baseVertex, baseInstance : baseInstance)
    }
    unsafe fn setObjectThreadgroupMemoryLength_atIndex_(
        &self,
        length: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectThreadgroupMemoryLength : length, atIndex : index)
    }
    unsafe fn setObjectBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setMeshBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn drawMeshThreadgroups_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup_(
        &self,
        threadgroupsPerGrid: MTLSize,
        threadsPerObjectThreadgroup: MTLSize,
        threadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawMeshThreadgroups : threadgroupsPerGrid, threadsPerObjectThreadgroup : threadsPerObjectThreadgroup, threadsPerMeshThreadgroup : threadsPerMeshThreadgroup)
    }
    unsafe fn drawMeshThreads_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup_(
        &self,
        threadsPerGrid: MTLSize,
        threadsPerObjectThreadgroup: MTLSize,
        threadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawMeshThreads : threadsPerGrid, threadsPerObjectThreadgroup : threadsPerObjectThreadgroup, threadsPerMeshThreadgroup : threadsPerMeshThreadgroup)
    }
    unsafe fn setBarrier(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setBarrier)
    }
    unsafe fn clearBarrier(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearBarrier)
    }
    unsafe fn setDepthStencilState_(&self, depthStencilState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStencilState : depthStencilState)
    }
    unsafe fn setDepthBias_slopeScale_clamp_(&self, depthBias: f32, slopeScale: f32, clamp: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthBias : depthBias, slopeScale : slopeScale, clamp : clamp)
    }
    unsafe fn setDepthClipMode_(&self, depthClipMode: MTLDepthClipMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthClipMode : depthClipMode)
    }
    unsafe fn setCullMode_(&self, cullMode: MTLCullMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCullMode : cullMode)
    }
    unsafe fn setFrontFacingWinding_(&self, frontFacingWindning: MTLWinding)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrontFacingWinding : frontFacingWindning)
    }
    unsafe fn setTriangleFillMode_(&self, fillMode: MTLTriangleFillMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTriangleFillMode : fillMode)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
}
pub trait PMTLIndirectComputeCommand: Sized + std::ops::Deref {
    unsafe fn setComputePipelineState_(&self, pipelineState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputePipelineState : pipelineState)
    }
    unsafe fn setKernelBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKernelBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setKernelBuffer_offset_attributeStride_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        stride: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKernelBuffer : buffer, offset : offset, attributeStride : stride, atIndex : index)
    }
    unsafe fn concurrentDispatchThreadgroups_threadsPerThreadgroup_(
        &self,
        threadgroupsPerGrid: MTLSize,
        threadsPerThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, concurrentDispatchThreadgroups : threadgroupsPerGrid, threadsPerThreadgroup : threadsPerThreadgroup)
    }
    unsafe fn concurrentDispatchThreads_threadsPerThreadgroup_(
        &self,
        threadsPerGrid: MTLSize,
        threadsPerThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, concurrentDispatchThreads : threadsPerGrid, threadsPerThreadgroup : threadsPerThreadgroup)
    }
    unsafe fn setBarrier(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setBarrier)
    }
    unsafe fn clearBarrier(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearBarrier)
    }
    unsafe fn setImageblockWidth_height_(&self, width: NSUInteger, height: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageblockWidth : width, height : height)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn setThreadgroupMemoryLength_atIndex_(&self, length: NSUInteger, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadgroupMemoryLength : length, atIndex : index)
    }
    unsafe fn setStageInRegion_(&self, region: MTLRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStageInRegion : region)
    }
}
pub type MTLIndirectCommandType = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLIndirectCommandBufferExecutionRange {
    pub location: u32,
    pub length: u32,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLIndirectCommandBufferDescriptor(pub id);
impl std::ops::Deref for MTLIndirectCommandBufferDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLIndirectCommandBufferDescriptor {}
impl MTLIndirectCommandBufferDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLIndirectCommandBufferDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLIndirectCommandBufferDescriptor {}
impl INSObject for MTLIndirectCommandBufferDescriptor {}
impl PNSObject for MTLIndirectCommandBufferDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLIndirectCommandBufferDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLIndirectCommandBufferDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLIndirectCommandBufferDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLIndirectCommandBufferDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLIndirectCommandBufferDescriptor")
        }
    }
}
impl IMTLIndirectCommandBufferDescriptor for MTLIndirectCommandBufferDescriptor {}
pub trait IMTLIndirectCommandBufferDescriptor: Sized + std::ops::Deref {
    unsafe fn commandTypes(&self) -> MTLIndirectCommandType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandTypes)
    }
    unsafe fn setCommandTypes_(&self, commandTypes: MTLIndirectCommandType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCommandTypes : commandTypes)
    }
    unsafe fn inheritPipelineState(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inheritPipelineState)
    }
    unsafe fn setInheritPipelineState_(&self, inheritPipelineState: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInheritPipelineState : inheritPipelineState)
    }
    unsafe fn inheritBuffers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inheritBuffers)
    }
    unsafe fn setInheritBuffers_(&self, inheritBuffers: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInheritBuffers : inheritBuffers)
    }
    unsafe fn inheritDepthStencilState(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inheritDepthStencilState)
    }
    unsafe fn setInheritDepthStencilState_(&self, inheritDepthStencilState: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInheritDepthStencilState : inheritDepthStencilState)
    }
    unsafe fn inheritDepthBias(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inheritDepthBias)
    }
    unsafe fn setInheritDepthBias_(&self, inheritDepthBias: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInheritDepthBias : inheritDepthBias)
    }
    unsafe fn inheritDepthClipMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inheritDepthClipMode)
    }
    unsafe fn setInheritDepthClipMode_(&self, inheritDepthClipMode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInheritDepthClipMode : inheritDepthClipMode)
    }
    unsafe fn inheritCullMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inheritCullMode)
    }
    unsafe fn setInheritCullMode_(&self, inheritCullMode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInheritCullMode : inheritCullMode)
    }
    unsafe fn inheritFrontFacingWinding(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inheritFrontFacingWinding)
    }
    unsafe fn setInheritFrontFacingWinding_(&self, inheritFrontFacingWinding: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInheritFrontFacingWinding : inheritFrontFacingWinding)
    }
    unsafe fn inheritTriangleFillMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inheritTriangleFillMode)
    }
    unsafe fn setInheritTriangleFillMode_(&self, inheritTriangleFillMode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInheritTriangleFillMode : inheritTriangleFillMode)
    }
    unsafe fn maxVertexBufferBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxVertexBufferBindCount)
    }
    unsafe fn setMaxVertexBufferBindCount_(&self, maxVertexBufferBindCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxVertexBufferBindCount : maxVertexBufferBindCount)
    }
    unsafe fn maxFragmentBufferBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxFragmentBufferBindCount)
    }
    unsafe fn setMaxFragmentBufferBindCount_(&self, maxFragmentBufferBindCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxFragmentBufferBindCount : maxFragmentBufferBindCount)
    }
    unsafe fn maxKernelBufferBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxKernelBufferBindCount)
    }
    unsafe fn setMaxKernelBufferBindCount_(&self, maxKernelBufferBindCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxKernelBufferBindCount : maxKernelBufferBindCount)
    }
    unsafe fn maxKernelThreadgroupMemoryBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxKernelThreadgroupMemoryBindCount)
    }
    unsafe fn setMaxKernelThreadgroupMemoryBindCount_(
        &self,
        maxKernelThreadgroupMemoryBindCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxKernelThreadgroupMemoryBindCount : maxKernelThreadgroupMemoryBindCount)
    }
    unsafe fn maxObjectBufferBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxObjectBufferBindCount)
    }
    unsafe fn setMaxObjectBufferBindCount_(&self, maxObjectBufferBindCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxObjectBufferBindCount : maxObjectBufferBindCount)
    }
    unsafe fn maxMeshBufferBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxMeshBufferBindCount)
    }
    unsafe fn setMaxMeshBufferBindCount_(&self, maxMeshBufferBindCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxMeshBufferBindCount : maxMeshBufferBindCount)
    }
    unsafe fn maxObjectThreadgroupMemoryBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxObjectThreadgroupMemoryBindCount)
    }
    unsafe fn setMaxObjectThreadgroupMemoryBindCount_(
        &self,
        maxObjectThreadgroupMemoryBindCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxObjectThreadgroupMemoryBindCount : maxObjectThreadgroupMemoryBindCount)
    }
    unsafe fn supportRayTracing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportRayTracing)
    }
    unsafe fn setSupportRayTracing_(&self, supportRayTracing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportRayTracing : supportRayTracing)
    }
    unsafe fn supportDynamicAttributeStride(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportDynamicAttributeStride)
    }
    unsafe fn setSupportDynamicAttributeStride_(&self, supportDynamicAttributeStride: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportDynamicAttributeStride : supportDynamicAttributeStride)
    }
    unsafe fn supportColorAttachmentMapping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportColorAttachmentMapping)
    }
    unsafe fn setSupportColorAttachmentMapping_(&self, supportColorAttachmentMapping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportColorAttachmentMapping : supportColorAttachmentMapping)
    }
}
pub trait PMTLIndirectCommandBuffer: Sized + std::ops::Deref {
    unsafe fn resetWithRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithRange : range)
    }
    unsafe fn indirectRenderCommandAtIndex_(&self, commandIndex: NSUInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indirectRenderCommandAtIndex : commandIndex)
    }
    unsafe fn indirectComputeCommandAtIndex_(&self, commandIndex: NSUInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indirectComputeCommandAtIndex : commandIndex)
    }
    unsafe fn size(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
}
pub type MTLFunctionLogType = NSUInteger;
pub trait PMTLLogContainer: Sized + std::ops::Deref {}
pub trait PMTLFunctionLogDebugLocation: Sized + std::ops::Deref {
    unsafe fn functionName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionName)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn line(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line)
    }
    unsafe fn column(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, column)
    }
}
pub trait PMTLFunctionLog: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> MTLFunctionLogType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn encoderLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encoderLabel)
    }
    unsafe fn function(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, function)
    }
    unsafe fn debugLocation(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, debugLocation)
    }
}
pub trait PMTLAccelerationStructureCommandEncoder: Sized + std::ops::Deref {
    unsafe fn buildAccelerationStructure_descriptor_scratchBuffer_scratchBufferOffset_(
        &self,
        accelerationStructure: *mut u64,
        descriptor: MTLAccelerationStructureDescriptor,
        scratchBuffer: *mut u64,
        scratchBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, buildAccelerationStructure : accelerationStructure, descriptor : descriptor, scratchBuffer : scratchBuffer, scratchBufferOffset : scratchBufferOffset)
    }
    unsafe fn refitAccelerationStructure_descriptor_destination_scratchBuffer_scratchBufferOffset_(
        &self,
        sourceAccelerationStructure: *mut u64,
        descriptor: MTLAccelerationStructureDescriptor,
        destinationAccelerationStructure: *mut u64,
        scratchBuffer: *mut u64,
        scratchBufferOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, refitAccelerationStructure : sourceAccelerationStructure, descriptor : descriptor, destination : destinationAccelerationStructure, scratchBuffer : scratchBuffer, scratchBufferOffset : scratchBufferOffset)
    }
    unsafe fn refitAccelerationStructure_descriptor_destination_scratchBuffer_scratchBufferOffset_options_(
        &self,
        sourceAccelerationStructure: *mut u64,
        descriptor: MTLAccelerationStructureDescriptor,
        destinationAccelerationStructure: *mut u64,
        scratchBuffer: *mut u64,
        scratchBufferOffset: NSUInteger,
        options: MTLAccelerationStructureRefitOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, refitAccelerationStructure : sourceAccelerationStructure, descriptor : descriptor, destination : destinationAccelerationStructure, scratchBuffer : scratchBuffer, scratchBufferOffset : scratchBufferOffset, options : options)
    }
    unsafe fn copyAccelerationStructure_toAccelerationStructure_(
        &self,
        sourceAccelerationStructure: *mut u64,
        destinationAccelerationStructure: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyAccelerationStructure : sourceAccelerationStructure, toAccelerationStructure : destinationAccelerationStructure)
    }
    unsafe fn writeCompactedAccelerationStructureSize_toBuffer_offset_(
        &self,
        accelerationStructure: *mut u64,
        buffer: *mut u64,
        offset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeCompactedAccelerationStructureSize : accelerationStructure, toBuffer : buffer, offset : offset)
    }
    unsafe fn writeCompactedAccelerationStructureSize_toBuffer_offset_sizeDataType_(
        &self,
        accelerationStructure: *mut u64,
        buffer: *mut u64,
        offset: NSUInteger,
        sizeDataType: MTLDataType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeCompactedAccelerationStructureSize : accelerationStructure, toBuffer : buffer, offset : offset, sizeDataType : sizeDataType)
    }
    unsafe fn copyAndCompactAccelerationStructure_toAccelerationStructure_(
        &self,
        sourceAccelerationStructure: *mut u64,
        destinationAccelerationStructure: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyAndCompactAccelerationStructure : sourceAccelerationStructure, toAccelerationStructure : destinationAccelerationStructure)
    }
    unsafe fn updateFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateFence : fence)
    }
    unsafe fn waitForFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForFence : fence)
    }
    unsafe fn useResource_usage_(&self, resource: *mut u64, usage: MTLResourceUsage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResource : resource, usage : usage)
    }
    unsafe fn useResources_count_usage_(
        &self,
        resources: *const *mut u64,
        count: NSUInteger,
        usage: MTLResourceUsage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResources : resources, count : count, usage : usage)
    }
    unsafe fn useHeap_(&self, heap: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useHeap : heap)
    }
    unsafe fn useHeaps_count_(&self, heaps: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useHeaps : heaps, count : count)
    }
    unsafe fn sampleCountersInBuffer_atSampleIndex_withBarrier_(
        &self,
        sampleBuffer: *mut u64,
        sampleIndex: NSUInteger,
        barrier: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleCountersInBuffer : sampleBuffer, atSampleIndex : sampleIndex, withBarrier : barrier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructurePassSampleBufferAttachmentDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}
impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructurePassSampleBufferAttachmentDescriptor").unwrap(), alloc)
        })
    }
}
impl PNSCopying for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}
impl INSObject for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}
impl PNSObject for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}
impl std::convert::TryFrom<NSObject>
    for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLAccelerationStructurePassSampleBufferAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructurePassSampleBufferAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructurePassSampleBufferAttachmentDescriptor(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MTLAccelerationStructurePassSampleBufferAttachmentDescriptor" ,)
        }
    }
}
impl IMTLAccelerationStructurePassSampleBufferAttachmentDescriptor
    for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor
{
}
pub trait IMTLAccelerationStructurePassSampleBufferAttachmentDescriptor:
    Sized + std::ops::Deref
{
    unsafe fn sampleBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBuffer)
    }
    unsafe fn setSampleBuffer_(&self, sampleBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleBuffer : sampleBuffer)
    }
    unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startOfEncoderSampleIndex)
    }
    unsafe fn setStartOfEncoderSampleIndex_(&self, startOfEncoderSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartOfEncoderSampleIndex : startOfEncoderSampleIndex)
    }
    unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endOfEncoderSampleIndex)
    }
    unsafe fn setEndOfEncoderSampleIndex_(&self, endOfEncoderSampleIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndOfEncoderSampleIndex : endOfEncoderSampleIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray(pub id);
impl std::ops::Deref for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {}
impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray").unwrap(), alloc)
        })
    }
}
impl INSObject for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {}
impl PNSObject for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {}
impl std::convert::TryFrom<NSObject>
    for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray
{
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray, Self::Error>
    {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray" ,)
        }
    }
}
impl IMTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray
    for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray
{
}
pub trait IMTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray:
    Sized + std::ops::Deref
{
    unsafe fn objectAtIndexedSubscript_(
        &self,
        attachmentIndex: NSUInteger,
    ) -> MTLAccelerationStructurePassSampleBufferAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : attachmentIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attachment: MTLAccelerationStructurePassSampleBufferAttachmentDescriptor,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attachment, atIndexedSubscript : attachmentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLAccelerationStructurePassDescriptor(pub id);
impl std::ops::Deref for MTLAccelerationStructurePassDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLAccelerationStructurePassDescriptor {}
impl MTLAccelerationStructurePassDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructurePassDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLAccelerationStructurePassDescriptor {}
impl INSObject for MTLAccelerationStructurePassDescriptor {}
impl PNSObject for MTLAccelerationStructurePassDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLAccelerationStructurePassDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLAccelerationStructurePassDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLAccelerationStructurePassDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLAccelerationStructurePassDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLAccelerationStructurePassDescriptor")
        }
    }
}
impl IMTLAccelerationStructurePassDescriptor for MTLAccelerationStructurePassDescriptor {}
pub trait IMTLAccelerationStructurePassDescriptor: Sized + std::ops::Deref {
    unsafe fn sampleBufferAttachments(
        &self,
    ) -> MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleBufferAttachments)
    }
    unsafe fn accelerationStructurePassDescriptor() -> MTLAccelerationStructurePassDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLAccelerationStructurePassDescriptor").unwrap(), accelerationStructurePassDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRasterizationRateSampleArray(pub id);
impl std::ops::Deref for MTLRasterizationRateSampleArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRasterizationRateSampleArray {}
impl MTLRasterizationRateSampleArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRasterizationRateSampleArray").unwrap(), alloc) })
    }
}
impl INSObject for MTLRasterizationRateSampleArray {}
impl PNSObject for MTLRasterizationRateSampleArray {}
impl std::convert::TryFrom<NSObject> for MTLRasterizationRateSampleArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLRasterizationRateSampleArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRasterizationRateSampleArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLRasterizationRateSampleArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRasterizationRateSampleArray")
        }
    }
}
impl IMTLRasterizationRateSampleArray for MTLRasterizationRateSampleArray {}
pub trait IMTLRasterizationRateSampleArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(&self, index: NSUInteger) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : index)
    }
    unsafe fn setObject_atIndexedSubscript_(&self, value: NSNumber, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : value, atIndexedSubscript : index)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRasterizationRateLayerDescriptor(pub id);
impl std::ops::Deref for MTLRasterizationRateLayerDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRasterizationRateLayerDescriptor {}
impl MTLRasterizationRateLayerDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRasterizationRateLayerDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLRasterizationRateLayerDescriptor {}
impl INSObject for MTLRasterizationRateLayerDescriptor {}
impl PNSObject for MTLRasterizationRateLayerDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLRasterizationRateLayerDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLRasterizationRateLayerDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRasterizationRateLayerDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLRasterizationRateLayerDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRasterizationRateLayerDescriptor")
        }
    }
}
impl IMTLRasterizationRateLayerDescriptor for MTLRasterizationRateLayerDescriptor {}
pub trait IMTLRasterizationRateLayerDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSampleCount_(&self, sampleCount: MTLSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSampleCount : sampleCount)
    }
    unsafe fn initWithSampleCount_horizontal_vertical_(
        &self,
        sampleCount: MTLSize,
        horizontal: *const f32,
        vertical: *const f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSampleCount : sampleCount, horizontal : horizontal, vertical : vertical)
    }
    unsafe fn sampleCount(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
    unsafe fn maxSampleCount(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxSampleCount)
    }
    unsafe fn horizontalSampleStorage(&self) -> *mut f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalSampleStorage)
    }
    unsafe fn verticalSampleStorage(&self) -> *mut f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalSampleStorage)
    }
    unsafe fn horizontal(&self) -> MTLRasterizationRateSampleArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontal)
    }
    unsafe fn vertical(&self) -> MTLRasterizationRateSampleArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertical)
    }
}
impl MTLRasterizationRateLayerDescriptor_ for MTLRasterizationRateLayerDescriptor {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRasterizationRateLayerArray(pub id);
impl std::ops::Deref for MTLRasterizationRateLayerArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRasterizationRateLayerArray {}
impl MTLRasterizationRateLayerArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRasterizationRateLayerArray").unwrap(), alloc) })
    }
}
impl INSObject for MTLRasterizationRateLayerArray {}
impl PNSObject for MTLRasterizationRateLayerArray {}
impl std::convert::TryFrom<NSObject> for MTLRasterizationRateLayerArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLRasterizationRateLayerArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRasterizationRateLayerArray").unwrap())
        };
        if is_kind_of {
            Ok(MTLRasterizationRateLayerArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRasterizationRateLayerArray")
        }
    }
}
impl IMTLRasterizationRateLayerArray for MTLRasterizationRateLayerArray {}
pub trait IMTLRasterizationRateLayerArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(
        &self,
        layerIndex: NSUInteger,
    ) -> MTLRasterizationRateLayerDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : layerIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        layer: MTLRasterizationRateLayerDescriptor,
        layerIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : layer, atIndexedSubscript : layerIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLRasterizationRateMapDescriptor(pub id);
impl std::ops::Deref for MTLRasterizationRateMapDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLRasterizationRateMapDescriptor {}
impl MTLRasterizationRateMapDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRasterizationRateMapDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLRasterizationRateMapDescriptor {}
impl INSObject for MTLRasterizationRateMapDescriptor {}
impl PNSObject for MTLRasterizationRateMapDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLRasterizationRateMapDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLRasterizationRateMapDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLRasterizationRateMapDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLRasterizationRateMapDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLRasterizationRateMapDescriptor")
        }
    }
}
impl IMTLRasterizationRateMapDescriptor for MTLRasterizationRateMapDescriptor {}
pub trait IMTLRasterizationRateMapDescriptor: Sized + std::ops::Deref {
    unsafe fn layerAtIndex_(&self, layerIndex: NSUInteger) -> MTLRasterizationRateLayerDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layerAtIndex : layerIndex)
    }
    unsafe fn setLayer_atIndex_(
        &self,
        layer: MTLRasterizationRateLayerDescriptor,
        layerIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayer : layer, atIndex : layerIndex)
    }
    unsafe fn layers(&self) -> MTLRasterizationRateLayerArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layers)
    }
    unsafe fn screenSize(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSize)
    }
    unsafe fn setScreenSize_(&self, screenSize: MTLSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScreenSize : screenSize)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn layerCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerCount)
    }
    unsafe fn rasterizationRateMapDescriptorWithScreenSize_(
        screenSize: MTLSize,
    ) -> MTLRasterizationRateMapDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRasterizationRateMapDescriptor").unwrap(), rasterizationRateMapDescriptorWithScreenSize : screenSize)
    }
    unsafe fn rasterizationRateMapDescriptorWithScreenSize_layer_(
        screenSize: MTLSize,
        layer: MTLRasterizationRateLayerDescriptor,
    ) -> MTLRasterizationRateMapDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRasterizationRateMapDescriptor").unwrap(), rasterizationRateMapDescriptorWithScreenSize : screenSize, layer : layer)
    }
    unsafe fn rasterizationRateMapDescriptorWithScreenSize_layerCount_layers_(
        screenSize: MTLSize,
        layerCount: NSUInteger,
        layers: *const MTLRasterizationRateLayerDescriptor,
    ) -> MTLRasterizationRateMapDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLRasterizationRateMapDescriptor").unwrap(), rasterizationRateMapDescriptorWithScreenSize : screenSize, layerCount : layerCount, layers : layers)
    }
}
pub trait PMTLRasterizationRateMap: Sized + std::ops::Deref {
    unsafe fn copyParameterDataToBuffer_offset_(&self, buffer: *mut u64, offset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyParameterDataToBuffer : buffer, offset : offset)
    }
    unsafe fn physicalSizeForLayer_(&self, layerIndex: NSUInteger) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, physicalSizeForLayer : layerIndex)
    }
    unsafe fn mapScreenToPhysicalCoordinates_forLayer_(
        &self,
        screenCoordinates: MTLCoordinate2D,
        layerIndex: NSUInteger,
    ) -> MTLCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapScreenToPhysicalCoordinates : screenCoordinates, forLayer : layerIndex)
    }
    unsafe fn mapPhysicalToScreenCoordinates_forLayer_(
        &self,
        physicalCoordinates: MTLCoordinate2D,
        layerIndex: NSUInteger,
    ) -> MTLCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapPhysicalToScreenCoordinates : physicalCoordinates, forLayer : layerIndex)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn screenSize(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenSize)
    }
    unsafe fn physicalGranularity(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicalGranularity)
    }
    unsafe fn layerCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerCount)
    }
    unsafe fn parameterBufferSizeAndAlign(&self) -> MTLSizeAndAlign
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterBufferSizeAndAlign)
    }
}
pub type MTLDynamicLibraryError = NSUInteger;
pub trait PMTLDynamicLibrary: Sized + std::ops::Deref {
    unsafe fn serializeToURL_error_(&self, url: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, serializeToURL : url, error : error)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn installName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, installName)
    }
}
pub type MTLLogLevel = NSInteger;
pub trait PMTLLogState: Sized + std::ops::Deref {
    unsafe fn addLogHandler_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addLogHandler : block)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLLogStateDescriptor(pub id);
impl std::ops::Deref for MTLLogStateDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLLogStateDescriptor {}
impl MTLLogStateDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLLogStateDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLLogStateDescriptor {}
impl INSObject for MTLLogStateDescriptor {}
impl PNSObject for MTLLogStateDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLLogStateDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLLogStateDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLLogStateDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLLogStateDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLLogStateDescriptor")
        }
    }
}
impl IMTLLogStateDescriptor for MTLLogStateDescriptor {}
pub trait IMTLLogStateDescriptor: Sized + std::ops::Deref {
    unsafe fn level(&self) -> MTLLogLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, level)
    }
    unsafe fn setLevel_(&self, level: MTLLogLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevel : level)
    }
    unsafe fn bufferSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferSize)
    }
    unsafe fn setBufferSize_(&self, bufferSize: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBufferSize : bufferSize)
    }
}
pub type MTLLogStateError = NSUInteger;
pub type MTLBinaryArchiveError = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLBinaryArchiveDescriptor(pub id);
impl std::ops::Deref for MTLBinaryArchiveDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLBinaryArchiveDescriptor {}
impl MTLBinaryArchiveDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLBinaryArchiveDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLBinaryArchiveDescriptor {}
impl INSObject for MTLBinaryArchiveDescriptor {}
impl PNSObject for MTLBinaryArchiveDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLBinaryArchiveDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLBinaryArchiveDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLBinaryArchiveDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLBinaryArchiveDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLBinaryArchiveDescriptor")
        }
    }
}
impl IMTLBinaryArchiveDescriptor for MTLBinaryArchiveDescriptor {}
pub trait IMTLBinaryArchiveDescriptor: Sized + std::ops::Deref {
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn setUrl_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUrl : url)
    }
}
pub trait PMTLBinaryArchive: Sized + std::ops::Deref {
    unsafe fn addComputePipelineFunctionsWithDescriptor_error_(
        &self,
        descriptor: MTLComputePipelineDescriptor,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addComputePipelineFunctionsWithDescriptor : descriptor, error : error)
    }
    unsafe fn addRenderPipelineFunctionsWithDescriptor_error_(
        &self,
        descriptor: MTLRenderPipelineDescriptor,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRenderPipelineFunctionsWithDescriptor : descriptor, error : error)
    }
    unsafe fn addTileRenderPipelineFunctionsWithDescriptor_error_(
        &self,
        descriptor: MTLTileRenderPipelineDescriptor,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTileRenderPipelineFunctionsWithDescriptor : descriptor, error : error)
    }
    unsafe fn addMeshRenderPipelineFunctionsWithDescriptor_error_(
        &self,
        descriptor: MTLMeshRenderPipelineDescriptor,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMeshRenderPipelineFunctionsWithDescriptor : descriptor, error : error)
    }
    unsafe fn addLibraryWithDescriptor_error_(
        &self,
        descriptor: MTLStitchedLibraryDescriptor,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addLibraryWithDescriptor : descriptor, error : error)
    }
    unsafe fn serializeToURL_error_(&self, url: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, serializeToURL : url, error : error)
    }
    unsafe fn addFunctionWithDescriptor_library_error_(
        &self,
        descriptor: MTLFunctionDescriptor,
        library: *mut u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addFunctionWithDescriptor : descriptor, library : library, error : error)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLIntersectionFunctionBufferArguments {
    pub intersectionFunctionBuffer: u64,
    pub intersectionFunctionBufferSize: u64,
    pub intersectionFunctionStride: u64,
}
pub type MTLIntersectionFunctionSignature = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLIntersectionFunctionTableDescriptor(pub id);
impl std::ops::Deref for MTLIntersectionFunctionTableDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLIntersectionFunctionTableDescriptor {}
impl MTLIntersectionFunctionTableDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLIntersectionFunctionTableDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLIntersectionFunctionTableDescriptor {}
impl INSObject for MTLIntersectionFunctionTableDescriptor {}
impl PNSObject for MTLIntersectionFunctionTableDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLIntersectionFunctionTableDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLIntersectionFunctionTableDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLIntersectionFunctionTableDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLIntersectionFunctionTableDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLIntersectionFunctionTableDescriptor")
        }
    }
}
impl IMTLIntersectionFunctionTableDescriptor for MTLIntersectionFunctionTableDescriptor {}
pub trait IMTLIntersectionFunctionTableDescriptor: Sized + std::ops::Deref {
    unsafe fn functionCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionCount)
    }
    unsafe fn setFunctionCount_(&self, functionCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionCount : functionCount)
    }
    unsafe fn intersectionFunctionTableDescriptor() -> MTLIntersectionFunctionTableDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLIntersectionFunctionTableDescriptor").unwrap(), intersectionFunctionTableDescriptor)
    }
}
pub trait PMTLIntersectionFunctionTable: Sized + std::ops::Deref {
    unsafe fn setBuffer_offset_atIndex_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBuffer : buffer, offset : offset, atIndex : index)
    }
    unsafe fn setBuffers_offsets_withRange_(
        &self,
        buffers: *const *mut u64,
        offsets: *const NSUInteger,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBuffers : buffers, offsets : offsets, withRange : range)
    }
    unsafe fn setFunction_atIndex_(&self, function: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunction : function, atIndex : index)
    }
    unsafe fn setFunctions_withRange_(&self, functions: *const *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctions : functions, withRange : range)
    }
    unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_atIndex_(
        &self,
        signature: MTLIntersectionFunctionSignature,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpaqueTriangleIntersectionFunctionWithSignature : signature, atIndex : index)
    }
    unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_withRange_(
        &self,
        signature: MTLIntersectionFunctionSignature,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpaqueTriangleIntersectionFunctionWithSignature : signature, withRange : range)
    }
    unsafe fn setOpaqueCurveIntersectionFunctionWithSignature_atIndex_(
        &self,
        signature: MTLIntersectionFunctionSignature,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpaqueCurveIntersectionFunctionWithSignature : signature, atIndex : index)
    }
    unsafe fn setOpaqueCurveIntersectionFunctionWithSignature_withRange_(
        &self,
        signature: MTLIntersectionFunctionSignature,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpaqueCurveIntersectionFunctionWithSignature : signature, withRange : range)
    }
    unsafe fn setVisibleFunctionTable_atBufferIndex_(
        &self,
        functionTable: *mut u64,
        bufferIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibleFunctionTable : functionTable, atBufferIndex : bufferIndex)
    }
    unsafe fn setVisibleFunctionTables_withBufferRange_(
        &self,
        functionTables: *const *mut u64,
        bufferRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibleFunctionTables : functionTables, withBufferRange : bufferRange)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
}
pub type MTLStitchedLibraryOptions = NSUInteger;
pub trait PMTLFunctionStitchingAttribute: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFunctionStitchingAttributeAlwaysInline(pub id);
impl std::ops::Deref for MTLFunctionStitchingAttributeAlwaysInline {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFunctionStitchingAttributeAlwaysInline {}
impl MTLFunctionStitchingAttributeAlwaysInline {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFunctionStitchingAttributeAlwaysInline").unwrap(), alloc) })
    }
}
impl PMTLFunctionStitchingAttribute for MTLFunctionStitchingAttributeAlwaysInline {}
impl INSObject for MTLFunctionStitchingAttributeAlwaysInline {}
impl PNSObject for MTLFunctionStitchingAttributeAlwaysInline {}
impl std::convert::TryFrom<NSObject> for MTLFunctionStitchingAttributeAlwaysInline {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTLFunctionStitchingAttributeAlwaysInline, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFunctionStitchingAttributeAlwaysInline").unwrap())
        };
        if is_kind_of {
            Ok(MTLFunctionStitchingAttributeAlwaysInline(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFunctionStitchingAttributeAlwaysInline")
        }
    }
}
impl IMTLFunctionStitchingAttributeAlwaysInline for MTLFunctionStitchingAttributeAlwaysInline {}
pub trait IMTLFunctionStitchingAttributeAlwaysInline: Sized + std::ops::Deref {}
pub trait PMTLFunctionStitchingNode: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFunctionStitchingInputNode(pub id);
impl std::ops::Deref for MTLFunctionStitchingInputNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFunctionStitchingInputNode {}
impl MTLFunctionStitchingInputNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFunctionStitchingInputNode").unwrap(), alloc) })
    }
}
impl PMTLFunctionStitchingNode for MTLFunctionStitchingInputNode {}
impl INSObject for MTLFunctionStitchingInputNode {}
impl PNSObject for MTLFunctionStitchingInputNode {}
impl std::convert::TryFrom<NSObject> for MTLFunctionStitchingInputNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFunctionStitchingInputNode, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFunctionStitchingInputNode").unwrap())
        };
        if is_kind_of {
            Ok(MTLFunctionStitchingInputNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFunctionStitchingInputNode")
        }
    }
}
impl IMTLFunctionStitchingInputNode for MTLFunctionStitchingInputNode {}
pub trait IMTLFunctionStitchingInputNode: Sized + std::ops::Deref {
    unsafe fn initWithArgumentIndex_(&self, argument: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithArgumentIndex : argument)
    }
    unsafe fn argumentIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentIndex)
    }
    unsafe fn setArgumentIndex_(&self, argumentIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArgumentIndex : argumentIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFunctionStitchingFunctionNode(pub id);
impl std::ops::Deref for MTLFunctionStitchingFunctionNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFunctionStitchingFunctionNode {}
impl MTLFunctionStitchingFunctionNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFunctionStitchingFunctionNode").unwrap(), alloc) })
    }
}
impl PMTLFunctionStitchingNode for MTLFunctionStitchingFunctionNode {}
impl INSObject for MTLFunctionStitchingFunctionNode {}
impl PNSObject for MTLFunctionStitchingFunctionNode {}
impl std::convert::TryFrom<NSObject> for MTLFunctionStitchingFunctionNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFunctionStitchingFunctionNode, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFunctionStitchingFunctionNode").unwrap())
        };
        if is_kind_of {
            Ok(MTLFunctionStitchingFunctionNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFunctionStitchingFunctionNode")
        }
    }
}
impl IMTLFunctionStitchingFunctionNode for MTLFunctionStitchingFunctionNode {}
pub trait IMTLFunctionStitchingFunctionNode: Sized + std::ops::Deref {
    unsafe fn initWithName_arguments_controlDependencies_(
        &self,
        name: NSString,
        arguments: NSArray,
        controlDependencies: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, arguments : arguments, controlDependencies : controlDependencies)
    }
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
    unsafe fn arguments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arguments)
    }
    unsafe fn setArguments_(&self, arguments: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArguments : arguments)
    }
    unsafe fn controlDependencies(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlDependencies)
    }
    unsafe fn setControlDependencies_(&self, controlDependencies: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlDependencies : controlDependencies)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFunctionStitchingGraph(pub id);
impl std::ops::Deref for MTLFunctionStitchingGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFunctionStitchingGraph {}
impl MTLFunctionStitchingGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFunctionStitchingGraph").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLFunctionStitchingGraph {}
impl INSObject for MTLFunctionStitchingGraph {}
impl PNSObject for MTLFunctionStitchingGraph {}
impl std::convert::TryFrom<NSObject> for MTLFunctionStitchingGraph {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFunctionStitchingGraph, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFunctionStitchingGraph").unwrap()) };
        if is_kind_of {
            Ok(MTLFunctionStitchingGraph(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFunctionStitchingGraph")
        }
    }
}
impl IMTLFunctionStitchingGraph for MTLFunctionStitchingGraph {}
pub trait IMTLFunctionStitchingGraph: Sized + std::ops::Deref {
    unsafe fn initWithFunctionName_nodes_outputNode_attributes_(
        &self,
        functionName: NSString,
        nodes: NSArray,
        outputNode: MTLFunctionStitchingFunctionNode,
        attributes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFunctionName : functionName, nodes : nodes, outputNode : outputNode, attributes : attributes)
    }
    unsafe fn functionName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionName)
    }
    unsafe fn setFunctionName_(&self, functionName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionName : functionName)
    }
    unsafe fn nodes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodes)
    }
    unsafe fn setNodes_(&self, nodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNodes : nodes)
    }
    unsafe fn outputNode(&self) -> MTLFunctionStitchingFunctionNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputNode)
    }
    unsafe fn setOutputNode_(&self, outputNode: MTLFunctionStitchingFunctionNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputNode : outputNode)
    }
    unsafe fn attributes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn setAttributes_(&self, attributes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributes : attributes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLStitchedLibraryDescriptor(pub id);
impl std::ops::Deref for MTLStitchedLibraryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLStitchedLibraryDescriptor {}
impl MTLStitchedLibraryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLStitchedLibraryDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLStitchedLibraryDescriptor {}
impl INSObject for MTLStitchedLibraryDescriptor {}
impl PNSObject for MTLStitchedLibraryDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLStitchedLibraryDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLStitchedLibraryDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLStitchedLibraryDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLStitchedLibraryDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLStitchedLibraryDescriptor")
        }
    }
}
impl IMTLStitchedLibraryDescriptor for MTLStitchedLibraryDescriptor {}
pub trait IMTLStitchedLibraryDescriptor: Sized + std::ops::Deref {
    unsafe fn functionGraphs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionGraphs)
    }
    unsafe fn setFunctionGraphs_(&self, functionGraphs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionGraphs : functionGraphs)
    }
    unsafe fn functions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functions)
    }
    unsafe fn setFunctions_(&self, functions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctions : functions)
    }
    unsafe fn binaryArchives(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, binaryArchives)
    }
    unsafe fn setBinaryArchives_(&self, binaryArchives: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBinaryArchives : binaryArchives)
    }
    unsafe fn options(&self) -> MTLStitchedLibraryOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: MTLStitchedLibraryOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
}
pub type MTLIOPriority = NSInteger;
pub type MTLIOCommandQueueType = NSInteger;
pub type MTLIOError = NSInteger;
pub trait PMTLIOCommandQueue: Sized + std::ops::Deref {
    unsafe fn enqueueBarrier(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enqueueBarrier)
    }
    unsafe fn commandBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandBuffer)
    }
    unsafe fn commandBufferWithUnretainedReferences(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandBufferWithUnretainedReferences)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub trait PMTLIOScratchBuffer: Sized + std::ops::Deref {
    unsafe fn buffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buffer)
    }
}
pub trait PMTLIOScratchBufferAllocator: Sized + std::ops::Deref {
    unsafe fn newScratchBufferWithMinimumSize_(&self, minimumSize: NSUInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newScratchBufferWithMinimumSize : minimumSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLIOCommandQueueDescriptor(pub id);
impl std::ops::Deref for MTLIOCommandQueueDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLIOCommandQueueDescriptor {}
impl MTLIOCommandQueueDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLIOCommandQueueDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLIOCommandQueueDescriptor {}
impl INSObject for MTLIOCommandQueueDescriptor {}
impl PNSObject for MTLIOCommandQueueDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLIOCommandQueueDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLIOCommandQueueDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLIOCommandQueueDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLIOCommandQueueDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLIOCommandQueueDescriptor")
        }
    }
}
impl IMTLIOCommandQueueDescriptor for MTLIOCommandQueueDescriptor {}
pub trait IMTLIOCommandQueueDescriptor: Sized + std::ops::Deref {
    unsafe fn maxCommandBufferCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxCommandBufferCount)
    }
    unsafe fn setMaxCommandBufferCount_(&self, maxCommandBufferCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxCommandBufferCount : maxCommandBufferCount)
    }
    unsafe fn priority(&self) -> MTLIOPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, priority)
    }
    unsafe fn setPriority_(&self, priority: MTLIOPriority)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPriority : priority)
    }
    unsafe fn type_(&self) -> MTLIOCommandQueueType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: MTLIOCommandQueueType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn maxCommandsInFlight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxCommandsInFlight)
    }
    unsafe fn setMaxCommandsInFlight_(&self, maxCommandsInFlight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxCommandsInFlight : maxCommandsInFlight)
    }
    unsafe fn scratchBufferAllocator(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scratchBufferAllocator)
    }
    unsafe fn setScratchBufferAllocator_(&self, scratchBufferAllocator: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScratchBufferAllocator : scratchBufferAllocator)
    }
}
pub trait PMTLIOFileHandle: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub type MTLIOStatus = NSInteger;
pub type MTLIOCommandBufferHandler = *mut ::std::os::raw::c_void;
pub trait PMTLIOCommandBuffer: Sized + std::ops::Deref {
    unsafe fn addCompletedHandler_(&self, block: MTLIOCommandBufferHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addCompletedHandler : block)
    }
    unsafe fn loadBytes_size_sourceHandle_sourceHandleOffset_(
        &self,
        pointer: *mut ::std::os::raw::c_void,
        size: NSUInteger,
        sourceHandle: *mut u64,
        sourceHandleOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadBytes : pointer, size : size, sourceHandle : sourceHandle, sourceHandleOffset : sourceHandleOffset)
    }
    unsafe fn loadBuffer_offset_size_sourceHandle_sourceHandleOffset_(
        &self,
        buffer: *mut u64,
        offset: NSUInteger,
        size: NSUInteger,
        sourceHandle: *mut u64,
        sourceHandleOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadBuffer : buffer, offset : offset, size : size, sourceHandle : sourceHandle, sourceHandleOffset : sourceHandleOffset)
    }
    unsafe fn loadTexture_slice_level_size_sourceBytesPerRow_sourceBytesPerImage_destinationOrigin_sourceHandle_sourceHandleOffset_(
        &self,
        texture: *mut u64,
        slice: NSUInteger,
        level: NSUInteger,
        size: MTLSize,
        sourceBytesPerRow: NSUInteger,
        sourceBytesPerImage: NSUInteger,
        destinationOrigin: MTLOrigin,
        sourceHandle: *mut u64,
        sourceHandleOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadTexture : texture, slice : slice, level : level, size : size, sourceBytesPerRow : sourceBytesPerRow, sourceBytesPerImage : sourceBytesPerImage, destinationOrigin : destinationOrigin, sourceHandle : sourceHandle, sourceHandleOffset : sourceHandleOffset)
    }
    unsafe fn copyStatusToBuffer_offset_(&self, buffer: *mut u64, offset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyStatusToBuffer : buffer, offset : offset)
    }
    unsafe fn commit(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commit)
    }
    unsafe fn waitUntilCompleted(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, waitUntilCompleted)
    }
    unsafe fn tryCancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tryCancel)
    }
    unsafe fn addBarrier(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addBarrier)
    }
    unsafe fn pushDebugGroup_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushDebugGroup : string)
    }
    unsafe fn popDebugGroup(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, popDebugGroup)
    }
    unsafe fn enqueue(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enqueue)
    }
    unsafe fn waitForEvent_value_(&self, event: *mut u64, value: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForEvent : event, value : value)
    }
    unsafe fn signalEvent_value_(&self, event: *mut u64, value: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signalEvent : event, value : value)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn status(&self) -> MTLIOStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
}
pub type MTLIOCompressionStatus = NSInteger;
pub type MTLIOCompressionContext = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLResidencySetDescriptor(pub id);
impl std::ops::Deref for MTLResidencySetDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLResidencySetDescriptor {}
impl MTLResidencySetDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLResidencySetDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLResidencySetDescriptor {}
impl INSObject for MTLResidencySetDescriptor {}
impl PNSObject for MTLResidencySetDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLResidencySetDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLResidencySetDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLResidencySetDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLResidencySetDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLResidencySetDescriptor")
        }
    }
}
impl IMTLResidencySetDescriptor for MTLResidencySetDescriptor {}
pub trait IMTLResidencySetDescriptor: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn initialCapacity(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialCapacity)
    }
    unsafe fn setInitialCapacity_(&self, initialCapacity: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialCapacity : initialCapacity)
    }
}
pub trait PMTLResidencySet: Sized + std::ops::Deref {
    unsafe fn requestResidency(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestResidency)
    }
    unsafe fn endResidency(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endResidency)
    }
    unsafe fn addAllocation_(&self, allocation: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAllocation : allocation)
    }
    unsafe fn addAllocations_count_(&self, allocations: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAllocations : allocations, count : count)
    }
    unsafe fn removeAllocation_(&self, allocation: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAllocation : allocation)
    }
    unsafe fn removeAllocations_count_(&self, allocations: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAllocations : allocations, count : count)
    }
    unsafe fn removeAllAllocations(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllAllocations)
    }
    unsafe fn containsAllocation_(&self, anAllocation: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsAllocation : anAllocation)
    }
    unsafe fn commit(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commit)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn allocatedSize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocatedSize)
    }
    unsafe fn allAllocations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allAllocations)
    }
    unsafe fn allocationCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocationCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLResourceViewPoolDescriptor(pub id);
impl std::ops::Deref for MTLResourceViewPoolDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLResourceViewPoolDescriptor {}
impl MTLResourceViewPoolDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLResourceViewPoolDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLResourceViewPoolDescriptor {}
impl INSObject for MTLResourceViewPoolDescriptor {}
impl PNSObject for MTLResourceViewPoolDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLResourceViewPoolDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLResourceViewPoolDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLResourceViewPoolDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLResourceViewPoolDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLResourceViewPoolDescriptor")
        }
    }
}
impl IMTLResourceViewPoolDescriptor for MTLResourceViewPoolDescriptor {}
pub trait IMTLResourceViewPoolDescriptor: Sized + std::ops::Deref {
    unsafe fn resourceViewCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceViewCount)
    }
    unsafe fn setResourceViewCount_(&self, resourceViewCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResourceViewCount : resourceViewCount)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub trait PMTLResourceViewPool: Sized + std::ops::Deref {
    unsafe fn copyResourceViewsFromPool_sourceRange_destinationIndex_(
        &self,
        sourcePool: *mut u64,
        sourceRange: NSRange,
        destinationIndex: NSUInteger,
    ) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyResourceViewsFromPool : sourcePool, sourceRange : sourceRange, destinationIndex : destinationIndex)
    }
    unsafe fn baseResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseResourceID)
    }
    unsafe fn resourceViewCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceViewCount)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
}
pub trait PMTLTextureViewPool: Sized + std::ops::Deref {
    unsafe fn setTextureView_atIndex_(&self, texture: *mut u64, index: NSUInteger) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureView : texture, atIndex : index)
    }
    unsafe fn setTextureView_descriptor_atIndex_(
        &self,
        texture: *mut u64,
        descriptor: MTLTextureViewDescriptor,
        index: NSUInteger,
    ) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureView : texture, descriptor : descriptor, atIndex : index)
    }
    unsafe fn setTextureViewFromBuffer_descriptor_offset_bytesPerRow_atIndex_(
        &self,
        buffer: *mut u64,
        descriptor: MTLTextureDescriptor,
        offset: NSUInteger,
        bytesPerRow: NSUInteger,
        index: NSUInteger,
    ) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureViewFromBuffer : buffer, descriptor : descriptor, offset : offset, bytesPerRow : bytesPerRow, atIndex : index)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4ArgumentTableDescriptor(pub id);
impl std::ops::Deref for MTL4ArgumentTableDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4ArgumentTableDescriptor {}
impl MTL4ArgumentTableDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4ArgumentTableDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4ArgumentTableDescriptor {}
impl INSObject for MTL4ArgumentTableDescriptor {}
impl PNSObject for MTL4ArgumentTableDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4ArgumentTableDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4ArgumentTableDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4ArgumentTableDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4ArgumentTableDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4ArgumentTableDescriptor")
        }
    }
}
impl IMTL4ArgumentTableDescriptor for MTL4ArgumentTableDescriptor {}
pub trait IMTL4ArgumentTableDescriptor: Sized + std::ops::Deref {
    unsafe fn maxBufferBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxBufferBindCount)
    }
    unsafe fn setMaxBufferBindCount_(&self, maxBufferBindCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxBufferBindCount : maxBufferBindCount)
    }
    unsafe fn maxTextureBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTextureBindCount)
    }
    unsafe fn setMaxTextureBindCount_(&self, maxTextureBindCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTextureBindCount : maxTextureBindCount)
    }
    unsafe fn maxSamplerStateBindCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxSamplerStateBindCount)
    }
    unsafe fn setMaxSamplerStateBindCount_(&self, maxSamplerStateBindCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxSamplerStateBindCount : maxSamplerStateBindCount)
    }
    unsafe fn initializeBindings(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initializeBindings)
    }
    unsafe fn setInitializeBindings_(&self, initializeBindings: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitializeBindings : initializeBindings)
    }
    unsafe fn supportAttributeStrides(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportAttributeStrides)
    }
    unsafe fn setSupportAttributeStrides_(&self, supportAttributeStrides: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportAttributeStrides : supportAttributeStrides)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub trait PMTL4ArgumentTable: Sized + std::ops::Deref {
    unsafe fn setAddress_atIndex_(&self, gpuAddress: MTLGPUAddress, bindingIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddress : gpuAddress, atIndex : bindingIndex)
    }
    unsafe fn setAddress_attributeStride_atIndex_(
        &self,
        gpuAddress: MTLGPUAddress,
        stride: NSUInteger,
        bindingIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddress : gpuAddress, attributeStride : stride, atIndex : bindingIndex)
    }
    unsafe fn setResource_atBufferIndex_(&self, resourceID: MTLResourceID, bindingIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResource : resourceID, atBufferIndex : bindingIndex)
    }
    unsafe fn setTexture_atIndex_(&self, resourceID: MTLResourceID, bindingIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTexture : resourceID, atIndex : bindingIndex)
    }
    unsafe fn setSamplerState_atIndex_(&self, resourceID: MTLResourceID, bindingIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplerState : resourceID, atIndex : bindingIndex)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
}
pub trait PMTL4BinaryFunction: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn functionType(&self) -> MTLFunctionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4CommandAllocatorDescriptor(pub id);
impl std::ops::Deref for MTL4CommandAllocatorDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4CommandAllocatorDescriptor {}
impl MTL4CommandAllocatorDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4CommandAllocatorDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4CommandAllocatorDescriptor {}
impl INSObject for MTL4CommandAllocatorDescriptor {}
impl PNSObject for MTL4CommandAllocatorDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4CommandAllocatorDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4CommandAllocatorDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4CommandAllocatorDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4CommandAllocatorDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4CommandAllocatorDescriptor")
        }
    }
}
impl IMTL4CommandAllocatorDescriptor for MTL4CommandAllocatorDescriptor {}
pub trait IMTL4CommandAllocatorDescriptor: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub trait PMTL4CommandAllocator: Sized + std::ops::Deref {
    unsafe fn allocatedSize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocatedSize)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4RenderPassDescriptor(pub id);
impl std::ops::Deref for MTL4RenderPassDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4RenderPassDescriptor {}
impl MTL4RenderPassDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4RenderPassDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4RenderPassDescriptor {}
impl INSObject for MTL4RenderPassDescriptor {}
impl PNSObject for MTL4RenderPassDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4RenderPassDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4RenderPassDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4RenderPassDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4RenderPassDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4RenderPassDescriptor")
        }
    }
}
impl IMTL4RenderPassDescriptor for MTL4RenderPassDescriptor {}
pub trait IMTL4RenderPassDescriptor: Sized + std::ops::Deref {
    unsafe fn setSamplePositions_count_(
        &self,
        positions: *const MTLSamplePosition,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplePositions : positions, count : count)
    }
    unsafe fn getSamplePositions_count_(
        &self,
        positions: *mut MTLSamplePosition,
        count: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSamplePositions : positions, count : count)
    }
    unsafe fn colorAttachments(&self) -> MTLRenderPassColorAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachments)
    }
    unsafe fn depthAttachment(&self) -> MTLRenderPassDepthAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthAttachment)
    }
    unsafe fn setDepthAttachment_(&self, depthAttachment: MTLRenderPassDepthAttachmentDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthAttachment : depthAttachment)
    }
    unsafe fn stencilAttachment(&self) -> MTLRenderPassStencilAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stencilAttachment)
    }
    unsafe fn setStencilAttachment_(
        &self,
        stencilAttachment: MTLRenderPassStencilAttachmentDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStencilAttachment : stencilAttachment)
    }
    unsafe fn renderTargetArrayLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderTargetArrayLength)
    }
    unsafe fn setRenderTargetArrayLength_(&self, renderTargetArrayLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderTargetArrayLength : renderTargetArrayLength)
    }
    unsafe fn imageblockSampleLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageblockSampleLength)
    }
    unsafe fn setImageblockSampleLength_(&self, imageblockSampleLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageblockSampleLength : imageblockSampleLength)
    }
    unsafe fn threadgroupMemoryLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadgroupMemoryLength)
    }
    unsafe fn setThreadgroupMemoryLength_(&self, threadgroupMemoryLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadgroupMemoryLength : threadgroupMemoryLength)
    }
    unsafe fn tileWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileWidth)
    }
    unsafe fn setTileWidth_(&self, tileWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileWidth : tileWidth)
    }
    unsafe fn tileHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileHeight)
    }
    unsafe fn setTileHeight_(&self, tileHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileHeight : tileHeight)
    }
    unsafe fn defaultRasterSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultRasterSampleCount)
    }
    unsafe fn setDefaultRasterSampleCount_(&self, defaultRasterSampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultRasterSampleCount : defaultRasterSampleCount)
    }
    unsafe fn renderTargetWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderTargetWidth)
    }
    unsafe fn setRenderTargetWidth_(&self, renderTargetWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderTargetWidth : renderTargetWidth)
    }
    unsafe fn renderTargetHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderTargetHeight)
    }
    unsafe fn setRenderTargetHeight_(&self, renderTargetHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderTargetHeight : renderTargetHeight)
    }
    unsafe fn rasterizationRateMap(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rasterizationRateMap)
    }
    unsafe fn setRasterizationRateMap_(&self, rasterizationRateMap: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterizationRateMap : rasterizationRateMap)
    }
    unsafe fn visibilityResultBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibilityResultBuffer)
    }
    unsafe fn setVisibilityResultBuffer_(&self, visibilityResultBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibilityResultBuffer : visibilityResultBuffer)
    }
    unsafe fn visibilityResultType(&self) -> MTLVisibilityResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibilityResultType)
    }
    unsafe fn setVisibilityResultType_(&self, visibilityResultType: MTLVisibilityResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibilityResultType : visibilityResultType)
    }
    unsafe fn supportColorAttachmentMapping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportColorAttachmentMapping)
    }
    unsafe fn setSupportColorAttachmentMapping_(&self, supportColorAttachmentMapping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportColorAttachmentMapping : supportColorAttachmentMapping)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4AccelerationStructureDescriptor(pub id);
impl std::ops::Deref for MTL4AccelerationStructureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4AccelerationStructureDescriptor {}
impl MTL4AccelerationStructureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureDescriptor").unwrap(), alloc) })
    }
}
impl IMTLAccelerationStructureDescriptor for MTL4AccelerationStructureDescriptor {}
impl PNSCopying for MTL4AccelerationStructureDescriptor {}
impl std::convert::TryFrom<MTLAccelerationStructureDescriptor>
    for MTL4AccelerationStructureDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTLAccelerationStructureDescriptor,
    ) -> Result<MTL4AccelerationStructureDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4AccelerationStructureDescriptor(parent.0))
        } else {
            Err ("This MTLAccelerationStructureDescriptor cannot be downcasted to MTL4AccelerationStructureDescriptor" ,)
        }
    }
}
impl INSObject for MTL4AccelerationStructureDescriptor {}
impl PNSObject for MTL4AccelerationStructureDescriptor {}
impl IMTL4AccelerationStructureDescriptor for MTL4AccelerationStructureDescriptor {}
pub trait IMTL4AccelerationStructureDescriptor: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4AccelerationStructureGeometryDescriptor(pub id);
impl std::ops::Deref for MTL4AccelerationStructureGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4AccelerationStructureGeometryDescriptor {}
impl MTL4AccelerationStructureGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureGeometryDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4AccelerationStructureGeometryDescriptor {}
impl INSObject for MTL4AccelerationStructureGeometryDescriptor {}
impl PNSObject for MTL4AccelerationStructureGeometryDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4AccelerationStructureGeometryDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTL4AccelerationStructureGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4AccelerationStructureGeometryDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4AccelerationStructureGeometryDescriptor")
        }
    }
}
impl IMTL4AccelerationStructureGeometryDescriptor for MTL4AccelerationStructureGeometryDescriptor {}
pub trait IMTL4AccelerationStructureGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn intersectionFunctionTableOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intersectionFunctionTableOffset)
    }
    unsafe fn setIntersectionFunctionTableOffset_(
        &self,
        intersectionFunctionTableOffset: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntersectionFunctionTableOffset : intersectionFunctionTableOffset)
    }
    unsafe fn opaque(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opaque)
    }
    unsafe fn setOpaque_(&self, opaque: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpaque : opaque)
    }
    unsafe fn allowDuplicateIntersectionFunctionInvocation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowDuplicateIntersectionFunctionInvocation)
    }
    unsafe fn setAllowDuplicateIntersectionFunctionInvocation_(
        &self,
        allowDuplicateIntersectionFunctionInvocation: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowDuplicateIntersectionFunctionInvocation : allowDuplicateIntersectionFunctionInvocation)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn primitiveDataBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveDataBuffer)
    }
    unsafe fn setPrimitiveDataBuffer_(&self, primitiveDataBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimitiveDataBuffer : primitiveDataBuffer)
    }
    unsafe fn primitiveDataStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveDataStride)
    }
    unsafe fn setPrimitiveDataStride_(&self, primitiveDataStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimitiveDataStride : primitiveDataStride)
    }
    unsafe fn primitiveDataElementSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveDataElementSize)
    }
    unsafe fn setPrimitiveDataElementSize_(&self, primitiveDataElementSize: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimitiveDataElementSize : primitiveDataElementSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4PrimitiveAccelerationStructureDescriptor(pub id);
impl std::ops::Deref for MTL4PrimitiveAccelerationStructureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4PrimitiveAccelerationStructureDescriptor {}
impl MTL4PrimitiveAccelerationStructureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4PrimitiveAccelerationStructureDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4AccelerationStructureDescriptor for MTL4PrimitiveAccelerationStructureDescriptor {}
impl From<MTL4PrimitiveAccelerationStructureDescriptor> for MTL4AccelerationStructureDescriptor {
    fn from(
        child: MTL4PrimitiveAccelerationStructureDescriptor,
    ) -> MTL4AccelerationStructureDescriptor {
        MTL4AccelerationStructureDescriptor(child.0)
    }
}
impl std::convert::TryFrom<MTL4AccelerationStructureDescriptor>
    for MTL4PrimitiveAccelerationStructureDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTL4AccelerationStructureDescriptor,
    ) -> Result<MTL4PrimitiveAccelerationStructureDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4PrimitiveAccelerationStructureDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4PrimitiveAccelerationStructureDescriptor(parent.0))
        } else {
            Err ("This MTL4AccelerationStructureDescriptor cannot be downcasted to MTL4PrimitiveAccelerationStructureDescriptor" ,)
        }
    }
}
impl IMTLAccelerationStructureDescriptor for MTL4PrimitiveAccelerationStructureDescriptor {}
impl PNSCopying for MTL4PrimitiveAccelerationStructureDescriptor {}
impl INSObject for MTL4PrimitiveAccelerationStructureDescriptor {}
impl PNSObject for MTL4PrimitiveAccelerationStructureDescriptor {}
impl IMTL4PrimitiveAccelerationStructureDescriptor
    for MTL4PrimitiveAccelerationStructureDescriptor
{
}
pub trait IMTL4PrimitiveAccelerationStructureDescriptor: Sized + std::ops::Deref {
    unsafe fn geometryDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometryDescriptors)
    }
    unsafe fn setGeometryDescriptors_(&self, geometryDescriptors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeometryDescriptors : geometryDescriptors)
    }
    unsafe fn motionStartBorderMode(&self) -> MTLMotionBorderMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionStartBorderMode)
    }
    unsafe fn setMotionStartBorderMode_(&self, motionStartBorderMode: MTLMotionBorderMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionStartBorderMode : motionStartBorderMode)
    }
    unsafe fn motionEndBorderMode(&self) -> MTLMotionBorderMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionEndBorderMode)
    }
    unsafe fn setMotionEndBorderMode_(&self, motionEndBorderMode: MTLMotionBorderMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionEndBorderMode : motionEndBorderMode)
    }
    unsafe fn motionStartTime(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionStartTime)
    }
    unsafe fn setMotionStartTime_(&self, motionStartTime: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionStartTime : motionStartTime)
    }
    unsafe fn motionEndTime(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionEndTime)
    }
    unsafe fn setMotionEndTime_(&self, motionEndTime: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionEndTime : motionEndTime)
    }
    unsafe fn motionKeyframeCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionKeyframeCount)
    }
    unsafe fn setMotionKeyframeCount_(&self, motionKeyframeCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionKeyframeCount : motionKeyframeCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4AccelerationStructureTriangleGeometryDescriptor(pub id);
impl std::ops::Deref for MTL4AccelerationStructureTriangleGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4AccelerationStructureTriangleGeometryDescriptor {}
impl MTL4AccelerationStructureTriangleGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureTriangleGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTL4AccelerationStructureGeometryDescriptor
    for MTL4AccelerationStructureTriangleGeometryDescriptor
{
}
impl PNSCopying for MTL4AccelerationStructureTriangleGeometryDescriptor {}
impl From<MTL4AccelerationStructureTriangleGeometryDescriptor>
    for MTL4AccelerationStructureGeometryDescriptor
{
    fn from(
        child: MTL4AccelerationStructureTriangleGeometryDescriptor,
    ) -> MTL4AccelerationStructureGeometryDescriptor {
        MTL4AccelerationStructureGeometryDescriptor(child.0)
    }
}
impl std::convert::TryFrom<MTL4AccelerationStructureGeometryDescriptor>
    for MTL4AccelerationStructureTriangleGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTL4AccelerationStructureGeometryDescriptor,
    ) -> Result<MTL4AccelerationStructureTriangleGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureTriangleGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4AccelerationStructureTriangleGeometryDescriptor(
                parent.0,
            ))
        } else {
            Err ("This MTL4AccelerationStructureGeometryDescriptor cannot be downcasted to MTL4AccelerationStructureTriangleGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTL4AccelerationStructureTriangleGeometryDescriptor {}
impl PNSObject for MTL4AccelerationStructureTriangleGeometryDescriptor {}
impl IMTL4AccelerationStructureTriangleGeometryDescriptor
    for MTL4AccelerationStructureTriangleGeometryDescriptor
{
}
pub trait IMTL4AccelerationStructureTriangleGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn vertexBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexBuffer)
    }
    unsafe fn setVertexBuffer_(&self, vertexBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffer : vertexBuffer)
    }
    unsafe fn vertexFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexFormat)
    }
    unsafe fn setVertexFormat_(&self, vertexFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexFormat : vertexFormat)
    }
    unsafe fn vertexStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexStride)
    }
    unsafe fn setVertexStride_(&self, vertexStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexStride : vertexStride)
    }
    unsafe fn indexBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn setIndexBuffer_(&self, indexBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBuffer : indexBuffer)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn setIndexType_(&self, indexType: MTLIndexType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexType : indexType)
    }
    unsafe fn triangleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triangleCount)
    }
    unsafe fn setTriangleCount_(&self, triangleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTriangleCount : triangleCount)
    }
    unsafe fn transformationMatrixBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixBuffer)
    }
    unsafe fn setTransformationMatrixBuffer_(&self, transformationMatrixBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixBuffer : transformationMatrixBuffer)
    }
    unsafe fn transformationMatrixLayout(&self) -> MTLMatrixLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixLayout)
    }
    unsafe fn setTransformationMatrixLayout_(&self, transformationMatrixLayout: MTLMatrixLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixLayout : transformationMatrixLayout)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4AccelerationStructureBoundingBoxGeometryDescriptor(pub id);
impl std::ops::Deref for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {}
impl MTL4AccelerationStructureBoundingBoxGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureBoundingBoxGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTL4AccelerationStructureGeometryDescriptor
    for MTL4AccelerationStructureBoundingBoxGeometryDescriptor
{
}
impl PNSCopying for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {}
impl std::convert::TryFrom<MTL4AccelerationStructureGeometryDescriptor>
    for MTL4AccelerationStructureBoundingBoxGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTL4AccelerationStructureGeometryDescriptor,
    ) -> Result<MTL4AccelerationStructureBoundingBoxGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureBoundingBoxGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4AccelerationStructureBoundingBoxGeometryDescriptor(
                parent.0,
            ))
        } else {
            Err ("This MTL4AccelerationStructureGeometryDescriptor cannot be downcasted to MTL4AccelerationStructureBoundingBoxGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {}
impl PNSObject for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {}
impl IMTL4AccelerationStructureBoundingBoxGeometryDescriptor
    for MTL4AccelerationStructureBoundingBoxGeometryDescriptor
{
}
pub trait IMTL4AccelerationStructureBoundingBoxGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn boundingBoxBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxBuffer)
    }
    unsafe fn setBoundingBoxBuffer_(&self, boundingBoxBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxBuffer : boundingBoxBuffer)
    }
    unsafe fn boundingBoxStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxStride)
    }
    unsafe fn setBoundingBoxStride_(&self, boundingBoxStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxStride : boundingBoxStride)
    }
    unsafe fn boundingBoxCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxCount)
    }
    unsafe fn setBoundingBoxCount_(&self, boundingBoxCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxCount : boundingBoxCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4AccelerationStructureMotionTriangleGeometryDescriptor(pub id);
impl std::ops::Deref for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {}
impl MTL4AccelerationStructureMotionTriangleGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureMotionTriangleGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTL4AccelerationStructureGeometryDescriptor
    for MTL4AccelerationStructureMotionTriangleGeometryDescriptor
{
}
impl PNSCopying for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {}
impl std::convert::TryFrom<MTL4AccelerationStructureGeometryDescriptor>
    for MTL4AccelerationStructureMotionTriangleGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTL4AccelerationStructureGeometryDescriptor,
    ) -> Result<MTL4AccelerationStructureMotionTriangleGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureMotionTriangleGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4AccelerationStructureMotionTriangleGeometryDescriptor(
                parent.0,
            ))
        } else {
            Err ("This MTL4AccelerationStructureGeometryDescriptor cannot be downcasted to MTL4AccelerationStructureMotionTriangleGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {}
impl PNSObject for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {}
impl IMTL4AccelerationStructureMotionTriangleGeometryDescriptor
    for MTL4AccelerationStructureMotionTriangleGeometryDescriptor
{
}
pub trait IMTL4AccelerationStructureMotionTriangleGeometryDescriptor:
    Sized + std::ops::Deref
{
    unsafe fn vertexBuffers(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexBuffers)
    }
    unsafe fn setVertexBuffers_(&self, vertexBuffers: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexBuffers : vertexBuffers)
    }
    unsafe fn vertexFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexFormat)
    }
    unsafe fn setVertexFormat_(&self, vertexFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexFormat : vertexFormat)
    }
    unsafe fn vertexStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexStride)
    }
    unsafe fn setVertexStride_(&self, vertexStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexStride : vertexStride)
    }
    unsafe fn indexBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn setIndexBuffer_(&self, indexBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBuffer : indexBuffer)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn setIndexType_(&self, indexType: MTLIndexType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexType : indexType)
    }
    unsafe fn triangleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triangleCount)
    }
    unsafe fn setTriangleCount_(&self, triangleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTriangleCount : triangleCount)
    }
    unsafe fn transformationMatrixBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixBuffer)
    }
    unsafe fn setTransformationMatrixBuffer_(&self, transformationMatrixBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixBuffer : transformationMatrixBuffer)
    }
    unsafe fn transformationMatrixLayout(&self) -> MTLMatrixLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformationMatrixLayout)
    }
    unsafe fn setTransformationMatrixLayout_(&self, transformationMatrixLayout: MTLMatrixLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransformationMatrixLayout : transformationMatrixLayout)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor(pub id);
impl std::ops::Deref for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {}
impl MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTL4AccelerationStructureGeometryDescriptor
    for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor
{
}
impl PNSCopying for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {}
impl std::convert::TryFrom<MTL4AccelerationStructureGeometryDescriptor>
    for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTL4AccelerationStructureGeometryDescriptor,
    ) -> Result<MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor(parent.0))
        } else {
            Err ("This MTL4AccelerationStructureGeometryDescriptor cannot be downcasted to MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {}
impl PNSObject for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {}
impl IMTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor
    for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor
{
}
pub trait IMTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor:
    Sized + std::ops::Deref
{
    unsafe fn boundingBoxBuffers(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxBuffers)
    }
    unsafe fn setBoundingBoxBuffers_(&self, boundingBoxBuffers: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxBuffers : boundingBoxBuffers)
    }
    unsafe fn boundingBoxStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxStride)
    }
    unsafe fn setBoundingBoxStride_(&self, boundingBoxStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxStride : boundingBoxStride)
    }
    unsafe fn boundingBoxCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBoxCount)
    }
    unsafe fn setBoundingBoxCount_(&self, boundingBoxCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBoxCount : boundingBoxCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4AccelerationStructureCurveGeometryDescriptor(pub id);
impl std::ops::Deref for MTL4AccelerationStructureCurveGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4AccelerationStructureCurveGeometryDescriptor {}
impl MTL4AccelerationStructureCurveGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureCurveGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTL4AccelerationStructureGeometryDescriptor
    for MTL4AccelerationStructureCurveGeometryDescriptor
{
}
impl PNSCopying for MTL4AccelerationStructureCurveGeometryDescriptor {}
impl std::convert::TryFrom<MTL4AccelerationStructureGeometryDescriptor>
    for MTL4AccelerationStructureCurveGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTL4AccelerationStructureGeometryDescriptor,
    ) -> Result<MTL4AccelerationStructureCurveGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureCurveGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4AccelerationStructureCurveGeometryDescriptor(parent.0))
        } else {
            Err ("This MTL4AccelerationStructureGeometryDescriptor cannot be downcasted to MTL4AccelerationStructureCurveGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTL4AccelerationStructureCurveGeometryDescriptor {}
impl PNSObject for MTL4AccelerationStructureCurveGeometryDescriptor {}
impl IMTL4AccelerationStructureCurveGeometryDescriptor
    for MTL4AccelerationStructureCurveGeometryDescriptor
{
}
pub trait IMTL4AccelerationStructureCurveGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn controlPointBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointBuffer)
    }
    unsafe fn setControlPointBuffer_(&self, controlPointBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointBuffer : controlPointBuffer)
    }
    unsafe fn controlPointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointCount)
    }
    unsafe fn setControlPointCount_(&self, controlPointCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointCount : controlPointCount)
    }
    unsafe fn controlPointStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointStride)
    }
    unsafe fn setControlPointStride_(&self, controlPointStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointStride : controlPointStride)
    }
    unsafe fn controlPointFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointFormat)
    }
    unsafe fn setControlPointFormat_(&self, controlPointFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointFormat : controlPointFormat)
    }
    unsafe fn radiusBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusBuffer)
    }
    unsafe fn setRadiusBuffer_(&self, radiusBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusBuffer : radiusBuffer)
    }
    unsafe fn radiusFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusFormat)
    }
    unsafe fn setRadiusFormat_(&self, radiusFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusFormat : radiusFormat)
    }
    unsafe fn radiusStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusStride)
    }
    unsafe fn setRadiusStride_(&self, radiusStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusStride : radiusStride)
    }
    unsafe fn indexBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn setIndexBuffer_(&self, indexBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBuffer : indexBuffer)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn setIndexType_(&self, indexType: MTLIndexType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexType : indexType)
    }
    unsafe fn segmentCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentCount)
    }
    unsafe fn setSegmentCount_(&self, segmentCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSegmentCount : segmentCount)
    }
    unsafe fn segmentControlPointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentControlPointCount)
    }
    unsafe fn setSegmentControlPointCount_(&self, segmentControlPointCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSegmentControlPointCount : segmentControlPointCount)
    }
    unsafe fn curveType(&self) -> MTLCurveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveType)
    }
    unsafe fn setCurveType_(&self, curveType: MTLCurveType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveType : curveType)
    }
    unsafe fn curveBasis(&self) -> MTLCurveBasis
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveBasis)
    }
    unsafe fn setCurveBasis_(&self, curveBasis: MTLCurveBasis)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveBasis : curveBasis)
    }
    unsafe fn curveEndCaps(&self) -> MTLCurveEndCaps
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveEndCaps)
    }
    unsafe fn setCurveEndCaps_(&self, curveEndCaps: MTLCurveEndCaps)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveEndCaps : curveEndCaps)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4AccelerationStructureMotionCurveGeometryDescriptor(pub id);
impl std::ops::Deref for MTL4AccelerationStructureMotionCurveGeometryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4AccelerationStructureMotionCurveGeometryDescriptor {}
impl MTL4AccelerationStructureMotionCurveGeometryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureMotionCurveGeometryDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTL4AccelerationStructureGeometryDescriptor
    for MTL4AccelerationStructureMotionCurveGeometryDescriptor
{
}
impl PNSCopying for MTL4AccelerationStructureMotionCurveGeometryDescriptor {}
impl std::convert::TryFrom<MTL4AccelerationStructureGeometryDescriptor>
    for MTL4AccelerationStructureMotionCurveGeometryDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTL4AccelerationStructureGeometryDescriptor,
    ) -> Result<MTL4AccelerationStructureMotionCurveGeometryDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4AccelerationStructureMotionCurveGeometryDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4AccelerationStructureMotionCurveGeometryDescriptor(
                parent.0,
            ))
        } else {
            Err ("This MTL4AccelerationStructureGeometryDescriptor cannot be downcasted to MTL4AccelerationStructureMotionCurveGeometryDescriptor" ,)
        }
    }
}
impl INSObject for MTL4AccelerationStructureMotionCurveGeometryDescriptor {}
impl PNSObject for MTL4AccelerationStructureMotionCurveGeometryDescriptor {}
impl IMTL4AccelerationStructureMotionCurveGeometryDescriptor
    for MTL4AccelerationStructureMotionCurveGeometryDescriptor
{
}
pub trait IMTL4AccelerationStructureMotionCurveGeometryDescriptor: Sized + std::ops::Deref {
    unsafe fn controlPointBuffers(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointBuffers)
    }
    unsafe fn setControlPointBuffers_(&self, controlPointBuffers: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointBuffers : controlPointBuffers)
    }
    unsafe fn controlPointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointCount)
    }
    unsafe fn setControlPointCount_(&self, controlPointCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointCount : controlPointCount)
    }
    unsafe fn controlPointStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointStride)
    }
    unsafe fn setControlPointStride_(&self, controlPointStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointStride : controlPointStride)
    }
    unsafe fn controlPointFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlPointFormat)
    }
    unsafe fn setControlPointFormat_(&self, controlPointFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlPointFormat : controlPointFormat)
    }
    unsafe fn radiusBuffers(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusBuffers)
    }
    unsafe fn setRadiusBuffers_(&self, radiusBuffers: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusBuffers : radiusBuffers)
    }
    unsafe fn radiusFormat(&self) -> MTLAttributeFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusFormat)
    }
    unsafe fn setRadiusFormat_(&self, radiusFormat: MTLAttributeFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusFormat : radiusFormat)
    }
    unsafe fn radiusStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radiusStride)
    }
    unsafe fn setRadiusStride_(&self, radiusStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadiusStride : radiusStride)
    }
    unsafe fn indexBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn setIndexBuffer_(&self, indexBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexBuffer : indexBuffer)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn setIndexType_(&self, indexType: MTLIndexType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndexType : indexType)
    }
    unsafe fn segmentCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentCount)
    }
    unsafe fn setSegmentCount_(&self, segmentCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSegmentCount : segmentCount)
    }
    unsafe fn segmentControlPointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segmentControlPointCount)
    }
    unsafe fn setSegmentControlPointCount_(&self, segmentControlPointCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSegmentControlPointCount : segmentControlPointCount)
    }
    unsafe fn curveType(&self) -> MTLCurveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveType)
    }
    unsafe fn setCurveType_(&self, curveType: MTLCurveType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveType : curveType)
    }
    unsafe fn curveBasis(&self) -> MTLCurveBasis
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveBasis)
    }
    unsafe fn setCurveBasis_(&self, curveBasis: MTLCurveBasis)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveBasis : curveBasis)
    }
    unsafe fn curveEndCaps(&self) -> MTLCurveEndCaps
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveEndCaps)
    }
    unsafe fn setCurveEndCaps_(&self, curveEndCaps: MTLCurveEndCaps)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveEndCaps : curveEndCaps)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4InstanceAccelerationStructureDescriptor(pub id);
impl std::ops::Deref for MTL4InstanceAccelerationStructureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4InstanceAccelerationStructureDescriptor {}
impl MTL4InstanceAccelerationStructureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4InstanceAccelerationStructureDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4AccelerationStructureDescriptor for MTL4InstanceAccelerationStructureDescriptor {}
impl std::convert::TryFrom<MTL4AccelerationStructureDescriptor>
    for MTL4InstanceAccelerationStructureDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTL4AccelerationStructureDescriptor,
    ) -> Result<MTL4InstanceAccelerationStructureDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4InstanceAccelerationStructureDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4InstanceAccelerationStructureDescriptor(parent.0))
        } else {
            Err ("This MTL4AccelerationStructureDescriptor cannot be downcasted to MTL4InstanceAccelerationStructureDescriptor" ,)
        }
    }
}
impl IMTLAccelerationStructureDescriptor for MTL4InstanceAccelerationStructureDescriptor {}
impl PNSCopying for MTL4InstanceAccelerationStructureDescriptor {}
impl INSObject for MTL4InstanceAccelerationStructureDescriptor {}
impl PNSObject for MTL4InstanceAccelerationStructureDescriptor {}
impl IMTL4InstanceAccelerationStructureDescriptor for MTL4InstanceAccelerationStructureDescriptor {}
pub trait IMTL4InstanceAccelerationStructureDescriptor: Sized + std::ops::Deref {
    unsafe fn instanceDescriptorBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorBuffer)
    }
    unsafe fn setInstanceDescriptorBuffer_(&self, instanceDescriptorBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorBuffer : instanceDescriptorBuffer)
    }
    unsafe fn instanceDescriptorStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorStride)
    }
    unsafe fn setInstanceDescriptorStride_(&self, instanceDescriptorStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorStride : instanceDescriptorStride)
    }
    unsafe fn instanceCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceCount)
    }
    unsafe fn setInstanceCount_(&self, instanceCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceCount : instanceCount)
    }
    unsafe fn instanceDescriptorType(&self) -> MTLAccelerationStructureInstanceDescriptorType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorType)
    }
    unsafe fn setInstanceDescriptorType_(
        &self,
        instanceDescriptorType: MTLAccelerationStructureInstanceDescriptorType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorType : instanceDescriptorType)
    }
    unsafe fn motionTransformBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformBuffer)
    }
    unsafe fn setMotionTransformBuffer_(&self, motionTransformBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformBuffer : motionTransformBuffer)
    }
    unsafe fn motionTransformCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformCount)
    }
    unsafe fn setMotionTransformCount_(&self, motionTransformCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformCount : motionTransformCount)
    }
    unsafe fn instanceTransformationMatrixLayout(&self) -> MTLMatrixLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceTransformationMatrixLayout)
    }
    unsafe fn setInstanceTransformationMatrixLayout_(
        &self,
        instanceTransformationMatrixLayout: MTLMatrixLayout,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceTransformationMatrixLayout : instanceTransformationMatrixLayout)
    }
    unsafe fn motionTransformType(&self) -> MTLTransformType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformType)
    }
    unsafe fn setMotionTransformType_(&self, motionTransformType: MTLTransformType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformType : motionTransformType)
    }
    unsafe fn motionTransformStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformStride)
    }
    unsafe fn setMotionTransformStride_(&self, motionTransformStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformStride : motionTransformStride)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4IndirectInstanceAccelerationStructureDescriptor(pub id);
impl std::ops::Deref for MTL4IndirectInstanceAccelerationStructureDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4IndirectInstanceAccelerationStructureDescriptor {}
impl MTL4IndirectInstanceAccelerationStructureDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4IndirectInstanceAccelerationStructureDescriptor").unwrap(), alloc)
        })
    }
}
impl IMTL4AccelerationStructureDescriptor for MTL4IndirectInstanceAccelerationStructureDescriptor {}
impl std::convert::TryFrom<MTL4AccelerationStructureDescriptor>
    for MTL4IndirectInstanceAccelerationStructureDescriptor
{
    type Error = &'static str;
    fn try_from(
        parent: MTL4AccelerationStructureDescriptor,
    ) -> Result<MTL4IndirectInstanceAccelerationStructureDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4IndirectInstanceAccelerationStructureDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4IndirectInstanceAccelerationStructureDescriptor(
                parent.0,
            ))
        } else {
            Err ("This MTL4AccelerationStructureDescriptor cannot be downcasted to MTL4IndirectInstanceAccelerationStructureDescriptor" ,)
        }
    }
}
impl IMTLAccelerationStructureDescriptor for MTL4IndirectInstanceAccelerationStructureDescriptor {}
impl PNSCopying for MTL4IndirectInstanceAccelerationStructureDescriptor {}
impl INSObject for MTL4IndirectInstanceAccelerationStructureDescriptor {}
impl PNSObject for MTL4IndirectInstanceAccelerationStructureDescriptor {}
impl IMTL4IndirectInstanceAccelerationStructureDescriptor
    for MTL4IndirectInstanceAccelerationStructureDescriptor
{
}
pub trait IMTL4IndirectInstanceAccelerationStructureDescriptor: Sized + std::ops::Deref {
    unsafe fn instanceDescriptorBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorBuffer)
    }
    unsafe fn setInstanceDescriptorBuffer_(&self, instanceDescriptorBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorBuffer : instanceDescriptorBuffer)
    }
    unsafe fn instanceDescriptorStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorStride)
    }
    unsafe fn setInstanceDescriptorStride_(&self, instanceDescriptorStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorStride : instanceDescriptorStride)
    }
    unsafe fn maxInstanceCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxInstanceCount)
    }
    unsafe fn setMaxInstanceCount_(&self, maxInstanceCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxInstanceCount : maxInstanceCount)
    }
    unsafe fn instanceCountBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceCountBuffer)
    }
    unsafe fn setInstanceCountBuffer_(&self, instanceCountBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceCountBuffer : instanceCountBuffer)
    }
    unsafe fn instanceDescriptorType(&self) -> MTLAccelerationStructureInstanceDescriptorType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDescriptorType)
    }
    unsafe fn setInstanceDescriptorType_(
        &self,
        instanceDescriptorType: MTLAccelerationStructureInstanceDescriptorType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDescriptorType : instanceDescriptorType)
    }
    unsafe fn motionTransformBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformBuffer)
    }
    unsafe fn setMotionTransformBuffer_(&self, motionTransformBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformBuffer : motionTransformBuffer)
    }
    unsafe fn maxMotionTransformCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxMotionTransformCount)
    }
    unsafe fn setMaxMotionTransformCount_(&self, maxMotionTransformCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxMotionTransformCount : maxMotionTransformCount)
    }
    unsafe fn motionTransformCountBuffer(&self) -> MTL4BufferRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformCountBuffer)
    }
    unsafe fn setMotionTransformCountBuffer_(&self, motionTransformCountBuffer: MTL4BufferRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformCountBuffer : motionTransformCountBuffer)
    }
    unsafe fn instanceTransformationMatrixLayout(&self) -> MTLMatrixLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceTransformationMatrixLayout)
    }
    unsafe fn setInstanceTransformationMatrixLayout_(
        &self,
        instanceTransformationMatrixLayout: MTLMatrixLayout,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceTransformationMatrixLayout : instanceTransformationMatrixLayout)
    }
    unsafe fn motionTransformType(&self) -> MTLTransformType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformType)
    }
    unsafe fn setMotionTransformType_(&self, motionTransformType: MTLTransformType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformType : motionTransformType)
    }
    unsafe fn motionTransformStride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTransformStride)
    }
    unsafe fn setMotionTransformStride_(&self, motionTransformStride: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTransformStride : motionTransformStride)
    }
}
pub trait PMTL4ComputeCommandEncoder: Sized + std::ops::Deref {
    unsafe fn stages(&self) -> MTLStages
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stages)
    }
    unsafe fn setComputePipelineState_(&self, state: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputePipelineState : state)
    }
    unsafe fn setThreadgroupMemoryLength_atIndex_(&self, length: NSUInteger, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadgroupMemoryLength : length, atIndex : index)
    }
    unsafe fn setImageblockWidth_height_(&self, width: NSUInteger, height: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageblockWidth : width, height : height)
    }
    unsafe fn dispatchThreads_threadsPerThreadgroup_(
        &self,
        threadsPerGrid: MTLSize,
        threadsPerThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchThreads : threadsPerGrid, threadsPerThreadgroup : threadsPerThreadgroup)
    }
    unsafe fn dispatchThreadgroups_threadsPerThreadgroup_(
        &self,
        threadgroupsPerGrid: MTLSize,
        threadsPerThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchThreadgroups : threadgroupsPerGrid, threadsPerThreadgroup : threadsPerThreadgroup)
    }
    unsafe fn dispatchThreadgroupsWithIndirectBuffer_threadsPerThreadgroup_(
        &self,
        indirectBuffer: MTLGPUAddress,
        threadsPerThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchThreadgroupsWithIndirectBuffer : indirectBuffer, threadsPerThreadgroup : threadsPerThreadgroup)
    }
    unsafe fn dispatchThreadsWithIndirectBuffer_(&self, indirectBuffer: MTLGPUAddress)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchThreadsWithIndirectBuffer : indirectBuffer)
    }
    unsafe fn executeCommandsInBuffer_withRange_(
        &self,
        indirectCommandBuffer: *mut u64,
        executionRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeCommandsInBuffer : indirectCommandBuffer, withRange : executionRange)
    }
    unsafe fn executeCommandsInBuffer_indirectBuffer_(
        &self,
        indirectCommandbuffer: *mut u64,
        indirectRangeBuffer: MTLGPUAddress,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeCommandsInBuffer : indirectCommandbuffer, indirectBuffer : indirectRangeBuffer)
    }
    unsafe fn copyFromTexture_toTexture_(
        &self,
        sourceTexture: *mut u64,
        destinationTexture: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, toTexture : destinationTexture)
    }
    unsafe fn copyFromTexture_sourceSlice_sourceLevel_toTexture_destinationSlice_destinationLevel_sliceCount_levelCount_(
        &self,
        sourceTexture: *mut u64,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        destinationTexture: *mut u64,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        sliceCount: NSUInteger,
        levelCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, sourceSlice : sourceSlice, sourceLevel : sourceLevel, toTexture : destinationTexture, destinationSlice : destinationSlice, destinationLevel : destinationLevel, sliceCount : sliceCount, levelCount : levelCount)
    }
    unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_(
        &self,
        sourceTexture: *mut u64,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        sourceOrigin: MTLOrigin,
        sourceSize: MTLSize,
        destinationTexture: *mut u64,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        destinationOrigin: MTLOrigin,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, sourceSlice : sourceSlice, sourceLevel : sourceLevel, sourceOrigin : sourceOrigin, sourceSize : sourceSize, toTexture : destinationTexture, destinationSlice : destinationSlice, destinationLevel : destinationLevel, destinationOrigin : destinationOrigin)
    }
    unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_(
        &self,
        sourceTexture: *mut u64,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        sourceOrigin: MTLOrigin,
        sourceSize: MTLSize,
        destinationBuffer: *mut u64,
        destinationOffset: NSUInteger,
        destinationBytesPerRow: NSUInteger,
        destinationBytesPerImage: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, sourceSlice : sourceSlice, sourceLevel : sourceLevel, sourceOrigin : sourceOrigin, sourceSize : sourceSize, toBuffer : destinationBuffer, destinationOffset : destinationOffset, destinationBytesPerRow : destinationBytesPerRow, destinationBytesPerImage : destinationBytesPerImage)
    }
    unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options_(
        &self,
        sourceTexture: *mut u64,
        sourceSlice: NSUInteger,
        sourceLevel: NSUInteger,
        sourceOrigin: MTLOrigin,
        sourceSize: MTLSize,
        destinationBuffer: *mut u64,
        destinationOffset: NSUInteger,
        destinationBytesPerRow: NSUInteger,
        destinationBytesPerImage: NSUInteger,
        options: MTLBlitOption,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTexture : sourceTexture, sourceSlice : sourceSlice, sourceLevel : sourceLevel, sourceOrigin : sourceOrigin, sourceSize : sourceSize, toBuffer : destinationBuffer, destinationOffset : destinationOffset, destinationBytesPerRow : destinationBytesPerRow, destinationBytesPerImage : destinationBytesPerImage, options : options)
    }
    unsafe fn copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size_(
        &self,
        sourceBuffer: *mut u64,
        sourceOffset: NSUInteger,
        destinationBuffer: *mut u64,
        destinationOffset: NSUInteger,
        size: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromBuffer : sourceBuffer, sourceOffset : sourceOffset, toBuffer : destinationBuffer, destinationOffset : destinationOffset, size : size)
    }
    unsafe fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_(
        &self,
        sourceBuffer: *mut u64,
        sourceOffset: NSUInteger,
        sourceBytesPerRow: NSUInteger,
        sourceBytesPerImage: NSUInteger,
        sourceSize: MTLSize,
        destinationTexture: *mut u64,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        destinationOrigin: MTLOrigin,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromBuffer : sourceBuffer, sourceOffset : sourceOffset, sourceBytesPerRow : sourceBytesPerRow, sourceBytesPerImage : sourceBytesPerImage, sourceSize : sourceSize, toTexture : destinationTexture, destinationSlice : destinationSlice, destinationLevel : destinationLevel, destinationOrigin : destinationOrigin)
    }
    unsafe fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options_(
        &self,
        sourceBuffer: *mut u64,
        sourceOffset: NSUInteger,
        sourceBytesPerRow: NSUInteger,
        sourceBytesPerImage: NSUInteger,
        sourceSize: MTLSize,
        destinationTexture: *mut u64,
        destinationSlice: NSUInteger,
        destinationLevel: NSUInteger,
        destinationOrigin: MTLOrigin,
        options: MTLBlitOption,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromBuffer : sourceBuffer, sourceOffset : sourceOffset, sourceBytesPerRow : sourceBytesPerRow, sourceBytesPerImage : sourceBytesPerImage, sourceSize : sourceSize, toTexture : destinationTexture, destinationSlice : destinationSlice, destinationLevel : destinationLevel, destinationOrigin : destinationOrigin, options : options)
    }
    unsafe fn copyFromTensor_sourceOrigin_sourceDimensions_toTensor_destinationOrigin_destinationDimensions_(
        &self,
        sourceTensor: *mut u64,
        sourceOrigin: MTLTensorExtents,
        sourceDimensions: MTLTensorExtents,
        destinationTensor: *mut u64,
        destinationOrigin: MTLTensorExtents,
        destinationDimensions: MTLTensorExtents,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyFromTensor : sourceTensor, sourceOrigin : sourceOrigin, sourceDimensions : sourceDimensions, toTensor : destinationTensor, destinationOrigin : destinationOrigin, destinationDimensions : destinationDimensions)
    }
    unsafe fn generateMipmapsForTexture_(&self, texture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateMipmapsForTexture : texture)
    }
    unsafe fn fillBuffer_range_value_(&self, buffer: *mut u64, range: NSRange, value: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fillBuffer : buffer, range : range, value : value)
    }
    unsafe fn optimizeContentsForGPUAccess_(&self, texture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeContentsForGPUAccess : texture)
    }
    unsafe fn optimizeContentsForGPUAccess_slice_level_(
        &self,
        texture: *mut u64,
        slice: NSUInteger,
        level: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeContentsForGPUAccess : texture, slice : slice, level : level)
    }
    unsafe fn optimizeContentsForCPUAccess_(&self, texture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeContentsForCPUAccess : texture)
    }
    unsafe fn optimizeContentsForCPUAccess_slice_level_(
        &self,
        texture: *mut u64,
        slice: NSUInteger,
        level: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeContentsForCPUAccess : texture, slice : slice, level : level)
    }
    unsafe fn resetCommandsInBuffer_withRange_(&self, buffer: *mut u64, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetCommandsInBuffer : buffer, withRange : range)
    }
    unsafe fn copyIndirectCommandBuffer_sourceRange_destination_destinationIndex_(
        &self,
        source: *mut u64,
        sourceRange: NSRange,
        destination: *mut u64,
        destinationIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyIndirectCommandBuffer : source, sourceRange : sourceRange, destination : destination, destinationIndex : destinationIndex)
    }
    unsafe fn optimizeIndirectCommandBuffer_withRange_(
        &self,
        indirectCommandBuffer: *mut u64,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optimizeIndirectCommandBuffer : indirectCommandBuffer, withRange : range)
    }
    unsafe fn setArgumentTable_(&self, argumentTable: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArgumentTable : argumentTable)
    }
    unsafe fn buildAccelerationStructure_descriptor_scratchBuffer_(
        &self,
        accelerationStructure: *mut u64,
        descriptor: MTL4AccelerationStructureDescriptor,
        scratchBuffer: MTL4BufferRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, buildAccelerationStructure : accelerationStructure, descriptor : descriptor, scratchBuffer : scratchBuffer)
    }
    unsafe fn refitAccelerationStructure_descriptor_destination_scratchBuffer_(
        &self,
        sourceAccelerationStructure: *mut u64,
        descriptor: MTL4AccelerationStructureDescriptor,
        destinationAccelerationStructure: *mut u64,
        scratchBuffer: MTL4BufferRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, refitAccelerationStructure : sourceAccelerationStructure, descriptor : descriptor, destination : destinationAccelerationStructure, scratchBuffer : scratchBuffer)
    }
    unsafe fn refitAccelerationStructure_descriptor_destination_scratchBuffer_options_(
        &self,
        sourceAccelerationStructure: *mut u64,
        descriptor: MTL4AccelerationStructureDescriptor,
        destinationAccelerationStructure: *mut u64,
        scratchBuffer: MTL4BufferRange,
        options: MTLAccelerationStructureRefitOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, refitAccelerationStructure : sourceAccelerationStructure, descriptor : descriptor, destination : destinationAccelerationStructure, scratchBuffer : scratchBuffer, options : options)
    }
    unsafe fn copyAccelerationStructure_toAccelerationStructure_(
        &self,
        sourceAccelerationStructure: *mut u64,
        destinationAccelerationStructure: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyAccelerationStructure : sourceAccelerationStructure, toAccelerationStructure : destinationAccelerationStructure)
    }
    unsafe fn writeCompactedAccelerationStructureSize_toBuffer_(
        &self,
        accelerationStructure: *mut u64,
        buffer: MTL4BufferRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeCompactedAccelerationStructureSize : accelerationStructure, toBuffer : buffer)
    }
    unsafe fn copyAndCompactAccelerationStructure_toAccelerationStructure_(
        &self,
        sourceAccelerationStructure: *mut u64,
        destinationAccelerationStructure: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyAndCompactAccelerationStructure : sourceAccelerationStructure, toAccelerationStructure : destinationAccelerationStructure)
    }
    unsafe fn writeTimestampWithGranularity_intoHeap_atIndex_(
        &self,
        granularity: MTL4TimestampGranularity,
        counterHeap: *mut u64,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeTimestampWithGranularity : granularity, intoHeap : counterHeap, atIndex : index)
    }
}
pub trait PMTL4MachineLearningCommandEncoder: Sized + std::ops::Deref {
    unsafe fn setPipelineState_(&self, pipelineState: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPipelineState : pipelineState)
    }
    unsafe fn setArgumentTable_(&self, argumentTable: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArgumentTable : argumentTable)
    }
    unsafe fn dispatchNetworkWithIntermediatesHeap_(&self, heap: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dispatchNetworkWithIntermediatesHeap : heap)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4LibraryDescriptor(pub id);
impl std::ops::Deref for MTL4LibraryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4LibraryDescriptor {}
impl MTL4LibraryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4LibraryDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4LibraryDescriptor {}
impl INSObject for MTL4LibraryDescriptor {}
impl PNSObject for MTL4LibraryDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4LibraryDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4LibraryDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4LibraryDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4LibraryDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4LibraryDescriptor")
        }
    }
}
impl IMTL4LibraryDescriptor for MTL4LibraryDescriptor {}
pub trait IMTL4LibraryDescriptor: Sized + std::ops::Deref {
    unsafe fn source(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn setSource_(&self, source: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSource : source)
    }
    unsafe fn options(&self) -> MTLCompileOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: MTLCompileOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4FunctionDescriptor(pub id);
impl std::ops::Deref for MTL4FunctionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4FunctionDescriptor {}
impl MTL4FunctionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4FunctionDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4FunctionDescriptor {}
impl INSObject for MTL4FunctionDescriptor {}
impl PNSObject for MTL4FunctionDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4FunctionDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4FunctionDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4FunctionDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4FunctionDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4FunctionDescriptor")
        }
    }
}
impl IMTL4FunctionDescriptor for MTL4FunctionDescriptor {}
pub trait IMTL4FunctionDescriptor: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4LibraryFunctionDescriptor(pub id);
impl std::ops::Deref for MTL4LibraryFunctionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4LibraryFunctionDescriptor {}
impl MTL4LibraryFunctionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4LibraryFunctionDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4FunctionDescriptor for MTL4LibraryFunctionDescriptor {}
impl PNSCopying for MTL4LibraryFunctionDescriptor {}
impl From<MTL4LibraryFunctionDescriptor> for MTL4FunctionDescriptor {
    fn from(child: MTL4LibraryFunctionDescriptor) -> MTL4FunctionDescriptor {
        MTL4FunctionDescriptor(child.0)
    }
}
impl std::convert::TryFrom<MTL4FunctionDescriptor> for MTL4LibraryFunctionDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MTL4FunctionDescriptor,
    ) -> Result<MTL4LibraryFunctionDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4LibraryFunctionDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4LibraryFunctionDescriptor(parent.0))
        } else {
            Err("This MTL4FunctionDescriptor cannot be downcasted to MTL4LibraryFunctionDescriptor")
        }
    }
}
impl INSObject for MTL4LibraryFunctionDescriptor {}
impl PNSObject for MTL4LibraryFunctionDescriptor {}
impl IMTL4LibraryFunctionDescriptor for MTL4LibraryFunctionDescriptor {}
pub trait IMTL4LibraryFunctionDescriptor: Sized + std::ops::Deref {
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
    unsafe fn library(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, library)
    }
    unsafe fn setLibrary_(&self, library: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLibrary : library)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4SpecializedFunctionDescriptor(pub id);
impl std::ops::Deref for MTL4SpecializedFunctionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4SpecializedFunctionDescriptor {}
impl MTL4SpecializedFunctionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4SpecializedFunctionDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4FunctionDescriptor for MTL4SpecializedFunctionDescriptor {}
impl PNSCopying for MTL4SpecializedFunctionDescriptor {}
impl std::convert::TryFrom<MTL4FunctionDescriptor> for MTL4SpecializedFunctionDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MTL4FunctionDescriptor,
    ) -> Result<MTL4SpecializedFunctionDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4SpecializedFunctionDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4SpecializedFunctionDescriptor(parent.0))
        } else {
            Err ("This MTL4FunctionDescriptor cannot be downcasted to MTL4SpecializedFunctionDescriptor" ,)
        }
    }
}
impl INSObject for MTL4SpecializedFunctionDescriptor {}
impl PNSObject for MTL4SpecializedFunctionDescriptor {}
impl IMTL4SpecializedFunctionDescriptor for MTL4SpecializedFunctionDescriptor {}
pub trait IMTL4SpecializedFunctionDescriptor: Sized + std::ops::Deref {
    unsafe fn functionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionDescriptor)
    }
    unsafe fn setFunctionDescriptor_(&self, functionDescriptor: MTL4FunctionDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionDescriptor : functionDescriptor)
    }
    unsafe fn specializedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specializedName)
    }
    unsafe fn setSpecializedName_(&self, specializedName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecializedName : specializedName)
    }
    unsafe fn constantValues(&self) -> MTLFunctionConstantValues
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constantValues)
    }
    unsafe fn setConstantValues_(&self, constantValues: MTLFunctionConstantValues)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstantValues : constantValues)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4StitchedFunctionDescriptor(pub id);
impl std::ops::Deref for MTL4StitchedFunctionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4StitchedFunctionDescriptor {}
impl MTL4StitchedFunctionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4StitchedFunctionDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4FunctionDescriptor for MTL4StitchedFunctionDescriptor {}
impl PNSCopying for MTL4StitchedFunctionDescriptor {}
impl std::convert::TryFrom<MTL4FunctionDescriptor> for MTL4StitchedFunctionDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MTL4FunctionDescriptor,
    ) -> Result<MTL4StitchedFunctionDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4StitchedFunctionDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4StitchedFunctionDescriptor(parent.0))
        } else {
            Err ("This MTL4FunctionDescriptor cannot be downcasted to MTL4StitchedFunctionDescriptor" ,)
        }
    }
}
impl INSObject for MTL4StitchedFunctionDescriptor {}
impl PNSObject for MTL4StitchedFunctionDescriptor {}
impl IMTL4StitchedFunctionDescriptor for MTL4StitchedFunctionDescriptor {}
pub trait IMTL4StitchedFunctionDescriptor: Sized + std::ops::Deref {
    unsafe fn functionGraph(&self) -> MTLFunctionStitchingGraph
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionGraph)
    }
    unsafe fn setFunctionGraph_(&self, functionGraph: MTLFunctionStitchingGraph)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionGraph : functionGraph)
    }
    unsafe fn functionDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionDescriptors)
    }
    unsafe fn setFunctionDescriptors_(&self, functionDescriptors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionDescriptors : functionDescriptors)
    }
}
pub type MTL4ShaderReflection = NSUInteger;
pub type MTL4AlphaToOneState = NSInteger;
pub type MTL4AlphaToCoverageState = NSInteger;
pub type MTL4BlendState = NSInteger;
pub type MTL4IndirectCommandBufferSupportState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4PipelineOptions(pub id);
impl std::ops::Deref for MTL4PipelineOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4PipelineOptions {}
impl MTL4PipelineOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4PipelineOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4PipelineOptions {}
impl INSObject for MTL4PipelineOptions {}
impl PNSObject for MTL4PipelineOptions {}
impl std::convert::TryFrom<NSObject> for MTL4PipelineOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4PipelineOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4PipelineOptions").unwrap()) };
        if is_kind_of {
            Ok(MTL4PipelineOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4PipelineOptions")
        }
    }
}
impl IMTL4PipelineOptions for MTL4PipelineOptions {}
pub trait IMTL4PipelineOptions: Sized + std::ops::Deref {
    unsafe fn shaderValidation(&self) -> MTLShaderValidation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shaderValidation)
    }
    unsafe fn setShaderValidation_(&self, shaderValidation: MTLShaderValidation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShaderValidation : shaderValidation)
    }
    unsafe fn shaderReflection(&self) -> MTL4ShaderReflection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shaderReflection)
    }
    unsafe fn setShaderReflection_(&self, shaderReflection: MTL4ShaderReflection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShaderReflection : shaderReflection)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4PipelineDescriptor(pub id);
impl std::ops::Deref for MTL4PipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4PipelineDescriptor {}
impl MTL4PipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4PipelineDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4PipelineDescriptor {}
impl INSObject for MTL4PipelineDescriptor {}
impl PNSObject for MTL4PipelineDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4PipelineDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4PipelineDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4PipelineDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4PipelineDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4PipelineDescriptor")
        }
    }
}
impl IMTL4PipelineDescriptor for MTL4PipelineDescriptor {}
pub trait IMTL4PipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn options(&self) -> MTL4PipelineOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: MTL4PipelineOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4ComputePipelineDescriptor(pub id);
impl std::ops::Deref for MTL4ComputePipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4ComputePipelineDescriptor {}
impl MTL4ComputePipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4ComputePipelineDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4PipelineDescriptor for MTL4ComputePipelineDescriptor {}
impl PNSCopying for MTL4ComputePipelineDescriptor {}
impl From<MTL4ComputePipelineDescriptor> for MTL4PipelineDescriptor {
    fn from(child: MTL4ComputePipelineDescriptor) -> MTL4PipelineDescriptor {
        MTL4PipelineDescriptor(child.0)
    }
}
impl std::convert::TryFrom<MTL4PipelineDescriptor> for MTL4ComputePipelineDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MTL4PipelineDescriptor,
    ) -> Result<MTL4ComputePipelineDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4ComputePipelineDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4ComputePipelineDescriptor(parent.0))
        } else {
            Err("This MTL4PipelineDescriptor cannot be downcasted to MTL4ComputePipelineDescriptor")
        }
    }
}
impl INSObject for MTL4ComputePipelineDescriptor {}
impl PNSObject for MTL4ComputePipelineDescriptor {}
impl IMTL4ComputePipelineDescriptor for MTL4ComputePipelineDescriptor {}
pub trait IMTL4ComputePipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn computeFunctionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, computeFunctionDescriptor)
    }
    unsafe fn setComputeFunctionDescriptor_(
        &self,
        computeFunctionDescriptor: MTL4FunctionDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputeFunctionDescriptor : computeFunctionDescriptor)
    }
    unsafe fn threadGroupSizeIsMultipleOfThreadExecutionWidth(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadGroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn setThreadGroupSizeIsMultipleOfThreadExecutionWidth_(
        &self,
        threadGroupSizeIsMultipleOfThreadExecutionWidth: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadGroupSizeIsMultipleOfThreadExecutionWidth : threadGroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerThreadgroup)
    }
    unsafe fn setMaxTotalThreadsPerThreadgroup_(&self, maxTotalThreadsPerThreadgroup: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadsPerThreadgroup : maxTotalThreadsPerThreadgroup)
    }
    unsafe fn requiredThreadsPerThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerThreadgroup)
    }
    unsafe fn setRequiredThreadsPerThreadgroup_(&self, requiredThreadsPerThreadgroup: MTLSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredThreadsPerThreadgroup : requiredThreadsPerThreadgroup)
    }
    unsafe fn supportBinaryLinking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportBinaryLinking)
    }
    unsafe fn setSupportBinaryLinking_(&self, supportBinaryLinking: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportBinaryLinking : supportBinaryLinking)
    }
    unsafe fn staticLinkingDescriptor(&self) -> MTL4StaticLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, staticLinkingDescriptor)
    }
    unsafe fn setStaticLinkingDescriptor_(
        &self,
        staticLinkingDescriptor: MTL4StaticLinkingDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStaticLinkingDescriptor : staticLinkingDescriptor)
    }
    unsafe fn supportIndirectCommandBuffers(&self) -> MTL4IndirectCommandBufferSupportState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportIndirectCommandBuffers)
    }
    unsafe fn setSupportIndirectCommandBuffers_(
        &self,
        supportIndirectCommandBuffers: MTL4IndirectCommandBufferSupportState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportIndirectCommandBuffers : supportIndirectCommandBuffers)
    }
}
pub type MTL4LogicalToPhysicalColorAttachmentMappingState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4RenderPipelineColorAttachmentDescriptor(pub id);
impl std::ops::Deref for MTL4RenderPipelineColorAttachmentDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4RenderPipelineColorAttachmentDescriptor {}
impl MTL4RenderPipelineColorAttachmentDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4RenderPipelineColorAttachmentDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4RenderPipelineColorAttachmentDescriptor {}
impl INSObject for MTL4RenderPipelineColorAttachmentDescriptor {}
impl PNSObject for MTL4RenderPipelineColorAttachmentDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4RenderPipelineColorAttachmentDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTL4RenderPipelineColorAttachmentDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4RenderPipelineColorAttachmentDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4RenderPipelineColorAttachmentDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4RenderPipelineColorAttachmentDescriptor")
        }
    }
}
impl IMTL4RenderPipelineColorAttachmentDescriptor for MTL4RenderPipelineColorAttachmentDescriptor {}
pub trait IMTL4RenderPipelineColorAttachmentDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn pixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn setPixelFormat_(&self, pixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelFormat : pixelFormat)
    }
    unsafe fn blendingState(&self) -> MTL4BlendState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendingState)
    }
    unsafe fn setBlendingState_(&self, blendingState: MTL4BlendState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendingState : blendingState)
    }
    unsafe fn sourceRGBBlendFactor(&self) -> MTLBlendFactor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceRGBBlendFactor)
    }
    unsafe fn setSourceRGBBlendFactor_(&self, sourceRGBBlendFactor: MTLBlendFactor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceRGBBlendFactor : sourceRGBBlendFactor)
    }
    unsafe fn destinationRGBBlendFactor(&self) -> MTLBlendFactor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationRGBBlendFactor)
    }
    unsafe fn setDestinationRGBBlendFactor_(&self, destinationRGBBlendFactor: MTLBlendFactor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationRGBBlendFactor : destinationRGBBlendFactor)
    }
    unsafe fn rgbBlendOperation(&self) -> MTLBlendOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rgbBlendOperation)
    }
    unsafe fn setRgbBlendOperation_(&self, rgbBlendOperation: MTLBlendOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRgbBlendOperation : rgbBlendOperation)
    }
    unsafe fn sourceAlphaBlendFactor(&self) -> MTLBlendFactor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAlphaBlendFactor)
    }
    unsafe fn setSourceAlphaBlendFactor_(&self, sourceAlphaBlendFactor: MTLBlendFactor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceAlphaBlendFactor : sourceAlphaBlendFactor)
    }
    unsafe fn destinationAlphaBlendFactor(&self) -> MTLBlendFactor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationAlphaBlendFactor)
    }
    unsafe fn setDestinationAlphaBlendFactor_(&self, destinationAlphaBlendFactor: MTLBlendFactor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationAlphaBlendFactor : destinationAlphaBlendFactor)
    }
    unsafe fn alphaBlendOperation(&self) -> MTLBlendOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaBlendOperation)
    }
    unsafe fn setAlphaBlendOperation_(&self, alphaBlendOperation: MTLBlendOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaBlendOperation : alphaBlendOperation)
    }
    unsafe fn writeMask(&self) -> MTLColorWriteMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writeMask)
    }
    unsafe fn setWriteMask_(&self, writeMask: MTLColorWriteMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWriteMask : writeMask)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4RenderPipelineColorAttachmentDescriptorArray(pub id);
impl std::ops::Deref for MTL4RenderPipelineColorAttachmentDescriptorArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4RenderPipelineColorAttachmentDescriptorArray {}
impl MTL4RenderPipelineColorAttachmentDescriptorArray {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4RenderPipelineColorAttachmentDescriptorArray").unwrap(), alloc)
        })
    }
}
impl PNSCopying for MTL4RenderPipelineColorAttachmentDescriptorArray {}
impl INSObject for MTL4RenderPipelineColorAttachmentDescriptorArray {}
impl PNSObject for MTL4RenderPipelineColorAttachmentDescriptorArray {}
impl std::convert::TryFrom<NSObject> for MTL4RenderPipelineColorAttachmentDescriptorArray {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTL4RenderPipelineColorAttachmentDescriptorArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4RenderPipelineColorAttachmentDescriptorArray").unwrap())
        };
        if is_kind_of {
            Ok(MTL4RenderPipelineColorAttachmentDescriptorArray(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MTL4RenderPipelineColorAttachmentDescriptorArray" ,)
        }
    }
}
impl IMTL4RenderPipelineColorAttachmentDescriptorArray
    for MTL4RenderPipelineColorAttachmentDescriptorArray
{
}
pub trait IMTL4RenderPipelineColorAttachmentDescriptorArray: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(
        &self,
        attachmentIndex: NSUInteger,
    ) -> MTL4RenderPipelineColorAttachmentDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : attachmentIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        attachment: MTL4RenderPipelineColorAttachmentDescriptor,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : attachment, atIndexedSubscript : attachmentIndex)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4RenderPipelineBinaryFunctionsDescriptor(pub id);
impl std::ops::Deref for MTL4RenderPipelineBinaryFunctionsDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4RenderPipelineBinaryFunctionsDescriptor {}
impl MTL4RenderPipelineBinaryFunctionsDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4RenderPipelineBinaryFunctionsDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4RenderPipelineBinaryFunctionsDescriptor {}
impl INSObject for MTL4RenderPipelineBinaryFunctionsDescriptor {}
impl PNSObject for MTL4RenderPipelineBinaryFunctionsDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4RenderPipelineBinaryFunctionsDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTL4RenderPipelineBinaryFunctionsDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4RenderPipelineBinaryFunctionsDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4RenderPipelineBinaryFunctionsDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4RenderPipelineBinaryFunctionsDescriptor")
        }
    }
}
impl IMTL4RenderPipelineBinaryFunctionsDescriptor for MTL4RenderPipelineBinaryFunctionsDescriptor {}
pub trait IMTL4RenderPipelineBinaryFunctionsDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn vertexAdditionalBinaryFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexAdditionalBinaryFunctions)
    }
    unsafe fn setVertexAdditionalBinaryFunctions_(&self, vertexAdditionalBinaryFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexAdditionalBinaryFunctions : vertexAdditionalBinaryFunctions)
    }
    unsafe fn fragmentAdditionalBinaryFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentAdditionalBinaryFunctions)
    }
    unsafe fn setFragmentAdditionalBinaryFunctions_(
        &self,
        fragmentAdditionalBinaryFunctions: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentAdditionalBinaryFunctions : fragmentAdditionalBinaryFunctions)
    }
    unsafe fn tileAdditionalBinaryFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileAdditionalBinaryFunctions)
    }
    unsafe fn setTileAdditionalBinaryFunctions_(&self, tileAdditionalBinaryFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileAdditionalBinaryFunctions : tileAdditionalBinaryFunctions)
    }
    unsafe fn objectAdditionalBinaryFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectAdditionalBinaryFunctions)
    }
    unsafe fn setObjectAdditionalBinaryFunctions_(&self, objectAdditionalBinaryFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectAdditionalBinaryFunctions : objectAdditionalBinaryFunctions)
    }
    unsafe fn meshAdditionalBinaryFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshAdditionalBinaryFunctions)
    }
    unsafe fn setMeshAdditionalBinaryFunctions_(&self, meshAdditionalBinaryFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshAdditionalBinaryFunctions : meshAdditionalBinaryFunctions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4RenderPipelineDescriptor(pub id);
impl std::ops::Deref for MTL4RenderPipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4RenderPipelineDescriptor {}
impl MTL4RenderPipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4RenderPipelineDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4PipelineDescriptor for MTL4RenderPipelineDescriptor {}
impl PNSCopying for MTL4RenderPipelineDescriptor {}
impl std::convert::TryFrom<MTL4PipelineDescriptor> for MTL4RenderPipelineDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MTL4PipelineDescriptor,
    ) -> Result<MTL4RenderPipelineDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4RenderPipelineDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4RenderPipelineDescriptor(parent.0))
        } else {
            Err("This MTL4PipelineDescriptor cannot be downcasted to MTL4RenderPipelineDescriptor")
        }
    }
}
impl INSObject for MTL4RenderPipelineDescriptor {}
impl PNSObject for MTL4RenderPipelineDescriptor {}
impl IMTL4RenderPipelineDescriptor for MTL4RenderPipelineDescriptor {}
pub trait IMTL4RenderPipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn vertexFunctionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexFunctionDescriptor)
    }
    unsafe fn setVertexFunctionDescriptor_(&self, vertexFunctionDescriptor: MTL4FunctionDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexFunctionDescriptor : vertexFunctionDescriptor)
    }
    unsafe fn fragmentFunctionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentFunctionDescriptor)
    }
    unsafe fn setFragmentFunctionDescriptor_(
        &self,
        fragmentFunctionDescriptor: MTL4FunctionDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentFunctionDescriptor : fragmentFunctionDescriptor)
    }
    unsafe fn vertexDescriptor(&self) -> MTLVertexDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexDescriptor)
    }
    unsafe fn setVertexDescriptor_(&self, vertexDescriptor: MTLVertexDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexDescriptor : vertexDescriptor)
    }
    unsafe fn rasterSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rasterSampleCount)
    }
    unsafe fn setRasterSampleCount_(&self, rasterSampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterSampleCount : rasterSampleCount)
    }
    unsafe fn alphaToCoverageState(&self) -> MTL4AlphaToCoverageState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaToCoverageState)
    }
    unsafe fn setAlphaToCoverageState_(&self, alphaToCoverageState: MTL4AlphaToCoverageState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaToCoverageState : alphaToCoverageState)
    }
    unsafe fn alphaToOneState(&self) -> MTL4AlphaToOneState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaToOneState)
    }
    unsafe fn setAlphaToOneState_(&self, alphaToOneState: MTL4AlphaToOneState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaToOneState : alphaToOneState)
    }
    unsafe fn isRasterizationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRasterizationEnabled)
    }
    unsafe fn setRasterizationEnabled_(&self, rasterizationEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterizationEnabled : rasterizationEnabled)
    }
    unsafe fn maxVertexAmplificationCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxVertexAmplificationCount)
    }
    unsafe fn setMaxVertexAmplificationCount_(&self, maxVertexAmplificationCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxVertexAmplificationCount : maxVertexAmplificationCount)
    }
    unsafe fn colorAttachments(&self) -> MTL4RenderPipelineColorAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachments)
    }
    unsafe fn inputPrimitiveTopology(&self) -> MTLPrimitiveTopologyClass
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputPrimitiveTopology)
    }
    unsafe fn setInputPrimitiveTopology_(&self, inputPrimitiveTopology: MTLPrimitiveTopologyClass)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputPrimitiveTopology : inputPrimitiveTopology)
    }
    unsafe fn vertexStaticLinkingDescriptor(&self) -> MTL4StaticLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexStaticLinkingDescriptor)
    }
    unsafe fn setVertexStaticLinkingDescriptor_(
        &self,
        vertexStaticLinkingDescriptor: MTL4StaticLinkingDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexStaticLinkingDescriptor : vertexStaticLinkingDescriptor)
    }
    unsafe fn fragmentStaticLinkingDescriptor(&self) -> MTL4StaticLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentStaticLinkingDescriptor)
    }
    unsafe fn setFragmentStaticLinkingDescriptor_(
        &self,
        fragmentStaticLinkingDescriptor: MTL4StaticLinkingDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentStaticLinkingDescriptor : fragmentStaticLinkingDescriptor)
    }
    unsafe fn supportVertexBinaryLinking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportVertexBinaryLinking)
    }
    unsafe fn setSupportVertexBinaryLinking_(&self, supportVertexBinaryLinking: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportVertexBinaryLinking : supportVertexBinaryLinking)
    }
    unsafe fn supportFragmentBinaryLinking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportFragmentBinaryLinking)
    }
    unsafe fn setSupportFragmentBinaryLinking_(&self, supportFragmentBinaryLinking: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportFragmentBinaryLinking : supportFragmentBinaryLinking)
    }
    unsafe fn colorAttachmentMappingState(&self) -> MTL4LogicalToPhysicalColorAttachmentMappingState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachmentMappingState)
    }
    unsafe fn setColorAttachmentMappingState_(
        &self,
        colorAttachmentMappingState: MTL4LogicalToPhysicalColorAttachmentMappingState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorAttachmentMappingState : colorAttachmentMappingState)
    }
    unsafe fn supportIndirectCommandBuffers(&self) -> MTL4IndirectCommandBufferSupportState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportIndirectCommandBuffers)
    }
    unsafe fn setSupportIndirectCommandBuffers_(
        &self,
        supportIndirectCommandBuffers: MTL4IndirectCommandBufferSupportState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportIndirectCommandBuffers : supportIndirectCommandBuffers)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4MachineLearningPipelineDescriptor(pub id);
impl std::ops::Deref for MTL4MachineLearningPipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4MachineLearningPipelineDescriptor {}
impl MTL4MachineLearningPipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4MachineLearningPipelineDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4PipelineDescriptor for MTL4MachineLearningPipelineDescriptor {}
impl PNSCopying for MTL4MachineLearningPipelineDescriptor {}
impl std::convert::TryFrom<MTL4PipelineDescriptor> for MTL4MachineLearningPipelineDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MTL4PipelineDescriptor,
    ) -> Result<MTL4MachineLearningPipelineDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4MachineLearningPipelineDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4MachineLearningPipelineDescriptor(parent.0))
        } else {
            Err ("This MTL4PipelineDescriptor cannot be downcasted to MTL4MachineLearningPipelineDescriptor" ,)
        }
    }
}
impl INSObject for MTL4MachineLearningPipelineDescriptor {}
impl PNSObject for MTL4MachineLearningPipelineDescriptor {}
impl IMTL4MachineLearningPipelineDescriptor for MTL4MachineLearningPipelineDescriptor {}
pub trait IMTL4MachineLearningPipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn setInputDimensions_atBufferIndex_(
        &self,
        dimensions: MTLTensorExtents,
        bufferIndex: NSInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputDimensions : dimensions, atBufferIndex : bufferIndex)
    }
    unsafe fn setInputDimensions_withRange_(&self, dimensions: NSArray, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputDimensions : dimensions, withRange : range)
    }
    unsafe fn inputDimensionsAtBufferIndex_(&self, bufferIndex: NSInteger) -> MTLTensorExtents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inputDimensionsAtBufferIndex : bufferIndex)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn machineLearningFunctionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, machineLearningFunctionDescriptor)
    }
    unsafe fn setMachineLearningFunctionDescriptor_(
        &self,
        machineLearningFunctionDescriptor: MTL4FunctionDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMachineLearningFunctionDescriptor : machineLearningFunctionDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4MachineLearningPipelineReflection(pub id);
impl std::ops::Deref for MTL4MachineLearningPipelineReflection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4MachineLearningPipelineReflection {}
impl MTL4MachineLearningPipelineReflection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4MachineLearningPipelineReflection").unwrap(), alloc) })
    }
}
impl INSObject for MTL4MachineLearningPipelineReflection {}
impl PNSObject for MTL4MachineLearningPipelineReflection {}
impl std::convert::TryFrom<NSObject> for MTL4MachineLearningPipelineReflection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4MachineLearningPipelineReflection, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4MachineLearningPipelineReflection").unwrap())
        };
        if is_kind_of {
            Ok(MTL4MachineLearningPipelineReflection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4MachineLearningPipelineReflection")
        }
    }
}
impl IMTL4MachineLearningPipelineReflection for MTL4MachineLearningPipelineReflection {}
pub trait IMTL4MachineLearningPipelineReflection: Sized + std::ops::Deref {
    unsafe fn bindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bindings)
    }
}
pub trait PMTL4MachineLearningPipelineState: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn reflection(&self) -> MTL4MachineLearningPipelineReflection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflection)
    }
    unsafe fn intermediatesHeapSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intermediatesHeapSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4TileRenderPipelineDescriptor(pub id);
impl std::ops::Deref for MTL4TileRenderPipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4TileRenderPipelineDescriptor {}
impl MTL4TileRenderPipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4TileRenderPipelineDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4PipelineDescriptor for MTL4TileRenderPipelineDescriptor {}
impl PNSCopying for MTL4TileRenderPipelineDescriptor {}
impl std::convert::TryFrom<MTL4PipelineDescriptor> for MTL4TileRenderPipelineDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MTL4PipelineDescriptor,
    ) -> Result<MTL4TileRenderPipelineDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4TileRenderPipelineDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4TileRenderPipelineDescriptor(parent.0))
        } else {
            Err ("This MTL4PipelineDescriptor cannot be downcasted to MTL4TileRenderPipelineDescriptor" ,)
        }
    }
}
impl INSObject for MTL4TileRenderPipelineDescriptor {}
impl PNSObject for MTL4TileRenderPipelineDescriptor {}
impl IMTL4TileRenderPipelineDescriptor for MTL4TileRenderPipelineDescriptor {}
pub trait IMTL4TileRenderPipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn tileFunctionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileFunctionDescriptor)
    }
    unsafe fn setTileFunctionDescriptor_(&self, tileFunctionDescriptor: MTL4FunctionDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileFunctionDescriptor : tileFunctionDescriptor)
    }
    unsafe fn rasterSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rasterSampleCount)
    }
    unsafe fn setRasterSampleCount_(&self, rasterSampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterSampleCount : rasterSampleCount)
    }
    unsafe fn colorAttachments(&self) -> MTLTileRenderPipelineColorAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachments)
    }
    unsafe fn threadgroupSizeMatchesTileSize(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadgroupSizeMatchesTileSize)
    }
    unsafe fn setThreadgroupSizeMatchesTileSize_(&self, threadgroupSizeMatchesTileSize: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadgroupSizeMatchesTileSize : threadgroupSizeMatchesTileSize)
    }
    unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerThreadgroup)
    }
    unsafe fn setMaxTotalThreadsPerThreadgroup_(&self, maxTotalThreadsPerThreadgroup: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadsPerThreadgroup : maxTotalThreadsPerThreadgroup)
    }
    unsafe fn requiredThreadsPerThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerThreadgroup)
    }
    unsafe fn setRequiredThreadsPerThreadgroup_(&self, requiredThreadsPerThreadgroup: MTLSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredThreadsPerThreadgroup : requiredThreadsPerThreadgroup)
    }
    unsafe fn staticLinkingDescriptor(&self) -> MTL4StaticLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, staticLinkingDescriptor)
    }
    unsafe fn setStaticLinkingDescriptor_(
        &self,
        staticLinkingDescriptor: MTL4StaticLinkingDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStaticLinkingDescriptor : staticLinkingDescriptor)
    }
    unsafe fn supportBinaryLinking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportBinaryLinking)
    }
    unsafe fn setSupportBinaryLinking_(&self, supportBinaryLinking: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportBinaryLinking : supportBinaryLinking)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4MeshRenderPipelineDescriptor(pub id);
impl std::ops::Deref for MTL4MeshRenderPipelineDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4MeshRenderPipelineDescriptor {}
impl MTL4MeshRenderPipelineDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4MeshRenderPipelineDescriptor").unwrap(), alloc) })
    }
}
impl IMTL4PipelineDescriptor for MTL4MeshRenderPipelineDescriptor {}
impl PNSCopying for MTL4MeshRenderPipelineDescriptor {}
impl std::convert::TryFrom<MTL4PipelineDescriptor> for MTL4MeshRenderPipelineDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MTL4PipelineDescriptor,
    ) -> Result<MTL4MeshRenderPipelineDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4MeshRenderPipelineDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4MeshRenderPipelineDescriptor(parent.0))
        } else {
            Err ("This MTL4PipelineDescriptor cannot be downcasted to MTL4MeshRenderPipelineDescriptor" ,)
        }
    }
}
impl INSObject for MTL4MeshRenderPipelineDescriptor {}
impl PNSObject for MTL4MeshRenderPipelineDescriptor {}
impl IMTL4MeshRenderPipelineDescriptor for MTL4MeshRenderPipelineDescriptor {}
pub trait IMTL4MeshRenderPipelineDescriptor: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn objectFunctionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectFunctionDescriptor)
    }
    unsafe fn setObjectFunctionDescriptor_(&self, objectFunctionDescriptor: MTL4FunctionDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectFunctionDescriptor : objectFunctionDescriptor)
    }
    unsafe fn meshFunctionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshFunctionDescriptor)
    }
    unsafe fn setMeshFunctionDescriptor_(&self, meshFunctionDescriptor: MTL4FunctionDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshFunctionDescriptor : meshFunctionDescriptor)
    }
    unsafe fn fragmentFunctionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentFunctionDescriptor)
    }
    unsafe fn setFragmentFunctionDescriptor_(
        &self,
        fragmentFunctionDescriptor: MTL4FunctionDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentFunctionDescriptor : fragmentFunctionDescriptor)
    }
    unsafe fn maxTotalThreadsPerObjectThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerObjectThreadgroup)
    }
    unsafe fn setMaxTotalThreadsPerObjectThreadgroup_(
        &self,
        maxTotalThreadsPerObjectThreadgroup: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadsPerObjectThreadgroup : maxTotalThreadsPerObjectThreadgroup)
    }
    unsafe fn maxTotalThreadsPerMeshThreadgroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadsPerMeshThreadgroup)
    }
    unsafe fn setMaxTotalThreadsPerMeshThreadgroup_(
        &self,
        maxTotalThreadsPerMeshThreadgroup: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadsPerMeshThreadgroup : maxTotalThreadsPerMeshThreadgroup)
    }
    unsafe fn requiredThreadsPerObjectThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerObjectThreadgroup)
    }
    unsafe fn setRequiredThreadsPerObjectThreadgroup_(
        &self,
        requiredThreadsPerObjectThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredThreadsPerObjectThreadgroup : requiredThreadsPerObjectThreadgroup)
    }
    unsafe fn requiredThreadsPerMeshThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredThreadsPerMeshThreadgroup)
    }
    unsafe fn setRequiredThreadsPerMeshThreadgroup_(
        &self,
        requiredThreadsPerMeshThreadgroup: MTLSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredThreadsPerMeshThreadgroup : requiredThreadsPerMeshThreadgroup)
    }
    unsafe fn objectThreadgroupSizeIsMultipleOfThreadExecutionWidth(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectThreadgroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth_(
        &self,
        objectThreadgroupSizeIsMultipleOfThreadExecutionWidth: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth : objectThreadgroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn meshThreadgroupSizeIsMultipleOfThreadExecutionWidth(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshThreadgroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth_(
        &self,
        meshThreadgroupSizeIsMultipleOfThreadExecutionWidth: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth : meshThreadgroupSizeIsMultipleOfThreadExecutionWidth)
    }
    unsafe fn payloadMemoryLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, payloadMemoryLength)
    }
    unsafe fn setPayloadMemoryLength_(&self, payloadMemoryLength: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPayloadMemoryLength : payloadMemoryLength)
    }
    unsafe fn maxTotalThreadgroupsPerMeshGrid(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTotalThreadgroupsPerMeshGrid)
    }
    unsafe fn setMaxTotalThreadgroupsPerMeshGrid_(
        &self,
        maxTotalThreadgroupsPerMeshGrid: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxTotalThreadgroupsPerMeshGrid : maxTotalThreadgroupsPerMeshGrid)
    }
    unsafe fn rasterSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rasterSampleCount)
    }
    unsafe fn setRasterSampleCount_(&self, rasterSampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterSampleCount : rasterSampleCount)
    }
    unsafe fn alphaToCoverageState(&self) -> MTL4AlphaToCoverageState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaToCoverageState)
    }
    unsafe fn setAlphaToCoverageState_(&self, alphaToCoverageState: MTL4AlphaToCoverageState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaToCoverageState : alphaToCoverageState)
    }
    unsafe fn alphaToOneState(&self) -> MTL4AlphaToOneState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaToOneState)
    }
    unsafe fn setAlphaToOneState_(&self, alphaToOneState: MTL4AlphaToOneState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaToOneState : alphaToOneState)
    }
    unsafe fn isRasterizationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRasterizationEnabled)
    }
    unsafe fn setRasterizationEnabled_(&self, rasterizationEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterizationEnabled : rasterizationEnabled)
    }
    unsafe fn maxVertexAmplificationCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxVertexAmplificationCount)
    }
    unsafe fn setMaxVertexAmplificationCount_(&self, maxVertexAmplificationCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxVertexAmplificationCount : maxVertexAmplificationCount)
    }
    unsafe fn colorAttachments(&self) -> MTL4RenderPipelineColorAttachmentDescriptorArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachments)
    }
    unsafe fn objectStaticLinkingDescriptor(&self) -> MTL4StaticLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectStaticLinkingDescriptor)
    }
    unsafe fn setObjectStaticLinkingDescriptor_(
        &self,
        objectStaticLinkingDescriptor: MTL4StaticLinkingDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectStaticLinkingDescriptor : objectStaticLinkingDescriptor)
    }
    unsafe fn meshStaticLinkingDescriptor(&self) -> MTL4StaticLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshStaticLinkingDescriptor)
    }
    unsafe fn setMeshStaticLinkingDescriptor_(
        &self,
        meshStaticLinkingDescriptor: MTL4StaticLinkingDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeshStaticLinkingDescriptor : meshStaticLinkingDescriptor)
    }
    unsafe fn fragmentStaticLinkingDescriptor(&self) -> MTL4StaticLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentStaticLinkingDescriptor)
    }
    unsafe fn setFragmentStaticLinkingDescriptor_(
        &self,
        fragmentStaticLinkingDescriptor: MTL4StaticLinkingDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFragmentStaticLinkingDescriptor : fragmentStaticLinkingDescriptor)
    }
    unsafe fn supportObjectBinaryLinking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportObjectBinaryLinking)
    }
    unsafe fn setSupportObjectBinaryLinking_(&self, supportObjectBinaryLinking: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportObjectBinaryLinking : supportObjectBinaryLinking)
    }
    unsafe fn supportMeshBinaryLinking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportMeshBinaryLinking)
    }
    unsafe fn setSupportMeshBinaryLinking_(&self, supportMeshBinaryLinking: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportMeshBinaryLinking : supportMeshBinaryLinking)
    }
    unsafe fn supportFragmentBinaryLinking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportFragmentBinaryLinking)
    }
    unsafe fn setSupportFragmentBinaryLinking_(&self, supportFragmentBinaryLinking: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportFragmentBinaryLinking : supportFragmentBinaryLinking)
    }
    unsafe fn colorAttachmentMappingState(&self) -> MTL4LogicalToPhysicalColorAttachmentMappingState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorAttachmentMappingState)
    }
    unsafe fn setColorAttachmentMappingState_(
        &self,
        colorAttachmentMappingState: MTL4LogicalToPhysicalColorAttachmentMappingState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorAttachmentMappingState : colorAttachmentMappingState)
    }
    unsafe fn supportIndirectCommandBuffers(&self) -> MTL4IndirectCommandBufferSupportState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportIndirectCommandBuffers)
    }
    unsafe fn setSupportIndirectCommandBuffers_(
        &self,
        supportIndirectCommandBuffers: MTL4IndirectCommandBufferSupportState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportIndirectCommandBuffers : supportIndirectCommandBuffers)
    }
}
pub type MTL4PipelineDataSetSerializerConfiguration = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4PipelineDataSetSerializerDescriptor(pub id);
impl std::ops::Deref for MTL4PipelineDataSetSerializerDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4PipelineDataSetSerializerDescriptor {}
impl MTL4PipelineDataSetSerializerDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4PipelineDataSetSerializerDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4PipelineDataSetSerializerDescriptor {}
impl INSObject for MTL4PipelineDataSetSerializerDescriptor {}
impl PNSObject for MTL4PipelineDataSetSerializerDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4PipelineDataSetSerializerDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4PipelineDataSetSerializerDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4PipelineDataSetSerializerDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4PipelineDataSetSerializerDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4PipelineDataSetSerializerDescriptor")
        }
    }
}
impl IMTL4PipelineDataSetSerializerDescriptor for MTL4PipelineDataSetSerializerDescriptor {}
pub trait IMTL4PipelineDataSetSerializerDescriptor: Sized + std::ops::Deref {
    unsafe fn configuration(&self) -> MTL4PipelineDataSetSerializerConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn setConfiguration_(&self, configuration: MTL4PipelineDataSetSerializerConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfiguration : configuration)
    }
}
pub trait PMTL4PipelineDataSetSerializer: Sized + std::ops::Deref {
    unsafe fn serializeAsArchiveAndFlushToURL_error_(&self, url: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, serializeAsArchiveAndFlushToURL : url, error : error)
    }
    unsafe fn serializeAsPipelinesScriptWithError_(&self, error: *mut NSError) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, serializeAsPipelinesScriptWithError : error)
    }
}
pub trait PMTL4Archive: Sized + std::ops::Deref {
    unsafe fn newComputePipelineStateWithDescriptor_error_(
        &self,
        descriptor: MTL4ComputePipelineDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithDescriptor : descriptor, error : error)
    }
    unsafe fn newComputePipelineStateWithDescriptor_dynamicLinkingDescriptor_error_(
        &self,
        descriptor: MTL4ComputePipelineDescriptor,
        dynamicLinkingDescriptor: MTL4PipelineStageDynamicLinkingDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithDescriptor : descriptor, dynamicLinkingDescriptor : dynamicLinkingDescriptor, error : error)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_error_(
        &self,
        descriptor: MTL4PipelineDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, error : error)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_dynamicLinkingDescriptor_error_(
        &self,
        descriptor: MTL4PipelineDescriptor,
        dynamicLinkingDescriptor: MTL4RenderPipelineDynamicLinkingDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, dynamicLinkingDescriptor : dynamicLinkingDescriptor, error : error)
    }
    unsafe fn newBinaryFunctionWithDescriptor_error_(
        &self,
        descriptor: MTL4BinaryFunctionDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBinaryFunctionWithDescriptor : descriptor, error : error)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
}
pub type MTL4BinaryFunctionOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4BinaryFunctionDescriptor(pub id);
impl std::ops::Deref for MTL4BinaryFunctionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4BinaryFunctionDescriptor {}
impl MTL4BinaryFunctionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4BinaryFunctionDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4BinaryFunctionDescriptor {}
impl INSObject for MTL4BinaryFunctionDescriptor {}
impl PNSObject for MTL4BinaryFunctionDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4BinaryFunctionDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4BinaryFunctionDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4BinaryFunctionDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4BinaryFunctionDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4BinaryFunctionDescriptor")
        }
    }
}
impl IMTL4BinaryFunctionDescriptor for MTL4BinaryFunctionDescriptor {}
pub trait IMTL4BinaryFunctionDescriptor: Sized + std::ops::Deref {
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
    unsafe fn functionDescriptor(&self) -> MTL4FunctionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionDescriptor)
    }
    unsafe fn setFunctionDescriptor_(&self, functionDescriptor: MTL4FunctionDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionDescriptor : functionDescriptor)
    }
    unsafe fn options(&self) -> MTL4BinaryFunctionOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: MTL4BinaryFunctionOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4BinaryFunction(pub id);
impl std::ops::Deref for MTL4BinaryFunction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4BinaryFunction {}
impl MTL4BinaryFunction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4BinaryFunction").unwrap(), alloc) })
    }
}
impl IMTL4BinaryFunction for MTL4BinaryFunction {}
pub trait IMTL4BinaryFunction: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLDynamicLibrary(pub id);
impl std::ops::Deref for MTLDynamicLibrary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLDynamicLibrary {}
impl MTLDynamicLibrary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLDynamicLibrary").unwrap(), alloc) })
    }
}
impl IMTLDynamicLibrary for MTLDynamicLibrary {}
pub trait IMTLDynamicLibrary: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4StaticLinkingDescriptor(pub id);
impl std::ops::Deref for MTL4StaticLinkingDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4StaticLinkingDescriptor {}
impl MTL4StaticLinkingDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4StaticLinkingDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4StaticLinkingDescriptor {}
impl INSObject for MTL4StaticLinkingDescriptor {}
impl PNSObject for MTL4StaticLinkingDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4StaticLinkingDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTL4StaticLinkingDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4StaticLinkingDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTL4StaticLinkingDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4StaticLinkingDescriptor")
        }
    }
}
impl IMTL4StaticLinkingDescriptor for MTL4StaticLinkingDescriptor {}
pub trait IMTL4StaticLinkingDescriptor: Sized + std::ops::Deref {
    unsafe fn functionDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionDescriptors)
    }
    unsafe fn setFunctionDescriptors_(&self, functionDescriptors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionDescriptors : functionDescriptors)
    }
    unsafe fn privateFunctionDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, privateFunctionDescriptors)
    }
    unsafe fn setPrivateFunctionDescriptors_(&self, privateFunctionDescriptors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrivateFunctionDescriptors : privateFunctionDescriptors)
    }
    unsafe fn groups(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groups)
    }
    unsafe fn setGroups_(&self, groups: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroups : groups)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4PipelineStageDynamicLinkingDescriptor(pub id);
impl std::ops::Deref for MTL4PipelineStageDynamicLinkingDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4PipelineStageDynamicLinkingDescriptor {}
impl MTL4PipelineStageDynamicLinkingDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4PipelineStageDynamicLinkingDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4PipelineStageDynamicLinkingDescriptor {}
impl INSObject for MTL4PipelineStageDynamicLinkingDescriptor {}
impl PNSObject for MTL4PipelineStageDynamicLinkingDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4PipelineStageDynamicLinkingDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTL4PipelineStageDynamicLinkingDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4PipelineStageDynamicLinkingDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4PipelineStageDynamicLinkingDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4PipelineStageDynamicLinkingDescriptor")
        }
    }
}
impl IMTL4PipelineStageDynamicLinkingDescriptor for MTL4PipelineStageDynamicLinkingDescriptor {}
pub trait IMTL4PipelineStageDynamicLinkingDescriptor: Sized + std::ops::Deref {
    unsafe fn maxCallStackDepth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxCallStackDepth)
    }
    unsafe fn setMaxCallStackDepth_(&self, maxCallStackDepth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxCallStackDepth : maxCallStackDepth)
    }
    unsafe fn binaryLinkedFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, binaryLinkedFunctions)
    }
    unsafe fn setBinaryLinkedFunctions_(&self, binaryLinkedFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBinaryLinkedFunctions : binaryLinkedFunctions)
    }
    unsafe fn preloadedLibraries(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preloadedLibraries)
    }
    unsafe fn setPreloadedLibraries_(&self, preloadedLibraries: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreloadedLibraries : preloadedLibraries)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTL4RenderPipelineDynamicLinkingDescriptor(pub id);
impl std::ops::Deref for MTL4RenderPipelineDynamicLinkingDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTL4RenderPipelineDynamicLinkingDescriptor {}
impl MTL4RenderPipelineDynamicLinkingDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTL4RenderPipelineDynamicLinkingDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTL4RenderPipelineDynamicLinkingDescriptor {}
impl INSObject for MTL4RenderPipelineDynamicLinkingDescriptor {}
impl PNSObject for MTL4RenderPipelineDynamicLinkingDescriptor {}
impl std::convert::TryFrom<NSObject> for MTL4RenderPipelineDynamicLinkingDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MTL4RenderPipelineDynamicLinkingDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTL4RenderPipelineDynamicLinkingDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTL4RenderPipelineDynamicLinkingDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTL4RenderPipelineDynamicLinkingDescriptor")
        }
    }
}
impl IMTL4RenderPipelineDynamicLinkingDescriptor for MTL4RenderPipelineDynamicLinkingDescriptor {}
pub trait IMTL4RenderPipelineDynamicLinkingDescriptor: Sized + std::ops::Deref {
    unsafe fn vertexLinkingDescriptor(&self) -> MTL4PipelineStageDynamicLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexLinkingDescriptor)
    }
    unsafe fn fragmentLinkingDescriptor(&self) -> MTL4PipelineStageDynamicLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fragmentLinkingDescriptor)
    }
    unsafe fn tileLinkingDescriptor(&self) -> MTL4PipelineStageDynamicLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileLinkingDescriptor)
    }
    unsafe fn objectLinkingDescriptor(&self) -> MTL4PipelineStageDynamicLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectLinkingDescriptor)
    }
    unsafe fn meshLinkingDescriptor(&self) -> MTL4PipelineStageDynamicLinkingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, meshLinkingDescriptor)
    }
}
unsafe extern "C" {
    pub static MTLTensorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MTLLibraryErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterTimestamp: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterTessellationInputPatches: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterVertexInvocations: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterPostTessellationVertexInvocations: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterClipperInvocations: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterClipperPrimitivesOut: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterFragmentInvocations: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterFragmentsPassed: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterComputeKernelInvocations: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterTotalCycles: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterVertexCycles: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterTessellationCycles: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterPostTessellationVertexCycles: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterFragmentCycles: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterRenderTargetWriteCycles: MTLCommonCounter;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterSetTimestamp: MTLCommonCounterSet;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterSetStageUtilization: MTLCommonCounterSet;
}
unsafe extern "C" {
    pub static mut MTLCommonCounterSetStatistic: MTLCommonCounterSet;
}
unsafe extern "C" {
    pub static MTLCounterErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub fn MTLCreateSystemDefaultDevice() -> *mut u64;
}
unsafe extern "C" {
    pub fn MTLCopyAllDevices() -> NSArray;
}
unsafe extern "C" {
    pub static MTLCommandBufferErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MTLCommandBufferEncoderInfoErrorKey: NSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static NSDeviceCertificationiPhonePerformanceGaming: NSDeviceCertification;
}
unsafe extern "C" {
    pub static NSProcessPerformanceProfileDefault: NSProcessPerformanceProfile;
}
unsafe extern "C" {
    pub static NSProcessPerformanceProfileSustained: NSProcessPerformanceProfile;
}
unsafe extern "C" {
    pub static NSProcessInfoPerformanceProfileDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MTL4CommandQueueErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MTLCaptureErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MTLDynamicLibraryDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MTLLogStateErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MTLBinaryArchiveDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MTLIOErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub fn MTLIOCompressionContextDefaultChunkSize() -> usize;
}
unsafe extern "C" {
    pub fn MTLIOCreateCompressionContext(
        path: *const ::std::os::raw::c_char,
        type_: MTLIOCompressionMethod,
        chunkSize: usize,
    ) -> MTLIOCompressionContext;
}
unsafe extern "C" {
    pub fn MTLIOCompressionContextAppendData(
        context: MTLIOCompressionContext,
        data: *const ::std::os::raw::c_void,
        size: usize,
    );
}
unsafe extern "C" {
    pub fn MTLIOFlushAndDestroyCompressionContext(
        context: MTLIOCompressionContext,
    ) -> MTLIOCompressionStatus;
}

unsafe impl objc2::encode::RefEncode for MTLOrigin {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLOrigin {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLOrigin", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLSize {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLSize {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLSize", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLRegion", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLSamplePosition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLSamplePosition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLSamplePosition", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLResourceID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLResourceID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLResourceID", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLTensorExtents {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTensorExtents {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLTensorDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTensorDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLTextureSwizzleChannels {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTextureSwizzleChannels {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLTextureSwizzleChannels", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLSharedTextureHandle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLSharedTextureHandle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLTextureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTextureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLTextureViewDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTextureViewDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLStructMember {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLStructMember {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLStructType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLStructType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLArrayType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLArrayType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLPointerType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLPointerType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLTextureReferenceType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTextureReferenceType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLTensorReferenceType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTensorReferenceType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLArgument {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLArgument {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFunctionConstantValues {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFunctionConstantValues {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFunctionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFunctionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLIntersectionFunctionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLIntersectionFunctionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLVertexAttribute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLVertexAttribute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAttribute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAttribute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFunctionConstant {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFunctionConstant {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLCompileOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLCompileOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFunctionReflection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFunctionReflection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLCounterResultTimestamp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLCounterResultTimestamp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLCounterResultTimestamp", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLCounterResultStageUtilization {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLCounterResultStageUtilization {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLCounterResultStageUtilization", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLCounterResultStatistic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLCounterResultStatistic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLCounterResultStatistic", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLCounterSampleBufferDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLCounterSampleBufferDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4CompilerDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4CompilerDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4CompilerTaskOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4CompilerTaskOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4TimestampHeapEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4TimestampHeapEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTL4TimestampHeapEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for MTL4CounterHeapDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4CounterHeapDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureSizes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureSizes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLAccelerationStructureSizes", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLSizeAndAlign {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLSizeAndAlign {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLSizeAndAlign", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLArgumentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLArgumentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLArchitecture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLArchitecture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLResourceStatePassSampleBufferAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLResourceStatePassSampleBufferAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLResourceStatePassDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLResourceStatePassDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLMapIndirectArguments {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLMapIndirectArguments {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLMapIndirectArguments", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLClearColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLClearColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLClearColor", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLRenderPassAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPassAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPassColorAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPassColorAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPassDepthAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPassDepthAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPassStencilAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPassStencilAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPassColorAttachmentDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPassColorAttachmentDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPassSampleBufferAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPassSampleBufferAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPassSampleBufferAttachmentDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPassSampleBufferAttachmentDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPassDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPassDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLBlitPassSampleBufferAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLBlitPassSampleBufferAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLBlitPassSampleBufferAttachmentDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLBlitPassSampleBufferAttachmentDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLBlitPassDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLBlitPassDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLCommandBufferDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLCommandBufferDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLComputePassSampleBufferAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLComputePassSampleBufferAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLComputePassSampleBufferAttachmentDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLComputePassSampleBufferAttachmentDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLComputePassDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLComputePassDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLDispatchThreadgroupsIndirectArguments {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLDispatchThreadgroupsIndirectArguments {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLDispatchThreadgroupsIndirectArguments", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLDispatchThreadsIndirectArguments {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLDispatchThreadsIndirectArguments {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLDispatchThreadsIndirectArguments", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLStageInRegionIndirectArguments {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLStageInRegionIndirectArguments {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLStageInRegionIndirectArguments", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLCommandQueueDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLCommandQueueDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLStencilDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLStencilDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLDepthStencilDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLDepthStencilDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLVertexBufferLayoutDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLVertexBufferLayoutDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLVertexBufferLayoutDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLVertexBufferLayoutDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLVertexAttributeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLVertexAttributeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLVertexAttributeDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLVertexAttributeDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLVertexDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLVertexDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLBufferLayoutDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLBufferLayoutDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLBufferLayoutDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLBufferLayoutDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAttributeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAttributeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAttributeDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAttributeDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLStageInputOutputDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLStageInputOutputDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLPipelineBufferDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLPipelineBufferDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLPipelineBufferDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLPipelineBufferDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLLinkedFunctions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLLinkedFunctions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLComputePipelineReflection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLComputePipelineReflection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLComputePipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLComputePipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLScissorRect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLScissorRect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLScissorRect", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLViewport {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLViewport {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLViewport", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLDrawPrimitivesIndirectArguments {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLDrawPrimitivesIndirectArguments {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLDrawPrimitivesIndirectArguments", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLDrawIndexedPrimitivesIndirectArguments {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLDrawIndexedPrimitivesIndirectArguments {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLDrawIndexedPrimitivesIndirectArguments", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLVertexAmplificationViewMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLVertexAmplificationViewMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLVertexAmplificationViewMapping", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLDrawPatchIndirectArguments {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLDrawPatchIndirectArguments {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLDrawPatchIndirectArguments", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLQuadTessellationFactorsHalf {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLQuadTessellationFactorsHalf {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLQuadTessellationFactorsHalf", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLTriangleTessellationFactorsHalf {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTriangleTessellationFactorsHalf {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLTriangleTessellationFactorsHalf", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLVisibleFunctionTableDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLVisibleFunctionTableDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPipelineColorAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPipelineColorAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLLogicalToPhysicalColorAttachmentMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLLogicalToPhysicalColorAttachmentMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPipelineReflection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPipelineReflection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPipelineFunctionsDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPipelineFunctionsDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRenderPipelineColorAttachmentDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRenderPipelineColorAttachmentDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLTileRenderPipelineColorAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTileRenderPipelineColorAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLTileRenderPipelineColorAttachmentDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTileRenderPipelineColorAttachmentDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLTileRenderPipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLTileRenderPipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLMeshRenderPipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLMeshRenderPipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLSamplerDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLSamplerDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4BufferRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4BufferRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTL4BufferRange", &[]);
}
unsafe impl objc2::encode::RefEncode for _MTLPackedFloat3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _MTLPackedFloat3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_MTLPackedFloat3", &[]);
}
unsafe impl objc2::encode::RefEncode for _MTLPackedFloat3__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _MTLPackedFloat3__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_MTLPackedFloat3__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for _MTLPackedFloat3__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _MTLPackedFloat3__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_MTLPackedFloat3__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLPackedFloatQuaternion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLPackedFloatQuaternion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLPackedFloatQuaternion", &[]);
}
unsafe impl objc2::encode::RefEncode for _MTLPackedFloat4x3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _MTLPackedFloat4x3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_MTLPackedFloat4x3", &[]);
}
unsafe impl objc2::encode::RefEncode for _MTLAxisAlignedBoundingBox {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _MTLAxisAlignedBoundingBox {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_MTLAxisAlignedBoundingBox", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLComponentTransform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLComponentTransform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLComponentTransform", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLPrimitiveAccelerationStructureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLPrimitiveAccelerationStructureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureTriangleGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureTriangleGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureBoundingBoxGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureBoundingBoxGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLMotionKeyframeData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLMotionKeyframeData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureMotionTriangleGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureMotionTriangleGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureCurveGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureCurveGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureMotionCurveGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureMotionCurveGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureInstanceDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureInstanceDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLAccelerationStructureInstanceDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureUserIDInstanceDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureUserIDInstanceDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLAccelerationStructureUserIDInstanceDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructureMotionInstanceDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructureMotionInstanceDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLAccelerationStructureMotionInstanceDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLIndirectAccelerationStructureInstanceDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLIndirectAccelerationStructureInstanceDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLIndirectAccelerationStructureInstanceDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLIndirectAccelerationStructureMotionInstanceDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLIndirectAccelerationStructureMotionInstanceDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLIndirectAccelerationStructureMotionInstanceDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLInstanceAccelerationStructureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLInstanceAccelerationStructureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLIndirectInstanceAccelerationStructureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLIndirectInstanceAccelerationStructureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLHeapDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLHeapDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4CommandBufferOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4CommandBufferOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLSharedEventListener {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLSharedEventListener {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLSharedEventHandle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLSharedEventHandle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4CommitOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4CommitOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4CommandQueueDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4CommandQueueDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4UpdateSparseTextureMappingOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4UpdateSparseTextureMappingOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTL4UpdateSparseTextureMappingOperation", &[]);
}
unsafe impl objc2::encode::RefEncode for MTL4CopySparseTextureMappingOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4CopySparseTextureMappingOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTL4CopySparseTextureMappingOperation", &[]);
}
unsafe impl objc2::encode::RefEncode for MTL4UpdateSparseBufferMappingOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4UpdateSparseBufferMappingOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTL4UpdateSparseBufferMappingOperation", &[]);
}
unsafe impl objc2::encode::RefEncode for MTL4CopySparseBufferMappingOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4CopySparseBufferMappingOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTL4CopySparseBufferMappingOperation", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLCaptureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLCaptureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLCaptureManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLCaptureManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLIndirectCommandBufferExecutionRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLIndirectCommandBufferExecutionRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLIndirectCommandBufferExecutionRange", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLIndirectCommandBufferDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLIndirectCommandBufferDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLAccelerationStructurePassDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLAccelerationStructurePassDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRasterizationRateSampleArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRasterizationRateSampleArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRasterizationRateLayerDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRasterizationRateLayerDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRasterizationRateLayerArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRasterizationRateLayerArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLRasterizationRateMapDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLRasterizationRateMapDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLLogStateDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLLogStateDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLBinaryArchiveDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLBinaryArchiveDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLIntersectionFunctionBufferArguments {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLIntersectionFunctionBufferArguments {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTLIntersectionFunctionBufferArguments", &[]);
}
unsafe impl objc2::encode::RefEncode for MTLIntersectionFunctionTableDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLIntersectionFunctionTableDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFunctionStitchingAttributeAlwaysInline {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFunctionStitchingAttributeAlwaysInline {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFunctionStitchingInputNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFunctionStitchingInputNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFunctionStitchingFunctionNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFunctionStitchingFunctionNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFunctionStitchingGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFunctionStitchingGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLStitchedLibraryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLStitchedLibraryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLIOCommandQueueDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLIOCommandQueueDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLResidencySetDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLResidencySetDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLResourceViewPoolDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLResourceViewPoolDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4ArgumentTableDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4ArgumentTableDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4CommandAllocatorDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4CommandAllocatorDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4RenderPassDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4RenderPassDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4AccelerationStructureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4AccelerationStructureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4AccelerationStructureGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4AccelerationStructureGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4PrimitiveAccelerationStructureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4PrimitiveAccelerationStructureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4AccelerationStructureTriangleGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4AccelerationStructureTriangleGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4AccelerationStructureCurveGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4AccelerationStructureCurveGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4AccelerationStructureMotionCurveGeometryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4AccelerationStructureMotionCurveGeometryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4InstanceAccelerationStructureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4InstanceAccelerationStructureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4IndirectInstanceAccelerationStructureDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4IndirectInstanceAccelerationStructureDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4LibraryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4LibraryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4FunctionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4FunctionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4LibraryFunctionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4LibraryFunctionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4SpecializedFunctionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4SpecializedFunctionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4StitchedFunctionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4StitchedFunctionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4PipelineOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4PipelineOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4PipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4PipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4ComputePipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4ComputePipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4RenderPipelineColorAttachmentDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4RenderPipelineColorAttachmentDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4RenderPipelineColorAttachmentDescriptorArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4RenderPipelineColorAttachmentDescriptorArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4RenderPipelineBinaryFunctionsDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4RenderPipelineBinaryFunctionsDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4RenderPipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4RenderPipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4MachineLearningPipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4MachineLearningPipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4MachineLearningPipelineReflection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4MachineLearningPipelineReflection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4TileRenderPipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4TileRenderPipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4MeshRenderPipelineDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4MeshRenderPipelineDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4PipelineDataSetSerializerDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4PipelineDataSetSerializerDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4BinaryFunctionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4BinaryFunctionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4BinaryFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4BinaryFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLDynamicLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLDynamicLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4StaticLinkingDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4StaticLinkingDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4PipelineStageDynamicLinkingDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4PipelineStageDynamicLinkingDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTL4RenderPipelineDynamicLinkingDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTL4RenderPipelineDynamicLinkingDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
