#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type vImagePixelCount = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImage_Buffer {
    pub data: *mut ::std::os::raw::c_void,
    pub height: vImagePixelCount,
    pub width: vImagePixelCount,
    pub rowBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImage_AffineTransform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImage_AffineTransform_Double {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}
pub type vImage_CGAffineTransform = vImage_AffineTransform_Double;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImage_PerpsectiveTransform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
    pub vx: f32,
    pub vy: f32,
    pub v: f32,
}
pub type vImage_WarpInterpolation = i32;
pub type Pixel_8 = u8;
pub type Pixel_F = f32;
pub type Pixel_16U = u16;
pub type Pixel_16S = i16;
pub type Pixel_16Q12 = i16;
pub type Pixel_32U = u32;
pub type Pixel_16F = u16;
pub type ResamplingFilter = *mut ::std::os::raw::c_void;
pub type GammaFunction = *mut ::std::os::raw::c_void;
pub type vImage_Error = isize;
pub type vImage_Flags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImageConverter {
    _unused: [u8; 0],
}
pub type vImageConverterRef = *mut vImageConverter;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImageCVImageFormat {
    _unused: [u8; 0],
}
pub type vImageCVImageFormatRef = *mut vImageCVImageFormat;
pub type vImageConstCVImageFormatRef = *const vImageCVImageFormat;
pub type vImageARGBType = ::std::os::raw::c_uint;
pub type vImageYpCbCrType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImage_YpCbCrToARGBMatrix {
    pub Yp: f32,
    pub Cr_R: f32,
    pub Cr_G: f32,
    pub Cb_G: f32,
    pub Cb_B: f32,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct vImage_YpCbCrToARGB {
    pub opaque: [u8; 128usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImage_ARGBToYpCbCrMatrix {
    pub R_Yp: f32,
    pub G_Yp: f32,
    pub B_Yp: f32,
    pub R_Cb: f32,
    pub G_Cb: f32,
    pub B_Cb_R_Cr: f32,
    pub G_Cr: f32,
    pub B_Cr: f32,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct vImage_ARGBToYpCbCr {
    pub opaque: [u8; 128usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImage_YpCbCrPixelRange {
    pub Yp_bias: i32,
    pub CbCr_bias: i32,
    pub YpRangeMax: i32,
    pub CbCrRangeMax: i32,
    pub YpMax: i32,
    pub YpMin: i32,
    pub CbCrMax: i32,
    pub CbCrMin: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImage_MultidimensionalTableData {
    _unused: [u8; 0],
}
pub type vImage_MultidimensionalTable = *mut vImage_MultidimensionalTableData;
pub type vImageMDTableUsageHint = ::std::os::raw::c_uint;
pub type vImage_InterpolationMethod = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImage_CGImageFormat {
    pub bitsPerComponent: u32,
    pub bitsPerPixel: u32,
    pub colorSpace: CGColorSpaceRef,
    pub bitmapInfo: CGBitmapInfo,
    pub version: u32,
    pub decode: *const CGFloat,
    pub renderingIntent: CGColorRenderingIntent,
}
pub type vImageBufferTypeCode = u32;
pub type vImageCVImageFormatError = isize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImageChannelDescription {
    pub min: CGFloat,
    pub zero: CGFloat,
    pub full: CGFloat,
    pub max: CGFloat,
}
pub type vImageMatrixType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImageTransferFunction {
    pub c0: CGFloat,
    pub c1: CGFloat,
    pub c2: CGFloat,
    pub c3: CGFloat,
    pub gamma: CGFloat,
    pub cutoff: CGFloat,
    pub c4: CGFloat,
    pub c5: CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImageRGBPrimaries {
    pub red_x: f32,
    pub green_x: f32,
    pub blue_x: f32,
    pub white_x: f32,
    pub red_y: f32,
    pub green_y: f32,
    pub blue_y: f32,
    pub white_y: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vImageWhitePoint {
    pub white_x: f32,
    pub white_y: f32,
}
unsafe extern "C" {
    pub static mut kvImage_YpCbCrToARGBMatrix_ITU_R_601_4: *const vImage_YpCbCrToARGBMatrix;
}
unsafe extern "C" {
    pub static mut kvImage_YpCbCrToARGBMatrix_ITU_R_709_2: *const vImage_YpCbCrToARGBMatrix;
}
unsafe extern "C" {
    pub static mut kvImage_ARGBToYpCbCrMatrix_ITU_R_601_4: *const vImage_ARGBToYpCbCrMatrix;
}
unsafe extern "C" {
    pub static mut kvImage_ARGBToYpCbCrMatrix_ITU_R_709_2: *const vImage_ARGBToYpCbCrMatrix;
}
unsafe extern "C" {
    pub fn vImageAlphaBlend_Planar8(
        srcTop: *const vImage_Buffer,
        srcTopAlpha: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        srcBottomAlpha: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAlphaBlend_PlanarF(
        srcTop: *const vImage_Buffer,
        srcTopAlpha: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        srcBottomAlpha: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAlphaBlend_ARGB8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAlphaBlend_ARGBFFFF(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlend_Planar8(
        srcTop: *const vImage_Buffer,
        srcTopAlpha: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlend_PlanarF(
        srcTop: *const vImage_Buffer,
        srcTopAlpha: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlend_ARGB8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlend_BGRA8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlend_ARGBFFFF(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlend_BGRAFFFF(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlendWithPermute_ARGB8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        makeDestAlphaOpaque: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlendWithPermute_RGBA8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        makeDestAlphaOpaque: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlendMultiply_RGBA8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlendScreen_RGBA8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlendDarken_RGBA8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedAlphaBlendLighten_RGBA8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_Planar8(
        src: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_PlanarF(
        src: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_RGBA8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_RGBAFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_RGBA16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_RGBA16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_ARGB16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultiplyData_RGBA16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_Planar8(
        src: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_PlanarF(
        src: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_RGBA8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_RGBAFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_RGBA16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_RGBA16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_ARGB16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageUnpremultiplyData_RGBA16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedConstAlphaBlend_Planar8(
        srcTop: *const vImage_Buffer,
        constAlpha: Pixel_8,
        srcTopAlpha: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedConstAlphaBlend_PlanarF(
        srcTop: *const vImage_Buffer,
        constAlpha: Pixel_F,
        srcTopAlpha: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedConstAlphaBlend_ARGB8888(
        srcTop: *const vImage_Buffer,
        constAlpha: Pixel_8,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePremultipliedConstAlphaBlend_ARGBFFFF(
        srcTop: *const vImage_Buffer,
        constAlpha: Pixel_F,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAlphaBlend_NonpremultipliedToPremultiplied_Planar8(
        srcTop: *const vImage_Buffer,
        srcTopAlpha: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAlphaBlend_NonpremultipliedToPremultiplied_PlanarF(
        srcTop: *const vImage_Buffer,
        srcTopAlpha: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAlphaBlend_NonpremultipliedToPremultiplied_ARGB8888(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAlphaBlend_NonpremultipliedToPremultiplied_ARGBFFFF(
        srcTop: *const vImage_Buffer,
        srcBottom: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageClipToAlpha_Planar8(
        src: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageClipToAlpha_PlanarF(
        src: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageClipToAlpha_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageClipToAlpha_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageClipToAlpha_RGBA8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageClipToAlpha_RGBAFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolve_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const i16,
        kernel_height: u32,
        kernel_width: u32,
        divisor: i32,
        backgroundColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolve_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        backgroundColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolve_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        backgroundColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolve_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const i16,
        kernel_height: u32,
        kernel_width: u32,
        divisor: i32,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolve_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        backgroundColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolve_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        backgroundColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolveWithBias_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const i16,
        kernel_height: u32,
        kernel_width: u32,
        divisor: i32,
        bias: i32,
        backgroundColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolveWithBias_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        bias: f32,
        backgroundColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolveWithBias_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        bias: f32,
        backgroundColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolveWithBias_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const i16,
        kernel_height: u32,
        kernel_width: u32,
        divisor: i32,
        bias: i32,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolveFloatKernel_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernelHeight: u32,
        kernelWidth: u32,
        bias: f32,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolveWithBias_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        bias: f32,
        backgroundColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolveWithBias_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        bias: f32,
        backgroundColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolveMultiKernel_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernels: *mut *const i16,
        kernel_height: u32,
        kernel_width: u32,
        divisors: *const i32,
        biases: *const i32,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvolveMultiKernel_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernels: *mut *const f32,
        kernel_height: u32,
        kernel_width: u32,
        biases: *const f32,
        backgroundColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRichardsonLucyDeConvolve_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const i16,
        kernel2: *const i16,
        kernel_height: u32,
        kernel_width: u32,
        kernel_height2: u32,
        kernel_width2: u32,
        divisor: i32,
        divisor2: i32,
        backgroundColor: Pixel_8,
        iterationCount: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRichardsonLucyDeConvolve_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel2: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        kernel_height2: u32,
        kernel_width2: u32,
        backgroundColor: Pixel_F,
        iterationCount: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRichardsonLucyDeConvolve_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const i16,
        kernel2: *const i16,
        kernel_height: u32,
        kernel_width: u32,
        kernel_height2: u32,
        kernel_width2: u32,
        divisor: i32,
        divisor2: i32,
        backgroundColor: *const u8,
        iterationCount: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRichardsonLucyDeConvolve_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel2: *const f32,
        kernel_height: u32,
        kernel_width: u32,
        kernel_height2: u32,
        kernel_width2: u32,
        backgroundColor: *const f32,
        iterationCount: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBoxConvolve_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: u32,
        kernel_width: u32,
        backgroundColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBoxConvolve_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: u32,
        kernel_width: u32,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageTentConvolve_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: u32,
        kernel_width: u32,
        backgroundColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageTentConvolve_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: u32,
        kernel_width: u32,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSepConvolve_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernelX: *const f32,
        kernelX_width: u32,
        kernelY: *const f32,
        kernelY_width: u32,
        bias: f32,
        backgroundColor: Pixel_16U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSepConvolve_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernelX: *const f32,
        kernelX_width: u32,
        kernelY: *const f32,
        kernelY_width: u32,
        bias: f32,
        backgroundColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSepConvolve_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernelX: *const f32,
        kernelX_width: u32,
        kernelY: *const f32,
        kernelY_width: u32,
        bias: f32,
        backgroundColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSepConvolve_Planar16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernelX: *const f32,
        kernelX_width: u32,
        kernelY: *const f32,
        kernelY_width: u32,
        bias: f32,
        backgroundColor: Pixel_16U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSepConvolve_Planar8to16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernelX: *const f32,
        kernelX_width: u32,
        kernelY: *const f32,
        kernelY_width: u32,
        scale: f32,
        bias: f32,
        backgroundColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSepConvolve_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernelX: *const f32,
        kernelX_width: u32,
        kernelY: *const f32,
        kernelY_width: u32,
        bias: f32,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageClip_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: Pixel_F,
        minFloat: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toPlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: Pixel_F,
        minFloat: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFtoPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: Pixel_F,
        minFloat: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFtoPlanar8_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: Pixel_F,
        minFloat: Pixel_F,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBFFFtoRGB888_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: *const Pixel_F,
        minFloat: *const Pixel_F,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGBFFFFtoARGB8888_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: *const f32,
        minFloat: *const f32,
        dither: ::std::os::raw::c_int,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toARGB8888(
        srcA: *const vImage_Buffer,
        srcR: *const vImage_Buffer,
        srcG: *const vImage_Buffer,
        srcB: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFtoARGBFFFF(
        srcA: *const vImage_Buffer,
        srcR: *const vImage_Buffer,
        srcG: *const vImage_Buffer,
        srcB: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888toPlanar8(
        srcARGB: *const vImage_Buffer,
        destA: *const vImage_Buffer,
        destR: *const vImage_Buffer,
        destG: *const vImage_Buffer,
        destB: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGBFFFFtoPlanarF(
        srcARGB: *const vImage_Buffer,
        destA: *const vImage_Buffer,
        destR: *const vImage_Buffer,
        destG: *const vImage_Buffer,
        destB: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ChunkyToPlanar8(
        srcChannels: *mut *const ::std::os::raw::c_void,
        destPlanarBuffers: *mut *const vImage_Buffer,
        channelCount: ::std::os::raw::c_uint,
        srcStrideBytes: usize,
        srcWidth: vImagePixelCount,
        srcHeight: vImagePixelCount,
        srcRowBytes: usize,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarToChunky8(
        srcPlanarBuffers: *mut *const vImage_Buffer,
        destChannels: *mut *mut ::std::os::raw::c_void,
        channelCount: ::std::os::raw::c_uint,
        destStrideBytes: usize,
        destWidth: vImagePixelCount,
        destHeight: vImagePixelCount,
        destRowBytes: usize,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ChunkyToPlanarF(
        srcChannels: *mut *const ::std::os::raw::c_void,
        destPlanarBuffers: *mut *const vImage_Buffer,
        channelCount: ::std::os::raw::c_uint,
        srcStrideBytes: usize,
        srcWidth: vImagePixelCount,
        srcHeight: vImagePixelCount,
        srcRowBytes: usize,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarToChunkyF(
        srcPlanarBuffers: *mut *const vImage_Buffer,
        destChannels: *mut *mut ::std::os::raw::c_void,
        channelCount: ::std::os::raw::c_uint,
        destStrideBytes: usize,
        destWidth: vImagePixelCount,
        destHeight: vImagePixelCount,
        destRowBytes: usize,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16SToF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        offset: f32,
        scale: f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16UToF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        offset: f32,
        scale: f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_FTo16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        offset: f32,
        scale: f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_FTo16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        offset: f32,
        scale: f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16Uto16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16Fto16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_12UTo16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16UTo12U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageTableLookUp_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        alphaTable: *const Pixel_8,
        redTable: *const Pixel_8,
        greenTable: *const Pixel_8,
        blueTable: *const Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageTableLookUp_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        table: *const Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannels_ARGB8888(
        newSrc: *const vImage_Buffer,
        origSrc: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        copyMask: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannels_ARGBFFFF(
        newSrc: *const vImage_Buffer,
        origSrc: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        copyMask: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithScalar_Planar8(
        scalar: Pixel_8,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithScalar_PlanarF(
        scalar: Pixel_F,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithScalar_Planar16S(
        scalar: Pixel_16S,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithScalar_Planar16U(
        scalar: Pixel_16U,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithScalar_Planar16F(
        scalar: Pixel_16F,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageExtractChannel_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        channelIndex: ::std::os::raw::c_long,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageExtractChannel_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        channelIndex: ::std::os::raw::c_long,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageExtractChannel_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        channelIndex: ::std::os::raw::c_long,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBufferFill_ARGB8888(
        dest: *const vImage_Buffer,
        color: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBufferFill_ARGB16U(
        dest: *const vImage_Buffer,
        color: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBufferFill_ARGB16S(
        dest: *const vImage_Buffer,
        color: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBufferFill_ARGBFFFF(
        dest: *const vImage_Buffer,
        color: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBufferFill_ARGB16F(
        dest: *const vImage_Buffer,
        color: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBufferFill_CbCr8(
        dest: *const vImage_Buffer,
        color: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBufferFill_CbCr16U(
        dest: *const vImage_Buffer,
        color: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBufferFill_CbCr16S(
        dest: *const vImage_Buffer,
        color: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithScalar_ARGB8888(
        scalar: Pixel_8,
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        copyMask: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithScalar_ARGBFFFF(
        scalar: Pixel_F,
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        copyMask: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePermuteChannels_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePermuteChannels_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePermuteChannels_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePermuteChannels_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePermuteChannelsWithMaskedInsert_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        copyMask: u8,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePermuteChannelsWithMaskedInsert_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        copyMask: u8,
        backgroundColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePermuteChannelsWithMaskedInsert_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        copyMask: u8,
        backgroundColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888toPlanarF(
        src: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        maxFloat: *const f32,
        minFloat: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGBFFFFtoPlanar8(
        src: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        maxFloat: *const f32,
        minFloat: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGBFFFFtoRGBFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBAFFFFtoRGBFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_BGRAFFFFtoRGBFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBFFFtoARGBFFFF(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: Pixel_F,
        arg4: *const vImage_Buffer,
        arg5: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBFFFtoRGBAFFFF(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: Pixel_F,
        arg4: *const vImage_Buffer,
        arg5: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBFFFtoBGRAFFFF(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: Pixel_F,
        arg4: *const vImage_Buffer,
        arg5: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB1555toPlanar8(
        src: *const vImage_Buffer,
        destA: *const vImage_Buffer,
        destR: *const vImage_Buffer,
        destG: *const vImage_Buffer,
        destB: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB1555toARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toARGB1555(
        srcA: *const vImage_Buffer,
        srcR: *const vImage_Buffer,
        srcG: *const vImage_Buffer,
        srcB: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888toARGB1555(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA5551toRGBA8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA8888toRGBA5551(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888toARGB1555_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA8888toRGBA5551_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB565toARGB8888(
        alpha: Pixel_8,
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB565toRGBA8888(
        alpha: Pixel_8,
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB565toBGRA8888(
        alpha: Pixel_8,
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB565toRGB888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888toRGB565(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA8888toRGB565(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_BGRA8888toRGB565(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB888toRGB565_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888toRGB565_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA8888toRGB565_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_BGRA8888toRGB565_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB565toPlanar8(
        src: *const vImage_Buffer,
        destR: *const vImage_Buffer,
        destG: *const vImage_Buffer,
        destB: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toRGB565(
        srcR: *const vImage_Buffer,
        srcG: *const vImage_Buffer,
        srcB: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA5551toRGB565(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB1555toRGB565(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB565toRGBA5551(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB565toARGB1555(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar16FtoPlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFtoPlanar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toPlanar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar16FtoPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16UToPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8To16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB888toARGB8888(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: Pixel_8,
        arg4: *const vImage_Buffer,
        arg5: bool,
        arg6: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB888toRGBA8888(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: Pixel_8,
        arg4: *const vImage_Buffer,
        arg5: bool,
        arg6: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB888toBGRA8888(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: Pixel_8,
        arg4: *const vImage_Buffer,
        arg5: bool,
        arg6: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888toRGB888(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA8888toRGB888(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_BGRA8888toRGB888(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_ARGB8888ToRGB888(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: *const u8,
        arg4: bool,
        arg5: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_ARGBFFFFToRGBFFF(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: *const f32,
        arg4: bool,
        arg5: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_RGBA8888ToRGB888(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: *const u8,
        arg4: bool,
        arg5: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_RGBAFFFFToRGBFFF(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: *const f32,
        arg4: bool,
        arg5: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_BGRA8888ToRGB888(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: *const u8,
        arg4: bool,
        arg5: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_BGRAFFFFToRGBFFF(
        arg1: *const vImage_Buffer,
        arg2: *const vImage_Buffer,
        arg3: *const f32,
        arg4: bool,
        arg5: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toRGB888(
        planarRed: *const vImage_Buffer,
        planarGreen: *const vImage_Buffer,
        planarBlue: *const vImage_Buffer,
        rgbDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFtoRGBFFF(
        planarRed: *const vImage_Buffer,
        planarGreen: *const vImage_Buffer,
        planarBlue: *const vImage_Buffer,
        rgbDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB888toPlanar8(
        rgbSrc: *const vImage_Buffer,
        redDest: *const vImage_Buffer,
        greenDest: *const vImage_Buffer,
        blueDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBFFFtoPlanarF(
        rgbSrc: *const vImage_Buffer,
        redDest: *const vImage_Buffer,
        greenDest: *const vImage_Buffer,
        blueDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSelectChannels_ARGB8888(
        newSrc: *const vImage_Buffer,
        origSrc: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        copyMask: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSelectChannels_ARGBFFFF(
        newSrc: *const vImage_Buffer,
        origSrc: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        copyMask: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithPixel_ARGB8888(
        the_pixel: *const u8,
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        copyMask: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithPixel_ARGB16U(
        the_pixel: *const u16,
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        copyMask: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageOverwriteChannelsWithPixel_ARGBFFFF(
        the_pixel: *const f32,
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        copyMask: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8ToXRGB8888(
        alpha: Pixel_8,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8ToBGRX8888(
        blue: *const vImage_Buffer,
        green: *const vImage_Buffer,
        red: *const vImage_Buffer,
        alpha: Pixel_8,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFToXRGBFFFF(
        alpha: Pixel_F,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFToBGRXFFFF(
        blue: *const vImage_Buffer,
        green: *const vImage_Buffer,
        red: *const vImage_Buffer,
        alpha: Pixel_F,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_XRGB8888ToPlanar8(
        src: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_BGRX8888ToPlanar8(
        src: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        green: *const vImage_Buffer,
        red: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_XRGBFFFFToPlanarF(
        src: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_BGRXFFFFToPlanarF(
        src: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        green: *const vImage_Buffer,
        red: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8ToARGBFFFF(
        alpha: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: *const f32,
        minFloat: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8ToXRGBFFFF(
        alpha: Pixel_F,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: *const f32,
        minFloat: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8ToBGRXFFFF(
        blue: *const vImage_Buffer,
        green: *const vImage_Buffer,
        red: *const vImage_Buffer,
        alpha: Pixel_F,
        dest: *const vImage_Buffer,
        maxFloat: *const f32,
        minFloat: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFToARGB8888(
        alpha: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: *const f32,
        minFloat: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFToXRGB8888(
        alpha: Pixel_8,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        maxFloat: *const f32,
        minFloat: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_PlanarFToBGRX8888(
        blue: *const vImage_Buffer,
        green: *const vImage_Buffer,
        red: *const vImage_Buffer,
        alpha: Pixel_8,
        dest: *const vImage_Buffer,
        maxFloat: *const f32,
        minFloat: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB16UtoARGB16U(
        rgbSrc: *const vImage_Buffer,
        aSrc: *const vImage_Buffer,
        alpha: Pixel_16U,
        argbDest: *const vImage_Buffer,
        premultiply: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB16UtoRGBA16U(
        rgbSrc: *const vImage_Buffer,
        aSrc: *const vImage_Buffer,
        alpha: Pixel_16U,
        rgbaDest: *const vImage_Buffer,
        premultiply: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB16UtoBGRA16U(
        rgbSrc: *const vImage_Buffer,
        aSrc: *const vImage_Buffer,
        alpha: Pixel_16U,
        bgraDest: *const vImage_Buffer,
        premultiply: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16UtoRGB16U(
        argbSrc: *const vImage_Buffer,
        rgbDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA16UtoRGB16U(
        rgbaSrc: *const vImage_Buffer,
        rgbDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_BGRA16UtoRGB16U(
        bgraSrc: *const vImage_Buffer,
        rgbDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar16UtoARGB16U(
        aSrc: *const vImage_Buffer,
        rSrc: *const vImage_Buffer,
        gSrc: *const vImage_Buffer,
        bSrc: *const vImage_Buffer,
        argbDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16UtoPlanar16U(
        argbSrc: *const vImage_Buffer,
        aDest: *const vImage_Buffer,
        rDest: *const vImage_Buffer,
        gDest: *const vImage_Buffer,
        bDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar16UtoRGB16U(
        rSrc: *const vImage_Buffer,
        gSrc: *const vImage_Buffer,
        bSrc: *const vImage_Buffer,
        rgbDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB16UtoPlanar16U(
        rgbSrc: *const vImage_Buffer,
        rDest: *const vImage_Buffer,
        gDest: *const vImage_Buffer,
        bDest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar16UtoPlanar8_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB16UtoRGB888_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16UtoARGB8888_dithered(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        dither: ::std::os::raw::c_int,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16UToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        copyMask: u8,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888ToARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        copyMask: u8,
        backgroundColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB16UToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        copyMask: u8,
        backgroundColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888ToRGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        copyMask: u8,
        backgroundColor: *const Pixel_16U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageByteSwap_Planar16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_ARGB8888(
        argbSrc: *const vImage_Buffer,
        argbDst: *const vImage_Buffer,
        argbBackgroundColorPtr: *const u8,
        isImagePremultiplied: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_RGBA8888(
        rgbaSrc: *const vImage_Buffer,
        rgbaDst: *const vImage_Buffer,
        rgbaBackgroundColorPtr: *const u8,
        isImagePremultiplied: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_ARGB16U(
        argbSrc: *const vImage_Buffer,
        argbDst: *const vImage_Buffer,
        argbBackgroundColorPtr: *const u16,
        isImagePremultiplied: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_RGBA16U(
        rgbaSrc: *const vImage_Buffer,
        rgbaDst: *const vImage_Buffer,
        rgbaBackgroundColorPtr: *const u16,
        isImagePremultiplied: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_ARGB16Q12(
        argbSrc: *const vImage_Buffer,
        argbDst: *const vImage_Buffer,
        argbBackgroundColorPtr: *const i16,
        isImagePremultiplied: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_RGBA16Q12(
        argbSrc: *const vImage_Buffer,
        argbDst: *const vImage_Buffer,
        argbBackgroundColorPtr: *const i16,
        isImagePremultiplied: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_ARGBFFFF(
        argbSrc: *const vImage_Buffer,
        argbDst: *const vImage_Buffer,
        argbBackgroundColorPtr: *const f32,
        isImagePremultiplied: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFlatten_RGBAFFFF(
        rgbaSrc: *const vImage_Buffer,
        rgbaDst: *const vImage_Buffer,
        rgbaBackgroundColorPtr: *const f32,
        isImagePremultiplied: bool,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar1toPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar2toPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar4toPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Indexed1toPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        colors: *const Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Indexed2toPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        colors: *const Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Indexed4toPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        colors: *const Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toPlanar1(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toPlanar2(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toPlanar4(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toIndexed1(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        colors: *mut Pixel_8,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toIndexed2(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        colors: *mut Pixel_8,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar8toIndexed4(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        colors: *mut Pixel_8,
        dither: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_8to16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGB888toPlanar16Q12(
        src: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888toPlanar16Q12(
        src: *const vImage_Buffer,
        alpha: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16Q12to8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar16Q12toRGB888(
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar16Q12toARGB8888(
        alpha: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16Q12to16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar16Q12toRGB16F(
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Planar16Q12toARGB16F(
        alpha: *const vImage_Buffer,
        red: *const vImage_Buffer,
        green: *const vImage_Buffer,
        blue: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16Fto16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16Q12toF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_Fto16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16Q12to16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_16Uto16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_YpCbCrToARGB_GenerateConversion(
        matrix: *const vImage_YpCbCrToARGBMatrix,
        pixelRange: *const vImage_YpCbCrPixelRange,
        outInfo: *mut vImage_YpCbCrToARGB,
        inYpCbCrType: vImageYpCbCrType,
        outARGBType: vImageARGBType,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGBToYpCbCr_GenerateConversion(
        matrix: *const vImage_ARGBToYpCbCrMatrix,
        pixelRange: *const vImage_YpCbCrPixelRange,
        outInfo: *mut vImage_ARGBToYpCbCr,
        inARGBType: vImageARGBType,
        outYpCbCrType: vImageYpCbCrType,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_422YpCbYpCr8ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To422YpCbYpCr8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_422CbYpCrYp8ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To422CbYpCrYp8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_422CbYpCrYp8_AA8ToARGB8888(
        src: *const vImage_Buffer,
        srcA: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To422CbYpCrYp8_AA8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        destA: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_444AYpCbCr8ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To444AYpCbCr8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_444CbYpCrA8ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To444CbYpCrA8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_444CrYpCb8ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To444CrYpCb8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_420Yp8_Cb8_Cr8ToARGB8888(
        srcYp: *const vImage_Buffer,
        srcCb: *const vImage_Buffer,
        srcCr: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To420Yp8_Cb8_Cr8(
        src: *const vImage_Buffer,
        destYp: *const vImage_Buffer,
        destCb: *const vImage_Buffer,
        destCr: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_420Yp8_CbCr8ToARGB8888(
        srcYp: *const vImage_Buffer,
        srcCbCr: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To420Yp8_CbCr8(
        src: *const vImage_Buffer,
        destYp: *const vImage_Buffer,
        destCbCr: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_444AYpCbCr16ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To444AYpCbCr16(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_444AYpCbCr16ToARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16UTo444AYpCbCr16(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_444CrYpCb10ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To444CrYpCb10(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_444CrYpCb10ToARGB16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: Pixel_16Q12,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16Q12To444CrYpCb10(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_422CrYpCbYpCbYpCbYpCrYpCrYp10ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To422CrYpCbYpCbYpCbYpCrYpCrYp10(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_422CrYpCbYpCbYpCbYpCrYpCrYp10ToARGB16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: Pixel_16Q12,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16Q12To422CrYpCbYpCbYpCbYpCrYpCrYp10(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_422CbYpCrYp16ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888To422CbYpCrYp16(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_422CbYpCrYp16ToARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_YpCbCrToARGB,
        permuteMap: *const u8,
        alpha: u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16UTo422CbYpCrYp16(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        info: *const vImage_ARGBToYpCbCr,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA1010102ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888ToRGBA1010102(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA1010102ToARGB16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16Q12ToRGBA1010102(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        RGB101010Min: i32,
        RGB101010Max: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_RGBA1010102ToARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16UToRGBA1010102(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePermuteChannels_RGB888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCopyBuffer(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        pixelSize: usize,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_XRGB2101010ToARGB8888(
        src: *const vImage_Buffer,
        alpha: Pixel_8,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB2101010ToARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888ToXRGB2101010(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB8888ToARGB2101010(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_XRGB2101010ToARGB16Q12(
        src: *const vImage_Buffer,
        alpha: Pixel_16Q12,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB2101010ToARGB16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16Q12ToXRGB2101010(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        RGB101010Min: i32,
        RGB101010Max: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16Q12ToARGB2101010(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        RGB101010Min: i32,
        RGB101010Max: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_XRGB2101010ToARGB16U(
        src: *const vImage_Buffer,
        alpha: u16,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB2101010ToARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16UToXRGB2101010(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB16UToARGB2101010(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_XRGB2101010ToARGBFFFF(
        src: *const vImage_Buffer,
        alpha: Pixel_F,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB2101010ToARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGBFFFFToXRGB2101010(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGBFFFFToARGB2101010(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_XRGB2101010ToARGB16F(
        src: *const vImage_Buffer,
        alpha: Pixel_F,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConvert_ARGB2101010ToARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        RGB101010RangeMin: i32,
        RGB101010RangeMax: i32,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        angleInRadians: f32,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        angleInRadians: f32,
        backColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        angleInRadians: f32,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        angleInRadians: f32,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        angleInRadians: f32,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        angleInRadians: f32,
        backColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        angleInRadians: f32,
        backColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        angleInRadians: f32,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        angleInRadians: f32,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_Planar16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_Planar16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_CbCr8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_CbCr16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageScale_XRGB2101010W(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarp_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarp_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform,
        backColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarp_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarp_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarp_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarp_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform,
        backColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarp_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform,
        backColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarp_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarp_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpD_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform_Double,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpD_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform_Double,
        backColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpD_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform_Double,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpD_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform_Double,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpD_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform_Double,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpD_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform_Double,
        backColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpD_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform_Double,
        backColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpD_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform_Double,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpD_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_AffineTransform_Double,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpCG_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_CGAffineTransform,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpCG_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_CGAffineTransform,
        backColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpCG_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_CGAffineTransform,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpCG_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_CGAffineTransform,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpCG_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_CGAffineTransform,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageAffineWarpCG_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_CGAffineTransform,
        backColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageGetPerspectiveWarp(
        srcPoints: *const [f32; 2usize],
        destPoints: *const [f32; 2usize],
        transform: *mut vImage_PerpsectiveTransform,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePerspectiveWarp_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_PerpsectiveTransform,
        interpolation: vImage_WarpInterpolation,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePerspectiveWarp_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_PerpsectiveTransform,
        interpolation: vImage_WarpInterpolation,
        backColor: *mut u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePerspectiveWarp_Planar16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_PerpsectiveTransform,
        interpolation: vImage_WarpInterpolation,
        backColor: Pixel_16U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePerspectiveWarp_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_PerpsectiveTransform,
        interpolation: vImage_WarpInterpolation,
        backColor: *mut u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePerspectiveWarp_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_PerpsectiveTransform,
        interpolation: vImage_WarpInterpolation,
        backColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePerspectiveWarp_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        transform: *const vImage_PerpsectiveTransform,
        interpolation: vImage_WarpInterpolation,
        backColor: *mut u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_Planar16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalReflect_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_Planar16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalReflect_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_Planar16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: Pixel_16U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageRotate90_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        rotationConstant: u8,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_Planar16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_16S,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_Planar16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_16U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_Planar16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_16S,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_Planar16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_16U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_ARGB16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_ARGB16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_Planar16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: Pixel_16F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_CbCr16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_ARGB16F(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_CbCr8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_CbCr16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_CbCr16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_CbCr16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShearD_CbCr16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_CbCr8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_CbCr16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_CbCr16U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const u16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_CbCr16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShearD_CbCr16S(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f64,
        shearSlope: f64,
        filter: ResamplingFilter,
        backColor: *const i16,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHorizontalShear_XRGB2101010W(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        xTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_32U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageVerticalShear_XRGB2101010W(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        yTranslate: f32,
        shearSlope: f32,
        filter: ResamplingFilter,
        backColor: Pixel_32U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageNewResamplingFilter(scale: f32, flags: vImage_Flags) -> ResamplingFilter;
}
unsafe extern "C" {
    pub fn vImageDestroyResamplingFilter(filter: ResamplingFilter);
}
unsafe extern "C" {
    pub fn vImageNewResamplingFilterForFunctionUsingBuffer(
        filter: ResamplingFilter,
        scale: f32,
        kernelFunc: ::std::option::Option<
            unsafe extern "C" fn(
                xArray: *const f32,
                yArray: *mut f32,
                count: ::std::os::raw::c_ulong,
                userData: *mut ::std::os::raw::c_void,
            ),
        >,
        kernelWidth: f32,
        userData: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageGetResamplingFilterSize(
        scale: f32,
        kernelFunc: ::std::option::Option<
            unsafe extern "C" fn(
                xArray: *const f32,
                yArray: *mut f32,
                count: ::std::os::raw::c_ulong,
                userData: *mut ::std::os::raw::c_void,
            ),
        >,
        kernelWidth: f32,
        flags: vImage_Flags,
    ) -> usize;
}
unsafe extern "C" {
    pub fn vImageGetResamplingFilterExtent(
        filter: ResamplingFilter,
        flags: vImage_Flags,
    ) -> vImagePixelCount;
}
unsafe extern "C" {
    pub fn vImageHistogramCalculation_Planar8(
        src: *const vImage_Buffer,
        histogram: *mut vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHistogramCalculation_PlanarF(
        src: *const vImage_Buffer,
        histogram: *mut vImagePixelCount,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHistogramCalculation_ARGB8888(
        src: *const vImage_Buffer,
        histogram: *mut *mut vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHistogramCalculation_ARGBFFFF(
        src: *const vImage_Buffer,
        histogram: *mut *mut vImagePixelCount,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageEqualization_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageEqualization_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageEqualization_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageEqualization_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHistogramSpecification_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        desired_histogram: *const vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHistogramSpecification_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        desired_histogram: *const vImagePixelCount,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHistogramSpecification_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        desired_histogram: *mut *const vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageHistogramSpecification_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        desired_histogram: *mut *const vImagePixelCount,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageContrastStretch_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageContrastStretch_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageContrastStretch_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageContrastStretch_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageEndsInContrastStretch_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        percent_low: ::std::os::raw::c_uint,
        percent_high: ::std::os::raw::c_uint,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageEndsInContrastStretch_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        percent_low: ::std::os::raw::c_uint,
        percent_high: ::std::os::raw::c_uint,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageEndsInContrastStretch_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        percent_low: *const ::std::os::raw::c_uint,
        percent_high: *const ::std::os::raw::c_uint,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageEndsInContrastStretch_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        percent_low: *const ::std::os::raw::c_uint,
        percent_high: *const ::std::os::raw::c_uint,
        histogram_entries: ::std::os::raw::c_uint,
        minVal: Pixel_F,
        maxVal: Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageDilate_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const ::std::os::raw::c_uchar,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageDilate_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageDilate_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const ::std::os::raw::c_uchar,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageDilate_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageErode_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const ::std::os::raw::c_uchar,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageErode_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageErode_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const ::std::os::raw::c_uchar,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageErode_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel: *const f32,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMax_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMax_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMax_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMax_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMin_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMin_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMin_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMin_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        srcOffsetToROI_X: vImagePixelCount,
        srcOffsetToROI_Y: vImagePixelCount,
        kernel_height: vImagePixelCount,
        kernel_width: vImagePixelCount,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePNGDecompressionFilter(
        buffer: *const vImage_Buffer,
        startScanline: vImagePixelCount,
        scanlineCount: vImagePixelCount,
        bitsPerPixel: u32,
        filterMethodNumber: u32,
        filterType: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMatrixMultiply_Planar16S(
        srcs: *mut *const vImage_Buffer,
        dests: *mut *const vImage_Buffer,
        src_planes: u32,
        dest_planes: u32,
        matrix: *const i16,
        divisor: i32,
        pre_bias: *const i16,
        post_bias: *const i32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMatrixMultiply_Planar8(
        srcs: *mut *const vImage_Buffer,
        dests: *mut *const vImage_Buffer,
        src_planes: u32,
        dest_planes: u32,
        matrix: *const i16,
        divisor: i32,
        pre_bias: *const i16,
        post_bias: *const i32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMatrixMultiply_PlanarF(
        srcs: *mut *const vImage_Buffer,
        dests: *mut *const vImage_Buffer,
        src_planes: u32,
        dest_planes: u32,
        matrix: *const f32,
        pre_bias: *const f32,
        post_bias: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMatrixMultiply_ARGB8888(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        matrix: *const i16,
        divisor: i32,
        pre_bias: *const i16,
        post_bias: *const i32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMatrixMultiply_ARGBFFFF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        matrix: *const f32,
        pre_bias: *const f32,
        post_bias: *const f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMatrixMultiply_ARGB8888ToPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        matrix: *const i16,
        divisor: i32,
        pre_bias: *const i16,
        post_bias: i32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMatrixMultiply_ARGBFFFFToPlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        matrix: *const f32,
        pre_bias: *const f32,
        post_bias: f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCreateGammaFunction(
        gamma: f32,
        gamma_type: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> GammaFunction;
}
unsafe extern "C" {
    pub fn vImageDestroyGammaFunction(f: GammaFunction);
}
unsafe extern "C" {
    pub fn vImageGamma_Planar8toPlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        gamma: GammaFunction,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageGamma_PlanarFtoPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        gamma: GammaFunction,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageGamma_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        gamma: GammaFunction,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewiseGamma_Planar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        exponentialCoeffs: *const f32,
        gamma: f32,
        linearCoeffs: *const f32,
        boundary: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewiseGamma_Planar8toPlanar16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        exponentialCoeffs: *const f32,
        gamma: f32,
        linearCoeffs: *const f32,
        boundary: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewiseGamma_Planar16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        exponentialCoeffs: *const f32,
        gamma: f32,
        linearCoeffs: *const f32,
        boundary: Pixel_16S,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewiseGamma_Planar16Q12toPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        exponentialCoeffs: *const f32,
        gamma: f32,
        linearCoeffs: *const f32,
        boundary: Pixel_16S,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewiseGamma_Planar8toPlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        exponentialCoeffs: *const f32,
        gamma: f32,
        linearCoeffs: *const f32,
        boundary: Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewiseGamma_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        exponentialCoeffs: *const f32,
        gamma: f32,
        linearCoeffs: *const f32,
        boundary: f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewiseGamma_PlanarFtoPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        exponentialCoeffs: *const f32,
        gamma: f32,
        linearCoeffs: *const f32,
        boundary: f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSymmetricPiecewiseGamma_Planar16Q12(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        exponentialCoeffs: *const f32,
        gamma: f32,
        linearCoeffs: *const f32,
        boundary: Pixel_16S,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSymmetricPiecewiseGamma_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        exponentialCoeffs: *const f32,
        gamma: f32,
        linearCoeffs: *const f32,
        boundary: f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewisePolynomial_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        coefficients: *mut *const f32,
        boundaries: *const f32,
        order: u32,
        log2segments: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewisePolynomial_Planar8toPlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        coefficients: *mut *const f32,
        boundaries: *const f32,
        order: u32,
        log2segments: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewisePolynomial_PlanarFtoPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        coefficients: *mut *const f32,
        boundaries: *const f32,
        order: u32,
        log2segments: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageSymmetricPiecewisePolynomial_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        coefficients: *mut *const f32,
        boundaries: *const f32,
        order: u32,
        log2segments: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImagePiecewiseRational_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        topCoefficients: *mut *const f32,
        bottomCoefficients: *mut *const f32,
        boundaries: *const f32,
        topOrder: u32,
        bottomOrder: u32,
        log2segments: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageLookupTable_Planar8toPlanar16(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        table: *const Pixel_16U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageLookupTable_Planar8toPlanar24(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        table: *const u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageLookupTable_Planar8toPlanar48(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        table: *const u64,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageLookupTable_Planar8toPlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        table: *const Pixel_F,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageLookupTable_PlanarFtoPlanar8(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        table: *const Pixel_8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageLookupTable_8to64U(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        LUT: *const u64,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageLookupTable_Planar16(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        table: *const Pixel_16U,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageInterpolatedLookupTable_PlanarF(
        src: *const vImage_Buffer,
        dest: *const vImage_Buffer,
        table: *const Pixel_F,
        tableEntries: vImagePixelCount,
        maxFloat: f32,
        minFloat: f32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMultidimensionalTable_Create(
        tableData: *const u16,
        numSrcChannels: u32,
        numDestChannels: u32,
        table_entries_per_dimension: *const u8,
        hint: vImageMDTableUsageHint,
        flags: vImage_Flags,
        err: *mut vImage_Error,
    ) -> vImage_MultidimensionalTable;
}
unsafe extern "C" {
    pub fn vImageMultidimensionalTable_Retain(table: vImage_MultidimensionalTable) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMultidimensionalTable_Release(table: vImage_MultidimensionalTable)
        -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMultiDimensionalInterpolatedLookupTable_PlanarF(
        srcs: *const vImage_Buffer,
        dests: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        table: vImage_MultidimensionalTable,
        method: vImage_InterpolationMethod,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageMultiDimensionalInterpolatedLookupTable_Planar16Q12(
        srcs: *const vImage_Buffer,
        dests: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        table: vImage_MultidimensionalTable,
        method: vImage_InterpolationMethod,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFloodFill_Planar8(
        srcDest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        seedX: vImagePixelCount,
        seedY: vImagePixelCount,
        newValue: Pixel_8,
        connectivity: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFloodFill_Planar16U(
        srcDest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        seedX: vImagePixelCount,
        seedY: vImagePixelCount,
        newValue: Pixel_16U,
        connectivity: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFloodFill_ARGB8888(
        srcDest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        seedX: vImagePixelCount,
        seedY: vImagePixelCount,
        newValue: *mut u8,
        connectivity: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageFloodFill_ARGB16U(
        srcDest: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        seedX: vImagePixelCount,
        seedY: vImagePixelCount,
        newValue: *mut u16,
        connectivity: ::std::os::raw::c_int,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub static mut kvImageDecodeArray_16Q12Format: *const CGFloat;
}
unsafe extern "C" {
    pub fn vImageBuffer_Init(
        buf: *mut vImage_Buffer,
        height: vImagePixelCount,
        width: vImagePixelCount,
        pixelBits: u32,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBuffer_GetSize(buf: *const vImage_Buffer) -> CGSize;
}
unsafe extern "C" {
    pub fn vImageCGImageFormat_GetComponentCount(format: *const vImage_CGImageFormat) -> u32;
}
unsafe extern "C" {
    pub fn vImageCGImageFormat_IsEqual(
        f1: *const vImage_CGImageFormat,
        f2: *const vImage_CGImageFormat,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn vImageBuffer_InitWithCGImage(
        buf: *mut vImage_Buffer,
        format: *mut vImage_CGImageFormat,
        backgroundColor: *const CGFloat,
        image: CGImageRef,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCreateCGImageFromBuffer(
        buf: *const vImage_Buffer,
        format: *const vImage_CGImageFormat,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                userData: *mut ::std::os::raw::c_void,
                buf_data: *mut ::std::os::raw::c_void,
            ),
        >,
        userData: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
        error: *mut vImage_Error,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn vImageConverter_Retain(converter: vImageConverterRef);
}
unsafe extern "C" {
    pub fn vImageConverter_Release(converter: vImageConverterRef);
}
unsafe extern "C" {
    pub fn vImageConverter_CreateWithCGImageFormat(
        srcFormat: *const vImage_CGImageFormat,
        destFormat: *const vImage_CGImageFormat,
        backgroundColor: *const CGFloat,
        flags: vImage_Flags,
        error: *mut vImage_Error,
    ) -> vImageConverterRef;
}
unsafe extern "C" {
    pub fn vImageConverter_CreateWithColorSyncCodeFragment(
        codeFragment: CFTypeRef,
        srcFormat: *const vImage_CGImageFormat,
        destFormat: *const vImage_CGImageFormat,
        backgroundColor: *const CGFloat,
        flags: vImage_Flags,
        error: *mut vImage_Error,
    ) -> vImageConverterRef;
}
unsafe extern "C" {
    pub fn vImageConverter_CreateWithCGColorConversionInfo(
        colorConversionInfoRef: CGColorConversionInfoRef,
        sFormat: *const vImage_CGImageFormat,
        dFormat: *const vImage_CGImageFormat,
        bg: *const CGFloat,
        flags: vImage_Flags,
        error: *mut vImage_Error,
    ) -> vImageConverterRef;
}
unsafe extern "C" {
    pub fn vImageConverter_MustOperateOutOfPlace(
        converter: vImageConverterRef,
        srcs: *const vImage_Buffer,
        dests: *const vImage_Buffer,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageConverter_GetNumberOfSourceBuffers(
        converter: vImageConverterRef,
    ) -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn vImageConverter_GetNumberOfDestinationBuffers(
        converter: vImageConverterRef,
    ) -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn vImageConverter_GetSourceBufferOrder(
        converter: vImageConverterRef,
    ) -> *const vImageBufferTypeCode;
}
unsafe extern "C" {
    pub fn vImageConverter_GetDestinationBufferOrder(
        converter: vImageConverterRef,
    ) -> *const vImageBufferTypeCode;
}
unsafe extern "C" {
    pub fn vImageConvert_AnyToAny(
        converter: vImageConverterRef,
        srcs: *const vImage_Buffer,
        dests: *const vImage_Buffer,
        tempBuffer: *mut ::std::os::raw::c_void,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBuffer_InitWithCVPixelBuffer(
        buffer: *mut vImage_Buffer,
        desiredFormat: *mut vImage_CGImageFormat,
        cvPixelBuffer: CVPixelBufferRef,
        cvImageFormat: vImageCVImageFormatRef,
        backgroundColor: *const CGFloat,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBuffer_CopyToCVPixelBuffer(
        buffer: *const vImage_Buffer,
        bufferFormat: *const vImage_CGImageFormat,
        cvPixelBuffer: CVPixelBufferRef,
        cvImageFormat: vImageCVImageFormatRef,
        backgroundColor: *const CGFloat,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_CreateWithCVPixelBuffer(
        buffer: CVPixelBufferRef,
    ) -> vImageCVImageFormatRef;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_Create(
        imageFormatType: u32,
        matrix: *const vImage_ARGBToYpCbCrMatrix,
        cvImageBufferChromaLocation: CFStringRef,
        baseColorspace: CGColorSpaceRef,
        alphaIsOneHint: ::std::os::raw::c_int,
    ) -> vImageCVImageFormatRef;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_Copy(format: vImageConstCVImageFormatRef) -> vImageCVImageFormatRef;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_Retain(fmt: vImageCVImageFormatRef);
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_Release(fmt: vImageCVImageFormatRef);
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_GetFormatCode(format: vImageConstCVImageFormatRef) -> u32;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_GetChannelCount(format: vImageConstCVImageFormatRef) -> u32;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_GetChannelNames(
        format: vImageConstCVImageFormatRef,
    ) -> *const vImageBufferTypeCode;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_GetColorSpace(
        format: vImageConstCVImageFormatRef,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_SetColorSpace(
        format: vImageCVImageFormatRef,
        colorspace: CGColorSpaceRef,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_GetChromaSiting(format: vImageConstCVImageFormatRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_SetChromaSiting(
        format: vImageCVImageFormatRef,
        siting: CFStringRef,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_GetConversionMatrix(
        format: vImageConstCVImageFormatRef,
        outType: *mut vImageMatrixType,
    ) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_CopyConversionMatrix(
        format: vImageCVImageFormatRef,
        matrix: *const ::std::os::raw::c_void,
        inType: vImageMatrixType,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_GetAlphaHint(
        format: vImageConstCVImageFormatRef,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_SetAlphaHint(
        format: vImageCVImageFormatRef,
        alphaIsOne: ::std::os::raw::c_int,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_GetChannelDescription(
        format: vImageConstCVImageFormatRef,
        type_: vImageBufferTypeCode,
    ) -> *const vImageChannelDescription;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_CopyChannelDescription(
        format: vImageCVImageFormatRef,
        desc: *const vImageChannelDescription,
        type_: vImageBufferTypeCode,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_GetUserData(
        format: vImageConstCVImageFormatRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn vImageCVImageFormat_SetUserData(
        format: vImageCVImageFormatRef,
        userData: *mut ::std::os::raw::c_void,
        userDataReleaseCallback: ::std::option::Option<
            unsafe extern "C" fn(
                callback_fmt: vImageCVImageFormatRef,
                callback_userData: *mut ::std::os::raw::c_void,
            ),
        >,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageCreateRGBColorSpaceWithPrimariesAndTransferFunction(
        primaries: *const vImageRGBPrimaries,
        tf: *const vImageTransferFunction,
        intent: CGColorRenderingIntent,
        flags: vImage_Flags,
        error: *mut vImage_Error,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn vImageCreateMonochromeColorSpaceWithWhitePointAndTransferFunction(
        whitePoint: *const vImageWhitePoint,
        tf: *const vImageTransferFunction,
        intent: CGColorRenderingIntent,
        flags: vImage_Flags,
        error: *mut vImage_Error,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn vImageConverter_CreateForCGToCVImageFormat(
        srcFormat: *const vImage_CGImageFormat,
        destFormat: vImageCVImageFormatRef,
        backgroundColor: *const CGFloat,
        flags: vImage_Flags,
        error: *mut vImage_Error,
    ) -> vImageConverterRef;
}
unsafe extern "C" {
    pub fn vImageConverter_CreateForCVToCGImageFormat(
        srcFormat: vImageCVImageFormatRef,
        destFormat: *const vImage_CGImageFormat,
        backgroundColor: *const CGFloat,
        flags: vImage_Flags,
        error: *mut vImage_Error,
    ) -> vImageConverterRef;
}
unsafe extern "C" {
    pub fn vImageBuffer_InitForCopyToCVPixelBuffer(
        buffers: *mut vImage_Buffer,
        converter: vImageConverterRef,
        pixelBuffer: CVPixelBufferRef,
        flags: vImage_Flags,
    ) -> vImage_Error;
}
unsafe extern "C" {
    pub fn vImageBuffer_InitForCopyFromCVPixelBuffer(
        buffers: *mut vImage_Buffer,
        converter: vImageConverterRef,
        pixelBuffer: CVPixelBufferRef,
        flags: vImage_Flags,
    ) -> vImage_Error;
}

unsafe impl objc2::encode::RefEncode for vImage_Buffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_Buffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_Buffer", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_AffineTransform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_AffineTransform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_AffineTransform", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_AffineTransform_Double {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_AffineTransform_Double {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_AffineTransform_Double", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_PerpsectiveTransform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_PerpsectiveTransform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_PerpsectiveTransform", &[]);
}
unsafe impl objc2::encode::RefEncode for vImageConverter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImageConverter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImageConverter", &[]);
}
unsafe impl objc2::encode::RefEncode for vImageCVImageFormat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImageCVImageFormat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImageCVImageFormat", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_YpCbCrToARGBMatrix {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_YpCbCrToARGBMatrix {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_YpCbCrToARGBMatrix", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_YpCbCrToARGB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_YpCbCrToARGB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_YpCbCrToARGB", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_ARGBToYpCbCrMatrix {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_ARGBToYpCbCrMatrix {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_ARGBToYpCbCrMatrix", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_ARGBToYpCbCr {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_ARGBToYpCbCr {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_ARGBToYpCbCr", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_YpCbCrPixelRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_YpCbCrPixelRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_YpCbCrPixelRange", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_MultidimensionalTableData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_MultidimensionalTableData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_MultidimensionalTableData", &[]);
}
unsafe impl objc2::encode::RefEncode for vImage_CGImageFormat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImage_CGImageFormat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImage_CGImageFormat", &[]);
}
unsafe impl objc2::encode::RefEncode for vImageChannelDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImageChannelDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImageChannelDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for vImageTransferFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImageTransferFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImageTransferFunction", &[]);
}
unsafe impl objc2::encode::RefEncode for vImageRGBPrimaries {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImageRGBPrimaries {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImageRGBPrimaries", &[]);
}
unsafe impl objc2::encode::RefEncode for vImageWhitePoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vImageWhitePoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vImageWhitePoint", &[]);
}
