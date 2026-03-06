#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type simd_int2 = [::std::os::raw::c_int; 2usize];
pub type simd_int4 = [::std::os::raw::c_int; 4usize];
pub type simd_uint2 = [::std::os::raw::c_uint; 2usize];
pub type simd_uint3 = [::std::os::raw::c_uint; 3usize];
pub type simd_float2 = [f32; 2usize];
pub type simd_float4 = [f32; 4usize];
pub type simd_double2 = [f64; 2usize];
pub type simd_double3 = [f64; 3usize];
pub type simd_double4 = [f64; 4usize];
pub type vector_int2 = simd_int2;
pub type vector_int4 = simd_int4;
pub type vector_uint2 = simd_uint2;
pub type vector_uint3 = simd_uint3;
pub type vector_float2 = simd_float2;
pub type vector_float3 = simd_float3;
pub type vector_float4 = simd_float4;
pub type vector_double2 = simd_double2;
pub type vector_double3 = simd_double3;
pub type vector_double4 = simd_double4;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct simd_float4x4 {
    pub columns: [simd_float4; 4usize],
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct simd_double4x4 {
    pub columns: [simd_double4; 4usize],
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct simd_quatf {
    pub vector: simd_float4,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct simd_quatd {
    pub vector: simd_double4,
}
pub type matrix_float4x4 = simd_float4x4;
pub type matrix_double4x4 = simd_double4x4;
pub trait PMDLAssetResolver: Sized + std::ops::Deref {
    unsafe fn canResolveAssetNamed_(&self, name: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canResolveAssetNamed : name)
    }
    unsafe fn resolveAssetNamed_(&self, name: NSString) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveAssetNamed : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLRelativeAssetResolver(pub id);
impl std::ops::Deref for MDLRelativeAssetResolver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLRelativeAssetResolver {}
impl MDLRelativeAssetResolver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLRelativeAssetResolver").unwrap(), alloc) })
    }
}
impl PMDLAssetResolver for MDLRelativeAssetResolver {}
impl INSObject for MDLRelativeAssetResolver {}
impl PNSObject for MDLRelativeAssetResolver {}
impl std::convert::TryFrom<NSObject> for MDLRelativeAssetResolver {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLRelativeAssetResolver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLRelativeAssetResolver").unwrap()) };
        if is_kind_of {
            Ok(MDLRelativeAssetResolver(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLRelativeAssetResolver")
        }
    }
}
impl IMDLRelativeAssetResolver for MDLRelativeAssetResolver {}
pub trait IMDLRelativeAssetResolver: Sized + std::ops::Deref {
    unsafe fn initWithAsset_(&self, asset: MDLAsset) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAsset : asset)
    }
    unsafe fn asset(&self) -> MDLAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, asset)
    }
    unsafe fn setAsset_(&self, asset: MDLAsset)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAsset : asset)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLPathAssetResolver(pub id);
impl std::ops::Deref for MDLPathAssetResolver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLPathAssetResolver {}
impl MDLPathAssetResolver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLPathAssetResolver").unwrap(), alloc) })
    }
}
impl PMDLAssetResolver for MDLPathAssetResolver {}
impl INSObject for MDLPathAssetResolver {}
impl PNSObject for MDLPathAssetResolver {}
impl std::convert::TryFrom<NSObject> for MDLPathAssetResolver {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLPathAssetResolver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLPathAssetResolver").unwrap()) };
        if is_kind_of {
            Ok(MDLPathAssetResolver(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLPathAssetResolver")
        }
    }
}
impl IMDLPathAssetResolver for MDLPathAssetResolver {}
pub trait IMDLPathAssetResolver: Sized + std::ops::Deref {
    unsafe fn initWithPath_(&self, path: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPath : path)
    }
    unsafe fn path(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn setPath_(&self, path: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPath : path)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLBundleAssetResolver(pub id);
impl std::ops::Deref for MDLBundleAssetResolver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLBundleAssetResolver {}
impl MDLBundleAssetResolver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLBundleAssetResolver").unwrap(), alloc) })
    }
}
impl PMDLAssetResolver for MDLBundleAssetResolver {}
impl INSObject for MDLBundleAssetResolver {}
impl PNSObject for MDLBundleAssetResolver {}
impl std::convert::TryFrom<NSObject> for MDLBundleAssetResolver {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLBundleAssetResolver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLBundleAssetResolver").unwrap()) };
        if is_kind_of {
            Ok(MDLBundleAssetResolver(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLBundleAssetResolver")
        }
    }
}
impl IMDLBundleAssetResolver for MDLBundleAssetResolver {}
pub trait IMDLBundleAssetResolver: Sized + std::ops::Deref {
    unsafe fn initWithBundle_(&self, path: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBundle : path)
    }
    unsafe fn path(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn setPath_(&self, path: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPath : path)
    }
}
pub type MDLIndexBitDepth = NSUInteger;
pub type MDLGeometryType = NSInteger;
pub type MDLProbePlacement = NSInteger;
pub type MDLDataPrecision = NSUInteger;
pub trait PMDLNamed: Sized + std::ops::Deref {
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
pub trait PMDLComponent: Sized + std::ops::Deref {}
pub trait PMDLObjectContainerComponent: Sized + std::ops::Deref {
    unsafe fn addObject_(&self, object: MDLObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObject : object)
    }
    unsafe fn removeObject_(&self, object: MDLObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeObject : object)
    }
    unsafe fn objectAtIndexedSubscript_(&self, index: NSUInteger) -> MDLObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : index)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn objects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objects)
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct MDLAxisAlignedBoundingBox {
    pub maxBounds: vector_float3,
    pub minBounds: vector_float3,
}
pub trait PMDLTransformComponent: Sized + std::ops::Deref {
    unsafe fn setLocalTransform_forTime_(&self, transform: matrix_float4x4, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalTransform : transform, forTime : time)
    }
    unsafe fn setLocalTransform_(&self, transform: matrix_float4x4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalTransform : transform)
    }
    unsafe fn localTransformAtTime_(&self, time: NSTimeInterval) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localTransformAtTime : time)
    }
    unsafe fn matrix(&self) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matrix)
    }
    unsafe fn setMatrix_(&self, matrix: matrix_float4x4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatrix : matrix)
    }
    unsafe fn resetsTransform(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetsTransform)
    }
    unsafe fn setResetsTransform_(&self, resetsTransform: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResetsTransform : resetsTransform)
    }
    unsafe fn minimumTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumTime)
    }
    unsafe fn maximumTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumTime)
    }
    unsafe fn keyTimes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyTimes)
    }
    unsafe fn globalTransformWithObject_atTime_(
        object: MDLObject,
        time: NSTimeInterval,
    ) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformComponent").unwrap(), globalTransformWithObject : object, atTime : time)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransform(pub id);
impl std::ops::Deref for MDLTransform {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransform {}
impl MDLTransform {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransform").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLTransform {}
impl PMDLTransformComponent for MDLTransform {}
impl INSObject for MDLTransform {}
impl PNSObject for MDLTransform {}
impl std::convert::TryFrom<NSObject> for MDLTransform {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransform, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransform").unwrap()) };
        if is_kind_of {
            Ok(MDLTransform(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransform")
        }
    }
}
impl IMDLTransform for MDLTransform {}
pub trait IMDLTransform: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithIdentity(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initWithIdentity)
    }
    unsafe fn initWithTransformComponent_(&self, component: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTransformComponent : component)
    }
    unsafe fn initWithTransformComponent_resetsTransform_(
        &self,
        component: *mut u64,
        resetsTransform: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTransformComponent : component, resetsTransform : resetsTransform)
    }
    unsafe fn initWithMatrix_(&self, matrix: matrix_float4x4) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMatrix : matrix)
    }
    unsafe fn initWithMatrix_resetsTransform_(
        &self,
        matrix: matrix_float4x4,
        resetsTransform: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMatrix : matrix, resetsTransform : resetsTransform)
    }
    unsafe fn setIdentity(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setIdentity)
    }
    unsafe fn translationAtTime_(&self, time: NSTimeInterval) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, translationAtTime : time)
    }
    unsafe fn rotationAtTime_(&self, time: NSTimeInterval) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotationAtTime : time)
    }
    unsafe fn shearAtTime_(&self, time: NSTimeInterval) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shearAtTime : time)
    }
    unsafe fn scaleAtTime_(&self, time: NSTimeInterval) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scaleAtTime : time)
    }
    unsafe fn setMatrix_forTime_(&self, matrix: matrix_float4x4, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatrix : matrix, forTime : time)
    }
    unsafe fn setTranslation_forTime_(&self, translation: vector_float3, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTranslation : translation, forTime : time)
    }
    unsafe fn setRotation_forTime_(&self, rotation: vector_float3, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation, forTime : time)
    }
    unsafe fn setShear_forTime_(&self, shear: vector_float3, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShear : shear, forTime : time)
    }
    unsafe fn setScale_forTime_(&self, scale: vector_float3, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale, forTime : time)
    }
    unsafe fn rotationMatrixAtTime_(&self, time: NSTimeInterval) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotationMatrixAtTime : time)
    }
    unsafe fn translation(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, translation)
    }
    unsafe fn setTranslation_(&self, translation: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTranslation : translation)
    }
    unsafe fn rotation(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn shear(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shear)
    }
    unsafe fn setShear_(&self, shear: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShear : shear)
    }
    unsafe fn scale(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLObject(pub id);
impl std::ops::Deref for MDLObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLObject {}
impl MDLObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLObject").unwrap(), alloc) })
    }
}
impl PMDLNamed for MDLObject {}
impl INSObject for MDLObject {}
impl PNSObject for MDLObject {}
impl std::convert::TryFrom<NSObject> for MDLObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLObject, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLObject").unwrap()) };
        if is_kind_of {
            Ok(MDLObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLObject")
        }
    }
}
impl IMDLObject for MDLObject {}
pub trait IMDLObject: Sized + std::ops::Deref {
    unsafe fn setComponent_forProtocol_(&self, component: *mut u64, protocol: Protocol)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComponent : component, forProtocol : protocol)
    }
    unsafe fn componentConformingToProtocol_(&self, protocol: Protocol) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, componentConformingToProtocol : protocol)
    }
    unsafe fn objectForKeyedSubscript_(&self, key: Protocol) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn setObject_forKeyedSubscript_(&self, obj: *mut u64, key: Protocol)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : obj, forKeyedSubscript : key)
    }
    unsafe fn objectAtPath_(&self, path: NSString) -> MDLObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtPath : path)
    }
    unsafe fn enumerateChildObjectsOfClass_root_usingBlock_stopPointer_(
        &self,
        objectClass: Class,
        root: MDLObject,
        block: *mut ::std::os::raw::c_void,
        stopPointer: *mut BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateChildObjectsOfClass : objectClass, root : root, usingBlock : block, stopPointer : stopPointer)
    }
    unsafe fn addChild_(&self, child: MDLObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addChild : child)
    }
    unsafe fn boundingBoxAtTime_(&self, time: NSTimeInterval) -> MDLAxisAlignedBoundingBox
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingBoxAtTime : time)
    }
    unsafe fn components(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, components)
    }
    unsafe fn parent(&self) -> MDLObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parent)
    }
    unsafe fn setParent_(&self, parent: MDLObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParent : parent)
    }
    unsafe fn instance(&self) -> MDLObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instance)
    }
    unsafe fn setInstance_(&self, instance: MDLObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstance : instance)
    }
    unsafe fn path(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn transform(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn setTransform_(&self, transform: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransform : transform)
    }
    unsafe fn children(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, children)
    }
    unsafe fn setChildren_(&self, children: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChildren : children)
    }
    unsafe fn hidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hidden)
    }
    unsafe fn setHidden_(&self, hidden: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidden : hidden)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLObjectContainer(pub id);
impl std::ops::Deref for MDLObjectContainer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLObjectContainer {}
impl MDLObjectContainer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLObjectContainer").unwrap(), alloc) })
    }
}
impl PMDLObjectContainerComponent for MDLObjectContainer {}
impl INSObject for MDLObjectContainer {}
impl PNSObject for MDLObjectContainer {}
impl std::convert::TryFrom<NSObject> for MDLObjectContainer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLObjectContainer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLObjectContainer").unwrap()) };
        if is_kind_of {
            Ok(MDLObjectContainer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLObjectContainer")
        }
    }
}
impl IMDLObjectContainer for MDLObjectContainer {}
pub trait IMDLObjectContainer: Sized + std::ops::Deref {}
pub type MDLMeshBufferType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMeshBufferMap(pub id);
impl std::ops::Deref for MDLMeshBufferMap {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMeshBufferMap {}
impl MDLMeshBufferMap {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMeshBufferMap").unwrap(), alloc) })
    }
}
impl INSObject for MDLMeshBufferMap {}
impl PNSObject for MDLMeshBufferMap {}
impl std::convert::TryFrom<NSObject> for MDLMeshBufferMap {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLMeshBufferMap, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMeshBufferMap").unwrap()) };
        if is_kind_of {
            Ok(MDLMeshBufferMap(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLMeshBufferMap")
        }
    }
}
impl IMDLMeshBufferMap for MDLMeshBufferMap {}
pub trait IMDLMeshBufferMap: Sized + std::ops::Deref {
    unsafe fn initWithBytes_deallocator_(
        &self,
        bytes: *mut ::std::os::raw::c_void,
        deallocator: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBytes : bytes, deallocator : deallocator)
    }
    unsafe fn bytes(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytes)
    }
}
pub trait PMDLMeshBuffer: Sized + std::ops::Deref {
    unsafe fn fillData_offset_(&self, data: NSData, offset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fillData : data, offset : offset)
    }
    unsafe fn map(&self) -> MDLMeshBufferMap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, map)
    }
    unsafe fn length(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn allocator(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocator)
    }
    unsafe fn zone(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zone)
    }
    unsafe fn type_(&self) -> MDLMeshBufferType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMeshBufferData(pub id);
impl std::ops::Deref for MDLMeshBufferData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMeshBufferData {}
impl MDLMeshBufferData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMeshBufferData").unwrap(), alloc) })
    }
}
impl PMDLMeshBuffer for MDLMeshBufferData {}
impl INSObject for MDLMeshBufferData {}
impl PNSObject for MDLMeshBufferData {}
impl std::convert::TryFrom<NSObject> for MDLMeshBufferData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLMeshBufferData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMeshBufferData").unwrap()) };
        if is_kind_of {
            Ok(MDLMeshBufferData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLMeshBufferData")
        }
    }
}
impl IMDLMeshBufferData for MDLMeshBufferData {}
pub trait IMDLMeshBufferData: Sized + std::ops::Deref {
    unsafe fn initWithType_length_(
        &self,
        type_: MDLMeshBufferType,
        length: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : type_, length : length)
    }
    unsafe fn initWithType_data_(&self, type_: MDLMeshBufferType, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : type_, data : data)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
}
pub trait PMDLMeshBufferZone: Sized + std::ops::Deref {
    unsafe fn capacity(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capacity)
    }
    unsafe fn allocator(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocator)
    }
}
pub trait PMDLMeshBufferAllocator: Sized + std::ops::Deref {
    unsafe fn newZone_(&self, capacity: NSUInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newZone : capacity)
    }
    unsafe fn newZoneForBuffersWithSize_andType_(&self, sizes: NSArray, types: NSArray) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newZoneForBuffersWithSize : sizes, andType : types)
    }
    unsafe fn newBuffer_type_(&self, length: NSUInteger, type_: MDLMeshBufferType) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBuffer : length, r#type : type_)
    }
    unsafe fn newBufferWithData_type_(&self, data: NSData, type_: MDLMeshBufferType) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBufferWithData : data, r#type : type_)
    }
    unsafe fn newBufferFromZone_length_type_(
        &self,
        zone: *mut u64,
        length: NSUInteger,
        type_: MDLMeshBufferType,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBufferFromZone : zone, length : length, r#type : type_)
    }
    unsafe fn newBufferFromZone_data_type_(
        &self,
        zone: *mut u64,
        data: NSData,
        type_: MDLMeshBufferType,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBufferFromZone : zone, data : data, r#type : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMeshBufferDataAllocator(pub id);
impl std::ops::Deref for MDLMeshBufferDataAllocator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMeshBufferDataAllocator {}
impl MDLMeshBufferDataAllocator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMeshBufferDataAllocator").unwrap(), alloc) })
    }
}
impl PMDLMeshBufferAllocator for MDLMeshBufferDataAllocator {}
impl INSObject for MDLMeshBufferDataAllocator {}
impl PNSObject for MDLMeshBufferDataAllocator {}
impl std::convert::TryFrom<NSObject> for MDLMeshBufferDataAllocator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLMeshBufferDataAllocator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMeshBufferDataAllocator").unwrap()) };
        if is_kind_of {
            Ok(MDLMeshBufferDataAllocator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLMeshBufferDataAllocator")
        }
    }
}
impl IMDLMeshBufferDataAllocator for MDLMeshBufferDataAllocator {}
pub trait IMDLMeshBufferDataAllocator: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMeshBufferZoneDefault(pub id);
impl std::ops::Deref for MDLMeshBufferZoneDefault {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMeshBufferZoneDefault {}
impl MDLMeshBufferZoneDefault {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMeshBufferZoneDefault").unwrap(), alloc) })
    }
}
impl PMDLMeshBufferZone for MDLMeshBufferZoneDefault {}
impl INSObject for MDLMeshBufferZoneDefault {}
impl PNSObject for MDLMeshBufferZoneDefault {}
impl std::convert::TryFrom<NSObject> for MDLMeshBufferZoneDefault {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLMeshBufferZoneDefault, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMeshBufferZoneDefault").unwrap()) };
        if is_kind_of {
            Ok(MDLMeshBufferZoneDefault(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLMeshBufferZoneDefault")
        }
    }
}
impl IMDLMeshBufferZoneDefault for MDLMeshBufferZoneDefault {}
pub trait IMDLMeshBufferZoneDefault: Sized + std::ops::Deref {
    unsafe fn capacity(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capacity)
    }
    unsafe fn allocator(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocator)
    }
}
pub type MDLVertexFormat = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLVertexBufferLayout(pub id);
impl std::ops::Deref for MDLVertexBufferLayout {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLVertexBufferLayout {}
impl MDLVertexBufferLayout {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLVertexBufferLayout").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLVertexBufferLayout {}
impl INSObject for MDLVertexBufferLayout {}
impl PNSObject for MDLVertexBufferLayout {}
impl std::convert::TryFrom<NSObject> for MDLVertexBufferLayout {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLVertexBufferLayout, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLVertexBufferLayout").unwrap()) };
        if is_kind_of {
            Ok(MDLVertexBufferLayout(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLVertexBufferLayout")
        }
    }
}
impl IMDLVertexBufferLayout for MDLVertexBufferLayout {}
pub trait IMDLVertexBufferLayout: Sized + std::ops::Deref {
    unsafe fn initWithStride_(&self, stride: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStride : stride)
    }
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLVertexAttribute(pub id);
impl std::ops::Deref for MDLVertexAttribute {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLVertexAttribute {}
impl MDLVertexAttribute {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLVertexAttribute").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLVertexAttribute {}
impl INSObject for MDLVertexAttribute {}
impl PNSObject for MDLVertexAttribute {}
impl std::convert::TryFrom<NSObject> for MDLVertexAttribute {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLVertexAttribute, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLVertexAttribute").unwrap()) };
        if is_kind_of {
            Ok(MDLVertexAttribute(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLVertexAttribute")
        }
    }
}
impl IMDLVertexAttribute for MDLVertexAttribute {}
pub trait IMDLVertexAttribute: Sized + std::ops::Deref {
    unsafe fn initWithName_format_offset_bufferIndex_(
        &self,
        name: NSString,
        format: MDLVertexFormat,
        offset: NSUInteger,
        bufferIndex: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, format : format, offset : offset, bufferIndex : bufferIndex)
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
    unsafe fn format(&self) -> MDLVertexFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn setFormat_(&self, format: MDLVertexFormat)
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
    unsafe fn time(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn setTime_(&self, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTime : time)
    }
    unsafe fn initializationValue(&self) -> vector_float4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initializationValue)
    }
    unsafe fn setInitializationValue_(&self, initializationValue: vector_float4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitializationValue : initializationValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLVertexDescriptor(pub id);
impl std::ops::Deref for MDLVertexDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLVertexDescriptor {}
impl MDLVertexDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLVertexDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLVertexDescriptor {}
impl INSObject for MDLVertexDescriptor {}
impl PNSObject for MDLVertexDescriptor {}
impl std::convert::TryFrom<NSObject> for MDLVertexDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLVertexDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLVertexDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MDLVertexDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLVertexDescriptor")
        }
    }
}
impl IMDLVertexDescriptor for MDLVertexDescriptor {}
pub trait IMDLVertexDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithVertexDescriptor_(
        &self,
        vertexDescriptor: MDLVertexDescriptor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVertexDescriptor : vertexDescriptor)
    }
    unsafe fn attributeNamed_(&self, name: NSString) -> MDLVertexAttribute
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributeNamed : name)
    }
    unsafe fn addOrReplaceAttribute_(&self, attribute: MDLVertexAttribute)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOrReplaceAttribute : attribute)
    }
    unsafe fn removeAttributeNamed_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAttributeNamed : name)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn setPackedStrides(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setPackedStrides)
    }
    unsafe fn setPackedOffsets(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setPackedOffsets)
    }
    unsafe fn attributes(&self) -> NSMutableArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn setAttributes_(&self, attributes: NSMutableArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributes : attributes)
    }
    unsafe fn layouts(&self) -> NSMutableArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layouts)
    }
    unsafe fn setLayouts_(&self, layouts: NSMutableArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayouts : layouts)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMatrix4x4Array(pub id);
