#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreML::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOSurface::*;
#[allow(unused_imports)]
use crate::ImageIO::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::OpenGLES::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CGLContextObj = *mut _CGLContextObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CGLPixelFormatObject {
    _unused: [u8; 0],
}
pub type CGLPixelFormatObj = *mut _CGLPixelFormatObject;
pub type GLcharARB = ::std::os::raw::c_char;
pub type GLhandleARB = *mut ::std::os::raw::c_void;
pub type GLdouble = f64;
pub type GLclampd = f64;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLIContextRec {
    _unused: [u8; 0],
}
pub type GLIContext = *mut __GLIContextRec;
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
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIVector(pub id);
impl std::ops::Deref for CIVector {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIVector {}
impl CIVector {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), alloc) })
    }
}
impl PNSCopying for CIVector {}
impl PNSSecureCoding for CIVector {}
impl INSObject for CIVector {}
impl PNSObject for CIVector {}
impl std::convert::TryFrom<NSObject> for CIVector {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIVector, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIVector").unwrap()) };
        if is_kind_of {
            Ok(CIVector(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIVector")
        }
    }
}
impl ICIVector for CIVector {}
pub trait ICIVector: Sized + std::ops::Deref {
    unsafe fn initWithValues_count_(&self, values: *const CGFloat, count: usize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValues : values, count : count)
    }
    unsafe fn initWithX_(&self, x: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x)
    }
    unsafe fn initWithX_Y_(&self, x: CGFloat, y: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x, Y : y)
    }
    unsafe fn initWithX_Y_Z_(&self, x: CGFloat, y: CGFloat, z: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x, Y : y, Z : z)
    }
    unsafe fn initWithX_Y_Z_W_(
        &self,
        x: CGFloat,
        y: CGFloat,
        z: CGFloat,
        w: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x, Y : y, Z : z, W : w)
    }
    unsafe fn initWithCGPoint_(&self, p: CGPoint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGPoint : p)
    }
    unsafe fn initWithCGRect_(&self, r: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGRect : r)
    }
    unsafe fn initWithCGAffineTransform_(&self, t: CGAffineTransform) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGAffineTransform : t)
    }
    unsafe fn initWithString_(&self, representation: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithString : representation)
    }
    unsafe fn valueAtIndex_(&self, index: usize) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueAtIndex : index)
    }
    unsafe fn count(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn X(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, X)
    }
    unsafe fn Y(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, Y)
    }
    unsafe fn Z(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, Z)
    }
    unsafe fn W(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, W)
    }
    unsafe fn CGPointValue(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGPointValue)
    }
    unsafe fn CGRectValue(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGRectValue)
    }
    unsafe fn CGAffineTransformValue(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGAffineTransformValue)
    }
    unsafe fn stringRepresentation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringRepresentation)
    }
    unsafe fn vectorWithValues_count_(values: *const CGFloat, count: usize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithValues : values, count : count)
    }
    unsafe fn vectorWithX_(x: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithX : x)
    }
    unsafe fn vectorWithX_Y_(x: CGFloat, y: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithX : x, Y : y)
    }
    unsafe fn vectorWithX_Y_Z_(x: CGFloat, y: CGFloat, z: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithX : x, Y : y, Z : z)
    }
    unsafe fn vectorWithX_Y_Z_W_(x: CGFloat, y: CGFloat, z: CGFloat, w: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithX : x, Y : y, Z : z, W : w)
    }
    unsafe fn vectorWithCGPoint_(p: CGPoint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithCGPoint : p)
    }
    unsafe fn vectorWithCGRect_(r: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithCGRect : r)
    }
    unsafe fn vectorWithCGAffineTransform_(t: CGAffineTransform) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithCGAffineTransform : t)
    }
    unsafe fn vectorWithString_(representation: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithString : representation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIColor(pub id);
impl std::ops::Deref for CIColor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIColor {}
impl CIColor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIColor {}
impl PNSCopying for CIColor {}
impl INSObject for CIColor {}
impl PNSObject for CIColor {}
impl std::convert::TryFrom<NSObject> for CIColor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIColor, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIColor").unwrap()) };
        if is_kind_of {
            Ok(CIColor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIColor")
        }
    }
}
impl ICIColor for CIColor {}
pub trait ICIColor: Sized + std::ops::Deref {
    unsafe fn initWithCGColor_(&self, color: CGColorRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGColor : color)
    }
    unsafe fn initWithRed_green_blue_alpha_(
        &self,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRed : red, green : green, blue : blue, alpha : alpha)
    }
    unsafe fn initWithRed_green_blue_(
        &self,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRed : red, green : green, blue : blue)
    }
    unsafe fn initWithRed_green_blue_alpha_colorSpace_(
        &self,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRed : red, green : green, blue : blue, alpha : alpha, colorSpace : colorSpace)
    }
    unsafe fn initWithRed_green_blue_colorSpace_(
        &self,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRed : red, green : green, blue : blue, colorSpace : colorSpace)
    }
    unsafe fn numberOfComponents(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfComponents)
    }
    unsafe fn components(&self) -> *const CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, components)
    }
    unsafe fn alpha(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alpha)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn red(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, red)
    }
    unsafe fn green(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, green)
    }
    unsafe fn blue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blue)
    }
    unsafe fn stringRepresentation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringRepresentation)
    }
    unsafe fn colorWithCGColor_(color: CGColorRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithCGColor : color)
    }
    unsafe fn colorWithRed_green_blue_alpha_(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithRed : red, green : green, blue : blue, alpha : alpha)
    }
    unsafe fn colorWithRed_green_blue_(red: CGFloat, green: CGFloat, blue: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithRed : red, green : green, blue : blue)
    }
    unsafe fn colorWithRed_green_blue_alpha_colorSpace_(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithRed : red, green : green, blue : blue, alpha : alpha, colorSpace : colorSpace)
    }
    unsafe fn colorWithRed_green_blue_colorSpace_(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithRed : red, green : green, blue : blue, colorSpace : colorSpace)
    }
    unsafe fn colorWithString_(representation: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithString : representation)
    }
    unsafe fn blackColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), blackColor)
    }
    unsafe fn whiteColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), whiteColor)
    }
    unsafe fn grayColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), grayColor)
    }
    unsafe fn redColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), redColor)
    }
    unsafe fn greenColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), greenColor)
    }
    unsafe fn blueColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), blueColor)
    }
    unsafe fn cyanColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), cyanColor)
    }
    unsafe fn magentaColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), magentaColor)
    }
    unsafe fn yellowColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), yellowColor)
    }
    unsafe fn clearColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), clearColor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIImage(pub id);
impl std::ops::Deref for CIImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIImage {}
impl CIImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIImage {}
impl PNSCopying for CIImage {}
impl INSObject for CIImage {}
impl PNSObject for CIImage {}
impl std::convert::TryFrom<NSObject> for CIImage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIImage, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIImage").unwrap()) };
        if is_kind_of {
            Ok(CIImage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIImage")
        }
    }
}
impl ICIImage for CIImage {}
pub trait ICIImage: Sized + std::ops::Deref {
    unsafe fn initWithCGImage_(&self, image: CGImageRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGImage : image)
    }
    unsafe fn initWithCGImage_options_(
        &self,
        image: CGImageRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGImage : image, options : options)
    }
    unsafe fn initWithCGImageSource_index_options_(
        &self,
        source: CGImageSourceRef,
        index: usize,
        dict: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGImageSource : source, index : index, options : dict)
    }
    unsafe fn initWithCGLayer_(&self, layer: CGLayerRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGLayer : layer)
    }
    unsafe fn initWithCGLayer_options_(
        &self,
        layer: CGLayerRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGLayer : layer, options : options)
    }
    unsafe fn initWithData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn initWithData_options_(&self, data: NSData, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, options : options)
    }
    unsafe fn initWithBitmapData_bytesPerRow_size_format_colorSpace_(
        &self,
        data: NSData,
        bytesPerRow: usize,
        size: CGSize,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBitmapData : data, bytesPerRow : bytesPerRow, size : size, format : format, colorSpace : colorSpace)
    }
    unsafe fn initWithTexture_size_flipped_colorSpace_(
        &self,
        name: ::std::os::raw::c_uint,
        size: CGSize,
        flipped: BOOL,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : name, size : size, flipped : flipped, colorSpace : colorSpace)
    }
    unsafe fn initWithTexture_size_flipped_options_(
        &self,
        name: ::std::os::raw::c_uint,
        size: CGSize,
        flipped: BOOL,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : name, size : size, flipped : flipped, options : options)
    }
    unsafe fn initWithMTLTexture_options_(
        &self,
        texture: *mut u64,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMTLTexture : texture, options : options)
    }
    unsafe fn initWithContentsOfURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url)
    }
    unsafe fn initWithContentsOfURL_options_(
        &self,
        url: NSURL,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url, options : options)
    }
    unsafe fn initWithIOSurface_(&self, surface: IOSurfaceRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOSurface : surface)
    }
    unsafe fn initWithIOSurface_options_(
        &self,
        surface: IOSurfaceRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOSurface : surface, options : options)
    }
    unsafe fn initWithIOSurface_plane_format_options_(
        &self,
        surface: IOSurfaceRef,
        plane: usize,
        format: CIFormat,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOSurface : surface, plane : plane, format : format, options : options)
    }
    unsafe fn initWithCVImageBuffer_(&self, imageBuffer: CVImageBufferRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVImageBuffer : imageBuffer)
    }
    unsafe fn initWithCVImageBuffer_options_(
        &self,
        imageBuffer: CVImageBufferRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVImageBuffer : imageBuffer, options : options)
    }
    unsafe fn initWithCVPixelBuffer_(&self, pixelBuffer: CVPixelBufferRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVPixelBuffer : pixelBuffer)
    }
    unsafe fn initWithCVPixelBuffer_options_(
        &self,
        pixelBuffer: CVPixelBufferRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVPixelBuffer : pixelBuffer, options : options)
    }
    unsafe fn initWithColor_(&self, color: CIColor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithColor : color)
    }
    unsafe fn imageByApplyingTransform_(&self, matrix: CGAffineTransform) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingTransform : matrix)
    }
    unsafe fn imageByApplyingTransform_highQualityDownsample_(
        &self,
        matrix: CGAffineTransform,
        highQualityDownsample: BOOL,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingTransform : matrix, highQualityDownsample : highQualityDownsample)
    }
    unsafe fn imageByApplyingOrientation_(&self, orientation: ::std::os::raw::c_int) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingOrientation : orientation)
    }
    unsafe fn imageTransformForOrientation_(
        &self,
        orientation: ::std::os::raw::c_int,
    ) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageTransformForOrientation : orientation)
    }
    unsafe fn imageByApplyingCGOrientation_(
        &self,
        orientation: CGImagePropertyOrientation,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingCGOrientation : orientation)
    }
    unsafe fn imageTransformForCGOrientation_(
        &self,
        orientation: CGImagePropertyOrientation,
    ) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageTransformForCGOrientation : orientation)
    }
    unsafe fn imageByCompositingOverImage_(&self, dest: CIImage) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByCompositingOverImage : dest)
    }
    unsafe fn imageByCroppingToRect_(&self, rect: CGRect) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByCroppingToRect : rect)
    }
    unsafe fn imageByClampingToExtent(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByClampingToExtent)
    }
    unsafe fn imageByClampingToRect_(&self, rect: CGRect) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByClampingToRect : rect)
    }
    unsafe fn imageByApplyingFilter_withInputParameters_(
        &self,
        filterName: NSString,
        params: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingFilter : filterName, withInputParameters : params)
    }
    unsafe fn imageByApplyingFilter_(&self, filterName: NSString) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingFilter : filterName)
    }
    unsafe fn imageByColorMatchingColorSpaceToWorkingSpace_(
        &self,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByColorMatchingColorSpaceToWorkingSpace : colorSpace)
    }
    unsafe fn imageByColorMatchingWorkingSpaceToColorSpace_(
        &self,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByColorMatchingWorkingSpaceToColorSpace : colorSpace)
    }
    unsafe fn imageByPremultiplyingAlpha(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByPremultiplyingAlpha)
    }
    unsafe fn imageByUnpremultiplyingAlpha(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByUnpremultiplyingAlpha)
    }
    unsafe fn imageBySettingAlphaOneInExtent_(&self, extent: CGRect) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBySettingAlphaOneInExtent : extent)
    }
    unsafe fn imageByApplyingGaussianBlurWithSigma_(&self, sigma: f64) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingGaussianBlurWithSigma : sigma)
    }
    unsafe fn imageBySettingProperties_(&self, properties: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBySettingProperties : properties)
    }
    unsafe fn imageBySamplingLinear(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageBySamplingLinear)
    }
    unsafe fn imageBySamplingNearest(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageBySamplingNearest)
    }
    unsafe fn imageByInsertingIntermediate(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByInsertingIntermediate)
    }
    unsafe fn imageByInsertingIntermediate_(&self, cache: BOOL) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByInsertingIntermediate : cache)
    }
    unsafe fn imageByInsertingTiledIntermediate(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByInsertingTiledIntermediate)
    }
    unsafe fn imageByApplyingGainMap_(&self, gainmap: CIImage) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingGainMap : gainmap)
    }
    unsafe fn imageByApplyingGainMap_headroom_(&self, gainmap: CIImage, headroom: f32) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingGainMap : gainmap, headroom : headroom)
    }
    unsafe fn imageBySettingContentHeadroom_(&self, headroom: f32) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBySettingContentHeadroom : headroom)
    }
    unsafe fn imageBySettingContentAverageLightLevel_(&self, average: f32) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBySettingContentAverageLightLevel : average)
    }
    unsafe fn regionOfInterestForImage_inRect_(&self, image: CIImage, rect: CGRect) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, regionOfInterestForImage : image, inRect : rect)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn isOpaque(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpaque)
    }
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn definition(&self) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, definition)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn contentHeadroom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentHeadroom)
    }
    unsafe fn contentAverageLightLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentAverageLightLevel)
    }
    unsafe fn pixelBuffer(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBuffer)
    }
    unsafe fn CGImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGImage)
    }
    unsafe fn metalTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalTexture)
    }
    unsafe fn imageWithCGImage_(image: CGImageRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGImage : image)
    }
    unsafe fn imageWithCGImage_options_(image: CGImageRef, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGImage : image, options : options)
    }
    unsafe fn imageWithCGImageSource_index_options_(
        source: CGImageSourceRef,
        index: usize,
        dict: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGImageSource : source, index : index, options : dict)
    }
    unsafe fn imageWithCGLayer_(layer: CGLayerRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGLayer : layer)
    }
    unsafe fn imageWithCGLayer_options_(layer: CGLayerRef, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGLayer : layer, options : options)
    }
    unsafe fn imageWithBitmapData_bytesPerRow_size_format_colorSpace_(
        data: NSData,
        bytesPerRow: usize,
        size: CGSize,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithBitmapData : data, bytesPerRow : bytesPerRow, size : size, format : format, colorSpace : colorSpace)
    }
    unsafe fn imageWithTexture_size_flipped_colorSpace_(
        name: ::std::os::raw::c_uint,
        size: CGSize,
        flipped: BOOL,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithTexture : name, size : size, flipped : flipped, colorSpace : colorSpace)
    }
    unsafe fn imageWithTexture_size_flipped_options_(
        name: ::std::os::raw::c_uint,
        size: CGSize,
        flipped: BOOL,
        options: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithTexture : name, size : size, flipped : flipped, options : options)
    }
    unsafe fn imageWithMTLTexture_options_(texture: *mut u64, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithMTLTexture : texture, options : options)
    }
    unsafe fn imageWithContentsOfURL_(url: NSURL) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithContentsOfURL : url)
    }
    unsafe fn imageWithContentsOfURL_options_(url: NSURL, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithContentsOfURL : url, options : options)
    }
    unsafe fn imageWithData_(data: NSData) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithData : data)
    }
    unsafe fn imageWithData_options_(data: NSData, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithData : data, options : options)
    }
    unsafe fn imageWithCVImageBuffer_(imageBuffer: CVImageBufferRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCVImageBuffer : imageBuffer)
    }
    unsafe fn imageWithCVImageBuffer_options_(
        imageBuffer: CVImageBufferRef,
        options: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCVImageBuffer : imageBuffer, options : options)
    }
    unsafe fn imageWithCVPixelBuffer_(pixelBuffer: CVPixelBufferRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCVPixelBuffer : pixelBuffer)
    }
    unsafe fn imageWithCVPixelBuffer_options_(
        pixelBuffer: CVPixelBufferRef,
        options: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCVPixelBuffer : pixelBuffer, options : options)
    }
    unsafe fn imageWithIOSurface_(surface: IOSurfaceRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithIOSurface : surface)
    }
    unsafe fn imageWithIOSurface_options_(surface: IOSurfaceRef, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithIOSurface : surface, options : options)
    }
    unsafe fn imageWithColor_(color: CIColor) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithColor : color)
    }
    unsafe fn emptyImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), emptyImage)
    }
    unsafe fn blackImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), blackImage)
    }
    unsafe fn whiteImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), whiteImage)
    }
    unsafe fn grayImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), grayImage)
    }
    unsafe fn redImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), redImage)
    }
    unsafe fn greenImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), greenImage)
    }
    unsafe fn blueImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), blueImage)
    }
    unsafe fn cyanImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), cyanImage)
    }
    unsafe fn magentaImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), magentaImage)
    }
    unsafe fn yellowImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), yellowImage)
    }
    unsafe fn clearImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), clearImage)
    }
}
pub type CIFormat = ::std::os::raw::c_int;
pub type CIImageOption = NSString;
impl CIImage_AutoAdjustment for CIImage {}
pub trait CIImage_AutoAdjustment: Sized + std::ops::Deref {
    unsafe fn autoAdjustmentFilters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoAdjustmentFilters)
    }
    unsafe fn autoAdjustmentFiltersWithOptions_(&self, options: NSDictionary) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, autoAdjustmentFiltersWithOptions : options)
    }
}
pub type CIImageAutoAdjustmentOption = NSString;
impl CIImage_LabConversion for CIImage {}
pub trait CIImage_LabConversion: Sized + std::ops::Deref {
    unsafe fn imageByConvertingWorkingSpaceToLab(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByConvertingWorkingSpaceToLab)
    }
    unsafe fn imageByConvertingLabToWorkingSpace(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByConvertingLabToWorkingSpace)
    }
}
impl CIImage_AVDepthData for CIImage {}
pub trait CIImage_AVDepthData: Sized + std::ops::Deref {
    unsafe fn initWithDepthData_options_(
        &self,
        data: AVDepthData,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDepthData : data, options : options)
    }
    unsafe fn initWithDepthData_(&self, data: AVDepthData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDepthData : data)
    }
    unsafe fn depthData(&self) -> AVDepthData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthData)
    }
    unsafe fn imageWithDepthData_options_(data: AVDepthData, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithDepthData : data, options : options)
    }
    unsafe fn imageWithDepthData_(data: AVDepthData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithDepthData : data)
    }
}
impl CIImage_AVPortraitEffectsMatte for CIImage {}
pub trait CIImage_AVPortraitEffectsMatte: Sized + std::ops::Deref {
    unsafe fn initWithPortaitEffectsMatte_options_(
        &self,
        matte: AVPortraitEffectsMatte,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPortaitEffectsMatte : matte, options : options)
    }
    unsafe fn initWithPortaitEffectsMatte_(&self, matte: AVPortraitEffectsMatte) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPortaitEffectsMatte : matte)
    }
    unsafe fn portraitEffectsMatte(&self) -> AVPortraitEffectsMatte
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, portraitEffectsMatte)
    }
    unsafe fn imageWithPortaitEffectsMatte_options_(
        matte: AVPortraitEffectsMatte,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithPortaitEffectsMatte : matte, options : options)
    }
    unsafe fn imageWithPortaitEffectsMatte_(matte: AVPortraitEffectsMatte) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithPortaitEffectsMatte : matte)
    }
}
impl CIImage_AVSemanticSegmentationMatte for CIImage {}
pub trait CIImage_AVSemanticSegmentationMatte: Sized + std::ops::Deref {
    unsafe fn initWithSemanticSegmentationMatte_options_(
        &self,
        matte: AVSemanticSegmentationMatte,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSemanticSegmentationMatte : matte, options : options)
    }
    unsafe fn initWithSemanticSegmentationMatte_(
        &self,
        matte: AVSemanticSegmentationMatte,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSemanticSegmentationMatte : matte)
    }
    unsafe fn semanticSegmentationMatte(&self) -> AVSemanticSegmentationMatte
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationMatte)
    }
    unsafe fn imageWithSemanticSegmentationMatte_options_(
        matte: AVSemanticSegmentationMatte,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithSemanticSegmentationMatte : matte, options : options)
    }
    unsafe fn imageWithSemanticSegmentationMatte_(
        matte: AVSemanticSegmentationMatte,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithSemanticSegmentationMatte : matte)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIContext(pub id);
