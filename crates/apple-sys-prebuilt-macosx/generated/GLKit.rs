#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::ModelIO::*;
#[allow(unused_imports)]
use crate::OpenGL::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKMeshBufferAllocator(pub id);
impl std::ops::Deref for GLKMeshBufferAllocator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKMeshBufferAllocator {}
impl GLKMeshBufferAllocator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKMeshBufferAllocator").unwrap(), alloc) })
    }
}
impl PMDLMeshBufferAllocator for GLKMeshBufferAllocator {}
impl INSObject for GLKMeshBufferAllocator {}
impl PNSObject for GLKMeshBufferAllocator {}
impl std::convert::TryFrom<NSObject> for GLKMeshBufferAllocator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GLKMeshBufferAllocator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKMeshBufferAllocator").unwrap()) };
        if is_kind_of {
            Ok(GLKMeshBufferAllocator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GLKMeshBufferAllocator")
        }
    }
}
impl IGLKMeshBufferAllocator for GLKMeshBufferAllocator {}
pub trait IGLKMeshBufferAllocator: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKMeshBuffer(pub id);
impl std::ops::Deref for GLKMeshBuffer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKMeshBuffer {}
impl GLKMeshBuffer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKMeshBuffer").unwrap(), alloc) })
    }
}
impl PMDLMeshBuffer for GLKMeshBuffer {}
impl INSObject for GLKMeshBuffer {}
impl PNSObject for GLKMeshBuffer {}
impl std::convert::TryFrom<NSObject> for GLKMeshBuffer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GLKMeshBuffer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKMeshBuffer").unwrap()) };
        if is_kind_of {
            Ok(GLKMeshBuffer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GLKMeshBuffer")
        }
    }
}
impl IGLKMeshBuffer for GLKMeshBuffer {}
pub trait IGLKMeshBuffer: Sized + std::ops::Deref {
    unsafe fn length(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn allocator(&self) -> GLKMeshBufferAllocator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocator)
    }
    unsafe fn glBufferName(&self) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, glBufferName)
    }
    unsafe fn offset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
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
pub struct GLKSubmesh(pub id);
impl std::ops::Deref for GLKSubmesh {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKSubmesh {}
impl GLKSubmesh {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKSubmesh").unwrap(), alloc) })
    }
}
impl INSObject for GLKSubmesh {}
impl PNSObject for GLKSubmesh {}
impl std::convert::TryFrom<NSObject> for GLKSubmesh {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GLKSubmesh, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKSubmesh").unwrap()) };
        if is_kind_of {
            Ok(GLKSubmesh(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GLKSubmesh")
        }
    }
}
impl IGLKSubmesh for GLKSubmesh {}
pub trait IGLKSubmesh: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn type_(&self) -> GLenum
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn mode(&self) -> GLenum
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn elementCount(&self) -> GLsizei
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementCount)
    }
    unsafe fn elementBuffer(&self) -> GLKMeshBuffer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementBuffer)
    }
    unsafe fn mesh(&self) -> GLKMesh
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mesh)
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
pub struct GLKMesh(pub id);
impl std::ops::Deref for GLKMesh {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKMesh {}
impl GLKMesh {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKMesh").unwrap(), alloc) })
    }
}
impl INSObject for GLKMesh {}
impl PNSObject for GLKMesh {}
impl std::convert::TryFrom<NSObject> for GLKMesh {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GLKMesh, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKMesh").unwrap()) };
        if is_kind_of {
            Ok(GLKMesh(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GLKMesh")
        }
    }
}
impl IGLKMesh for GLKMesh {}
pub trait IGLKMesh: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMesh_error_(&self, mesh: MDLMesh, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMesh : mesh, error : error)
    }
    unsafe fn vertexCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexCount)
    }
    unsafe fn vertexBuffers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexBuffers)
    }
    unsafe fn vertexDescriptor(&self) -> MDLVertexDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexDescriptor)
    }
    unsafe fn submeshes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, submeshes)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn newMeshesFromAsset_sourceMeshes_error_(
        asset: MDLAsset,
        sourceMeshes: *mut NSArray,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GLKMesh").unwrap(), newMeshesFromAsset : asset, sourceMeshes : sourceMeshes, error : error)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKVertexAttributeParameters {
    pub type_: GLenum,
    pub size: GLint,
    pub normalized: GLboolean,
}
pub type GLKVertexAttributeParameters = _GLKVertexAttributeParameters;
pub trait PGLKNamedEffect: Sized + std::ops::Deref {
    unsafe fn prepareToDraw(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareToDraw)
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _GLKMatrix2 {
    pub __bindgen_anon_1: _GLKMatrix2__bindgen_ty_1,
    pub m2: [[f32; 2usize]; 2usize],
    pub m: [f32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKMatrix2__bindgen_ty_1 {
    pub m00: f32,
    pub m01: f32,
    pub m10: f32,
    pub m11: f32,
}
pub type GLKMatrix2 = _GLKMatrix2;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _GLKMatrix3 {
    pub __bindgen_anon_1: _GLKMatrix3__bindgen_ty_1,
    pub m: [f32; 9usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKMatrix3__bindgen_ty_1 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
}
pub type GLKMatrix3 = _GLKMatrix3;
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union _GLKMatrix4 {
    pub __bindgen_anon_1: _GLKMatrix4__bindgen_ty_1,
    pub m: [f32; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKMatrix4__bindgen_ty_1 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}
pub type GLKMatrix4 = _GLKMatrix4;
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union _GLKVector2 {
    pub __bindgen_anon_1: _GLKVector2__bindgen_ty_1,
    pub __bindgen_anon_2: _GLKVector2__bindgen_ty_2,
    pub v: [f32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKVector2__bindgen_ty_1 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKVector2__bindgen_ty_2 {
    pub s: f32,
    pub t: f32,
}
pub type GLKVector2 = _GLKVector2;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _GLKVector3 {
    pub __bindgen_anon_1: _GLKVector3__bindgen_ty_1,
    pub __bindgen_anon_2: _GLKVector3__bindgen_ty_2,
    pub __bindgen_anon_3: _GLKVector3__bindgen_ty_3,
    pub v: [f32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKVector3__bindgen_ty_1 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKVector3__bindgen_ty_2 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKVector3__bindgen_ty_3 {
    pub s: f32,
    pub t: f32,
    pub p: f32,
}
pub type GLKVector3 = _GLKVector3;
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union _GLKVector4 {
    pub __bindgen_anon_1: _GLKVector4__bindgen_ty_1,
    pub __bindgen_anon_2: _GLKVector4__bindgen_ty_2,
    pub __bindgen_anon_3: _GLKVector4__bindgen_ty_3,
    pub v: [f32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKVector4__bindgen_ty_1 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKVector4__bindgen_ty_2 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKVector4__bindgen_ty_3 {
    pub s: f32,
    pub t: f32,
    pub p: f32,
    pub q: f32,
}
pub type GLKVector4 = _GLKVector4;
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union _GLKQuaternion {
    pub __bindgen_anon_1: _GLKQuaternion__bindgen_ty_1,
    pub __bindgen_anon_2: _GLKQuaternion__bindgen_ty_2,
    pub q: [f32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKQuaternion__bindgen_ty_1 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _GLKQuaternion__bindgen_ty_2 {
    pub v: GLKVector3,
    pub s: f32,
}
pub type GLKQuaternion = _GLKQuaternion;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GLKMatrixStack {
    _unused: [u8; 0],
}
pub type GLKMatrixStackRef = *mut _GLKMatrixStack;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLKEffectPropertyPrv {
    _unused: [u8; 0],
}
pub type GLKEffectPropertyPrvPtr = *mut GLKEffectPropertyPrv;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKEffectProperty(pub id);
impl std::ops::Deref for GLKEffectProperty {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKEffectProperty {}
impl GLKEffectProperty {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKEffectProperty").unwrap(), alloc) })
    }
}
impl INSObject for GLKEffectProperty {}
impl PNSObject for GLKEffectProperty {}
impl std::convert::TryFrom<NSObject> for GLKEffectProperty {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GLKEffectProperty, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKEffectProperty").unwrap()) };
        if is_kind_of {
            Ok(GLKEffectProperty(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GLKEffectProperty")
        }
    }
}
impl IGLKEffectProperty for GLKEffectProperty {}
pub trait IGLKEffectProperty: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKEffectPropertyTransform(pub id);
impl std::ops::Deref for GLKEffectPropertyTransform {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKEffectPropertyTransform {}
impl GLKEffectPropertyTransform {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKEffectPropertyTransform").unwrap(), alloc) })
    }
}
impl IGLKEffectProperty for GLKEffectPropertyTransform {}
impl From<GLKEffectPropertyTransform> for GLKEffectProperty {
    fn from(child: GLKEffectPropertyTransform) -> GLKEffectProperty {
        GLKEffectProperty(child.0)
    }
}
impl std::convert::TryFrom<GLKEffectProperty> for GLKEffectPropertyTransform {
    type Error = &'static str;
    fn try_from(parent: GLKEffectProperty) -> Result<GLKEffectPropertyTransform, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKEffectPropertyTransform").unwrap()) };
        if is_kind_of {
            Ok(GLKEffectPropertyTransform(parent.0))
        } else {
            Err("This GLKEffectProperty cannot be downcasted to GLKEffectPropertyTransform")
        }
    }
}
impl INSObject for GLKEffectPropertyTransform {}
impl PNSObject for GLKEffectPropertyTransform {}
impl IGLKEffectPropertyTransform for GLKEffectPropertyTransform {}
pub trait IGLKEffectPropertyTransform: Sized + std::ops::Deref {
    unsafe fn modelviewMatrix(&self) -> GLKMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelviewMatrix)
    }
    unsafe fn setModelviewMatrix_(&self, modelviewMatrix: GLKMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModelviewMatrix : modelviewMatrix)
    }
    unsafe fn projectionMatrix(&self) -> GLKMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projectionMatrix)
    }
    unsafe fn setProjectionMatrix_(&self, projectionMatrix: GLKMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProjectionMatrix : projectionMatrix)
    }
    unsafe fn normalMatrix(&self) -> GLKMatrix3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalMatrix)
    }
}
pub type GLKLightingType = GLint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKEffectPropertyLight(pub id);
impl std::ops::Deref for GLKEffectPropertyLight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKEffectPropertyLight {}
impl GLKEffectPropertyLight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKEffectPropertyLight").unwrap(), alloc) })
    }
}
impl IGLKEffectProperty for GLKEffectPropertyLight {}
impl std::convert::TryFrom<GLKEffectProperty> for GLKEffectPropertyLight {
    type Error = &'static str;
    fn try_from(parent: GLKEffectProperty) -> Result<GLKEffectPropertyLight, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKEffectPropertyLight").unwrap()) };
        if is_kind_of {
            Ok(GLKEffectPropertyLight(parent.0))
        } else {
            Err("This GLKEffectProperty cannot be downcasted to GLKEffectPropertyLight")
        }
    }
}
impl INSObject for GLKEffectPropertyLight {}
impl PNSObject for GLKEffectPropertyLight {}
impl IGLKEffectPropertyLight for GLKEffectPropertyLight {}
pub trait IGLKEffectPropertyLight: Sized + std::ops::Deref {
    unsafe fn enabled(&self) -> GLboolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabled)
    }
    unsafe fn setEnabled_(&self, enabled: GLboolean)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn position(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn ambientColor(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ambientColor)
    }
    unsafe fn setAmbientColor_(&self, ambientColor: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmbientColor : ambientColor)
    }
    unsafe fn diffuseColor(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diffuseColor)
    }
    unsafe fn setDiffuseColor_(&self, diffuseColor: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiffuseColor : diffuseColor)
    }
    unsafe fn specularColor(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularColor)
    }
    unsafe fn setSpecularColor_(&self, specularColor: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecularColor : specularColor)
    }
    unsafe fn spotDirection(&self) -> GLKVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spotDirection)
    }
    unsafe fn setSpotDirection_(&self, spotDirection: GLKVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpotDirection : spotDirection)
    }
    unsafe fn spotExponent(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spotExponent)
    }
    unsafe fn setSpotExponent_(&self, spotExponent: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpotExponent : spotExponent)
    }
    unsafe fn spotCutoff(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spotCutoff)
    }
    unsafe fn setSpotCutoff_(&self, spotCutoff: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpotCutoff : spotCutoff)
    }
    unsafe fn constantAttenuation(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constantAttenuation)
    }
    unsafe fn setConstantAttenuation_(&self, constantAttenuation: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstantAttenuation : constantAttenuation)
    }
    unsafe fn linearAttenuation(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linearAttenuation)
    }
    unsafe fn setLinearAttenuation_(&self, linearAttenuation: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinearAttenuation : linearAttenuation)
    }
    unsafe fn quadraticAttenuation(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quadraticAttenuation)
    }
    unsafe fn setQuadraticAttenuation_(&self, quadraticAttenuation: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuadraticAttenuation : quadraticAttenuation)
    }
    unsafe fn transform(&self) -> GLKEffectPropertyTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn setTransform_(&self, transform: GLKEffectPropertyTransform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransform : transform)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKEffectPropertyMaterial(pub id);
