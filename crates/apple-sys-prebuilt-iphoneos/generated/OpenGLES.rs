#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOSurface::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type EAGLRenderingAPI = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EAGLSharegroup(pub id);
impl std::ops::Deref for EAGLSharegroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EAGLSharegroup {}
impl EAGLSharegroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EAGLSharegroup").unwrap(), alloc) })
    }
}
impl INSObject for EAGLSharegroup {}
impl PNSObject for EAGLSharegroup {}
impl std::convert::TryFrom<NSObject> for EAGLSharegroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EAGLSharegroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EAGLSharegroup").unwrap()) };
        if is_kind_of {
            Ok(EAGLSharegroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EAGLSharegroup")
        }
    }
}
impl IEAGLSharegroup for EAGLSharegroup {}
pub trait IEAGLSharegroup: Sized + std::ops::Deref {
    unsafe fn debugLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, debugLabel)
    }
    unsafe fn setDebugLabel_(&self, debugLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDebugLabel : debugLabel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EAGLContext(pub id);
impl std::ops::Deref for EAGLContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EAGLContext {}
impl EAGLContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EAGLContext").unwrap(), alloc) })
    }
}
impl INSObject for EAGLContext {}
impl PNSObject for EAGLContext {}
impl std::convert::TryFrom<NSObject> for EAGLContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EAGLContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EAGLContext").unwrap()) };
        if is_kind_of {
            Ok(EAGLContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EAGLContext")
        }
    }
}
impl IEAGLContext for EAGLContext {}
pub trait IEAGLContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAPI_(&self, api: EAGLRenderingAPI) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAPI : api)
    }
    unsafe fn initWithAPI_sharegroup_(
        &self,
        api: EAGLRenderingAPI,
        sharegroup: EAGLSharegroup,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAPI : api, sharegroup : sharegroup)
    }
    unsafe fn API(&self) -> EAGLRenderingAPI
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, API)
    }
    unsafe fn sharegroup(&self) -> EAGLSharegroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharegroup)
    }
    unsafe fn debugLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, debugLabel)
    }
    unsafe fn setDebugLabel_(&self, debugLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDebugLabel : debugLabel)
    }
    unsafe fn isMultiThreaded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMultiThreaded)
    }
    unsafe fn setMultiThreaded_(&self, multiThreaded: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMultiThreaded : multiThreaded)
    }
    unsafe fn setCurrentContext_(context: EAGLContext) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EAGLContext").unwrap(), setCurrentContext : context)
    }
    unsafe fn currentContext() -> EAGLContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EAGLContext").unwrap(), currentContext)
    }
}
pub trait PEAGLDrawable: Sized + std::ops::Deref {
    unsafe fn drawableProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawableProperties)
    }
    unsafe fn setDrawableProperties_(&self, drawableProperties: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawableProperties : drawableProperties)
    }
}
impl EAGLContext_EAGLContextDrawableAdditions for EAGLContext {}
pub trait EAGLContext_EAGLContextDrawableAdditions: Sized + std::ops::Deref {
    unsafe fn renderbufferStorage_fromDrawable_(
        &self,
        target: NSUInteger,
        drawable: *mut u64,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderbufferStorage : target, fromDrawable : drawable)
    }
    unsafe fn presentRenderbuffer_(&self, target: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentRenderbuffer : target)
    }
    unsafe fn presentRenderbuffer_atTime_(
        &self,
        target: NSUInteger,
        presentationTime: CFTimeInterval,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentRenderbuffer : target, atTime : presentationTime)
    }
    unsafe fn presentRenderbuffer_afterMinimumDuration_(
        &self,
        target: NSUInteger,
        duration: CFTimeInterval,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentRenderbuffer : target, afterMinimumDuration : duration)
    }
}
impl EAGLContext_IOSurface for EAGLContext {}
pub trait EAGLContext_IOSurface: Sized + std::ops::Deref {
    unsafe fn texImageIOSurface_target_internalFormat_width_height_format_type_plane_(
        &self,
        ioSurface: IOSurfaceRef,
        target: NSUInteger,
        internalFormat: NSUInteger,
        width: u32,
        height: u32,
        format: NSUInteger,
        type_: NSUInteger,
        plane: u32,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, texImageIOSurface : ioSurface, target : target, internalFormat : internalFormat, width : width, height : height, format : format, r#type : type_, plane : plane)
    }
}
pub type GLbitfield = u32;
pub type GLboolean = u8;
pub type GLbyte = i8;
pub type GLclampf = f32;
pub type GLenum = u32;
pub type GLfloat = f32;
pub type GLint = i32;
pub type GLshort = i16;
pub type GLsizei = i32;
pub type GLubyte = u8;
pub type GLuint = u32;
pub type GLushort = u16;
pub type GLvoid = ::std::os::raw::c_void;
pub type GLchar = ::std::os::raw::c_char;
pub type GLclampx = i32;
pub type GLfixed = i32;
pub type GLhalf = u16;
pub type GLint64 = i64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLsync {
    _unused: [u8; 0],
}
pub type GLsync = *mut __GLsync;
pub type GLuint64 = u64;
pub type GLintptr = isize;
pub type GLsizeiptr = isize;
unsafe extern "C" {
    pub fn EAGLGetVersion(major: *mut ::std::os::raw::c_uint, minor: *mut ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub static kEAGLDrawablePropertyRetainedBacking: NSString;
}
unsafe extern "C" {
    pub static kEAGLDrawablePropertyColorFormat: NSString;
}
unsafe extern "C" {
    pub static kEAGLColorFormatRGBA8: NSString;
}
unsafe extern "C" {
    pub static kEAGLColorFormatRGB565: NSString;
}
unsafe extern "C" {
    pub static kEAGLColorFormatSRGBA8: NSString;
}
unsafe extern "C" {
    pub fn glAlphaFunc(func: GLenum, ref_: GLclampf);
}
unsafe extern "C" {
    pub fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
}
unsafe extern "C" {
    pub fn glClearDepthf(depth: GLclampf);
}
unsafe extern "C" {
    pub fn glClipPlanef(plane: GLenum, equation: *const GLfloat);
}
unsafe extern "C" {
    pub fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
}
unsafe extern "C" {
    pub fn glDepthRangef(zNear: GLclampf, zFar: GLclampf);
}
unsafe extern "C" {
    pub fn glFogf(pname: GLenum, param: GLfloat);
}
unsafe extern "C" {
    pub fn glFogfv(pname: GLenum, params: *const GLfloat);
}
unsafe extern "C" {
    pub fn glFrustumf(
        left: GLfloat,
        right: GLfloat,
        bottom: GLfloat,
        top: GLfloat,
        zNear: GLfloat,
        zFar: GLfloat,
    );
}
unsafe extern "C" {
    pub fn glGetClipPlanef(pname: GLenum, equation: *mut GLfloat);
}
unsafe extern "C" {
    pub fn glGetFloatv(pname: GLenum, params: *mut GLfloat);
}
unsafe extern "C" {
    pub fn glGetLightfv(light: GLenum, pname: GLenum, params: *mut GLfloat);
}
unsafe extern "C" {
    pub fn glGetMaterialfv(face: GLenum, pname: GLenum, params: *mut GLfloat);
}
unsafe extern "C" {
    pub fn glGetTexEnvfv(env: GLenum, pname: GLenum, params: *mut GLfloat);
}
unsafe extern "C" {
    pub fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat);
}
unsafe extern "C" {
    pub fn glLightModelf(pname: GLenum, param: GLfloat);
}
unsafe extern "C" {
    pub fn glLightModelfv(pname: GLenum, params: *const GLfloat);
}
unsafe extern "C" {
    pub fn glLightf(light: GLenum, pname: GLenum, param: GLfloat);
}
unsafe extern "C" {
    pub fn glLightfv(light: GLenum, pname: GLenum, params: *const GLfloat);
}
unsafe extern "C" {
    pub fn glLineWidth(width: GLfloat);
}
unsafe extern "C" {
    pub fn glLoadMatrixf(m: *const GLfloat);
}
unsafe extern "C" {
    pub fn glMaterialf(face: GLenum, pname: GLenum, param: GLfloat);
}
unsafe extern "C" {
    pub fn glMaterialfv(face: GLenum, pname: GLenum, params: *const GLfloat);
}
unsafe extern "C" {
    pub fn glMultMatrixf(m: *const GLfloat);
}
unsafe extern "C" {
    pub fn glMultiTexCoord4f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);
}
unsafe extern "C" {
    pub fn glNormal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat);
}
unsafe extern "C" {
    pub fn glOrthof(
        left: GLfloat,
        right: GLfloat,
        bottom: GLfloat,
        top: GLfloat,
        zNear: GLfloat,
        zFar: GLfloat,
    );
}
unsafe extern "C" {
    pub fn glPointParameterf(pname: GLenum, param: GLfloat);
}
unsafe extern "C" {
    pub fn glPointParameterfv(pname: GLenum, params: *const GLfloat);
}
unsafe extern "C" {
    pub fn glPointSize(size: GLfloat);
}
unsafe extern "C" {
    pub fn glPolygonOffset(factor: GLfloat, units: GLfloat);
}
unsafe extern "C" {
    pub fn glRotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat);
}
unsafe extern "C" {
    pub fn glSampleCoverage(value: GLclampf, invert: GLboolean);
}
unsafe extern "C" {
    pub fn glScalef(x: GLfloat, y: GLfloat, z: GLfloat);
}
unsafe extern "C" {
    pub fn glTexEnvf(target: GLenum, pname: GLenum, param: GLfloat);
}
unsafe extern "C" {
    pub fn glTexEnvfv(target: GLenum, pname: GLenum, params: *const GLfloat);
}
unsafe extern "C" {
    pub fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
}
unsafe extern "C" {
    pub fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat);
}
unsafe extern "C" {
    pub fn glTranslatef(x: GLfloat, y: GLfloat, z: GLfloat);
}
unsafe extern "C" {
    pub fn glActiveTexture(texture: GLenum);
}
unsafe extern "C" {
    pub fn glAlphaFuncx(func: GLenum, ref_: GLclampx);
}
unsafe extern "C" {
    pub fn glBindBuffer(target: GLenum, buffer: GLuint);
}
unsafe extern "C" {
    pub fn glBindTexture(target: GLenum, texture: GLuint);
}
unsafe extern "C" {
    pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
}
unsafe extern "C" {
    pub fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum);
}
unsafe extern "C" {
    pub fn glBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const GLvoid);
}
unsafe extern "C" {
    pub fn glClear(mask: GLbitfield);
}
unsafe extern "C" {
    pub fn glClearColorx(red: GLclampx, green: GLclampx, blue: GLclampx, alpha: GLclampx);
}
unsafe extern "C" {
    pub fn glClearDepthx(depth: GLclampx);
}
unsafe extern "C" {
    pub fn glClearStencil(s: GLint);
}
unsafe extern "C" {
    pub fn glClientActiveTexture(texture: GLenum);
}
unsafe extern "C" {
    pub fn glClipPlanex(plane: GLenum, equation: *const GLfixed);
}
unsafe extern "C" {
    pub fn glColor4ub(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte);
}
unsafe extern "C" {
    pub fn glColor4x(red: GLfixed, green: GLfixed, blue: GLfixed, alpha: GLfixed);
}
unsafe extern "C" {
    pub fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
}
unsafe extern "C" {
    pub fn glColorPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
}
unsafe extern "C" {
    pub fn glCompressedTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glCompressedTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glCopyTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    );
}
unsafe extern "C" {
    pub fn glCopyTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glCullFace(mode: GLenum);
}
unsafe extern "C" {
    pub fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint);
}
unsafe extern "C" {
    pub fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
}
unsafe extern "C" {
    pub fn glDepthFunc(func: GLenum);
}
unsafe extern "C" {
    pub fn glDepthMask(flag: GLboolean);
}
unsafe extern "C" {
    pub fn glDepthRangex(zNear: GLclampx, zFar: GLclampx);
}
unsafe extern "C" {
    pub fn glDisable(cap: GLenum);
}
unsafe extern "C" {
    pub fn glDisableClientState(array: GLenum);
}
unsafe extern "C" {
    pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
}
unsafe extern "C" {
    pub fn glDrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid);
}
unsafe extern "C" {
    pub fn glEnable(cap: GLenum);
}
unsafe extern "C" {
    pub fn glEnableClientState(array: GLenum);
}
unsafe extern "C" {
    pub fn glFinish();
}
unsafe extern "C" {
    pub fn glFlush();
}
unsafe extern "C" {
    pub fn glFogx(pname: GLenum, param: GLfixed);
}
unsafe extern "C" {
    pub fn glFogxv(pname: GLenum, params: *const GLfixed);
}
unsafe extern "C" {
    pub fn glFrontFace(mode: GLenum);
}
unsafe extern "C" {
    pub fn glFrustumx(
        left: GLfixed,
        right: GLfixed,
        bottom: GLfixed,
        top: GLfixed,
        zNear: GLfixed,
        zFar: GLfixed,
    );
}
unsafe extern "C" {
    pub fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);
}
unsafe extern "C" {
    pub fn glGenTextures(n: GLsizei, textures: *mut GLuint);
}
unsafe extern "C" {
    pub fn glGetBooleanv(pname: GLenum, params: *mut GLboolean);
}
unsafe extern "C" {
    pub fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetClipPlanex(pname: GLenum, eqn: *mut GLfixed);
}
unsafe extern "C" {
    pub fn glGetError() -> GLenum;
}
unsafe extern "C" {
    pub fn glGetFixedv(pname: GLenum, params: *mut GLfixed);
}
unsafe extern "C" {
    pub fn glGetIntegerv(pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetLightxv(light: GLenum, pname: GLenum, params: *mut GLfixed);
}
unsafe extern "C" {
    pub fn glGetMaterialxv(face: GLenum, pname: GLenum, params: *mut GLfixed);
}
unsafe extern "C" {
    pub fn glGetPointerv(pname: GLenum, params: *mut *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn glGetString(name: GLenum) -> *const GLubyte;
}
unsafe extern "C" {
    pub fn glGetTexEnviv(env: GLenum, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetTexEnvxv(env: GLenum, pname: GLenum, params: *mut GLfixed);
}
unsafe extern "C" {
    pub fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetTexParameterxv(target: GLenum, pname: GLenum, params: *mut GLfixed);
}
unsafe extern "C" {
    pub fn glHint(target: GLenum, mode: GLenum);
}
unsafe extern "C" {
    pub fn glIsBuffer(buffer: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glIsEnabled(cap: GLenum) -> GLboolean;
}
unsafe extern "C" {
    pub fn glIsTexture(texture: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glLightModelx(pname: GLenum, param: GLfixed);
}
unsafe extern "C" {
    pub fn glLightModelxv(pname: GLenum, params: *const GLfixed);
}
unsafe extern "C" {
    pub fn glLightx(light: GLenum, pname: GLenum, param: GLfixed);
}
unsafe extern "C" {
    pub fn glLightxv(light: GLenum, pname: GLenum, params: *const GLfixed);
}
unsafe extern "C" {
    pub fn glLineWidthx(width: GLfixed);
}
unsafe extern "C" {
    pub fn glLoadIdentity();
}
unsafe extern "C" {
    pub fn glLoadMatrixx(m: *const GLfixed);
}
unsafe extern "C" {
    pub fn glLogicOp(opcode: GLenum);
}
unsafe extern "C" {
    pub fn glMaterialx(face: GLenum, pname: GLenum, param: GLfixed);
}
unsafe extern "C" {
    pub fn glMaterialxv(face: GLenum, pname: GLenum, params: *const GLfixed);
}
unsafe extern "C" {
    pub fn glMatrixMode(mode: GLenum);
}
unsafe extern "C" {
    pub fn glMultMatrixx(m: *const GLfixed);
}
unsafe extern "C" {
    pub fn glMultiTexCoord4x(target: GLenum, s: GLfixed, t: GLfixed, r: GLfixed, q: GLfixed);
}
unsafe extern "C" {
    pub fn glNormal3x(nx: GLfixed, ny: GLfixed, nz: GLfixed);
}
unsafe extern "C" {
    pub fn glNormalPointer(type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
}
unsafe extern "C" {
    pub fn glOrthox(
        left: GLfixed,
        right: GLfixed,
        bottom: GLfixed,
        top: GLfixed,
        zNear: GLfixed,
        zFar: GLfixed,
    );
}
unsafe extern "C" {
    pub fn glPixelStorei(pname: GLenum, param: GLint);
}
unsafe extern "C" {
    pub fn glPointParameterx(pname: GLenum, param: GLfixed);
}
unsafe extern "C" {
    pub fn glPointParameterxv(pname: GLenum, params: *const GLfixed);
}
unsafe extern "C" {
    pub fn glPointSizex(size: GLfixed);
}
unsafe extern "C" {
    pub fn glPolygonOffsetx(factor: GLfixed, units: GLfixed);
}
unsafe extern "C" {
    pub fn glPopMatrix();
}
unsafe extern "C" {
    pub fn glPushMatrix();
}
unsafe extern "C" {
    pub fn glReadPixels(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut GLvoid,
    );
}
unsafe extern "C" {
    pub fn glRotatex(angle: GLfixed, x: GLfixed, y: GLfixed, z: GLfixed);
}
unsafe extern "C" {
    pub fn glSampleCoveragex(value: GLclampx, invert: GLboolean);
}
unsafe extern "C" {
    pub fn glScalex(x: GLfixed, y: GLfixed, z: GLfixed);
}
unsafe extern "C" {
    pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}
unsafe extern "C" {
    pub fn glShadeModel(mode: GLenum);
}
unsafe extern "C" {
    pub fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint);
}
unsafe extern "C" {
    pub fn glStencilMask(mask: GLuint);
}
unsafe extern "C" {
    pub fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);
}
unsafe extern "C" {
    pub fn glTexCoordPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
}
unsafe extern "C" {
    pub fn glTexEnvi(target: GLenum, pname: GLenum, param: GLint);
}
unsafe extern "C" {
    pub fn glTexEnvx(target: GLenum, pname: GLenum, param: GLfixed);
}
unsafe extern "C" {
    pub fn glTexEnviv(target: GLenum, pname: GLenum, params: *const GLint);
}
unsafe extern "C" {
    pub fn glTexEnvxv(target: GLenum, pname: GLenum, params: *const GLfixed);
}
unsafe extern "C" {
    pub fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
}
unsafe extern "C" {
    pub fn glTexParameterx(target: GLenum, pname: GLenum, param: GLfixed);
}
unsafe extern "C" {
    pub fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint);
}
unsafe extern "C" {
    pub fn glTexParameterxv(target: GLenum, pname: GLenum, params: *const GLfixed);
}
unsafe extern "C" {
    pub fn glTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glTranslatex(x: GLfixed, y: GLfixed, z: GLfixed);
}
unsafe extern "C" {
    pub fn glVertexPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
}
unsafe extern "C" {
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}
unsafe extern "C" {
    pub fn glCurrentPaletteMatrixOES(matrixpaletteindex: GLuint);
}
unsafe extern "C" {
    pub fn glLoadPaletteFromModelViewMatrixOES();
}
unsafe extern "C" {
    pub fn glMatrixIndexPointerOES(
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glWeightPointerOES(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
}
unsafe extern "C" {
    pub fn glPointSizePointerOES(type_: GLenum, stride: GLsizei, pointer: *const GLvoid);
}
unsafe extern "C" {
    pub fn glDrawTexsOES(x: GLshort, y: GLshort, z: GLshort, width: GLshort, height: GLshort);
}
unsafe extern "C" {
    pub fn glDrawTexiOES(x: GLint, y: GLint, z: GLint, width: GLint, height: GLint);
}
unsafe extern "C" {
    pub fn glDrawTexxOES(x: GLfixed, y: GLfixed, z: GLfixed, width: GLfixed, height: GLfixed);
}
unsafe extern "C" {
    pub fn glDrawTexsvOES(coords: *const GLshort);
}
unsafe extern "C" {
    pub fn glDrawTexivOES(coords: *const GLint);
}
unsafe extern "C" {
    pub fn glDrawTexxvOES(coords: *const GLfixed);
}
unsafe extern "C" {
    pub fn glDrawTexfOES(x: GLfloat, y: GLfloat, z: GLfloat, width: GLfloat, height: GLfloat);
}
unsafe extern "C" {
    pub fn glDrawTexfvOES(coords: *const GLfloat);
}
unsafe extern "C" {
    pub fn glCopyTextureLevelsAPPLE(
        destinationTexture: GLuint,
        sourceTexture: GLuint,
        sourceBaseLevel: GLint,
        sourceLevelCount: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glRenderbufferStorageMultisampleAPPLE(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glResolveMultisampleFramebufferAPPLE();
}
unsafe extern "C" {
    pub fn glLabelObjectEXT(
        type_: GLenum,
        object: GLuint,
        length: GLsizei,
        label: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn glGetObjectLabelEXT(
        type_: GLenum,
        object: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn glInsertEventMarkerEXT(length: GLsizei, marker: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn glPushGroupMarkerEXT(length: GLsizei, marker: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn glPopGroupMarkerEXT();
}
unsafe extern "C" {
    pub fn glDiscardFramebufferEXT(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
    );
}
unsafe extern "C" {
    pub fn glMapBufferRangeEXT(
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) -> *mut GLvoid;
}
unsafe extern "C" {
    pub fn glFlushMappedBufferRangeEXT(target: GLenum, offset: GLintptr, length: GLsizeiptr);
}
unsafe extern "C" {
    pub fn glTexStorage2DEXT(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glBlendEquationSeparateOES(modeRGB: GLenum, modeAlpha: GLenum);
}
unsafe extern "C" {
    pub fn glBlendFuncSeparateOES(
        srcRGB: GLenum,
        dstRGB: GLenum,
        srcAlpha: GLenum,
        dstAlpha: GLenum,
    );
}
unsafe extern "C" {
    pub fn glBlendEquationOES(mode: GLenum);
}
unsafe extern "C" {
    pub fn glIsRenderbufferOES(renderbuffer: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glBindRenderbufferOES(target: GLenum, renderbuffer: GLuint);
}
unsafe extern "C" {
    pub fn glDeleteRenderbuffersOES(n: GLsizei, renderbuffers: *const GLuint);
}
unsafe extern "C" {
    pub fn glGenRenderbuffersOES(n: GLsizei, renderbuffers: *mut GLuint);
}
unsafe extern "C" {
    pub fn glRenderbufferStorageOES(
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glGetRenderbufferParameterivOES(target: GLenum, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glIsFramebufferOES(framebuffer: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glBindFramebufferOES(target: GLenum, framebuffer: GLuint);
}
unsafe extern "C" {
    pub fn glDeleteFramebuffersOES(n: GLsizei, framebuffers: *const GLuint);
}
unsafe extern "C" {
    pub fn glGenFramebuffersOES(n: GLsizei, framebuffers: *mut GLuint);
}
unsafe extern "C" {
    pub fn glCheckFramebufferStatusOES(target: GLenum) -> GLenum;
}
unsafe extern "C" {
    pub fn glFramebufferRenderbufferOES(
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    );
}
unsafe extern "C" {
    pub fn glFramebufferTexture2DOES(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    );
}
unsafe extern "C" {
    pub fn glGetFramebufferAttachmentParameterivOES(
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    );
}
unsafe extern "C" {
    pub fn glGenerateMipmapOES(target: GLenum);
}
unsafe extern "C" {
    pub fn glGetBufferPointervOES(target: GLenum, pname: GLenum, params: *mut *mut GLvoid);
}
unsafe extern "C" {
    pub fn glMapBufferOES(target: GLenum, access: GLenum) -> *mut GLvoid;
}
unsafe extern "C" {
    pub fn glUnmapBufferOES(target: GLenum) -> GLboolean;
}
unsafe extern "C" {
    pub fn glBindVertexArrayOES(array: GLuint);
}
unsafe extern "C" {
    pub fn glDeleteVertexArraysOES(n: GLsizei, arrays: *const GLuint);
}
unsafe extern "C" {
    pub fn glGenVertexArraysOES(n: GLsizei, arrays: *mut GLuint);
}
unsafe extern "C" {
    pub fn glIsVertexArrayOES(array: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glAttachShader(program: GLuint, shader: GLuint);
}
unsafe extern "C" {
    pub fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar);
}
unsafe extern "C" {
    pub fn glBindFramebuffer(target: GLenum, framebuffer: GLuint);
}
unsafe extern "C" {
    pub fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint);
}
unsafe extern "C" {
    pub fn glBlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
}
unsafe extern "C" {
    pub fn glBlendEquation(mode: GLenum);
}
unsafe extern "C" {
    pub fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);
}
unsafe extern "C" {
    pub fn glBlendFuncSeparate(srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
}
unsafe extern "C" {
    pub fn glCheckFramebufferStatus(target: GLenum) -> GLenum;
}
unsafe extern "C" {
    pub fn glCompileShader(shader: GLuint);
}
unsafe extern "C" {
    pub fn glCreateProgram() -> GLuint;
}
unsafe extern "C" {
    pub fn glCreateShader(type_: GLenum) -> GLuint;
}
unsafe extern "C" {
    pub fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
}
unsafe extern "C" {
    pub fn glDeleteProgram(program: GLuint);
}
unsafe extern "C" {
    pub fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint);
}
unsafe extern "C" {
    pub fn glDeleteShader(shader: GLuint);
}
unsafe extern "C" {
    pub fn glDetachShader(program: GLuint, shader: GLuint);
}
unsafe extern "C" {
    pub fn glDisableVertexAttribArray(index: GLuint);
}
unsafe extern "C" {
    pub fn glEnableVertexAttribArray(index: GLuint);
}
unsafe extern "C" {
    pub fn glFramebufferRenderbuffer(
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    );
}
unsafe extern "C" {
    pub fn glFramebufferTexture2D(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    );
}
unsafe extern "C" {
    pub fn glGenerateMipmap(target: GLenum);
}
unsafe extern "C" {
    pub fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
}
unsafe extern "C" {
    pub fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);
}
unsafe extern "C" {
    pub fn glGetActiveAttrib(
        program: GLuint,
        index: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
}
unsafe extern "C" {
    pub fn glGetActiveUniform(
        program: GLuint,
        index: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
}
unsafe extern "C" {
    pub fn glGetAttachedShaders(
        program: GLuint,
        maxcount: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint,
    );
}
unsafe extern "C" {
    pub fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glGetFramebufferAttachmentParameteriv(
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    );
}
unsafe extern "C" {
    pub fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetProgramInfoLog(
        program: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        infolog: *mut GLchar,
    );
}
unsafe extern "C" {
    pub fn glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetShaderInfoLog(
        shader: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        infolog: *mut GLchar,
    );
}
unsafe extern "C" {
    pub fn glGetShaderPrecisionFormat(
        shadertype: GLenum,
        precisiontype: GLenum,
        range: *mut GLint,
        precision: *mut GLint,
    );
}
unsafe extern "C" {
    pub fn glGetShaderSource(
        shader: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar,
    );
}
unsafe extern "C" {
    pub fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat);
}
unsafe extern "C" {
    pub fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat);
}
unsafe extern "C" {
    pub fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *mut *mut GLvoid);
}
unsafe extern "C" {
    pub fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glIsProgram(program: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glIsShader(shader: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glLinkProgram(program: GLuint);
}
unsafe extern "C" {
    pub fn glReleaseShaderCompiler();
}
unsafe extern "C" {
    pub fn glRenderbufferStorage(
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glShaderBinary(
        n: GLsizei,
        shaders: *const GLuint,
        binaryformat: GLenum,
        binary: *const GLvoid,
        length: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    );
}
unsafe extern "C" {
    pub fn glStencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
}
unsafe extern "C" {
    pub fn glStencilMaskSeparate(face: GLenum, mask: GLuint);
}
unsafe extern "C" {
    pub fn glStencilOpSeparate(face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum);
}
unsafe extern "C" {
    pub fn glUniform1f(location: GLint, x: GLfloat);
}
unsafe extern "C" {
    pub fn glUniform1fv(location: GLint, count: GLsizei, v: *const GLfloat);
}
unsafe extern "C" {
    pub fn glUniform1i(location: GLint, x: GLint);
}
unsafe extern "C" {
    pub fn glUniform1iv(location: GLint, count: GLsizei, v: *const GLint);
}
unsafe extern "C" {
    pub fn glUniform2f(location: GLint, x: GLfloat, y: GLfloat);
}
unsafe extern "C" {
    pub fn glUniform2fv(location: GLint, count: GLsizei, v: *const GLfloat);
}
unsafe extern "C" {
    pub fn glUniform2i(location: GLint, x: GLint, y: GLint);
}
unsafe extern "C" {
    pub fn glUniform2iv(location: GLint, count: GLsizei, v: *const GLint);
}
unsafe extern "C" {
    pub fn glUniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat);
}
unsafe extern "C" {
    pub fn glUniform3fv(location: GLint, count: GLsizei, v: *const GLfloat);
}
unsafe extern "C" {
    pub fn glUniform3i(location: GLint, x: GLint, y: GLint, z: GLint);
}
unsafe extern "C" {
    pub fn glUniform3iv(location: GLint, count: GLsizei, v: *const GLint);
}
unsafe extern "C" {
    pub fn glUniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
}
unsafe extern "C" {
    pub fn glUniform4fv(location: GLint, count: GLsizei, v: *const GLfloat);
}
unsafe extern "C" {
    pub fn glUniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint);
}
unsafe extern "C" {
    pub fn glUniform4iv(location: GLint, count: GLsizei, v: *const GLint);
}
unsafe extern "C" {
    pub fn glUniformMatrix2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glUniformMatrix3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glUseProgram(program: GLuint);
}
unsafe extern "C" {
    pub fn glValidateProgram(program: GLuint);
}
unsafe extern "C" {
    pub fn glVertexAttrib1f(indx: GLuint, x: GLfloat);
}
unsafe extern "C" {
    pub fn glVertexAttrib1fv(indx: GLuint, values: *const GLfloat);
}
unsafe extern "C" {
    pub fn glVertexAttrib2f(indx: GLuint, x: GLfloat, y: GLfloat);
}
unsafe extern "C" {
    pub fn glVertexAttrib2fv(indx: GLuint, values: *const GLfloat);
}
unsafe extern "C" {
    pub fn glVertexAttrib3f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
}
unsafe extern "C" {
    pub fn glVertexAttrib3fv(indx: GLuint, values: *const GLfloat);
}
unsafe extern "C" {
    pub fn glVertexAttrib4f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
}
unsafe extern "C" {
    pub fn glVertexAttrib4fv(indx: GLuint, values: *const GLfloat);
}
unsafe extern "C" {
    pub fn glVertexAttribPointer(
        indx: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        ptr: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glFenceSyncAPPLE(condition: GLenum, flags: GLbitfield) -> GLsync;
}
unsafe extern "C" {
    pub fn glIsSyncAPPLE(sync: GLsync) -> GLboolean;
}
unsafe extern "C" {
    pub fn glDeleteSyncAPPLE(sync: GLsync);
}
unsafe extern "C" {
    pub fn glClientWaitSyncAPPLE(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
}
unsafe extern "C" {
    pub fn glWaitSyncAPPLE(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
}
unsafe extern "C" {
    pub fn glGetInteger64vAPPLE(pname: GLenum, params: *mut GLint64);
}
unsafe extern "C" {
    pub fn glGetSyncivAPPLE(
        sync: GLsync,
        pname: GLenum,
        bufSize: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint,
    );
}
unsafe extern "C" {
    pub fn glDrawArraysInstancedEXT(
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instanceCount: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glDrawElementsInstancedEXT(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
        instanceCount: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glVertexAttribDivisorEXT(index: GLuint, divisor: GLuint);
}
unsafe extern "C" {
    pub fn glGenQueriesEXT(n: GLsizei, ids: *mut GLuint);
}
unsafe extern "C" {
    pub fn glDeleteQueriesEXT(n: GLsizei, ids: *const GLuint);
}
unsafe extern "C" {
    pub fn glIsQueryEXT(id: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glBeginQueryEXT(target: GLenum, id: GLuint);
}
unsafe extern "C" {
    pub fn glEndQueryEXT(target: GLenum);
}
unsafe extern "C" {
    pub fn glGetQueryivEXT(target: GLenum, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetQueryObjectuivEXT(id: GLuint, pname: GLenum, params: *mut GLuint);
}
unsafe extern "C" {
    pub fn glUseProgramStagesEXT(pipeline: GLuint, stages: GLbitfield, program: GLuint);
}
unsafe extern "C" {
    pub fn glActiveShaderProgramEXT(pipeline: GLuint, program: GLuint);
}
unsafe extern "C" {
    pub fn glCreateShaderProgramvEXT(
        type_: GLenum,
        count: GLsizei,
        strings: *const *const GLchar,
    ) -> GLuint;
}
unsafe extern "C" {
    pub fn glBindProgramPipelineEXT(pipeline: GLuint);
}
unsafe extern "C" {
    pub fn glDeleteProgramPipelinesEXT(n: GLsizei, pipelines: *const GLuint);
}
unsafe extern "C" {
    pub fn glGenProgramPipelinesEXT(n: GLsizei, pipelines: *mut GLuint);
}
unsafe extern "C" {
    pub fn glIsProgramPipelineEXT(pipeline: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glProgramParameteriEXT(program: GLuint, pname: GLenum, value: GLint);
}
unsafe extern "C" {
    pub fn glGetProgramPipelineivEXT(pipeline: GLuint, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glValidateProgramPipelineEXT(pipeline: GLuint);
}
unsafe extern "C" {
    pub fn glGetProgramPipelineInfoLogEXT(
        pipeline: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform1iEXT(program: GLuint, location: GLint, x: GLint);
}
unsafe extern "C" {
    pub fn glProgramUniform2iEXT(program: GLuint, location: GLint, x: GLint, y: GLint);
}
unsafe extern "C" {
    pub fn glProgramUniform3iEXT(program: GLuint, location: GLint, x: GLint, y: GLint, z: GLint);
}
unsafe extern "C" {
    pub fn glProgramUniform4iEXT(
        program: GLuint,
        location: GLint,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform1fEXT(program: GLuint, location: GLint, x: GLfloat);
}
unsafe extern "C" {
    pub fn glProgramUniform2fEXT(program: GLuint, location: GLint, x: GLfloat, y: GLfloat);
}
unsafe extern "C" {
    pub fn glProgramUniform3fEXT(
        program: GLuint,
        location: GLint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform4fEXT(
        program: GLuint,
        location: GLint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform1ivEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform2ivEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform3ivEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform4ivEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform1fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform2fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform3fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform4fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniformMatrix2fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniformMatrix3fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniformMatrix4fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glReadBuffer(mode: GLenum);
}
unsafe extern "C" {
    pub fn glDrawRangeElements(
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glCopyTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glCompressedTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glCompressedTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glGenQueries(n: GLsizei, ids: *mut GLuint);
}
unsafe extern "C" {
    pub fn glDeleteQueries(n: GLsizei, ids: *const GLuint);
}
unsafe extern "C" {
    pub fn glIsQuery(id: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glBeginQuery(target: GLenum, id: GLuint);
}
unsafe extern "C" {
    pub fn glEndQuery(target: GLenum);
}
unsafe extern "C" {
    pub fn glGetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint);
}
unsafe extern "C" {
    pub fn glUnmapBuffer(target: GLenum) -> GLboolean;
}
unsafe extern "C" {
    pub fn glGetBufferPointerv(target: GLenum, pname: GLenum, params: *mut *mut GLvoid);
}
unsafe extern "C" {
    pub fn glDrawBuffers(n: GLsizei, bufs: *const GLenum);
}
unsafe extern "C" {
    pub fn glUniformMatrix2x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glUniformMatrix3x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glUniformMatrix2x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glUniformMatrix4x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glUniformMatrix3x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glUniformMatrix4x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glBlitFramebuffer(
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    );
}
unsafe extern "C" {
    pub fn glRenderbufferStorageMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glFramebufferTextureLayer(
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    );
}
unsafe extern "C" {
    pub fn glMapBufferRange(
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) -> *mut GLvoid;
}
unsafe extern "C" {
    pub fn glFlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr);
}
unsafe extern "C" {
    pub fn glBindVertexArray(array: GLuint);
}
unsafe extern "C" {
    pub fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint);
}
unsafe extern "C" {
    pub fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);
}
unsafe extern "C" {
    pub fn glIsVertexArray(array: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glGetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint);
}
unsafe extern "C" {
    pub fn glBeginTransformFeedback(primitiveMode: GLenum);
}
unsafe extern "C" {
    pub fn glEndTransformFeedback();
}
unsafe extern "C" {
    pub fn glBindBufferRange(
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    );
}
unsafe extern "C" {
    pub fn glBindBufferBase(target: GLenum, index: GLuint, buffer: GLuint);
}
unsafe extern "C" {
    pub fn glTransformFeedbackVaryings(
        program: GLuint,
        count: GLsizei,
        varyings: *const *const GLchar,
        bufferMode: GLenum,
    );
}
unsafe extern "C" {
    pub fn glGetTransformFeedbackVarying(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
}
unsafe extern "C" {
    pub fn glVertexAttribIPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const GLvoid,
    );
}
unsafe extern "C" {
    pub fn glGetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint);
}
unsafe extern "C" {
    pub fn glVertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
}
unsafe extern "C" {
    pub fn glVertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
}
unsafe extern "C" {
    pub fn glVertexAttribI4iv(index: GLuint, v: *const GLint);
}
unsafe extern "C" {
    pub fn glVertexAttribI4uiv(index: GLuint, v: *const GLuint);
}
unsafe extern "C" {
    pub fn glGetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint);
}
unsafe extern "C" {
    pub fn glGetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint;
}
unsafe extern "C" {
    pub fn glUniform1ui(location: GLint, v0: GLuint);
}
unsafe extern "C" {
    pub fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint);
}
unsafe extern "C" {
    pub fn glUniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
}
unsafe extern "C" {
    pub fn glUniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
}
unsafe extern "C" {
    pub fn glUniform1uiv(location: GLint, count: GLsizei, value: *const GLuint);
}
unsafe extern "C" {
    pub fn glUniform2uiv(location: GLint, count: GLsizei, value: *const GLuint);
}
unsafe extern "C" {
    pub fn glUniform3uiv(location: GLint, count: GLsizei, value: *const GLuint);
}
unsafe extern "C" {
    pub fn glUniform4uiv(location: GLint, count: GLsizei, value: *const GLuint);
}
unsafe extern "C" {
    pub fn glClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
}
unsafe extern "C" {
    pub fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
}
unsafe extern "C" {
    pub fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
}
unsafe extern "C" {
    pub fn glClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
}
unsafe extern "C" {
    pub fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte;
}
unsafe extern "C" {
    pub fn glCopyBufferSubData(
        readTarget: GLenum,
        writeTarget: GLenum,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr,
    );
}
unsafe extern "C" {
    pub fn glGetUniformIndices(
        program: GLuint,
        uniformCount: GLsizei,
        uniformNames: *const *const GLchar,
        uniformIndices: *mut GLuint,
    );
}
unsafe extern "C" {
    pub fn glGetActiveUniformsiv(
        program: GLuint,
        uniformCount: GLsizei,
        uniformIndices: *const GLuint,
        pname: GLenum,
        params: *mut GLint,
    );
}
unsafe extern "C" {
    pub fn glGetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
}
unsafe extern "C" {
    pub fn glGetActiveUniformBlockiv(
        program: GLuint,
        uniformBlockIndex: GLuint,
        pname: GLenum,
        params: *mut GLint,
    );
}
unsafe extern "C" {
    pub fn glGetActiveUniformBlockName(
        program: GLuint,
        uniformBlockIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformBlockName: *mut GLchar,
    );
}
unsafe extern "C" {
    pub fn glUniformBlockBinding(
        program: GLuint,
        uniformBlockIndex: GLuint,
        uniformBlockBinding: GLuint,
    );
}
unsafe extern "C" {
    pub fn glDrawArraysInstanced(
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glDrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
        instancecount: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glFenceSync(condition: GLenum, flags: GLbitfield) -> GLsync;
}
unsafe extern "C" {
    pub fn glIsSync(sync: GLsync) -> GLboolean;
}
unsafe extern "C" {
    pub fn glDeleteSync(sync: GLsync);
}
unsafe extern "C" {
    pub fn glClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
}
unsafe extern "C" {
    pub fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
}
unsafe extern "C" {
    pub fn glGetInteger64v(pname: GLenum, params: *mut GLint64);
}
unsafe extern "C" {
    pub fn glGetSynciv(
        sync: GLsync,
        pname: GLenum,
        bufSize: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint,
    );
}
unsafe extern "C" {
    pub fn glGetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64);
}
unsafe extern "C" {
    pub fn glGetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64);
}
unsafe extern "C" {
    pub fn glGenSamplers(count: GLsizei, samplers: *mut GLuint);
}
unsafe extern "C" {
    pub fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint);
}
unsafe extern "C" {
    pub fn glIsSampler(sampler: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glBindSampler(unit: GLuint, sampler: GLuint);
}
unsafe extern "C" {
    pub fn glSamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint);
}
unsafe extern "C" {
    pub fn glSamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint);
}
unsafe extern "C" {
    pub fn glSamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat);
}
unsafe extern "C" {
    pub fn glSamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat);
}
unsafe extern "C" {
    pub fn glGetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint);
}
unsafe extern "C" {
    pub fn glGetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
}
unsafe extern "C" {
    pub fn glVertexAttribDivisor(index: GLuint, divisor: GLuint);
}
unsafe extern "C" {
    pub fn glBindTransformFeedback(target: GLenum, id: GLuint);
}
unsafe extern "C" {
    pub fn glDeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint);
}
unsafe extern "C" {
    pub fn glGenTransformFeedbacks(n: GLsizei, ids: *mut GLuint);
}
unsafe extern "C" {
    pub fn glIsTransformFeedback(id: GLuint) -> GLboolean;
}
unsafe extern "C" {
    pub fn glPauseTransformFeedback();
}
unsafe extern "C" {
    pub fn glResumeTransformFeedback();
}
unsafe extern "C" {
    pub fn glGetProgramBinary(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        binaryFormat: *mut GLenum,
        binary: *mut GLvoid,
    );
}
unsafe extern "C" {
    pub fn glProgramBinary(
        program: GLuint,
        binaryFormat: GLenum,
        binary: *const GLvoid,
        length: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glProgramParameteri(program: GLuint, pname: GLenum, value: GLint);
}
unsafe extern "C" {
    pub fn glInvalidateFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
    );
}
unsafe extern "C" {
    pub fn glInvalidateSubFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glTexStorage2D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glTexStorage3D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    );
}
unsafe extern "C" {
    pub fn glGetInternalformativ(
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        bufSize: GLsizei,
        params: *mut GLint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform1uiEXT(program: GLuint, location: GLint, x: GLuint);
}
unsafe extern "C" {
    pub fn glProgramUniform2uiEXT(program: GLuint, location: GLint, x: GLuint, y: GLuint);
}
unsafe extern "C" {
    pub fn glProgramUniform3uiEXT(
        program: GLuint,
        location: GLint,
        x: GLuint,
        y: GLuint,
        z: GLuint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform4uiEXT(
        program: GLuint,
        location: GLint,
        x: GLuint,
        y: GLuint,
        z: GLuint,
        w: GLuint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform1uivEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform2uivEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform3uivEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniform4uivEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    );
}
unsafe extern "C" {
    pub fn glProgramUniformMatrix2x3fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniformMatrix3x2fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniformMatrix2x4fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniformMatrix4x2fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniformMatrix3x4fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
unsafe extern "C" {
    pub fn glProgramUniformMatrix4x3fvEXT(
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}

unsafe impl objc2::encode::RefEncode for EAGLSharegroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EAGLSharegroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EAGLContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EAGLContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for __GLsync {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __GLsync {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__GLsync", &[]);
}
