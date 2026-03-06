#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::OpenGL::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type cl_device_id = *mut _cl_device_id;
pub type IOSurfaceRef = *mut __IOSurface;
pub type cl_char = i8;
pub type cl_uchar = u8;
pub type cl_short = i16;
pub type cl_ushort = u16;
pub type cl_int = i32;
pub type cl_uint = u32;
pub type cl_long = i64;
pub type cl_ulong = u64;
pub type cl_half = u16;
pub type cl_float = f32;
pub type cl_double = f64;
pub type cl_GLuint = ::std::os::raw::c_uint;
pub type cl_GLint = ::std::os::raw::c_int;
pub type cl_GLenum = ::std::os::raw::c_uint;
#[repr(C)]
#[repr(align(2))]
#[derive(Copy, Clone)]
pub union cl_char2 {
    pub __bindgen_anon_1: cl_char2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_char2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_char2__bindgen_ty_3,
    pub s: [cl_char; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_char2__bindgen_ty_1 {
    pub x: cl_char,
    pub y: cl_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_char2__bindgen_ty_2 {
    pub s0: cl_char,
    pub s1: cl_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_char2__bindgen_ty_3 {
    pub lo: cl_char,
    pub hi: cl_char,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union cl_char4 {
    pub __bindgen_anon_1: cl_char4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_char4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_char4__bindgen_ty_3,
    pub s: [cl_char; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_char4__bindgen_ty_1 {
    pub x: cl_char,
    pub y: cl_char,
    pub z: cl_char,
    pub w: cl_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_char4__bindgen_ty_2 {
    pub s0: cl_char,
    pub s1: cl_char,
    pub s2: cl_char,
    pub s3: cl_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_char4__bindgen_ty_3 {
    pub lo: cl_char2,
    pub hi: cl_char2,
}
pub type cl_char3 = cl_char4;
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union cl_char8 {
    pub __bindgen_anon_1: cl_char8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_char8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_char8__bindgen_ty_3,
    pub s: [cl_char; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_char8__bindgen_ty_1 {
    pub x: cl_char,
    pub y: cl_char,
    pub z: cl_char,
    pub w: cl_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_char8__bindgen_ty_2 {
    pub s0: cl_char,
    pub s1: cl_char,
    pub s2: cl_char,
    pub s3: cl_char,
    pub s4: cl_char,
    pub s5: cl_char,
    pub s6: cl_char,
    pub s7: cl_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_char8__bindgen_ty_3 {
    pub lo: cl_char4,
    pub hi: cl_char4,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_char16 {
    pub __bindgen_anon_1: cl_char16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_char16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_char16__bindgen_ty_3,
    pub s: [cl_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_char16__bindgen_ty_1 {
    pub x: cl_char,
    pub y: cl_char,
    pub z: cl_char,
    pub w: cl_char,
    pub __spacer4: cl_char,
    pub __spacer5: cl_char,
    pub __spacer6: cl_char,
    pub __spacer7: cl_char,
    pub __spacer8: cl_char,
    pub __spacer9: cl_char,
    pub sa: cl_char,
    pub sb: cl_char,
    pub sc: cl_char,
    pub sd: cl_char,
    pub se: cl_char,
    pub sf: cl_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_char16__bindgen_ty_2 {
    pub s0: cl_char,
    pub s1: cl_char,
    pub s2: cl_char,
    pub s3: cl_char,
    pub s4: cl_char,
    pub s5: cl_char,
    pub s6: cl_char,
    pub s7: cl_char,
    pub s8: cl_char,
    pub s9: cl_char,
    pub sA: cl_char,
    pub sB: cl_char,
    pub sC: cl_char,
    pub sD: cl_char,
    pub sE: cl_char,
    pub sF: cl_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_char16__bindgen_ty_3 {
    pub lo: cl_char8,
    pub hi: cl_char8,
}
#[repr(C)]
#[repr(align(2))]
#[derive(Copy, Clone)]
pub union cl_uchar2 {
    pub __bindgen_anon_1: cl_uchar2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_uchar2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_uchar2__bindgen_ty_3,
    pub s: [cl_uchar; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uchar2__bindgen_ty_1 {
    pub x: cl_uchar,
    pub y: cl_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uchar2__bindgen_ty_2 {
    pub s0: cl_uchar,
    pub s1: cl_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uchar2__bindgen_ty_3 {
    pub lo: cl_uchar,
    pub hi: cl_uchar,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union cl_uchar4 {
    pub __bindgen_anon_1: cl_uchar4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_uchar4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_uchar4__bindgen_ty_3,
    pub s: [cl_uchar; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uchar4__bindgen_ty_1 {
    pub x: cl_uchar,
    pub y: cl_uchar,
    pub z: cl_uchar,
    pub w: cl_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uchar4__bindgen_ty_2 {
    pub s0: cl_uchar,
    pub s1: cl_uchar,
    pub s2: cl_uchar,
    pub s3: cl_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_uchar4__bindgen_ty_3 {
    pub lo: cl_uchar2,
    pub hi: cl_uchar2,
}
pub type cl_uchar3 = cl_uchar4;
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union cl_uchar8 {
    pub __bindgen_anon_1: cl_uchar8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_uchar8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_uchar8__bindgen_ty_3,
    pub s: [cl_uchar; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uchar8__bindgen_ty_1 {
    pub x: cl_uchar,
    pub y: cl_uchar,
    pub z: cl_uchar,
    pub w: cl_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uchar8__bindgen_ty_2 {
    pub s0: cl_uchar,
    pub s1: cl_uchar,
    pub s2: cl_uchar,
    pub s3: cl_uchar,
    pub s4: cl_uchar,
    pub s5: cl_uchar,
    pub s6: cl_uchar,
    pub s7: cl_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_uchar8__bindgen_ty_3 {
    pub lo: cl_uchar4,
    pub hi: cl_uchar4,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_uchar16 {
    pub __bindgen_anon_1: cl_uchar16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_uchar16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_uchar16__bindgen_ty_3,
    pub s: [cl_uchar; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uchar16__bindgen_ty_1 {
    pub x: cl_uchar,
    pub y: cl_uchar,
    pub z: cl_uchar,
    pub w: cl_uchar,
    pub __spacer4: cl_uchar,
    pub __spacer5: cl_uchar,
    pub __spacer6: cl_uchar,
    pub __spacer7: cl_uchar,
    pub __spacer8: cl_uchar,
    pub __spacer9: cl_uchar,
    pub sa: cl_uchar,
    pub sb: cl_uchar,
    pub sc: cl_uchar,
    pub sd: cl_uchar,
    pub se: cl_uchar,
    pub sf: cl_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uchar16__bindgen_ty_2 {
    pub s0: cl_uchar,
    pub s1: cl_uchar,
    pub s2: cl_uchar,
    pub s3: cl_uchar,
    pub s4: cl_uchar,
    pub s5: cl_uchar,
    pub s6: cl_uchar,
    pub s7: cl_uchar,
    pub s8: cl_uchar,
    pub s9: cl_uchar,
    pub sA: cl_uchar,
    pub sB: cl_uchar,
    pub sC: cl_uchar,
    pub sD: cl_uchar,
    pub sE: cl_uchar,
    pub sF: cl_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_uchar16__bindgen_ty_3 {
    pub lo: cl_uchar8,
    pub hi: cl_uchar8,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union cl_short2 {
    pub __bindgen_anon_1: cl_short2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_short2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_short2__bindgen_ty_3,
    pub s: [cl_short; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_short2__bindgen_ty_1 {
    pub x: cl_short,
    pub y: cl_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_short2__bindgen_ty_2 {
    pub s0: cl_short,
    pub s1: cl_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_short2__bindgen_ty_3 {
    pub lo: cl_short,
    pub hi: cl_short,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union cl_short4 {
    pub __bindgen_anon_1: cl_short4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_short4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_short4__bindgen_ty_3,
    pub s: [cl_short; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_short4__bindgen_ty_1 {
    pub x: cl_short,
    pub y: cl_short,
    pub z: cl_short,
    pub w: cl_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_short4__bindgen_ty_2 {
    pub s0: cl_short,
    pub s1: cl_short,
    pub s2: cl_short,
    pub s3: cl_short,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_short4__bindgen_ty_3 {
    pub lo: cl_short2,
    pub hi: cl_short2,
}
pub type cl_short3 = cl_short4;
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_short8 {
    pub __bindgen_anon_1: cl_short8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_short8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_short8__bindgen_ty_3,
    pub s: [cl_short; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_short8__bindgen_ty_1 {
    pub x: cl_short,
    pub y: cl_short,
    pub z: cl_short,
    pub w: cl_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_short8__bindgen_ty_2 {
    pub s0: cl_short,
    pub s1: cl_short,
    pub s2: cl_short,
    pub s3: cl_short,
    pub s4: cl_short,
    pub s5: cl_short,
    pub s6: cl_short,
    pub s7: cl_short,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_short8__bindgen_ty_3 {
    pub lo: cl_short4,
    pub hi: cl_short4,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub union cl_short16 {
    pub __bindgen_anon_1: cl_short16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_short16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_short16__bindgen_ty_3,
    pub s: [cl_short; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_short16__bindgen_ty_1 {
    pub x: cl_short,
    pub y: cl_short,
    pub z: cl_short,
    pub w: cl_short,
    pub __spacer4: cl_short,
    pub __spacer5: cl_short,
    pub __spacer6: cl_short,
    pub __spacer7: cl_short,
    pub __spacer8: cl_short,
    pub __spacer9: cl_short,
    pub sa: cl_short,
    pub sb: cl_short,
    pub sc: cl_short,
    pub sd: cl_short,
    pub se: cl_short,
    pub sf: cl_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_short16__bindgen_ty_2 {
    pub s0: cl_short,
    pub s1: cl_short,
    pub s2: cl_short,
    pub s3: cl_short,
    pub s4: cl_short,
    pub s5: cl_short,
    pub s6: cl_short,
    pub s7: cl_short,
    pub s8: cl_short,
    pub s9: cl_short,
    pub sA: cl_short,
    pub sB: cl_short,
    pub sC: cl_short,
    pub sD: cl_short,
    pub sE: cl_short,
    pub sF: cl_short,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct cl_short16__bindgen_ty_3 {
    pub lo: cl_short8,
    pub hi: cl_short8,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub union cl_ushort2 {
    pub __bindgen_anon_1: cl_ushort2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_ushort2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_ushort2__bindgen_ty_3,
    pub s: [cl_ushort; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ushort2__bindgen_ty_1 {
    pub x: cl_ushort,
    pub y: cl_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ushort2__bindgen_ty_2 {
    pub s0: cl_ushort,
    pub s1: cl_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ushort2__bindgen_ty_3 {
    pub lo: cl_ushort,
    pub hi: cl_ushort,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union cl_ushort4 {
    pub __bindgen_anon_1: cl_ushort4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_ushort4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_ushort4__bindgen_ty_3,
    pub s: [cl_ushort; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ushort4__bindgen_ty_1 {
    pub x: cl_ushort,
    pub y: cl_ushort,
    pub z: cl_ushort,
    pub w: cl_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ushort4__bindgen_ty_2 {
    pub s0: cl_ushort,
    pub s1: cl_ushort,
    pub s2: cl_ushort,
    pub s3: cl_ushort,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_ushort4__bindgen_ty_3 {
    pub lo: cl_ushort2,
    pub hi: cl_ushort2,
}
pub type cl_ushort3 = cl_ushort4;
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_ushort8 {
    pub __bindgen_anon_1: cl_ushort8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_ushort8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_ushort8__bindgen_ty_3,
    pub s: [cl_ushort; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ushort8__bindgen_ty_1 {
    pub x: cl_ushort,
    pub y: cl_ushort,
    pub z: cl_ushort,
    pub w: cl_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ushort8__bindgen_ty_2 {
    pub s0: cl_ushort,
    pub s1: cl_ushort,
    pub s2: cl_ushort,
    pub s3: cl_ushort,
    pub s4: cl_ushort,
    pub s5: cl_ushort,
    pub s6: cl_ushort,
    pub s7: cl_ushort,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_ushort8__bindgen_ty_3 {
    pub lo: cl_ushort4,
    pub hi: cl_ushort4,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub union cl_ushort16 {
    pub __bindgen_anon_1: cl_ushort16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_ushort16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_ushort16__bindgen_ty_3,
    pub s: [cl_ushort; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ushort16__bindgen_ty_1 {
    pub x: cl_ushort,
    pub y: cl_ushort,
    pub z: cl_ushort,
    pub w: cl_ushort,
    pub __spacer4: cl_ushort,
    pub __spacer5: cl_ushort,
    pub __spacer6: cl_ushort,
    pub __spacer7: cl_ushort,
    pub __spacer8: cl_ushort,
    pub __spacer9: cl_ushort,
    pub sa: cl_ushort,
    pub sb: cl_ushort,
    pub sc: cl_ushort,
    pub sd: cl_ushort,
    pub se: cl_ushort,
    pub sf: cl_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ushort16__bindgen_ty_2 {
    pub s0: cl_ushort,
    pub s1: cl_ushort,
    pub s2: cl_ushort,
    pub s3: cl_ushort,
    pub s4: cl_ushort,
    pub s5: cl_ushort,
    pub s6: cl_ushort,
    pub s7: cl_ushort,
    pub s8: cl_ushort,
    pub s9: cl_ushort,
    pub sA: cl_ushort,
    pub sB: cl_ushort,
    pub sC: cl_ushort,
    pub sD: cl_ushort,
    pub sE: cl_ushort,
    pub sF: cl_ushort,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct cl_ushort16__bindgen_ty_3 {
    pub lo: cl_ushort8,
    pub hi: cl_ushort8,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union cl_int2 {
    pub __bindgen_anon_1: cl_int2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_int2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_int2__bindgen_ty_3,
    pub s: [cl_int; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_int2__bindgen_ty_1 {
    pub x: cl_int,
    pub y: cl_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_int2__bindgen_ty_2 {
    pub s0: cl_int,
    pub s1: cl_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_int2__bindgen_ty_3 {
    pub lo: cl_int,
    pub hi: cl_int,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_int4 {
    pub __bindgen_anon_1: cl_int4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_int4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_int4__bindgen_ty_3,
    pub s: [cl_int; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_int4__bindgen_ty_1 {
    pub x: cl_int,
    pub y: cl_int,
    pub z: cl_int,
    pub w: cl_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_int4__bindgen_ty_2 {
    pub s0: cl_int,
    pub s1: cl_int,
    pub s2: cl_int,
    pub s3: cl_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_int4__bindgen_ty_3 {
    pub lo: cl_int2,
    pub hi: cl_int2,
}
pub type cl_int3 = cl_int4;
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub union cl_int8 {
    pub __bindgen_anon_1: cl_int8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_int8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_int8__bindgen_ty_3,
    pub s: [cl_int; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_int8__bindgen_ty_1 {
    pub x: cl_int,
    pub y: cl_int,
    pub z: cl_int,
    pub w: cl_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_int8__bindgen_ty_2 {
    pub s0: cl_int,
    pub s1: cl_int,
    pub s2: cl_int,
    pub s3: cl_int,
    pub s4: cl_int,
    pub s5: cl_int,
    pub s6: cl_int,
    pub s7: cl_int,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct cl_int8__bindgen_ty_3 {
    pub lo: cl_int4,
    pub hi: cl_int4,
}
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub union cl_int16 {
    pub __bindgen_anon_1: cl_int16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_int16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_int16__bindgen_ty_3,
    pub s: [cl_int; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_int16__bindgen_ty_1 {
    pub x: cl_int,
    pub y: cl_int,
    pub z: cl_int,
    pub w: cl_int,
    pub __spacer4: cl_int,
    pub __spacer5: cl_int,
    pub __spacer6: cl_int,
    pub __spacer7: cl_int,
    pub __spacer8: cl_int,
    pub __spacer9: cl_int,
    pub sa: cl_int,
    pub sb: cl_int,
    pub sc: cl_int,
    pub sd: cl_int,
    pub se: cl_int,
    pub sf: cl_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_int16__bindgen_ty_2 {
    pub s0: cl_int,
    pub s1: cl_int,
    pub s2: cl_int,
    pub s3: cl_int,
    pub s4: cl_int,
    pub s5: cl_int,
    pub s6: cl_int,
    pub s7: cl_int,
    pub s8: cl_int,
    pub s9: cl_int,
    pub sA: cl_int,
    pub sB: cl_int,
    pub sC: cl_int,
    pub sD: cl_int,
    pub sE: cl_int,
    pub sF: cl_int,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub struct cl_int16__bindgen_ty_3 {
    pub lo: cl_int8,
    pub hi: cl_int8,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union cl_uint2 {
    pub __bindgen_anon_1: cl_uint2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_uint2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_uint2__bindgen_ty_3,
    pub s: [cl_uint; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uint2__bindgen_ty_1 {
    pub x: cl_uint,
    pub y: cl_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uint2__bindgen_ty_2 {
    pub s0: cl_uint,
    pub s1: cl_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uint2__bindgen_ty_3 {
    pub lo: cl_uint,
    pub hi: cl_uint,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_uint4 {
    pub __bindgen_anon_1: cl_uint4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_uint4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_uint4__bindgen_ty_3,
    pub s: [cl_uint; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uint4__bindgen_ty_1 {
    pub x: cl_uint,
    pub y: cl_uint,
    pub z: cl_uint,
    pub w: cl_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uint4__bindgen_ty_2 {
    pub s0: cl_uint,
    pub s1: cl_uint,
    pub s2: cl_uint,
    pub s3: cl_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_uint4__bindgen_ty_3 {
    pub lo: cl_uint2,
    pub hi: cl_uint2,
}
pub type cl_uint3 = cl_uint4;
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub union cl_uint8 {
    pub __bindgen_anon_1: cl_uint8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_uint8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_uint8__bindgen_ty_3,
    pub s: [cl_uint; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uint8__bindgen_ty_1 {
    pub x: cl_uint,
    pub y: cl_uint,
    pub z: cl_uint,
    pub w: cl_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uint8__bindgen_ty_2 {
    pub s0: cl_uint,
    pub s1: cl_uint,
    pub s2: cl_uint,
    pub s3: cl_uint,
    pub s4: cl_uint,
    pub s5: cl_uint,
    pub s6: cl_uint,
    pub s7: cl_uint,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct cl_uint8__bindgen_ty_3 {
    pub lo: cl_uint4,
    pub hi: cl_uint4,
}
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub union cl_uint16 {
    pub __bindgen_anon_1: cl_uint16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_uint16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_uint16__bindgen_ty_3,
    pub s: [cl_uint; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uint16__bindgen_ty_1 {
    pub x: cl_uint,
    pub y: cl_uint,
    pub z: cl_uint,
    pub w: cl_uint,
    pub __spacer4: cl_uint,
    pub __spacer5: cl_uint,
    pub __spacer6: cl_uint,
    pub __spacer7: cl_uint,
    pub __spacer8: cl_uint,
    pub __spacer9: cl_uint,
    pub sa: cl_uint,
    pub sb: cl_uint,
    pub sc: cl_uint,
    pub sd: cl_uint,
    pub se: cl_uint,
    pub sf: cl_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_uint16__bindgen_ty_2 {
    pub s0: cl_uint,
    pub s1: cl_uint,
    pub s2: cl_uint,
    pub s3: cl_uint,
    pub s4: cl_uint,
    pub s5: cl_uint,
    pub s6: cl_uint,
    pub s7: cl_uint,
    pub s8: cl_uint,
    pub s9: cl_uint,
    pub sA: cl_uint,
    pub sB: cl_uint,
    pub sC: cl_uint,
    pub sD: cl_uint,
    pub sE: cl_uint,
    pub sF: cl_uint,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub struct cl_uint16__bindgen_ty_3 {
    pub lo: cl_uint8,
    pub hi: cl_uint8,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_long2 {
    pub __bindgen_anon_1: cl_long2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_long2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_long2__bindgen_ty_3,
    pub s: [cl_long; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_long2__bindgen_ty_1 {
    pub x: cl_long,
    pub y: cl_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_long2__bindgen_ty_2 {
    pub s0: cl_long,
    pub s1: cl_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_long2__bindgen_ty_3 {
    pub lo: cl_long,
    pub hi: cl_long,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub union cl_long4 {
    pub __bindgen_anon_1: cl_long4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_long4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_long4__bindgen_ty_3,
    pub s: [cl_long; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_long4__bindgen_ty_1 {
    pub x: cl_long,
    pub y: cl_long,
    pub z: cl_long,
    pub w: cl_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_long4__bindgen_ty_2 {
    pub s0: cl_long,
    pub s1: cl_long,
    pub s2: cl_long,
    pub s3: cl_long,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct cl_long4__bindgen_ty_3 {
    pub lo: cl_long2,
    pub hi: cl_long2,
}
pub type cl_long3 = cl_long4;
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub union cl_long8 {
    pub __bindgen_anon_1: cl_long8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_long8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_long8__bindgen_ty_3,
    pub s: [cl_long; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_long8__bindgen_ty_1 {
    pub x: cl_long,
    pub y: cl_long,
    pub z: cl_long,
    pub w: cl_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_long8__bindgen_ty_2 {
    pub s0: cl_long,
    pub s1: cl_long,
    pub s2: cl_long,
    pub s3: cl_long,
    pub s4: cl_long,
    pub s5: cl_long,
    pub s6: cl_long,
    pub s7: cl_long,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub struct cl_long8__bindgen_ty_3 {
    pub lo: cl_long4,
    pub hi: cl_long4,
}
#[repr(C)]
#[repr(align(128))]
#[derive(Copy, Clone)]
pub union cl_long16 {
    pub __bindgen_anon_1: cl_long16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_long16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_long16__bindgen_ty_3,
    pub s: [cl_long; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_long16__bindgen_ty_1 {
    pub x: cl_long,
    pub y: cl_long,
    pub z: cl_long,
    pub w: cl_long,
    pub __spacer4: cl_long,
    pub __spacer5: cl_long,
    pub __spacer6: cl_long,
    pub __spacer7: cl_long,
    pub __spacer8: cl_long,
    pub __spacer9: cl_long,
    pub sa: cl_long,
    pub sb: cl_long,
    pub sc: cl_long,
    pub sd: cl_long,
    pub se: cl_long,
    pub sf: cl_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_long16__bindgen_ty_2 {
    pub s0: cl_long,
    pub s1: cl_long,
    pub s2: cl_long,
    pub s3: cl_long,
    pub s4: cl_long,
    pub s5: cl_long,
    pub s6: cl_long,
    pub s7: cl_long,
    pub s8: cl_long,
    pub s9: cl_long,
    pub sA: cl_long,
    pub sB: cl_long,
    pub sC: cl_long,
    pub sD: cl_long,
    pub sE: cl_long,
    pub sF: cl_long,
}
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct cl_long16__bindgen_ty_3 {
    pub lo: cl_long8,
    pub hi: cl_long8,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_ulong2 {
    pub __bindgen_anon_1: cl_ulong2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_ulong2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_ulong2__bindgen_ty_3,
    pub s: [cl_ulong; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ulong2__bindgen_ty_1 {
    pub x: cl_ulong,
    pub y: cl_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ulong2__bindgen_ty_2 {
    pub s0: cl_ulong,
    pub s1: cl_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ulong2__bindgen_ty_3 {
    pub lo: cl_ulong,
    pub hi: cl_ulong,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub union cl_ulong4 {
    pub __bindgen_anon_1: cl_ulong4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_ulong4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_ulong4__bindgen_ty_3,
    pub s: [cl_ulong; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ulong4__bindgen_ty_1 {
    pub x: cl_ulong,
    pub y: cl_ulong,
    pub z: cl_ulong,
    pub w: cl_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ulong4__bindgen_ty_2 {
    pub s0: cl_ulong,
    pub s1: cl_ulong,
    pub s2: cl_ulong,
    pub s3: cl_ulong,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct cl_ulong4__bindgen_ty_3 {
    pub lo: cl_ulong2,
    pub hi: cl_ulong2,
}
pub type cl_ulong3 = cl_ulong4;
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub union cl_ulong8 {
    pub __bindgen_anon_1: cl_ulong8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_ulong8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_ulong8__bindgen_ty_3,
    pub s: [cl_ulong; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ulong8__bindgen_ty_1 {
    pub x: cl_ulong,
    pub y: cl_ulong,
    pub z: cl_ulong,
    pub w: cl_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ulong8__bindgen_ty_2 {
    pub s0: cl_ulong,
    pub s1: cl_ulong,
    pub s2: cl_ulong,
    pub s3: cl_ulong,
    pub s4: cl_ulong,
    pub s5: cl_ulong,
    pub s6: cl_ulong,
    pub s7: cl_ulong,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub struct cl_ulong8__bindgen_ty_3 {
    pub lo: cl_ulong4,
    pub hi: cl_ulong4,
}
#[repr(C)]
#[repr(align(128))]
#[derive(Copy, Clone)]
pub union cl_ulong16 {
    pub __bindgen_anon_1: cl_ulong16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_ulong16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_ulong16__bindgen_ty_3,
    pub s: [cl_ulong; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ulong16__bindgen_ty_1 {
    pub x: cl_ulong,
    pub y: cl_ulong,
    pub z: cl_ulong,
    pub w: cl_ulong,
    pub __spacer4: cl_ulong,
    pub __spacer5: cl_ulong,
    pub __spacer6: cl_ulong,
    pub __spacer7: cl_ulong,
    pub __spacer8: cl_ulong,
    pub __spacer9: cl_ulong,
    pub sa: cl_ulong,
    pub sb: cl_ulong,
    pub sc: cl_ulong,
    pub sd: cl_ulong,
    pub se: cl_ulong,
    pub sf: cl_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_ulong16__bindgen_ty_2 {
    pub s0: cl_ulong,
    pub s1: cl_ulong,
    pub s2: cl_ulong,
    pub s3: cl_ulong,
    pub s4: cl_ulong,
    pub s5: cl_ulong,
    pub s6: cl_ulong,
    pub s7: cl_ulong,
    pub s8: cl_ulong,
    pub s9: cl_ulong,
    pub sA: cl_ulong,
    pub sB: cl_ulong,
    pub sC: cl_ulong,
    pub sD: cl_ulong,
    pub sE: cl_ulong,
    pub sF: cl_ulong,
}
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct cl_ulong16__bindgen_ty_3 {
    pub lo: cl_ulong8,
    pub hi: cl_ulong8,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub union cl_float2 {
    pub __bindgen_anon_1: cl_float2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_float2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_float2__bindgen_ty_3,
    pub s: [cl_float; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_float2__bindgen_ty_1 {
    pub x: cl_float,
    pub y: cl_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_float2__bindgen_ty_2 {
    pub s0: cl_float,
    pub s1: cl_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_float2__bindgen_ty_3 {
    pub lo: cl_float,
    pub hi: cl_float,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_float4 {
    pub __bindgen_anon_1: cl_float4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_float4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_float4__bindgen_ty_3,
    pub s: [cl_float; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_float4__bindgen_ty_1 {
    pub x: cl_float,
    pub y: cl_float,
    pub z: cl_float,
    pub w: cl_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_float4__bindgen_ty_2 {
    pub s0: cl_float,
    pub s1: cl_float,
    pub s2: cl_float,
    pub s3: cl_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cl_float4__bindgen_ty_3 {
    pub lo: cl_float2,
    pub hi: cl_float2,
}
pub type cl_float3 = cl_float4;
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub union cl_float8 {
    pub __bindgen_anon_1: cl_float8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_float8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_float8__bindgen_ty_3,
    pub s: [cl_float; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_float8__bindgen_ty_1 {
    pub x: cl_float,
    pub y: cl_float,
    pub z: cl_float,
    pub w: cl_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_float8__bindgen_ty_2 {
    pub s0: cl_float,
    pub s1: cl_float,
    pub s2: cl_float,
    pub s3: cl_float,
    pub s4: cl_float,
    pub s5: cl_float,
    pub s6: cl_float,
    pub s7: cl_float,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct cl_float8__bindgen_ty_3 {
    pub lo: cl_float4,
    pub hi: cl_float4,
}
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub union cl_float16 {
    pub __bindgen_anon_1: cl_float16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_float16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_float16__bindgen_ty_3,
    pub s: [cl_float; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_float16__bindgen_ty_1 {
    pub x: cl_float,
    pub y: cl_float,
    pub z: cl_float,
    pub w: cl_float,
    pub __spacer4: cl_float,
    pub __spacer5: cl_float,
    pub __spacer6: cl_float,
    pub __spacer7: cl_float,
    pub __spacer8: cl_float,
    pub __spacer9: cl_float,
    pub sa: cl_float,
    pub sb: cl_float,
    pub sc: cl_float,
    pub sd: cl_float,
    pub se: cl_float,
    pub sf: cl_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_float16__bindgen_ty_2 {
    pub s0: cl_float,
    pub s1: cl_float,
    pub s2: cl_float,
    pub s3: cl_float,
    pub s4: cl_float,
    pub s5: cl_float,
    pub s6: cl_float,
    pub s7: cl_float,
    pub s8: cl_float,
    pub s9: cl_float,
    pub sA: cl_float,
    pub sB: cl_float,
    pub sC: cl_float,
    pub sD: cl_float,
    pub sE: cl_float,
    pub sF: cl_float,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub struct cl_float16__bindgen_ty_3 {
    pub lo: cl_float8,
    pub hi: cl_float8,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union cl_double2 {
    pub __bindgen_anon_1: cl_double2__bindgen_ty_1,
    pub __bindgen_anon_2: cl_double2__bindgen_ty_2,
    pub __bindgen_anon_3: cl_double2__bindgen_ty_3,
    pub s: [cl_double; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_double2__bindgen_ty_1 {
    pub x: cl_double,
    pub y: cl_double,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_double2__bindgen_ty_2 {
    pub s0: cl_double,
    pub s1: cl_double,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_double2__bindgen_ty_3 {
    pub lo: cl_double,
    pub hi: cl_double,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub union cl_double4 {
    pub __bindgen_anon_1: cl_double4__bindgen_ty_1,
    pub __bindgen_anon_2: cl_double4__bindgen_ty_2,
    pub __bindgen_anon_3: cl_double4__bindgen_ty_3,
    pub s: [cl_double; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_double4__bindgen_ty_1 {
    pub x: cl_double,
    pub y: cl_double,
    pub z: cl_double,
    pub w: cl_double,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_double4__bindgen_ty_2 {
    pub s0: cl_double,
    pub s1: cl_double,
    pub s2: cl_double,
    pub s3: cl_double,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct cl_double4__bindgen_ty_3 {
    pub lo: cl_double2,
    pub hi: cl_double2,
}
pub type cl_double3 = cl_double4;
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub union cl_double8 {
    pub __bindgen_anon_1: cl_double8__bindgen_ty_1,
    pub __bindgen_anon_2: cl_double8__bindgen_ty_2,
    pub __bindgen_anon_3: cl_double8__bindgen_ty_3,
    pub s: [cl_double; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_double8__bindgen_ty_1 {
    pub x: cl_double,
    pub y: cl_double,
    pub z: cl_double,
    pub w: cl_double,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_double8__bindgen_ty_2 {
    pub s0: cl_double,
    pub s1: cl_double,
    pub s2: cl_double,
    pub s3: cl_double,
    pub s4: cl_double,
    pub s5: cl_double,
    pub s6: cl_double,
    pub s7: cl_double,
}
#[repr(C)]
#[repr(align(32))]
#[derive(Copy, Clone)]
pub struct cl_double8__bindgen_ty_3 {
    pub lo: cl_double4,
    pub hi: cl_double4,
}
#[repr(C)]
#[repr(align(128))]
#[derive(Copy, Clone)]
pub union cl_double16 {
    pub __bindgen_anon_1: cl_double16__bindgen_ty_1,
    pub __bindgen_anon_2: cl_double16__bindgen_ty_2,
    pub __bindgen_anon_3: cl_double16__bindgen_ty_3,
    pub s: [cl_double; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_double16__bindgen_ty_1 {
    pub x: cl_double,
    pub y: cl_double,
    pub z: cl_double,
    pub w: cl_double,
    pub __spacer4: cl_double,
    pub __spacer5: cl_double,
    pub __spacer6: cl_double,
    pub __spacer7: cl_double,
    pub __spacer8: cl_double,
    pub __spacer9: cl_double,
    pub sa: cl_double,
    pub sb: cl_double,
    pub sc: cl_double,
    pub sd: cl_double,
    pub se: cl_double,
    pub sf: cl_double,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_double16__bindgen_ty_2 {
    pub s0: cl_double,
    pub s1: cl_double,
    pub s2: cl_double,
    pub s3: cl_double,
    pub s4: cl_double,
    pub s5: cl_double,
    pub s6: cl_double,
    pub s7: cl_double,
    pub s8: cl_double,
    pub s9: cl_double,
    pub sA: cl_double,
    pub sB: cl_double,
    pub sC: cl_double,
    pub sD: cl_double,
    pub sE: cl_double,
    pub sF: cl_double,
}
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct cl_double16__bindgen_ty_3 {
    pub lo: cl_double8,
    pub hi: cl_double8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_platform_id {
    _unused: [u8; 0],
}
pub type cl_platform_id = *mut _cl_platform_id;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_context {
    _unused: [u8; 0],
}
pub type cl_context = *mut _cl_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_command_queue {
    _unused: [u8; 0],
}
pub type cl_command_queue = *mut _cl_command_queue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_mem {
    _unused: [u8; 0],
}
pub type cl_mem = *mut _cl_mem;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_program {
    _unused: [u8; 0],
}
pub type cl_program = *mut _cl_program;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_kernel {
    _unused: [u8; 0],
}
pub type cl_kernel = *mut _cl_kernel;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_event {
    _unused: [u8; 0],
}
pub type cl_event = *mut _cl_event;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_sampler {
    _unused: [u8; 0],
}
pub type cl_sampler = *mut _cl_sampler;
pub type cl_bool = cl_uint;
pub type cl_bitfield = cl_ulong;
pub type cl_device_type = cl_bitfield;
pub type cl_platform_info = cl_uint;
pub type cl_device_info = cl_uint;
pub type cl_device_fp_config = cl_bitfield;
pub type cl_device_mem_cache_type = cl_uint;
pub type cl_device_local_mem_type = cl_uint;
pub type cl_device_exec_capabilities = cl_bitfield;
pub type cl_command_queue_properties = cl_bitfield;
pub type cl_device_partition_property = isize;
pub type cl_device_affinity_domain = cl_bitfield;
pub type cl_context_properties = isize;
pub type cl_context_info = cl_uint;
pub type cl_command_queue_info = cl_uint;
pub type cl_channel_order = cl_uint;
pub type cl_channel_type = cl_uint;
pub type cl_mem_flags = cl_bitfield;
pub type cl_mem_object_type = cl_uint;
pub type cl_mem_info = cl_uint;
pub type cl_mem_migration_flags = cl_bitfield;
pub type cl_image_info = cl_uint;
pub type cl_buffer_create_type = cl_uint;
pub type cl_addressing_mode = cl_uint;
pub type cl_filter_mode = cl_uint;
pub type cl_sampler_info = cl_uint;
pub type cl_map_flags = cl_bitfield;
pub type cl_program_info = cl_uint;
pub type cl_program_build_info = cl_uint;
pub type cl_program_binary_type = cl_uint;
pub type cl_build_status = cl_int;
pub type cl_kernel_info = cl_uint;
pub type cl_kernel_arg_info = cl_uint;
pub type cl_kernel_arg_address_qualifier = cl_uint;
pub type cl_kernel_arg_access_qualifier = cl_uint;
pub type cl_kernel_arg_type_qualifier = cl_bitfield;
pub type cl_kernel_work_group_info = cl_uint;
pub type cl_event_info = cl_uint;
pub type cl_command_type = cl_uint;
pub type cl_profiling_info = cl_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_image_format {
    pub image_channel_order: cl_channel_order,
    pub image_channel_data_type: cl_channel_type,
}
pub type cl_image_format = _cl_image_format;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_image_desc {
    pub image_type: cl_mem_object_type,
    pub image_width: usize,
    pub image_height: usize,
    pub image_depth: usize,
    pub image_array_size: usize,
    pub image_row_pitch: usize,
    pub image_slice_pitch: usize,
    pub num_mip_levels: cl_uint,
    pub num_samples: cl_uint,
    pub buffer: cl_mem,
}
pub type cl_image_desc = _cl_image_desc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_buffer_region {
    pub origin: usize,
    pub size: usize,
}
pub type cl_buffer_region = _cl_buffer_region;
pub type cl_gl_object_type = cl_uint;
pub type cl_gl_texture_info = cl_uint;
pub type cl_gl_platform_info = cl_uint;
pub type cl_GLsync = *mut __GLsync;
pub type cl_iosurface_properties_APPLE = isize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_dag {
    _unused: [u8; 0],
}
pub type cl_dag = *mut _cl_dag;
pub type cl_dag_node = ::std::os::raw::c_int;
pub type cl_queue_properties_APPLE = isize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cl_ndrange {
    pub work_dim: usize,
    pub global_work_offset: [usize; 3usize],
    pub global_work_size: [usize; 3usize],
    pub local_work_size: [usize; 3usize],
}
pub type cl_ndrange = _cl_ndrange;
pub type cl_image = cl_mem;
pub type cl_timer = u64;
pub type sampler_t = usize;
pub type cl_queue_flags = cl_bitfield;
pub type cl_malloc_flags = cl_bitfield;
pub type cl_image_type = cl_mem_object_type;
pub type clk_sampler_type = ::std::os::raw::c_uint;
unsafe extern "C" {
    pub fn clGetPlatformIDs(arg1: cl_uint, arg2: *mut cl_platform_id, arg3: *mut cl_uint)
        -> cl_int;
}
unsafe extern "C" {
    pub fn clGetPlatformInfo(
        arg1: cl_platform_id,
        arg2: cl_platform_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetDeviceIDs(
        arg1: cl_platform_id,
        arg2: cl_device_type,
        arg3: cl_uint,
        arg4: *mut cl_device_id,
        arg5: *mut cl_uint,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetDeviceInfo(
        arg1: cl_device_id,
        arg2: cl_device_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateSubDevices(
        arg1: cl_device_id,
        arg2: *const cl_device_partition_property,
        arg3: cl_uint,
        arg4: *mut cl_device_id,
        arg5: *mut cl_uint,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clRetainDevice(arg1: cl_device_id) -> cl_int;
}
unsafe extern "C" {
    pub fn clReleaseDevice(arg1: cl_device_id) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateContext(
        arg1: *const cl_context_properties,
        arg2: cl_uint,
        arg3: *const cl_device_id,
        arg4: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_char,
                arg2: *const ::std::os::raw::c_void,
                arg3: usize,
                arg4: *mut ::std::os::raw::c_void,
            ),
        >,
        arg5: *mut ::std::os::raw::c_void,
        arg6: *mut cl_int,
    ) -> cl_context;
}
unsafe extern "C" {
    pub fn clCreateContextFromType(
        arg1: *const cl_context_properties,
        arg2: cl_device_type,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_char,
                arg2: *const ::std::os::raw::c_void,
                arg3: usize,
                arg4: *mut ::std::os::raw::c_void,
            ),
        >,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut cl_int,
    ) -> cl_context;
}
unsafe extern "C" {
    pub fn clRetainContext(arg1: cl_context) -> cl_int;
}
unsafe extern "C" {
    pub fn clReleaseContext(arg1: cl_context) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetContextInfo(
        arg1: cl_context,
        arg2: cl_context_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateCommandQueue(
        arg1: cl_context,
        arg2: cl_device_id,
        arg3: cl_command_queue_properties,
        arg4: *mut cl_int,
    ) -> cl_command_queue;
}
unsafe extern "C" {
    pub fn clRetainCommandQueue(arg1: cl_command_queue) -> cl_int;
}
unsafe extern "C" {
    pub fn clReleaseCommandQueue(arg1: cl_command_queue) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetCommandQueueInfo(
        arg1: cl_command_queue,
        arg2: cl_command_queue_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateBuffer(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clCreateSubBuffer(
        arg1: cl_mem,
        arg2: cl_mem_flags,
        arg3: cl_buffer_create_type,
        arg4: *const ::std::os::raw::c_void,
        arg5: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clCreateImage(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: *const cl_image_format,
        arg4: *const cl_image_desc,
        arg5: *mut ::std::os::raw::c_void,
        arg6: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clRetainMemObject(arg1: cl_mem) -> cl_int;
}
unsafe extern "C" {
    pub fn clReleaseMemObject(arg1: cl_mem) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetSupportedImageFormats(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: cl_mem_object_type,
        arg4: cl_uint,
        arg5: *mut cl_image_format,
        arg6: *mut cl_uint,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetMemObjectInfo(
        arg1: cl_mem,
        arg2: cl_mem_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetImageInfo(
        arg1: cl_mem,
        arg2: cl_image_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clSetMemObjectDestructorCallback(
        arg1: cl_mem,
        arg2: ::std::option::Option<
            unsafe extern "C" fn(arg1: cl_mem, arg2: *mut ::std::os::raw::c_void),
        >,
        arg3: *mut ::std::os::raw::c_void,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateSampler(
        arg1: cl_context,
        arg2: cl_bool,
        arg3: cl_addressing_mode,
        arg4: cl_filter_mode,
        arg5: *mut cl_int,
    ) -> cl_sampler;
}
unsafe extern "C" {
    pub fn clRetainSampler(arg1: cl_sampler) -> cl_int;
}
unsafe extern "C" {
    pub fn clReleaseSampler(arg1: cl_sampler) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetSamplerInfo(
        arg1: cl_sampler,
        arg2: cl_sampler_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateProgramWithSource(
        arg1: cl_context,
        arg2: cl_uint,
        arg3: *mut *const ::std::os::raw::c_char,
        arg4: *const usize,
        arg5: *mut cl_int,
    ) -> cl_program;
}
unsafe extern "C" {
    pub fn clCreateProgramWithBinary(
        arg1: cl_context,
        arg2: cl_uint,
        arg3: *const cl_device_id,
        arg4: *const usize,
        arg5: *mut *const ::std::os::raw::c_uchar,
        arg6: *mut cl_int,
        arg7: *mut cl_int,
    ) -> cl_program;
}
unsafe extern "C" {
    pub fn clCreateProgramWithBuiltInKernels(
        arg1: cl_context,
        arg2: cl_uint,
        arg3: *const cl_device_id,
        arg4: *const ::std::os::raw::c_char,
        arg5: *mut cl_int,
    ) -> cl_program;
}
unsafe extern "C" {
    pub fn clRetainProgram(arg1: cl_program) -> cl_int;
}
unsafe extern "C" {
    pub fn clReleaseProgram(arg1: cl_program) -> cl_int;
}
unsafe extern "C" {
    pub fn clBuildProgram(
        arg1: cl_program,
        arg2: cl_uint,
        arg3: *const cl_device_id,
        arg4: *const ::std::os::raw::c_char,
        arg5: ::std::option::Option<
            unsafe extern "C" fn(arg1: cl_program, arg2: *mut ::std::os::raw::c_void),
        >,
        arg6: *mut ::std::os::raw::c_void,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCompileProgram(
        arg1: cl_program,
        arg2: cl_uint,
        arg3: *const cl_device_id,
        arg4: *const ::std::os::raw::c_char,
        arg5: cl_uint,
        arg6: *const cl_program,
        arg7: *mut *const ::std::os::raw::c_char,
        arg8: ::std::option::Option<
            unsafe extern "C" fn(arg1: cl_program, arg2: *mut ::std::os::raw::c_void),
        >,
        arg9: *mut ::std::os::raw::c_void,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clLinkProgram(
        arg1: cl_context,
        arg2: cl_uint,
        arg3: *const cl_device_id,
        arg4: *const ::std::os::raw::c_char,
        arg5: cl_uint,
        arg6: *const cl_program,
        arg7: ::std::option::Option<
            unsafe extern "C" fn(arg1: cl_program, arg2: *mut ::std::os::raw::c_void),
        >,
        arg8: *mut ::std::os::raw::c_void,
        arg9: *mut cl_int,
    ) -> cl_program;
}
unsafe extern "C" {
    pub fn clUnloadPlatformCompiler(arg1: cl_platform_id) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetProgramInfo(
        arg1: cl_program,
        arg2: cl_program_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetProgramBuildInfo(
        arg1: cl_program,
        arg2: cl_device_id,
        arg3: cl_program_build_info,
        arg4: usize,
        arg5: *mut ::std::os::raw::c_void,
        arg6: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateKernel(
        arg1: cl_program,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut cl_int,
    ) -> cl_kernel;
}
unsafe extern "C" {
    pub fn clCreateKernelsInProgram(
        arg1: cl_program,
        arg2: cl_uint,
        arg3: *mut cl_kernel,
        arg4: *mut cl_uint,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clRetainKernel(arg1: cl_kernel) -> cl_int;
}
unsafe extern "C" {
    pub fn clReleaseKernel(arg1: cl_kernel) -> cl_int;
}
unsafe extern "C" {
    pub fn clSetKernelArg(
        arg1: cl_kernel,
        arg2: cl_uint,
        arg3: usize,
        arg4: *const ::std::os::raw::c_void,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetKernelInfo(
        arg1: cl_kernel,
        arg2: cl_kernel_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetKernelArgInfo(
        arg1: cl_kernel,
        arg2: cl_uint,
        arg3: cl_kernel_arg_info,
        arg4: usize,
        arg5: *mut ::std::os::raw::c_void,
        arg6: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetKernelWorkGroupInfo(
        arg1: cl_kernel,
        arg2: cl_device_id,
        arg3: cl_kernel_work_group_info,
        arg4: usize,
        arg5: *mut ::std::os::raw::c_void,
        arg6: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clWaitForEvents(arg1: cl_uint, arg2: *const cl_event) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetEventInfo(
        arg1: cl_event,
        arg2: cl_event_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateUserEvent(arg1: cl_context, arg2: *mut cl_int) -> cl_event;
}
unsafe extern "C" {
    pub fn clRetainEvent(arg1: cl_event) -> cl_int;
}
unsafe extern "C" {
    pub fn clReleaseEvent(arg1: cl_event) -> cl_int;
}
unsafe extern "C" {
    pub fn clSetUserEventStatus(arg1: cl_event, arg2: cl_int) -> cl_int;
}
unsafe extern "C" {
    pub fn clSetEventCallback(
        arg1: cl_event,
        arg2: cl_int,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(arg1: cl_event, arg2: cl_int, arg3: *mut ::std::os::raw::c_void),
        >,
        arg4: *mut ::std::os::raw::c_void,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetEventProfilingInfo(
        arg1: cl_event,
        arg2: cl_profiling_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clFlush(arg1: cl_command_queue) -> cl_int;
}
unsafe extern "C" {
    pub fn clFinish(arg1: cl_command_queue) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueReadBuffer(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_bool,
        arg4: usize,
        arg5: usize,
        arg6: *mut ::std::os::raw::c_void,
        arg7: cl_uint,
        arg8: *const cl_event,
        arg9: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueReadBufferRect(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_bool,
        arg4: *const usize,
        arg5: *const usize,
        arg6: *const usize,
        arg7: usize,
        arg8: usize,
        arg9: usize,
        arg10: usize,
        arg11: *mut ::std::os::raw::c_void,
        arg12: cl_uint,
        arg13: *const cl_event,
        arg14: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueWriteBuffer(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_bool,
        arg4: usize,
        arg5: usize,
        arg6: *const ::std::os::raw::c_void,
        arg7: cl_uint,
        arg8: *const cl_event,
        arg9: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueWriteBufferRect(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_bool,
        arg4: *const usize,
        arg5: *const usize,
        arg6: *const usize,
        arg7: usize,
        arg8: usize,
        arg9: usize,
        arg10: usize,
        arg11: *const ::std::os::raw::c_void,
        arg12: cl_uint,
        arg13: *const cl_event,
        arg14: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueFillBuffer(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: *const ::std::os::raw::c_void,
        arg4: usize,
        arg5: usize,
        arg6: usize,
        arg7: cl_uint,
        arg8: *const cl_event,
        arg9: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueCopyBuffer(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_mem,
        arg4: usize,
        arg5: usize,
        arg6: usize,
        arg7: cl_uint,
        arg8: *const cl_event,
        arg9: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueCopyBufferRect(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_mem,
        arg4: *const usize,
        arg5: *const usize,
        arg6: *const usize,
        arg7: usize,
        arg8: usize,
        arg9: usize,
        arg10: usize,
        arg11: cl_uint,
        arg12: *const cl_event,
        arg13: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueReadImage(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_bool,
        arg4: *const usize,
        arg5: *const usize,
        arg6: usize,
        arg7: usize,
        arg8: *mut ::std::os::raw::c_void,
        arg9: cl_uint,
        arg10: *const cl_event,
        arg11: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueWriteImage(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_bool,
        arg4: *const usize,
        arg5: *const usize,
        arg6: usize,
        arg7: usize,
        arg8: *const ::std::os::raw::c_void,
        arg9: cl_uint,
        arg10: *const cl_event,
        arg11: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueFillImage(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: *const ::std::os::raw::c_void,
        arg4: *const usize,
        arg5: *const usize,
        arg6: cl_uint,
        arg7: *const cl_event,
        arg8: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueCopyImage(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_mem,
        arg4: *const usize,
        arg5: *const usize,
        arg6: *const usize,
        arg7: cl_uint,
        arg8: *const cl_event,
        arg9: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueCopyImageToBuffer(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_mem,
        arg4: *const usize,
        arg5: *const usize,
        arg6: usize,
        arg7: cl_uint,
        arg8: *const cl_event,
        arg9: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueCopyBufferToImage(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_mem,
        arg4: usize,
        arg5: *const usize,
        arg6: *const usize,
        arg7: cl_uint,
        arg8: *const cl_event,
        arg9: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueMapBuffer(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_bool,
        arg4: cl_map_flags,
        arg5: usize,
        arg6: usize,
        arg7: cl_uint,
        arg8: *const cl_event,
        arg9: *mut cl_event,
        arg10: *mut cl_int,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn clEnqueueMapImage(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: cl_bool,
        arg4: cl_map_flags,
        arg5: *const usize,
        arg6: *const usize,
        arg7: *mut usize,
        arg8: *mut usize,
        arg9: cl_uint,
        arg10: *const cl_event,
        arg11: *mut cl_event,
        arg12: *mut cl_int,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn clEnqueueUnmapMemObject(
        arg1: cl_command_queue,
        arg2: cl_mem,
        arg3: *mut ::std::os::raw::c_void,
        arg4: cl_uint,
        arg5: *const cl_event,
        arg6: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueMigrateMemObjects(
        arg1: cl_command_queue,
        arg2: cl_uint,
        arg3: *const cl_mem,
        arg4: cl_mem_migration_flags,
        arg5: cl_uint,
        arg6: *const cl_event,
        arg7: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueNDRangeKernel(
        arg1: cl_command_queue,
        arg2: cl_kernel,
        arg3: cl_uint,
        arg4: *const usize,
        arg5: *const usize,
        arg6: *const usize,
        arg7: cl_uint,
        arg8: *const cl_event,
        arg9: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueTask(
        arg1: cl_command_queue,
        arg2: cl_kernel,
        arg3: cl_uint,
        arg4: *const cl_event,
        arg5: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueNativeKernel(
        arg1: cl_command_queue,
        arg2: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg3: *mut ::std::os::raw::c_void,
        arg4: usize,
        arg5: cl_uint,
        arg6: *const cl_mem,
        arg7: *mut *const ::std::os::raw::c_void,
        arg8: cl_uint,
        arg9: *const cl_event,
        arg10: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueMarkerWithWaitList(
        arg1: cl_command_queue,
        arg2: cl_uint,
        arg3: *const cl_event,
        arg4: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueBarrierWithWaitList(
        arg1: cl_command_queue,
        arg2: cl_uint,
        arg3: *const cl_event,
        arg4: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetExtensionFunctionAddressForPlatform(
        arg1: cl_platform_id,
        arg2: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn clCreateImage2D(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: *const cl_image_format,
        arg4: usize,
        arg5: usize,
        arg6: usize,
        arg7: *mut ::std::os::raw::c_void,
        arg8: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clCreateImage3D(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: *const cl_image_format,
        arg4: usize,
        arg5: usize,
        arg6: usize,
        arg7: usize,
        arg8: usize,
        arg9: *mut ::std::os::raw::c_void,
        arg10: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clEnqueueMarker(arg1: cl_command_queue, arg2: *mut cl_event) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueWaitForEvents(
        arg1: cl_command_queue,
        arg2: cl_uint,
        arg3: *const cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueBarrier(arg1: cl_command_queue) -> cl_int;
}
unsafe extern "C" {
    pub fn clUnloadCompiler() -> cl_int;
}
unsafe extern "C" {
    pub fn clGetExtensionFunctionAddress(
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn clCreateFromGLBuffer(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: cl_GLuint,
        arg4: *mut ::std::os::raw::c_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clCreateFromGLTexture(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: cl_GLenum,
        arg4: cl_GLint,
        arg5: cl_GLuint,
        arg6: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clCreateFromGLRenderbuffer(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: cl_GLuint,
        arg4: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clGetGLObjectInfo(
        arg1: cl_mem,
        arg2: *mut cl_gl_object_type,
        arg3: *mut cl_GLuint,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clGetGLTextureInfo(
        arg1: cl_mem,
        arg2: cl_gl_texture_info,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
        arg5: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueAcquireGLObjects(
        arg1: cl_command_queue,
        arg2: cl_uint,
        arg3: *const cl_mem,
        arg4: cl_uint,
        arg5: *const cl_event,
        arg6: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clEnqueueReleaseGLObjects(
        arg1: cl_command_queue,
        arg2: cl_uint,
        arg3: *const cl_mem,
        arg4: cl_uint,
        arg5: *const cl_event,
        arg6: *mut cl_event,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateFromGLTexture2D(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: cl_GLenum,
        arg4: cl_GLint,
        arg5: cl_GLuint,
        arg6: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clCreateFromGLTexture3D(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: cl_GLenum,
        arg4: cl_GLint,
        arg5: cl_GLuint,
        arg6: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clGetGLContextInfoAPPLE(
        arg1: cl_context,
        arg2: *mut ::std::os::raw::c_void,
        arg3: cl_gl_platform_info,
        arg4: usize,
        arg5: *mut ::std::os::raw::c_void,
        arg6: *mut usize,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateEventFromGLsyncKHR(
        arg1: cl_context,
        arg2: cl_GLsync,
        arg3: *mut cl_int,
    ) -> cl_event;
}
unsafe extern "C" {
    pub fn clCreateImageFromIOSurface2DAPPLE(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: *const cl_image_format,
        arg4: usize,
        arg5: usize,
        arg6: IOSurfaceRef,
        arg7: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clCreateImageFromIOSurfaceWithPropertiesAPPLE(
        arg1: cl_context,
        arg2: cl_mem_flags,
        arg3: *const cl_image_format,
        arg4: *const cl_image_desc,
        arg5: *mut cl_iosurface_properties_APPLE,
        arg6: *mut cl_int,
    ) -> cl_mem;
}
unsafe extern "C" {
    pub fn clSetMemObjectDestructorAPPLE(
        arg1: cl_mem,
        arg2: ::std::option::Option<
            unsafe extern "C" fn(arg1: cl_mem, arg2: *mut ::std::os::raw::c_void),
        >,
        arg3: *mut ::std::os::raw::c_void,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clLogMessagesToSystemLogAPPLE(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_void,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn clLogMessagesToStdoutAPPLE(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_void,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn clLogMessagesToStderrAPPLE(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_void,
        arg3: usize,
        arg4: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn clCreateContextAndCommandQueueAPPLE(
        arg1: *const cl_context_properties,
        arg2: cl_uint,
        arg3: *const cl_device_id,
        arg4: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_char,
                arg2: *const ::std::os::raw::c_void,
                arg3: usize,
                arg4: *mut ::std::os::raw::c_void,
            ),
        >,
        arg5: *mut ::std::os::raw::c_void,
        arg6: cl_command_queue_properties,
        arg7: *mut cl_context,
        arg8: *mut cl_command_queue,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateProgramAndKernelsWithSourceAPPLE(
        arg1: cl_context,
        arg2: cl_uint,
        arg3: *mut *const ::std::os::raw::c_char,
        arg4: *const usize,
        arg5: cl_uint,
        arg6: *const cl_device_id,
        arg7: *const ::std::os::raw::c_char,
        arg8: cl_uint,
        arg9: *mut *const ::std::os::raw::c_char,
        arg10: *mut cl_program,
        arg11: *mut cl_kernel,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clSetKernelArgsListAPPLE(arg1: cl_kernel, arg2: cl_uint, ...) -> cl_int;
}
unsafe extern "C" {
    pub fn clSetKernelArgsVaListAPPLE(arg1: cl_kernel, arg2: cl_uint, arg3: va_list) -> cl_int;
}
unsafe extern "C" {
    pub fn clSetKernelArgByNameAPPLE(
        arg1: cl_kernel,
        arg2: *const ::std::os::raw::c_char,
        arg3: usize,
        arg4: *const ::std::os::raw::c_void,
    ) -> cl_int;
}
unsafe extern "C" {
    pub fn clCreateDAGAPPLE(c: cl_context) -> cl_dag;
}
unsafe extern "C" {
    pub fn clReleaseDAGAPPLE(dag: cl_dag);
}
unsafe extern "C" {
    pub fn clGetDAGNodeAPPLE(
        d: cl_dag,
        f: cl_kernel,
        args: *mut cl_dag_node,
        arg_indices: *mut ::std::os::raw::c_uint,
        nargs: ::std::os::raw::c_uint,
    ) -> cl_dag_node;
}
unsafe extern "C" {
    pub fn clCreateKernelFromDAGAPPLE(
        d: cl_dag,
        n: cl_uint,
        list: *const cl_device_id,
    ) -> cl_kernel;
}
unsafe extern "C" {
    pub fn clCreateCommandQueueWithPropertiesAPPLE(
        arg1: cl_context,
        arg2: cl_device_id,
        arg3: *const cl_queue_properties_APPLE,
        arg4: *mut cl_int,
    ) -> cl_command_queue;
}
unsafe extern "C" {
    pub fn gcl_create_dispatch_queue(
        flags: cl_queue_flags,
        device_id: cl_device_id,
    ) -> dispatch_queue_t;
}
unsafe extern "C" {
    pub fn gcl_malloc(
        bytes: usize,
        host_ptr: *mut ::std::os::raw::c_void,
        flags: cl_malloc_flags,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn gcl_free(ptr: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn gcl_create_image(
        image_format: *const cl_image_format,
        image_width: usize,
        image_height: usize,
        image_depth: usize,
        io_surface: IOSurfaceRef,
    ) -> cl_image;
}
unsafe extern "C" {
    pub fn gcl_retain_image(image: cl_image);
}
unsafe extern "C" {
    pub fn gcl_release_image(image: cl_image);
}
unsafe extern "C" {
    pub fn gcl_get_supported_image_formats(
        device_id: cl_device_id,
        image_type: cl_image_type,
        num_entries: ::std::os::raw::c_uint,
        image_formats: *mut cl_image_format,
        num_image_formats: *mut ::std::os::raw::c_uint,
    );
}
unsafe extern "C" {
    pub fn gcl_memcpy(
        dst: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        size: usize,
    );
}
unsafe extern "C" {
    pub fn gcl_memcpy_rect(
        dst: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        dst_origin: *const usize,
        src_origin: *const usize,
        region: *const usize,
        dst_row_pitch: usize,
        dst_slice_pitch: usize,
        src_row_pitch: usize,
        src_slice_pitch: usize,
    );
}
unsafe extern "C" {
    pub fn gcl_copy_image(
        dst_image: cl_image,
        src_image: cl_image,
        dst_origin: *const usize,
        src_origin: *const usize,
        region: *const usize,
    );
}
unsafe extern "C" {
    pub fn gcl_copy_ptr_to_image(
        dst_image: cl_mem,
        src_ptr: *mut ::std::os::raw::c_void,
        dst_origin: *const usize,
        region: *const usize,
    );
}
unsafe extern "C" {
    pub fn gcl_copy_image_to_ptr(
        dst_ptr: *mut ::std::os::raw::c_void,
        src_image: cl_image,
        src_origin: *const usize,
        region: *const usize,
    );
}
unsafe extern "C" {
    pub fn gcl_map_ptr(
        ptr: *mut ::std::os::raw::c_void,
        map_flags: cl_map_flags,
        cb: usize,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn gcl_map_image(
        image: cl_image,
        map_flags: cl_map_flags,
        origin: *const usize,
        region: *const usize,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn gcl_unmap(arg1: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn gcl_create_kernel_from_block(kernel_block_ptr: *mut ::std::os::raw::c_void)
        -> cl_kernel;
}
unsafe extern "C" {
    pub fn gcl_get_kernel_block_workgroup_info(
        kernel_block_ptr: *mut ::std::os::raw::c_void,
        param_name: cl_kernel_work_group_info,
        param_value_size: usize,
        param_value: *mut ::std::os::raw::c_void,
        param_value_size_ret: *mut usize,
    );
}
unsafe extern "C" {
    pub fn gcl_get_device_id_with_dispatch_queue(queue: NSObject) -> cl_device_id;
}
unsafe extern "C" {
    pub fn gcl_set_finalizer(
        object: *mut ::std::os::raw::c_void,
        cl_pfn_finalizer: ::std::option::Option<
            unsafe extern "C" fn(
                object: *mut ::std::os::raw::c_void,
                user_data: *mut ::std::os::raw::c_void,
            ),
        >,
        user_data: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn gcl_start_timer() -> cl_timer;
}
unsafe extern "C" {
    pub fn gcl_stop_timer(t: cl_timer) -> f64;
}
unsafe extern "C" {
    pub fn gcl_create_buffer_from_ptr(ptr: *mut ::std::os::raw::c_void) -> cl_mem;
}
unsafe extern "C" {
    pub fn gcl_gl_create_ptr_from_buffer(bufobj: GLuint) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn gcl_gl_create_image_from_texture(
        texture_target: GLenum,
        mip_level: GLint,
        texture: GLuint,
    ) -> cl_image;
}
unsafe extern "C" {
    pub fn gcl_gl_create_image_from_renderbuffer(render_buffer: GLuint) -> cl_image;
}
unsafe extern "C" {
    pub fn gcl_gl_set_sharegroup(share: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn gcl_get_context() -> cl_context;
}

unsafe impl objc2::encode::RefEncode for cl_char2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_char16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_char16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_char16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uchar16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uchar16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uchar16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_short16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_short16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_short16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ushort16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ushort16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ushort16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_int16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_int16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_int16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_uint16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_uint16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_uint16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_long16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_long16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_long16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_ulong16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_ulong16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_ulong16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_float16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_float16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_float16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double2__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double2__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double2__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double2__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double2__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double2__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double4", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double4__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double4__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double4__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double4__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double4__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double4__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double4__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double4__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double4__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double8", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double8__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double8__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double8__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double8__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double8__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double8__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double8__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double8__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double8__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double16 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double16 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double16", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double16__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double16__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double16__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double16__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double16__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double16__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for cl_double16__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cl_double16__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cl_double16__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_platform_id {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_platform_id {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_platform_id", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_context {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_context {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_context", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_command_queue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_command_queue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_command_queue", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_mem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_mem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_mem", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_program {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_program {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_program", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_kernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_kernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_kernel", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_event {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_event {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_event", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_sampler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_sampler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_sampler", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_image_format {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_image_format {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_image_format", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_image_desc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_image_desc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_image_desc", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_buffer_region {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_buffer_region {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_buffer_region", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_dag {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_dag {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_dag", &[]);
}
unsafe impl objc2::encode::RefEncode for _cl_ndrange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _cl_ndrange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_cl_ndrange", &[]);
}