impl std::ops::Deref for CIContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIContext {}
impl CIContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), alloc) })
    }
}
impl INSObject for CIContext {}
impl PNSObject for CIContext {}
impl std::convert::TryFrom<NSObject> for CIContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIContext, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIContext").unwrap()) };
        if is_kind_of {
            Ok(CIContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIContext")
        }
    }
}
impl ICIContext for CIContext {}
pub trait ICIContext: Sized + std::ops::Deref {
    unsafe fn initWithOptions_(&self, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOptions : options)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn drawImage_atPoint_fromRect_(&self, image: CIImage, atPoint: CGPoint, fromRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawImage : image, atPoint : atPoint, fromRect : fromRect)
    }
    unsafe fn drawImage_inRect_fromRect_(&self, image: CIImage, inRect: CGRect, fromRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawImage : image, inRect : inRect, fromRect : fromRect)
    }
    unsafe fn createCGLayerWithSize_info_(&self, size: CGSize, info: CFDictionaryRef) -> CGLayerRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGLayerWithSize : size, info : info)
    }
    unsafe fn render_toBitmap_rowBytes_bounds_format_colorSpace_(
        &self,
        image: CIImage,
        data: *mut ::std::os::raw::c_void,
        rowBytes: isize,
        bounds: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toBitmap : data, rowBytes : rowBytes, bounds : bounds, format : format, colorSpace : colorSpace)
    }
    unsafe fn render_toIOSurface_bounds_colorSpace_(
        &self,
        image: CIImage,
        surface: IOSurfaceRef,
        bounds: CGRect,
        colorSpace: CGColorSpaceRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toIOSurface : surface, bounds : bounds, colorSpace : colorSpace)
    }
    unsafe fn render_toCVPixelBuffer_(&self, image: CIImage, buffer: CVPixelBufferRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toCVPixelBuffer : buffer)
    }
    unsafe fn render_toCVPixelBuffer_bounds_colorSpace_(
        &self,
        image: CIImage,
        buffer: CVPixelBufferRef,
        bounds: CGRect,
        colorSpace: CGColorSpaceRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toCVPixelBuffer : buffer, bounds : bounds, colorSpace : colorSpace)
    }
    unsafe fn render_toMTLTexture_commandBuffer_bounds_colorSpace_(
        &self,
        image: CIImage,
        texture: *mut u64,
        commandBuffer: *mut u64,
        bounds: CGRect,
        colorSpace: CGColorSpaceRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toMTLTexture : texture, commandBuffer : commandBuffer, bounds : bounds, colorSpace : colorSpace)
    }
    unsafe fn reclaimResources(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reclaimResources)
    }
    unsafe fn clearCaches(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearCaches)
    }
    unsafe fn inputImageMaximumSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImageMaximumSize)
    }
    unsafe fn outputImageMaximumSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputImageMaximumSize)
    }
    unsafe fn workingColorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workingColorSpace)
    }
    unsafe fn workingFormat(&self) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workingFormat)
    }
    unsafe fn contextWithCGLContext_pixelFormat_colorSpace_options_(
        cglctx: CGLContextObj,
        pixelFormat: CGLPixelFormatObj,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithCGLContext : cglctx, pixelFormat : pixelFormat, colorSpace : colorSpace, options : options)
    }
    unsafe fn contextWithCGLContext_pixelFormat_options_(
        cglctx: CGLContextObj,
        pixelFormat: CGLPixelFormatObj,
        options: NSDictionary,
    ) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithCGLContext : cglctx, pixelFormat : pixelFormat, options : options)
    }
    unsafe fn contextWithCGContext_options_(cgctx: CGContextRef, options: NSDictionary) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithCGContext : cgctx, options : options)
    }
    unsafe fn contextWithOptions_(options: NSDictionary) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithOptions : options)
    }
    unsafe fn context() -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), context)
    }
    unsafe fn contextWithMTLDevice_(device: *mut u64) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithMTLDevice : device)
    }
    unsafe fn contextWithMTLDevice_options_(device: *mut u64, options: NSDictionary) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithMTLDevice : device, options : options)
    }
    unsafe fn contextWithMTLCommandQueue_(commandQueue: *mut u64) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithMTLCommandQueue : commandQueue)
    }
    unsafe fn contextWithMTLCommandQueue_options_(
        commandQueue: *mut u64,
        options: NSDictionary,
    ) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithMTLCommandQueue : commandQueue, options : options)
    }
}
pub type CIContextOption = NSString;
impl CIContext_createCGImage for CIContext {}
pub trait CIContext_createCGImage: Sized + std::ops::Deref {
    unsafe fn createCGImage_fromRect_(&self, image: CIImage, fromRect: CGRect) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGImage : image, fromRect : fromRect)
    }
    unsafe fn createCGImage_fromRect_format_colorSpace_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGImage : image, fromRect : fromRect, format : format, colorSpace : colorSpace)
    }
    unsafe fn createCGImage_fromRect_format_colorSpace_deferred_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        deferred: BOOL,
    ) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGImage : image, fromRect : fromRect, format : format, colorSpace : colorSpace, deferred : deferred)
    }
    unsafe fn createCGImage_fromRect_format_colorSpace_deferred_calculateHDRStats_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        deferred: BOOL,
        calculateHDRStats: BOOL,
    ) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGImage : image, fromRect : fromRect, format : format, colorSpace : colorSpace, deferred : deferred, calculateHDRStats : calculateHDRStats)
    }
}
impl CIContext_CalculateHDRStats for CIContext {}
pub trait CIContext_CalculateHDRStats: Sized + std::ops::Deref {
    unsafe fn calculateHDRStatsForIOSurface_(&self, surface: IOSurfaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateHDRStatsForIOSurface : surface)
    }
    unsafe fn calculateHDRStatsForCVPixelBuffer_(&self, buffer: CVPixelBufferRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateHDRStatsForCVPixelBuffer : buffer)
    }
    unsafe fn calculateHDRStatsForCGImage_(&self, cgimage: CGImageRef) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateHDRStatsForCGImage : cgimage)
    }
    unsafe fn calculateHDRStatsForImage_(&self, image: CIImage) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateHDRStatsForImage : image)
    }
}
impl CIContext_OfflineGPUSupport for CIContext {}
pub trait CIContext_OfflineGPUSupport: Sized + std::ops::Deref {
    unsafe fn offlineGPUCount() -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), offlineGPUCount)
    }
    unsafe fn contextForOfflineGPUAtIndex_(index: ::std::os::raw::c_uint) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextForOfflineGPUAtIndex : index)
    }
    unsafe fn contextForOfflineGPUAtIndex_colorSpace_options_sharedContext_(
        index: ::std::os::raw::c_uint,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        sharedContext: CGLContextObj,
    ) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextForOfflineGPUAtIndex : index, colorSpace : colorSpace, options : options, sharedContext : sharedContext)
    }
}
pub type CIImageRepresentationOption = NSString;
impl CIContext_ImageRepresentation for CIContext {}
pub trait CIContext_ImageRepresentation: Sized + std::ops::Deref {
    unsafe fn TIFFRepresentationOfImage_format_colorSpace_options_(
        &self,
        image: CIImage,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, TIFFRepresentationOfImage : image, format : format, colorSpace : colorSpace, options : options)
    }
    unsafe fn JPEGRepresentationOfImage_colorSpace_options_(
        &self,
        image: CIImage,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, JPEGRepresentationOfImage : image, colorSpace : colorSpace, options : options)
    }
    unsafe fn HEIFRepresentationOfImage_format_colorSpace_options_(
        &self,
        image: CIImage,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, HEIFRepresentationOfImage : image, format : format, colorSpace : colorSpace, options : options)
    }
    unsafe fn HEIF10RepresentationOfImage_colorSpace_options_error_(
        &self,
        image: CIImage,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, HEIF10RepresentationOfImage : image, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn PNGRepresentationOfImage_format_colorSpace_options_(
        &self,
        image: CIImage,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PNGRepresentationOfImage : image, format : format, colorSpace : colorSpace, options : options)
    }
    unsafe fn OpenEXRRepresentationOfImage_options_error_(
        &self,
        image: CIImage,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OpenEXRRepresentationOfImage : image, options : options, error : errorPtr)
    }
    unsafe fn writeTIFFRepresentationOfImage_toURL_format_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeTIFFRepresentationOfImage : image, toURL : url, format : format, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writePNGRepresentationOfImage_toURL_format_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writePNGRepresentationOfImage : image, toURL : url, format : format, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writeJPEGRepresentationOfImage_toURL_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeJPEGRepresentationOfImage : image, toURL : url, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writeHEIFRepresentationOfImage_toURL_format_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeHEIFRepresentationOfImage : image, toURL : url, format : format, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writeHEIF10RepresentationOfImage_toURL_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeHEIF10RepresentationOfImage : image, toURL : url, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writeOpenEXRRepresentationOfImage_toURL_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeOpenEXRRepresentationOfImage : image, toURL : url, options : options, error : errorPtr)
    }
}
impl CIContext_CIDepthBlurEffect for CIContext {}
pub trait CIContext_CIDepthBlurEffect: Sized + std::ops::Deref {
    unsafe fn depthBlurEffectFilterForImageURL_options_(
        &self,
        url: NSURL,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImageURL : url, options : options)
    }
    unsafe fn depthBlurEffectFilterForImageData_options_(
        &self,
        data: NSData,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImageData : data, options : options)
    }
    unsafe fn depthBlurEffectFilterForImage_disparityImage_portraitEffectsMatte_orientation_options_(
        &self,
        image: CIImage,
        disparityImage: CIImage,
        portraitEffectsMatte: CIImage,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImage : image, disparityImage : disparityImage, portraitEffectsMatte : portraitEffectsMatte, orientation : orientation, options : options)
    }
    unsafe fn depthBlurEffectFilterForImage_disparityImage_portraitEffectsMatte_hairSemanticSegmentation_orientation_options_(
        &self,
        image: CIImage,
        disparityImage: CIImage,
        portraitEffectsMatte: CIImage,
        hairSemanticSegmentation: CIImage,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImage : image, disparityImage : disparityImage, portraitEffectsMatte : portraitEffectsMatte, hairSemanticSegmentation : hairSemanticSegmentation, orientation : orientation, options : options)
    }
    unsafe fn depthBlurEffectFilterForImage_disparityImage_portraitEffectsMatte_hairSemanticSegmentation_glassesMatte_gainMap_orientation_options_(
        &self,
        image: CIImage,
        disparityImage: CIImage,
        portraitEffectsMatte: CIImage,
        hairSemanticSegmentation: CIImage,
        glassesMatte: CIImage,
        gainMap: CIImage,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImage : image, disparityImage : disparityImage, portraitEffectsMatte : portraitEffectsMatte, hairSemanticSegmentation : hairSemanticSegmentation, glassesMatte : glassesMatte, gainMap : gainMap, orientation : orientation, options : options)
    }
}
pub trait PCIFilterConstructor: Sized + std::ops::Deref {
    unsafe fn filterWithName_(&self, name: NSString) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, filterWithName : name)
    }
}
pub type CIDynamicRangeOption = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFilter(pub id);
impl std::ops::Deref for CIFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFilter {}
impl CIFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIFilter {}
impl PNSCopying for CIFilter {}
impl INSObject for CIFilter {}
impl PNSObject for CIFilter {}
impl std::convert::TryFrom<NSObject> for CIFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIFilter, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFilter").unwrap()) };
        if is_kind_of {
            Ok(CIFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIFilter")
        }
    }
}
impl ICIFilter for CIFilter {}
pub trait ICIFilter: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, aString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : aString)
    }
    unsafe fn setDefaults(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setDefaults)
    }
    unsafe fn apply_arguments_options_(
        &self,
        k: CIKernel,
        args: NSArray,
        dict: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, apply : k, arguments : args, options : dict)
    }
    unsafe fn apply_(&self, k: CIKernel) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, apply : k)
    }
    unsafe fn outputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputImage)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn inputKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputKeys)
    }
    unsafe fn outputKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputKeys)
    }
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
}
pub trait PCIFilter: Sized + std::ops::Deref {
    unsafe fn outputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputImage)
    }
    unsafe fn customAttributes() -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), customAttributes)
    }
}
impl CIFilter_CIFilterRegistry for CIFilter {}
pub trait CIFilter_CIFilterRegistry: Sized + std::ops::Deref {
    unsafe fn filterWithName_(name: NSString) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithName : name)
    }
    unsafe fn filterWithName_keysAndValues_(name: NSString, key0: id) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithName : name, keysAndValues : key0)
    }
    unsafe fn filterWithName_withInputParameters_(name: NSString, params: NSDictionary) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithName : name, withInputParameters : params)
    }
    unsafe fn filterNamesInCategory_(category: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterNamesInCategory : category)
    }
    unsafe fn filterNamesInCategories_(categories: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterNamesInCategories : categories)
    }
    unsafe fn registerFilterName_constructor_classAttributes_(
        name: NSString,
        anObject: *mut u64,
        attributes: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), registerFilterName : name, constructor : anObject, classAttributes : attributes)
    }
    unsafe fn localizedNameForFilterName_(filterName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), localizedNameForFilterName : filterName)
    }
    unsafe fn localizedNameForCategory_(category: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), localizedNameForCategory : category)
    }
    unsafe fn localizedDescriptionForFilterName_(filterName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), localizedDescriptionForFilterName : filterName)
    }
    unsafe fn localizedReferenceDocumentationForFilterName_(filterName: NSString) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), localizedReferenceDocumentationForFilterName : filterName)
    }
}
impl CIFilter_CIFilterXMPSerialization for CIFilter {}
pub trait CIFilter_CIFilterXMPSerialization: Sized + std::ops::Deref {
    unsafe fn serializedXMPFromFilters_inputImageExtent_(filters: NSArray, extent: CGRect) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), serializedXMPFromFilters : filters, inputImageExtent : extent)
    }
    unsafe fn filterArrayFromSerializedXMP_inputImageExtent_error_(
        xmpData: NSData,
        extent: CGRect,
        outError: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterArrayFromSerializedXMP : xmpData, inputImageExtent : extent, error : outError)
    }
}
pub type CIKernelROICallback = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIKernel(pub id);
impl std::ops::Deref for CIKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIKernel {}
impl CIKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), alloc) })
    }
}
impl INSObject for CIKernel {}
impl PNSObject for CIKernel {}
impl std::convert::TryFrom<NSObject> for CIKernel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIKernel, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIKernel").unwrap()) };
        if is_kind_of {
            Ok(CIKernel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIKernel")
        }
    }
}
impl ICIKernel for CIKernel {}
pub trait ICIKernel: Sized + std::ops::Deref {
    unsafe fn setROISelector_(&self, method: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setROISelector : method)
    }
    unsafe fn applyWithExtent_roiCallback_arguments_(
        &self,
        extent: CGRect,
        callback: CIKernelROICallback,
        args: NSArray,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithExtent : extent, roiCallback : callback, arguments : args)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn kernelsWithString_(string: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelsWithString : string)
    }
    unsafe fn kernelsWithMetalString_error_(source: NSString, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelsWithMetalString : source, error : error)
    }
    unsafe fn kernelWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelWithString : string)
    }
    unsafe fn kernelWithFunctionName_fromMetalLibraryData_error_(
        name: NSString,
        data: NSData,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelWithFunctionName : name, fromMetalLibraryData : data, error : error)
    }
    unsafe fn kernelWithFunctionName_fromMetalLibraryData_outputPixelFormat_error_(
        name: NSString,
        data: NSData,
        format: CIFormat,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelWithFunctionName : name, fromMetalLibraryData : data, outputPixelFormat : format, error : error)
    }
    unsafe fn kernelNamesFromMetalLibraryData_(data: NSData) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelNamesFromMetalLibraryData : data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIColorKernel(pub id);
impl std::ops::Deref for CIColorKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIColorKernel {}
impl CIColorKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIColorKernel").unwrap(), alloc) })
    }
}
impl ICIKernel for CIColorKernel {}
impl From<CIColorKernel> for CIKernel {
    fn from(child: CIColorKernel) -> CIKernel {
        CIKernel(child.0)
    }
}
impl std::convert::TryFrom<CIKernel> for CIColorKernel {
    type Error = &'static str;
    fn try_from(parent: CIKernel) -> Result<CIColorKernel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIColorKernel").unwrap()) };
        if is_kind_of {
            Ok(CIColorKernel(parent.0))
        } else {
            Err("This CIKernel cannot be downcasted to CIColorKernel")
        }
    }
}
impl INSObject for CIColorKernel {}
impl PNSObject for CIColorKernel {}
impl ICIColorKernel for CIColorKernel {}
pub trait ICIColorKernel: Sized + std::ops::Deref {
    unsafe fn applyWithExtent_arguments_(&self, extent: CGRect, args: NSArray) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithExtent : extent, arguments : args)
    }
    unsafe fn kernelWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColorKernel").unwrap(), kernelWithString : string)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIWarpKernel(pub id);
impl std::ops::Deref for CIWarpKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIWarpKernel {}
impl CIWarpKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIWarpKernel").unwrap(), alloc) })
    }
}
impl ICIKernel for CIWarpKernel {}
impl std::convert::TryFrom<CIKernel> for CIWarpKernel {
    type Error = &'static str;
    fn try_from(parent: CIKernel) -> Result<CIWarpKernel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIWarpKernel").unwrap()) };
        if is_kind_of {
            Ok(CIWarpKernel(parent.0))
        } else {
            Err("This CIKernel cannot be downcasted to CIWarpKernel")
        }
    }
}
impl INSObject for CIWarpKernel {}
impl PNSObject for CIWarpKernel {}
impl ICIWarpKernel for CIWarpKernel {}
pub trait ICIWarpKernel: Sized + std::ops::Deref {
    unsafe fn applyWithExtent_roiCallback_inputImage_arguments_(
        &self,
        extent: CGRect,
        callback: CIKernelROICallback,
        image: CIImage,
        args: NSArray,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithExtent : extent, roiCallback : callback, inputImage : image, arguments : args)
    }
    unsafe fn kernelWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIWarpKernel").unwrap(), kernelWithString : string)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIBlendKernel(pub id);