impl std::ops::Deref for GLKEffectPropertyMaterial {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKEffectPropertyMaterial {}
impl GLKEffectPropertyMaterial {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKEffectPropertyMaterial").unwrap(), alloc) })
    }
}
impl IGLKEffectProperty for GLKEffectPropertyMaterial {}
impl std::convert::TryFrom<GLKEffectProperty> for GLKEffectPropertyMaterial {
    type Error = &'static str;
    fn try_from(parent: GLKEffectProperty) -> Result<GLKEffectPropertyMaterial, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKEffectPropertyMaterial").unwrap()) };
        if is_kind_of {
            Ok(GLKEffectPropertyMaterial(parent.0))
        } else {
            Err("This GLKEffectProperty cannot be downcasted to GLKEffectPropertyMaterial")
        }
    }
}
impl INSObject for GLKEffectPropertyMaterial {}
impl PNSObject for GLKEffectPropertyMaterial {}
impl IGLKEffectPropertyMaterial for GLKEffectPropertyMaterial {}
pub trait IGLKEffectPropertyMaterial: Sized + std::ops::Deref {
    unsafe fn ambientColor(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ambientColor)
    }
    unsafe fn setAmbientColor_(&self, ambientColor: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmbientColor : ambientColor)
    }
    unsafe fn diffuseColor(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diffuseColor)
    }
    unsafe fn setDiffuseColor_(&self, diffuseColor: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiffuseColor : diffuseColor)
    }
    unsafe fn specularColor(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularColor)
    }
    unsafe fn setSpecularColor_(&self, specularColor: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecularColor : specularColor)
    }
    unsafe fn emissiveColor(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emissiveColor)
    }
    unsafe fn setEmissiveColor_(&self, emissiveColor: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmissiveColor : emissiveColor)
    }
    unsafe fn shininess(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shininess)
    }
    unsafe fn setShininess_(&self, shininess: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShininess : shininess)
    }
}
pub type GLKTextureTarget = GLenum;
pub type GLKTextureEnvMode = GLint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKEffectPropertyTexture(pub id);
impl std::ops::Deref for GLKEffectPropertyTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKEffectPropertyTexture {}
impl GLKEffectPropertyTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKEffectPropertyTexture").unwrap(), alloc) })
    }
}
impl IGLKEffectProperty for GLKEffectPropertyTexture {}
impl std::convert::TryFrom<GLKEffectProperty> for GLKEffectPropertyTexture {
    type Error = &'static str;
    fn try_from(parent: GLKEffectProperty) -> Result<GLKEffectPropertyTexture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKEffectPropertyTexture").unwrap()) };
        if is_kind_of {
            Ok(GLKEffectPropertyTexture(parent.0))
        } else {
            Err("This GLKEffectProperty cannot be downcasted to GLKEffectPropertyTexture")
        }
    }
}
impl INSObject for GLKEffectPropertyTexture {}
impl PNSObject for GLKEffectPropertyTexture {}
impl IGLKEffectPropertyTexture for GLKEffectPropertyTexture {}
pub trait IGLKEffectPropertyTexture: Sized + std::ops::Deref {
    unsafe fn enabled(&self) -> GLboolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabled)
    }
    unsafe fn setEnabled_(&self, enabled: GLboolean)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn name(&self) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: GLuint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn target(&self) -> GLKTextureTarget
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: GLKTextureTarget)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn envMode(&self) -> GLKTextureEnvMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, envMode)
    }
    unsafe fn setEnvMode_(&self, envMode: GLKTextureEnvMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnvMode : envMode)
    }
}
pub type GLKFogMode = GLint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKEffectPropertyFog(pub id);
impl std::ops::Deref for GLKEffectPropertyFog {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKEffectPropertyFog {}
impl GLKEffectPropertyFog {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKEffectPropertyFog").unwrap(), alloc) })
    }
}
impl IGLKEffectProperty for GLKEffectPropertyFog {}
impl std::convert::TryFrom<GLKEffectProperty> for GLKEffectPropertyFog {
    type Error = &'static str;
    fn try_from(parent: GLKEffectProperty) -> Result<GLKEffectPropertyFog, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKEffectPropertyFog").unwrap()) };
        if is_kind_of {
            Ok(GLKEffectPropertyFog(parent.0))
        } else {
            Err("This GLKEffectProperty cannot be downcasted to GLKEffectPropertyFog")
        }
    }
}
impl INSObject for GLKEffectPropertyFog {}
impl PNSObject for GLKEffectPropertyFog {}
impl IGLKEffectPropertyFog for GLKEffectPropertyFog {}
pub trait IGLKEffectPropertyFog: Sized + std::ops::Deref {
    unsafe fn enabled(&self) -> GLboolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabled)
    }
    unsafe fn setEnabled_(&self, enabled: GLboolean)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn mode(&self) -> GLint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn setMode_(&self, mode: GLint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMode : mode)
    }
    unsafe fn color(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn density(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, density)
    }
    unsafe fn setDensity_(&self, density: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDensity : density)
    }
    unsafe fn start(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn setStart_(&self, start: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStart : start)
    }
    unsafe fn end(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, end)
    }
    unsafe fn setEnd_(&self, end: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnd : end)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKBaseEffect(pub id);