impl std::ops::Deref for MDLMatrix4x4Array {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMatrix4x4Array {}
impl MDLMatrix4x4Array {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMatrix4x4Array").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLMatrix4x4Array {}
impl INSObject for MDLMatrix4x4Array {}
impl PNSObject for MDLMatrix4x4Array {}
impl std::convert::TryFrom<NSObject> for MDLMatrix4x4Array {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLMatrix4x4Array, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMatrix4x4Array").unwrap()) };
        if is_kind_of {
            Ok(MDLMatrix4x4Array(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLMatrix4x4Array")
        }
    }
}
impl IMDLMatrix4x4Array for MDLMatrix4x4Array {}
pub trait IMDLMatrix4x4Array: Sized + std::ops::Deref {
    unsafe fn clear(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clear)
    }
    unsafe fn initWithElementCount_(&self, arrayElementCount: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElementCount : arrayElementCount)
    }
    unsafe fn setFloat4x4Array_count_(&self, valuesArray: *const matrix_float4x4, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat4x4Array : valuesArray, count : count)
    }
    unsafe fn setDouble4x4Array_count_(
        &self,
        valuesArray: *const matrix_double4x4,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDouble4x4Array : valuesArray, count : count)
    }
    unsafe fn getFloat4x4Array_maxCount_(
        &self,
        valuesArray: *mut matrix_float4x4,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloat4x4Array : valuesArray, maxCount : maxCount)
    }
    unsafe fn getDouble4x4Array_maxCount_(
        &self,
        valuesArray: *mut matrix_double4x4,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDouble4x4Array : valuesArray, maxCount : maxCount)
    }
    unsafe fn elementCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementCount)
    }
    unsafe fn precision(&self) -> MDLDataPrecision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, precision)
    }
}
pub type MDLAnimatedValueInterpolation = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedValue(pub id);
impl std::ops::Deref for MDLAnimatedValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedValue {}
impl MDLAnimatedValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedValue").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLAnimatedValue {}
impl INSObject for MDLAnimatedValue {}
impl PNSObject for MDLAnimatedValue {}
impl std::convert::TryFrom<NSObject> for MDLAnimatedValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLAnimatedValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedValue").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLAnimatedValue")
        }
    }
}
impl IMDLAnimatedValue for MDLAnimatedValue {}
pub trait IMDLAnimatedValue: Sized + std::ops::Deref {
    unsafe fn isAnimated(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnimated)
    }
    unsafe fn clear(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clear)
    }
    unsafe fn getTimes_maxCount_(
        &self,
        timesArray: *mut NSTimeInterval,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getTimes : timesArray, maxCount : maxCount)
    }
    unsafe fn precision(&self) -> MDLDataPrecision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, precision)
    }
    unsafe fn timeSampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeSampleCount)
    }
    unsafe fn minimumTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumTime)
    }
    unsafe fn maximumTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumTime)
    }
    unsafe fn interpolation(&self) -> MDLAnimatedValueInterpolation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interpolation)
    }
    unsafe fn setInterpolation_(&self, interpolation: MDLAnimatedValueInterpolation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterpolation : interpolation)
    }
    unsafe fn keyTimes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyTimes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedScalarArray(pub id);
impl std::ops::Deref for MDLAnimatedScalarArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedScalarArray {}
impl MDLAnimatedScalarArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedScalarArray").unwrap(), alloc) })
    }
}
impl IMDLAnimatedValue for MDLAnimatedScalarArray {}
impl PNSCopying for MDLAnimatedScalarArray {}
impl From<MDLAnimatedScalarArray> for MDLAnimatedValue {
    fn from(child: MDLAnimatedScalarArray) -> MDLAnimatedValue {
        MDLAnimatedValue(child.0)
    }
}
impl std::convert::TryFrom<MDLAnimatedValue> for MDLAnimatedScalarArray {
    type Error = &'static str;
    fn try_from(parent: MDLAnimatedValue) -> Result<MDLAnimatedScalarArray, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedScalarArray").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedScalarArray(parent.0))
        } else {
            Err("This MDLAnimatedValue cannot be downcasted to MDLAnimatedScalarArray")
        }
    }
}
impl INSObject for MDLAnimatedScalarArray {}
impl PNSObject for MDLAnimatedScalarArray {}
impl IMDLAnimatedScalarArray for MDLAnimatedScalarArray {}
pub trait IMDLAnimatedScalarArray: Sized + std::ops::Deref {
    unsafe fn initWithElementCount_(&self, arrayElementCount: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElementCount : arrayElementCount)
    }
    unsafe fn setFloatArray_count_atTime_(
        &self,
        array: *const f32,
        count: NSUInteger,
        time: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatArray : array, count : count, atTime : time)
    }
    unsafe fn setDoubleArray_count_atTime_(
        &self,
        array: *const f64,
        count: NSUInteger,
        time: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoubleArray : array, count : count, atTime : time)
    }
    unsafe fn getFloatArray_maxCount_atTime_(
        &self,
        array: *mut f32,
        maxCount: NSUInteger,
        time: NSTimeInterval,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloatArray : array, maxCount : maxCount, atTime : time)
    }
    unsafe fn getDoubleArray_maxCount_atTime_(
        &self,
        array: *mut f64,
        maxCount: NSUInteger,
        time: NSTimeInterval,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDoubleArray : array, maxCount : maxCount, atTime : time)
    }
    unsafe fn resetWithFloatArray_count_atTimes_count_(
        &self,
        valuesArray: *const f32,
        valuesCount: NSUInteger,
        timesArray: *const NSTimeInterval,
        timesCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithFloatArray : valuesArray, count : valuesCount, atTimes : timesArray, count : timesCount)
    }
    unsafe fn resetWithDoubleArray_count_atTimes_count_(
        &self,
        valuesArray: *const f64,
        valuesCount: NSUInteger,
        timesArray: *const NSTimeInterval,
        timesCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithDoubleArray : valuesArray, count : valuesCount, atTimes : timesArray, count : timesCount)
    }
    unsafe fn getFloatArray_maxCount_(
        &self,
        valuesArray: *mut f32,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloatArray : valuesArray, maxCount : maxCount)
    }
    unsafe fn getDoubleArray_maxCount_(
        &self,
        valuesArray: *mut f64,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDoubleArray : valuesArray, maxCount : maxCount)
    }
    unsafe fn elementCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedVector3Array(pub id);
impl std::ops::Deref for MDLAnimatedVector3Array {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedVector3Array {}
impl MDLAnimatedVector3Array {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedVector3Array").unwrap(), alloc) })
    }
}
impl IMDLAnimatedValue for MDLAnimatedVector3Array {}
impl PNSCopying for MDLAnimatedVector3Array {}
impl std::convert::TryFrom<MDLAnimatedValue> for MDLAnimatedVector3Array {
    type Error = &'static str;
    fn try_from(parent: MDLAnimatedValue) -> Result<MDLAnimatedVector3Array, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedVector3Array").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedVector3Array(parent.0))
        } else {
            Err("This MDLAnimatedValue cannot be downcasted to MDLAnimatedVector3Array")
        }
    }
}
impl INSObject for MDLAnimatedVector3Array {}
impl PNSObject for MDLAnimatedVector3Array {}
impl IMDLAnimatedVector3Array for MDLAnimatedVector3Array {}
pub trait IMDLAnimatedVector3Array: Sized + std::ops::Deref {
    unsafe fn initWithElementCount_(&self, arrayElementCount: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElementCount : arrayElementCount)
    }
    unsafe fn setFloat3Array_count_atTime_(
        &self,
        array: *const vector_float3,
        count: NSUInteger,
        time: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat3Array : array, count : count, atTime : time)
    }
    unsafe fn setDouble3Array_count_atTime_(
        &self,
        array: *const vector_double3,
        count: NSUInteger,
        time: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDouble3Array : array, count : count, atTime : time)
    }
    unsafe fn getFloat3Array_maxCount_atTime_(
        &self,
        array: *mut vector_float3,
        maxCount: NSUInteger,
        time: NSTimeInterval,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloat3Array : array, maxCount : maxCount, atTime : time)
    }
    unsafe fn getDouble3Array_maxCount_atTime_(
        &self,
        array: *mut vector_double3,
        maxCount: NSUInteger,
        time: NSTimeInterval,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDouble3Array : array, maxCount : maxCount, atTime : time)
    }
    unsafe fn resetWithFloat3Array_count_atTimes_count_(
        &self,
        valuesArray: *const vector_float3,
        valuesCount: NSUInteger,
        timesArray: *const NSTimeInterval,
        timesCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithFloat3Array : valuesArray, count : valuesCount, atTimes : timesArray, count : timesCount)
    }
    unsafe fn resetWithDouble3Array_count_atTimes_count_(
        &self,
        valuesArray: *const vector_double3,
        valuesCount: NSUInteger,
        timesArray: *const NSTimeInterval,
        timesCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithDouble3Array : valuesArray, count : valuesCount, atTimes : timesArray, count : timesCount)
    }
    unsafe fn getFloat3Array_maxCount_(
        &self,
        valuesArray: *mut vector_float3,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloat3Array : valuesArray, maxCount : maxCount)
    }
    unsafe fn getDouble3Array_maxCount_(
        &self,
        valuesArray: *mut vector_double3,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDouble3Array : valuesArray, maxCount : maxCount)
    }
    unsafe fn elementCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedQuaternionArray(pub id);
impl std::ops::Deref for MDLAnimatedQuaternionArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedQuaternionArray {}
impl MDLAnimatedQuaternionArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedQuaternionArray").unwrap(), alloc) })
    }
}
impl IMDLAnimatedValue for MDLAnimatedQuaternionArray {}
impl PNSCopying for MDLAnimatedQuaternionArray {}
impl std::convert::TryFrom<MDLAnimatedValue> for MDLAnimatedQuaternionArray {
    type Error = &'static str;
    fn try_from(parent: MDLAnimatedValue) -> Result<MDLAnimatedQuaternionArray, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedQuaternionArray").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedQuaternionArray(parent.0))
        } else {
            Err("This MDLAnimatedValue cannot be downcasted to MDLAnimatedQuaternionArray")
        }
    }
}
impl INSObject for MDLAnimatedQuaternionArray {}
impl PNSObject for MDLAnimatedQuaternionArray {}
impl IMDLAnimatedQuaternionArray for MDLAnimatedQuaternionArray {}
pub trait IMDLAnimatedQuaternionArray: Sized + std::ops::Deref {
    unsafe fn initWithElementCount_(&self, arrayElementCount: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElementCount : arrayElementCount)
    }
    unsafe fn setDoubleQuaternionArray_count_atTime_(
        &self,
        array: *const simd_quatd,
        count: NSUInteger,
        time: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoubleQuaternionArray : array, count : count, atTime : time)
    }
    unsafe fn getDoubleQuaternionArray_maxCount_atTime_(
        &self,
        array: *mut simd_quatd,
        maxCount: NSUInteger,
        time: NSTimeInterval,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDoubleQuaternionArray : array, maxCount : maxCount, atTime : time)
    }
    unsafe fn resetWithDoubleQuaternionArray_count_atTimes_count_(
        &self,
        valuesArray: *const simd_quatd,
        valuesCount: NSUInteger,
        timesArray: *const NSTimeInterval,
        timesCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithDoubleQuaternionArray : valuesArray, count : valuesCount, atTimes : timesArray, count : timesCount)
    }
    unsafe fn getDoubleQuaternionArray_maxCount_(
        &self,
        valuesArray: *mut simd_quatd,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDoubleQuaternionArray : valuesArray, maxCount : maxCount)
    }
    unsafe fn elementCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedScalar(pub id);
