#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueVDADecoder {
    _unused: [u8; 0],
}
pub type VDADecoder = *mut OpaqueVDADecoder;
pub type VDADecoderOutputCallback = ::std::option::Option<
    unsafe extern "C" fn(
        decompressionOutputRefCon: *mut ::std::os::raw::c_void,
        frameInfo: CFDictionaryRef,
        status: OSStatus,
        infoFlags: u32,
        imageBuffer: CVImageBufferRef,
    ),
>;
unsafe extern "C" {
    pub static kVDADecoderConfiguration_Height: CFStringRef;
}
unsafe extern "C" {
    pub static kVDADecoderConfiguration_Width: CFStringRef;
}
unsafe extern "C" {
    pub static kVDADecoderConfiguration_SourceFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kVDADecoderConfiguration_avcCData: CFStringRef;
}
unsafe extern "C" {
    pub fn VDADecoderCreate(
        decoderConfiguration: CFDictionaryRef,
        destinationImageBufferAttributes: CFDictionaryRef,
        outputCallback: *mut VDADecoderOutputCallback,
        decoderOutputCallbackRefcon: *mut ::std::os::raw::c_void,
        decoderOut: *mut VDADecoder,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VDADecoderDecode(
        decoder: VDADecoder,
        decodeFlags: u32,
        compressedBuffer: CFTypeRef,
        frameInfo: CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn VDADecoderFlush(decoder: VDADecoder, flushFlags: u32) -> OSStatus;
}
unsafe extern "C" {
    pub fn VDADecoderDestroy(decoder: VDADecoder) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for OpaqueVDADecoder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueVDADecoder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueVDADecoder", &[]);
}