impl std::ops::Deref for CIBlendKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIBlendKernel {}
impl CIBlendKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), alloc) })
    }
}
impl ICIColorKernel for CIBlendKernel {}
impl From<CIBlendKernel> for CIColorKernel {
    fn from(child: CIBlendKernel) -> CIColorKernel {
        CIColorKernel(child.0)
    }
}
impl std::convert::TryFrom<CIColorKernel> for CIBlendKernel {
    type Error = &'static str;
    fn try_from(parent: CIColorKernel) -> Result<CIBlendKernel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap()) };
        if is_kind_of {
            Ok(CIBlendKernel(parent.0))
        } else {
            Err("This CIColorKernel cannot be downcasted to CIBlendKernel")
        }
    }
}
impl ICIKernel for CIBlendKernel {}
impl INSObject for CIBlendKernel {}
impl PNSObject for CIBlendKernel {}
impl ICIBlendKernel for CIBlendKernel {}
pub trait ICIBlendKernel: Sized + std::ops::Deref {
    unsafe fn applyWithForeground_background_(
        &self,
        foreground: CIImage,
        background: CIImage,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithForeground : foreground, background : background)
    }
    unsafe fn applyWithForeground_background_colorSpace_(
        &self,
        foreground: CIImage,
        background: CIImage,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithForeground : foreground, background : background, colorSpace : colorSpace)
    }
    unsafe fn kernelWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), kernelWithString : string)
    }
}
impl CIBlendKernel_BuiltIn for CIBlendKernel {}
pub trait CIBlendKernel_BuiltIn: Sized + std::ops::Deref {
    unsafe fn componentAdd() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), componentAdd)
    }
    unsafe fn componentMultiply() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), componentMultiply)
    }
    unsafe fn componentMin() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), componentMin)
    }
    unsafe fn componentMax() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), componentMax)
    }
    unsafe fn clear() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), clear)
    }
    unsafe fn source() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), source)
    }
    unsafe fn destination() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destination)
    }
    unsafe fn sourceOver() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), sourceOver)
    }
    unsafe fn destinationOver() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destinationOver)
    }
    unsafe fn sourceIn() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), sourceIn)
    }
    unsafe fn destinationIn() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destinationIn)
    }
    unsafe fn sourceOut() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), sourceOut)
    }
    unsafe fn destinationOut() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destinationOut)
    }
    unsafe fn sourceAtop() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), sourceAtop)
    }
    unsafe fn destinationAtop() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destinationAtop)
    }
    unsafe fn exclusiveOr() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), exclusiveOr)
    }
    unsafe fn multiply() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), multiply)
    }
    unsafe fn screen() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), screen)
    }
    unsafe fn overlay() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), overlay)
    }
    unsafe fn darken() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), darken)
    }
    unsafe fn lighten() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), lighten)
    }
    unsafe fn colorDodge() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), colorDodge)
    }
    unsafe fn colorBurn() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), colorBurn)
    }
    unsafe fn hardLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), hardLight)
    }
    unsafe fn softLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), softLight)
    }
    unsafe fn difference() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), difference)
    }
    unsafe fn exclusion() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), exclusion)
    }
    unsafe fn hue() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), hue)
    }
    unsafe fn saturation() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), saturation)
    }
    unsafe fn color() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), color)
    }
    unsafe fn luminosity() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), luminosity)
    }
    unsafe fn subtract() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), subtract)
    }
    unsafe fn divide() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), divide)
    }
    unsafe fn linearBurn() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), linearBurn)
    }
    unsafe fn linearDodge() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), linearDodge)
    }
    unsafe fn vividLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), vividLight)
    }
    unsafe fn linearLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), linearLight)
    }
    unsafe fn pinLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), pinLight)
    }
    unsafe fn hardMix() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), hardMix)
    }
    unsafe fn darkerColor() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), darkerColor)
    }
    unsafe fn lighterColor() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), lighterColor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIDetector(pub id);
impl std::ops::Deref for CIDetector {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIDetector {}
impl CIDetector {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIDetector").unwrap(), alloc) })
    }
}
impl INSObject for CIDetector {}
impl PNSObject for CIDetector {}
impl std::convert::TryFrom<NSObject> for CIDetector {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIDetector, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIDetector").unwrap()) };
        if is_kind_of {
            Ok(CIDetector(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIDetector")
        }
    }
}
impl ICIDetector for CIDetector {}
pub trait ICIDetector: Sized + std::ops::Deref {
    unsafe fn featuresInImage_(&self, image: CIImage) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, featuresInImage : image)
    }
    unsafe fn featuresInImage_options_(&self, image: CIImage, options: NSDictionary) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, featuresInImage : image, options : options)
    }
    unsafe fn detectorOfType_context_options_(
        type_: NSString,
        context: CIContext,
        options: NSDictionary,
    ) -> CIDetector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIDetector").unwrap(), detectorOfType : type_, context : context, options : options)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFeature(pub id);
impl std::ops::Deref for CIFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFeature {}
impl CIFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFeature").unwrap(), alloc) })
    }
}
impl INSObject for CIFeature {}
impl PNSObject for CIFeature {}
impl std::convert::TryFrom<NSObject> for CIFeature {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIFeature, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFeature").unwrap()) };
        if is_kind_of {
            Ok(CIFeature(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIFeature")
        }
    }
}
impl ICIFeature for CIFeature {}
pub trait ICIFeature: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFaceFeature(pub id);
impl std::ops::Deref for CIFaceFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFaceFeature {}
impl CIFaceFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFaceFeature").unwrap(), alloc) })
    }
}
impl ICIFeature for CIFaceFeature {}
impl From<CIFaceFeature> for CIFeature {
    fn from(child: CIFaceFeature) -> CIFeature {
        CIFeature(child.0)
    }
}
impl std::convert::TryFrom<CIFeature> for CIFaceFeature {
    type Error = &'static str;
    fn try_from(parent: CIFeature) -> Result<CIFaceFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFaceFeature").unwrap()) };
        if is_kind_of {
            Ok(CIFaceFeature(parent.0))
        } else {
            Err("This CIFeature cannot be downcasted to CIFaceFeature")
        }
    }
}
impl INSObject for CIFaceFeature {}
impl PNSObject for CIFaceFeature {}
impl ICIFaceFeature for CIFaceFeature {}
pub trait ICIFaceFeature: Sized + std::ops::Deref {
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn hasLeftEyePosition(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasLeftEyePosition)
    }
    unsafe fn leftEyePosition(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftEyePosition)
    }
    unsafe fn hasRightEyePosition(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasRightEyePosition)
    }
    unsafe fn rightEyePosition(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightEyePosition)
    }
    unsafe fn hasMouthPosition(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasMouthPosition)
    }
    unsafe fn mouthPosition(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mouthPosition)
    }
    unsafe fn hasTrackingID(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasTrackingID)
    }
    unsafe fn trackingID(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackingID)
    }
    unsafe fn hasTrackingFrameCount(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasTrackingFrameCount)
    }
    unsafe fn trackingFrameCount(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackingFrameCount)
    }
    unsafe fn hasFaceAngle(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasFaceAngle)
    }
    unsafe fn faceAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, faceAngle)
    }
    unsafe fn hasSmile(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasSmile)
    }
    unsafe fn leftEyeClosed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftEyeClosed)
    }
    unsafe fn rightEyeClosed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightEyeClosed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRectangleFeature(pub id);
impl std::ops::Deref for CIRectangleFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRectangleFeature {}
impl CIRectangleFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRectangleFeature").unwrap(), alloc) })
    }
}
impl ICIFeature for CIRectangleFeature {}
impl std::convert::TryFrom<CIFeature> for CIRectangleFeature {
    type Error = &'static str;
    fn try_from(parent: CIFeature) -> Result<CIRectangleFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRectangleFeature").unwrap()) };
        if is_kind_of {
            Ok(CIRectangleFeature(parent.0))
        } else {
            Err("This CIFeature cannot be downcasted to CIRectangleFeature")
        }
    }
}
impl INSObject for CIRectangleFeature {}
impl PNSObject for CIRectangleFeature {}
impl ICIRectangleFeature for CIRectangleFeature {}
pub trait ICIRectangleFeature: Sized + std::ops::Deref {
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn topLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLeft)
    }
    unsafe fn topRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topRight)
    }
    unsafe fn bottomLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomLeft)
    }
    unsafe fn bottomRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomRight)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIQRCodeFeature(pub id);
impl std::ops::Deref for CIQRCodeFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIQRCodeFeature {}
impl CIQRCodeFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIQRCodeFeature").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIQRCodeFeature {}
impl PNSCopying for CIQRCodeFeature {}
impl ICIFeature for CIQRCodeFeature {}
impl std::convert::TryFrom<CIFeature> for CIQRCodeFeature {
    type Error = &'static str;
    fn try_from(parent: CIFeature) -> Result<CIQRCodeFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIQRCodeFeature").unwrap()) };
        if is_kind_of {
            Ok(CIQRCodeFeature(parent.0))
        } else {
            Err("This CIFeature cannot be downcasted to CIQRCodeFeature")
        }
    }
}
impl INSObject for CIQRCodeFeature {}
impl PNSObject for CIQRCodeFeature {}
impl ICIQRCodeFeature for CIQRCodeFeature {}
pub trait ICIQRCodeFeature: Sized + std::ops::Deref {
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn topLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLeft)
    }
    unsafe fn topRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topRight)
    }
    unsafe fn bottomLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomLeft)
    }
    unsafe fn bottomRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomRight)
    }
    unsafe fn messageString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageString)
    }
    unsafe fn symbolDescriptor(&self) -> CIQRCodeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symbolDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CITextFeature(pub id);
impl std::ops::Deref for CITextFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CITextFeature {}
impl CITextFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CITextFeature").unwrap(), alloc) })
    }
}
impl ICIFeature for CITextFeature {}
impl std::convert::TryFrom<CIFeature> for CITextFeature {
    type Error = &'static str;
    fn try_from(parent: CIFeature) -> Result<CITextFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CITextFeature").unwrap()) };
        if is_kind_of {
            Ok(CITextFeature(parent.0))
        } else {
            Err("This CIFeature cannot be downcasted to CITextFeature")
        }
    }
}
impl INSObject for CITextFeature {}
impl PNSObject for CITextFeature {}
impl ICITextFeature for CITextFeature {}
pub trait ICITextFeature: Sized + std::ops::Deref {
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn topLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLeft)
    }
    unsafe fn topRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topRight)
    }
    unsafe fn bottomLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomLeft)
    }
    unsafe fn bottomRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomRight)
    }
    unsafe fn subFeatures(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subFeatures)
    }
}
impl CIImage_CIImageProvider for CIImage {}
pub trait CIImage_CIImageProvider: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_size__format_colorSpace_options_(
        &self,
        provider: id,
        width: usize,
        height: usize,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : provider, size : width, height : height, format : format, colorSpace : colorSpace, options : options)
    }
    unsafe fn imageWithImageProvider_size__format_colorSpace_options_(
        provider: id,
        width: usize,
        height: usize,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithImageProvider : provider, size : width, height : height, format : format, colorSpace : colorSpace, options : options)
    }
}
pub trait NSObject_CIImageProvider: Sized + std::ops::Deref {
    unsafe fn provideImageData_bytesPerRow_origin__size__userInfo_(
        &self,
        data: *mut ::std::os::raw::c_void,
        rowbytes: usize,
        originx: usize,
        originy: usize,
        width: usize,
        height: usize,
        info: id,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provideImageData : data, bytesPerRow : rowbytes, origin : originx, originy : originy, size : width, height : height, userInfo : info)
    }
    unsafe fn provideImageToMTLTexture_commandBuffer_originx_originy_width_height_userInfo_(
        &self,
        texture: *mut u64,
        commandBuffer: *mut u64,
        originx: usize,
        originy: usize,
        width: usize,
        height: usize,
        info: id,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provideImageToMTLTexture : texture, commandBuffer : commandBuffer, originx : originx, originy : originy, width : width, height : height, userInfo : info)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIImageProcessorKernel(pub id);
impl std::ops::Deref for CIImageProcessorKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIImageProcessorKernel {}
impl CIImageProcessorKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), alloc) })
    }
}
impl INSObject for CIImageProcessorKernel {}
impl PNSObject for CIImageProcessorKernel {}
impl std::convert::TryFrom<NSObject> for CIImageProcessorKernel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIImageProcessorKernel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap()) };
        if is_kind_of {
            Ok(CIImageProcessorKernel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIImageProcessorKernel")
        }
    }
}
impl ICIImageProcessorKernel for CIImageProcessorKernel {}
pub trait ICIImageProcessorKernel: Sized + std::ops::Deref {
    unsafe fn processWithInputs_arguments_output_error_(
        inputs: NSArray,
        arguments: NSDictionary,
        output: *mut u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), processWithInputs : inputs, arguments : arguments, output : output, error : error)
    }
    unsafe fn roiForInput_arguments_outputRect_(
        inputIndex: ::std::os::raw::c_int,
        arguments: NSDictionary,
        outputRect: CGRect,
    ) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), roiForInput : inputIndex, arguments : arguments, outputRect : outputRect)
    }
    unsafe fn roiTileArrayForInput_arguments_outputRect_(
        inputIndex: ::std::os::raw::c_int,
        arguments: NSDictionary,
        outputRect: CGRect,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), roiTileArrayForInput : inputIndex, arguments : arguments, outputRect : outputRect)
    }
    unsafe fn formatForInputAtIndex_(inputIndex: ::std::os::raw::c_int) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), formatForInputAtIndex : inputIndex)
    }
    unsafe fn applyWithExtent_inputs_arguments_error_(
        extent: CGRect,
        inputs: NSArray,
        arguments: NSDictionary,
        error: *mut NSError,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), applyWithExtent : extent, inputs : inputs, arguments : arguments, error : error)
    }
    unsafe fn outputFormat() -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), outputFormat)
    }
    unsafe fn outputIsOpaque() -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), outputIsOpaque)
    }
    unsafe fn synchronizeInputs() -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), synchronizeInputs)
    }
}
impl CIImageProcessorKernel_MultipleOutputSupport for CIImageProcessorKernel {}
pub trait CIImageProcessorKernel_MultipleOutputSupport: Sized + std::ops::Deref {
    unsafe fn processWithInputs_arguments_outputs_error_(
        inputs: NSArray,
        arguments: NSDictionary,
        outputs: NSArray,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), processWithInputs : inputs, arguments : arguments, outputs : outputs, error : error)
    }
    unsafe fn outputFormatAtIndex_arguments_(
        outputIndex: ::std::os::raw::c_int,
        arguments: NSDictionary,
    ) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), outputFormatAtIndex : outputIndex, arguments : arguments)
    }
    unsafe fn applyWithExtents_inputs_arguments_error_(
        extents: NSArray,
        inputs: NSArray,
        arguments: NSDictionary,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), applyWithExtents : extents, inputs : inputs, arguments : arguments, error : error)
    }
}
pub trait PCIImageProcessorInput: Sized + std::ops::Deref {
    unsafe fn region(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn bytesPerRow(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesPerRow)
    }
    unsafe fn format(&self) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn baseAddress(&self) -> *const ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseAddress)
    }
    unsafe fn surface(&self) -> IOSurfaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, surface)
    }
    unsafe fn pixelBuffer(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBuffer)
    }
    unsafe fn metalTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalTexture)
    }
    unsafe fn digest(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, digest)
    }
    unsafe fn roiTileIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roiTileIndex)
    }
    unsafe fn roiTileCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roiTileCount)
    }
}
pub trait PCIImageProcessorOutput: Sized + std::ops::Deref {
    unsafe fn region(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn bytesPerRow(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesPerRow)
    }
    unsafe fn format(&self) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn baseAddress(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseAddress)
    }
    unsafe fn surface(&self) -> IOSurfaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, surface)
    }
    unsafe fn pixelBuffer(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBuffer)
    }
    unsafe fn metalTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalTexture)
    }
    unsafe fn metalCommandBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalCommandBuffer)
    }
    unsafe fn digest(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, digest)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIImageAccumulator(pub id);
impl std::ops::Deref for CIImageAccumulator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIImageAccumulator {}
impl CIImageAccumulator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageAccumulator").unwrap(), alloc) })
    }
}
impl INSObject for CIImageAccumulator {}
impl PNSObject for CIImageAccumulator {}
impl std::convert::TryFrom<NSObject> for CIImageAccumulator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIImageAccumulator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIImageAccumulator").unwrap()) };
        if is_kind_of {
            Ok(CIImageAccumulator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIImageAccumulator")
        }
    }
}
impl ICIImageAccumulator for CIImageAccumulator {}
pub trait ICIImageAccumulator: Sized + std::ops::Deref {
    unsafe fn initWithExtent_format_(&self, extent: CGRect, format: CIFormat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExtent : extent, format : format)
    }
    unsafe fn initWithExtent_format_colorSpace_(
        &self,
        extent: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExtent : extent, format : format, colorSpace : colorSpace)
    }
    unsafe fn image(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn setImage_dirtyRect_(&self, image: CIImage, dirtyRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image, dirtyRect : dirtyRect)
    }
    unsafe fn clear(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clear)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn format(&self) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn imageAccumulatorWithExtent_format_(extent: CGRect, format: CIFormat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageAccumulator").unwrap(), imageAccumulatorWithExtent : extent, format : format)
    }
    unsafe fn imageAccumulatorWithExtent_format_colorSpace_(
        extent: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageAccumulator").unwrap(), imageAccumulatorWithExtent : extent, format : format, colorSpace : colorSpace)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFilterShape(pub id);
impl std::ops::Deref for CIFilterShape {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFilterShape {}
impl CIFilterShape {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterShape").unwrap(), alloc) })
    }
}
impl PNSCopying for CIFilterShape {}
impl INSObject for CIFilterShape {}
impl PNSObject for CIFilterShape {}
impl std::convert::TryFrom<NSObject> for CIFilterShape {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIFilterShape, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFilterShape").unwrap()) };
        if is_kind_of {
            Ok(CIFilterShape(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIFilterShape")
        }
    }
}
impl ICIFilterShape for CIFilterShape {}
pub trait ICIFilterShape: Sized + std::ops::Deref {
    unsafe fn initWithRect_(&self, r: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRect : r)
    }
    unsafe fn transformBy_interior_(&self, m: CGAffineTransform, flag: BOOL) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transformBy : m, interior : flag)
    }
    unsafe fn insetByX_Y_(
        &self,
        dx: ::std::os::raw::c_int,
        dy: ::std::os::raw::c_int,
    ) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insetByX : dx, Y : dy)
    }
    unsafe fn unionWith_(&self, s2: CIFilterShape) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unionWith : s2)
    }
    unsafe fn unionWithRect_(&self, r: CGRect) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unionWithRect : r)
    }
    unsafe fn intersectWith_(&self, s2: CIFilterShape) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intersectWith : s2)
    }
    unsafe fn intersectWithRect_(&self, r: CGRect) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intersectWithRect : r)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn shapeWithRect_(r: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterShape").unwrap(), shapeWithRect : r)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CISampler(pub id);
impl std::ops::Deref for CISampler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CISampler {}
impl CISampler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CISampler").unwrap(), alloc) })
    }
}
impl PNSCopying for CISampler {}
impl INSObject for CISampler {}
impl PNSObject for CISampler {}
impl std::convert::TryFrom<NSObject> for CISampler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CISampler, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CISampler").unwrap()) };
        if is_kind_of {
            Ok(CISampler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CISampler")
        }
    }
}
impl ICISampler for CISampler {}
pub trait ICISampler: Sized + std::ops::Deref {
    unsafe fn initWithImage_(&self, im: CIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : im)
    }
    unsafe fn initWithImage_keysAndValues_(&self, im: CIImage, key0: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : im, keysAndValues : key0)
    }
    unsafe fn initWithImage_options_(&self, im: CIImage, dict: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : im, options : dict)
    }
    unsafe fn definition(&self) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, definition)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn samplerWithImage_(im: CIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CISampler").unwrap(), samplerWithImage : im)
    }
    unsafe fn samplerWithImage_keysAndValues_(im: CIImage, key0: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CISampler").unwrap(), samplerWithImage : im, keysAndValues : key0)
    }
    unsafe fn samplerWithImage_options_(im: CIImage, dict: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CISampler").unwrap(), samplerWithImage : im, options : dict)
    }
}
pub type CIRAWFilterOption = NSString;
impl CIFilter_CIRAWFilter for CIFilter {}
pub trait CIFilter_CIRAWFilter: Sized + std::ops::Deref {
    unsafe fn filterWithImageURL_options_(url: NSURL, options: NSDictionary) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithImageURL : url, options : options)
    }
    unsafe fn filterWithImageData_options_(data: NSData, options: NSDictionary) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithImageData : data, options : options)
    }
    unsafe fn filterWithCVPixelBuffer_properties_options_(
        pixelBuffer: CVPixelBufferRef,
        properties: NSDictionary,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithCVPixelBuffer : pixelBuffer, properties : properties, options : options)
    }
    unsafe fn supportedRawCameraModels() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), supportedRawCameraModels)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRAWFilter(pub id);