impl std::ops::Deref for MDLAnimatedScalar {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedScalar {}
impl MDLAnimatedScalar {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedScalar").unwrap(), alloc) })
    }
}
impl IMDLAnimatedValue for MDLAnimatedScalar {}
impl PNSCopying for MDLAnimatedScalar {}
impl std::convert::TryFrom<MDLAnimatedValue> for MDLAnimatedScalar {
    type Error = &'static str;
    fn try_from(parent: MDLAnimatedValue) -> Result<MDLAnimatedScalar, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedScalar").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedScalar(parent.0))
        } else {
            Err("This MDLAnimatedValue cannot be downcasted to MDLAnimatedScalar")
        }
    }
}
impl INSObject for MDLAnimatedScalar {}
impl PNSObject for MDLAnimatedScalar {}
impl IMDLAnimatedScalar for MDLAnimatedScalar {}
pub trait IMDLAnimatedScalar: Sized + std::ops::Deref {
    unsafe fn setFloat_atTime_(&self, value: f32, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat : value, atTime : time)
    }
    unsafe fn setDouble_atTime_(&self, value: f64, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDouble : value, atTime : time)
    }
    unsafe fn floatAtTime_(&self, time: NSTimeInterval) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, floatAtTime : time)
    }
    unsafe fn doubleAtTime_(&self, time: NSTimeInterval) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, doubleAtTime : time)
    }
    unsafe fn resetWithFloatArray_atTimes_count_(
        &self,
        valuesArray: *const f32,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithFloatArray : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn resetWithDoubleArray_atTimes_count_(
        &self,
        valuesArray: *const f64,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithDoubleArray : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn getFloatArray_maxCount_(
        &self,
        valuesArray: *mut f32,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloatArray : valuesArray, maxCount : maxCount)
    }
    unsafe fn getDoubleArray_maxCount_(
        &self,
        valuesArray: *mut f64,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDoubleArray : valuesArray, maxCount : maxCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedVector2(pub id);
impl std::ops::Deref for MDLAnimatedVector2 {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedVector2 {}
impl MDLAnimatedVector2 {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedVector2").unwrap(), alloc) })
    }
}
impl IMDLAnimatedValue for MDLAnimatedVector2 {}
impl PNSCopying for MDLAnimatedVector2 {}
impl std::convert::TryFrom<MDLAnimatedValue> for MDLAnimatedVector2 {
    type Error = &'static str;
    fn try_from(parent: MDLAnimatedValue) -> Result<MDLAnimatedVector2, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedVector2").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedVector2(parent.0))
        } else {
            Err("This MDLAnimatedValue cannot be downcasted to MDLAnimatedVector2")
        }
    }
}
impl INSObject for MDLAnimatedVector2 {}
impl PNSObject for MDLAnimatedVector2 {}
impl IMDLAnimatedVector2 for MDLAnimatedVector2 {}
pub trait IMDLAnimatedVector2: Sized + std::ops::Deref {
    unsafe fn setFloat2_atTime_(&self, value: vector_float2, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat2 : value, atTime : time)
    }
    unsafe fn setDouble2_atTime_(&self, value: vector_double2, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDouble2 : value, atTime : time)
    }
    unsafe fn float2AtTime_(&self, time: NSTimeInterval) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, float2AtTime : time)
    }
    unsafe fn double2AtTime_(&self, time: NSTimeInterval) -> vector_double2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, double2AtTime : time)
    }
    unsafe fn resetWithFloat2Array_atTimes_count_(
        &self,
        valuesArray: *const vector_float2,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithFloat2Array : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn resetWithDouble2Array_atTimes_count_(
        &self,
        valuesArray: *const vector_double2,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithDouble2Array : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn getFloat2Array_maxCount_(
        &self,
        valuesArray: *mut vector_float2,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloat2Array : valuesArray, maxCount : maxCount)
    }
    unsafe fn getDouble2Array_maxCount_(
        &self,
        valuesArray: *mut vector_double2,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDouble2Array : valuesArray, maxCount : maxCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedVector3(pub id);
impl std::ops::Deref for MDLAnimatedVector3 {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedVector3 {}
impl MDLAnimatedVector3 {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedVector3").unwrap(), alloc) })
    }
}
impl IMDLAnimatedValue for MDLAnimatedVector3 {}
impl PNSCopying for MDLAnimatedVector3 {}
impl std::convert::TryFrom<MDLAnimatedValue> for MDLAnimatedVector3 {
    type Error = &'static str;
    fn try_from(parent: MDLAnimatedValue) -> Result<MDLAnimatedVector3, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedVector3").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedVector3(parent.0))
        } else {
            Err("This MDLAnimatedValue cannot be downcasted to MDLAnimatedVector3")
        }
    }
}
impl INSObject for MDLAnimatedVector3 {}
impl PNSObject for MDLAnimatedVector3 {}
impl IMDLAnimatedVector3 for MDLAnimatedVector3 {}
pub trait IMDLAnimatedVector3: Sized + std::ops::Deref {
    unsafe fn setFloat3_atTime_(&self, value: vector_float3, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat3 : value, atTime : time)
    }
    unsafe fn setDouble3_atTime_(&self, value: vector_double3, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDouble3 : value, atTime : time)
    }
    unsafe fn float3AtTime_(&self, time: NSTimeInterval) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, float3AtTime : time)
    }
    unsafe fn double3AtTime_(&self, time: NSTimeInterval) -> vector_double3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, double3AtTime : time)
    }
    unsafe fn resetWithFloat3Array_atTimes_count_(
        &self,
        valuesArray: *const vector_float3,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithFloat3Array : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn resetWithDouble3Array_atTimes_count_(
        &self,
        valuesArray: *const vector_double3,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithDouble3Array : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn getFloat3Array_maxCount_(
        &self,
        valuesArray: *mut vector_float3,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloat3Array : valuesArray, maxCount : maxCount)
    }
    unsafe fn getDouble3Array_maxCount_(
        &self,
        valuesArray: *mut vector_double3,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDouble3Array : valuesArray, maxCount : maxCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedVector4(pub id);
impl std::ops::Deref for MDLAnimatedVector4 {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedVector4 {}
impl MDLAnimatedVector4 {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedVector4").unwrap(), alloc) })
    }
}
impl IMDLAnimatedValue for MDLAnimatedVector4 {}
impl PNSCopying for MDLAnimatedVector4 {}
impl std::convert::TryFrom<MDLAnimatedValue> for MDLAnimatedVector4 {
    type Error = &'static str;
    fn try_from(parent: MDLAnimatedValue) -> Result<MDLAnimatedVector4, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedVector4").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedVector4(parent.0))
        } else {
            Err("This MDLAnimatedValue cannot be downcasted to MDLAnimatedVector4")
        }
    }
}
impl INSObject for MDLAnimatedVector4 {}
impl PNSObject for MDLAnimatedVector4 {}
impl IMDLAnimatedVector4 for MDLAnimatedVector4 {}
pub trait IMDLAnimatedVector4: Sized + std::ops::Deref {
    unsafe fn setFloat4_atTime_(&self, value: vector_float4, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat4 : value, atTime : time)
    }
    unsafe fn setDouble4_atTime_(&self, value: vector_double4, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDouble4 : value, atTime : time)
    }
    unsafe fn float4AtTime_(&self, time: NSTimeInterval) -> vector_float4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, float4AtTime : time)
    }
    unsafe fn double4AtTime_(&self, time: NSTimeInterval) -> vector_double4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, double4AtTime : time)
    }
    unsafe fn resetWithFloat4Array_atTimes_count_(
        &self,
        valuesArray: *const vector_float4,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithFloat4Array : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn resetWithDouble4Array_atTimes_count_(
        &self,
        valuesArray: *const vector_double4,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithDouble4Array : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn getFloat4Array_maxCount_(
        &self,
        valuesArray: *mut vector_float4,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloat4Array : valuesArray, maxCount : maxCount)
    }
    unsafe fn getDouble4Array_maxCount_(
        &self,
        valuesArray: *mut vector_double4,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDouble4Array : valuesArray, maxCount : maxCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedQuaternion(pub id);
impl std::ops::Deref for MDLAnimatedQuaternion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedQuaternion {}
impl MDLAnimatedQuaternion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedQuaternion").unwrap(), alloc) })
    }
}
impl IMDLAnimatedValue for MDLAnimatedQuaternion {}
impl PNSCopying for MDLAnimatedQuaternion {}
impl std::convert::TryFrom<MDLAnimatedValue> for MDLAnimatedQuaternion {
    type Error = &'static str;
    fn try_from(parent: MDLAnimatedValue) -> Result<MDLAnimatedQuaternion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedQuaternion").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedQuaternion(parent.0))
        } else {
            Err("This MDLAnimatedValue cannot be downcasted to MDLAnimatedQuaternion")
        }
    }
}
impl INSObject for MDLAnimatedQuaternion {}
impl PNSObject for MDLAnimatedQuaternion {}
impl IMDLAnimatedQuaternion for MDLAnimatedQuaternion {}
pub trait IMDLAnimatedQuaternion: Sized + std::ops::Deref {
    unsafe fn setDoubleQuaternion_atTime_(&self, value: simd_quatd, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoubleQuaternion : value, atTime : time)
    }
    unsafe fn doubleQuaternionAtTime_(&self, time: NSTimeInterval) -> simd_quatd
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, doubleQuaternionAtTime : time)
    }
    unsafe fn resetWithDoubleQuaternionArray_atTimes_count_(
        &self,
        valuesArray: *const simd_quatd,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithDoubleQuaternionArray : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn getDoubleQuaternionArray_maxCount_(
        &self,
        valuesArray: *mut simd_quatd,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDoubleQuaternionArray : valuesArray, maxCount : maxCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimatedMatrix4x4(pub id);
impl std::ops::Deref for MDLAnimatedMatrix4x4 {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimatedMatrix4x4 {}
impl MDLAnimatedMatrix4x4 {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimatedMatrix4x4").unwrap(), alloc) })
    }
}
impl IMDLAnimatedValue for MDLAnimatedMatrix4x4 {}
impl PNSCopying for MDLAnimatedMatrix4x4 {}
impl std::convert::TryFrom<MDLAnimatedValue> for MDLAnimatedMatrix4x4 {
    type Error = &'static str;
    fn try_from(parent: MDLAnimatedValue) -> Result<MDLAnimatedMatrix4x4, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimatedMatrix4x4").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimatedMatrix4x4(parent.0))
        } else {
            Err("This MDLAnimatedValue cannot be downcasted to MDLAnimatedMatrix4x4")
        }
    }
}
impl INSObject for MDLAnimatedMatrix4x4 {}
impl PNSObject for MDLAnimatedMatrix4x4 {}
impl IMDLAnimatedMatrix4x4 for MDLAnimatedMatrix4x4 {}
pub trait IMDLAnimatedMatrix4x4: Sized + std::ops::Deref {
    unsafe fn setFloat4x4_atTime_(&self, value: matrix_float4x4, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat4x4 : value, atTime : time)
    }
    unsafe fn setDouble4x4_atTime_(&self, value: matrix_double4x4, time: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDouble4x4 : value, atTime : time)
    }
    unsafe fn float4x4AtTime_(&self, time: NSTimeInterval) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, float4x4AtTime : time)
    }
    unsafe fn double4x4AtTime_(&self, time: NSTimeInterval) -> matrix_double4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, double4x4AtTime : time)
    }
    unsafe fn resetWithFloat4x4Array_atTimes_count_(
        &self,
        valuesArray: *const matrix_float4x4,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithFloat4x4Array : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn resetWithDouble4x4Array_atTimes_count_(
        &self,
        valuesArray: *const matrix_double4x4,
        timesArray: *const NSTimeInterval,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithDouble4x4Array : valuesArray, atTimes : timesArray, count : count)
    }
    unsafe fn getFloat4x4Array_maxCount_(
        &self,
        valuesArray: *mut matrix_float4x4,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFloat4x4Array : valuesArray, maxCount : maxCount)
    }
    unsafe fn getDouble4x4Array_maxCount_(
        &self,
        valuesArray: *mut matrix_double4x4,
        maxCount: NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDouble4x4Array : valuesArray, maxCount : maxCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLSkeleton(pub id);
impl std::ops::Deref for MDLSkeleton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLSkeleton {}
impl MDLSkeleton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLSkeleton").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLSkeleton {}
impl IMDLObject for MDLSkeleton {}
impl PMDLNamed for MDLSkeleton {}
impl From<MDLSkeleton> for MDLObject {
    fn from(child: MDLSkeleton) -> MDLObject {
        MDLObject(child.0)
    }
}
impl std::convert::TryFrom<MDLObject> for MDLSkeleton {
    type Error = &'static str;
    fn try_from(parent: MDLObject) -> Result<MDLSkeleton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLSkeleton").unwrap()) };
        if is_kind_of {
            Ok(MDLSkeleton(parent.0))
        } else {
            Err("This MDLObject cannot be downcasted to MDLSkeleton")
        }
    }
}
impl INSObject for MDLSkeleton {}
impl PNSObject for MDLSkeleton {}
impl IMDLSkeleton for MDLSkeleton {}
pub trait IMDLSkeleton: Sized + std::ops::Deref {
    unsafe fn initWithName_jointPaths_(&self, name: NSString, jointPaths: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, jointPaths : jointPaths)
    }
    unsafe fn jointPaths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jointPaths)
    }
    unsafe fn jointBindTransforms(&self) -> MDLMatrix4x4Array
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jointBindTransforms)
    }
    unsafe fn jointRestTransforms(&self) -> MDLMatrix4x4Array
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jointRestTransforms)
    }
}
pub trait PMDLJointAnimation: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLPackedJointAnimation(pub id);
impl std::ops::Deref for MDLPackedJointAnimation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLPackedJointAnimation {}
impl MDLPackedJointAnimation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLPackedJointAnimation").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLPackedJointAnimation {}
impl PMDLJointAnimation for MDLPackedJointAnimation {}
impl IMDLObject for MDLPackedJointAnimation {}
impl PMDLNamed for MDLPackedJointAnimation {}
impl std::convert::TryFrom<MDLObject> for MDLPackedJointAnimation {
    type Error = &'static str;
    fn try_from(parent: MDLObject) -> Result<MDLPackedJointAnimation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLPackedJointAnimation").unwrap()) };
        if is_kind_of {
            Ok(MDLPackedJointAnimation(parent.0))
        } else {
            Err("This MDLObject cannot be downcasted to MDLPackedJointAnimation")
        }
    }
}
impl INSObject for MDLPackedJointAnimation {}
impl PNSObject for MDLPackedJointAnimation {}
impl IMDLPackedJointAnimation for MDLPackedJointAnimation {}
pub trait IMDLPackedJointAnimation: Sized + std::ops::Deref {
    unsafe fn initWithName_jointPaths_(&self, name: NSString, jointPaths: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, jointPaths : jointPaths)
    }
    unsafe fn jointPaths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jointPaths)
    }
    unsafe fn translations(&self) -> MDLAnimatedVector3Array
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, translations)
    }
    unsafe fn rotations(&self) -> MDLAnimatedQuaternionArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotations)
    }
    unsafe fn scales(&self) -> MDLAnimatedVector3Array
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scales)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAnimationBindComponent(pub id);
impl std::ops::Deref for MDLAnimationBindComponent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAnimationBindComponent {}
impl MDLAnimationBindComponent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAnimationBindComponent").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLAnimationBindComponent {}
impl PMDLComponent for MDLAnimationBindComponent {}
impl INSObject for MDLAnimationBindComponent {}
impl PNSObject for MDLAnimationBindComponent {}
impl std::convert::TryFrom<NSObject> for MDLAnimationBindComponent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLAnimationBindComponent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAnimationBindComponent").unwrap()) };
        if is_kind_of {
            Ok(MDLAnimationBindComponent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLAnimationBindComponent")
        }
    }
}
impl IMDLAnimationBindComponent for MDLAnimationBindComponent {}
pub trait IMDLAnimationBindComponent: Sized + std::ops::Deref {
    unsafe fn skeleton(&self) -> MDLSkeleton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, skeleton)
    }
    unsafe fn setSkeleton_(&self, skeleton: MDLSkeleton)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSkeleton : skeleton)
    }
    unsafe fn jointAnimation(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jointAnimation)
    }
    unsafe fn setJointAnimation_(&self, jointAnimation: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJointAnimation : jointAnimation)
    }
    unsafe fn jointPaths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jointPaths)
    }
    unsafe fn setJointPaths_(&self, jointPaths: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJointPaths : jointPaths)
    }
    unsafe fn geometryBindTransform(&self) -> matrix_double4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometryBindTransform)
    }
    unsafe fn setGeometryBindTransform_(&self, geometryBindTransform: matrix_double4x4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeometryBindTransform : geometryBindTransform)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAsset(pub id);
impl std::ops::Deref for MDLAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAsset {}
impl MDLAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAsset").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLAsset {}
impl PNSFastEnumeration for MDLAsset {}
impl INSObject for MDLAsset {}
impl PNSObject for MDLAsset {}
impl std::convert::TryFrom<NSObject> for MDLAsset {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLAsset, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAsset").unwrap()) };
        if is_kind_of {
            Ok(MDLAsset(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLAsset")
        }
    }
}
impl IMDLAsset for MDLAsset {}
pub trait IMDLAsset: Sized + std::ops::Deref {
    unsafe fn initWithURL_(&self, URL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL)
    }
    unsafe fn initWithURL_vertexDescriptor_bufferAllocator_(
        &self,
        URL: NSURL,
        vertexDescriptor: MDLVertexDescriptor,
        bufferAllocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL, vertexDescriptor : vertexDescriptor, bufferAllocator : bufferAllocator)
    }
    unsafe fn initWithBufferAllocator_(&self, bufferAllocator: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBufferAllocator : bufferAllocator)
    }
    unsafe fn initWithURL_vertexDescriptor_bufferAllocator_preserveTopology_error_(
        &self,
        URL: NSURL,
        vertexDescriptor: MDLVertexDescriptor,
        bufferAllocator: *mut u64,
        preserveTopology: BOOL,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL, vertexDescriptor : vertexDescriptor, bufferAllocator : bufferAllocator, preserveTopology : preserveTopology, error : error)
    }
    unsafe fn exportAssetToURL_(&self, URL: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exportAssetToURL : URL)
    }
    unsafe fn exportAssetToURL_error_(&self, URL: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exportAssetToURL : URL, error : error)
    }
    unsafe fn objectAtPath_(&self, path: NSString) -> MDLObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtPath : path)
    }
    unsafe fn childObjectsOfClass_(&self, objectClass: Class) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, childObjectsOfClass : objectClass)
    }
    unsafe fn loadTextures(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loadTextures)
    }
    unsafe fn boundingBoxAtTime_(&self, time: NSTimeInterval) -> MDLAxisAlignedBoundingBox
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingBoxAtTime : time)
    }
    unsafe fn addObject_(&self, object: MDLObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObject : object)
    }
    unsafe fn removeObject_(&self, object: MDLObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeObject : object)
    }
    unsafe fn objectAtIndexedSubscript_(&self, index: NSUInteger) -> MDLObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : index)
    }
    unsafe fn objectAtIndex_(&self, index: NSUInteger) -> MDLObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndex : index)
    }
    unsafe fn boundingBox(&self) -> MDLAxisAlignedBoundingBox
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBox)
    }
    unsafe fn frameInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameInterval)
    }
    unsafe fn setFrameInterval_(&self, frameInterval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameInterval : frameInterval)
    }
    unsafe fn startTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startTime)
    }
    unsafe fn setStartTime_(&self, startTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartTime : startTime)
    }
    unsafe fn endTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endTime)
    }
    unsafe fn setEndTime_(&self, endTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndTime : endTime)
    }
    unsafe fn upAxis(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upAxis)
    }
    unsafe fn setUpAxis_(&self, upAxis: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpAxis : upAxis)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn resolver(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolver)
    }
    unsafe fn setResolver_(&self, resolver: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResolver : resolver)
    }
    unsafe fn bufferAllocator(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferAllocator)
    }
    unsafe fn vertexDescriptor(&self) -> MDLVertexDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexDescriptor)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn masters(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, masters)
    }
    unsafe fn setMasters_(&self, masters: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMasters : masters)
    }
    unsafe fn originals(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originals)
    }
    unsafe fn setOriginals_(&self, originals: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOriginals : originals)
    }
    unsafe fn animations(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animations)
    }
    unsafe fn setAnimations_(&self, animations: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimations : animations)
    }
    unsafe fn canImportFileExtension_(extension: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAsset").unwrap(), canImportFileExtension : extension)
    }
    unsafe fn canExportFileExtension_(extension: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAsset").unwrap(), canExportFileExtension : extension)
    }
}
pub trait PMDLLightProbeIrradianceDataSource: Sized + std::ops::Deref {
    unsafe fn sphericalHarmonicsCoefficientsAtPosition_(&self, position: vector_float3) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sphericalHarmonicsCoefficientsAtPosition : position)
    }
    unsafe fn boundingBox(&self) -> MDLAxisAlignedBoundingBox
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBox)
    }
    unsafe fn setBoundingBox_(&self, boundingBox: MDLAxisAlignedBoundingBox)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundingBox : boundingBox)
    }
    unsafe fn sphericalHarmonicsLevel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sphericalHarmonicsLevel)
    }
    unsafe fn setSphericalHarmonicsLevel_(&self, sphericalHarmonicsLevel: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSphericalHarmonicsLevel : sphericalHarmonicsLevel)
    }
}
impl MDLAsset_MDLLightBaking for MDLAsset {}
pub trait MDLAsset_MDLLightBaking: Sized + std::ops::Deref {
    unsafe fn placeLightProbesWithDensity_heuristic_usingIrradianceDataSource_(
        value: f32,
        type_: MDLProbePlacement,
        dataSource: *mut u64,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAsset").unwrap(), placeLightProbesWithDensity : value, heuristic : type_, usingIrradianceDataSource : dataSource)
    }
}
pub type MDLCameraProjection = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLCamera(pub id);
impl std::ops::Deref for MDLCamera {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLCamera {}
impl MDLCamera {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLCamera").unwrap(), alloc) })
    }
}
impl IMDLObject for MDLCamera {}
impl PMDLNamed for MDLCamera {}
impl std::convert::TryFrom<MDLObject> for MDLCamera {
    type Error = &'static str;
    fn try_from(parent: MDLObject) -> Result<MDLCamera, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLCamera").unwrap()) };
        if is_kind_of {
            Ok(MDLCamera(parent.0))
        } else {
            Err("This MDLObject cannot be downcasted to MDLCamera")
        }
    }
}
impl INSObject for MDLCamera {}
impl PNSObject for MDLCamera {}
impl IMDLCamera for MDLCamera {}
pub trait IMDLCamera: Sized + std::ops::Deref {
    unsafe fn frameBoundingBox_setNearAndFar_(
        &self,
        boundingBox: MDLAxisAlignedBoundingBox,
        setNearAndFar: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, frameBoundingBox : boundingBox, setNearAndFar : setNearAndFar)
    }
    unsafe fn lookAt_(&self, focusPosition: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAt : focusPosition)
    }
    unsafe fn lookAt_from_(&self, focusPosition: vector_float3, cameraPosition: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAt : focusPosition, from : cameraPosition)
    }
    unsafe fn rayTo_forViewPort_(&self, pixel: vector_int2, size: vector_int2) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rayTo : pixel, forViewPort : size)
    }
    unsafe fn bokehKernelWithSize_(&self, size: vector_int2) -> MDLTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bokehKernelWithSize : size)
    }
    unsafe fn projectionMatrix(&self) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projectionMatrix)
    }
    unsafe fn projection(&self) -> MDLCameraProjection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projection)
    }
    unsafe fn setProjection_(&self, projection: MDLCameraProjection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProjection : projection)
    }
    unsafe fn nearVisibilityDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nearVisibilityDistance)
    }
    unsafe fn setNearVisibilityDistance_(&self, nearVisibilityDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNearVisibilityDistance : nearVisibilityDistance)
    }
    unsafe fn farVisibilityDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, farVisibilityDistance)
    }
    unsafe fn setFarVisibilityDistance_(&self, farVisibilityDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFarVisibilityDistance : farVisibilityDistance)
    }
    unsafe fn worldToMetersConversionScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, worldToMetersConversionScale)
    }
    unsafe fn setWorldToMetersConversionScale_(&self, worldToMetersConversionScale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorldToMetersConversionScale : worldToMetersConversionScale)
    }
    unsafe fn barrelDistortion(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, barrelDistortion)
    }
    unsafe fn setBarrelDistortion_(&self, barrelDistortion: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBarrelDistortion : barrelDistortion)
    }
    unsafe fn fisheyeDistortion(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fisheyeDistortion)
    }
    unsafe fn setFisheyeDistortion_(&self, fisheyeDistortion: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFisheyeDistortion : fisheyeDistortion)
    }
    unsafe fn opticalVignetting(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opticalVignetting)
    }
    unsafe fn setOpticalVignetting_(&self, opticalVignetting: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpticalVignetting : opticalVignetting)
    }
    unsafe fn chromaticAberration(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chromaticAberration)
    }
    unsafe fn setChromaticAberration_(&self, chromaticAberration: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChromaticAberration : chromaticAberration)
    }
    unsafe fn focalLength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalLength)
    }
    unsafe fn setFocalLength_(&self, focalLength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalLength : focalLength)
    }
    unsafe fn focusDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusDistance)
    }
    unsafe fn setFocusDistance_(&self, focusDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusDistance : focusDistance)
    }
    unsafe fn fieldOfView(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldOfView)
    }
    unsafe fn setFieldOfView_(&self, fieldOfView: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldOfView : fieldOfView)
    }
    unsafe fn fStop(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fStop)
    }
    unsafe fn setFStop_(&self, fStop: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFStop : fStop)
    }
    unsafe fn apertureBladeCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, apertureBladeCount)
    }
    unsafe fn setApertureBladeCount_(&self, apertureBladeCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApertureBladeCount : apertureBladeCount)
    }
    unsafe fn maximumCircleOfConfusion(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumCircleOfConfusion)
    }
    unsafe fn setMaximumCircleOfConfusion_(&self, maximumCircleOfConfusion: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumCircleOfConfusion : maximumCircleOfConfusion)
    }
    unsafe fn shutterOpenInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shutterOpenInterval)
    }
    unsafe fn setShutterOpenInterval_(&self, shutterOpenInterval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShutterOpenInterval : shutterOpenInterval)
    }
    unsafe fn sensorVerticalAperture(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensorVerticalAperture)
    }
    unsafe fn setSensorVerticalAperture_(&self, sensorVerticalAperture: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSensorVerticalAperture : sensorVerticalAperture)
    }
    unsafe fn sensorAspect(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensorAspect)
    }
    unsafe fn setSensorAspect_(&self, sensorAspect: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSensorAspect : sensorAspect)
    }
    unsafe fn sensorEnlargement(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensorEnlargement)
    }
    unsafe fn setSensorEnlargement_(&self, sensorEnlargement: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSensorEnlargement : sensorEnlargement)
    }
    unsafe fn sensorShift(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensorShift)
    }
    unsafe fn setSensorShift_(&self, sensorShift: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSensorShift : sensorShift)
    }
    unsafe fn flash(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flash)
    }
    unsafe fn setFlash_(&self, flash: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlash : flash)
    }
    unsafe fn exposureCompression(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureCompression)
    }
    unsafe fn setExposureCompression_(&self, exposureCompression: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureCompression : exposureCompression)
    }
    unsafe fn exposure(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposure)
    }
    unsafe fn setExposure_(&self, exposure: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposure : exposure)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLStereoscopicCamera(pub id);