impl std::ops::Deref for GLKBaseEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKBaseEffect {}
impl GLKBaseEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKBaseEffect").unwrap(), alloc) })
    }
}
impl PGLKNamedEffect for GLKBaseEffect {}
impl INSObject for GLKBaseEffect {}
impl PNSObject for GLKBaseEffect {}
impl std::convert::TryFrom<NSObject> for GLKBaseEffect {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GLKBaseEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKBaseEffect").unwrap()) };
        if is_kind_of {
            Ok(GLKBaseEffect(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GLKBaseEffect")
        }
    }
}
impl IGLKBaseEffect for GLKBaseEffect {}
pub trait IGLKBaseEffect: Sized + std::ops::Deref {
    unsafe fn prepareToDraw(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareToDraw)
    }
    unsafe fn colorMaterialEnabled(&self) -> GLboolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorMaterialEnabled)
    }
    unsafe fn setColorMaterialEnabled_(&self, colorMaterialEnabled: GLboolean)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorMaterialEnabled : colorMaterialEnabled)
    }
    unsafe fn lightModelTwoSided(&self) -> GLboolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightModelTwoSided)
    }
    unsafe fn setLightModelTwoSided_(&self, lightModelTwoSided: GLboolean)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightModelTwoSided : lightModelTwoSided)
    }
    unsafe fn useConstantColor(&self) -> GLboolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, useConstantColor)
    }
    unsafe fn setUseConstantColor_(&self, useConstantColor: GLboolean)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUseConstantColor : useConstantColor)
    }
    unsafe fn transform(&self) -> GLKEffectPropertyTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn light0(&self) -> GLKEffectPropertyLight
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, light0)
    }
    unsafe fn light1(&self) -> GLKEffectPropertyLight
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, light1)
    }
    unsafe fn light2(&self) -> GLKEffectPropertyLight
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, light2)
    }
    unsafe fn lightingType(&self) -> GLKLightingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightingType)
    }
    unsafe fn setLightingType_(&self, lightingType: GLKLightingType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightingType : lightingType)
    }
    unsafe fn lightModelAmbientColor(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightModelAmbientColor)
    }
    unsafe fn setLightModelAmbientColor_(&self, lightModelAmbientColor: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightModelAmbientColor : lightModelAmbientColor)
    }
    unsafe fn material(&self) -> GLKEffectPropertyMaterial
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, material)
    }
    unsafe fn texture2d0(&self) -> GLKEffectPropertyTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texture2d0)
    }
    unsafe fn texture2d1(&self) -> GLKEffectPropertyTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texture2d1)
    }
    unsafe fn textureOrder(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureOrder)
    }
    unsafe fn setTextureOrder_(&self, textureOrder: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureOrder : textureOrder)
    }
    unsafe fn constantColor(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constantColor)
    }
    unsafe fn setConstantColor_(&self, constantColor: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstantColor : constantColor)
    }
    unsafe fn fog(&self) -> GLKEffectPropertyFog
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fog)
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
pub struct GLKReflectionMapEffect(pub id);
impl std::ops::Deref for GLKReflectionMapEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKReflectionMapEffect {}
impl GLKReflectionMapEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKReflectionMapEffect").unwrap(), alloc) })
    }
}
impl PGLKNamedEffect for GLKReflectionMapEffect {}
impl IGLKBaseEffect for GLKReflectionMapEffect {}
impl From<GLKReflectionMapEffect> for GLKBaseEffect {
    fn from(child: GLKReflectionMapEffect) -> GLKBaseEffect {
        GLKBaseEffect(child.0)
    }
}
impl std::convert::TryFrom<GLKBaseEffect> for GLKReflectionMapEffect {
    type Error = &'static str;
    fn try_from(parent: GLKBaseEffect) -> Result<GLKReflectionMapEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKReflectionMapEffect").unwrap()) };
        if is_kind_of {
            Ok(GLKReflectionMapEffect(parent.0))
        } else {
            Err("This GLKBaseEffect cannot be downcasted to GLKReflectionMapEffect")
        }
    }
}
impl INSObject for GLKReflectionMapEffect {}
impl PNSObject for GLKReflectionMapEffect {}
impl IGLKReflectionMapEffect for GLKReflectionMapEffect {}
pub trait IGLKReflectionMapEffect: Sized + std::ops::Deref {
    unsafe fn prepareToDraw(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareToDraw)
    }
    unsafe fn textureCubeMap(&self) -> GLKEffectPropertyTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureCubeMap)
    }
    unsafe fn matrix(&self) -> GLKMatrix3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matrix)
    }
    unsafe fn setMatrix_(&self, matrix: GLKMatrix3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatrix : matrix)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKSkyboxEffect(pub id);