impl std::ops::Deref for CIRAWFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRAWFilter {}
impl CIRAWFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), alloc) })
    }
}
impl ICIFilter for CIRAWFilter {}
impl PNSSecureCoding for CIRAWFilter {}
impl PNSCopying for CIRAWFilter {}
impl From<CIRAWFilter> for CIFilter {
    fn from(child: CIRAWFilter) -> CIFilter {
        CIFilter(child.0)
    }
}
impl std::convert::TryFrom<CIFilter> for CIRAWFilter {
    type Error = &'static str;
    fn try_from(parent: CIFilter) -> Result<CIRAWFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap()) };
        if is_kind_of {
            Ok(CIRAWFilter(parent.0))
        } else {
            Err("This CIFilter cannot be downcasted to CIRAWFilter")
        }
    }
}
impl INSObject for CIRAWFilter {}
impl PNSObject for CIRAWFilter {}
impl ICIRAWFilter for CIRAWFilter {}
pub trait ICIRAWFilter: Sized + std::ops::Deref {
    unsafe fn supportedDecoderVersions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedDecoderVersions)
    }
    unsafe fn nativeSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nativeSize)
    }
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn orientation(&self) -> CGImagePropertyOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
    unsafe fn setOrientation_(&self, orientation: CGImagePropertyOrientation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrientation : orientation)
    }
    unsafe fn isDraftModeEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDraftModeEnabled)
    }
    unsafe fn setDraftModeEnabled_(&self, draftModeEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDraftModeEnabled : draftModeEnabled)
    }
    unsafe fn decoderVersion(&self) -> CIRAWDecoderVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decoderVersion)
    }
    unsafe fn setDecoderVersion_(&self, decoderVersion: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDecoderVersion : decoderVersion)
    }
    unsafe fn scaleFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn setScaleFactor_(&self, scaleFactor: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleFactor : scaleFactor)
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
    unsafe fn baselineExposure(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baselineExposure)
    }
    unsafe fn setBaselineExposure_(&self, baselineExposure: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaselineExposure : baselineExposure)
    }
    unsafe fn shadowBias(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowBias)
    }
    unsafe fn setShadowBias_(&self, shadowBias: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowBias : shadowBias)
    }
    unsafe fn boostAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boostAmount)
    }
    unsafe fn setBoostAmount_(&self, boostAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoostAmount : boostAmount)
    }
    unsafe fn boostShadowAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boostShadowAmount)
    }
    unsafe fn setBoostShadowAmount_(&self, boostShadowAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoostShadowAmount : boostShadowAmount)
    }
    unsafe fn isHighlightRecoverySupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHighlightRecoverySupported)
    }
    unsafe fn isHighlightRecoveryEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHighlightRecoveryEnabled)
    }
    unsafe fn setHighlightRecoveryEnabled_(&self, highlightRecoveryEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightRecoveryEnabled : highlightRecoveryEnabled)
    }
    unsafe fn isGamutMappingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGamutMappingEnabled)
    }
    unsafe fn setGamutMappingEnabled_(&self, gamutMappingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGamutMappingEnabled : gamutMappingEnabled)
    }
    unsafe fn isLensCorrectionSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLensCorrectionSupported)
    }
    unsafe fn isLensCorrectionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLensCorrectionEnabled)
    }
    unsafe fn setLensCorrectionEnabled_(&self, lensCorrectionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLensCorrectionEnabled : lensCorrectionEnabled)
    }
    unsafe fn isLuminanceNoiseReductionSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLuminanceNoiseReductionSupported)
    }
    unsafe fn luminanceNoiseReductionAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, luminanceNoiseReductionAmount)
    }
    unsafe fn setLuminanceNoiseReductionAmount_(&self, luminanceNoiseReductionAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLuminanceNoiseReductionAmount : luminanceNoiseReductionAmount)
    }
    unsafe fn isColorNoiseReductionSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isColorNoiseReductionSupported)
    }
    unsafe fn colorNoiseReductionAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorNoiseReductionAmount)
    }
    unsafe fn setColorNoiseReductionAmount_(&self, colorNoiseReductionAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorNoiseReductionAmount : colorNoiseReductionAmount)
    }
    unsafe fn isSharpnessSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSharpnessSupported)
    }
    unsafe fn sharpnessAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpnessAmount)
    }
    unsafe fn setSharpnessAmount_(&self, sharpnessAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpnessAmount : sharpnessAmount)
    }
    unsafe fn isContrastSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContrastSupported)
    }
    unsafe fn contrastAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrastAmount)
    }
    unsafe fn setContrastAmount_(&self, contrastAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrastAmount : contrastAmount)
    }
    unsafe fn isDetailSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDetailSupported)
    }
    unsafe fn detailAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailAmount)
    }
    unsafe fn setDetailAmount_(&self, detailAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetailAmount : detailAmount)
    }
    unsafe fn isMoireReductionSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMoireReductionSupported)
    }
    unsafe fn moireReductionAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, moireReductionAmount)
    }
    unsafe fn setMoireReductionAmount_(&self, moireReductionAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMoireReductionAmount : moireReductionAmount)
    }
    unsafe fn isLocalToneMapSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocalToneMapSupported)
    }
    unsafe fn localToneMapAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localToneMapAmount)
    }
    unsafe fn setLocalToneMapAmount_(&self, localToneMapAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalToneMapAmount : localToneMapAmount)
    }
    unsafe fn extendedDynamicRangeAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extendedDynamicRangeAmount)
    }
    unsafe fn setExtendedDynamicRangeAmount_(&self, extendedDynamicRangeAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtendedDynamicRangeAmount : extendedDynamicRangeAmount)
    }
    unsafe fn neutralChromaticity(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutralChromaticity)
    }
    unsafe fn setNeutralChromaticity_(&self, neutralChromaticity: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutralChromaticity : neutralChromaticity)
    }
    unsafe fn neutralLocation(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutralLocation)
    }
    unsafe fn setNeutralLocation_(&self, neutralLocation: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutralLocation : neutralLocation)
    }
    unsafe fn neutralTemperature(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutralTemperature)
    }
    unsafe fn setNeutralTemperature_(&self, neutralTemperature: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutralTemperature : neutralTemperature)
    }
    unsafe fn neutralTint(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutralTint)
    }
    unsafe fn setNeutralTint_(&self, neutralTint: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutralTint : neutralTint)
    }
    unsafe fn linearSpaceFilter(&self) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linearSpaceFilter)
    }
    unsafe fn setLinearSpaceFilter_(&self, linearSpaceFilter: CIFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinearSpaceFilter : linearSpaceFilter)
    }
    unsafe fn previewImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previewImage)
    }
    unsafe fn portraitEffectsMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, portraitEffectsMatte)
    }
    unsafe fn semanticSegmentationSkinMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationSkinMatte)
    }
    unsafe fn semanticSegmentationHairMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationHairMatte)
    }
    unsafe fn semanticSegmentationGlassesMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationGlassesMatte)
    }
    unsafe fn semanticSegmentationSkyMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationSkyMatte)
    }
    unsafe fn semanticSegmentationTeethMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationTeethMatte)
    }
    unsafe fn filterWithImageURL_(url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), filterWithImageURL : url)
    }
    unsafe fn filterWithImageData_identifierHint_(
        data: NSData,
        identifierHint: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), filterWithImageData : data, identifierHint : identifierHint)
    }
    unsafe fn filterWithCVPixelBuffer_properties_(
        buffer: CVPixelBufferRef,
        properties: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), filterWithCVPixelBuffer : buffer, properties : properties)
    }
    unsafe fn supportedCameraModels() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), supportedCameraModels)
    }
}
pub type CIRAWDecoderVersion = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRenderDestination(pub id);
impl std::ops::Deref for CIRenderDestination {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRenderDestination {}
impl CIRenderDestination {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRenderDestination").unwrap(), alloc) })
    }
}
impl INSObject for CIRenderDestination {}
impl PNSObject for CIRenderDestination {}
impl std::convert::TryFrom<NSObject> for CIRenderDestination {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIRenderDestination, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRenderDestination").unwrap()) };
        if is_kind_of {
            Ok(CIRenderDestination(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIRenderDestination")
        }
    }
}
impl ICIRenderDestination for CIRenderDestination {}
pub trait ICIRenderDestination: Sized + std::ops::Deref {
    unsafe fn initWithPixelBuffer_(&self, pixelBuffer: CVPixelBufferRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPixelBuffer : pixelBuffer)
    }
    unsafe fn initWithIOSurface_(&self, surface: IOSurface) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOSurface : surface)
    }
    unsafe fn initWithMTLTexture_commandBuffer_(
        &self,
        texture: *mut u64,
        commandBuffer: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMTLTexture : texture, commandBuffer : commandBuffer)
    }
    unsafe fn initWithWidth_height_pixelFormat_commandBuffer_mtlTextureProvider_(
        &self,
        width: NSUInteger,
        height: NSUInteger,
        pixelFormat: MTLPixelFormat,
        commandBuffer: *mut u64,
        block: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWidth : width, height : height, pixelFormat : pixelFormat, commandBuffer : commandBuffer, mtlTextureProvider : block)
    }
    unsafe fn initWithGLTexture_target_width_height_(
        &self,
        texture: ::std::os::raw::c_uint,
        target: ::std::os::raw::c_uint,
        width: NSUInteger,
        height: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGLTexture : texture, target : target, width : width, height : height)
    }
    unsafe fn initWithBitmapData_width_height_bytesPerRow_format_(
        &self,
        data: *mut ::std::os::raw::c_void,
        width: NSUInteger,
        height: NSUInteger,
        bytesPerRow: NSUInteger,
        format: CIFormat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBitmapData : data, width : width, height : height, bytesPerRow : bytesPerRow, format : format)
    }
    unsafe fn width(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn alphaMode(&self) -> CIRenderDestinationAlphaMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaMode)
    }
    unsafe fn setAlphaMode_(&self, alphaMode: CIRenderDestinationAlphaMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaMode : alphaMode)
    }
    unsafe fn isFlipped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFlipped)
    }
    unsafe fn setFlipped_(&self, flipped: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlipped : flipped)
    }
    unsafe fn isDithered(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDithered)
    }
    unsafe fn setDithered_(&self, dithered: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDithered : dithered)
    }
    unsafe fn isClamped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isClamped)
    }
    unsafe fn setClamped_(&self, clamped: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClamped : clamped)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
    unsafe fn blendKernel(&self) -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendKernel)
    }
    unsafe fn setBlendKernel_(&self, blendKernel: CIBlendKernel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendKernel : blendKernel)
    }
    unsafe fn blendsInDestinationColorSpace(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendsInDestinationColorSpace)
    }
    unsafe fn setBlendsInDestinationColorSpace_(&self, blendsInDestinationColorSpace: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendsInDestinationColorSpace : blendsInDestinationColorSpace)
    }
    unsafe fn captureTraceURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, captureTraceURL)
    }
    unsafe fn setCaptureTraceURL_(&self, captureTraceURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaptureTraceURL : captureTraceURL)
    }
}
pub type CIRenderDestinationAlphaMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRenderInfo(pub id);
impl std::ops::Deref for CIRenderInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRenderInfo {}
impl CIRenderInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRenderInfo").unwrap(), alloc) })
    }
}
impl INSObject for CIRenderInfo {}
impl PNSObject for CIRenderInfo {}
impl std::convert::TryFrom<NSObject> for CIRenderInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIRenderInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRenderInfo").unwrap()) };
        if is_kind_of {
            Ok(CIRenderInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIRenderInfo")
        }
    }
}
impl ICIRenderInfo for CIRenderInfo {}
pub trait ICIRenderInfo: Sized + std::ops::Deref {
    unsafe fn kernelExecutionTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelExecutionTime)
    }
    unsafe fn kernelCompileTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelCompileTime)
    }
    unsafe fn passCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passCount)
    }
    unsafe fn pixelsProcessed(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsProcessed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRenderTask(pub id);
impl std::ops::Deref for CIRenderTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRenderTask {}
impl CIRenderTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRenderTask").unwrap(), alloc) })
    }
}
impl INSObject for CIRenderTask {}
impl PNSObject for CIRenderTask {}
impl std::convert::TryFrom<NSObject> for CIRenderTask {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIRenderTask, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRenderTask").unwrap()) };
        if is_kind_of {
            Ok(CIRenderTask(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIRenderTask")
        }
    }
}
impl ICIRenderTask for CIRenderTask {}
pub trait ICIRenderTask: Sized + std::ops::Deref {
    unsafe fn waitUntilCompletedAndReturnError_(&self, error: *mut NSError) -> CIRenderInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitUntilCompletedAndReturnError : error)
    }
}
impl CIContext_CIRenderDestination for CIContext {}
pub trait CIContext_CIRenderDestination: Sized + std::ops::Deref {
    unsafe fn startTaskToRender_fromRect_toDestination_atPoint_error_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        destination: CIRenderDestination,
        atPoint: CGPoint,
        error: *mut NSError,
    ) -> CIRenderTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startTaskToRender : image, fromRect : fromRect, toDestination : destination, atPoint : atPoint, error : error)
    }
    unsafe fn startTaskToRender_toDestination_error_(
        &self,
        image: CIImage,
        destination: CIRenderDestination,
        error: *mut NSError,
    ) -> CIRenderTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startTaskToRender : image, toDestination : destination, error : error)
    }
    unsafe fn prepareRender_fromRect_toDestination_atPoint_error_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        destination: CIRenderDestination,
        atPoint: CGPoint,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareRender : image, fromRect : fromRect, toDestination : destination, atPoint : atPoint, error : error)
    }
    unsafe fn startTaskToClear_error_(
        &self,
        destination: CIRenderDestination,
        error: *mut NSError,
    ) -> CIRenderTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startTaskToClear : destination, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIBarcodeDescriptor(pub id);