impl std::ops::Deref for MDLStereoscopicCamera {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLStereoscopicCamera {}
impl MDLStereoscopicCamera {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLStereoscopicCamera").unwrap(), alloc) })
    }
}
impl IMDLCamera for MDLStereoscopicCamera {}
impl From<MDLStereoscopicCamera> for MDLCamera {
    fn from(child: MDLStereoscopicCamera) -> MDLCamera {
        MDLCamera(child.0)
    }
}
impl std::convert::TryFrom<MDLCamera> for MDLStereoscopicCamera {
    type Error = &'static str;
    fn try_from(parent: MDLCamera) -> Result<MDLStereoscopicCamera, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLStereoscopicCamera").unwrap()) };
        if is_kind_of {
            Ok(MDLStereoscopicCamera(parent.0))
        } else {
            Err("This MDLCamera cannot be downcasted to MDLStereoscopicCamera")
        }
    }
}
impl IMDLObject for MDLStereoscopicCamera {}
impl PMDLNamed for MDLStereoscopicCamera {}
impl INSObject for MDLStereoscopicCamera {}
impl PNSObject for MDLStereoscopicCamera {}
impl IMDLStereoscopicCamera for MDLStereoscopicCamera {}
pub trait IMDLStereoscopicCamera: Sized + std::ops::Deref {
    unsafe fn interPupillaryDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interPupillaryDistance)
    }
    unsafe fn setInterPupillaryDistance_(&self, interPupillaryDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterPupillaryDistance : interPupillaryDistance)
    }
    unsafe fn leftVergence(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftVergence)
    }
    unsafe fn setLeftVergence_(&self, leftVergence: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeftVergence : leftVergence)
    }
    unsafe fn rightVergence(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightVergence)
    }
    unsafe fn setRightVergence_(&self, rightVergence: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRightVergence : rightVergence)
    }
    unsafe fn overlap(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overlap)
    }
    unsafe fn setOverlap_(&self, overlap: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverlap : overlap)
    }
    unsafe fn leftViewMatrix(&self) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftViewMatrix)
    }
    unsafe fn rightViewMatrix(&self) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightViewMatrix)
    }
    unsafe fn leftProjectionMatrix(&self) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftProjectionMatrix)
    }
    unsafe fn rightProjectionMatrix(&self) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightProjectionMatrix)
    }
}
pub type MDLLightType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLLight(pub id);
impl std::ops::Deref for MDLLight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLLight {}
impl MDLLight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLLight").unwrap(), alloc) })
    }
}
impl IMDLObject for MDLLight {}
impl PMDLNamed for MDLLight {}
impl std::convert::TryFrom<MDLObject> for MDLLight {
    type Error = &'static str;
    fn try_from(parent: MDLObject) -> Result<MDLLight, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLLight").unwrap()) };
        if is_kind_of {
            Ok(MDLLight(parent.0))
        } else {
            Err("This MDLObject cannot be downcasted to MDLLight")
        }
    }
}
impl INSObject for MDLLight {}
impl PNSObject for MDLLight {}
impl IMDLLight for MDLLight {}
pub trait IMDLLight: Sized + std::ops::Deref {
    unsafe fn irradianceAtPoint_(&self, point: vector_float3) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, irradianceAtPoint : point)
    }
    unsafe fn irradianceAtPoint_colorSpace_(
        &self,
        point: vector_float3,
        colorSpace: CGColorSpaceRef,
    ) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, irradianceAtPoint : point, colorSpace : colorSpace)
    }
    unsafe fn lightType(&self) -> MDLLightType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightType)
    }
    unsafe fn setLightType_(&self, lightType: MDLLightType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightType : lightType)
    }
    unsafe fn colorSpace(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLPhysicallyPlausibleLight(pub id);
impl std::ops::Deref for MDLPhysicallyPlausibleLight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLPhysicallyPlausibleLight {}
impl MDLPhysicallyPlausibleLight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLPhysicallyPlausibleLight").unwrap(), alloc) })
    }
}
impl IMDLLight for MDLPhysicallyPlausibleLight {}
impl From<MDLPhysicallyPlausibleLight> for MDLLight {
    fn from(child: MDLPhysicallyPlausibleLight) -> MDLLight {
        MDLLight(child.0)
    }
}
impl std::convert::TryFrom<MDLLight> for MDLPhysicallyPlausibleLight {
    type Error = &'static str;
    fn try_from(parent: MDLLight) -> Result<MDLPhysicallyPlausibleLight, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLPhysicallyPlausibleLight").unwrap()) };
        if is_kind_of {
            Ok(MDLPhysicallyPlausibleLight(parent.0))
        } else {
            Err("This MDLLight cannot be downcasted to MDLPhysicallyPlausibleLight")
        }
    }
}
impl IMDLObject for MDLPhysicallyPlausibleLight {}
impl PMDLNamed for MDLPhysicallyPlausibleLight {}
impl INSObject for MDLPhysicallyPlausibleLight {}
impl PNSObject for MDLPhysicallyPlausibleLight {}
impl IMDLPhysicallyPlausibleLight for MDLPhysicallyPlausibleLight {}
pub trait IMDLPhysicallyPlausibleLight: Sized + std::ops::Deref {
    unsafe fn setColorByTemperature_(&self, temperature: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorByTemperature : temperature)
    }
    unsafe fn color(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn lumens(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lumens)
    }
    unsafe fn setLumens_(&self, lumens: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLumens : lumens)
    }
    unsafe fn innerConeAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, innerConeAngle)
    }
    unsafe fn setInnerConeAngle_(&self, innerConeAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInnerConeAngle : innerConeAngle)
    }
    unsafe fn outerConeAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerConeAngle)
    }
    unsafe fn setOuterConeAngle_(&self, outerConeAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOuterConeAngle : outerConeAngle)
    }
    unsafe fn attenuationStartDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationStartDistance)
    }
    unsafe fn setAttenuationStartDistance_(&self, attenuationStartDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttenuationStartDistance : attenuationStartDistance)
    }
    unsafe fn attenuationEndDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attenuationEndDistance)
    }
    unsafe fn setAttenuationEndDistance_(&self, attenuationEndDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttenuationEndDistance : attenuationEndDistance)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLAreaLight(pub id);
impl std::ops::Deref for MDLAreaLight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLAreaLight {}
impl MDLAreaLight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLAreaLight").unwrap(), alloc) })
    }
}
impl IMDLPhysicallyPlausibleLight for MDLAreaLight {}
impl From<MDLAreaLight> for MDLPhysicallyPlausibleLight {
    fn from(child: MDLAreaLight) -> MDLPhysicallyPlausibleLight {
        MDLPhysicallyPlausibleLight(child.0)
    }
}
impl std::convert::TryFrom<MDLPhysicallyPlausibleLight> for MDLAreaLight {
    type Error = &'static str;
    fn try_from(parent: MDLPhysicallyPlausibleLight) -> Result<MDLAreaLight, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLAreaLight").unwrap()) };
        if is_kind_of {
            Ok(MDLAreaLight(parent.0))
        } else {
            Err("This MDLPhysicallyPlausibleLight cannot be downcasted to MDLAreaLight")
        }
    }
}
impl IMDLLight for MDLAreaLight {}
impl IMDLObject for MDLAreaLight {}
impl PMDLNamed for MDLAreaLight {}
impl INSObject for MDLAreaLight {}
impl PNSObject for MDLAreaLight {}
impl IMDLAreaLight for MDLAreaLight {}
pub trait IMDLAreaLight: Sized + std::ops::Deref {
    unsafe fn areaRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areaRadius)
    }
    unsafe fn setAreaRadius_(&self, areaRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAreaRadius : areaRadius)
    }
    unsafe fn superEllipticPower(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, superEllipticPower)
    }
    unsafe fn setSuperEllipticPower_(&self, superEllipticPower: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuperEllipticPower : superEllipticPower)
    }
    unsafe fn aspect(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspect)
    }
    unsafe fn setAspect_(&self, aspect: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAspect : aspect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLPhotometricLight(pub id);
impl std::ops::Deref for MDLPhotometricLight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLPhotometricLight {}
impl MDLPhotometricLight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLPhotometricLight").unwrap(), alloc) })
    }
}
impl IMDLPhysicallyPlausibleLight for MDLPhotometricLight {}
impl std::convert::TryFrom<MDLPhysicallyPlausibleLight> for MDLPhotometricLight {
    type Error = &'static str;
    fn try_from(parent: MDLPhysicallyPlausibleLight) -> Result<MDLPhotometricLight, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLPhotometricLight").unwrap()) };
        if is_kind_of {
            Ok(MDLPhotometricLight(parent.0))
        } else {
            Err("This MDLPhysicallyPlausibleLight cannot be downcasted to MDLPhotometricLight")
        }
    }
}
impl IMDLLight for MDLPhotometricLight {}
impl IMDLObject for MDLPhotometricLight {}
impl PMDLNamed for MDLPhotometricLight {}
impl INSObject for MDLPhotometricLight {}
impl PNSObject for MDLPhotometricLight {}
impl IMDLPhotometricLight for MDLPhotometricLight {}
pub trait IMDLPhotometricLight: Sized + std::ops::Deref {
    unsafe fn initWithIESProfile_(&self, URL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIESProfile : URL)
    }
    unsafe fn generateSphericalHarmonicsFromLight_(&self, sphericalHarmonicsLevel: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateSphericalHarmonicsFromLight : sphericalHarmonicsLevel)
    }
    unsafe fn generateCubemapFromLight_(&self, textureSize: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateCubemapFromLight : textureSize)
    }
    unsafe fn generateTexture_(&self, textureSize: NSUInteger) -> MDLTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateTexture : textureSize)
    }
    unsafe fn lightCubeMap(&self) -> MDLTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightCubeMap)
    }
    unsafe fn sphericalHarmonicsLevel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sphericalHarmonicsLevel)
    }
    unsafe fn sphericalHarmonicsCoefficients(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sphericalHarmonicsCoefficients)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLLightProbe(pub id);
impl std::ops::Deref for MDLLightProbe {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLLightProbe {}
impl MDLLightProbe {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLLightProbe").unwrap(), alloc) })
    }
}
impl IMDLLight for MDLLightProbe {}
impl std::convert::TryFrom<MDLLight> for MDLLightProbe {
    type Error = &'static str;
    fn try_from(parent: MDLLight) -> Result<MDLLightProbe, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLLightProbe").unwrap()) };
        if is_kind_of {
            Ok(MDLLightProbe(parent.0))
        } else {
            Err("This MDLLight cannot be downcasted to MDLLightProbe")
        }
    }
}
impl IMDLObject for MDLLightProbe {}
impl PMDLNamed for MDLLightProbe {}
impl INSObject for MDLLightProbe {}
impl PNSObject for MDLLightProbe {}
impl IMDLLightProbe for MDLLightProbe {}
pub trait IMDLLightProbe: Sized + std::ops::Deref {
    unsafe fn initWithReflectiveTexture_irradianceTexture_(
        &self,
        reflectiveTexture: MDLTexture,
        irradianceTexture: MDLTexture,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithReflectiveTexture : reflectiveTexture, irradianceTexture : irradianceTexture)
    }
    unsafe fn generateSphericalHarmonicsFromIrradiance_(&self, sphericalHarmonicsLevel: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateSphericalHarmonicsFromIrradiance : sphericalHarmonicsLevel)
    }
    unsafe fn reflectiveTexture(&self) -> MDLTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reflectiveTexture)
    }
    unsafe fn irradianceTexture(&self) -> MDLTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, irradianceTexture)
    }
    unsafe fn sphericalHarmonicsLevel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sphericalHarmonicsLevel)
    }
    unsafe fn sphericalHarmonicsCoefficients(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sphericalHarmonicsCoefficients)
    }
}
impl MDLLightProbe_MDLLightBaking for MDLLightProbe {}
pub trait MDLLightProbe_MDLLightBaking: Sized + std::ops::Deref {
    unsafe fn lightProbeWithTextureSize_forLocation_lightsToConsider_objectsToConsider_reflectiveCubemap_irradianceCubemap_(
        textureSize: NSInteger,
        transform: MDLTransform,
        lightsToConsider: NSArray,
        objectsToConsider: NSArray,
        reflectiveCubemap: MDLTexture,
        irradianceCubemap: MDLTexture,
    ) -> MDLLightProbe
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLLightProbe").unwrap(), lightProbeWithTextureSize : textureSize, forLocation : transform, lightsToConsider : lightsToConsider, objectsToConsider : objectsToConsider, reflectiveCubemap : reflectiveCubemap, irradianceCubemap : irradianceCubemap)
    }
}
pub type MDLMaterialSemantic = NSUInteger;
pub type MDLMaterialPropertyType = NSUInteger;
pub type MDLMaterialTextureWrapMode = NSUInteger;
pub type MDLMaterialTextureFilterMode = NSUInteger;
pub type MDLMaterialMipMapFilterMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTextureFilter(pub id);
impl std::ops::Deref for MDLTextureFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTextureFilter {}
impl MDLTextureFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTextureFilter").unwrap(), alloc) })
    }
}
impl INSObject for MDLTextureFilter {}
impl PNSObject for MDLTextureFilter {}
impl std::convert::TryFrom<NSObject> for MDLTextureFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTextureFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTextureFilter").unwrap()) };
        if is_kind_of {
            Ok(MDLTextureFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTextureFilter")
        }
    }
}
impl IMDLTextureFilter for MDLTextureFilter {}
pub trait IMDLTextureFilter: Sized + std::ops::Deref {
    unsafe fn sWrapMode(&self) -> MDLMaterialTextureWrapMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sWrapMode)
    }
    unsafe fn setSWrapMode_(&self, sWrapMode: MDLMaterialTextureWrapMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSWrapMode : sWrapMode)
    }
    unsafe fn tWrapMode(&self) -> MDLMaterialTextureWrapMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tWrapMode)
    }
    unsafe fn setTWrapMode_(&self, tWrapMode: MDLMaterialTextureWrapMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTWrapMode : tWrapMode)
    }
    unsafe fn rWrapMode(&self) -> MDLMaterialTextureWrapMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rWrapMode)
    }
    unsafe fn setRWrapMode_(&self, rWrapMode: MDLMaterialTextureWrapMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRWrapMode : rWrapMode)
    }
    unsafe fn minFilter(&self) -> MDLMaterialTextureFilterMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minFilter)
    }
    unsafe fn setMinFilter_(&self, minFilter: MDLMaterialTextureFilterMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinFilter : minFilter)
    }
    unsafe fn magFilter(&self) -> MDLMaterialTextureFilterMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magFilter)
    }
    unsafe fn setMagFilter_(&self, magFilter: MDLMaterialTextureFilterMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagFilter : magFilter)
    }
    unsafe fn mipFilter(&self) -> MDLMaterialMipMapFilterMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mipFilter)
    }
    unsafe fn setMipFilter_(&self, mipFilter: MDLMaterialMipMapFilterMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMipFilter : mipFilter)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTextureSampler(pub id);
impl std::ops::Deref for MDLTextureSampler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTextureSampler {}
impl MDLTextureSampler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTextureSampler").unwrap(), alloc) })
    }
}
impl INSObject for MDLTextureSampler {}
impl PNSObject for MDLTextureSampler {}
impl std::convert::TryFrom<NSObject> for MDLTextureSampler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTextureSampler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTextureSampler").unwrap()) };
        if is_kind_of {
            Ok(MDLTextureSampler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTextureSampler")
        }
    }
}
impl IMDLTextureSampler for MDLTextureSampler {}
pub trait IMDLTextureSampler: Sized + std::ops::Deref {
    unsafe fn texture(&self) -> MDLTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texture)
    }
    unsafe fn setTexture_(&self, texture: MDLTexture)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTexture : texture)
    }
    unsafe fn hardwareFilter(&self) -> MDLTextureFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hardwareFilter)
    }
    unsafe fn setHardwareFilter_(&self, hardwareFilter: MDLTextureFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHardwareFilter : hardwareFilter)
    }
    unsafe fn transform(&self) -> MDLTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn setTransform_(&self, transform: MDLTransform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransform : transform)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMaterialProperty(pub id);
