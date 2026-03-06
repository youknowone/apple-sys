#[allow(unused_imports)]
use crate::CoreFoundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __QLPreviewRequest {
    _unused: [u8; 0],
}
pub type QLPreviewRequestRef = *mut __QLPreviewRequest;
unsafe extern "C" {
    pub static kQLPreviewPropertyWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub fn QLPreviewRequestIsCancelled(preview: QLPreviewRequestRef) -> Boolean;
}
unsafe extern "C" {
    pub fn QLPreviewRequestSetDataRepresentation(
        preview: QLPreviewRequestRef,
        data: CFDataRef,
        contentTypeUTI: CFStringRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub static kQLPreviewPropertyMIMETypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyTextEncodingNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyAttachmentDataKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyAttachmentsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewContentIDScheme: CFStringRef;
}

unsafe impl objc2::encode::RefEncode for __QLPreviewRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __QLPreviewRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__QLPreviewRequest", &[]);
}
