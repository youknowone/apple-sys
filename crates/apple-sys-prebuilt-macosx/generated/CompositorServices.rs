#[allow(unused_imports)]
use crate::CoreFoundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type cp_layer_renderer_configuration_error_code = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cp_time {
    pub cp_mach_abs_time: u64,
}
pub type cp_axis_direction_convention = u8;
pub type cp_layer_renderer_layout = u32;
pub type cp_supported_color_formats_options = u32;
pub type cp_supported_layouts_options = u32;
pub type cp_drawable_state = u32;
pub type cp_drawable_target = u32;
pub type cp_layer_renderer_state = u32;
unsafe extern "C" {
    pub static cp_layer_renderer_configuration_error_domain: CFErrorDomain;
}

unsafe impl objc2::encode::RefEncode for cp_time {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for cp_time {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("cp_time", &[]);
}