impl std::ops::Deref for MDLMaterialProperty {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMaterialProperty {}
impl MDLMaterialProperty {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMaterialProperty").unwrap(), alloc) })
    }
}
impl PMDLNamed for MDLMaterialProperty {}
impl PNSCopying for MDLMaterialProperty {}
impl INSObject for MDLMaterialProperty {}
impl PNSObject for MDLMaterialProperty {}
impl std::convert::TryFrom<NSObject> for MDLMaterialProperty {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLMaterialProperty, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMaterialProperty").unwrap()) };
        if is_kind_of {
            Ok(MDLMaterialProperty(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLMaterialProperty")
        }
    }
}
impl IMDLMaterialProperty for MDLMaterialProperty {}
pub trait IMDLMaterialProperty: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithName_semantic_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic)
    }
    unsafe fn initWithName_semantic_float_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
        value: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic, float : value)
    }
    unsafe fn initWithName_semantic_float2_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
        value: vector_float2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic, float2 : value)
    }
    unsafe fn initWithName_semantic_float3_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
        value: vector_float3,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic, float3 : value)
    }
    unsafe fn initWithName_semantic_float4_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
        value: vector_float4,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic, float4 : value)
    }
    unsafe fn initWithName_semantic_matrix4x4_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
        value: matrix_float4x4,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic, matrix4x4 : value)
    }
    unsafe fn initWithName_semantic_URL_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
        URL: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic, URL : URL)
    }
    unsafe fn initWithName_semantic_string_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
        string: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic, string : string)
    }
    unsafe fn initWithName_semantic_textureSampler_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
        textureSampler: MDLTextureSampler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic, textureSampler : textureSampler)
    }
    unsafe fn initWithName_semantic_color_(
        &self,
        name: NSString,
        semantic: MDLMaterialSemantic,
        color: CGColorRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, semantic : semantic, color : color)
    }
    unsafe fn setProperties_(&self, property: MDLMaterialProperty)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : property)
    }
    unsafe fn semantic(&self) -> MDLMaterialSemantic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semantic)
    }
    unsafe fn setSemantic_(&self, semantic: MDLMaterialSemantic)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSemantic : semantic)
    }
    unsafe fn type_(&self) -> MDLMaterialPropertyType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: MDLMaterialPropertyType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
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
    unsafe fn stringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringValue)
    }
    unsafe fn setStringValue_(&self, stringValue: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStringValue : stringValue)
    }
    unsafe fn URLValue(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URLValue)
    }
    unsafe fn setURLValue_(&self, URLValue: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURLValue : URLValue)
    }
    unsafe fn textureSamplerValue(&self) -> MDLTextureSampler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureSamplerValue)
    }
    unsafe fn setTextureSamplerValue_(&self, textureSamplerValue: MDLTextureSampler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureSamplerValue : textureSamplerValue)
    }
    unsafe fn color(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn floatValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatValue)
    }
    unsafe fn setFloatValue_(&self, floatValue: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatValue : floatValue)
    }
    unsafe fn float2Value(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, float2Value)
    }
    unsafe fn setFloat2Value_(&self, float2Value: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat2Value : float2Value)
    }
    unsafe fn float3Value(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, float3Value)
    }
    unsafe fn setFloat3Value_(&self, float3Value: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat3Value : float3Value)
    }
    unsafe fn float4Value(&self) -> vector_float4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, float4Value)
    }
    unsafe fn setFloat4Value_(&self, float4Value: vector_float4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloat4Value : float4Value)
    }
    unsafe fn matrix4x4(&self) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matrix4x4)
    }
    unsafe fn setMatrix4x4_(&self, matrix4x4: matrix_float4x4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatrix4x4 : matrix4x4)
    }
    unsafe fn luminance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, luminance)
    }
    unsafe fn setLuminance_(&self, luminance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLuminance : luminance)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMaterialPropertyConnection(pub id);
impl std::ops::Deref for MDLMaterialPropertyConnection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMaterialPropertyConnection {}
impl MDLMaterialPropertyConnection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMaterialPropertyConnection").unwrap(), alloc) })
    }
}
impl PMDLNamed for MDLMaterialPropertyConnection {}
impl INSObject for MDLMaterialPropertyConnection {}
impl PNSObject for MDLMaterialPropertyConnection {}
impl std::convert::TryFrom<NSObject> for MDLMaterialPropertyConnection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLMaterialPropertyConnection, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMaterialPropertyConnection").unwrap())
        };
        if is_kind_of {
            Ok(MDLMaterialPropertyConnection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLMaterialPropertyConnection")
        }
    }
}
impl IMDLMaterialPropertyConnection for MDLMaterialPropertyConnection {}
pub trait IMDLMaterialPropertyConnection: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithOutput_input_(
        &self,
        output: MDLMaterialProperty,
        input: MDLMaterialProperty,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOutput : output, input : input)
    }
    unsafe fn output(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, output)
    }
    unsafe fn input(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, input)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMaterialPropertyNode(pub id);
impl std::ops::Deref for MDLMaterialPropertyNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMaterialPropertyNode {}
impl MDLMaterialPropertyNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMaterialPropertyNode").unwrap(), alloc) })
    }
}
impl PMDLNamed for MDLMaterialPropertyNode {}
impl INSObject for MDLMaterialPropertyNode {}
impl PNSObject for MDLMaterialPropertyNode {}
impl std::convert::TryFrom<NSObject> for MDLMaterialPropertyNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLMaterialPropertyNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMaterialPropertyNode").unwrap()) };
        if is_kind_of {
            Ok(MDLMaterialPropertyNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLMaterialPropertyNode")
        }
    }
}
impl IMDLMaterialPropertyNode for MDLMaterialPropertyNode {}
pub trait IMDLMaterialPropertyNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithInputs_outputs_evaluationFunction_(
        &self,
        inputs: NSArray,
        outputs: NSArray,
        function: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInputs : inputs, outputs : outputs, evaluationFunction : function)
    }
    unsafe fn evaluationFunction(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, evaluationFunction)
    }
    unsafe fn setEvaluationFunction_(&self, evaluationFunction: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEvaluationFunction : evaluationFunction)
    }
    unsafe fn inputs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputs)
    }
    unsafe fn outputs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputs)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMaterialPropertyGraph(pub id);
impl std::ops::Deref for MDLMaterialPropertyGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMaterialPropertyGraph {}
impl MDLMaterialPropertyGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMaterialPropertyGraph").unwrap(), alloc) })
    }
}
impl IMDLMaterialPropertyNode for MDLMaterialPropertyGraph {}
impl PMDLNamed for MDLMaterialPropertyGraph {}
impl From<MDLMaterialPropertyGraph> for MDLMaterialPropertyNode {
    fn from(child: MDLMaterialPropertyGraph) -> MDLMaterialPropertyNode {
        MDLMaterialPropertyNode(child.0)
    }
}
impl std::convert::TryFrom<MDLMaterialPropertyNode> for MDLMaterialPropertyGraph {
    type Error = &'static str;
    fn try_from(parent: MDLMaterialPropertyNode) -> Result<MDLMaterialPropertyGraph, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMaterialPropertyGraph").unwrap()) };
        if is_kind_of {
            Ok(MDLMaterialPropertyGraph(parent.0))
        } else {
            Err("This MDLMaterialPropertyNode cannot be downcasted to MDLMaterialPropertyGraph")
        }
    }
}
impl INSObject for MDLMaterialPropertyGraph {}
impl PNSObject for MDLMaterialPropertyGraph {}
impl IMDLMaterialPropertyGraph for MDLMaterialPropertyGraph {}
pub trait IMDLMaterialPropertyGraph: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithNodes_connections_(
        &self,
        nodes: NSArray,
        connections: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNodes : nodes, connections : connections)
    }
    unsafe fn evaluate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, evaluate)
    }
    unsafe fn nodes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodes)
    }
    unsafe fn connections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connections)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLScatteringFunction(pub id);
impl std::ops::Deref for MDLScatteringFunction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLScatteringFunction {}
impl MDLScatteringFunction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLScatteringFunction").unwrap(), alloc) })
    }
}
impl PMDLNamed for MDLScatteringFunction {}
impl INSObject for MDLScatteringFunction {}
impl PNSObject for MDLScatteringFunction {}
impl std::convert::TryFrom<NSObject> for MDLScatteringFunction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLScatteringFunction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLScatteringFunction").unwrap()) };
        if is_kind_of {
            Ok(MDLScatteringFunction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLScatteringFunction")
        }
    }
}
impl IMDLScatteringFunction for MDLScatteringFunction {}
pub trait IMDLScatteringFunction: Sized + std::ops::Deref {
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
    unsafe fn baseColor(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseColor)
    }
    unsafe fn emission(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emission)
    }
    unsafe fn specular(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specular)
    }
    unsafe fn materialIndexOfRefraction(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, materialIndexOfRefraction)
    }
    unsafe fn interfaceIndexOfRefraction(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceIndexOfRefraction)
    }
    unsafe fn normal(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normal)
    }
    unsafe fn ambientOcclusion(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ambientOcclusion)
    }
    unsafe fn ambientOcclusionScale(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ambientOcclusionScale)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLPhysicallyPlausibleScatteringFunction(pub id);
impl std::ops::Deref for MDLPhysicallyPlausibleScatteringFunction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLPhysicallyPlausibleScatteringFunction {}
impl MDLPhysicallyPlausibleScatteringFunction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLPhysicallyPlausibleScatteringFunction").unwrap(), alloc) })
    }
}
impl IMDLScatteringFunction for MDLPhysicallyPlausibleScatteringFunction {}
impl PMDLNamed for MDLPhysicallyPlausibleScatteringFunction {}
impl From<MDLPhysicallyPlausibleScatteringFunction> for MDLScatteringFunction {
    fn from(child: MDLPhysicallyPlausibleScatteringFunction) -> MDLScatteringFunction {
        MDLScatteringFunction(child.0)
    }
}
impl std::convert::TryFrom<MDLScatteringFunction> for MDLPhysicallyPlausibleScatteringFunction {
    type Error = &'static str;
    fn try_from(
        parent: MDLScatteringFunction,
    ) -> Result<MDLPhysicallyPlausibleScatteringFunction, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLPhysicallyPlausibleScatteringFunction").unwrap())
        };
        if is_kind_of {
            Ok(MDLPhysicallyPlausibleScatteringFunction(parent.0))
        } else {
            Err ("This MDLScatteringFunction cannot be downcasted to MDLPhysicallyPlausibleScatteringFunction" ,)
        }
    }
}
impl INSObject for MDLPhysicallyPlausibleScatteringFunction {}
impl PNSObject for MDLPhysicallyPlausibleScatteringFunction {}
impl IMDLPhysicallyPlausibleScatteringFunction for MDLPhysicallyPlausibleScatteringFunction {}
pub trait IMDLPhysicallyPlausibleScatteringFunction: Sized + std::ops::Deref {
    unsafe fn version(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn subsurface(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subsurface)
    }
    unsafe fn metallic(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metallic)
    }
    unsafe fn specularAmount(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularAmount)
    }
    unsafe fn specularTint(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularTint)
    }
    unsafe fn roughness(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roughness)
    }
    unsafe fn anisotropic(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anisotropic)
    }
    unsafe fn anisotropicRotation(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anisotropicRotation)
    }
    unsafe fn sheen(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sheen)
    }
    unsafe fn sheenTint(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sheenTint)
    }
    unsafe fn clearcoat(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearcoat)
    }
    unsafe fn clearcoatGloss(&self) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearcoatGloss)
    }
}
pub type MDLMaterialFace = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMaterial(pub id);
impl std::ops::Deref for MDLMaterial {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMaterial {}
impl MDLMaterial {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMaterial").unwrap(), alloc) })
    }
}
impl PMDLNamed for MDLMaterial {}
impl PNSFastEnumeration for MDLMaterial {}
impl INSObject for MDLMaterial {}
impl PNSObject for MDLMaterial {}
impl std::convert::TryFrom<NSObject> for MDLMaterial {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLMaterial, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMaterial").unwrap()) };
        if is_kind_of {
            Ok(MDLMaterial(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLMaterial")
        }
    }
}
impl IMDLMaterial for MDLMaterial {}
pub trait IMDLMaterial: Sized + std::ops::Deref {
    unsafe fn initWithName_scatteringFunction_(
        &self,
        name: NSString,
        scatteringFunction: MDLScatteringFunction,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, scatteringFunction : scatteringFunction)
    }
    unsafe fn setProperty_(&self, property: MDLMaterialProperty)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperty : property)
    }
    unsafe fn removeProperty_(&self, property: MDLMaterialProperty)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeProperty : property)
    }
    unsafe fn propertyNamed_(&self, name: NSString) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, propertyNamed : name)
    }
    unsafe fn propertyWithSemantic_(&self, semantic: MDLMaterialSemantic) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, propertyWithSemantic : semantic)
    }
    unsafe fn propertiesWithSemantic_(&self, semantic: MDLMaterialSemantic) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, propertiesWithSemantic : semantic)
    }
    unsafe fn removeAllProperties(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllProperties)
    }
    unsafe fn resolveTexturesWithResolver_(&self, resolver: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveTexturesWithResolver : resolver)
    }
    unsafe fn loadTexturesUsingResolver_(&self, resolver: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadTexturesUsingResolver : resolver)
    }
    unsafe fn objectAtIndexedSubscript_(&self, idx: NSUInteger) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : idx)
    }
    unsafe fn objectForKeyedSubscript_(&self, name: NSString) -> MDLMaterialProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : name)
    }
    unsafe fn scatteringFunction(&self) -> MDLScatteringFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scatteringFunction)
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
    unsafe fn baseMaterial(&self) -> MDLMaterial
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseMaterial)
    }
    unsafe fn setBaseMaterial_(&self, baseMaterial: MDLMaterial)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaseMaterial : baseMaterial)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn materialFace(&self) -> MDLMaterialFace
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, materialFace)
    }
    unsafe fn setMaterialFace_(&self, materialFace: MDLMaterialFace)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaterialFace : materialFace)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLSubmeshTopology(pub id);