impl std::ops::Deref for CIBarcodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIBarcodeDescriptor {}
impl CIBarcodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIBarcodeDescriptor").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIBarcodeDescriptor {}
impl PNSCopying for CIBarcodeDescriptor {}
impl INSObject for CIBarcodeDescriptor {}
impl PNSObject for CIBarcodeDescriptor {}
impl std::convert::TryFrom<NSObject> for CIBarcodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIBarcodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIBarcodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIBarcodeDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIBarcodeDescriptor")
        }
    }
}
impl ICIBarcodeDescriptor for CIBarcodeDescriptor {}
pub trait ICIBarcodeDescriptor: Sized + std::ops::Deref {}
pub type CIQRCodeErrorCorrectionLevel = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIQRCodeDescriptor(pub id);
impl std::ops::Deref for CIQRCodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIQRCodeDescriptor {}
impl CIQRCodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIQRCodeDescriptor").unwrap(), alloc) })
    }
}
impl ICIBarcodeDescriptor for CIQRCodeDescriptor {}
impl PNSSecureCoding for CIQRCodeDescriptor {}
impl PNSCopying for CIQRCodeDescriptor {}
impl From<CIQRCodeDescriptor> for CIBarcodeDescriptor {
    fn from(child: CIQRCodeDescriptor) -> CIBarcodeDescriptor {
        CIBarcodeDescriptor(child.0)
    }
}
impl std::convert::TryFrom<CIBarcodeDescriptor> for CIQRCodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: CIBarcodeDescriptor) -> Result<CIQRCodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIQRCodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIQRCodeDescriptor(parent.0))
        } else {
            Err("This CIBarcodeDescriptor cannot be downcasted to CIQRCodeDescriptor")
        }
    }
}
impl INSObject for CIQRCodeDescriptor {}
impl PNSObject for CIQRCodeDescriptor {}
impl ICIQRCodeDescriptor for CIQRCodeDescriptor {}
pub trait ICIQRCodeDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithPayload_symbolVersion_maskPattern_errorCorrectionLevel_(
        &self,
        errorCorrectedPayload: NSData,
        symbolVersion: NSInteger,
        maskPattern: u8,
        errorCorrectionLevel: CIQRCodeErrorCorrectionLevel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPayload : errorCorrectedPayload, symbolVersion : symbolVersion, maskPattern : maskPattern, errorCorrectionLevel : errorCorrectionLevel)
    }
    unsafe fn errorCorrectedPayload(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectedPayload)
    }
    unsafe fn symbolVersion(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symbolVersion)
    }
    unsafe fn maskPattern(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskPattern)
    }
    unsafe fn errorCorrectionLevel(&self) -> CIQRCodeErrorCorrectionLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectionLevel)
    }
    unsafe fn descriptorWithPayload_symbolVersion_maskPattern_errorCorrectionLevel_(
        errorCorrectedPayload: NSData,
        symbolVersion: NSInteger,
        maskPattern: u8,
        errorCorrectionLevel: CIQRCodeErrorCorrectionLevel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIQRCodeDescriptor").unwrap(), descriptorWithPayload : errorCorrectedPayload, symbolVersion : symbolVersion, maskPattern : maskPattern, errorCorrectionLevel : errorCorrectionLevel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIAztecCodeDescriptor(pub id);
impl std::ops::Deref for CIAztecCodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIAztecCodeDescriptor {}
impl CIAztecCodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIAztecCodeDescriptor").unwrap(), alloc) })
    }
}
impl ICIBarcodeDescriptor for CIAztecCodeDescriptor {}
impl PNSSecureCoding for CIAztecCodeDescriptor {}
impl PNSCopying for CIAztecCodeDescriptor {}
impl std::convert::TryFrom<CIBarcodeDescriptor> for CIAztecCodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: CIBarcodeDescriptor) -> Result<CIAztecCodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIAztecCodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIAztecCodeDescriptor(parent.0))
        } else {
            Err("This CIBarcodeDescriptor cannot be downcasted to CIAztecCodeDescriptor")
        }
    }
}
impl INSObject for CIAztecCodeDescriptor {}
impl PNSObject for CIAztecCodeDescriptor {}
impl ICIAztecCodeDescriptor for CIAztecCodeDescriptor {}
pub trait ICIAztecCodeDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithPayload_isCompact_layerCount_dataCodewordCount_(
        &self,
        errorCorrectedPayload: NSData,
        isCompact: BOOL,
        layerCount: NSInteger,
        dataCodewordCount: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPayload : errorCorrectedPayload, isCompact : isCompact, layerCount : layerCount, dataCodewordCount : dataCodewordCount)
    }
    unsafe fn errorCorrectedPayload(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectedPayload)
    }
    unsafe fn isCompact(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompact)
    }
    unsafe fn layerCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerCount)
    }
    unsafe fn dataCodewordCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataCodewordCount)
    }
    unsafe fn descriptorWithPayload_isCompact_layerCount_dataCodewordCount_(
        errorCorrectedPayload: NSData,
        isCompact: BOOL,
        layerCount: NSInteger,
        dataCodewordCount: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIAztecCodeDescriptor").unwrap(), descriptorWithPayload : errorCorrectedPayload, isCompact : isCompact, layerCount : layerCount, dataCodewordCount : dataCodewordCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIPDF417CodeDescriptor(pub id);
impl std::ops::Deref for CIPDF417CodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIPDF417CodeDescriptor {}
impl CIPDF417CodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIPDF417CodeDescriptor").unwrap(), alloc) })
    }
}
impl ICIBarcodeDescriptor for CIPDF417CodeDescriptor {}
impl PNSSecureCoding for CIPDF417CodeDescriptor {}
impl PNSCopying for CIPDF417CodeDescriptor {}
impl std::convert::TryFrom<CIBarcodeDescriptor> for CIPDF417CodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: CIBarcodeDescriptor) -> Result<CIPDF417CodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIPDF417CodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIPDF417CodeDescriptor(parent.0))
        } else {
            Err("This CIBarcodeDescriptor cannot be downcasted to CIPDF417CodeDescriptor")
        }
    }
}
impl INSObject for CIPDF417CodeDescriptor {}
impl PNSObject for CIPDF417CodeDescriptor {}
impl ICIPDF417CodeDescriptor for CIPDF417CodeDescriptor {}
pub trait ICIPDF417CodeDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithPayload_isCompact_rowCount_columnCount_(
        &self,
        errorCorrectedPayload: NSData,
        isCompact: BOOL,
        rowCount: NSInteger,
        columnCount: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPayload : errorCorrectedPayload, isCompact : isCompact, rowCount : rowCount, columnCount : columnCount)
    }
    unsafe fn errorCorrectedPayload(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectedPayload)
    }
    unsafe fn isCompact(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompact)
    }
    unsafe fn rowCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rowCount)
    }
    unsafe fn columnCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, columnCount)
    }
    unsafe fn descriptorWithPayload_isCompact_rowCount_columnCount_(
        errorCorrectedPayload: NSData,
        isCompact: BOOL,
        rowCount: NSInteger,
        columnCount: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPDF417CodeDescriptor").unwrap(), descriptorWithPayload : errorCorrectedPayload, isCompact : isCompact, rowCount : rowCount, columnCount : columnCount)
    }
}
pub type CIDataMatrixCodeECCVersion = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIDataMatrixCodeDescriptor(pub id);
impl std::ops::Deref for CIDataMatrixCodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIDataMatrixCodeDescriptor {}
impl CIDataMatrixCodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIDataMatrixCodeDescriptor").unwrap(), alloc) })
    }
}
impl ICIBarcodeDescriptor for CIDataMatrixCodeDescriptor {}
impl PNSSecureCoding for CIDataMatrixCodeDescriptor {}
impl PNSCopying for CIDataMatrixCodeDescriptor {}
impl std::convert::TryFrom<CIBarcodeDescriptor> for CIDataMatrixCodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: CIBarcodeDescriptor) -> Result<CIDataMatrixCodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIDataMatrixCodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIDataMatrixCodeDescriptor(parent.0))
        } else {
            Err("This CIBarcodeDescriptor cannot be downcasted to CIDataMatrixCodeDescriptor")
        }
    }
}
impl INSObject for CIDataMatrixCodeDescriptor {}
impl PNSObject for CIDataMatrixCodeDescriptor {}
impl ICIDataMatrixCodeDescriptor for CIDataMatrixCodeDescriptor {}
pub trait ICIDataMatrixCodeDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithPayload_rowCount_columnCount_eccVersion_(
        &self,
        errorCorrectedPayload: NSData,
        rowCount: NSInteger,
        columnCount: NSInteger,
        eccVersion: CIDataMatrixCodeECCVersion,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPayload : errorCorrectedPayload, rowCount : rowCount, columnCount : columnCount, eccVersion : eccVersion)
    }
    unsafe fn errorCorrectedPayload(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectedPayload)
    }
    unsafe fn rowCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rowCount)
    }
    unsafe fn columnCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, columnCount)
    }
    unsafe fn eccVersion(&self) -> CIDataMatrixCodeECCVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eccVersion)
    }
    unsafe fn descriptorWithPayload_rowCount_columnCount_eccVersion_(
        errorCorrectedPayload: NSData,
        rowCount: NSInteger,
        columnCount: NSInteger,
        eccVersion: CIDataMatrixCodeECCVersion,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIDataMatrixCodeDescriptor").unwrap(), descriptorWithPayload : errorCorrectedPayload, rowCount : rowCount, columnCount : columnCount, eccVersion : eccVersion)
    }
}
pub trait NSUserActivity_CIBarcodeDescriptor: Sized + std::ops::Deref {
    unsafe fn detectedBarcodeDescriptor(&self) -> CIBarcodeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectedBarcodeDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFilterGenerator(pub id);
impl std::ops::Deref for CIFilterGenerator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFilterGenerator {}
impl CIFilterGenerator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterGenerator").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIFilterGenerator {}
impl PNSCopying for CIFilterGenerator {}
impl PCIFilterConstructor for CIFilterGenerator {}
impl INSObject for CIFilterGenerator {}
impl PNSObject for CIFilterGenerator {}
impl std::convert::TryFrom<NSObject> for CIFilterGenerator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIFilterGenerator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFilterGenerator").unwrap()) };
        if is_kind_of {
            Ok(CIFilterGenerator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIFilterGenerator")
        }
    }
}
impl ICIFilterGenerator for CIFilterGenerator {}
pub trait ICIFilterGenerator: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_(&self, aURL: NSURL) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : aURL)
    }
    unsafe fn connectObject_withKey_toObject_withKey_(
        &self,
        sourceObject: id,
        sourceKey: NSString,
        targetObject: id,
        targetKey: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectObject : sourceObject, withKey : sourceKey, toObject : targetObject, withKey : targetKey)
    }
    unsafe fn disconnectObject_withKey_toObject_withKey_(
        &self,
        sourceObject: id,
        sourceKey: NSString,
        targetObject: id,
        targetKey: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectObject : sourceObject, withKey : sourceKey, toObject : targetObject, withKey : targetKey)
    }
    unsafe fn exportKey_fromObject_withName_(
        &self,
        key: NSString,
        targetObject: id,
        exportedKeyName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exportKey : key, fromObject : targetObject, withName : exportedKeyName)
    }
    unsafe fn removeExportedKey_(&self, exportedKeyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeExportedKey : exportedKeyName)
    }
    unsafe fn setAttributes_forExportedKey_(&self, attributes: NSDictionary, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributes : attributes, forExportedKey : key)
    }
    unsafe fn filter(&self) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filter)
    }
    unsafe fn registerFilterName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerFilterName : name)
    }
    unsafe fn writeToURL_atomically_(&self, aURL: NSURL, flag: BOOL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : aURL, atomically : flag)
    }
    unsafe fn exportedKeys(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exportedKeys)
    }
    unsafe fn classAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classAttributes)
    }
    unsafe fn setClassAttributes_(&self, classAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClassAttributes : classAttributes)
    }
    unsafe fn filterGenerator() -> CIFilterGenerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterGenerator").unwrap(), filterGenerator)
    }
    unsafe fn filterGeneratorWithContentsOfURL_(aURL: NSURL) -> CIFilterGenerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterGenerator").unwrap(), filterGeneratorWithContentsOfURL : aURL)
    }
}
pub trait PCIPlugInRegistration: Sized + std::ops::Deref {
    unsafe fn load_(&self, host: *mut ::std::os::raw::c_void) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, load : host)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIPlugIn(pub id);
impl std::ops::Deref for CIPlugIn {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIPlugIn {}
impl CIPlugIn {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), alloc) })
    }
}
impl INSObject for CIPlugIn {}
impl PNSObject for CIPlugIn {}
impl std::convert::TryFrom<NSObject> for CIPlugIn {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIPlugIn, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap()) };
        if is_kind_of {
            Ok(CIPlugIn(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIPlugIn")
        }
    }
}
impl ICIPlugIn for CIPlugIn {}
pub trait ICIPlugIn: Sized + std::ops::Deref {
    unsafe fn loadAllPlugIns()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadAllPlugIns)
    }
    unsafe fn loadNonExecutablePlugIns()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadNonExecutablePlugIns)
    }
    unsafe fn loadPlugIn_allowNonExecutable_(url: NSURL, allowNonExecutable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadPlugIn : url, allowNonExecutable : allowNonExecutable)
    }
    unsafe fn loadPlugIn_allowExecutableCode_(url: NSURL, allowExecutableCode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadPlugIn : url, allowExecutableCode : allowExecutableCode)
    }
    unsafe fn loadNonExecutablePlugIn_(url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadNonExecutablePlugIn : url)
    }
}
pub trait PCIDistanceGradientFromRedMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn maximumDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDistance)
    }
    unsafe fn setMaximumDistance_(&self, maximumDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumDistance : maximumDistance)
    }
}
pub trait PCIGaussianGradient: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIHueSaturationValueGradient: Sized + std::ops::Deref {
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn softness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, softness)
    }
    unsafe fn setSoftness_(&self, softness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSoftness : softness)
    }
    unsafe fn dither(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dither)
    }
    unsafe fn setDither_(&self, dither: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDither : dither)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
}
pub trait PCILinearGradient: Sized + std::ops::Deref {
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCIRadialGradient: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius0(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius0)
    }
    unsafe fn setRadius0_(&self, radius0: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius0 : radius0)
    }
    unsafe fn radius1(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius1)
    }
    unsafe fn setRadius1_(&self, radius1: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius1 : radius1)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCISignedDistanceGradientFromRedMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn maximumDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDistance)
    }
    unsafe fn setMaximumDistance_(&self, maximumDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumDistance : maximumDistance)
    }
}
pub trait PCISmoothLinearGradient: Sized + std::ops::Deref {
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCISharpenLuminance: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIUnsharpMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCICircularScreen: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCICMYKHalftone: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
    unsafe fn grayComponentReplacement(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, grayComponentReplacement)
    }
    unsafe fn setGrayComponentReplacement_(&self, grayComponentReplacement: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrayComponentReplacement : grayComponentReplacement)
    }
    unsafe fn underColorRemoval(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, underColorRemoval)
    }
    unsafe fn setUnderColorRemoval_(&self, underColorRemoval: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnderColorRemoval : underColorRemoval)
    }
}
pub trait PCIDotScreen: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCIHatchedScreen: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCILineScreen: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCIFourCoordinateGeometryFilter: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn topLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLeft)
    }
    unsafe fn setTopLeft_(&self, topLeft: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopLeft : topLeft)
    }
    unsafe fn topRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topRight)
    }
    unsafe fn setTopRight_(&self, topRight: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopRight : topRight)
    }
    unsafe fn bottomRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomRight)
    }
    unsafe fn setBottomRight_(&self, bottomRight: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomRight : bottomRight)
    }
    unsafe fn bottomLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomLeft)
    }
    unsafe fn setBottomLeft_(&self, bottomLeft: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomLeft : bottomLeft)
    }
}
pub trait PCIBicubicScaleTransform: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn aspectRatio(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatio)
    }
    unsafe fn setAspectRatio_(&self, aspectRatio: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAspectRatio : aspectRatio)
    }
    unsafe fn parameterB(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterB)
    }
    unsafe fn setParameterB_(&self, parameterB: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParameterB : parameterB)
    }
    unsafe fn parameterC(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterC)
    }
    unsafe fn setParameterC_(&self, parameterC: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParameterC : parameterC)
    }
}
pub trait PCIEdgePreserveUpsample: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn smallImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smallImage)
    }
    unsafe fn setSmallImage_(&self, smallImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmallImage : smallImage)
    }
    unsafe fn spatialSigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spatialSigma)
    }
    unsafe fn setSpatialSigma_(&self, spatialSigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpatialSigma : spatialSigma)
    }
    unsafe fn lumaSigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lumaSigma)
    }
    unsafe fn setLumaSigma_(&self, lumaSigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLumaSigma : lumaSigma)
    }
}
pub trait PCIKeystoneCorrectionCombined: Sized + std::ops::Deref {
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
}
pub trait PCIKeystoneCorrectionHorizontal: Sized + std::ops::Deref {
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
}
pub trait PCIKeystoneCorrectionVertical: Sized + std::ops::Deref {
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
}
pub trait PCILanczosScaleTransform: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn aspectRatio(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatio)
    }
    unsafe fn setAspectRatio_(&self, aspectRatio: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAspectRatio : aspectRatio)
    }
}
pub trait PCIMaximumScaleTransform: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn aspectRatio(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatio)
    }
    unsafe fn setAspectRatio_(&self, aspectRatio: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAspectRatio : aspectRatio)
    }
}
pub trait PCIPerspectiveCorrection: Sized + std::ops::Deref {
    unsafe fn crop(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crop)
    }
    unsafe fn setCrop_(&self, crop: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrop : crop)
    }
}
pub trait PCIPerspectiveRotate: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
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
    unsafe fn pitch(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitch)
    }
    unsafe fn setPitch_(&self, pitch: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPitch : pitch)
    }
    unsafe fn yaw(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yaw)
    }
    unsafe fn setYaw_(&self, yaw: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setYaw : yaw)
    }
    unsafe fn roll(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roll)
    }
    unsafe fn setRoll_(&self, roll: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoll : roll)
    }
}
pub trait PCIPerspectiveTransform: Sized + std::ops::Deref {}
pub trait PCIPerspectiveTransformWithExtent: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
}
pub trait PCIStraighten: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCITransitionFilter: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn targetImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetImage)
    }
    unsafe fn setTargetImage_(&self, targetImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetImage : targetImage)
    }
    unsafe fn time(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn setTime_(&self, time: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTime : time)
    }
}
pub trait PCIAccordionFoldTransition: Sized + std::ops::Deref {
    unsafe fn bottomHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomHeight)
    }
    unsafe fn setBottomHeight_(&self, bottomHeight: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomHeight : bottomHeight)
    }
    unsafe fn numberOfFolds(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfFolds)
    }
    unsafe fn setNumberOfFolds_(&self, numberOfFolds: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfFolds : numberOfFolds)
    }
    unsafe fn foldShadowAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foldShadowAmount)
    }
    unsafe fn setFoldShadowAmount_(&self, foldShadowAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFoldShadowAmount : foldShadowAmount)
    }
}
pub trait PCIBarsSwipeTransition: Sized + std::ops::Deref {
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn barOffset(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, barOffset)
    }
    unsafe fn setBarOffset_(&self, barOffset: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBarOffset : barOffset)
    }
}
pub trait PCICopyMachineTransition: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn opacity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opacity)
    }
    unsafe fn setOpacity_(&self, opacity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpacity : opacity)
    }
}
pub trait PCIDisintegrateWithMaskTransition: Sized + std::ops::Deref {
    unsafe fn maskImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskImage)
    }
    unsafe fn setMaskImage_(&self, maskImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaskImage : maskImage)
    }
    unsafe fn shadowRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowRadius)
    }
    unsafe fn setShadowRadius_(&self, shadowRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowRadius : shadowRadius)
    }
    unsafe fn shadowDensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowDensity)
    }
    unsafe fn setShadowDensity_(&self, shadowDensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowDensity : shadowDensity)
    }
    unsafe fn shadowOffset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowOffset)
    }
    unsafe fn setShadowOffset_(&self, shadowOffset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowOffset : shadowOffset)
    }
}
pub trait PCIDissolveTransition: Sized + std::ops::Deref {}
pub trait PCIFlashTransition: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn maxStriationRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxStriationRadius)
    }
    unsafe fn setMaxStriationRadius_(&self, maxStriationRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxStriationRadius : maxStriationRadius)
    }
    unsafe fn striationStrength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationStrength)
    }
    unsafe fn setStriationStrength_(&self, striationStrength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationStrength : striationStrength)
    }
    unsafe fn striationContrast(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationContrast)
    }
    unsafe fn setStriationContrast_(&self, striationContrast: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationContrast : striationContrast)
    }
    unsafe fn fadeThreshold(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fadeThreshold)
    }
    unsafe fn setFadeThreshold_(&self, fadeThreshold: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFadeThreshold : fadeThreshold)
    }
}
pub trait PCIModTransition: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn compression(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compression)
    }
    unsafe fn setCompression_(&self, compression: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompression : compression)
    }
}
pub trait PCIPageCurlTransition: Sized + std::ops::Deref {
    unsafe fn backsideImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backsideImage)
    }
    unsafe fn setBacksideImage_(&self, backsideImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBacksideImage : backsideImage)
    }
    unsafe fn shadingImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadingImage)
    }
    unsafe fn setShadingImage_(&self, shadingImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadingImage : shadingImage)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIPageCurlWithShadowTransition: Sized + std::ops::Deref {
    unsafe fn backsideImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backsideImage)
    }
    unsafe fn setBacksideImage_(&self, backsideImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBacksideImage : backsideImage)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn shadowSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowSize)
    }
    unsafe fn setShadowSize_(&self, shadowSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowSize : shadowSize)
    }
    unsafe fn shadowAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowAmount)
    }
    unsafe fn setShadowAmount_(&self, shadowAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowAmount : shadowAmount)
    }
    unsafe fn shadowExtent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowExtent)
    }
    unsafe fn setShadowExtent_(&self, shadowExtent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowExtent : shadowExtent)
    }
}
pub trait PCIRippleTransition: Sized + std::ops::Deref {
    unsafe fn shadingImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadingImage)
    }
    unsafe fn setShadingImage_(&self, shadingImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadingImage : shadingImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
pub trait PCISwipeTransition: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn opacity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opacity)
    }
    unsafe fn setOpacity_(&self, opacity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpacity : opacity)
    }
}
pub trait PCICompositeOperation: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn backgroundImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundImage)
    }
    unsafe fn setBackgroundImage_(&self, backgroundImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundImage : backgroundImage)
    }
}
pub trait PCIColorAbsoluteDifference: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn inputImage2(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage2)
    }
    unsafe fn setInputImage2_(&self, inputImage2: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage2 : inputImage2)
    }
}
pub trait PCIColorClamp: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn minComponents(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minComponents)
    }
    unsafe fn setMinComponents_(&self, minComponents: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinComponents : minComponents)
    }
    unsafe fn maxComponents(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxComponents)
    }
    unsafe fn setMaxComponents_(&self, maxComponents: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxComponents : maxComponents)
    }
}
pub trait PCIColorControls: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
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
}
pub trait PCIColorMatrix: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn RVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, RVector)
    }
    unsafe fn setRVector_(&self, RVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRVector : RVector)
    }
    unsafe fn GVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GVector)
    }
    unsafe fn setGVector_(&self, GVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGVector : GVector)
    }
    unsafe fn BVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, BVector)
    }
    unsafe fn setBVector_(&self, BVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBVector : BVector)
    }
    unsafe fn AVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, AVector)
    }
    unsafe fn setAVector_(&self, AVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAVector : AVector)
    }
    unsafe fn biasVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biasVector)
    }
    unsafe fn setBiasVector_(&self, biasVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBiasVector : biasVector)
    }
}
pub trait PCIColorPolynomial: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn redCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, redCoefficients)
    }
    unsafe fn setRedCoefficients_(&self, redCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRedCoefficients : redCoefficients)
    }
    unsafe fn greenCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, greenCoefficients)
    }
    unsafe fn setGreenCoefficients_(&self, greenCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGreenCoefficients : greenCoefficients)
    }
    unsafe fn blueCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blueCoefficients)
    }
    unsafe fn setBlueCoefficients_(&self, blueCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlueCoefficients : blueCoefficients)
    }
    unsafe fn alphaCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaCoefficients)
    }
    unsafe fn setAlphaCoefficients_(&self, alphaCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaCoefficients : alphaCoefficients)
    }
}
pub trait PCIColorThreshold: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn threshold(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threshold)
    }
    unsafe fn setThreshold_(&self, threshold: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreshold : threshold)
    }
}
pub trait PCIColorThresholdOtsu: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIDepthToDisparity: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIDisparityToDepth: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIExposureAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn EV(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, EV)
    }
    unsafe fn setEV_(&self, EV: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEV : EV)
    }
}
pub trait PCIGammaAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn power(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, power)
    }
    unsafe fn setPower_(&self, power: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPower : power)
    }
}
pub trait PCIHueAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCILinearToSRGBToneCurve: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCISRGBToneCurveToLinear: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCISystemToneMap: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn displayHeadroom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayHeadroom)
    }
    unsafe fn setDisplayHeadroom_(&self, displayHeadroom: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayHeadroom : displayHeadroom)
    }
    unsafe fn preferredDynamicRange(&self) -> CIDynamicRangeOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDynamicRange)
    }
    unsafe fn setPreferredDynamicRange_(&self, preferredDynamicRange: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredDynamicRange : preferredDynamicRange)
    }
}
pub trait PCITemperatureAndTint: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn neutral(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutral)
    }
    unsafe fn setNeutral_(&self, neutral: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutral : neutral)
    }
    unsafe fn targetNeutral(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetNeutral)
    }
    unsafe fn setTargetNeutral_(&self, targetNeutral: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetNeutral : targetNeutral)
    }
}
pub trait PCIToneCurve: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
    }
    unsafe fn point2(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point2)
    }
    unsafe fn setPoint2_(&self, point2: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint2 : point2)
    }
    unsafe fn point3(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point3)
    }
    unsafe fn setPoint3_(&self, point3: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint3 : point3)
    }
    unsafe fn point4(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point4)
    }
    unsafe fn setPoint4_(&self, point4: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint4 : point4)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
}
pub trait PCIToneMapHeadroom: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn sourceHeadroom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceHeadroom)
    }
    unsafe fn setSourceHeadroom_(&self, sourceHeadroom: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceHeadroom : sourceHeadroom)
    }
    unsafe fn targetHeadroom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetHeadroom)
    }
    unsafe fn setTargetHeadroom_(&self, targetHeadroom: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetHeadroom : targetHeadroom)
    }
}
pub trait PCIVibrance: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn amount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
}
pub trait PCIWhitePointAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCIColorCrossPolynomial: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn redCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, redCoefficients)
    }
    unsafe fn setRedCoefficients_(&self, redCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRedCoefficients : redCoefficients)
    }
    unsafe fn greenCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, greenCoefficients)
    }
    unsafe fn setGreenCoefficients_(&self, greenCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGreenCoefficients : greenCoefficients)
    }
    unsafe fn blueCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blueCoefficients)
    }
    unsafe fn setBlueCoefficients_(&self, blueCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlueCoefficients : blueCoefficients)
    }
}
pub trait PCIColorCube: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn cubeDimension(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeDimension)
    }
    unsafe fn setCubeDimension_(&self, cubeDimension: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeDimension : cubeDimension)
    }
    unsafe fn cubeData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeData)
    }
    unsafe fn setCubeData_(&self, cubeData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeData : cubeData)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
}
pub trait PCIColorCubesMixedWithMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn maskImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskImage)
    }
    unsafe fn setMaskImage_(&self, maskImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaskImage : maskImage)
    }
    unsafe fn cubeDimension(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeDimension)
    }
    unsafe fn setCubeDimension_(&self, cubeDimension: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeDimension : cubeDimension)
    }
    unsafe fn cube0Data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cube0Data)
    }
    unsafe fn setCube0Data_(&self, cube0Data: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCube0Data : cube0Data)
    }
    unsafe fn cube1Data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cube1Data)
    }
    unsafe fn setCube1Data_(&self, cube1Data: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCube1Data : cube1Data)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
}
pub trait PCIColorCubeWithColorSpace: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn cubeDimension(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeDimension)
    }
    unsafe fn setCubeDimension_(&self, cubeDimension: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeDimension : cubeDimension)
    }
    unsafe fn cubeData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeData)
    }
    unsafe fn setCubeData_(&self, cubeData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeData : cubeData)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
}
pub trait PCIColorCurves: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn curvesData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curvesData)
    }
    unsafe fn setCurvesData_(&self, curvesData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurvesData : curvesData)
    }
    unsafe fn curvesDomain(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curvesDomain)
    }
    unsafe fn setCurvesDomain_(&self, curvesDomain: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurvesDomain : curvesDomain)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
}
pub trait PCIColorInvert: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIColorMap: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn gradientImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientImage)
    }
    unsafe fn setGradientImage_(&self, gradientImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGradientImage : gradientImage)
    }
}
pub trait PCIColorMonochrome: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIColorPosterize: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn levels(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, levels)
    }
    unsafe fn setLevels_(&self, levels: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevels : levels)
    }
}
pub trait PCIConvertLab: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn normalize(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalize)
    }
    unsafe fn setNormalize_(&self, normalize: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalize : normalize)
    }
}
pub trait PCIDither: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIDocumentEnhancer: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn amount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
}
pub trait PCIFalseColor: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCILabDeltaE: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn image2(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image2)
    }
    unsafe fn setImage2_(&self, image2: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage2 : image2)
    }
}
pub trait PCIMaskToAlpha: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIMaximumComponent: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIMinimumComponent: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIPaletteCentroid: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn paletteImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paletteImage)
    }
    unsafe fn setPaletteImage_(&self, paletteImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaletteImage : paletteImage)
    }
    unsafe fn perceptual(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perceptual)
    }
    unsafe fn setPerceptual_(&self, perceptual: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerceptual : perceptual)
    }
}
pub trait PCIPalettize: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn paletteImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paletteImage)
    }
    unsafe fn setPaletteImage_(&self, paletteImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaletteImage : paletteImage)
    }
    unsafe fn perceptual(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perceptual)
    }
    unsafe fn setPerceptual_(&self, perceptual: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerceptual : perceptual)
    }
}
pub trait PCIPhotoEffect: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
}
pub trait PCISepiaTone: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIThermal: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIVignette: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIVignetteEffect: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
    unsafe fn falloff(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, falloff)
    }
    unsafe fn setFalloff_(&self, falloff: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFalloff : falloff)
    }
}
pub trait PCIXRay: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIBumpDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
pub trait PCIBumpDistortionLinear: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
pub trait PCICircleSplashDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCICircularWrap: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCIDisplacementDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn displacementImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displacementImage)
    }
    unsafe fn setDisplacementImage_(&self, displacementImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplacementImage : displacementImage)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