impl std::ops::Deref for GLKSkyboxEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKSkyboxEffect {}
impl GLKSkyboxEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKSkyboxEffect").unwrap(), alloc) })
    }
}
impl PGLKNamedEffect for GLKSkyboxEffect {}
impl INSObject for GLKSkyboxEffect {}
impl PNSObject for GLKSkyboxEffect {}
impl std::convert::TryFrom<NSObject> for GLKSkyboxEffect {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GLKSkyboxEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKSkyboxEffect").unwrap()) };
        if is_kind_of {
            Ok(GLKSkyboxEffect(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GLKSkyboxEffect")
        }
    }
}
impl IGLKSkyboxEffect for GLKSkyboxEffect {}
pub trait IGLKSkyboxEffect: Sized + std::ops::Deref {
    unsafe fn prepareToDraw(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareToDraw)
    }
    unsafe fn draw(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, draw)
    }
    unsafe fn center(&self) -> GLKVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: GLKVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn xSize(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xSize)
    }
    unsafe fn setXSize_(&self, xSize: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXSize : xSize)
    }
    unsafe fn ySize(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ySize)
    }
    unsafe fn setYSize_(&self, ySize: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setYSize : ySize)
    }
    unsafe fn zSize(&self) -> GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zSize)
    }
    unsafe fn setZSize_(&self, zSize: GLfloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZSize : zSize)
    }
    unsafe fn textureCubeMap(&self) -> GLKEffectPropertyTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureCubeMap)
    }
    unsafe fn transform(&self) -> GLKEffectPropertyTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
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
pub type GLKVertexAttrib = GLint;
pub type GLKTextureLoaderError = GLuint;
pub type GLKTextureInfoAlphaState = GLint;
pub type GLKTextureInfoOrigin = GLint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKTextureInfo(pub id);
impl std::ops::Deref for GLKTextureInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKTextureInfo {}
impl GLKTextureInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureInfo").unwrap(), alloc) })
    }
}
impl PNSCopying for GLKTextureInfo {}
impl INSObject for GLKTextureInfo {}
impl PNSObject for GLKTextureInfo {}
impl std::convert::TryFrom<NSObject> for GLKTextureInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GLKTextureInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKTextureInfo").unwrap()) };
        if is_kind_of {
            Ok(GLKTextureInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GLKTextureInfo")
        }
    }
}
impl IGLKTextureInfo for GLKTextureInfo {}
pub trait IGLKTextureInfo: Sized + std::ops::Deref {
    unsafe fn name(&self) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn target(&self) -> GLenum
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn width(&self) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn depth(&self) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depth)
    }
    unsafe fn alphaState(&self) -> GLKTextureInfoAlphaState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaState)
    }
    unsafe fn textureOrigin(&self) -> GLKTextureInfoOrigin
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureOrigin)
    }
    unsafe fn containsMipmaps(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containsMipmaps)
    }
    unsafe fn mimapLevelCount(&self) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mimapLevelCount)
    }
    unsafe fn arrayLength(&self) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrayLength)
    }
}
pub type GLKTextureLoaderCallback = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GLKTextureLoader(pub id);
impl std::ops::Deref for GLKTextureLoader {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GLKTextureLoader {}
impl GLKTextureLoader {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap(), alloc) })
    }
}
impl INSObject for GLKTextureLoader {}
impl PNSObject for GLKTextureLoader {}
impl std::convert::TryFrom<NSObject> for GLKTextureLoader {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GLKTextureLoader, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap()) };
        if is_kind_of {
            Ok(GLKTextureLoader(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GLKTextureLoader")
        }
    }
}
impl IGLKTextureLoader for GLKTextureLoader {}
pub trait IGLKTextureLoader: Sized + std::ops::Deref {
    unsafe fn initWithShareContext_(&self, context: NSOpenGLContext) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithShareContext : context)
    }
    unsafe fn textureWithContentsOfFile_options_queue_completionHandler_(
        &self,
        path: NSString,
        options: NSDictionary,
        queue: NSObject,
        block: GLKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureWithContentsOfFile : path, options : options, queue : queue, completionHandler : block)
    }
    unsafe fn textureWithContentsOfURL_options_queue_completionHandler_(
        &self,
        url: NSURL,
        options: NSDictionary,
        queue: NSObject,
        block: GLKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureWithContentsOfURL : url, options : options, queue : queue, completionHandler : block)
    }
    unsafe fn textureWithName_scaleFactor_bundle_options_queue_completionHandler_(
        &self,
        name: NSString,
        scaleFactor: CGFloat,
        bundle: NSBundle,
        options: NSDictionary,
        queue: NSObject,
        block: GLKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureWithName : name, scaleFactor : scaleFactor, bundle : bundle, options : options, queue : queue, completionHandler : block)
    }
    unsafe fn textureWithContentsOfData_options_queue_completionHandler_(
        &self,
        data: NSData,
        options: NSDictionary,
        queue: NSObject,
        block: GLKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureWithContentsOfData : data, options : options, queue : queue, completionHandler : block)
    }
    unsafe fn textureWithCGImage_options_queue_completionHandler_(
        &self,
        cgImage: CGImageRef,
        options: NSDictionary,
        queue: NSObject,
        block: GLKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureWithCGImage : cgImage, options : options, queue : queue, completionHandler : block)
    }
    unsafe fn cubeMapWithContentsOfFiles_options_queue_completionHandler_(
        &self,
        paths: NSArray,
        options: NSDictionary,
        queue: NSObject,
        block: GLKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cubeMapWithContentsOfFiles : paths, options : options, queue : queue, completionHandler : block)
    }
    unsafe fn cubeMapWithContentsOfFile_options_queue_completionHandler_(
        &self,
        path: NSString,
        options: NSDictionary,
        queue: NSObject,
        block: GLKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cubeMapWithContentsOfFile : path, options : options, queue : queue, completionHandler : block)
    }
    unsafe fn cubeMapWithContentsOfURL_options_queue_completionHandler_(
        &self,
        url: NSURL,
        options: NSDictionary,
        queue: NSObject,
        block: GLKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cubeMapWithContentsOfURL : url, options : options, queue : queue, completionHandler : block)
    }
    unsafe fn textureWithContentsOfFile_options_error_(
        path: NSString,
        options: NSDictionary,
        outError: *mut NSError,
    ) -> GLKTextureInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap(), textureWithContentsOfFile : path, options : options, error : outError)
    }
    unsafe fn textureWithContentsOfURL_options_error_(
        url: NSURL,
        options: NSDictionary,
        outError: *mut NSError,
    ) -> GLKTextureInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap(), textureWithContentsOfURL : url, options : options, error : outError)
    }
    unsafe fn textureWithName_scaleFactor_bundle_options_error_(
        name: NSString,
        scaleFactor: CGFloat,
        bundle: NSBundle,
        options: NSDictionary,
        outError: *mut NSError,
    ) -> GLKTextureInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap(), textureWithName : name, scaleFactor : scaleFactor, bundle : bundle, options : options, error : outError)
    }
    unsafe fn textureWithContentsOfData_options_error_(
        data: NSData,
        options: NSDictionary,
        outError: *mut NSError,
    ) -> GLKTextureInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap(), textureWithContentsOfData : data, options : options, error : outError)
    }
    unsafe fn textureWithCGImage_options_error_(
        cgImage: CGImageRef,
        options: NSDictionary,
        outError: *mut NSError,
    ) -> GLKTextureInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap(), textureWithCGImage : cgImage, options : options, error : outError)
    }
    unsafe fn cubeMapWithContentsOfFiles_options_error_(
        paths: NSArray,
        options: NSDictionary,
        outError: *mut NSError,
    ) -> GLKTextureInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap(), cubeMapWithContentsOfFiles : paths, options : options, error : outError)
    }
    unsafe fn cubeMapWithContentsOfFile_options_error_(
        path: NSString,
        options: NSDictionary,
        outError: *mut NSError,
    ) -> GLKTextureInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap(), cubeMapWithContentsOfFile : path, options : options, error : outError)
    }
    unsafe fn cubeMapWithContentsOfURL_options_error_(
        url: NSURL,
        options: NSDictionary,
        outError: *mut NSError,
    ) -> GLKTextureInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GLKTextureLoader").unwrap(), cubeMapWithContentsOfURL : url, options : options, error : outError)
    }
}
unsafe extern "C" {
    pub static kGLKModelErrorDomain: NSString;
}
unsafe extern "C" {
    pub static kGLKModelErrorKey: NSString;
}
unsafe extern "C" {
    pub fn GLKVertexAttributeParametersFromModelIO(
        vertexFormat: MDLVertexFormat,
    ) -> GLKVertexAttributeParameters;
}
unsafe extern "C" {
    pub static GLKQuaternionIdentity: GLKQuaternion;
}
unsafe extern "C" {
    pub fn GLKQuaternionMakeWithMatrix3(matrix: GLKMatrix3) -> GLKQuaternion;
}
unsafe extern "C" {
    pub fn GLKQuaternionMakeWithMatrix4(matrix: GLKMatrix4) -> GLKQuaternion;
}
unsafe extern "C" {
    pub fn GLKQuaternionAngle(quaternion: GLKQuaternion) -> f32;
}
unsafe extern "C" {
    pub fn GLKQuaternionAxis(quaternion: GLKQuaternion) -> GLKVector3;
}
unsafe extern "C" {
    pub fn GLKQuaternionSlerp(
        quaternionStart: GLKQuaternion,
        quaternionEnd: GLKQuaternion,
        t: f32,
    ) -> GLKQuaternion;
}
unsafe extern "C" {
    pub fn GLKQuaternionRotateVector3Array(
        quaternion: GLKQuaternion,
        vectors: *mut GLKVector3,
        vectorCount: usize,
    );
}
unsafe extern "C" {
    pub fn GLKQuaternionRotateVector4Array(
        quaternion: GLKQuaternion,
        vectors: *mut GLKVector4,
        vectorCount: usize,
    );
}
unsafe extern "C" {
    pub static GLKMatrix3Identity: GLKMatrix3;
}
unsafe extern "C" {
    pub fn GLKMatrix3Invert(matrix: GLKMatrix3, isInvertible: *mut bool) -> GLKMatrix3;
}
unsafe extern "C" {
    pub fn GLKMatrix3InvertAndTranspose(matrix: GLKMatrix3, isInvertible: *mut bool) -> GLKMatrix3;
}
unsafe extern "C" {
    pub static GLKMatrix4Identity: GLKMatrix4;
}
unsafe extern "C" {
    pub fn GLKMatrix4Invert(matrix: GLKMatrix4, isInvertible: *mut bool) -> GLKMatrix4;
}
unsafe extern "C" {
    pub fn GLKMatrix4InvertAndTranspose(matrix: GLKMatrix4, isInvertible: *mut bool) -> GLKMatrix4;
}
unsafe extern "C" {
    pub fn GLKMatrixStackCreate(alloc: CFAllocatorRef) -> GLKMatrixStackRef;
}
unsafe extern "C" {
    pub fn GLKMatrixStackGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn GLKMatrixStackPush(stack: GLKMatrixStackRef);
}
unsafe extern "C" {
    pub fn GLKMatrixStackPop(stack: GLKMatrixStackRef);
}
unsafe extern "C" {
    pub fn GLKMatrixStackSize(stack: GLKMatrixStackRef) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn GLKMatrixStackLoadMatrix4(stack: GLKMatrixStackRef, matrix: GLKMatrix4);
}
unsafe extern "C" {
    pub fn GLKMatrixStackGetMatrix4(stack: GLKMatrixStackRef) -> GLKMatrix4;
}
unsafe extern "C" {
    pub fn GLKMatrixStackGetMatrix3(stack: GLKMatrixStackRef) -> GLKMatrix3;
}
unsafe extern "C" {
    pub fn GLKMatrixStackGetMatrix2(stack: GLKMatrixStackRef) -> GLKMatrix2;
}
unsafe extern "C" {
    pub fn GLKMatrixStackGetMatrix4Inverse(stack: GLKMatrixStackRef) -> GLKMatrix4;
}
unsafe extern "C" {
    pub fn GLKMatrixStackGetMatrix4InverseTranspose(stack: GLKMatrixStackRef) -> GLKMatrix4;
}
unsafe extern "C" {
    pub fn GLKMatrixStackGetMatrix3Inverse(stack: GLKMatrixStackRef) -> GLKMatrix3;
}
unsafe extern "C" {
    pub fn GLKMatrixStackGetMatrix3InverseTranspose(stack: GLKMatrixStackRef) -> GLKMatrix3;
}
unsafe extern "C" {
    pub fn GLKMatrixStackMultiplyMatrix4(stack: GLKMatrixStackRef, matrix: GLKMatrix4);
}
unsafe extern "C" {
    pub fn GLKMatrixStackMultiplyMatrixStack(
        stackLeft: GLKMatrixStackRef,
        stackRight: GLKMatrixStackRef,
    );
}
unsafe extern "C" {
    pub fn GLKMatrixStackTranslate(stack: GLKMatrixStackRef, tx: f32, ty: f32, tz: f32);
}
unsafe extern "C" {
    pub fn GLKMatrixStackTranslateWithVector3(
        stack: GLKMatrixStackRef,
        translationVector: GLKVector3,
    );
}
unsafe extern "C" {
    pub fn GLKMatrixStackTranslateWithVector4(
        stack: GLKMatrixStackRef,
        translationVector: GLKVector4,
    );
}
unsafe extern "C" {
    pub fn GLKMatrixStackScale(stack: GLKMatrixStackRef, sx: f32, sy: f32, sz: f32);
}
unsafe extern "C" {
    pub fn GLKMatrixStackScaleWithVector3(stack: GLKMatrixStackRef, scaleVector: GLKVector3);
}
unsafe extern "C" {
    pub fn GLKMatrixStackScaleWithVector4(stack: GLKMatrixStackRef, scaleVector: GLKVector4);
}
unsafe extern "C" {
    pub fn GLKMatrixStackRotate(stack: GLKMatrixStackRef, radians: f32, x: f32, y: f32, z: f32);
}
unsafe extern "C" {
    pub fn GLKMatrixStackRotateWithVector3(
        stack: GLKMatrixStackRef,
        radians: f32,
        axisVector: GLKVector3,
    );
}
unsafe extern "C" {
    pub fn GLKMatrixStackRotateWithVector4(
        stack: GLKMatrixStackRef,
        radians: f32,
        axisVector: GLKVector4,
    );
}
unsafe extern "C" {
    pub fn GLKMatrixStackRotateX(stack: GLKMatrixStackRef, radians: f32);
}
unsafe extern "C" {
    pub fn GLKMatrixStackRotateY(stack: GLKMatrixStackRef, radians: f32);
}
unsafe extern "C" {
    pub fn GLKMatrixStackRotateZ(stack: GLKMatrixStackRef, radians: f32);
}
unsafe extern "C" {
    pub fn GLKMathProject(
        object: GLKVector3,
        model: GLKMatrix4,
        projection: GLKMatrix4,
        viewport: *mut ::std::os::raw::c_int,
    ) -> GLKVector3;
}
unsafe extern "C" {
    pub fn GLKMathUnproject(
        window: GLKVector3,
        model: GLKMatrix4,
        projection: GLKMatrix4,
        viewport: *mut ::std::os::raw::c_int,
        success: *mut bool,
    ) -> GLKVector3;
}
unsafe extern "C" {
    pub fn NSStringFromGLKMatrix2(matrix: GLKMatrix2) -> NSString;
}
unsafe extern "C" {
    pub fn NSStringFromGLKMatrix3(matrix: GLKMatrix3) -> NSString;
}
unsafe extern "C" {
    pub fn NSStringFromGLKMatrix4(matrix: GLKMatrix4) -> NSString;
}
unsafe extern "C" {
    pub fn NSStringFromGLKVector2(vector: GLKVector2) -> NSString;
}
unsafe extern "C" {
    pub fn NSStringFromGLKVector3(vector: GLKVector3) -> NSString;
}
unsafe extern "C" {
    pub fn NSStringFromGLKVector4(vector: GLKVector4) -> NSString;
}
unsafe extern "C" {
    pub fn NSStringFromGLKQuaternion(quaternion: GLKQuaternion) -> NSString;
}
unsafe extern "C" {
    pub static GLKTextureLoaderApplyPremultiplication: NSString;
}
unsafe extern "C" {
    pub static GLKTextureLoaderGenerateMipmaps: NSString;
}
unsafe extern "C" {
    pub static GLKTextureLoaderOriginBottomLeft: NSString;
}
unsafe extern "C" {
    pub static GLKTextureLoaderGrayscaleAsAlpha: NSString;
}
unsafe extern "C" {
    pub static GLKTextureLoaderSRGB: NSString;
}
unsafe extern "C" {
    pub static GLKTextureLoaderErrorDomain: NSString;
}
unsafe extern "C" {
    pub static GLKTextureLoaderErrorKey: NSString;
}
unsafe extern "C" {
    pub static GLKTextureLoaderGLErrorKey: NSString;
}