impl std::ops::Deref for MDLSubmeshTopology {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLSubmeshTopology {}
impl MDLSubmeshTopology {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLSubmeshTopology").unwrap(), alloc) })
    }
}
impl INSObject for MDLSubmeshTopology {}
impl PNSObject for MDLSubmeshTopology {}
impl std::convert::TryFrom<NSObject> for MDLSubmeshTopology {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLSubmeshTopology, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLSubmeshTopology").unwrap()) };
        if is_kind_of {
            Ok(MDLSubmeshTopology(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLSubmeshTopology")
        }
    }
}
impl IMDLSubmeshTopology for MDLSubmeshTopology {}
pub trait IMDLSubmeshTopology: Sized + std::ops::Deref {
    unsafe fn initWithSubmesh_(&self, submesh: MDLSubmesh) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSubmesh : submesh)
    }
    unsafe fn faceTopology(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, faceTopology)
    }
    unsafe fn setFaceTopology_(&self, faceTopology: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFaceTopology : faceTopology)
    }
    unsafe fn faceCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, faceCount)
    }
    unsafe fn setFaceCount_(&self, faceCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFaceCount : faceCount)
    }
    unsafe fn vertexCreaseIndices(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexCreaseIndices)
    }
    unsafe fn setVertexCreaseIndices_(&self, vertexCreaseIndices: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexCreaseIndices : vertexCreaseIndices)
    }
    unsafe fn vertexCreases(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexCreases)
    }
    unsafe fn setVertexCreases_(&self, vertexCreases: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexCreases : vertexCreases)
    }
    unsafe fn vertexCreaseCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexCreaseCount)
    }
    unsafe fn setVertexCreaseCount_(&self, vertexCreaseCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexCreaseCount : vertexCreaseCount)
    }
    unsafe fn edgeCreaseIndices(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgeCreaseIndices)
    }
    unsafe fn setEdgeCreaseIndices_(&self, edgeCreaseIndices: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgeCreaseIndices : edgeCreaseIndices)
    }
    unsafe fn edgeCreases(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgeCreases)
    }
    unsafe fn setEdgeCreases_(&self, edgeCreases: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgeCreases : edgeCreases)
    }
    unsafe fn edgeCreaseCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgeCreaseCount)
    }
    unsafe fn setEdgeCreaseCount_(&self, edgeCreaseCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgeCreaseCount : edgeCreaseCount)
    }
    unsafe fn holes(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, holes)
    }
    unsafe fn setHoles_(&self, holes: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHoles : holes)
    }
    unsafe fn holeCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, holeCount)
    }
    unsafe fn setHoleCount_(&self, holeCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHoleCount : holeCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLSubmesh(pub id);
impl std::ops::Deref for MDLSubmesh {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLSubmesh {}
impl MDLSubmesh {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLSubmesh").unwrap(), alloc) })
    }
}
impl PMDLNamed for MDLSubmesh {}
impl INSObject for MDLSubmesh {}
impl PNSObject for MDLSubmesh {}
impl std::convert::TryFrom<NSObject> for MDLSubmesh {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLSubmesh, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLSubmesh").unwrap()) };
        if is_kind_of {
            Ok(MDLSubmesh(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLSubmesh")
        }
    }
}
impl IMDLSubmesh for MDLSubmesh {}
pub trait IMDLSubmesh: Sized + std::ops::Deref {
    unsafe fn initWithName_indexBuffer_indexCount_indexType_geometryType_material_(
        &self,
        name: NSString,
        indexBuffer: *mut u64,
        indexCount: NSUInteger,
        indexType: MDLIndexBitDepth,
        geometryType: MDLGeometryType,
        material: MDLMaterial,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, indexBuffer : indexBuffer, indexCount : indexCount, indexType : indexType, geometryType : geometryType, material : material)
    }
    unsafe fn initWithIndexBuffer_indexCount_indexType_geometryType_material_(
        &self,
        indexBuffer: *mut u64,
        indexCount: NSUInteger,
        indexType: MDLIndexBitDepth,
        geometryType: MDLGeometryType,
        material: MDLMaterial,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIndexBuffer : indexBuffer, indexCount : indexCount, indexType : indexType, geometryType : geometryType, material : material)
    }
    unsafe fn initWithName_indexBuffer_indexCount_indexType_geometryType_material_topology_(
        &self,
        name: NSString,
        indexBuffer: *mut u64,
        indexCount: NSUInteger,
        indexType: MDLIndexBitDepth,
        geometryType: MDLGeometryType,
        material: MDLMaterial,
        topology: MDLSubmeshTopology,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, indexBuffer : indexBuffer, indexCount : indexCount, indexType : indexType, geometryType : geometryType, material : material, topology : topology)
    }
    unsafe fn initWithMDLSubmesh_indexType_geometryType_(
        &self,
        submesh: MDLSubmesh,
        indexType: MDLIndexBitDepth,
        geometryType: MDLGeometryType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMDLSubmesh : submesh, indexType : indexType, geometryType : geometryType)
    }
    unsafe fn indexBufferAsIndexType_(&self, indexType: MDLIndexBitDepth) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexBufferAsIndexType : indexType)
    }
    unsafe fn indexBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn indexCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexCount)
    }
    unsafe fn indexType(&self) -> MDLIndexBitDepth
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn geometryType(&self) -> MDLGeometryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometryType)
    }
    unsafe fn material(&self) -> MDLMaterial
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, material)
    }
    unsafe fn setMaterial_(&self, material: MDLMaterial)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaterial : material)
    }
    unsafe fn topology(&self) -> MDLSubmeshTopology
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topology)
    }
    unsafe fn setTopology_(&self, topology: MDLSubmeshTopology)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopology : topology)
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
pub struct MDLVertexAttributeData(pub id);
impl std::ops::Deref for MDLVertexAttributeData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLVertexAttributeData {}
impl MDLVertexAttributeData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLVertexAttributeData").unwrap(), alloc) })
    }
}
impl INSObject for MDLVertexAttributeData {}
impl PNSObject for MDLVertexAttributeData {}
impl std::convert::TryFrom<NSObject> for MDLVertexAttributeData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLVertexAttributeData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLVertexAttributeData").unwrap()) };
        if is_kind_of {
            Ok(MDLVertexAttributeData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLVertexAttributeData")
        }
    }
}
impl IMDLVertexAttributeData for MDLVertexAttributeData {}
pub trait IMDLVertexAttributeData: Sized + std::ops::Deref {
    unsafe fn map(&self) -> MDLMeshBufferMap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, map)
    }
    unsafe fn setMap_(&self, map: MDLMeshBufferMap)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMap : map)
    }
    unsafe fn dataStart(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataStart)
    }
    unsafe fn setDataStart_(&self, dataStart: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataStart : dataStart)
    }
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
    unsafe fn format(&self) -> MDLVertexFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn setFormat_(&self, format: MDLVertexFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFormat : format)
    }
    unsafe fn bufferSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferSize)
    }
    unsafe fn setBufferSize_(&self, bufferSize: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBufferSize : bufferSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLMesh(pub id);
impl std::ops::Deref for MDLMesh {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLMesh {}
impl MDLMesh {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), alloc) })
    }
}
impl IMDLObject for MDLMesh {}
impl PMDLNamed for MDLMesh {}
impl std::convert::TryFrom<MDLObject> for MDLMesh {
    type Error = &'static str;
    fn try_from(parent: MDLObject) -> Result<MDLMesh, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLMesh").unwrap()) };
        if is_kind_of {
            Ok(MDLMesh(parent.0))
        } else {
            Err("This MDLObject cannot be downcasted to MDLMesh")
        }
    }
}
impl INSObject for MDLMesh {}
impl PNSObject for MDLMesh {}
impl IMDLMesh for MDLMesh {}
pub trait IMDLMesh: Sized + std::ops::Deref {
    unsafe fn initWithBufferAllocator_(&self, bufferAllocator: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBufferAllocator : bufferAllocator)
    }
    unsafe fn initWithVertexBuffer_vertexCount_descriptor_submeshes_(
        &self,
        vertexBuffer: *mut u64,
        vertexCount: NSUInteger,
        descriptor: MDLVertexDescriptor,
        submeshes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVertexBuffer : vertexBuffer, vertexCount : vertexCount, descriptor : descriptor, submeshes : submeshes)
    }
    unsafe fn initWithVertexBuffers_vertexCount_descriptor_submeshes_(
        &self,
        vertexBuffers: NSArray,
        vertexCount: NSUInteger,
        descriptor: MDLVertexDescriptor,
        submeshes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVertexBuffers : vertexBuffers, vertexCount : vertexCount, descriptor : descriptor, submeshes : submeshes)
    }
    unsafe fn vertexAttributeDataForAttributeNamed_(&self, name: NSString) -> MDLVertexAttributeData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, vertexAttributeDataForAttributeNamed : name)
    }
    unsafe fn vertexAttributeDataForAttributeNamed_asFormat_(
        &self,
        name: NSString,
        format: MDLVertexFormat,
    ) -> MDLVertexAttributeData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, vertexAttributeDataForAttributeNamed : name, asFormat : format)
    }
    unsafe fn boundingBox(&self) -> MDLAxisAlignedBoundingBox
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBox)
    }
    unsafe fn vertexDescriptor(&self) -> MDLVertexDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexDescriptor)
    }
    unsafe fn setVertexDescriptor_(&self, vertexDescriptor: MDLVertexDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexDescriptor : vertexDescriptor)
    }
    unsafe fn vertexCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexCount)
    }
    unsafe fn setVertexCount_(&self, vertexCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVertexCount : vertexCount)
    }
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
    unsafe fn submeshes(&self) -> NSMutableArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, submeshes)
    }
    unsafe fn setSubmeshes_(&self, submeshes: NSMutableArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubmeshes : submeshes)
    }
    unsafe fn allocator(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocator)
    }
}
impl MDLMesh_Modifiers for MDLMesh {}
pub trait MDLMesh_Modifiers: Sized + std::ops::Deref {
    unsafe fn addAttributeWithName_format_(&self, name: NSString, format: MDLVertexFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAttributeWithName : name, format : format)
    }
    unsafe fn addAttributeWithName_format_type_data_stride_(
        &self,
        name: NSString,
        format: MDLVertexFormat,
        type_: NSString,
        data: NSData,
        stride: NSInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAttributeWithName : name, format : format, r#type : type_, data : data, stride : stride)
    }
    unsafe fn addAttributeWithName_format_type_data_stride_time_(
        &self,
        name: NSString,
        format: MDLVertexFormat,
        type_: NSString,
        data: NSData,
        stride: NSInteger,
        time: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAttributeWithName : name, format : format, r#type : type_, data : data, stride : stride, time : time)
    }
    unsafe fn addNormalsWithAttributeNamed_creaseThreshold_(
        &self,
        attributeName: NSString,
        creaseThreshold: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addNormalsWithAttributeNamed : attributeName, creaseThreshold : creaseThreshold)
    }
    unsafe fn addTangentBasisForTextureCoordinateAttributeNamed_tangentAttributeNamed_bitangentAttributeNamed_(
        &self,
        textureCoordinateAttributeName: NSString,
        tangentAttributeName: NSString,
        bitangentAttributeName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTangentBasisForTextureCoordinateAttributeNamed : textureCoordinateAttributeName, tangentAttributeNamed : tangentAttributeName, bitangentAttributeNamed : bitangentAttributeName)
    }
    unsafe fn addTangentBasisForTextureCoordinateAttributeNamed_normalAttributeNamed_tangentAttributeNamed_(
        &self,
        textureCoordinateAttributeName: NSString,
        normalAttributeName: NSString,
        tangentAttributeName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTangentBasisForTextureCoordinateAttributeNamed : textureCoordinateAttributeName, normalAttributeNamed : normalAttributeName, tangentAttributeNamed : tangentAttributeName)
    }
    unsafe fn addOrthTanBasisForTextureCoordinateAttributeNamed_normalAttributeNamed_tangentAttributeNamed_(
        &self,
        textureCoordinateAttributeName: NSString,
        normalAttributeName: NSString,
        tangentAttributeName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOrthTanBasisForTextureCoordinateAttributeNamed : textureCoordinateAttributeName, normalAttributeNamed : normalAttributeName, tangentAttributeNamed : tangentAttributeName)
    }
    unsafe fn addUnwrappedTextureCoordinatesForAttributeNamed_(
        &self,
        textureCoordinateAttributeName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addUnwrappedTextureCoordinatesForAttributeNamed : textureCoordinateAttributeName)
    }
    unsafe fn flipTextureCoordinatesInAttributeNamed_(
        &self,
        textureCoordinateAttributeName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, flipTextureCoordinatesInAttributeNamed : textureCoordinateAttributeName)
    }
    unsafe fn makeVerticesUnique(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, makeVerticesUnique)
    }
    unsafe fn makeVerticesUniqueAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeVerticesUniqueAndReturnError : error)
    }
    unsafe fn replaceAttributeNamed_withData_(
        &self,
        name: NSString,
        newData: MDLVertexAttributeData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceAttributeNamed : name, withData : newData)
    }
    unsafe fn updateAttributeNamed_withData_(&self, name: NSString, newData: MDLVertexAttributeData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateAttributeNamed : name, withData : newData)
    }
    unsafe fn removeAttributeNamed_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAttributeNamed : name)
    }
}
impl MDLMesh_Generators for MDLMesh {}
pub trait MDLMesh_Generators: Sized + std::ops::Deref {
    unsafe fn initBoxWithExtent_segments_inwardNormals_geometryType_allocator_(
        &self,
        extent: vector_float3,
        segments: vector_uint3,
        inwardNormals: BOOL,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initBoxWithExtent : extent, segments : segments, inwardNormals : inwardNormals, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn initSphereWithExtent_segments_inwardNormals_geometryType_allocator_(
        &self,
        extent: vector_float3,
        segments: vector_uint2,
        inwardNormals: BOOL,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initSphereWithExtent : extent, segments : segments, inwardNormals : inwardNormals, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn initHemisphereWithExtent_segments_inwardNormals_cap_geometryType_allocator_(
        &self,
        extent: vector_float3,
        segments: vector_uint2,
        inwardNormals: BOOL,
        cap: BOOL,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initHemisphereWithExtent : extent, segments : segments, inwardNormals : inwardNormals, cap : cap, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn initCylinderWithExtent_segments_inwardNormals_topCap_bottomCap_geometryType_allocator_(
        &self,
        extent: vector_float3,
        segments: vector_uint2,
        inwardNormals: BOOL,
        topCap: BOOL,
        bottomCap: BOOL,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initCylinderWithExtent : extent, segments : segments, inwardNormals : inwardNormals, topCap : topCap, bottomCap : bottomCap, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn initCapsuleWithExtent_cylinderSegments_hemisphereSegments_inwardNormals_geometryType_allocator_(
        &self,
        extent: vector_float3,
        segments: vector_uint2,
        hemisphereSegments: ::std::os::raw::c_int,
        inwardNormals: BOOL,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initCapsuleWithExtent : extent, cylinderSegments : segments, hemisphereSegments : hemisphereSegments, inwardNormals : inwardNormals, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn initConeWithExtent_segments_inwardNormals_cap_geometryType_allocator_(
        &self,
        extent: vector_float3,
        segments: vector_uint2,
        inwardNormals: BOOL,
        cap: BOOL,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initConeWithExtent : extent, segments : segments, inwardNormals : inwardNormals, cap : cap, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn initPlaneWithExtent_segments_geometryType_allocator_(
        &self,
        extent: vector_float3,
        segments: vector_uint2,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initPlaneWithExtent : extent, segments : segments, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn initIcosahedronWithExtent_inwardNormals_geometryType_allocator_(
        &self,
        extent: vector_float3,
        inwardNormals: BOOL,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initIcosahedronWithExtent : extent, inwardNormals : inwardNormals, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn initMeshBySubdividingMesh_submeshIndex_subdivisionLevels_allocator_(
        &self,
        mesh: MDLMesh,
        submeshIndex: ::std::os::raw::c_int,
        subdivisionLevels: ::std::os::raw::c_uint,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initMeshBySubdividingMesh : mesh, submeshIndex : submeshIndex, subdivisionLevels : subdivisionLevels, allocator : allocator)
    }
    unsafe fn newBoxWithDimensions_segments_geometryType_inwardNormals_allocator_(
        dimensions: vector_float3,
        segments: vector_uint3,
        geometryType: MDLGeometryType,
        inwardNormals: BOOL,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), newBoxWithDimensions : dimensions, segments : segments, geometryType : geometryType, inwardNormals : inwardNormals, allocator : allocator)
    }
    unsafe fn newEllipsoidWithRadii_radialSegments_verticalSegments_geometryType_inwardNormals_hemisphere_allocator_(
        radii: vector_float3,
        radialSegments: NSUInteger,
        verticalSegments: NSUInteger,
        geometryType: MDLGeometryType,
        inwardNormals: BOOL,
        hemisphere: BOOL,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), newEllipsoidWithRadii : radii, radialSegments : radialSegments, verticalSegments : verticalSegments, geometryType : geometryType, inwardNormals : inwardNormals, hemisphere : hemisphere, allocator : allocator)
    }
    unsafe fn newCylinderWithHeight_radii_radialSegments_verticalSegments_geometryType_inwardNormals_allocator_(
        height: f32,
        radii: vector_float2,
        radialSegments: NSUInteger,
        verticalSegments: NSUInteger,
        geometryType: MDLGeometryType,
        inwardNormals: BOOL,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), newCylinderWithHeight : height, radii : radii, radialSegments : radialSegments, verticalSegments : verticalSegments, geometryType : geometryType, inwardNormals : inwardNormals, allocator : allocator)
    }
    unsafe fn newCapsuleWithHeight_radii_radialSegments_verticalSegments_hemisphereSegments_geometryType_inwardNormals_allocator_(
        height: f32,
        radii: vector_float2,
        radialSegments: NSUInteger,
        verticalSegments: NSUInteger,
        hemisphereSegments: NSUInteger,
        geometryType: MDLGeometryType,
        inwardNormals: BOOL,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), newCapsuleWithHeight : height, radii : radii, radialSegments : radialSegments, verticalSegments : verticalSegments, hemisphereSegments : hemisphereSegments, geometryType : geometryType, inwardNormals : inwardNormals, allocator : allocator)
    }
    unsafe fn newEllipticalConeWithHeight_radii_radialSegments_verticalSegments_geometryType_inwardNormals_allocator_(
        height: f32,
        radii: vector_float2,
        radialSegments: NSUInteger,
        verticalSegments: NSUInteger,
        geometryType: MDLGeometryType,
        inwardNormals: BOOL,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), newEllipticalConeWithHeight : height, radii : radii, radialSegments : radialSegments, verticalSegments : verticalSegments, geometryType : geometryType, inwardNormals : inwardNormals, allocator : allocator)
    }
    unsafe fn newPlaneWithDimensions_segments_geometryType_allocator_(
        dimensions: vector_float2,
        segments: vector_uint2,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), newPlaneWithDimensions : dimensions, segments : segments, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn newIcosahedronWithRadius_inwardNormals_geometryType_allocator_(
        radius: f32,
        inwardNormals: BOOL,
        geometryType: MDLGeometryType,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), newIcosahedronWithRadius : radius, inwardNormals : inwardNormals, geometryType : geometryType, allocator : allocator)
    }
    unsafe fn newIcosahedronWithRadius_inwardNormals_allocator_(
        radius: f32,
        inwardNormals: BOOL,
        allocator: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), newIcosahedronWithRadius : radius, inwardNormals : inwardNormals, allocator : allocator)
    }
    unsafe fn newSubdividedMesh_submeshIndex_subdivisionLevels_(
        mesh: MDLMesh,
        submeshIndex: NSUInteger,
        subdivisionLevels: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLMesh").unwrap(), newSubdividedMesh : mesh, submeshIndex : submeshIndex, subdivisionLevels : subdivisionLevels)
    }
}
impl MDLMesh_MDLLightBaking for MDLMesh {}
pub trait MDLMesh_MDLLightBaking: Sized + std::ops::Deref {
    unsafe fn generateAmbientOcclusionTextureWithSize_raysPerSample_attenuationFactor_objectsToConsider_vertexAttributeNamed_materialPropertyNamed_(
        &self,
        textureSize: vector_int2,
        raysPerSample: NSInteger,
        attenuationFactor: f32,
        objectsToConsider: NSArray,
        vertexAttributeName: NSString,
        materialPropertyName: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateAmbientOcclusionTextureWithSize : textureSize, raysPerSample : raysPerSample, attenuationFactor : attenuationFactor, objectsToConsider : objectsToConsider, vertexAttributeNamed : vertexAttributeName, materialPropertyNamed : materialPropertyName)
    }
    unsafe fn generateAmbientOcclusionTextureWithQuality_attenuationFactor_objectsToConsider_vertexAttributeNamed_materialPropertyNamed_(
        &self,
        bakeQuality: f32,
        attenuationFactor: f32,
        objectsToConsider: NSArray,
        vertexAttributeName: NSString,
        materialPropertyName: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateAmbientOcclusionTextureWithQuality : bakeQuality, attenuationFactor : attenuationFactor, objectsToConsider : objectsToConsider, vertexAttributeNamed : vertexAttributeName, materialPropertyNamed : materialPropertyName)
    }
    unsafe fn generateAmbientOcclusionVertexColorsWithRaysPerSample_attenuationFactor_objectsToConsider_vertexAttributeNamed_(
        &self,
        raysPerSample: NSInteger,
        attenuationFactor: f32,
        objectsToConsider: NSArray,
        vertexAttributeName: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateAmbientOcclusionVertexColorsWithRaysPerSample : raysPerSample, attenuationFactor : attenuationFactor, objectsToConsider : objectsToConsider, vertexAttributeNamed : vertexAttributeName)
    }
    unsafe fn generateAmbientOcclusionVertexColorsWithQuality_attenuationFactor_objectsToConsider_vertexAttributeNamed_(
        &self,
        bakeQuality: f32,
        attenuationFactor: f32,
        objectsToConsider: NSArray,
        vertexAttributeName: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateAmbientOcclusionVertexColorsWithQuality : bakeQuality, attenuationFactor : attenuationFactor, objectsToConsider : objectsToConsider, vertexAttributeNamed : vertexAttributeName)
    }
    unsafe fn generateLightMapTextureWithTextureSize_lightsToConsider_objectsToConsider_vertexAttributeNamed_materialPropertyNamed_(
        &self,
        textureSize: vector_int2,
        lightsToConsider: NSArray,
        objectsToConsider: NSArray,
        vertexAttributeName: NSString,
        materialPropertyName: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateLightMapTextureWithTextureSize : textureSize, lightsToConsider : lightsToConsider, objectsToConsider : objectsToConsider, vertexAttributeNamed : vertexAttributeName, materialPropertyNamed : materialPropertyName)
    }
    unsafe fn generateLightMapTextureWithQuality_lightsToConsider_objectsToConsider_vertexAttributeNamed_materialPropertyNamed_(
        &self,
        bakeQuality: f32,
        lightsToConsider: NSArray,
        objectsToConsider: NSArray,
        vertexAttributeName: NSString,
        materialPropertyName: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateLightMapTextureWithQuality : bakeQuality, lightsToConsider : lightsToConsider, objectsToConsider : objectsToConsider, vertexAttributeNamed : vertexAttributeName, materialPropertyNamed : materialPropertyName)
    }
    unsafe fn generateLightMapVertexColorsWithLightsToConsider_objectsToConsider_vertexAttributeNamed_(
        &self,
        lightsToConsider: NSArray,
        objectsToConsider: NSArray,
        vertexAttributeName: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateLightMapVertexColorsWithLightsToConsider : lightsToConsider, objectsToConsider : objectsToConsider, vertexAttributeNamed : vertexAttributeName)
    }
}
pub type MDLTextureChannelEncoding = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTexture(pub id);
impl std::ops::Deref for MDLTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTexture {}
impl MDLTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTexture").unwrap(), alloc) })
    }
}
impl PMDLNamed for MDLTexture {}
impl INSObject for MDLTexture {}
impl PNSObject for MDLTexture {}
impl std::convert::TryFrom<NSObject> for MDLTexture {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTexture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTexture").unwrap()) };
        if is_kind_of {
            Ok(MDLTexture(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTexture")
        }
    }
}
impl IMDLTexture for MDLTexture {}
pub trait IMDLTexture: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithData_topLeftOrigin_name_dimensions_rowStride_channelCount_channelEncoding_isCube_(
        &self,
        pixelData: NSData,
        topLeftOrigin: BOOL,
        name: NSString,
        dimensions: vector_int2,
        rowStride: NSInteger,
        channelCount: NSUInteger,
        channelEncoding: MDLTextureChannelEncoding,
        isCube: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : pixelData, topLeftOrigin : topLeftOrigin, name : name, dimensions : dimensions, rowStride : rowStride, channelCount : channelCount, channelEncoding : channelEncoding, isCube : isCube)
    }
    unsafe fn writeToURL_(&self, URL: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : URL)
    }
    unsafe fn writeToURL_level_(&self, URL: NSURL, level: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : URL, level : level)
    }
    unsafe fn writeToURL_type_(&self, nsurl: NSURL, type_: CFStringRef) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : nsurl, r#type : type_)
    }
    unsafe fn writeToURL_type_level_(
        &self,
        nsurl: NSURL,
        type_: CFStringRef,
        level: NSUInteger,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : nsurl, r#type : type_, level : level)
    }
    unsafe fn imageFromTexture(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageFromTexture)
    }
    unsafe fn imageFromTextureAtLevel_(&self, level: NSUInteger) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageFromTextureAtLevel : level)
    }
    unsafe fn texelDataWithTopLeftOrigin(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texelDataWithTopLeftOrigin)
    }
    unsafe fn texelDataWithBottomLeftOrigin(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texelDataWithBottomLeftOrigin)
    }
    unsafe fn texelDataWithTopLeftOriginAtMipLevel_create_(
        &self,
        level: NSInteger,
        create: BOOL,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, texelDataWithTopLeftOriginAtMipLevel : level, create : create)
    }
    unsafe fn texelDataWithBottomLeftOriginAtMipLevel_create_(
        &self,
        level: NSInteger,
        create: BOOL,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, texelDataWithBottomLeftOriginAtMipLevel : level, create : create)
    }
    unsafe fn dimensions(&self) -> vector_int2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensions)
    }
    unsafe fn rowStride(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rowStride)
    }
    unsafe fn channelCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelCount)
    }
    unsafe fn mipLevelCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mipLevelCount)
    }
    unsafe fn channelEncoding(&self) -> MDLTextureChannelEncoding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelEncoding)
    }
    unsafe fn isCube(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCube)
    }
    unsafe fn setIsCube_(&self, isCube: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsCube : isCube)
    }
    unsafe fn hasAlphaValues(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAlphaValues)
    }
    unsafe fn setHasAlphaValues_(&self, hasAlphaValues: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasAlphaValues : hasAlphaValues)
    }
    unsafe fn textureNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTexture").unwrap(), textureNamed : name)
    }
    unsafe fn textureNamed_bundle_(name: NSString, bundleOrNil: NSBundle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTexture").unwrap(), textureNamed : name, bundle : bundleOrNil)
    }
    unsafe fn textureNamed_assetResolver_(name: NSString, resolver: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTexture").unwrap(), textureNamed : name, assetResolver : resolver)
    }
    unsafe fn textureCubeWithImagesNamed_(names: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTexture").unwrap(), textureCubeWithImagesNamed : names)
    }
    unsafe fn textureCubeWithImagesNamed_bundle_(
        names: NSArray,
        bundleOrNil: NSBundle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTexture").unwrap(), textureCubeWithImagesNamed : names, bundle : bundleOrNil)
    }
    unsafe fn irradianceTextureCubeWithTexture_name_dimensions_(
        texture: MDLTexture,
        name: NSString,
        dimensions: vector_int2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTexture").unwrap(), irradianceTextureCubeWithTexture : texture, name : name, dimensions : dimensions)
    }
    unsafe fn irradianceTextureCubeWithTexture_name_dimensions_roughness_(
        texture: MDLTexture,
        name: NSString,
        dimensions: vector_int2,
        roughness: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTexture").unwrap(), irradianceTextureCubeWithTexture : texture, name : name, dimensions : dimensions, roughness : roughness)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLURLTexture(pub id);
impl std::ops::Deref for MDLURLTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLURLTexture {}
impl MDLURLTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLURLTexture").unwrap(), alloc) })
    }
}
impl IMDLTexture for MDLURLTexture {}
impl PMDLNamed for MDLURLTexture {}
impl From<MDLURLTexture> for MDLTexture {
    fn from(child: MDLURLTexture) -> MDLTexture {
        MDLTexture(child.0)
    }
}
impl std::convert::TryFrom<MDLTexture> for MDLURLTexture {
    type Error = &'static str;
    fn try_from(parent: MDLTexture) -> Result<MDLURLTexture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLURLTexture").unwrap()) };
        if is_kind_of {
            Ok(MDLURLTexture(parent.0))
        } else {
            Err("This MDLTexture cannot be downcasted to MDLURLTexture")
        }
    }
}
impl INSObject for MDLURLTexture {}
impl PNSObject for MDLURLTexture {}
impl IMDLURLTexture for MDLURLTexture {}
pub trait IMDLURLTexture: Sized + std::ops::Deref {
    unsafe fn initWithURL_name_(&self, URL: NSURL, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL, name : name)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : URL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLCheckerboardTexture(pub id);
impl std::ops::Deref for MDLCheckerboardTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLCheckerboardTexture {}
impl MDLCheckerboardTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLCheckerboardTexture").unwrap(), alloc) })
    }
}
impl IMDLTexture for MDLCheckerboardTexture {}
impl PMDLNamed for MDLCheckerboardTexture {}
impl std::convert::TryFrom<MDLTexture> for MDLCheckerboardTexture {
    type Error = &'static str;
    fn try_from(parent: MDLTexture) -> Result<MDLCheckerboardTexture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLCheckerboardTexture").unwrap()) };
        if is_kind_of {
            Ok(MDLCheckerboardTexture(parent.0))
        } else {
            Err("This MDLTexture cannot be downcasted to MDLCheckerboardTexture")
        }
    }
}
impl INSObject for MDLCheckerboardTexture {}
impl PNSObject for MDLCheckerboardTexture {}
impl IMDLCheckerboardTexture for MDLCheckerboardTexture {}
pub trait IMDLCheckerboardTexture: Sized + std::ops::Deref {
    unsafe fn initWithDivisions_name_dimensions_channelCount_channelEncoding_color1_color2_(
        &self,
        divisions: f32,
        name: NSString,
        dimensions: vector_int2,
        channelCount: ::std::os::raw::c_int,
        channelEncoding: MDLTextureChannelEncoding,
        color1: CGColorRef,
        color2: CGColorRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDivisions : divisions, name : name, dimensions : dimensions, channelCount : channelCount, channelEncoding : channelEncoding, color1 : color1, color2 : color2)
    }
    unsafe fn divisions(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, divisions)
    }
    unsafe fn setDivisions_(&self, divisions: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDivisions : divisions)
    }
    unsafe fn color1(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
    unsafe fn color2(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color2)
    }
    unsafe fn setColor2_(&self, color2: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor2 : color2)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLSkyCubeTexture(pub id);