pub trait PCIDroste: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn insetPoint0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insetPoint0)
    }
    unsafe fn setInsetPoint0_(&self, insetPoint0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInsetPoint0 : insetPoint0)
    }
    unsafe fn insetPoint1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insetPoint1)
    }
    unsafe fn setInsetPoint1_(&self, insetPoint1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInsetPoint1 : insetPoint1)
    }
    unsafe fn strands(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strands)
    }
    unsafe fn setStrands_(&self, strands: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrands : strands)
    }
    unsafe fn periodicity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, periodicity)
    }
    unsafe fn setPeriodicity_(&self, periodicity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPeriodicity : periodicity)
    }
    unsafe fn rotation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn zoom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoom)
    }
    unsafe fn setZoom_(&self, zoom: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZoom : zoom)
    }
}
pub trait PCIGlassDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn textureImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureImage)
    }
    unsafe fn setTextureImage_(&self, textureImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureImage : textureImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
pub trait PCIGlassLozenge: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn refraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refraction)
    }
    unsafe fn setRefraction_(&self, refraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRefraction : refraction)
    }
}
pub trait PCIHoleDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCILightTunnel: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn rotation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCINinePartStretched: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn breakpoint0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, breakpoint0)
    }
    unsafe fn setBreakpoint0_(&self, breakpoint0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBreakpoint0 : breakpoint0)
    }
    unsafe fn breakpoint1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, breakpoint1)
    }
    unsafe fn setBreakpoint1_(&self, breakpoint1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBreakpoint1 : breakpoint1)
    }
    unsafe fn growAmount(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, growAmount)
    }
    unsafe fn setGrowAmount_(&self, growAmount: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrowAmount : growAmount)
    }
}
pub trait PCINinePartTiled: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn breakpoint0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, breakpoint0)
    }
    unsafe fn setBreakpoint0_(&self, breakpoint0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBreakpoint0 : breakpoint0)
    }
    unsafe fn breakpoint1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, breakpoint1)
    }
    unsafe fn setBreakpoint1_(&self, breakpoint1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBreakpoint1 : breakpoint1)
    }
    unsafe fn growAmount(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, growAmount)
    }
    unsafe fn setGrowAmount_(&self, growAmount: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrowAmount : growAmount)
    }
    unsafe fn flipYTiles(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flipYTiles)
    }
    unsafe fn setFlipYTiles_(&self, flipYTiles: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlipYTiles : flipYTiles)
    }
}
pub trait PCIPinchDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
pub trait PCIStretchCrop: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn size(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn cropAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cropAmount)
    }
    unsafe fn setCropAmount_(&self, cropAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCropAmount : cropAmount)
    }
    unsafe fn centerStretchAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerStretchAmount)
    }
    unsafe fn setCenterStretchAmount_(&self, centerStretchAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterStretchAmount : centerStretchAmount)
    }
}
pub trait PCITorusLensDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn refraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refraction)
    }
    unsafe fn setRefraction_(&self, refraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRefraction : refraction)
    }
}
pub trait PCITwirlDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCIVortexDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCIAffineClamp: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn transform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn setTransform_(&self, transform: CGAffineTransform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransform : transform)
    }
}
pub trait PCIAffineTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn transform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn setTransform_(&self, transform: CGAffineTransform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransform : transform)
    }
}
pub trait PCIEightfoldReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIFourfoldReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn acuteAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acuteAngle)
    }
    unsafe fn setAcuteAngle_(&self, acuteAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcuteAngle : acuteAngle)
    }
}
pub trait PCIFourfoldRotatedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIFourfoldTranslatedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn acuteAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acuteAngle)
    }
    unsafe fn setAcuteAngle_(&self, acuteAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcuteAngle : acuteAngle)
    }
}
pub trait PCIGlideReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIKaleidoscope: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn setCount_(&self, count: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCount : count)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCIOpTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIParallelogramTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn acuteAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acuteAngle)
    }
    unsafe fn setAcuteAngle_(&self, acuteAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcuteAngle : acuteAngle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIPerspectiveTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn topLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLeft)
    }
    unsafe fn setTopLeft_(&self, topLeft: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopLeft : topLeft)
    }
    unsafe fn topRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topRight)
    }
    unsafe fn setTopRight_(&self, topRight: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopRight : topRight)
    }
    unsafe fn bottomRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomRight)
    }
    unsafe fn setBottomRight_(&self, bottomRight: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomRight : bottomRight)
    }
    unsafe fn bottomLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomLeft)
    }
    unsafe fn setBottomLeft_(&self, bottomLeft: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomLeft : bottomLeft)
    }
}
pub trait PCISixfoldReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCISixfoldRotatedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCITriangleKaleidoscope: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn point(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point)
    }
    unsafe fn setPoint_(&self, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint : point)
    }
    unsafe fn size(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn rotation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn decay(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decay)
    }
    unsafe fn setDecay_(&self, decay: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDecay : decay)
    }
}
pub trait PCITriangleTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCITwelvefoldReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIAttributedTextImageGenerator: Sized + std::ops::Deref {
    unsafe fn text(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn setText_(&self, text: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
    }
    unsafe fn scaleFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn setScaleFactor_(&self, scaleFactor: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleFactor : scaleFactor)
    }
    unsafe fn padding(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, padding)
    }
    unsafe fn setPadding_(&self, padding: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPadding : padding)
    }
}
pub trait PCIAztecCodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn correctionLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correctionLevel)
    }
    unsafe fn setCorrectionLevel_(&self, correctionLevel: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCorrectionLevel : correctionLevel)
    }
    unsafe fn layers(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layers)
    }
    unsafe fn setLayers_(&self, layers: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayers : layers)
    }
    unsafe fn compactStyle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compactStyle)
    }
    unsafe fn setCompactStyle_(&self, compactStyle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompactStyle : compactStyle)
    }
}
pub trait PCIBarcodeGenerator: Sized + std::ops::Deref {
    unsafe fn barcodeDescriptor(&self) -> CIBarcodeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, barcodeDescriptor)
    }
    unsafe fn setBarcodeDescriptor_(&self, barcodeDescriptor: CIBarcodeDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBarcodeDescriptor : barcodeDescriptor)
    }
}
pub trait PCIBlurredRectangleGenerator: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn sigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sigma)
    }
    unsafe fn setSigma_(&self, sigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSigma : sigma)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCIBlurredRoundedRectangleGenerator: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn smoothness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smoothness)
    }
    unsafe fn setSmoothness_(&self, smoothness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmoothness : smoothness)
    }
    unsafe fn sigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sigma)
    }
    unsafe fn setSigma_(&self, sigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSigma : sigma)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCICheckerboardGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCICode128BarcodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn quietSpace(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quietSpace)
    }
    unsafe fn setQuietSpace_(&self, quietSpace: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuietSpace : quietSpace)
    }
    unsafe fn barcodeHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, barcodeHeight)
    }
    unsafe fn setBarcodeHeight_(&self, barcodeHeight: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBarcodeHeight : barcodeHeight)
    }
}
pub trait PCILenticularHaloGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn haloRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, haloRadius)
    }
    unsafe fn setHaloRadius_(&self, haloRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHaloRadius : haloRadius)
    }
    unsafe fn haloWidth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, haloWidth)
    }
    unsafe fn setHaloWidth_(&self, haloWidth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHaloWidth : haloWidth)
    }
    unsafe fn haloOverlap(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, haloOverlap)
    }
    unsafe fn setHaloOverlap_(&self, haloOverlap: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHaloOverlap : haloOverlap)
    }
    unsafe fn striationStrength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationStrength)
    }
    unsafe fn setStriationStrength_(&self, striationStrength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationStrength : striationStrength)
    }
    unsafe fn striationContrast(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationContrast)
    }
    unsafe fn setStriationContrast_(&self, striationContrast: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationContrast : striationContrast)
    }
    unsafe fn time(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn setTime_(&self, time: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTime : time)
    }
}
pub trait PCIMeshGenerator: Sized + std::ops::Deref {
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn mesh(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mesh)
    }
    unsafe fn setMesh_(&self, mesh: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMesh : mesh)
    }
}
pub trait PCIPDF417BarcodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn minWidth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minWidth)
    }
    unsafe fn setMinWidth_(&self, minWidth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinWidth : minWidth)
    }
    unsafe fn maxWidth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxWidth)
    }
    unsafe fn setMaxWidth_(&self, maxWidth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxWidth : maxWidth)
    }
    unsafe fn minHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minHeight)
    }
    unsafe fn setMinHeight_(&self, minHeight: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinHeight : minHeight)
    }
    unsafe fn maxHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxHeight)
    }
    unsafe fn setMaxHeight_(&self, maxHeight: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxHeight : maxHeight)
    }
    unsafe fn dataColumns(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataColumns)
    }
    unsafe fn setDataColumns_(&self, dataColumns: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataColumns : dataColumns)
    }
    unsafe fn rows(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rows)
    }
    unsafe fn setRows_(&self, rows: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRows : rows)
    }
    unsafe fn preferredAspectRatio(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredAspectRatio)
    }
    unsafe fn setPreferredAspectRatio_(&self, preferredAspectRatio: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredAspectRatio : preferredAspectRatio)
    }
    unsafe fn compactionMode(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compactionMode)
    }
    unsafe fn setCompactionMode_(&self, compactionMode: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompactionMode : compactionMode)
    }
    unsafe fn compactStyle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compactStyle)
    }
    unsafe fn setCompactStyle_(&self, compactStyle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompactStyle : compactStyle)
    }
    unsafe fn correctionLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correctionLevel)
    }
    unsafe fn setCorrectionLevel_(&self, correctionLevel: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCorrectionLevel : correctionLevel)
    }
    unsafe fn alwaysSpecifyCompaction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alwaysSpecifyCompaction)
    }
    unsafe fn setAlwaysSpecifyCompaction_(&self, alwaysSpecifyCompaction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlwaysSpecifyCompaction : alwaysSpecifyCompaction)
    }
}
pub trait PCIQRCodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn correctionLevel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correctionLevel)
    }
    unsafe fn setCorrectionLevel_(&self, correctionLevel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCorrectionLevel : correctionLevel)
    }
}
pub trait PCIRandomGenerator: Sized + std::ops::Deref {}
pub trait PCIRoundedQRCodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn correctionLevel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correctionLevel)
    }
    unsafe fn setCorrectionLevel_(&self, correctionLevel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCorrectionLevel : correctionLevel)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn roundedMarkers(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roundedMarkers)
    }
    unsafe fn setRoundedMarkers_(&self, roundedMarkers: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoundedMarkers : roundedMarkers)
    }
    unsafe fn roundedData(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roundedData)
    }
    unsafe fn setRoundedData_(&self, roundedData: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoundedData : roundedData)
    }
    unsafe fn centerSpaceSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerSpaceSize)
    }
    unsafe fn setCenterSpaceSize_(&self, centerSpaceSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterSpaceSize : centerSpaceSize)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCIRoundedRectangleGenerator: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn smoothness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smoothness)
    }
    unsafe fn setSmoothness_(&self, smoothness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmoothness : smoothness)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCIRoundedRectangleStrokeGenerator: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn smoothness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smoothness)
    }
    unsafe fn setSmoothness_(&self, smoothness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmoothness : smoothness)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIStarShineGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn crossScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crossScale)
    }
    unsafe fn setCrossScale_(&self, crossScale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrossScale : crossScale)
    }
    unsafe fn crossAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crossAngle)
    }
    unsafe fn setCrossAngle_(&self, crossAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrossAngle : crossAngle)
    }
    unsafe fn crossOpacity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crossOpacity)
    }
    unsafe fn setCrossOpacity_(&self, crossOpacity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrossOpacity : crossOpacity)
    }
    unsafe fn crossWidth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crossWidth)
    }
    unsafe fn setCrossWidth_(&self, crossWidth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrossWidth : crossWidth)
    }
    unsafe fn epsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, epsilon)
    }
    unsafe fn setEpsilon_(&self, epsilon: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEpsilon : epsilon)
    }
}
pub trait PCIStripesGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCISunbeamsGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn sunRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sunRadius)
    }
    unsafe fn setSunRadius_(&self, sunRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSunRadius : sunRadius)
    }
    unsafe fn maxStriationRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxStriationRadius)
    }
    unsafe fn setMaxStriationRadius_(&self, maxStriationRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxStriationRadius : maxStriationRadius)
    }
    unsafe fn striationStrength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationStrength)
    }
    unsafe fn setStriationStrength_(&self, striationStrength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationStrength : striationStrength)
    }
    unsafe fn striationContrast(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationContrast)
    }
    unsafe fn setStriationContrast_(&self, striationContrast: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationContrast : striationContrast)
    }
    unsafe fn time(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn setTime_(&self, time: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTime : time)
    }
}
pub trait PCITextImageGenerator: Sized + std::ops::Deref {
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn setText_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
    }
    unsafe fn fontName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontName)
    }
    unsafe fn setFontName_(&self, fontName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontName : fontName)
    }
    unsafe fn fontSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontSize)
    }
    unsafe fn setFontSize_(&self, fontSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontSize : fontSize)
    }
    unsafe fn scaleFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn setScaleFactor_(&self, scaleFactor: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleFactor : scaleFactor)
    }
    unsafe fn padding(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, padding)
    }
    unsafe fn setPadding_(&self, padding: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPadding : padding)
    }
}
pub trait PCIBlendWithMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn backgroundImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundImage)
    }
    unsafe fn setBackgroundImage_(&self, backgroundImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundImage : backgroundImage)
    }
    unsafe fn maskImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskImage)
    }
    unsafe fn setMaskImage_(&self, maskImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaskImage : maskImage)
    }
}
pub trait PCIBloom: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCICannyEdgeDetector: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn gaussianSigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaussianSigma)
    }
    unsafe fn setGaussianSigma_(&self, gaussianSigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaussianSigma : gaussianSigma)
    }
    unsafe fn perceptual(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perceptual)
    }
    unsafe fn setPerceptual_(&self, perceptual: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerceptual : perceptual)
    }
    unsafe fn thresholdHigh(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thresholdHigh)
    }
    unsafe fn setThresholdHigh_(&self, thresholdHigh: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThresholdHigh : thresholdHigh)
    }
    unsafe fn thresholdLow(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thresholdLow)
    }
    unsafe fn setThresholdLow_(&self, thresholdLow: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThresholdLow : thresholdLow)
    }
    unsafe fn hysteresisPasses(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hysteresisPasses)
    }
    unsafe fn setHysteresisPasses_(&self, hysteresisPasses: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHysteresisPasses : hysteresisPasses)
    }
}
pub trait PCIComicEffect: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIConvolution: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn weights(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weights)
    }
    unsafe fn setWeights_(&self, weights: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeights : weights)
    }
    unsafe fn bias(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bias)
    }
    unsafe fn setBias_(&self, bias: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBias : bias)
    }
}
pub trait PCICoreMLModel: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn model(&self) -> MLModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, model)
    }
    unsafe fn setModel_(&self, model: MLModel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModel : model)
    }
    unsafe fn headIndex(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headIndex)
    }
    unsafe fn setHeadIndex_(&self, headIndex: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeadIndex : headIndex)
    }
    unsafe fn softmaxNormalization(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, softmaxNormalization)
    }
    unsafe fn setSoftmaxNormalization_(&self, softmaxNormalization: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSoftmaxNormalization : softmaxNormalization)
    }
}
pub trait PCICrystallize: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
}
pub trait PCIDepthOfField: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
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
    unsafe fn unsharpMaskRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unsharpMaskRadius)
    }
    unsafe fn setUnsharpMaskRadius_(&self, unsharpMaskRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnsharpMaskRadius : unsharpMaskRadius)
    }
    unsafe fn unsharpMaskIntensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unsharpMaskIntensity)
    }
    unsafe fn setUnsharpMaskIntensity_(&self, unsharpMaskIntensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnsharpMaskIntensity : unsharpMaskIntensity)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIEdges: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIEdgeWork: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIGaborGradients: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIGloom: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIHeightFieldFromMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIHexagonalPixellate: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