unsafe impl objc2::encode::RefEncode for GLKMeshBufferAllocator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKMeshBufferAllocator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKMeshBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKMeshBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKSubmesh {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKSubmesh {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKMesh {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKMesh {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for _GLKVertexAttributeParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVertexAttributeParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVertexAttributeParameters", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKMatrix2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKMatrix2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKMatrix2", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKMatrix2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKMatrix2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKMatrix2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKMatrix3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKMatrix3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKMatrix3", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKMatrix3__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKMatrix3__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKMatrix3__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKMatrix4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKMatrix4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKMatrix4", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKMatrix4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKMatrix4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKMatrix4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector2", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector3", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector3__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector3__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector3__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector3__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector3__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector3__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector3__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector3__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector3__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector4", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKVector4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKVector4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKVector4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKQuaternion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKQuaternion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKQuaternion", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKQuaternion__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKQuaternion__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKQuaternion__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKQuaternion__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKQuaternion__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKQuaternion__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for _GLKMatrixStack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _GLKMatrixStack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_GLKMatrixStack", &[]);
}
unsafe impl objc2::encode::RefEncode for GLKEffectPropertyPrv {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKEffectPropertyPrv {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GLKEffectPropertyPrv", &[]);
}
unsafe impl objc2::encode::RefEncode for GLKEffectProperty {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKEffectProperty {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKEffectPropertyTransform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKEffectPropertyTransform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKEffectPropertyLight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKEffectPropertyLight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKEffectPropertyMaterial {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKEffectPropertyMaterial {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKEffectPropertyTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKEffectPropertyTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKEffectPropertyFog {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKEffectPropertyFog {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKBaseEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKBaseEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKReflectionMapEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKReflectionMapEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKSkyboxEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKSkyboxEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKTextureInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKTextureInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GLKTextureLoader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLKTextureLoader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