impl std::ops::Deref for MDLSkyCubeTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLSkyCubeTexture {}
impl MDLSkyCubeTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLSkyCubeTexture").unwrap(), alloc) })
    }
}
impl IMDLTexture for MDLSkyCubeTexture {}
impl PMDLNamed for MDLSkyCubeTexture {}
impl std::convert::TryFrom<MDLTexture> for MDLSkyCubeTexture {
    type Error = &'static str;
    fn try_from(parent: MDLTexture) -> Result<MDLSkyCubeTexture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLSkyCubeTexture").unwrap()) };
        if is_kind_of {
            Ok(MDLSkyCubeTexture(parent.0))
        } else {
            Err("This MDLTexture cannot be downcasted to MDLSkyCubeTexture")
        }
    }
}
impl INSObject for MDLSkyCubeTexture {}
impl PNSObject for MDLSkyCubeTexture {}
impl IMDLSkyCubeTexture for MDLSkyCubeTexture {}
pub trait IMDLSkyCubeTexture: Sized + std::ops::Deref {
    unsafe fn initWithName_channelEncoding_textureDimensions_turbidity_sunElevation_upperAtmosphereScattering_groundAlbedo_(
        &self,
        name: NSString,
        channelEncoding: MDLTextureChannelEncoding,
        textureDimensions: vector_int2,
        turbidity: f32,
        sunElevation: f32,
        upperAtmosphereScattering: f32,
        groundAlbedo: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, channelEncoding : channelEncoding, textureDimensions : textureDimensions, turbidity : turbidity, sunElevation : sunElevation, upperAtmosphereScattering : upperAtmosphereScattering, groundAlbedo : groundAlbedo)
    }
    unsafe fn initWithName_channelEncoding_textureDimensions_turbidity_sunElevation_sunAzimuth_upperAtmosphereScattering_groundAlbedo_(
        &self,
        name: NSString,
        channelEncoding: MDLTextureChannelEncoding,
        textureDimensions: vector_int2,
        turbidity: f32,
        sunElevation: f32,
        sunAzimuth: f32,
        upperAtmosphereScattering: f32,
        groundAlbedo: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, channelEncoding : channelEncoding, textureDimensions : textureDimensions, turbidity : turbidity, sunElevation : sunElevation, sunAzimuth : sunAzimuth, upperAtmosphereScattering : upperAtmosphereScattering, groundAlbedo : groundAlbedo)
    }
    unsafe fn updateTexture(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateTexture)
    }
    unsafe fn turbidity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, turbidity)
    }
    unsafe fn setTurbidity_(&self, turbidity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTurbidity : turbidity)
    }
    unsafe fn sunElevation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sunElevation)
    }
    unsafe fn setSunElevation_(&self, sunElevation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSunElevation : sunElevation)
    }
    unsafe fn sunAzimuth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sunAzimuth)
    }
    unsafe fn setSunAzimuth_(&self, sunAzimuth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSunAzimuth : sunAzimuth)
    }
    unsafe fn upperAtmosphereScattering(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperAtmosphereScattering)
    }
    unsafe fn setUpperAtmosphereScattering_(&self, upperAtmosphereScattering: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpperAtmosphereScattering : upperAtmosphereScattering)
    }
    unsafe fn groundAlbedo(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groundAlbedo)
    }
    unsafe fn setGroundAlbedo_(&self, groundAlbedo: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroundAlbedo : groundAlbedo)
    }
    unsafe fn horizonElevation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizonElevation)
    }
    unsafe fn setHorizonElevation_(&self, horizonElevation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHorizonElevation : horizonElevation)
    }
    unsafe fn groundColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groundColor)
    }
    unsafe fn setGroundColor_(&self, groundColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroundColor : groundColor)
    }
    unsafe fn gamma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gamma)
    }
    unsafe fn setGamma_(&self, gamma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGamma : gamma)
    }
    unsafe fn exposure(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposure)
    }
    unsafe fn setExposure_(&self, exposure: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposure : exposure)
    }
    unsafe fn brightness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, brightness)
    }
    unsafe fn setBrightness_(&self, brightness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrightness : brightness)
    }
    unsafe fn contrast(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast)
    }
    unsafe fn setContrast_(&self, contrast: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast : contrast)
    }
    unsafe fn saturation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saturation)
    }
    unsafe fn setSaturation_(&self, saturation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSaturation : saturation)
    }
    unsafe fn highDynamicRangeCompression(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highDynamicRangeCompression)
    }
    unsafe fn setHighDynamicRangeCompression_(&self, highDynamicRangeCompression: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighDynamicRangeCompression : highDynamicRangeCompression)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLColorSwatchTexture(pub id);
impl std::ops::Deref for MDLColorSwatchTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLColorSwatchTexture {}
impl MDLColorSwatchTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLColorSwatchTexture").unwrap(), alloc) })
    }
}
impl IMDLTexture for MDLColorSwatchTexture {}
impl PMDLNamed for MDLColorSwatchTexture {}
impl std::convert::TryFrom<MDLTexture> for MDLColorSwatchTexture {
    type Error = &'static str;
    fn try_from(parent: MDLTexture) -> Result<MDLColorSwatchTexture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLColorSwatchTexture").unwrap()) };
        if is_kind_of {
            Ok(MDLColorSwatchTexture(parent.0))
        } else {
            Err("This MDLTexture cannot be downcasted to MDLColorSwatchTexture")
        }
    }
}
impl INSObject for MDLColorSwatchTexture {}
impl PNSObject for MDLColorSwatchTexture {}
impl IMDLColorSwatchTexture for MDLColorSwatchTexture {}
pub trait IMDLColorSwatchTexture: Sized + std::ops::Deref {
    unsafe fn initWithColorTemperatureGradientFrom_toColorTemperature_name_textureDimensions_(
        &self,
        colorTemperature1: f32,
        colorTemperature2: f32,
        name: NSString,
        textureDimensions: vector_int2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithColorTemperatureGradientFrom : colorTemperature1, toColorTemperature : colorTemperature2, name : name, textureDimensions : textureDimensions)
    }
    unsafe fn initWithColorGradientFrom_toColor_name_textureDimensions_(
        &self,
        color1: CGColorRef,
        color2: CGColorRef,
        name: NSString,
        textureDimensions: vector_int2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithColorGradientFrom : color1, toColor : color2, name : name, textureDimensions : textureDimensions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLNoiseTexture(pub id);
impl std::ops::Deref for MDLNoiseTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLNoiseTexture {}
impl MDLNoiseTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLNoiseTexture").unwrap(), alloc) })
    }
}
impl IMDLTexture for MDLNoiseTexture {}
impl PMDLNamed for MDLNoiseTexture {}
impl std::convert::TryFrom<MDLTexture> for MDLNoiseTexture {
    type Error = &'static str;
    fn try_from(parent: MDLTexture) -> Result<MDLNoiseTexture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLNoiseTexture").unwrap()) };
        if is_kind_of {
            Ok(MDLNoiseTexture(parent.0))
        } else {
            Err("This MDLTexture cannot be downcasted to MDLNoiseTexture")
        }
    }
}
impl INSObject for MDLNoiseTexture {}
impl PNSObject for MDLNoiseTexture {}
impl IMDLNoiseTexture for MDLNoiseTexture {}
pub trait IMDLNoiseTexture: Sized + std::ops::Deref {
    unsafe fn initVectorNoiseWithSmoothness_name_textureDimensions_channelEncoding_(
        &self,
        smoothness: f32,
        name: NSString,
        textureDimensions: vector_int2,
        channelEncoding: MDLTextureChannelEncoding,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initVectorNoiseWithSmoothness : smoothness, name : name, textureDimensions : textureDimensions, channelEncoding : channelEncoding)
    }
    unsafe fn initScalarNoiseWithSmoothness_name_textureDimensions_channelCount_channelEncoding_grayscale_(
        &self,
        smoothness: f32,
        name: NSString,
        textureDimensions: vector_int2,
        channelCount: ::std::os::raw::c_int,
        channelEncoding: MDLTextureChannelEncoding,
        grayscale: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initScalarNoiseWithSmoothness : smoothness, name : name, textureDimensions : textureDimensions, channelCount : channelCount, channelEncoding : channelEncoding, grayscale : grayscale)
    }
    unsafe fn initCellularNoiseWithFrequency_name_textureDimensions_channelEncoding_(
        &self,
        frequency: f32,
        name: NSString,
        textureDimensions: vector_int2,
        channelEncoding: MDLTextureChannelEncoding,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initCellularNoiseWithFrequency : frequency, name : name, textureDimensions : textureDimensions, channelEncoding : channelEncoding)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLNormalMapTexture(pub id);
impl std::ops::Deref for MDLNormalMapTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLNormalMapTexture {}
impl MDLNormalMapTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLNormalMapTexture").unwrap(), alloc) })
    }
}
impl IMDLTexture for MDLNormalMapTexture {}
impl PMDLNamed for MDLNormalMapTexture {}
impl std::convert::TryFrom<MDLTexture> for MDLNormalMapTexture {
    type Error = &'static str;
    fn try_from(parent: MDLTexture) -> Result<MDLNormalMapTexture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLNormalMapTexture").unwrap()) };
        if is_kind_of {
            Ok(MDLNormalMapTexture(parent.0))
        } else {
            Err("This MDLTexture cannot be downcasted to MDLNormalMapTexture")
        }
    }
}
impl INSObject for MDLNormalMapTexture {}
impl PNSObject for MDLNormalMapTexture {}
impl IMDLNormalMapTexture for MDLNormalMapTexture {}
pub trait IMDLNormalMapTexture: Sized + std::ops::Deref {
    unsafe fn initByGeneratingNormalMapWithTexture_name_smoothness_contrast_(
        &self,
        sourceTexture: MDLTexture,
        name: NSString,
        smoothness: f32,
        contrast: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initByGeneratingNormalMapWithTexture : sourceTexture, name : name, smoothness : smoothness, contrast : contrast)
    }
}
pub type MDLTransformOpRotationOrder = NSUInteger;
pub trait PMDLTransformOp: Sized + std::ops::Deref {
    unsafe fn float4x4AtTime_(&self, time: NSTimeInterval) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, float4x4AtTime : time)
    }
    unsafe fn double4x4AtTime_(&self, time: NSTimeInterval) -> matrix_double4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, double4x4AtTime : time)
    }
    unsafe fn IsInverseOp(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, IsInverseOp)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransformRotateXOp(pub id);