pub trait PCIHighlightShadowAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn shadowAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowAmount)
    }
    unsafe fn setShadowAmount_(&self, shadowAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowAmount : shadowAmount)
    }
    unsafe fn highlightAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightAmount)
    }
    unsafe fn setHighlightAmount_(&self, highlightAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightAmount : highlightAmount)
    }
}
pub trait PCILineOverlay: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn NRNoiseLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, NRNoiseLevel)
    }
    unsafe fn setNRNoiseLevel_(&self, NRNoiseLevel: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNRNoiseLevel : NRNoiseLevel)
    }
    unsafe fn NRSharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, NRSharpness)
    }
    unsafe fn setNRSharpness_(&self, NRSharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNRSharpness : NRSharpness)
    }
    unsafe fn edgeIntensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgeIntensity)
    }
    unsafe fn setEdgeIntensity_(&self, edgeIntensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgeIntensity : edgeIntensity)
    }
    unsafe fn threshold(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threshold)
    }
    unsafe fn setThreshold_(&self, threshold: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreshold : threshold)
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
}
pub trait PCIMix: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn backgroundImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundImage)
    }
    unsafe fn setBackgroundImage_(&self, backgroundImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundImage : backgroundImage)
    }
    unsafe fn amount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
}
pub trait PCIPersonSegmentation: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn qualityLevel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, qualityLevel)
    }
    unsafe fn setQualityLevel_(&self, qualityLevel: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQualityLevel : qualityLevel)
    }
}
pub trait PCIPixellate: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
pub trait PCIPointillize: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
}
pub trait PCISaliencyMap: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIShadedMaterial: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn shadingImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadingImage)
    }
    unsafe fn setShadingImage_(&self, shadingImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadingImage : shadingImage)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
}
pub trait PCISobelGradients: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCISpotColor: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn centerColor1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerColor1)
    }
    unsafe fn setCenterColor1_(&self, centerColor1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterColor1 : centerColor1)
    }
    unsafe fn replacementColor1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replacementColor1)
    }
    unsafe fn setReplacementColor1_(&self, replacementColor1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReplacementColor1 : replacementColor1)
    }
    unsafe fn closeness1(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeness1)
    }
    unsafe fn setCloseness1_(&self, closeness1: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCloseness1 : closeness1)
    }
    unsafe fn contrast1(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast1)
    }
    unsafe fn setContrast1_(&self, contrast1: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast1 : contrast1)
    }
    unsafe fn centerColor2(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerColor2)
    }
    unsafe fn setCenterColor2_(&self, centerColor2: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterColor2 : centerColor2)
    }
    unsafe fn replacementColor2(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replacementColor2)
    }
    unsafe fn setReplacementColor2_(&self, replacementColor2: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReplacementColor2 : replacementColor2)
    }
    unsafe fn closeness2(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeness2)
    }
    unsafe fn setCloseness2_(&self, closeness2: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCloseness2 : closeness2)
    }
    unsafe fn contrast2(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast2)
    }
    unsafe fn setContrast2_(&self, contrast2: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast2 : contrast2)
    }
    unsafe fn centerColor3(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerColor3)
    }
    unsafe fn setCenterColor3_(&self, centerColor3: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterColor3 : centerColor3)
    }
    unsafe fn replacementColor3(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replacementColor3)
    }
    unsafe fn setReplacementColor3_(&self, replacementColor3: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReplacementColor3 : replacementColor3)
    }
    unsafe fn closeness3(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeness3)
    }
    unsafe fn setCloseness3_(&self, closeness3: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCloseness3 : closeness3)
    }
    unsafe fn contrast3(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast3)
    }
    unsafe fn setContrast3_(&self, contrast3: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast3 : contrast3)
    }
}
pub trait PCISpotLight: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn lightPosition(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightPosition)
    }
    unsafe fn setLightPosition_(&self, lightPosition: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightPosition : lightPosition)
    }
    unsafe fn lightPointsAt(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightPointsAt)
    }
    unsafe fn setLightPointsAt_(&self, lightPointsAt: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightPointsAt : lightPointsAt)
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
    unsafe fn concentration(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, concentration)
    }
    unsafe fn setConcentration_(&self, concentration: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConcentration : concentration)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCIBokehBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn ringAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringAmount)
    }
    unsafe fn setRingAmount_(&self, ringAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingAmount : ringAmount)
    }
    unsafe fn ringSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringSize)
    }
    unsafe fn setRingSize_(&self, ringSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingSize : ringSize)
    }
    unsafe fn softness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, softness)
    }
    unsafe fn setSoftness_(&self, softness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSoftness : softness)
    }
}
pub trait PCIBoxBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIDiscBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIGaussianBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMaskedVariableBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn mask(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mask)
    }
    unsafe fn setMask_(&self, mask: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMask : mask)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMedian: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIMorphologyGradient: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMorphologyMaximum: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMorphologyMinimum: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMorphologyRectangleMaximum: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
}
pub trait PCIMorphologyRectangleMinimum: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
}
pub trait PCIMotionBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCINoiseReduction: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn noiseLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noiseLevel)
    }
    unsafe fn setNoiseLevel_(&self, noiseLevel: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNoiseLevel : noiseLevel)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCIZoomBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn amount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
}
pub trait PCIAreaReductionFilter: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
}
pub trait PCIAreaAverage: Sized + std::ops::Deref {}
pub trait PCIAreaAverageMaximumRed: Sized + std::ops::Deref {}
pub trait PCIAreaBoundsRed: Sized + std::ops::Deref {}
pub trait PCIAreaHistogram: Sized + std::ops::Deref {
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn setCount_(&self, count: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCount : count)
    }
}
pub trait PCIAreaLogarithmicHistogram: Sized + std::ops::Deref {
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn setCount_(&self, count: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCount : count)
    }
    unsafe fn minimumStop(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumStop)
    }
    unsafe fn setMinimumStop_(&self, minimumStop: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumStop : minimumStop)
    }
    unsafe fn maximumStop(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumStop)
    }
    unsafe fn setMaximumStop_(&self, maximumStop: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumStop : maximumStop)
    }
}
pub trait PCIAreaMaximum: Sized + std::ops::Deref {}
pub trait PCIAreaMaximumAlpha: Sized + std::ops::Deref {}
pub trait PCIAreaMinimum: Sized + std::ops::Deref {}
pub trait PCIAreaMinimumAlpha: Sized + std::ops::Deref {}
pub trait PCIAreaMinMax: Sized + std::ops::Deref {}
pub trait PCIAreaMinMaxRed: Sized + std::ops::Deref {}
pub trait PCIColumnAverage: Sized + std::ops::Deref {}
pub trait PCIHistogramDisplay: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn height(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn highLimit(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highLimit)
    }
    unsafe fn setHighLimit_(&self, highLimit: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighLimit : highLimit)
    }
    unsafe fn lowLimit(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowLimit)
    }
    unsafe fn setLowLimit_(&self, lowLimit: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLowLimit : lowLimit)
    }
}
pub trait PCIKMeans: Sized + std::ops::Deref {
    unsafe fn inputMeans(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputMeans)
    }
    unsafe fn setInputMeans_(&self, inputMeans: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputMeans : inputMeans)
    }
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn setCount_(&self, count: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCount : count)
    }
    unsafe fn passes(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passes)
    }
    unsafe fn setPasses_(&self, passes: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPasses : passes)
    }
    unsafe fn perceptual(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perceptual)
    }
    unsafe fn setPerceptual_(&self, perceptual: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerceptual : perceptual)
    }
}
pub trait PCIRowAverage: Sized + std::ops::Deref {}
impl CIFilter_Builtins for CIFilter {}
pub trait CIFilter_Builtins: Sized + std::ops::Deref {
    unsafe fn distanceGradientFromRedMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), distanceGradientFromRedMaskFilter)
    }
    unsafe fn gaussianGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gaussianGradientFilter)
    }
    unsafe fn hueSaturationValueGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hueSaturationValueGradientFilter)
    }
    unsafe fn linearGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearGradientFilter)
    }
    unsafe fn radialGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), radialGradientFilter)
    }
    unsafe fn signedDistanceGradientFromRedMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), signedDistanceGradientFromRedMaskFilter)
    }
    unsafe fn smoothLinearGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), smoothLinearGradientFilter)
    }
    unsafe fn sharpenLuminanceFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sharpenLuminanceFilter)
    }
    unsafe fn unsharpMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), unsharpMaskFilter)
    }
    unsafe fn circularScreenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), circularScreenFilter)
    }
    unsafe fn CMYKHalftone() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), CMYKHalftone)
    }
    unsafe fn dotScreenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), dotScreenFilter)
    }
    unsafe fn hatchedScreenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hatchedScreenFilter)
    }
    unsafe fn lineScreenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lineScreenFilter)
    }
    unsafe fn bicubicScaleTransformFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bicubicScaleTransformFilter)
    }
    unsafe fn edgePreserveUpsampleFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), edgePreserveUpsampleFilter)
    }
    unsafe fn keystoneCorrectionCombinedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), keystoneCorrectionCombinedFilter)
    }
    unsafe fn keystoneCorrectionHorizontalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), keystoneCorrectionHorizontalFilter)
    }
    unsafe fn keystoneCorrectionVerticalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), keystoneCorrectionVerticalFilter)
    }
    unsafe fn lanczosScaleTransformFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lanczosScaleTransformFilter)
    }
    unsafe fn maximumScaleTransformFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maximumScaleTransformFilter)
    }
    unsafe fn perspectiveCorrectionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveCorrectionFilter)
    }
    unsafe fn perspectiveRotateFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveRotateFilter)
    }
    unsafe fn perspectiveTransformFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveTransformFilter)
    }
    unsafe fn perspectiveTransformWithExtentFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveTransformWithExtentFilter)
    }
    unsafe fn straightenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), straightenFilter)
    }
    unsafe fn accordionFoldTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), accordionFoldTransitionFilter)
    }
    unsafe fn barsSwipeTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), barsSwipeTransitionFilter)
    }
    unsafe fn copyMachineTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), copyMachineTransitionFilter)
    }
    unsafe fn disintegrateWithMaskTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), disintegrateWithMaskTransitionFilter)
    }
    unsafe fn dissolveTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), dissolveTransitionFilter)
    }
    unsafe fn flashTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), flashTransitionFilter)
    }
    unsafe fn modTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), modTransitionFilter)
    }
    unsafe fn pageCurlTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pageCurlTransitionFilter)
    }
    unsafe fn pageCurlWithShadowTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pageCurlWithShadowTransitionFilter)
    }
    unsafe fn rippleTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), rippleTransitionFilter)
    }
    unsafe fn swipeTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), swipeTransitionFilter)
    }
    unsafe fn additionCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), additionCompositingFilter)
    }
    unsafe fn colorBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorBlendModeFilter)
    }
    unsafe fn colorBurnBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorBurnBlendModeFilter)
    }
    unsafe fn colorDodgeBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorDodgeBlendModeFilter)
    }
    unsafe fn darkenBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), darkenBlendModeFilter)
    }
    unsafe fn differenceBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), differenceBlendModeFilter)
    }
    unsafe fn divideBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), divideBlendModeFilter)
    }
    unsafe fn exclusionBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), exclusionBlendModeFilter)
    }
    unsafe fn hardLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hardLightBlendModeFilter)
    }
    unsafe fn hueBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hueBlendModeFilter)
    }
    unsafe fn lightenBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lightenBlendModeFilter)
    }
    unsafe fn linearBurnBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearBurnBlendModeFilter)
    }
    unsafe fn linearDodgeBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearDodgeBlendModeFilter)
    }
    unsafe fn linearLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearLightBlendModeFilter)
    }
    unsafe fn luminosityBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), luminosityBlendModeFilter)
    }
    unsafe fn maximumCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maximumCompositingFilter)
    }
    unsafe fn minimumCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), minimumCompositingFilter)
    }
    unsafe fn multiplyBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), multiplyBlendModeFilter)
    }
    unsafe fn multiplyCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), multiplyCompositingFilter)
    }
    unsafe fn overlayBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), overlayBlendModeFilter)
    }
    unsafe fn pinLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pinLightBlendModeFilter)
    }
    unsafe fn saturationBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), saturationBlendModeFilter)
    }
    unsafe fn screenBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), screenBlendModeFilter)
    }
    unsafe fn softLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), softLightBlendModeFilter)
    }
    unsafe fn sourceAtopCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sourceAtopCompositingFilter)
    }
    unsafe fn sourceInCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sourceInCompositingFilter)
    }
    unsafe fn sourceOutCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sourceOutCompositingFilter)
    }
    unsafe fn sourceOverCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sourceOverCompositingFilter)
    }
    unsafe fn subtractBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), subtractBlendModeFilter)
    }
    unsafe fn vividLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vividLightBlendModeFilter)
    }
    unsafe fn colorAbsoluteDifferenceFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorAbsoluteDifferenceFilter)
    }
    unsafe fn colorClampFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorClampFilter)
    }
    unsafe fn colorControlsFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorControlsFilter)
    }
    unsafe fn colorMatrixFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorMatrixFilter)
    }
    unsafe fn colorPolynomialFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorPolynomialFilter)
    }
    unsafe fn colorThresholdFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorThresholdFilter)
    }
    unsafe fn colorThresholdOtsuFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorThresholdOtsuFilter)
    }
    unsafe fn depthToDisparityFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), depthToDisparityFilter)
    }
    unsafe fn disparityToDepthFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), disparityToDepthFilter)
    }
    unsafe fn exposureAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), exposureAdjustFilter)
    }
    unsafe fn gammaAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gammaAdjustFilter)
    }
    unsafe fn hueAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hueAdjustFilter)
    }
    unsafe fn linearToSRGBToneCurveFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearToSRGBToneCurveFilter)
    }
    unsafe fn sRGBToneCurveToLinearFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sRGBToneCurveToLinearFilter)
    }
    unsafe fn systemToneMapFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), systemToneMapFilter)
    }
    unsafe fn temperatureAndTintFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), temperatureAndTintFilter)
    }
    unsafe fn toneCurveFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), toneCurveFilter)
    }
    unsafe fn toneMapHeadroomFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), toneMapHeadroomFilter)
    }
    unsafe fn vibranceFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vibranceFilter)
    }
    unsafe fn whitePointAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), whitePointAdjustFilter)
    }
    unsafe fn colorCrossPolynomialFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCrossPolynomialFilter)
    }
    unsafe fn colorCubeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCubeFilter)
    }
    unsafe fn colorCubesMixedWithMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCubesMixedWithMaskFilter)
    }
    unsafe fn colorCubeWithColorSpaceFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCubeWithColorSpaceFilter)
    }
    unsafe fn colorCurvesFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCurvesFilter)
    }
    unsafe fn colorInvertFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorInvertFilter)
    }
    unsafe fn colorMapFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorMapFilter)
    }
    unsafe fn colorMonochromeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorMonochromeFilter)
    }
    unsafe fn colorPosterizeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorPosterizeFilter)
    }
    unsafe fn convertLabToRGBFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convertLabToRGBFilter)
    }
    unsafe fn convertRGBtoLabFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convertRGBtoLabFilter)
    }
    unsafe fn ditherFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), ditherFilter)
    }
    unsafe fn documentEnhancerFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), documentEnhancerFilter)
    }
    unsafe fn falseColorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), falseColorFilter)
    }
    unsafe fn LabDeltaE() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), LabDeltaE)
    }
    unsafe fn maskToAlphaFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maskToAlphaFilter)
    }
    unsafe fn maximumComponentFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maximumComponentFilter)
    }
    unsafe fn minimumComponentFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), minimumComponentFilter)
    }
    unsafe fn paletteCentroidFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), paletteCentroidFilter)
    }
    unsafe fn palettizeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), palettizeFilter)
    }
    unsafe fn photoEffectChromeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectChromeFilter)
    }
    unsafe fn photoEffectFadeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectFadeFilter)
    }
    unsafe fn photoEffectInstantFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectInstantFilter)
    }
    unsafe fn photoEffectMonoFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectMonoFilter)
    }
    unsafe fn photoEffectNoirFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectNoirFilter)
    }
    unsafe fn photoEffectProcessFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectProcessFilter)
    }
    unsafe fn photoEffectTonalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectTonalFilter)
    }
    unsafe fn photoEffectTransferFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectTransferFilter)
    }
    unsafe fn sepiaToneFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sepiaToneFilter)
    }
    unsafe fn thermalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), thermalFilter)
    }
    unsafe fn vignetteFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vignetteFilter)
    }
    unsafe fn vignetteEffectFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vignetteEffectFilter)
    }
    unsafe fn xRayFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), xRayFilter)
    }
    unsafe fn bumpDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bumpDistortionFilter)
    }
    unsafe fn bumpDistortionLinearFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bumpDistortionLinearFilter)
    }
    unsafe fn circleSplashDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), circleSplashDistortionFilter)
    }
    unsafe fn circularWrapFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), circularWrapFilter)
    }
    unsafe fn displacementDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), displacementDistortionFilter)
    }
    unsafe fn drosteFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), drosteFilter)
    }
    unsafe fn glassDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), glassDistortionFilter)
    }
    unsafe fn glassLozengeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), glassLozengeFilter)
    }
    unsafe fn holeDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), holeDistortionFilter)
    }
    unsafe fn lightTunnelFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lightTunnelFilter)
    }
    unsafe fn ninePartStretchedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), ninePartStretchedFilter)
    }
    unsafe fn ninePartTiledFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), ninePartTiledFilter)
    }
    unsafe fn pinchDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pinchDistortionFilter)
    }
    unsafe fn stretchCropFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), stretchCropFilter)
    }
    unsafe fn torusLensDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), torusLensDistortionFilter)
    }
    unsafe fn twirlDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), twirlDistortionFilter)
    }
    unsafe fn vortexDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vortexDistortionFilter)
    }
    unsafe fn affineClampFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), affineClampFilter)
    }
    unsafe fn affineTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), affineTileFilter)
    }
    unsafe fn eightfoldReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), eightfoldReflectedTileFilter)
    }
    unsafe fn fourfoldReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), fourfoldReflectedTileFilter)
    }
    unsafe fn fourfoldRotatedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), fourfoldRotatedTileFilter)
    }
    unsafe fn fourfoldTranslatedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), fourfoldTranslatedTileFilter)
    }
    unsafe fn glideReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), glideReflectedTileFilter)
    }
    unsafe fn kaleidoscopeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), kaleidoscopeFilter)
    }
    unsafe fn opTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), opTileFilter)
    }
    unsafe fn parallelogramTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), parallelogramTileFilter)
    }
    unsafe fn perspectiveTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveTileFilter)
    }
    unsafe fn sixfoldReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sixfoldReflectedTileFilter)
    }
    unsafe fn sixfoldRotatedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sixfoldRotatedTileFilter)
    }
    unsafe fn triangleKaleidoscopeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), triangleKaleidoscopeFilter)
    }
    unsafe fn triangleTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), triangleTileFilter)
    }
    unsafe fn twelvefoldReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), twelvefoldReflectedTileFilter)
    }
    unsafe fn attributedTextImageGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), attributedTextImageGeneratorFilter)
    }
    unsafe fn aztecCodeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), aztecCodeGeneratorFilter)
    }
    unsafe fn barcodeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), barcodeGeneratorFilter)
    }
    unsafe fn blurredRectangleGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blurredRectangleGeneratorFilter)
    }
    unsafe fn blurredRoundedRectangleGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blurredRoundedRectangleGeneratorFilter)
    }
    unsafe fn checkerboardGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), checkerboardGeneratorFilter)
    }
    unsafe fn code128BarcodeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), code128BarcodeGeneratorFilter)
    }
    unsafe fn lenticularHaloGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lenticularHaloGeneratorFilter)
    }
    unsafe fn meshGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), meshGeneratorFilter)
    }
    unsafe fn PDF417BarcodeGenerator() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), PDF417BarcodeGenerator)
    }
    unsafe fn QRCodeGenerator() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), QRCodeGenerator)
    }
    unsafe fn randomGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), randomGeneratorFilter)
    }
    unsafe fn roundedQRCodeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), roundedQRCodeGeneratorFilter)
    }
    unsafe fn roundedRectangleGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), roundedRectangleGeneratorFilter)
    }
    unsafe fn roundedRectangleStrokeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), roundedRectangleStrokeGeneratorFilter)
    }
    unsafe fn starShineGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), starShineGeneratorFilter)
    }
    unsafe fn stripesGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), stripesGeneratorFilter)
    }
    unsafe fn sunbeamsGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sunbeamsGeneratorFilter)
    }
    unsafe fn textImageGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), textImageGeneratorFilter)
    }
    unsafe fn blendWithAlphaMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blendWithAlphaMaskFilter)
    }
    unsafe fn blendWithBlueMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blendWithBlueMaskFilter)
    }
    unsafe fn blendWithMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blendWithMaskFilter)
    }
    unsafe fn blendWithRedMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blendWithRedMaskFilter)
    }
    unsafe fn bloomFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bloomFilter)
    }
    unsafe fn cannyEdgeDetectorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), cannyEdgeDetectorFilter)
    }
    unsafe fn comicEffectFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), comicEffectFilter)
    }
    unsafe fn convolution3X3Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution3X3Filter)
    }
    unsafe fn convolution5X5Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution5X5Filter)
    }
    unsafe fn convolution7X7Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution7X7Filter)
    }
    unsafe fn convolution9HorizontalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution9HorizontalFilter)
    }
    unsafe fn convolution9VerticalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution9VerticalFilter)
    }
    unsafe fn convolutionRGB3X3Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB3X3Filter)
    }
    unsafe fn convolutionRGB5X5Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB5X5Filter)
    }
    unsafe fn convolutionRGB7X7Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB7X7Filter)
    }
    unsafe fn convolutionRGB9HorizontalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB9HorizontalFilter)
    }
    unsafe fn convolutionRGB9VerticalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB9VerticalFilter)
    }
    unsafe fn coreMLModelFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), coreMLModelFilter)
    }
    unsafe fn crystallizeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), crystallizeFilter)
    }
    unsafe fn depthOfFieldFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), depthOfFieldFilter)
    }
    unsafe fn edgesFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), edgesFilter)
    }
    unsafe fn edgeWorkFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), edgeWorkFilter)
    }
    unsafe fn gaborGradientsFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gaborGradientsFilter)
    }
    unsafe fn gloomFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gloomFilter)
    }
    unsafe fn heightFieldFromMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), heightFieldFromMaskFilter)
    }
    unsafe fn hexagonalPixellateFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hexagonalPixellateFilter)
    }
    unsafe fn highlightShadowAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), highlightShadowAdjustFilter)
    }
    unsafe fn lineOverlayFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lineOverlayFilter)
    }
    unsafe fn mixFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), mixFilter)
    }
    unsafe fn personSegmentationFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), personSegmentationFilter)
    }
    unsafe fn pixellateFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pixellateFilter)
    }
    unsafe fn pointillizeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pointillizeFilter)
    }
    unsafe fn saliencyMapFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), saliencyMapFilter)
    }
    unsafe fn shadedMaterialFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), shadedMaterialFilter)
    }
    unsafe fn sobelGradientsFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sobelGradientsFilter)
    }
    unsafe fn spotColorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), spotColorFilter)
    }
    unsafe fn spotLightFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), spotLightFilter)
    }
    unsafe fn bokehBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bokehBlurFilter)
    }
    unsafe fn boxBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), boxBlurFilter)
    }
    unsafe fn discBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), discBlurFilter)
    }
    unsafe fn gaussianBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gaussianBlurFilter)
    }
    unsafe fn maskedVariableBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maskedVariableBlurFilter)
    }
    unsafe fn medianFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), medianFilter)
    }
    unsafe fn morphologyGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyGradientFilter)
    }
    unsafe fn morphologyMaximumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyMaximumFilter)
    }
    unsafe fn morphologyMinimumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyMinimumFilter)
    }
    unsafe fn morphologyRectangleMaximumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyRectangleMaximumFilter)
    }
    unsafe fn morphologyRectangleMinimumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyRectangleMinimumFilter)
    }
    unsafe fn motionBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), motionBlurFilter)
    }
    unsafe fn noiseReductionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), noiseReductionFilter)
    }
    unsafe fn zoomBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), zoomBlurFilter)
    }
    unsafe fn areaAlphaWeightedHistogramFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaAlphaWeightedHistogramFilter)
    }
    unsafe fn areaAverageFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaAverageFilter)
    }
    unsafe fn areaAverageMaximumRedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaAverageMaximumRedFilter)
    }
    unsafe fn areaBoundsRedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaBoundsRedFilter)
    }
    unsafe fn areaHistogramFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaHistogramFilter)
    }
    unsafe fn areaLogarithmicHistogramFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaLogarithmicHistogramFilter)
    }
    unsafe fn areaMaximumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMaximumFilter)
    }
    unsafe fn areaMaximumAlphaFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMaximumAlphaFilter)
    }
    unsafe fn areaMinimumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMinimumFilter)
    }
    unsafe fn areaMinimumAlphaFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMinimumAlphaFilter)
    }
    unsafe fn areaMinMaxFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMinMaxFilter)
    }
    unsafe fn areaMinMaxRedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMinMaxRedFilter)
    }
    unsafe fn columnAverageFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), columnAverageFilter)
    }
    unsafe fn histogramDisplayFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), histogramDisplayFilter)
    }
    unsafe fn KMeansFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), KMeansFilter)
    }
    unsafe fn rowAverageFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), rowAverageFilter)
    }
}
unsafe extern "C" {
    pub static kCIFormatARGB8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatBGRA8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBA8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBX8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatABGR8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBAh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBA16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBAf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBX16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBXh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBXf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGB10: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatA8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatA16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatAh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatAf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatR8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatR16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRG8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRG16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatL8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatL16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLA8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLA16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLAh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLAf: CIFormat;
}
unsafe extern "C" {
    pub static kCIImageColorSpace: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageApplyCleanAperture: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageToneMapHDRtoSDR: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageExpandToHDR: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageContentHeadroom: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageContentAverageLightLevel: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageNearestSampling: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageCacheImmediately: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageProperties: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageApplyOrientationProperty: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliaryDepth: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliaryDisparity: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliaryPortraitEffectsMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationSkinMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationHairMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationTeethMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationGlassesMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationSkyMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliaryHDRGainMap: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustEnhance: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustRedEye: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustFeatures: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustCrop: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustLevel: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIContextOutputColorSpace: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextWorkingColorSpace: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextWorkingFormat: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextHighQualityDownsample: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextOutputPremultiplied: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextCacheIntermediates: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextUseSoftwareRenderer: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextPriorityRequestLow: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextAllowLowPower: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextName: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextCVMetalTextureCache: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextMemoryLimit: CIContextOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationAVDepthData: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationDepthImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationDisparityImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationAVPortraitEffectsMatte: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationPortraitEffectsMatteImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationAVSemanticSegmentationMattes: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationSkinMatteImage:
        CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationHairMatteImage:
        CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationTeethMatteImage:
        CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationGlassesMatteImage:
        CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationSkyMatteImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationHDRImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationHDRGainMapImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationHDRGainMapAsRGB: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIAttributeFilterName: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeFilterDisplayName: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeDescription: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeFilterAvailable_Mac: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeFilterAvailable_iOS: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeReferenceDocumentation: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeFilterCategories: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeClass: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeType: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeMin: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeMax: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeSliderMin: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeSliderMax: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeDefault: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeIdentity: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeName: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeDisplayName: NSString;
}
unsafe extern "C" {
    pub static kCIUIParameterSet: NSString;
}
unsafe extern "C" {
    pub static kCIUISetBasic: NSString;
}
unsafe extern "C" {
    pub static kCIUISetIntermediate: NSString;
}
unsafe extern "C" {
    pub static kCIUISetAdvanced: NSString;
}
unsafe extern "C" {
    pub static kCIUISetDevelopment: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeTime: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeScalar: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeDistance: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeAngle: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeBoolean: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeInteger: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeCount: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypePosition: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeOffset: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypePosition3: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeRectangle: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeOpaqueColor: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeColor: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeGradient: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeImage: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeTransform: NSString;
}
unsafe extern "C" {
    pub static kCICategoryDistortionEffect: NSString;
}
unsafe extern "C" {
    pub static kCICategoryGeometryAdjustment: NSString;
}
unsafe extern "C" {
    pub static kCICategoryCompositeOperation: NSString;
}
unsafe extern "C" {
    pub static kCICategoryHalftoneEffect: NSString;
}
unsafe extern "C" {
    pub static kCICategoryColorAdjustment: NSString;
}
unsafe extern "C" {
    pub static kCICategoryColorEffect: NSString;
}
unsafe extern "C" {
    pub static kCICategoryTransition: NSString;
}
unsafe extern "C" {
    pub static kCICategoryTileEffect: NSString;
}
unsafe extern "C" {
    pub static kCICategoryGenerator: NSString;
}
unsafe extern "C" {
    pub static kCICategoryReduction: NSString;
}
unsafe extern "C" {
    pub static kCICategoryGradient: NSString;
}
unsafe extern "C" {
    pub static kCICategoryStylize: NSString;
}
unsafe extern "C" {
    pub static kCICategorySharpen: NSString;
}
unsafe extern "C" {
    pub static kCICategoryBlur: NSString;
}
unsafe extern "C" {
    pub static kCICategoryVideo: NSString;
}
unsafe extern "C" {
    pub static kCICategoryStillImage: NSString;
}
unsafe extern "C" {
    pub static kCICategoryInterlaced: NSString;
}
unsafe extern "C" {
    pub static kCICategoryNonSquarePixels: NSString;
}
unsafe extern "C" {
    pub static kCICategoryHighDynamicRange: NSString;
}
unsafe extern "C" {
    pub static kCICategoryBuiltIn: NSString;
}
unsafe extern "C" {
    pub static kCICategoryFilterGenerator: NSString;
}
unsafe extern "C" {
    pub static kCIOutputImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBackgroundImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputDepthImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputDisparityImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputAmountKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputCountKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputThresholdKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputTimeKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputTransformKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputScaleKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputAspectRatioKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputCenterKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputRadiusKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputRadius0Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputRadius1Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputAngleKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputRefractionKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputWidthKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputSharpnessKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputIntensityKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputEVKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputSaturationKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputColorKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputColor0Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputColor1Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputColorSpaceKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBrightnessKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputContrastKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputExtrapolateKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputPerceptualKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBiasKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBiasVectorKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputWeightsKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputGradientImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputMaskImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputMatteImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputShadingImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputTargetImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBacksideImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputPaletteImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputExtentKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputPoint0Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputPoint1Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputVersionKey: NSString;
}
unsafe extern "C" {
    pub static kCIDynamicRangeStandard: CIDynamicRangeOption;
}
unsafe extern "C" {
    pub static kCIDynamicRangeConstrainedHigh: CIDynamicRangeOption;
}
unsafe extern "C" {
    pub static kCIDynamicRangeHigh: CIDynamicRangeOption;
}
unsafe extern "C" {
    pub static CIDetectorTypeFace: NSString;
}
unsafe extern "C" {
    pub static CIDetectorTypeRectangle: NSString;
}
unsafe extern "C" {
    pub static CIDetectorTypeQRCode: NSString;
}
unsafe extern "C" {
    pub static CIDetectorTypeText: NSString;
}
unsafe extern "C" {
    pub static CIDetectorAccuracy: NSString;
}
unsafe extern "C" {
    pub static CIDetectorAccuracyLow: NSString;
}
unsafe extern "C" {
    pub static CIDetectorAccuracyHigh: NSString;
}
unsafe extern "C" {
    pub static CIDetectorTracking: NSString;
}
unsafe extern "C" {
    pub static CIDetectorMinFeatureSize: NSString;
}
unsafe extern "C" {
    pub static CIDetectorMaxFeatureCount: NSString;
}
unsafe extern "C" {
    pub static CIDetectorNumberOfAngles: NSString;
}
unsafe extern "C" {
    pub static CIDetectorImageOrientation: NSString;
}
unsafe extern "C" {
    pub static CIDetectorEyeBlink: NSString;
}
unsafe extern "C" {
    pub static CIDetectorSmile: NSString;
}
unsafe extern "C" {
    pub static CIDetectorFocalLength: NSString;
}
unsafe extern "C" {
    pub static CIDetectorAspectRatio: NSString;
}
unsafe extern "C" {
    pub static CIDetectorReturnSubFeatures: NSString;
}
unsafe extern "C" {
    pub static CIFeatureTypeFace: NSString;
}
unsafe extern "C" {
    pub static CIFeatureTypeRectangle: NSString;
}
unsafe extern "C" {
    pub static CIFeatureTypeQRCode: NSString;
}
unsafe extern "C" {
    pub static CIFeatureTypeText: NSString;
}
unsafe extern "C" {
    pub static kCIImageProviderTileSize: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageProviderUserInfo: CIImageOption;
}
unsafe extern "C" {
    pub static kCISamplerAffineMatrix: NSString;
}
unsafe extern "C" {
    pub static kCISamplerWrapMode: NSString;
}
unsafe extern "C" {
    pub static kCISamplerFilterMode: NSString;
}
unsafe extern "C" {
    pub static kCISamplerWrapBlack: NSString;
}
unsafe extern "C" {
    pub static kCISamplerWrapClamp: NSString;
}
unsafe extern "C" {
    pub static kCISamplerFilterNearest: NSString;
}
unsafe extern "C" {
    pub static kCISamplerFilterLinear: NSString;
}
unsafe extern "C" {
    pub static kCISamplerColorSpace: NSString;
}
unsafe extern "C" {
    pub static kCIInputAllowDraftModeKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputDecoderVersionKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCISupportedDecoderVersionsKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputBaselineExposureKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputBoostKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputBoostShadowAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputDisableGamutMapKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralChromaticityXKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralChromaticityYKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralTemperatureKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralTintKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralLocationKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputScaleFactorKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputIgnoreImageOrientationKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputImageOrientationKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputEnableSharpeningKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputEnableChromaticNoiseTrackingKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNoiseReductionAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputMoireAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputEnableVendorLensCorrectionKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputLuminanceNoiseReductionAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputColorNoiseReductionAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNoiseReductionSharpnessAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNoiseReductionContrastAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNoiseReductionDetailAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputEnableEDRModeKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputLocalToneMapAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputLinearSpaceFilter: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIOutputNativeSizeKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIActiveKeys: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIPropertiesKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersionNone: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion9: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion9DNG: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion8: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion8DNG: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion7: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion7DNG: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion6: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion6DNG: CIRAWDecoderVersion;
}

unsafe impl objc2::encode::RefEncode for _CGLPixelFormatObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CGLPixelFormatObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CGLPixelFormatObject", &[]);
}
unsafe impl objc2::encode::RefEncode for __GLIContextRec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __GLIContextRec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__GLIContextRec", &[]);
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
unsafe impl objc2::encode::RefEncode for CIVector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIVector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIColorKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIColorKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIWarpKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIWarpKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIBlendKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIBlendKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIDetector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIDetector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFaceFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFaceFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRectangleFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRectangleFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIQRCodeFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIQRCodeFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CITextFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CITextFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIImageProcessorKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIImageProcessorKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIImageAccumulator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIImageAccumulator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFilterShape {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFilterShape {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CISampler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CISampler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRAWFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRAWFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRenderDestination {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRenderDestination {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRenderInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRenderInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRenderTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRenderTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIBarcodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIBarcodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIQRCodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIQRCodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIAztecCodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIAztecCodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIPDF417CodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIPDF417CodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIDataMatrixCodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIDataMatrixCodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFilterGenerator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFilterGenerator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIPlugIn {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIPlugIn {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
