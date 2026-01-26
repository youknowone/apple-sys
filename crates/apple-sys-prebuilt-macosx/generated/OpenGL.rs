#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::OpenCL::*;

pub type CGLContextObj = *mut _CGLContextObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CGLPixelFormatObject {
    _unused: [u8; 0],
}
pub type CGLPixelFormatObj = *mut _CGLPixelFormatObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CGLRendererInfoObject {
    _unused: [u8; 0],
}
pub type CGLRendererInfoObj = *mut _CGLRendererInfoObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CGLPBufferObject {
    _unused: [u8; 0],
}
pub type _CGLPixelFormatAttribute = ::std::os::raw::c_uint;
pub use self::_CGLPixelFormatAttribute as CGLPixelFormatAttribute;
pub type _CGLRendererProperty = ::std::os::raw::c_uint;
pub use self::_CGLRendererProperty as CGLRendererProperty;
pub type _CGLContextEnable = ::std::os::raw::c_uint;
pub use self::_CGLContextEnable as CGLContextEnable;
pub type _CGLGPURestartStatus = ::std::os::raw::c_uint;
pub use self::_CGLGPURestartStatus as CGLGPURestartStatus;
pub type _CGLContextParameter = ::std::os::raw::c_uint;
pub use self::_CGLContextParameter as CGLContextParameter;
pub type CGLCPContextPriorityRequest = ::std::os::raw::c_uint;
pub type _CGLGlobalOption = ::std::os::raw::c_uint;
pub use self::_CGLGlobalOption as CGLGlobalOption;
pub type _CGLOpenGLProfile = ::std::os::raw::c_uint;
pub use self::_CGLOpenGLProfile as CGLOpenGLProfile;
pub type _CGLError = ::std::os::raw::c_uint;
pub use self::_CGLError as CGLError;
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
pub type GLcharARB = ::std::os::raw::c_char;
pub type GLhandleARB = *mut ::std::os::raw::c_void;
pub type GLdouble = f64;
pub type GLclampd = f64;
pub type GLfixed = i32;
pub type GLhalf = u16;
pub type GLhalfARB = u16;
pub type GLint64 = i64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLsync {
    _unused: [u8; 0],
}
pub type GLsync = *mut __GLsync;
pub type GLuint64 = u64;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;
pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGLShareGroupRec {
    _unused: [u8; 0],
}
pub type CGLShareGroupObj = *mut CGLShareGroupRec;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_device_id {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLUnurbs {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLUquadric {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLUtesselator {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLIContextRec {
    _unused: [u8; 0],
}
pub type GLIContext = *mut __GLIContextRec;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLISharedRec {
    _unused: [u8; 0],
}
pub type GLIShared = *mut __GLISharedRec;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLIFunctionDispatchRec {
    pub accum:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, op: GLenum, value: GLfloat)>,
    pub alpha_func:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, func: GLenum, ref_: GLclampf)>,
    pub are_textures_resident: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            n: GLsizei,
            textures: *const GLuint,
            residences: *mut GLboolean,
        ) -> GLboolean,
    >,
    pub array_element: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, i: GLint)>,
    pub begin: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub bind_texture: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, texture: GLuint),
    >,
    pub bitmap: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            width: GLsizei,
            height: GLsizei,
            xorig: GLfloat,
            yorig: GLfloat,
            xmove: GLfloat,
            ymove: GLfloat,
            bitmap: *const GLubyte,
        ),
    >,
    pub blend_func: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sfactor: GLenum, dfactor: GLenum),
    >,
    pub call_list: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, list: GLuint)>,
    pub call_lists: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, type_: GLenum, lists: *const GLvoid),
    >,
    pub clear: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mask: GLbitfield)>,
    pub clear_accum: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLfloat,
            green: GLfloat,
            blue: GLfloat,
            alpha: GLfloat,
        ),
    >,
    pub clear_color: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLclampf,
            green: GLclampf,
            blue: GLclampf,
            alpha: GLclampf,
        ),
    >,
    pub clear_depth: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, depth: GLclampd)>,
    pub clear_index: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: GLfloat)>,
    pub clear_stencil: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLint)>,
    pub clip_plane: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, plane: GLenum, equation: *const GLdouble),
    >,
    pub color3b: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLbyte, green: GLbyte, blue: GLbyte),
    >,
    pub color3bv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLbyte)>,
    pub color3d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLdouble, green: GLdouble, blue: GLdouble),
    >,
    pub color3dv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub color3f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLfloat, green: GLfloat, blue: GLfloat),
    >,
    pub color3fv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub color3i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLint, green: GLint, blue: GLint),
    >,
    pub color3iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub color3s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLshort, green: GLshort, blue: GLshort),
    >,
    pub color3sv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub color3ub: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLubyte, green: GLubyte, blue: GLubyte),
    >,
    pub color3ubv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLubyte)>,
    pub color3ui: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLuint, green: GLuint, blue: GLuint),
    >,
    pub color3uiv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLuint)>,
    pub color3us: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLushort, green: GLushort, blue: GLushort),
    >,
    pub color3usv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLushort)>,
    pub color4b: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLbyte,
            green: GLbyte,
            blue: GLbyte,
            alpha: GLbyte,
        ),
    >,
    pub color4bv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLbyte)>,
    pub color4d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLdouble,
            green: GLdouble,
            blue: GLdouble,
            alpha: GLdouble,
        ),
    >,
    pub color4dv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub color4f: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLfloat,
            green: GLfloat,
            blue: GLfloat,
            alpha: GLfloat,
        ),
    >,
    pub color4fv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub color4i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLint, green: GLint, blue: GLint, alpha: GLint),
    >,
    pub color4iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub color4s: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLshort,
            green: GLshort,
            blue: GLshort,
            alpha: GLshort,
        ),
    >,
    pub color4sv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub color4ub: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLubyte,
            green: GLubyte,
            blue: GLubyte,
            alpha: GLubyte,
        ),
    >,
    pub color4ubv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLubyte)>,
    pub color4ui: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLuint,
            green: GLuint,
            blue: GLuint,
            alpha: GLuint,
        ),
    >,
    pub color4uiv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLuint)>,
    pub color4us: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLushort,
            green: GLushort,
            blue: GLushort,
            alpha: GLushort,
        ),
    >,
    pub color4usv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLushort)>,
    pub color_mask: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLboolean,
            green: GLboolean,
            blue: GLboolean,
            alpha: GLboolean,
        ),
    >,
    pub color_material:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, face: GLenum, mode: GLenum)>,
    pub color_pointer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            size: GLint,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub copy_pixels: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            type_: GLenum,
        ),
    >,
    pub copy_tex_image1D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            internalFormat: GLenum,
            x: GLint,
            y: GLint,
            width: GLsizei,
            border: GLint,
        ),
    >,
    pub copy_tex_image2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            internalFormat: GLenum,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
        ),
    >,
    pub copy_tex_sub_image1D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            x: GLint,
            y: GLint,
            width: GLsizei,
        ),
    >,
    pub copy_tex_sub_image2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
        ),
    >,
    pub cull_face: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub delete_lists:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, list: GLuint, range: GLsizei)>,
    pub delete_textures: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, textures: *const GLuint),
    >,
    pub depth_func: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, func: GLenum)>,
    pub depth_mask: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, flag: GLboolean)>,
    pub depth_range: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, zNear: GLclampd, zFar: GLclampd),
    >,
    pub disable: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, cap: GLenum)>,
    pub disable_client_state:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, array: GLenum)>,
    pub draw_arrays: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, mode: GLenum, first: GLint, count: GLsizei),
    >,
    pub draw_buffer: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub draw_elements: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            count: GLsizei,
            type_: GLenum,
            indices: *const GLvoid,
        ),
    >,
    pub draw_pixels: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            type_: GLenum,
            pixels: *const GLvoid,
        ),
    >,
    pub edge_flag: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, flag: GLboolean)>,
    pub edge_flag_pointer: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, stride: GLsizei, pointer: *const GLvoid),
    >,
    pub edge_flagv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, flag: *const GLboolean)>,
    pub enable: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, cap: GLenum)>,
    pub enable_client_state:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, array: GLenum)>,
    pub end: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub end_list: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub eval_coord1d: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, u: GLdouble)>,
    pub eval_coord1dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, u: *const GLdouble)>,
    pub eval_coord1f: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, u: GLfloat)>,
    pub eval_coord1fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, u: *const GLfloat)>,
    pub eval_coord2d:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, u: GLdouble, v: GLdouble)>,
    pub eval_coord2dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, u: *const GLdouble)>,
    pub eval_coord2f:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, u: GLfloat, v: GLfloat)>,
    pub eval_coord2fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, u: *const GLfloat)>,
    pub eval_mesh1: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, mode: GLenum, i1: GLint, i2: GLint),
    >,
    pub eval_mesh2: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            i1: GLint,
            i2: GLint,
            j1: GLint,
            j2: GLint,
        ),
    >,
    pub eval_point1: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, i: GLint)>,
    pub eval_point2:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, i: GLint, j: GLint)>,
    pub feedback_buffer: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLsizei, type_: GLenum, buffer: *mut GLfloat),
    >,
    pub finish: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub flush: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub fogf:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLfloat)>,
    pub fogfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *const GLfloat),
    >,
    pub fogi:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLint)>,
    pub fogiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *const GLint),
    >,
    pub front_face: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub frustum: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            left: GLdouble,
            right: GLdouble,
            bottom: GLdouble,
            top: GLdouble,
            zNear: GLdouble,
            zFar: GLdouble,
        ),
    >,
    pub gen_lists:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, range: GLsizei) -> GLuint>,
    pub gen_textures: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, textures: *mut GLuint),
    >,
    pub get_booleanv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *mut GLboolean),
    >,
    pub get_clip_plane: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, plane: GLenum, equation: *mut GLdouble),
    >,
    pub get_doublev: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *mut GLdouble),
    >,
    pub get_error: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext) -> GLenum>,
    pub get_floatv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_integerv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *mut GLint),
    >,
    pub get_lightfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, light: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_lightiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, light: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub get_mapdv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, query: GLenum, v: *mut GLdouble),
    >,
    pub get_mapfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, query: GLenum, v: *mut GLfloat),
    >,
    pub get_mapiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, query: GLenum, v: *mut GLint),
    >,
    pub get_materialfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, face: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_materialiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, face: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub get_pixel_mapfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, map: GLenum, values: *mut GLfloat),
    >,
    pub get_pixel_mapuiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, map: GLenum, values: *mut GLuint),
    >,
    pub get_pixel_mapusv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, map: GLenum, values: *mut GLushort),
    >,
    pub get_pointerv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *mut *mut GLvoid),
    >,
    pub get_polygon_stipple:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mask: *mut GLubyte)>,
    pub get_string: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, name: GLenum) -> *const GLubyte,
    >,
    pub get_tex_envfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_tex_enviv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub get_tex_gendv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, coord: GLenum, pname: GLenum, params: *mut GLdouble),
    >,
    pub get_tex_genfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, coord: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_tex_geniv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, coord: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub get_tex_image: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            format: GLenum,
            type_: GLenum,
            pixels: *mut GLvoid,
        ),
    >,
    pub get_tex_level_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            pname: GLenum,
            params: *mut GLfloat,
        ),
    >,
    pub get_tex_level_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            pname: GLenum,
            params: *mut GLint,
        ),
    >,
    pub get_tex_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_tex_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub hint:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, mode: GLenum)>,
    pub index_mask: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mask: GLuint)>,
    pub index_pointer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub indexd: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: GLdouble)>,
    pub indexdv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: *const GLdouble)>,
    pub indexf: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: GLfloat)>,
    pub indexfv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: *const GLfloat)>,
    pub indexi: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: GLint)>,
    pub indexiv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: *const GLint)>,
    pub indexs: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: GLshort)>,
    pub indexsv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: *const GLshort)>,
    pub indexub: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: GLubyte)>,
    pub indexubv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, c: *const GLubyte)>,
    pub init_names: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub interleaved_arrays: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            format: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub is_enabled:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, cap: GLenum) -> GLboolean>,
    pub is_list:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, list: GLuint) -> GLboolean>,
    pub is_texture:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, texture: GLuint) -> GLboolean>,
    pub light_modelf:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLfloat)>,
    pub light_modelfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *const GLfloat),
    >,
    pub light_modeli:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLint)>,
    pub light_modeliv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *const GLint),
    >,
    pub lightf: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, light: GLenum, pname: GLenum, param: GLfloat),
    >,
    pub lightfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, light: GLenum, pname: GLenum, params: *const GLfloat),
    >,
    pub lighti: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, light: GLenum, pname: GLenum, param: GLint),
    >,
    pub lightiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, light: GLenum, pname: GLenum, params: *const GLint),
    >,
    pub line_stipple: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, factor: GLint, pattern: GLushort),
    >,
    pub line_width: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, width: GLfloat)>,
    pub list_base: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, base: GLuint)>,
    pub load_identity: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub load_matrixd:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, m: *const GLdouble)>,
    pub load_matrixf:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, m: *const GLfloat)>,
    pub load_name: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, name: GLuint)>,
    pub logic_op: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, opcode: GLenum)>,
    pub map1d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            u1: GLdouble,
            u2: GLdouble,
            stride: GLint,
            order: GLint,
            points: *const GLdouble,
        ),
    >,
    pub map1f: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            u1: GLfloat,
            u2: GLfloat,
            stride: GLint,
            order: GLint,
            points: *const GLfloat,
        ),
    >,
    pub map2d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            u1: GLdouble,
            u2: GLdouble,
            ustride: GLint,
            uorder: GLint,
            v1: GLdouble,
            v2: GLdouble,
            vstride: GLint,
            vorder: GLint,
            points: *const GLdouble,
        ),
    >,
    pub map2f: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            u1: GLfloat,
            u2: GLfloat,
            ustride: GLint,
            uorder: GLint,
            v1: GLfloat,
            v2: GLfloat,
            vstride: GLint,
            vorder: GLint,
            points: *const GLfloat,
        ),
    >,
    pub map_grid1d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, un: GLint, u1: GLdouble, u2: GLdouble),
    >,
    pub map_grid1f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, un: GLint, u1: GLfloat, u2: GLfloat),
    >,
    pub map_grid2d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            un: GLint,
            u1: GLdouble,
            u2: GLdouble,
            vn: GLint,
            v1: GLdouble,
            v2: GLdouble,
        ),
    >,
    pub map_grid2f: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            un: GLint,
            u1: GLfloat,
            u2: GLfloat,
            vn: GLint,
            v1: GLfloat,
            v2: GLfloat,
        ),
    >,
    pub materialf: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, face: GLenum, pname: GLenum, param: GLfloat),
    >,
    pub materialfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, face: GLenum, pname: GLenum, params: *const GLfloat),
    >,
    pub materiali: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, face: GLenum, pname: GLenum, param: GLint),
    >,
    pub materialiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, face: GLenum, pname: GLenum, params: *const GLint),
    >,
    pub matrix_mode: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub mult_matrixd:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, m: *const GLdouble)>,
    pub mult_matrixf:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, m: *const GLfloat)>,
    pub new_list:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, list: GLuint, mode: GLenum)>,
    pub normal3b: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, nx: GLbyte, ny: GLbyte, nz: GLbyte),
    >,
    pub normal3bv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLbyte)>,
    pub normal3d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, nx: GLdouble, ny: GLdouble, nz: GLdouble),
    >,
    pub normal3dv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub normal3f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, nx: GLfloat, ny: GLfloat, nz: GLfloat),
    >,
    pub normal3fv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub normal3i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, nx: GLint, ny: GLint, nz: GLint),
    >,
    pub normal3iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub normal3s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, nx: GLshort, ny: GLshort, nz: GLshort),
    >,
    pub normal3sv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub normal_pointer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub ortho: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            left: GLdouble,
            right: GLdouble,
            bottom: GLdouble,
            top: GLdouble,
            zNear: GLdouble,
            zFar: GLdouble,
        ),
    >,
    pub pass_through: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, token: GLfloat)>,
    pub pixel_mapfv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            map: GLenum,
            mapsize: GLsizei,
            values: *const GLfloat,
        ),
    >,
    pub pixel_mapuiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, map: GLenum, mapsize: GLsizei, values: *const GLuint),
    >,
    pub pixel_mapusv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            map: GLenum,
            mapsize: GLsizei,
            values: *const GLushort,
        ),
    >,
    pub pixel_storef:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLfloat)>,
    pub pixel_storei:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLint)>,
    pub pixel_transferf:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLfloat)>,
    pub pixel_transferi:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLint)>,
    pub pixel_zoom: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, xfactor: GLfloat, yfactor: GLfloat),
    >,
    pub point_size: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, size: GLfloat)>,
    pub polygon_mode:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, face: GLenum, mode: GLenum)>,
    pub polygon_offset: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, factor: GLfloat, units: GLfloat),
    >,
    pub polygon_stipple:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mask: *const GLubyte)>,
    pub pop_attrib: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub pop_client_attrib: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub pop_matrix: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub pop_name: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub prioritize_textures: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            n: GLsizei,
            textures: *const GLuint,
            priorities: *const GLclampf,
        ),
    >,
    pub push_attrib: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mask: GLbitfield)>,
    pub push_client_attrib:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mask: GLbitfield)>,
    pub push_matrix: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub push_name: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, name: GLuint)>,
    pub raster_pos2d:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble)>,
    pub raster_pos2dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub raster_pos2f:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat)>,
    pub raster_pos2fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub raster_pos2i:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint)>,
    pub raster_pos2iv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub raster_pos2s:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLshort, y: GLshort)>,
    pub raster_pos2sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub raster_pos3d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble, z: GLdouble),
    >,
    pub raster_pos3dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub raster_pos3f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat, z: GLfloat),
    >,
    pub raster_pos3fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub raster_pos3i:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint, z: GLint)>,
    pub raster_pos3iv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub raster_pos3s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLshort, y: GLshort, z: GLshort),
    >,
    pub raster_pos3sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub raster_pos4d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble),
    >,
    pub raster_pos4dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub raster_pos4f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat),
    >,
    pub raster_pos4fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub raster_pos4i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint, z: GLint, w: GLint),
    >,
    pub raster_pos4iv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub raster_pos4s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLshort, y: GLshort, z: GLshort, w: GLshort),
    >,
    pub raster_pos4sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub read_buffer: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub read_pixels: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            type_: GLenum,
            pixels: *mut GLvoid,
        ),
    >,
    pub rectd: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            x1: GLdouble,
            y1: GLdouble,
            x2: GLdouble,
            y2: GLdouble,
        ),
    >,
    pub rectdv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, v1: *const GLdouble, v2: *const GLdouble),
    >,
    pub rectf: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat),
    >,
    pub rectfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, v1: *const GLfloat, v2: *const GLfloat),
    >,
    pub recti: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x1: GLint, y1: GLint, x2: GLint, y2: GLint),
    >,
    pub rectiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, v1: *const GLint, v2: *const GLint),
    >,
    pub rects: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort),
    >,
    pub rectsv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, v1: *const GLshort, v2: *const GLshort),
    >,
    pub render_mode:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum) -> GLint>,
    pub rotated: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            angle: GLdouble,
            x: GLdouble,
            y: GLdouble,
            z: GLdouble,
        ),
    >,
    pub rotatef: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat),
    >,
    pub scaled: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble, z: GLdouble),
    >,
    pub scalef: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat, z: GLfloat),
    >,
    pub scissor: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint, width: GLsizei, height: GLsizei),
    >,
    pub select_buffer: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLsizei, buffer: *mut GLuint),
    >,
    pub shade_model: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub stencil_func: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, func: GLenum, ref_: GLint, mask: GLuint),
    >,
    pub stencil_mask: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mask: GLuint)>,
    pub stencil_op: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, fail: GLenum, zfail: GLenum, zpass: GLenum),
    >,
    pub tex_coord1d: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLdouble)>,
    pub tex_coord1dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub tex_coord1f: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLfloat)>,
    pub tex_coord1fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub tex_coord1i: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLint)>,
    pub tex_coord1iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub tex_coord1s: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLshort)>,
    pub tex_coord1sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub tex_coord2d:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLdouble, t: GLdouble)>,
    pub tex_coord2dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub tex_coord2f:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLfloat, t: GLfloat)>,
    pub tex_coord2fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub tex_coord2i:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLint, t: GLint)>,
    pub tex_coord2iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub tex_coord2s:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLshort, t: GLshort)>,
    pub tex_coord2sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub tex_coord3d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, s: GLdouble, t: GLdouble, r: GLdouble),
    >,
    pub tex_coord3dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub tex_coord3f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, s: GLfloat, t: GLfloat, r: GLfloat),
    >,
    pub tex_coord3fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub tex_coord3i:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, s: GLint, t: GLint, r: GLint)>,
    pub tex_coord3iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub tex_coord3s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, s: GLshort, t: GLshort, r: GLshort),
    >,
    pub tex_coord3sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub tex_coord4d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble),
    >,
    pub tex_coord4dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub tex_coord4f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat),
    >,
    pub tex_coord4fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub tex_coord4i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, s: GLint, t: GLint, r: GLint, q: GLint),
    >,
    pub tex_coord4iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub tex_coord4s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, s: GLshort, t: GLshort, r: GLshort, q: GLshort),
    >,
    pub tex_coord4sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub tex_coord_pointer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            size: GLint,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub tex_envf: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, param: GLfloat),
    >,
    pub tex_envfv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            pname: GLenum,
            params: *const GLfloat,
        ),
    >,
    pub tex_envi: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, param: GLint),
    >,
    pub tex_enviv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *const GLint),
    >,
    pub tex_gend: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, coord: GLenum, pname: GLenum, param: GLdouble),
    >,
    pub tex_gendv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            coord: GLenum,
            pname: GLenum,
            params: *const GLdouble,
        ),
    >,
    pub tex_genf: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, coord: GLenum, pname: GLenum, param: GLfloat),
    >,
    pub tex_genfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, coord: GLenum, pname: GLenum, params: *const GLfloat),
    >,
    pub tex_geni: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, coord: GLenum, pname: GLenum, param: GLint),
    >,
    pub tex_geniv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, coord: GLenum, pname: GLenum, params: *const GLint),
    >,
    pub tex_image1D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            internalformat: GLenum,
            width: GLsizei,
            border: GLint,
            format: GLenum,
            type_: GLenum,
            pixels: *const GLvoid,
        ),
    >,
    pub tex_image2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
            format: GLenum,
            type_: GLenum,
            pixels: *const GLvoid,
        ),
    >,
    pub tex_parameterf: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, param: GLfloat),
    >,
    pub tex_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            pname: GLenum,
            params: *const GLfloat,
        ),
    >,
    pub tex_parameteri: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, param: GLint),
    >,
    pub tex_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *const GLint),
    >,
    pub tex_sub_image1D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            width: GLsizei,
            format: GLenum,
            type_: GLenum,
            pixels: *const GLvoid,
        ),
    >,
    pub tex_sub_image2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            type_: GLenum,
            pixels: *const GLvoid,
        ),
    >,
    pub translated: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble, z: GLdouble),
    >,
    pub translatef: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat, z: GLfloat),
    >,
    pub vertex2d:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble)>,
    pub vertex2dv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub vertex2f:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat)>,
    pub vertex2fv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub vertex2i: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint)>,
    pub vertex2iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub vertex2s:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLshort, y: GLshort)>,
    pub vertex2sv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub vertex3d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble, z: GLdouble),
    >,
    pub vertex3dv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub vertex3f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat, z: GLfloat),
    >,
    pub vertex3fv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub vertex3i:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint, z: GLint)>,
    pub vertex3iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub vertex3s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLshort, y: GLshort, z: GLshort),
    >,
    pub vertex3sv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub vertex4d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble),
    >,
    pub vertex4dv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub vertex4f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat),
    >,
    pub vertex4fv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub vertex4i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint, z: GLint, w: GLint),
    >,
    pub vertex4iv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub vertex4s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLshort, y: GLshort, z: GLshort, w: GLshort),
    >,
    pub vertex4sv: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub vertex_pointer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            size: GLint,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub viewport: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint, width: GLsizei, height: GLsizei),
    >,
    pub blend_func_separate: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            sfactorRGB: GLenum,
            dfactorRGB: GLenum,
            sfactorAlpha: GLenum,
            dfactorAlpha: GLenum,
        ),
    >,
    pub blend_color: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            red: GLclampf,
            green: GLclampf,
            blue: GLclampf,
            alpha: GLclampf,
        ),
    >,
    pub blend_equation: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub lock_arrays_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, first: GLint, count: GLsizei)>,
    pub unlock_arrays_EXT: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub client_active_texture:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum)>,
    pub active_texture:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum)>,
    pub multi_tex_coord1d:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLdouble)>,
    pub multi_tex_coord1dv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLdouble),
    >,
    pub multi_tex_coord1f:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLfloat)>,
    pub multi_tex_coord1fv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLfloat),
    >,
    pub multi_tex_coord1i:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLint)>,
    pub multi_tex_coord1iv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLint),
    >,
    pub multi_tex_coord1s:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLshort)>,
    pub multi_tex_coord1sv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLshort),
    >,
    pub multi_tex_coord2d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLdouble, t: GLdouble),
    >,
    pub multi_tex_coord2dv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLdouble),
    >,
    pub multi_tex_coord2f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLfloat, t: GLfloat),
    >,
    pub multi_tex_coord2fv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLfloat),
    >,
    pub multi_tex_coord2i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLint, t: GLint),
    >,
    pub multi_tex_coord2iv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLint),
    >,
    pub multi_tex_coord2s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLshort, t: GLshort),
    >,
    pub multi_tex_coord2sv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLshort),
    >,
    pub multi_tex_coord3d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            s: GLdouble,
            t: GLdouble,
            r: GLdouble,
        ),
    >,
    pub multi_tex_coord3dv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLdouble),
    >,
    pub multi_tex_coord3f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat),
    >,
    pub multi_tex_coord3fv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLfloat),
    >,
    pub multi_tex_coord3i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLint, t: GLint, r: GLint),
    >,
    pub multi_tex_coord3iv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLint),
    >,
    pub multi_tex_coord3s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, s: GLshort, t: GLshort, r: GLshort),
    >,
    pub multi_tex_coord3sv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLshort),
    >,
    pub multi_tex_coord4d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            s: GLdouble,
            t: GLdouble,
            r: GLdouble,
            q: GLdouble,
        ),
    >,
    pub multi_tex_coord4dv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLdouble),
    >,
    pub multi_tex_coord4f: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            s: GLfloat,
            t: GLfloat,
            r: GLfloat,
            q: GLfloat,
        ),
    >,
    pub multi_tex_coord4fv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLfloat),
    >,
    pub multi_tex_coord4i: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            s: GLint,
            t: GLint,
            r: GLint,
            q: GLint,
        ),
    >,
    pub multi_tex_coord4iv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLint),
    >,
    pub multi_tex_coord4s: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            s: GLshort,
            t: GLshort,
            r: GLshort,
            q: GLshort,
        ),
    >,
    pub multi_tex_coord4sv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, v: *const GLshort),
    >,
    pub load_transpose_matrixd:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, m: *const GLdouble)>,
    pub load_transpose_matrixf:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, m: *const GLfloat)>,
    pub mult_transpose_matrixd:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, m: *const GLdouble)>,
    pub mult_transpose_matrixf:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, m: *const GLfloat)>,
    pub compressed_tex_image3D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
            depth: GLsizei,
            border: GLint,
            imageSize: GLsizei,
            data: *const GLvoid,
        ),
    >,
    pub compressed_tex_image2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
            border: GLint,
            imageSize: GLsizei,
            data: *const GLvoid,
        ),
    >,
    pub compressed_tex_image1D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            internalformat: GLenum,
            width: GLsizei,
            border: GLint,
            imageSize: GLsizei,
            data: *const GLvoid,
        ),
    >,
    pub compressed_tex_sub_image3D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
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
        ),
    >,
    pub compressed_tex_sub_image2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            imageSize: GLsizei,
            data: *const GLvoid,
        ),
    >,
    pub compressed_tex_sub_image1D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            width: GLsizei,
            format: GLenum,
            imageSize: GLsizei,
            data: *const GLvoid,
        ),
    >,
    pub get_compressed_tex_image: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, level: GLint, img: *mut GLvoid),
    >,
    pub secondary_color3b: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLbyte, green: GLbyte, blue: GLbyte),
    >,
    pub secondary_color3bv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLbyte)>,
    pub secondary_color3d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLdouble, green: GLdouble, blue: GLdouble),
    >,
    pub secondary_color3dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub secondary_color3f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLfloat, green: GLfloat, blue: GLfloat),
    >,
    pub secondary_color3fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub secondary_color3i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLint, green: GLint, blue: GLint),
    >,
    pub secondary_color3iv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub secondary_color3s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLshort, green: GLshort, blue: GLshort),
    >,
    pub secondary_color3sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub secondary_color3ub: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLubyte, green: GLubyte, blue: GLubyte),
    >,
    pub secondary_color3ubv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLubyte)>,
    pub secondary_color3ui: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLuint, green: GLuint, blue: GLuint),
    >,
    pub secondary_color3uiv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLuint)>,
    pub secondary_color3us: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, red: GLushort, green: GLushort, blue: GLushort),
    >,
    pub secondary_color3usv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLushort)>,
    pub secondary_color_pointer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            size: GLint,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub vertex_array_range_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, count: GLsizei, pointer: *const GLvoid),
    >,
    pub flush_vertex_array_range_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, count: GLsizei, pointer: *const GLvoid),
    >,
    pub draw_range_elements: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            start: GLuint,
            end: GLuint,
            count: GLsizei,
            type_: GLenum,
            indices: *const GLvoid,
        ),
    >,
    pub color_table: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            width: GLsizei,
            format: GLenum,
            type_: GLenum,
            table: *const GLvoid,
        ),
    >,
    pub color_table_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            pname: GLenum,
            params: *const GLfloat,
        ),
    >,
    pub color_table_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *const GLint),
    >,
    pub copy_color_table: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            x: GLint,
            y: GLint,
            width: GLsizei,
        ),
    >,
    pub get_color_table: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            format: GLenum,
            type_: GLenum,
            table: *mut GLvoid,
        ),
    >,
    pub get_color_table_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_color_table_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub color_sub_table: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            start: GLsizei,
            count: GLsizei,
            format: GLenum,
            type_: GLenum,
            data: *const GLvoid,
        ),
    >,
    pub copy_color_sub_table: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            start: GLsizei,
            x: GLint,
            y: GLint,
            width: GLsizei,
        ),
    >,
    pub convolution_filter1D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            width: GLsizei,
            format: GLenum,
            type_: GLenum,
            image: *const GLvoid,
        ),
    >,
    pub convolution_filter2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            type_: GLenum,
            image: *const GLvoid,
        ),
    >,
    pub convolution_parameterf: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: GLfloat),
    >,
    pub convolution_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            pname: GLenum,
            params: *const GLfloat,
        ),
    >,
    pub convolution_parameteri: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: GLint),
    >,
    pub convolution_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *const GLint),
    >,
    pub copy_convolution_filter1D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            x: GLint,
            y: GLint,
            width: GLsizei,
        ),
    >,
    pub copy_convolution_filter2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
        ),
    >,
    pub get_convolution_filter: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            format: GLenum,
            type_: GLenum,
            image: *mut GLvoid,
        ),
    >,
    pub get_convolution_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_convolution_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub get_separable_filter: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            format: GLenum,
            type_: GLenum,
            row: *mut GLvoid,
            column: *mut GLvoid,
            span: *mut GLvoid,
        ),
    >,
    pub separable_filter2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
            format: GLenum,
            type_: GLenum,
            row: *const GLvoid,
            column: *const GLvoid,
        ),
    >,
    pub get_histogram: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            reset: GLboolean,
            format: GLenum,
            type_: GLenum,
            values: *mut GLvoid,
        ),
    >,
    pub get_histogram_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_histogram_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub get_minmax: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            reset: GLboolean,
            format: GLenum,
            type_: GLenum,
            values: *mut GLvoid,
        ),
    >,
    pub get_minmax_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_minmax_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub histogram: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            width: GLsizei,
            internalformat: GLenum,
            sink: GLboolean,
        ),
    >,
    pub minmax: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            sink: GLboolean,
        ),
    >,
    pub reset_histogram:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum)>,
    pub reset_minmax: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum)>,
    pub tex_image3D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            internalFormat: GLenum,
            width: GLsizei,
            height: GLsizei,
            depth: GLsizei,
            border: GLint,
            format: GLenum,
            type_: GLenum,
            pixels: *const GLvoid,
        ),
    >,
    pub tex_sub_image3D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
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
        ),
    >,
    pub copy_tex_sub_image3D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            level: GLint,
            xoffset: GLint,
            yoffset: GLint,
            zoffset: GLint,
            x: GLint,
            y: GLint,
            width: GLsizei,
            height: GLsizei,
        ),
    >,
    pub get_uniform_indices: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            uniformCount: GLsizei,
            uniformNames: *const *const GLchar,
            uniformIndices: *mut GLuint,
        ),
    >,
    pub get_active_uniformsiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            uniformCount: GLsizei,
            uniformIndices: *const GLuint,
            pname: GLenum,
            params: *mut GLint,
        ),
    >,
    pub get_active_uniform_name: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            uniformIndex: GLuint,
            bufSize: GLsizei,
            length: *mut GLsizei,
            uniformName: *mut GLchar,
        ),
    >,
    pub get_uniform_block_index: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            uniformBlockName: *const GLchar,
        ) -> GLuint,
    >,
    pub get_active_uniform_blockiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            uniformBlockIndex: GLuint,
            pname: GLenum,
            params: *mut GLint,
        ),
    >,
    pub get_active_uniform_block_name: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            uniformBlockIndex: GLuint,
            bufSize: GLsizei,
            length: *mut GLsizei,
            uniformBlockName: *mut GLchar,
        ),
    >,
    pub uniform_block_binding: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            uniformBlockIndex: GLuint,
            uniformBlockBinding: GLuint,
        ),
    >,
    pub get_combiner_input_parameterfv_NV: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            stage: GLenum,
            portion: GLenum,
            variable: GLenum,
            pname: GLenum,
            params: *mut GLfloat,
        ),
    >,
    pub get_combiner_input_parameteriv_NV: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            stage: GLenum,
            portion: GLenum,
            variable: GLenum,
            pname: GLenum,
            params: *mut GLint,
        ),
    >,
    pub get_combiner_output_parameterfv_NV: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            stage: GLenum,
            portion: GLenum,
            pname: GLenum,
            params: *mut GLfloat,
        ),
    >,
    pub get_combiner_output_parameteriv_NV: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            stage: GLenum,
            portion: GLenum,
            pname: GLenum,
            params: *mut GLint,
        ),
    >,
    pub get_final_combiner_input_parameterfv_NV: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            variable: GLenum,
            pname: GLenum,
            params: *mut GLfloat,
        ),
    >,
    pub get_final_combiner_input_parameteriv_NV: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, variable: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub combiner_stage_parameterfv_NV: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, stage: GLenum, pname: GLenum, params: *const GLfloat),
    >,
    pub get_combiner_stage_parameterfv_NV: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, stage: GLenum, pname: GLenum, params: *mut GLfloat),
    >,
    pub texture_range_APPLE: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            length: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub get_tex_parameter_pointerv_APPLE: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            pname: GLenum,
            params: *mut *mut GLvoid,
        ),
    >,
    pub blend_equation_separate_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, equationRGB: GLenum, equationAlpha: GLenum),
    >,
    pub sample_coverage: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, value: GLclampf, invert: GLboolean),
    >,
    pub sample_pass: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub pn_trianglesi_ATI:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLint)>,
    pub pn_trianglesf_ATI:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLfloat)>,
    pub gen_fences_APPLE: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, fences: *mut GLuint),
    >,
    pub delete_fences_APPLE: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, fences: *const GLuint),
    >,
    pub set_fence_APPLE:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, fence: GLuint)>,
    pub is_fence_APPLE:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, fence: GLuint) -> GLboolean>,
    pub test_fence_APPLE:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, fence: GLuint) -> GLboolean>,
    pub finish_fence_APPLE:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, fence: GLuint)>,
    pub test_object_APPLE: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, object: GLenum, name: GLuint) -> GLboolean,
    >,
    pub finish_object_APPLE:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, object: GLenum, name: GLuint)>,
    pub bind_program_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, program: GLuint),
    >,
    pub delete_programs_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, programs: *const GLuint),
    >,
    pub gen_programs_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, programs: *mut GLuint),
    >,
    pub is_program_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, program: GLuint) -> GLboolean>,
    pub vertex_attrib1s_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLshort)>,
    pub vertex_attrib1f_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLfloat)>,
    pub vertex_attrib1d_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLdouble)>,
    pub vertex_attrib2s_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLshort, y: GLshort),
    >,
    pub vertex_attrib2f_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLfloat, y: GLfloat),
    >,
    pub vertex_attrib2d_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLdouble, y: GLdouble),
    >,
    pub vertex_attrib3s_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLshort, y: GLshort, z: GLshort),
    >,
    pub vertex_attrib3f_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat),
    >,
    pub vertex_attrib3d_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble),
    >,
    pub vertex_attrib4s_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            x: GLshort,
            y: GLshort,
            z: GLshort,
            w: GLshort,
        ),
    >,
    pub vertex_attrib4f_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            x: GLfloat,
            y: GLfloat,
            z: GLfloat,
            w: GLfloat,
        ),
    >,
    pub vertex_attrib4d_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            x: GLdouble,
            y: GLdouble,
            z: GLdouble,
            w: GLdouble,
        ),
    >,
    pub vertex_attrib4Nub_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            x: GLubyte,
            y: GLubyte,
            z: GLubyte,
            w: GLubyte,
        ),
    >,
    pub vertex_attrib1sv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLshort),
    >,
    pub vertex_attrib1fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLfloat),
    >,
    pub vertex_attrib1dv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLdouble),
    >,
    pub vertex_attrib2sv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLshort),
    >,
    pub vertex_attrib2fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLfloat),
    >,
    pub vertex_attrib2dv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLdouble),
    >,
    pub vertex_attrib3sv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLshort),
    >,
    pub vertex_attrib3fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLfloat),
    >,
    pub vertex_attrib3dv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLdouble),
    >,
    pub vertex_attrib4bv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLbyte),
    >,
    pub vertex_attrib4sv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLshort),
    >,
    pub vertex_attrib4iv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLint),
    >,
    pub vertex_attrib4ubv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLubyte),
    >,
    pub vertex_attrib4usv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLushort),
    >,
    pub vertex_attrib4uiv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLuint),
    >,
    pub vertex_attrib4fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLfloat),
    >,
    pub vertex_attrib4dv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLdouble),
    >,
    pub vertex_attrib4Nbv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLbyte),
    >,
    pub vertex_attrib4Nsv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLshort),
    >,
    pub vertex_attrib4Niv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLint),
    >,
    pub vertex_attrib4Nubv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLubyte),
    >,
    pub vertex_attrib4Nusv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLushort),
    >,
    pub vertex_attrib4Nuiv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLuint),
    >,
    pub vertex_attrib_pointer_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            size: GLint,
            type_: GLenum,
            normalized: GLboolean,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub enable_vertex_attrib_array_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint)>,
    pub disable_vertex_attrib_array_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint)>,
    pub get_vertex_attribdv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, pname: GLenum, params: *mut GLdouble),
    >,
    pub get_vertex_attribfv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_vertex_attribiv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, pname: GLenum, params: *mut GLint),
    >,
    pub get_vertex_attrib_pointerv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            pname: GLenum,
            pointer: *mut *mut GLvoid,
        ),
    >,
    pub program_env_parameter4d_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            x: GLdouble,
            y: GLdouble,
            z: GLdouble,
            w: GLdouble,
        ),
    >,
    pub program_env_parameter4dv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            params: *const GLdouble,
        ),
    >,
    pub program_env_parameter4f_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            x: GLfloat,
            y: GLfloat,
            z: GLfloat,
            w: GLfloat,
        ),
    >,
    pub program_env_parameter4fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            params: *const GLfloat,
        ),
    >,
    pub program_local_parameter4d_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            x: GLdouble,
            y: GLdouble,
            z: GLdouble,
            w: GLdouble,
        ),
    >,
    pub program_local_parameter4dv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            params: *const GLdouble,
        ),
    >,
    pub program_local_parameter4f_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            x: GLfloat,
            y: GLfloat,
            z: GLfloat,
            w: GLfloat,
        ),
    >,
    pub program_local_parameter4fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            params: *const GLfloat,
        ),
    >,
    pub get_program_env_parameterdv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint, params: *mut GLdouble),
    >,
    pub get_program_env_parameterfv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint, params: *mut GLfloat),
    >,
    pub get_program_local_parameterdv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint, params: *mut GLdouble),
    >,
    pub get_program_local_parameterfv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint, params: *mut GLfloat),
    >,
    pub program_string_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            format: GLenum,
            len: GLsizei,
            string: *const GLvoid,
        ),
    >,
    pub get_program_string_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, string: *mut GLvoid),
    >,
    pub get_programiv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub enable_vertex_attrib_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint, pname: GLenum)>,
    pub disable_vertex_attrib_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint, pname: GLenum)>,
    pub is_vertex_attrib_enabled_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, pname: GLenum) -> GLboolean,
    >,
    pub map_vertex_attrib1d_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            size: GLuint,
            u1: GLdouble,
            u2: GLdouble,
            stride: GLint,
            order: GLint,
            points: *const GLdouble,
        ),
    >,
    pub map_vertex_attrib1f_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            size: GLuint,
            u1: GLfloat,
            u2: GLfloat,
            stride: GLint,
            order: GLint,
            points: *const GLfloat,
        ),
    >,
    pub map_vertex_attrib2d_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            size: GLuint,
            u1: GLdouble,
            u2: GLdouble,
            ustride: GLint,
            uorder: GLint,
            v1: GLdouble,
            v2: GLdouble,
            vstride: GLint,
            vorder: GLint,
            points: *const GLdouble,
        ),
    >,
    pub map_vertex_attrib2f_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            size: GLuint,
            u1: GLfloat,
            u2: GLfloat,
            ustride: GLint,
            uorder: GLint,
            v1: GLfloat,
            v2: GLfloat,
            vstride: GLint,
            vorder: GLint,
            points: *const GLfloat,
        ),
    >,
    pub point_parameterf:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLfloat)>,
    pub point_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *const GLfloat),
    >,
    pub point_parameteri:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLint)>,
    pub point_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *const GLint),
    >,
    pub fog_coordf: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, coord: GLfloat)>,
    pub fog_coordfv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, coord: *const GLfloat)>,
    pub fog_coordd: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, coord: GLdouble)>,
    pub fog_coorddv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, coord: *const GLdouble)>,
    pub fog_coord_pointer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub vertex_array_parameteri_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, param: GLint)>,
    pub bind_vertex_array_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, id: GLuint)>,
    pub delete_vertex_arrays_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, ids: *const GLuint),
    >,
    pub gen_vertex_arrays_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, ids: *mut GLuint)>,
    pub is_vertex_array_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, id: GLuint) -> GLboolean>,
    pub element_pointer_APPLE: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, type_: GLenum, pointer: *const GLvoid),
    >,
    pub draw_element_array_APPLE: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, mode: GLenum, first: GLint, count: GLsizei),
    >,
    pub draw_range_element_array_APPLE: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            start: GLuint,
            end: GLuint,
            first: GLint,
            count: GLsizei,
        ),
    >,
    pub weightbv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLint, weights: *const GLbyte),
    >,
    pub weightsv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLint, weights: *const GLshort),
    >,
    pub weightiv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLint, weights: *const GLint),
    >,
    pub weightfv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLint, weights: *const GLfloat),
    >,
    pub weightdv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLint, weights: *const GLdouble),
    >,
    pub weightubv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLint, weights: *const GLubyte),
    >,
    pub weightusv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLint, weights: *const GLushort),
    >,
    pub weightuiv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, size: GLint, weights: *const GLuint),
    >,
    pub weight_pointer_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            size: GLint,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub vertex_blend_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, count: GLint)>,
    pub multi_draw_arrays: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            first: *const GLint,
            count: *const GLsizei,
            drawcount: GLsizei,
        ),
    >,
    pub multi_draw_elements: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            count: *const GLsizei,
            type_: GLenum,
            indices: *const *const GLvoid,
            drawcount: GLsizei,
        ),
    >,
    pub window_pos2d:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble)>,
    pub window_pos2dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub window_pos2f:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat)>,
    pub window_pos2fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub window_pos2i:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint)>,
    pub window_pos2iv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub window_pos2s:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLshort, y: GLshort)>,
    pub window_pos2sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub window_pos3d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLdouble, y: GLdouble, z: GLdouble),
    >,
    pub window_pos3dv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLdouble)>,
    pub window_pos3f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLfloat, y: GLfloat, z: GLfloat),
    >,
    pub window_pos3fv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLfloat)>,
    pub window_pos3i:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, x: GLint, y: GLint, z: GLint)>,
    pub window_pos3iv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLint)>,
    pub window_pos3s: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, x: GLshort, y: GLshort, z: GLshort),
    >,
    pub window_pos3sv:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, v: *const GLshort)>,
    pub active_stencil_face_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, face: GLenum)>,
    pub stencil_op_separate_ATI: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            face: GLenum,
            sfail: GLenum,
            dpfail: GLenum,
            dppass: GLenum,
        ),
    >,
    pub stencil_func_separate_ATI: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            frontfunc: GLenum,
            backfunc: GLenum,
            ref_: GLint,
            mask: GLuint,
        ),
    >,
    pub flush_render_APPLE: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub finish_render_APPLE: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub swap_APPLE: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub delete_object_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, obj: GLhandleARB)>,
    pub get_handle_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum) -> GLhandleARB>,
    pub detach_object_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, containerObj: GLhandleARB, attachedObj: GLhandleARB),
    >,
    pub create_shader_object_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, shaderType: GLenum) -> GLhandleARB,
    >,
    pub shader_source_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            shaderObj: GLhandleARB,
            count: GLsizei,
            string: *const *const GLcharARB,
            length: *const GLint,
        ),
    >,
    pub compile_shader_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, shaderObj: GLhandleARB)>,
    pub create_program_object_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext) -> GLhandleARB>,
    pub attach_object_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, containerObj: GLhandleARB, obj: GLhandleARB),
    >,
    pub link_program_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, programObj: GLhandleARB)>,
    pub use_program_object_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, programObj: GLhandleARB)>,
    pub validate_program_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, programObj: GLhandleARB)>,
    pub uniform1f_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, location: GLint, v0: GLfloat)>,
    pub uniform2f_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, v0: GLfloat, v1: GLfloat),
    >,
    pub uniform3f_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            v0: GLfloat,
            v1: GLfloat,
            v2: GLfloat,
        ),
    >,
    pub uniform4f_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            v0: GLfloat,
            v1: GLfloat,
            v2: GLfloat,
            v3: GLfloat,
        ),
    >,
    pub uniform1i_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, location: GLint, v0: GLint)>,
    pub uniform2i_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, v0: GLint, v1: GLint),
    >,
    pub uniform3i_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, v0: GLint, v1: GLint, v2: GLint),
    >,
    pub uniform4i_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            v0: GLint,
            v1: GLint,
            v2: GLint,
            v3: GLint,
        ),
    >,
    pub uniform1fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLfloat,
        ),
    >,
    pub uniform2fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLfloat,
        ),
    >,
    pub uniform3fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLfloat,
        ),
    >,
    pub uniform4fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLfloat,
        ),
    >,
    pub uniform1iv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, count: GLsizei, value: *const GLint),
    >,
    pub uniform2iv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, count: GLsizei, value: *const GLint),
    >,
    pub uniform3iv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, count: GLsizei, value: *const GLint),
    >,
    pub uniform4iv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, count: GLsizei, value: *const GLint),
    >,
    pub uniform_matrix2fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub uniform_matrix3fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub uniform_matrix4fv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub get_object_parameterfv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            obj: GLhandleARB,
            pname: GLenum,
            params: *mut GLfloat,
        ),
    >,
    pub get_object_parameteriv_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, obj: GLhandleARB, pname: GLenum, params: *mut GLint),
    >,
    pub get_info_log_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            obj: GLhandleARB,
            maxLength: GLsizei,
            length: *mut GLsizei,
            infoLog: *mut GLcharARB,
        ),
    >,
    pub get_attached_objects_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            containerObj: GLhandleARB,
            maxCount: GLsizei,
            count: *mut GLsizei,
            obj: *mut GLhandleARB,
        ),
    >,
    pub get_uniform_location_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            programObj: GLhandleARB,
            name: *const GLcharARB,
        ) -> GLint,
    >,
    pub get_active_uniform_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            programObj: GLhandleARB,
            index: GLuint,
            maxLength: GLsizei,
            length: *mut GLsizei,
            size: *mut GLint,
            type_: *mut GLenum,
            name: *mut GLcharARB,
        ),
    >,
    pub get_uniformfv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            programObj: GLhandleARB,
            location: GLint,
            params: *mut GLfloat,
        ),
    >,
    pub get_uniformiv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            programObj: GLhandleARB,
            location: GLint,
            params: *mut GLint,
        ),
    >,
    pub get_shader_source_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            obj: GLhandleARB,
            maxLength: GLsizei,
            length: *mut GLsizei,
            source: *mut GLcharARB,
        ),
    >,
    pub bind_attrib_location_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            programObj: GLhandleARB,
            index: GLuint,
            name: *const GLcharARB,
        ),
    >,
    pub get_active_attrib_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            programObj: GLhandleARB,
            index: GLuint,
            maxLength: GLsizei,
            length: *mut GLsizei,
            size: *mut GLint,
            type_: *mut GLenum,
            name: *mut GLcharARB,
        ),
    >,
    pub get_attrib_location_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            programObj: GLhandleARB,
            name: *const GLcharARB,
        ) -> GLint,
    >,
    pub clamp_color_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, clamp: GLenum)>,
    pub gen_queries:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, ids: *mut GLuint)>,
    pub delete_queries: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, ids: *const GLuint),
    >,
    pub is_query:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, id: GLuint) -> GLboolean>,
    pub begin_query:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, id: GLuint)>,
    pub end_query: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum)>,
    pub get_queryiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub get_query_objectiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, id: GLuint, pname: GLenum, params: *mut GLint),
    >,
    pub get_query_objectuiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, id: GLuint, pname: GLenum, params: *mut GLuint),
    >,
    pub bind_buffer: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, buffer: GLuint),
    >,
    pub delete_buffers: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, buffers: *const GLuint),
    >,
    pub gen_buffers: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, buffers: *mut GLuint),
    >,
    pub is_buffer:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, buffer: GLuint) -> GLboolean>,
    pub buffer_data: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            size: GLsizeiptrARB,
            data: *const GLvoid,
            usage: GLenum,
        ),
    >,
    pub buffer_sub_data: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            offset: GLintptrARB,
            size: GLsizeiptrARB,
            data: *const GLvoid,
        ),
    >,
    pub get_buffer_sub_data: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            offset: GLintptrARB,
            size: GLsizeiptrARB,
            data: *mut GLvoid,
        ),
    >,
    pub map_buffer: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, access: GLenum) -> *mut GLvoid,
    >,
    pub unmap_buffer:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum) -> GLboolean>,
    pub get_buffer_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub get_buffer_pointerv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            pname: GLenum,
            params: *mut *mut GLvoid,
        ),
    >,
    pub depth_bounds_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, zmin: GLclampd, zmax: GLclampd),
    >,
    pub draw_buffers_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, bufs: *const GLenum),
    >,
    pub is_shader:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, shader: GLuint) -> GLboolean>,
    pub is_program:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, program: GLuint) -> GLboolean>,
    pub get_shaderiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, shader: GLuint, pname: GLenum, params: *mut GLint),
    >,
    pub get_programiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, pname: GLenum, params: *mut GLint),
    >,
    pub get_shader_info_log: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            shader: GLuint,
            bufSize: GLsizei,
            length: *mut GLsizei,
            infoLog: *mut GLchar,
        ),
    >,
    pub get_program_info_log: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            bufSize: GLsizei,
            length: *mut GLsizei,
            infoLog: *mut GLchar,
        ),
    >,
    pub stencil_func_separate: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            face: GLenum,
            func: GLenum,
            ref_: GLint,
            mask: GLuint,
        ),
    >,
    pub stencil_mask_separate:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, face: GLenum, mask: GLuint)>,
    pub multi_draw_element_array_APPLE: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            first: *const GLint,
            count: *const GLsizei,
            primcount: GLsizei,
        ),
    >,
    pub multi_draw_range_element_array_APPLE: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            start: GLuint,
            end: GLuint,
            first: *const GLint,
            count: *const GLsizei,
            primcount: GLsizei,
        ),
    >,
    pub is_renderbuffer_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, renderbuffer: GLuint) -> GLboolean,
    >,
    pub bind_renderbuffer_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, renderbuffer: GLuint),
    >,
    pub delete_renderbuffers_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, renderbuffers: *const GLuint),
    >,
    pub gen_renderbuffers_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, renderbuffers: *mut GLuint),
    >,
    pub renderbuffer_storage_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
        ),
    >,
    pub get_renderbuffer_parameteriv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub is_framebuffer_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, framebuffer: GLuint) -> GLboolean,
    >,
    pub bind_framebuffer_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, framebuffer: GLuint),
    >,
    pub delete_framebuffers_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, framebuffers: *const GLuint),
    >,
    pub gen_framebuffers_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, framebuffers: *mut GLuint),
    >,
    pub check_framebuffer_status_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum) -> GLenum>,
    pub framebuffer_texture1D_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            attachment: GLenum,
            textarget: GLenum,
            texture: GLuint,
            level: GLint,
        ),
    >,
    pub framebuffer_texture2D_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            attachment: GLenum,
            textarget: GLenum,
            texture: GLuint,
            level: GLint,
        ),
    >,
    pub framebuffer_texture3D_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            attachment: GLenum,
            textarget: GLenum,
            texture: GLuint,
            level: GLint,
            zoffset: GLint,
        ),
    >,
    pub framebuffer_renderbuffer_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            attachment: GLenum,
            renderbuffertarget: GLenum,
            renderbuffer: GLuint,
        ),
    >,
    pub get_framebuffer_attachment_parameteriv_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            attachment: GLenum,
            pname: GLenum,
            params: *mut GLint,
        ),
    >,
    pub generate_mipmap_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum)>,
    pub buffer_parameteri_APPLE: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, param: GLint),
    >,
    pub flush_mapped_buffer_range_APPLE: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, offset: GLintptr, size: GLsizeiptr),
    >,
    pub program_env_parameters4fv_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            count: GLsizei,
            params: *const GLfloat,
        ),
    >,
    pub program_local_parameters4fv_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            count: GLsizei,
            params: *const GLfloat,
        ),
    >,
    pub object_purgeable_APPLE: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            objectType: GLenum,
            name: GLuint,
            option: GLenum,
        ) -> GLenum,
    >,
    pub object_unpurgeable_APPLE: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            objectType: GLenum,
            name: GLuint,
            option: GLenum,
        ) -> GLenum,
    >,
    pub get_object_parameteriv_APPLE: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            objectType: GLenum,
            name: GLuint,
            pname: GLenum,
            params: *mut GLint,
        ),
    >,
    pub program_parameteri_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program_name: GLuint, pname: GLenum, value: GLint),
    >,
    pub framebuffer_texture_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            attachment: GLenum,
            texture: GLuint,
            level: GLint,
        ),
    >,
    pub framebuffer_texture_layer_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            attachment: GLenum,
            texture: GLuint,
            level: GLint,
            layer: GLint,
        ),
    >,
    pub framebuffer_texture_face_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            attachment: GLenum,
            texture: GLuint,
            level: GLint,
            face: GLenum,
        ),
    >,
    pub bind_buffer_range_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            buffer: GLuint,
            offset: GLintptr,
            size: GLsizeiptr,
        ),
    >,
    pub bind_buffer_offset_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            buffer: GLuint,
            offset: GLintptr,
        ),
    >,
    pub bind_buffer_base_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint, buffer: GLuint),
    >,
    pub begin_transform_feedback_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, primitiveMode: GLenum)>,
    pub end_transform_feedback_EXT: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub transform_feedback_varyings_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            count: GLsizei,
            varyings: *const *const GLchar,
            bufferMode: GLenum,
        ),
    >,
    pub get_transform_feedback_varying_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            index: GLuint,
            bufSize: GLsizei,
            length: *mut GLsizei,
            size: *mut GLsizei,
            type_: *mut GLenum,
            name: *mut GLchar,
        ),
    >,
    pub get_integer_indexedv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, param: GLenum, index: GLuint, values: *mut GLint),
    >,
    pub get_boolean_indexedv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, param: GLenum, index: GLuint, values: *mut GLboolean),
    >,
    pub uniform_buffer_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, location: GLint, buffer: GLuint),
    >,
    pub get_uniform_buffer_size_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, location: GLint) -> GLint,
    >,
    pub get_uniform_buffer_offset_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, location: GLint) -> GLintptr,
    >,
    pub clear_colorIi_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, r: GLint, g: GLint, b: GLint, a: GLint),
    >,
    pub clear_colorIui_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, r: GLuint, g: GLuint, b: GLuint, a: GLuint),
    >,
    pub tex_parameterIiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub tex_parameterIuiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLuint),
    >,
    pub get_tex_parameterIiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint),
    >,
    pub get_tex_parameterIuiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLuint),
    >,
    pub vertex_attribI1i_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLint)>,
    pub vertex_attribI2i_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLint, y: GLint),
    >,
    pub vertex_attribI3i_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLint, y: GLint, z: GLint),
    >,
    pub vertex_attribI4i_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            x: GLint,
            y: GLint,
            z: GLint,
            w: GLint,
        ),
    >,
    pub vertex_attribI1ui_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLuint)>,
    pub vertex_attribI2ui_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLuint, y: GLuint),
    >,
    pub vertex_attribI3ui_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLuint, y: GLuint, z: GLuint),
    >,
    pub vertex_attribI4ui_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            x: GLuint,
            y: GLuint,
            z: GLuint,
            w: GLuint,
        ),
    >,
    pub vertex_attribI1iv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLint),
    >,
    pub vertex_attribI2iv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLint),
    >,
    pub vertex_attribI3iv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLint),
    >,
    pub vertex_attribI4iv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLint),
    >,
    pub vertex_attribI1uiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLuint),
    >,
    pub vertex_attribI2uiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLuint),
    >,
    pub vertex_attribI3uiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLuint),
    >,
    pub vertex_attribI4uiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLuint),
    >,
    pub vertex_attribI4bv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLbyte),
    >,
    pub vertex_attribI4sv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLshort),
    >,
    pub vertex_attribI4ubv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLubyte),
    >,
    pub vertex_attribI4usv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLushort),
    >,
    pub vertex_attribI_pointer_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            size: GLint,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub get_vertex_attribIiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, pname: GLenum, params: *mut GLint),
    >,
    pub get_vertex_attribIuiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, pname: GLenum, params: *mut GLuint),
    >,
    pub uniform1ui_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, location: GLint, v0: GLuint)>,
    pub uniform2ui_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, v0: GLuint, v1: GLuint),
    >,
    pub uniform3ui_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint),
    >,
    pub uniform4ui_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            v0: GLuint,
            v1: GLuint,
            v2: GLuint,
            v3: GLuint,
        ),
    >,
    pub uniform1uiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLuint,
        ),
    >,
    pub uniform2uiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLuint,
        ),
    >,
    pub uniform3uiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLuint,
        ),
    >,
    pub uniform4uiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLuint,
        ),
    >,
    pub get_uniformuiv_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            params: *mut GLuint,
        ),
    >,
    pub bind_frag_data_location_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            colorNumber: GLuint,
            name: *const GLchar,
        ),
    >,
    pub get_frag_data_location_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, name: *const GLchar) -> GLint,
    >,
    pub color_mask_indexed_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            r: GLboolean,
            g: GLboolean,
            b: GLboolean,
            a: GLboolean,
        ),
    >,
    pub enable_indexed_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint)>,
    pub disable_indexed_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint)>,
    pub is_enabled_indexed_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint) -> GLboolean,
    >,
    pub uniform_matrix2x3fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub uniform_matrix3x2fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub uniform_matrix2x4fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub uniform_matrix4x2fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub uniform_matrix3x4fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub uniform_matrix4x3fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub blit_framebuffer_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
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
        ),
    >,
    pub renderbuffer_storage_multisample_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            samples: GLsizei,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
        ),
    >,
    pub begin_conditional_render_NV:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, id: GLuint, mode: GLenum)>,
    pub end_conditional_render_NV: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub get_attached_shaders: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            maxCount: GLsizei,
            count: *mut GLsizei,
            shaders: *mut GLuint,
        ),
    >,
    pub provoking_vertex_EXT:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum)>,
    pub vertex_attrib_divisor: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, divisor: GLuint),
    >,
    pub draw_arrays_instanced: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            first: GLint,
            count: GLsizei,
            instancecount: GLsizei,
        ),
    >,
    pub draw_elements_instanced: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            count: GLsizei,
            type_: GLenum,
            indices: *const GLvoid,
            instancecount: GLsizei,
        ),
    >,
    pub draw_elements_base_vertex: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            count: GLsizei,
            type_: GLenum,
            indices: *const GLvoid,
            base_vertex: GLint,
        ),
    >,
    pub draw_range_elements_base_vertex: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            start: GLuint,
            end: GLuint,
            count: GLsizei,
            type_: GLenum,
            indices: *const GLvoid,
            base_vertex: GLint,
        ),
    >,
    pub draw_elements_instanced_base_vertex: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            count: GLsizei,
            type_: GLenum,
            indices: *const GLvoid,
            instancecount: GLsizei,
            base_vertex: GLint,
        ),
    >,
    pub multi_draw_elements_base_vertex: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            count: *const GLsizei,
            type_: GLenum,
            indices: *const *const GLvoid,
            drawcount: GLsizei,
            base_vertex: *const GLint,
        ),
    >,
    pub bind_vertex_array_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, array: GLuint)>,
    pub delete_vertex_arrays_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, arrays: *const GLuint),
    >,
    pub gen_vertex_arrays_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, arrays: *mut GLuint),
    >,
    pub is_vertex_array_ARB:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, array: GLuint) -> GLboolean>,
    pub point_size_pointer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const GLvoid,
        ),
    >,
    pub vertex_point_sizef_APPLE:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, size: GLfloat)>,
    pub clear_bufferiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            buffer: GLenum,
            drawbuffer: GLint,
            value: *const GLint,
        ),
    >,
    pub clear_bufferuiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            buffer: GLenum,
            drawbuffer: GLint,
            value: *const GLuint,
        ),
    >,
    pub clear_bufferfv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            buffer: GLenum,
            drawbuffer: GLint,
            value: *const GLfloat,
        ),
    >,
    pub clear_bufferfi: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            buffer: GLenum,
            drawbuffer: GLint,
            depth: GLfloat,
            stencil: GLint,
        ),
    >,
    pub get_stringi: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, name: GLenum, index: GLuint) -> *const GLubyte,
    >,
    pub fence_sync: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, condition: GLenum, flags: GLbitfield) -> GLsync,
    >,
    pub is_sync:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, sync: GLsync) -> GLboolean>,
    pub delete_sync: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, sync: GLsync)>,
    pub client_wait_sync: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            sync: GLsync,
            flags: GLbitfield,
            timeout: GLuint64,
        ) -> GLenum,
    >,
    pub wait_sync: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sync: GLsync, flags: GLbitfield, timeout: GLuint64),
    >,
    pub get_integer64v_sync: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, params: *mut GLint64),
    >,
    pub get_synciv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            sync: GLsync,
            pname: GLenum,
            bufSize: GLsizei,
            length: *mut GLsizei,
            values: *mut GLint,
        ),
    >,
    pub tex_image2D_multisample: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            samples: GLsizei,
            internalformat: GLint,
            width: GLsizei,
            height: GLsizei,
            fixedsamplelocations: GLboolean,
        ),
    >,
    pub tex_image3D_multisample: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            samples: GLsizei,
            internalformat: GLint,
            width: GLsizei,
            height: GLsizei,
            depth: GLsizei,
            fixedsamplelocations: GLboolean,
        ),
    >,
    pub get_multisamplefv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, index: GLuint, val: *mut GLfloat),
    >,
    pub sample_maski: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, mask: GLbitfield),
    >,
    pub tex_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            buffer: GLuint,
        ),
    >,
    pub copy_buffer_sub_data: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            readtarget: GLenum,
            writetarget: GLenum,
            readoffset: GLintptr,
            writeoffset: GLintptr,
            size: GLsizeiptr,
        ),
    >,
    pub primitive_restart_index:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint)>,
    pub get_query_objecti64v: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, id: GLuint, pname: GLenum, params: *mut GLint64EXT),
    >,
    pub get_query_objectui64v: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, id: GLuint, pname: GLenum, params: *mut GLuint64EXT),
    >,
    pub map_buffer_range: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            offset: GLintptr,
            length: GLsizeiptr,
            access: GLenum,
        ) -> *mut GLvoid,
    >,
    pub flush_mapped_buffer_range: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, offset: GLintptr, length: GLsizeiptr),
    >,
    pub query_counter:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, id: GLuint, target: GLenum)>,
    pub get_integer64i_v: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint, data: *mut GLint64),
    >,
    pub get_buffer_parameteri64v: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, pname: GLenum, params: *mut GLint64),
    >,
    pub gen_samplers: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, count: GLsizei, samplers: *mut GLuint),
    >,
    pub delete_samplers: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, count: GLsizei, samplers: *const GLuint),
    >,
    pub is_sampler:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint) -> GLboolean>,
    pub bind_sampler:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, unit: GLuint, sampler: GLuint)>,
    pub sampler_parameteri: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint, pname: GLenum, param: GLint),
    >,
    pub sampler_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint, pname: GLenum, param: *const GLint),
    >,
    pub sampler_parameterf: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint, pname: GLenum, param: GLfloat),
    >,
    pub sampler_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            sampler: GLuint,
            pname: GLenum,
            param: *const GLfloat,
        ),
    >,
    pub sampler_parameterIiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint, pname: GLenum, param: *const GLint),
    >,
    pub sampler_parameterIuiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint, pname: GLenum, param: *const GLuint),
    >,
    pub get_sampler_parameteriv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint, pname: GLenum, params: *mut GLint),
    >,
    pub get_sampler_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint, pname: GLenum, params: *mut GLfloat),
    >,
    pub get_sampler_parameterIiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint, pname: GLenum, params: *mut GLint),
    >,
    pub get_sampler_parameterIuiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, sampler: GLuint, pname: GLenum, params: *mut GLuint),
    >,
    pub label_object_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            type_: GLenum,
            object: GLuint,
            length: GLsizei,
            label: *const GLchar,
        ),
    >,
    pub get_object_label_EXT: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            type_: GLenum,
            object: GLuint,
            bufSize: GLsizei,
            length: *mut GLsizei,
            label: *mut GLchar,
        ),
    >,
    pub insert_event_marker_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, length: GLsizei, marker: *const GLchar),
    >,
    pub push_group_marker_EXT: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, length: GLsizei, marker: *const GLchar),
    >,
    pub pop_group_marker_EXT: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub use_program_stages: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            pipeline: GLuint,
            stages: GLbitfield,
            program: GLuint,
        ),
    >,
    pub active_shader_program: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pipeline: GLuint, program: GLuint),
    >,
    pub create_shader_programv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            type_: GLenum,
            count: GLsizei,
            strings: *const *const GLchar,
        ) -> GLuint,
    >,
    pub bind_program_pipeline:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pipeline: GLuint)>,
    pub delete_program_pipelines: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, pipelines: *const GLuint),
    >,
    pub gen_program_pipelines: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, pipelines: *mut GLuint),
    >,
    pub is_program_pipeline:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pipeline: GLuint) -> GLboolean>,
    pub get_program_pipelineiv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pipeline: GLuint, pname: GLenum, params: *mut GLint),
    >,
    pub validate_program_pipeline:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pipeline: GLuint)>,
    pub get_program_pipeline_info_log: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            pipeline: GLuint,
            bufSize: GLsizei,
            length: *mut GLsizei,
            infoLog: *mut GLchar,
        ),
    >,
    pub program_uniform1i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, location: GLint, x: GLint),
    >,
    pub program_uniform2i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, location: GLint, x: GLint, y: GLint),
    >,
    pub program_uniform3i: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLint,
            y: GLint,
            z: GLint,
        ),
    >,
    pub program_uniform4i: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLint,
            y: GLint,
            z: GLint,
            w: GLint,
        ),
    >,
    pub program_uniform1f: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, location: GLint, x: GLfloat),
    >,
    pub program_uniform2f: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLfloat,
            y: GLfloat,
        ),
    >,
    pub program_uniform3f: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLfloat,
            y: GLfloat,
            z: GLfloat,
        ),
    >,
    pub program_uniform4f: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLfloat,
            y: GLfloat,
            z: GLfloat,
            w: GLfloat,
        ),
    >,
    pub program_uniform1iv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLint,
        ),
    >,
    pub program_uniform2iv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLint,
        ),
    >,
    pub program_uniform3iv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLint,
        ),
    >,
    pub program_uniform4iv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLint,
        ),
    >,
    pub program_uniform1fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform2fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform3fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform4fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform_matrix2fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform_matrix3fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform_matrix4fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform1ui: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, location: GLint, x: GLuint),
    >,
    pub program_uniform2ui: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLuint,
            y: GLuint,
        ),
    >,
    pub program_uniform3ui: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLuint,
            y: GLuint,
            z: GLuint,
        ),
    >,
    pub program_uniform4ui: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLuint,
            y: GLuint,
            z: GLuint,
            w: GLuint,
        ),
    >,
    pub program_uniform1uiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLuint,
        ),
    >,
    pub program_uniform2uiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLuint,
        ),
    >,
    pub program_uniform3uiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLuint,
        ),
    >,
    pub program_uniform4uiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLuint,
        ),
    >,
    pub program_uniform_matrix2x3fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform_matrix3x2fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform_matrix2x4fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform_matrix4x2fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform_matrix3x4fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub program_uniform_matrix4x3fv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ),
    >,
    pub bind_frag_data_location_indexed: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            colorNumber: GLuint,
            index: GLuint,
            name: *const GLchar,
        ),
    >,
    pub get_frag_data_index: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, name: *const GLchar) -> GLint,
    >,
    pub blend_func_i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, buf: GLuint, src: GLenum, dst: GLenum),
    >,
    pub blend_func_separate_i: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            buf: GLuint,
            srcRGB: GLenum,
            dstRGB: GLenum,
            srcAlpha: GLenum,
            dstAlpha: GLenum,
        ),
    >,
    pub blend_equation_i:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, buf: GLuint, mode: GLenum)>,
    pub blend_equation_separate_i: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum),
    >,
    pub named_string_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            type_: GLenum,
            namelen: GLint,
            name: *const GLchar,
            stringlen: GLint,
            string: *const GLchar,
        ),
    >,
    pub delete_named_string_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, namelen: GLint, name: *const GLchar),
    >,
    pub compile_shader_include_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            shader: GLuint,
            count: GLsizei,
            path: *const *const GLchar,
            length: *const GLint,
        ),
    >,
    pub is_named_string_ARB: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, namelen: GLint, name: *const GLchar) -> GLboolean,
    >,
    pub get_named_string_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            namelen: GLint,
            name: *const GLchar,
            bufSize: GLsizei,
            stringlen: *mut GLint,
            string: *mut GLchar,
        ),
    >,
    pub get_named_string_iv_ARB: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            namelen: GLint,
            name: *const GLchar,
            pname: GLenum,
            params: *mut GLint,
        ),
    >,
    pub release_shader_compiler: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub shader_binary: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            n: GLint,
            shaders: *mut GLuint,
            binaryformat: GLenum,
            binary: *const GLvoid,
            length: GLint,
        ),
    >,
    pub get_shader_precision_format: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            shadertype: GLenum,
            precisiontype: GLenum,
            range: *mut GLint,
            precision: *mut GLint,
        ),
    >,
    pub depth_rangef: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, zNear: GLclampf, zFar: GLclampf),
    >,
    pub clear_depthf: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, depth: GLclampf)>,
    pub vertex_attribP1ui: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            type_: GLenum,
            normalized: GLboolean,
            value: GLuint,
        ),
    >,
    pub vertex_attribP2ui: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            type_: GLenum,
            normalized: GLboolean,
            value: GLuint,
        ),
    >,
    pub vertex_attribP3ui: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            type_: GLenum,
            normalized: GLboolean,
            value: GLuint,
        ),
    >,
    pub vertex_attribP4ui: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            type_: GLenum,
            normalized: GLboolean,
            value: GLuint,
        ),
    >,
    pub vertex_attribP1uiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            type_: GLenum,
            normalized: GLboolean,
            value: *const GLuint,
        ),
    >,
    pub vertex_attribP2uiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            type_: GLenum,
            normalized: GLboolean,
            value: *const GLuint,
        ),
    >,
    pub vertex_attribP3uiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            type_: GLenum,
            normalized: GLboolean,
            value: *const GLuint,
        ),
    >,
    pub vertex_attribP4uiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            type_: GLenum,
            normalized: GLboolean,
            value: *const GLuint,
        ),
    >,
    pub get_program_binary: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            bufSize: GLsizei,
            length: *mut GLsizei,
            binaryFormat: *mut GLenum,
            binary: *mut GLvoid,
        ),
    >,
    pub program_binary: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            binaryFormat: GLenum,
            binary: *const GLvoid,
            length: GLsizei,
        ),
    >,
    pub min_sample_shading:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, value: GLclampf)>,
    pub viewport_arrayv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, first: GLuint, count: GLsizei, v: *const GLfloat),
    >,
    pub viewport_indexedf: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            x: GLfloat,
            y: GLfloat,
            w: GLfloat,
            h: GLfloat,
        ),
    >,
    pub viewport_indexedfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLfloat),
    >,
    pub scissor_arrayv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, first: GLuint, count: GLsizei, v: *const GLint),
    >,
    pub scissor_indexed: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            left: GLint,
            bottom: GLint,
            width: GLsizei,
            height: GLsizei,
        ),
    >,
    pub scissor_indexedv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLint),
    >,
    pub depth_range_arrayv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, first: GLuint, count: GLsizei, v: *const GLclampd),
    >,
    pub depth_range_indexed: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, n: GLclampd, f: GLclampd),
    >,
    pub get_floati_v: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint, data: *mut GLfloat),
    >,
    pub get_doublei_v: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint, data: *mut GLdouble),
    >,
    pub draw_arrays_indirect: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, mode: GLenum, indirect: *const GLvoid),
    >,
    pub draw_elements_indirect: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, mode: GLenum, type_: GLenum, indirect: *const GLvoid),
    >,
    pub patch_parameteri:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, value: GLint)>,
    pub patch_parameterfv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, pname: GLenum, values: *const GLfloat),
    >,
    pub bind_transform_feedback:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, name: GLuint)>,
    pub gen_transform_feedbacks:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, ids: *mut GLuint)>,
    pub delete_transform_feedbacks: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, n: GLsizei, ids: *const GLuint),
    >,
    pub pause_transform_feedback: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub resume_transform_feedback: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub is_transform_feedback:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, name: GLuint) -> GLboolean>,
    pub draw_transform_feedback:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, mode: GLenum, name: GLuint)>,
    pub begin_query_indexed: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint, id: GLuint),
    >,
    pub end_query_indexed:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, target: GLenum, index: GLuint)>,
    pub get_query_indexediv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            index: GLuint,
            pname: GLenum,
            params: *mut GLint,
        ),
    >,
    pub draw_transform_feedback_stream: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, mode: GLenum, name: GLuint, stream: GLuint),
    >,
    pub program_uniform1d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, program: GLuint, location: GLint, x: GLdouble),
    >,
    pub program_uniform2d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLdouble,
            y: GLdouble,
        ),
    >,
    pub program_uniform3d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLdouble,
            y: GLdouble,
            z: GLdouble,
        ),
    >,
    pub program_uniform4d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            x: GLdouble,
            y: GLdouble,
            z: GLdouble,
            w: GLdouble,
        ),
    >,
    pub program_uniform1dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform2dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform3dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform4dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform_matrix2dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform_matrix3dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform_matrix4dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform_matrix2x3dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform_matrix3x2dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform_matrix2x4dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform_matrix4x2dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform_matrix3x4dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub program_uniform_matrix4x3dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub uniform1d:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, location: GLint, x: GLdouble)>,
    pub uniform2d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, location: GLint, x: GLdouble, y: GLdouble),
    >,
    pub uniform3d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            x: GLdouble,
            y: GLdouble,
            z: GLdouble,
        ),
    >,
    pub uniform4d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            x: GLdouble,
            y: GLdouble,
            z: GLdouble,
            w: GLdouble,
        ),
    >,
    pub uniform1dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLdouble,
        ),
    >,
    pub uniform2dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLdouble,
        ),
    >,
    pub uniform3dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLdouble,
        ),
    >,
    pub uniform4dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            value: *const GLdouble,
        ),
    >,
    pub uniform_matrix2dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub uniform_matrix3dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub uniform_matrix4dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub uniform_matrix2x3dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub uniform_matrix3x2dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub uniform_matrix2x4dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub uniform_matrix4x2dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub uniform_matrix3x4dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub uniform_matrix4x3dv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLdouble,
        ),
    >,
    pub get_uniformdv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program_obj: GLuint,
            location: GLint,
            params: *mut GLdouble,
        ),
    >,
    pub vertex_attribl1d:
        ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLdouble)>,
    pub vertex_attribl2d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLdouble, y: GLdouble),
    >,
    pub vertex_attribl3d: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble),
    >,
    pub vertex_attribl4d: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            x: GLdouble,
            y: GLdouble,
            z: GLdouble,
            w: GLdouble,
        ),
    >,
    pub vertex_attribl1dv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLdouble),
    >,
    pub vertex_attribl2dv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLdouble),
    >,
    pub vertex_attribl3dv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLdouble),
    >,
    pub vertex_attribl4dv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, v: *const GLdouble),
    >,
    pub vertex_attrib_lpointer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            index: GLuint,
            size: GLint,
            type_: GLenum,
            stride: GLsizei,
            pointer: *const ::std::os::raw::c_void,
        ),
    >,
    pub get_vertex_attrib_ldv: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, index: GLuint, pname: GLenum, params: *mut GLdouble),
    >,
    pub get_subroutine_uniform_location: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            shadertype: GLenum,
            name: *const GLchar,
        ) -> GLint,
    >,
    pub get_subroutine_index: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            shadertype: GLenum,
            name: *const GLchar,
        ) -> GLuint,
    >,
    pub get_active_subroutine_uniformiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            shadertype: GLenum,
            index: GLuint,
            pname: GLenum,
            values: *mut GLint,
        ),
    >,
    pub get_active_subroutine_uniform_name: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            shadertype: GLenum,
            index: GLuint,
            bufsize: GLsizei,
            length: *mut GLsizei,
            name: *mut GLchar,
        ),
    >,
    pub get_active_subroutine_name: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            shadertype: GLenum,
            index: GLuint,
            bufsize: GLsizei,
            length: *mut GLsizei,
            name: *mut GLchar,
        ),
    >,
    pub uniform_subroutinesuiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            shadertype: GLenum,
            count: GLsizei,
            indices: *const GLuint,
        ),
    >,
    pub get_uniform_subroutineuiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            shadertype: GLenum,
            location: GLint,
            params: *mut GLuint,
        ),
    >,
    pub get_program_stageiv: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            program: GLuint,
            shadertype: GLenum,
            pname: GLenum,
            values: *mut GLint,
        ),
    >,
    pub get_internal_formativ: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            internalformat: GLenum,
            pname: GLenum,
            bufSize: GLsizei,
            params: *mut GLint,
        ),
    >,
    pub tex_storage1D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            levels: GLsizei,
            internalformat: GLenum,
            width: GLsizei,
        ),
    >,
    pub tex_storage2D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            levels: GLsizei,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
        ),
    >,
    pub tex_storage3D: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            target: GLenum,
            levels: GLsizei,
            internalformat: GLenum,
            width: GLsizei,
            height: GLsizei,
            depth: GLsizei,
        ),
    >,
    pub label_object_with_responsible_process_APPLE: ::std::option::Option<
        unsafe extern "C" fn(ctx: GLIContext, type_: GLenum, name: GLuint, pid: GLint),
    >,
    pub texture_barrier_NV: ::std::option::Option<unsafe extern "C" fn(ctx: GLIContext)>,
    pub multi_draw_elements_indirect_APPLE: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: GLIContext,
            mode: GLenum,
            type_: GLenum,
            indirect: *const GLvoid,
            drawcount: GLsizei,
            stride: GLsizei,
        ),
    >,
}
pub type GLIFunctionDispatch = __GLIFunctionDispatchRec;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CGLPrivateObject {
    _unused: [u8; 0],
}
pub type CGLPrivateObj = *mut _CGLPrivateObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CGLContextObject {
    pub rend: GLIContext,
    pub disp: GLIFunctionDispatch,
    pub priv_: CGLPrivateObj,
    pub stak: *mut ::std::os::raw::c_void,
}
unsafe extern "C" {
    pub fn CGLGetShareGroup(ctx: CGLContextObj) -> CGLShareGroupObj;
}
unsafe extern "C" {
    pub fn CGLGetDeviceFromGLRenderer(rendererID: GLint) -> cl_device_id;
}
unsafe extern "C" {
    pub fn CGLTexImageIOSurface2D(
        ctx: CGLContextObj,
        target: GLenum,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        ioSurface: IOSurfaceRef,
        plane: GLuint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLChoosePixelFormat(
        attribs: *const CGLPixelFormatAttribute,
        pix: *mut CGLPixelFormatObj,
        npix: *mut GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLDestroyPixelFormat(pix: CGLPixelFormatObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLDescribePixelFormat(
        pix: CGLPixelFormatObj,
        pix_num: GLint,
        attrib: CGLPixelFormatAttribute,
        value: *mut GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLReleasePixelFormat(pix: CGLPixelFormatObj);
}
unsafe extern "C" {
    pub fn CGLRetainPixelFormat(pix: CGLPixelFormatObj) -> CGLPixelFormatObj;
}
unsafe extern "C" {
    pub fn CGLGetPixelFormatRetainCount(pix: CGLPixelFormatObj) -> GLuint;
}
unsafe extern "C" {
    pub fn CGLQueryRendererInfo(
        display_mask: GLuint,
        rend: *mut CGLRendererInfoObj,
        nrend: *mut GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLDestroyRendererInfo(rend: CGLRendererInfoObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLDescribeRenderer(
        rend: CGLRendererInfoObj,
        rend_num: GLint,
        prop: CGLRendererProperty,
        value: *mut GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLCreateContext(
        pix: CGLPixelFormatObj,
        share: CGLContextObj,
        ctx: *mut CGLContextObj,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLDestroyContext(ctx: CGLContextObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLCopyContext(src: CGLContextObj, dst: CGLContextObj, mask: GLbitfield) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLRetainContext(ctx: CGLContextObj) -> CGLContextObj;
}
unsafe extern "C" {
    pub fn CGLReleaseContext(ctx: CGLContextObj);
}
unsafe extern "C" {
    pub fn CGLGetContextRetainCount(ctx: CGLContextObj) -> GLuint;
}
unsafe extern "C" {
    pub fn CGLGetPixelFormat(ctx: CGLContextObj) -> CGLPixelFormatObj;
}
unsafe extern "C" {
    pub fn CGLCreatePBuffer(
        width: GLsizei,
        height: GLsizei,
        target: GLenum,
        internalFormat: GLenum,
        max_level: GLint,
        pbuffer: *mut CGLPBufferObj,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLDestroyPBuffer(pbuffer: CGLPBufferObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLDescribePBuffer(
        obj: CGLPBufferObj,
        width: *mut GLsizei,
        height: *mut GLsizei,
        target: *mut GLenum,
        internalFormat: *mut GLenum,
        mipmap: *mut GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLTexImagePBuffer(
        ctx: CGLContextObj,
        pbuffer: CGLPBufferObj,
        source: GLenum,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLRetainPBuffer(pbuffer: CGLPBufferObj) -> CGLPBufferObj;
}
unsafe extern "C" {
    pub fn CGLReleasePBuffer(pbuffer: CGLPBufferObj);
}
unsafe extern "C" {
    pub fn CGLGetPBufferRetainCount(pbuffer: CGLPBufferObj) -> GLuint;
}
unsafe extern "C" {
    pub fn CGLSetOffScreen(
        ctx: CGLContextObj,
        width: GLsizei,
        height: GLsizei,
        rowbytes: GLint,
        baseaddr: *mut ::std::os::raw::c_void,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLGetOffScreen(
        ctx: CGLContextObj,
        width: *mut GLsizei,
        height: *mut GLsizei,
        rowbytes: *mut GLint,
        baseaddr: *mut *mut ::std::os::raw::c_void,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLSetFullScreen(ctx: CGLContextObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLSetFullScreenOnDisplay(ctx: CGLContextObj, display_mask: GLuint) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLSetPBuffer(
        ctx: CGLContextObj,
        pbuffer: CGLPBufferObj,
        face: GLenum,
        level: GLint,
        screen: GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLGetPBuffer(
        ctx: CGLContextObj,
        pbuffer: *mut CGLPBufferObj,
        face: *mut GLenum,
        level: *mut GLint,
        screen: *mut GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLClearDrawable(ctx: CGLContextObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLFlushDrawable(ctx: CGLContextObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLEnable(ctx: CGLContextObj, pname: CGLContextEnable) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLDisable(ctx: CGLContextObj, pname: CGLContextEnable) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLIsEnabled(
        ctx: CGLContextObj,
        pname: CGLContextEnable,
        enable: *mut GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLSetParameter(
        ctx: CGLContextObj,
        pname: CGLContextParameter,
        params: *const GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLGetParameter(
        ctx: CGLContextObj,
        pname: CGLContextParameter,
        params: *mut GLint,
    ) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLSetVirtualScreen(ctx: CGLContextObj, screen: GLint) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLGetVirtualScreen(ctx: CGLContextObj, screen: *mut GLint) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLUpdateContext(ctx: CGLContextObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLSetGlobalOption(pname: CGLGlobalOption, params: *const GLint) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLGetGlobalOption(pname: CGLGlobalOption, params: *mut GLint) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLSetOption(pname: CGLGlobalOption, param: GLint) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLGetOption(pname: CGLGlobalOption, param: *mut GLint) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLLockContext(ctx: CGLContextObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLUnlockContext(ctx: CGLContextObj) -> CGLError;
}
unsafe extern "C" {
    pub fn CGLGetVersion(majorvers: *mut GLint, minorvers: *mut GLint);
}
unsafe extern "C" {
    pub fn CGLErrorString(error: CGLError) -> *const ::std::os::raw::c_char;
}

unsafe impl objc2::encode::RefEncode for _CGLPixelFormatObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CGLPixelFormatObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CGLPixelFormatObject", &[]);
}
unsafe impl objc2::encode::RefEncode for _CGLRendererInfoObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CGLRendererInfoObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CGLRendererInfoObject", &[]);
}
unsafe impl objc2::encode::RefEncode for _CGLPBufferObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CGLPBufferObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CGLPBufferObject", &[]);
}
unsafe impl objc2::encode::RefEncode for __GLsync {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __GLsync {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__GLsync", &[]);
}
unsafe impl objc2::encode::RefEncode for CGLShareGroupRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGLShareGroupRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGLShareGroupRec", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_device_id {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_device_id {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_device_id", &[]);
}
unsafe impl objc2::encode::RefEncode for GLUnurbs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLUnurbs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GLUnurbs", &[]);
}
unsafe impl objc2::encode::RefEncode for GLUquadric {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLUquadric {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GLUquadric", &[]);
}
unsafe impl objc2::encode::RefEncode for GLUtesselator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GLUtesselator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GLUtesselator", &[]);
}
unsafe impl objc2::encode::RefEncode for __GLIContextRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __GLIContextRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__GLIContextRec", &[]);
}
unsafe impl objc2::encode::RefEncode for __GLISharedRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __GLISharedRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__GLISharedRec", &[]);
}
unsafe impl objc2::encode::RefEncode for __GLIFunctionDispatchRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __GLIFunctionDispatchRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__GLIFunctionDispatchRec", &[]);
}
unsafe impl objc2::encode::RefEncode for _CGLPrivateObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CGLPrivateObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CGLPrivateObject", &[]);
}
unsafe impl objc2::encode::RefEncode for _CGLContextObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CGLContextObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CGLContextObject", &[]);
}