impl std::ops::Deref for MDLTransformRotateXOp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransformRotateXOp {}
impl MDLTransformRotateXOp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformRotateXOp").unwrap(), alloc) })
    }
}
impl PMDLTransformOp for MDLTransformRotateXOp {}
impl INSObject for MDLTransformRotateXOp {}
impl PNSObject for MDLTransformRotateXOp {}
impl std::convert::TryFrom<NSObject> for MDLTransformRotateXOp {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransformRotateXOp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransformRotateXOp").unwrap()) };
        if is_kind_of {
            Ok(MDLTransformRotateXOp(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransformRotateXOp")
        }
    }
}
impl IMDLTransformRotateXOp for MDLTransformRotateXOp {}
pub trait IMDLTransformRotateXOp: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn animatedValue(&self) -> MDLAnimatedScalar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatedValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransformRotateYOp(pub id);
impl std::ops::Deref for MDLTransformRotateYOp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransformRotateYOp {}
impl MDLTransformRotateYOp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformRotateYOp").unwrap(), alloc) })
    }
}
impl PMDLTransformOp for MDLTransformRotateYOp {}
impl INSObject for MDLTransformRotateYOp {}
impl PNSObject for MDLTransformRotateYOp {}
impl std::convert::TryFrom<NSObject> for MDLTransformRotateYOp {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransformRotateYOp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransformRotateYOp").unwrap()) };
        if is_kind_of {
            Ok(MDLTransformRotateYOp(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransformRotateYOp")
        }
    }
}
impl IMDLTransformRotateYOp for MDLTransformRotateYOp {}
pub trait IMDLTransformRotateYOp: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn animatedValue(&self) -> MDLAnimatedScalar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatedValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransformRotateZOp(pub id);
impl std::ops::Deref for MDLTransformRotateZOp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransformRotateZOp {}
impl MDLTransformRotateZOp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformRotateZOp").unwrap(), alloc) })
    }
}
impl PMDLTransformOp for MDLTransformRotateZOp {}
impl INSObject for MDLTransformRotateZOp {}
impl PNSObject for MDLTransformRotateZOp {}
impl std::convert::TryFrom<NSObject> for MDLTransformRotateZOp {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransformRotateZOp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransformRotateZOp").unwrap()) };
        if is_kind_of {
            Ok(MDLTransformRotateZOp(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransformRotateZOp")
        }
    }
}
impl IMDLTransformRotateZOp for MDLTransformRotateZOp {}
pub trait IMDLTransformRotateZOp: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn animatedValue(&self) -> MDLAnimatedScalar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatedValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransformRotateOp(pub id);
impl std::ops::Deref for MDLTransformRotateOp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransformRotateOp {}
impl MDLTransformRotateOp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformRotateOp").unwrap(), alloc) })
    }
}
impl PMDLTransformOp for MDLTransformRotateOp {}
impl INSObject for MDLTransformRotateOp {}
impl PNSObject for MDLTransformRotateOp {}
impl std::convert::TryFrom<NSObject> for MDLTransformRotateOp {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransformRotateOp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransformRotateOp").unwrap()) };
        if is_kind_of {
            Ok(MDLTransformRotateOp(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransformRotateOp")
        }
    }
}
impl IMDLTransformRotateOp for MDLTransformRotateOp {}
pub trait IMDLTransformRotateOp: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn animatedValue(&self) -> MDLAnimatedVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatedValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransformTranslateOp(pub id);
impl std::ops::Deref for MDLTransformTranslateOp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransformTranslateOp {}
impl MDLTransformTranslateOp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformTranslateOp").unwrap(), alloc) })
    }
}
impl PMDLTransformOp for MDLTransformTranslateOp {}
impl INSObject for MDLTransformTranslateOp {}
impl PNSObject for MDLTransformTranslateOp {}
impl std::convert::TryFrom<NSObject> for MDLTransformTranslateOp {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransformTranslateOp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransformTranslateOp").unwrap()) };
        if is_kind_of {
            Ok(MDLTransformTranslateOp(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransformTranslateOp")
        }
    }
}
impl IMDLTransformTranslateOp for MDLTransformTranslateOp {}
pub trait IMDLTransformTranslateOp: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn animatedValue(&self) -> MDLAnimatedVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatedValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransformScaleOp(pub id);
impl std::ops::Deref for MDLTransformScaleOp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransformScaleOp {}
impl MDLTransformScaleOp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformScaleOp").unwrap(), alloc) })
    }
}
impl PMDLTransformOp for MDLTransformScaleOp {}
impl INSObject for MDLTransformScaleOp {}
impl PNSObject for MDLTransformScaleOp {}
impl std::convert::TryFrom<NSObject> for MDLTransformScaleOp {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransformScaleOp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransformScaleOp").unwrap()) };
        if is_kind_of {
            Ok(MDLTransformScaleOp(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransformScaleOp")
        }
    }
}
impl IMDLTransformScaleOp for MDLTransformScaleOp {}
pub trait IMDLTransformScaleOp: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn animatedValue(&self) -> MDLAnimatedVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatedValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransformMatrixOp(pub id);
impl std::ops::Deref for MDLTransformMatrixOp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransformMatrixOp {}
impl MDLTransformMatrixOp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformMatrixOp").unwrap(), alloc) })
    }
}
impl PMDLTransformOp for MDLTransformMatrixOp {}
impl INSObject for MDLTransformMatrixOp {}
impl PNSObject for MDLTransformMatrixOp {}
impl std::convert::TryFrom<NSObject> for MDLTransformMatrixOp {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransformMatrixOp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransformMatrixOp").unwrap()) };
        if is_kind_of {
            Ok(MDLTransformMatrixOp(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransformMatrixOp")
        }
    }
}
impl IMDLTransformMatrixOp for MDLTransformMatrixOp {}
pub trait IMDLTransformMatrixOp: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn animatedValue(&self) -> MDLAnimatedMatrix4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatedValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransformOrientOp(pub id);
impl std::ops::Deref for MDLTransformOrientOp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransformOrientOp {}
impl MDLTransformOrientOp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformOrientOp").unwrap(), alloc) })
    }
}
impl PMDLTransformOp for MDLTransformOrientOp {}
impl INSObject for MDLTransformOrientOp {}
impl PNSObject for MDLTransformOrientOp {}
impl std::convert::TryFrom<NSObject> for MDLTransformOrientOp {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransformOrientOp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransformOrientOp").unwrap()) };
        if is_kind_of {
            Ok(MDLTransformOrientOp(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransformOrientOp")
        }
    }
}
impl IMDLTransformOrientOp for MDLTransformOrientOp {}
pub trait IMDLTransformOrientOp: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn animatedValue(&self) -> MDLAnimatedQuaternion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatedValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLTransformStack(pub id);
impl std::ops::Deref for MDLTransformStack {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLTransformStack {}
impl MDLTransformStack {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLTransformStack").unwrap(), alloc) })
    }
}
impl PNSCopying for MDLTransformStack {}
impl PMDLTransformComponent for MDLTransformStack {}
impl INSObject for MDLTransformStack {}
impl PNSObject for MDLTransformStack {}
impl std::convert::TryFrom<NSObject> for MDLTransformStack {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLTransformStack, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLTransformStack").unwrap()) };
        if is_kind_of {
            Ok(MDLTransformStack(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLTransformStack")
        }
    }
}
impl IMDLTransformStack for MDLTransformStack {}
pub trait IMDLTransformStack: Sized + std::ops::Deref {
    unsafe fn init(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addTranslateOp_inverse_(
        &self,
        animatedValueName: NSString,
        inverse: bool,
    ) -> MDLTransformTranslateOp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTranslateOp : animatedValueName, inverse : inverse)
    }
    unsafe fn addRotateXOp_inverse_(
        &self,
        animatedValueName: NSString,
        inverse: bool,
    ) -> MDLTransformRotateXOp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRotateXOp : animatedValueName, inverse : inverse)
    }
    unsafe fn addRotateYOp_inverse_(
        &self,
        animatedValueName: NSString,
        inverse: bool,
    ) -> MDLTransformRotateYOp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRotateYOp : animatedValueName, inverse : inverse)
    }
    unsafe fn addRotateZOp_inverse_(
        &self,
        animatedValueName: NSString,
        inverse: bool,
    ) -> MDLTransformRotateZOp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRotateZOp : animatedValueName, inverse : inverse)
    }
    unsafe fn addRotateOp_order_inverse_(
        &self,
        animatedValueName: NSString,
        order: MDLTransformOpRotationOrder,
        inverse: bool,
    ) -> MDLTransformRotateOp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRotateOp : animatedValueName, order : order, inverse : inverse)
    }
    unsafe fn addScaleOp_inverse_(
        &self,
        animatedValueName: NSString,
        inverse: bool,
    ) -> MDLTransformScaleOp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addScaleOp : animatedValueName, inverse : inverse)
    }
    unsafe fn addMatrixOp_inverse_(
        &self,
        animatedValueName: NSString,
        inverse: bool,
    ) -> MDLTransformMatrixOp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMatrixOp : animatedValueName, inverse : inverse)
    }
    unsafe fn addOrientOp_inverse_(
        &self,
        animatedValueName: NSString,
        inverse: bool,
    ) -> MDLTransformOrientOp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOrientOp : animatedValueName, inverse : inverse)
    }
    unsafe fn animatedValueWithName_(&self, name: NSString) -> MDLAnimatedValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, animatedValueWithName : name)
    }
    unsafe fn float4x4AtTime_(&self, time: NSTimeInterval) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, float4x4AtTime : time)
    }
    unsafe fn double4x4AtTime_(&self, time: NSTimeInterval) -> matrix_double4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, double4x4AtTime : time)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn keyTimes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyTimes)
    }
    unsafe fn transformOps(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transformOps)
    }
}
pub type MDLVoxelIndex = vector_int4;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct MDLVoxelIndexExtent {
    pub minimumExtent: MDLVoxelIndex,
    pub maximumExtent: MDLVoxelIndex,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLVoxelArray(pub id);
impl std::ops::Deref for MDLVoxelArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLVoxelArray {}
impl MDLVoxelArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLVoxelArray").unwrap(), alloc) })
    }
}
impl IMDLObject for MDLVoxelArray {}
impl PMDLNamed for MDLVoxelArray {}
impl std::convert::TryFrom<MDLObject> for MDLVoxelArray {
    type Error = &'static str;
    fn try_from(parent: MDLObject) -> Result<MDLVoxelArray, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLVoxelArray").unwrap()) };
        if is_kind_of {
            Ok(MDLVoxelArray(parent.0))
        } else {
            Err("This MDLObject cannot be downcasted to MDLVoxelArray")
        }
    }
}
impl INSObject for MDLVoxelArray {}
impl PNSObject for MDLVoxelArray {}
impl IMDLVoxelArray for MDLVoxelArray {}
pub trait IMDLVoxelArray: Sized + std::ops::Deref {
    unsafe fn initWithAsset_divisions_patchRadius_(
        &self,
        asset: MDLAsset,
        divisions: ::std::os::raw::c_int,
        patchRadius: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAsset : asset, divisions : divisions, patchRadius : patchRadius)
    }
    unsafe fn initWithData_boundingBox_voxelExtent_(
        &self,
        voxelData: NSData,
        boundingBox: MDLAxisAlignedBoundingBox,
        voxelExtent: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : voxelData, boundingBox : boundingBox, voxelExtent : voxelExtent)
    }
    unsafe fn initWithAsset_divisions_interiorShells_exteriorShells_patchRadius_(
        &self,
        asset: MDLAsset,
        divisions: ::std::os::raw::c_int,
        interiorShells: ::std::os::raw::c_int,
        exteriorShells: ::std::os::raw::c_int,
        patchRadius: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAsset : asset, divisions : divisions, interiorShells : interiorShells, exteriorShells : exteriorShells, patchRadius : patchRadius)
    }
    unsafe fn initWithAsset_divisions_interiorNBWidth_exteriorNBWidth_patchRadius_(
        &self,
        asset: MDLAsset,
        divisions: ::std::os::raw::c_int,
        interiorNBWidth: f32,
        exteriorNBWidth: f32,
        patchRadius: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAsset : asset, divisions : divisions, interiorNBWidth : interiorNBWidth, exteriorNBWidth : exteriorNBWidth, patchRadius : patchRadius)
    }
    unsafe fn voxelExistsAtIndex_allowAnyX_allowAnyY_allowAnyZ_allowAnyShell_(
        &self,
        index: MDLVoxelIndex,
        allowAnyX: BOOL,
        allowAnyY: BOOL,
        allowAnyZ: BOOL,
        allowAnyShell: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voxelExistsAtIndex : index, allowAnyX : allowAnyX, allowAnyY : allowAnyY, allowAnyZ : allowAnyZ, allowAnyShell : allowAnyShell)
    }
    unsafe fn voxelsWithinExtent_(&self, extent: MDLVoxelIndexExtent) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voxelsWithinExtent : extent)
    }
    unsafe fn voxelIndices(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voxelIndices)
    }
    unsafe fn setVoxelAtIndex_(&self, index: MDLVoxelIndex)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoxelAtIndex : index)
    }
    unsafe fn setVoxelsForMesh_divisions_patchRadius_(
        &self,
        mesh: MDLMesh,
        divisions: ::std::os::raw::c_int,
        patchRadius: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoxelsForMesh : mesh, divisions : divisions, patchRadius : patchRadius)
    }
    unsafe fn setVoxelsForMesh_divisions_interiorShells_exteriorShells_patchRadius_(
        &self,
        mesh: MDLMesh,
        divisions: ::std::os::raw::c_int,
        interiorShells: ::std::os::raw::c_int,
        exteriorShells: ::std::os::raw::c_int,
        patchRadius: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoxelsForMesh : mesh, divisions : divisions, interiorShells : interiorShells, exteriorShells : exteriorShells, patchRadius : patchRadius)
    }
    unsafe fn setVoxelsForMesh_divisions_interiorNBWidth_exteriorNBWidth_patchRadius_(
        &self,
        mesh: MDLMesh,
        divisions: ::std::os::raw::c_int,
        interiorNBWidth: f32,
        exteriorNBWidth: f32,
        patchRadius: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoxelsForMesh : mesh, divisions : divisions, interiorNBWidth : interiorNBWidth, exteriorNBWidth : exteriorNBWidth, patchRadius : patchRadius)
    }
    unsafe fn unionWithVoxels_(&self, voxels: MDLVoxelArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unionWithVoxels : voxels)
    }
    unsafe fn intersectWithVoxels_(&self, voxels: MDLVoxelArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intersectWithVoxels : voxels)
    }
    unsafe fn differenceWithVoxels_(&self, voxels: MDLVoxelArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, differenceWithVoxels : voxels)
    }
    unsafe fn indexOfSpatialLocation_(&self, location: vector_float3) -> MDLVoxelIndex
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexOfSpatialLocation : location)
    }
    unsafe fn spatialLocationOfIndex_(&self, index: MDLVoxelIndex) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, spatialLocationOfIndex : index)
    }
    unsafe fn voxelBoundingBoxAtIndex_(&self, index: MDLVoxelIndex) -> MDLAxisAlignedBoundingBox
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voxelBoundingBoxAtIndex : index)
    }
    unsafe fn convertToSignedShellField(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, convertToSignedShellField)
    }
    unsafe fn coarseMesh(&self) -> MDLMesh
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coarseMesh)
    }
    unsafe fn coarseMeshUsingAllocator_(&self, allocator: *mut u64) -> MDLMesh
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, coarseMeshUsingAllocator : allocator)
    }
    unsafe fn meshUsingAllocator_(&self, allocator: *mut u64) -> MDLMesh
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, meshUsingAllocator : allocator)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn voxelIndexExtent(&self) -> MDLVoxelIndexExtent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voxelIndexExtent)
    }
    unsafe fn boundingBox(&self) -> MDLAxisAlignedBoundingBox
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBox)
    }
    unsafe fn isValidSignedShellField(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isValidSignedShellField)
    }
    unsafe fn shellFieldInteriorThickness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shellFieldInteriorThickness)
    }
    unsafe fn setShellFieldInteriorThickness_(&self, shellFieldInteriorThickness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShellFieldInteriorThickness : shellFieldInteriorThickness)
    }
    unsafe fn shellFieldExteriorThickness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shellFieldExteriorThickness)
    }
    unsafe fn setShellFieldExteriorThickness_(&self, shellFieldExteriorThickness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShellFieldExteriorThickness : shellFieldExteriorThickness)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MDLUtility(pub id);
impl std::ops::Deref for MDLUtility {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MDLUtility {}
impl MDLUtility {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MDLUtility").unwrap(), alloc) })
    }
}
impl INSObject for MDLUtility {}
impl PNSObject for MDLUtility {}
impl std::convert::TryFrom<NSObject> for MDLUtility {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MDLUtility, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MDLUtility").unwrap()) };
        if is_kind_of {
            Ok(MDLUtility(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MDLUtility")
        }
    }
}
impl IMDLUtility for MDLUtility {}
pub trait IMDLUtility: Sized + std::ops::Deref {
    unsafe fn convertToUSDZ_writeToURL_(inputURL: NSURL, outputURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MDLUtility").unwrap(), convertToUSDZ : inputURL, writeToURL : outputURL)
    }
}
unsafe extern "C" {
    pub static kUTTypeAlembic: NSString;
}
unsafe extern "C" {
    pub static kUTType3dObject: NSString;
}
unsafe extern "C" {
    pub static kUTTypePolygon: NSString;
}
unsafe extern "C" {
    pub static kUTTypeStereolithography: NSString;
}
unsafe extern "C" {
    pub static kUTTypeUniversalSceneDescription: NSString;
}
unsafe extern "C" {
    pub static kUTTypeUniversalSceneDescriptionMobile: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeAnisotropy: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeBinormal: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeBitangent: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeColor: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeEdgeCrease: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeJointIndices: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeJointWeights: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeNormal: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeOcclusionValue: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributePosition: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeShadingBasisU: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeShadingBasisV: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeSubdivisionStencil: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeTangent: NSString;
}
unsafe extern "C" {
    pub static MDLVertexAttributeTextureCoordinate: NSString;
}

unsafe impl objc2::encode::RefEncode for simd_float4x4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for simd_float4x4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("simd_float4x4", &[]);
}
unsafe impl objc2::encode::RefEncode for simd_double4x4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for simd_double4x4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("simd_double4x4", &[]);
}
unsafe impl objc2::encode::RefEncode for simd_quatf {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for simd_quatf {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("simd_quatf", &[]);
}
unsafe impl objc2::encode::RefEncode for simd_quatd {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for simd_quatd {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("simd_quatd", &[]);
}
unsafe impl objc2::encode::RefEncode for MDLRelativeAssetResolver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLRelativeAssetResolver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLPathAssetResolver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLPathAssetResolver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLBundleAssetResolver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLBundleAssetResolver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAxisAlignedBoundingBox {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAxisAlignedBoundingBox {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDLAxisAlignedBoundingBox", &[]);
}
unsafe impl objc2::encode::RefEncode for MDLTransform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLObjectContainer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLObjectContainer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMeshBufferMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMeshBufferMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMeshBufferData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMeshBufferData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMeshBufferDataAllocator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMeshBufferDataAllocator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMeshBufferZoneDefault {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMeshBufferZoneDefault {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLVertexBufferLayout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLVertexBufferLayout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLVertexAttribute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLVertexAttribute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLVertexDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLVertexDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMatrix4x4Array {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMatrix4x4Array {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedScalarArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedScalarArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedVector3Array {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedVector3Array {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedQuaternionArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedQuaternionArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedScalar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedScalar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedVector2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedVector2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedVector3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedVector3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedVector4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedVector4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedQuaternion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedQuaternion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimatedMatrix4x4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimatedMatrix4x4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLSkeleton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLSkeleton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLPackedJointAnimation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLPackedJointAnimation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAnimationBindComponent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAnimationBindComponent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLCamera {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLCamera {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLStereoscopicCamera {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLStereoscopicCamera {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLLight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLLight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLPhysicallyPlausibleLight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLPhysicallyPlausibleLight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLAreaLight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLAreaLight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLPhotometricLight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLPhotometricLight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLLightProbe {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLLightProbe {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTextureFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTextureFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTextureSampler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTextureSampler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMaterialProperty {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMaterialProperty {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMaterialPropertyConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMaterialPropertyConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMaterialPropertyNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMaterialPropertyNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMaterialPropertyGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMaterialPropertyGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLScatteringFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLScatteringFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLPhysicallyPlausibleScatteringFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLPhysicallyPlausibleScatteringFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMaterial {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMaterial {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLSubmeshTopology {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLSubmeshTopology {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLSubmesh {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLSubmesh {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLVertexAttributeData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLVertexAttributeData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLMesh {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLMesh {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLURLTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLURLTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLCheckerboardTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLCheckerboardTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLSkyCubeTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLSkyCubeTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLColorSwatchTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLColorSwatchTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLNoiseTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLNoiseTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLNormalMapTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLNormalMapTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTransformRotateXOp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransformRotateXOp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTransformRotateYOp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransformRotateYOp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTransformRotateZOp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransformRotateZOp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTransformRotateOp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransformRotateOp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTransformTranslateOp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransformTranslateOp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTransformScaleOp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransformScaleOp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTransformMatrixOp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransformMatrixOp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTransformOrientOp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransformOrientOp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLTransformStack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLTransformStack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLVoxelIndexExtent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLVoxelIndexExtent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MDLVoxelIndexExtent", &[]);
}
unsafe impl objc2::encode::RefEncode for MDLVoxelArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLVoxelArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MDLUtility {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MDLUtility {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
