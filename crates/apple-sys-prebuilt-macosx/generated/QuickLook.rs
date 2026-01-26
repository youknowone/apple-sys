#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::IOKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __QLThumbnailRequest {
    _unused: [u8; 0],
}
pub type QLThumbnailRequestRef = *mut __QLThumbnailRequest;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __QLPreviewRequest {
    _unused: [u8; 0],
}
pub type QLPreviewRequestRef = *mut __QLPreviewRequest;
pub type QLPreviewPDFStyle = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QLGeneratorInterfaceStruct {
    pub _reserved: *mut ::std::os::raw::c_void,
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            thisPointer: *mut ::std::os::raw::c_void,
            iid: REFIID,
            ppv: *mut LPVOID,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(thisPointer: *mut ::std::os::raw::c_void) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(thisPointer: *mut ::std::os::raw::c_void) -> ULONG,
    >,
    pub GenerateThumbnailForURL: ::std::option::Option<
        unsafe extern "C" fn(
            thisInterface: *mut ::std::os::raw::c_void,
            thumbnail: QLThumbnailRequestRef,
            url: CFURLRef,
            contentTypeUTI: CFStringRef,
            options: CFDictionaryRef,
            maxSize: CGSize,
        ) -> OSStatus,
    >,
    pub CancelThumbnailGeneration: ::std::option::Option<
        unsafe extern "C" fn(
            thisInterface: *mut ::std::os::raw::c_void,
            thumbnail: QLThumbnailRequestRef,
        ),
    >,
    pub GeneratePreviewForURL: ::std::option::Option<
        unsafe extern "C" fn(
            thisInterface: *mut ::std::os::raw::c_void,
            preview: QLPreviewRequestRef,
            url: CFURLRef,
            contentTypeUTI: CFStringRef,
            options: CFDictionaryRef,
        ) -> OSStatus,
    >,
    pub CancelPreviewGeneration: ::std::option::Option<
        unsafe extern "C" fn(
            thisInterface: *mut ::std::os::raw::c_void,
            preview: QLPreviewRequestRef,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __QLThumbnail {
    _unused: [u8; 0],
}
pub type QLThumbnailRef = *mut __QLThumbnail;
unsafe extern "C" {
    pub fn QLThumbnailRequestGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn QLThumbnailRequestCopyURL(thumbnail: QLThumbnailRequestRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn QLThumbnailRequestCopyOptions(thumbnail: QLThumbnailRequestRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn QLThumbnailRequestCopyContentUTI(thumbnail: QLThumbnailRequestRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn QLThumbnailRequestGetMaximumSize(thumbnail: QLThumbnailRequestRef) -> CGSize;
}
unsafe extern "C" {
    pub fn QLThumbnailRequestGetGeneratorBundle(thumbnail: QLThumbnailRequestRef) -> CFBundleRef;
}
unsafe extern "C" {
    pub fn QLThumbnailRequestSetDocumentObject(
        thumbnail: QLThumbnailRequestRef,
        object: *const ::std::os::raw::c_void,
        callbacks: *const CFArrayCallBacks,
    );
}
unsafe extern "C" {
    pub fn QLThumbnailRequestGetDocumentObject(
        thumbnail: QLThumbnailRequestRef,
    ) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn QLThumbnailRequestSetImage(
        thumbnail: QLThumbnailRequestRef,
        image: CGImageRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn QLThumbnailRequestSetImageWithData(
        thumbnail: QLThumbnailRequestRef,
        data: CFDataRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn QLThumbnailRequestCreateContext(
        thumbnail: QLThumbnailRequestRef,
        size: CGSize,
        isBitmap: Boolean,
        properties: CFDictionaryRef,
    ) -> CGContextRef;
}
unsafe extern "C" {
    pub fn QLThumbnailRequestFlushContext(thumbnail: QLThumbnailRequestRef, context: CGContextRef);
}
unsafe extern "C" {
    pub fn QLThumbnailRequestSetImageAtURL(
        thumbnail: QLThumbnailRequestRef,
        url: CFURLRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn QLThumbnailRequestSetThumbnailWithDataRepresentation(
        thumbnail: QLThumbnailRequestRef,
        data: CFDataRef,
        contentTypeUTI: CFStringRef,
        previewProperties: CFDictionaryRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn QLThumbnailRequestSetThumbnailWithURLRepresentation(
        thumbnail: QLThumbnailRequestRef,
        url: CFURLRef,
        contentTypeUTI: CFStringRef,
        previewProperties: CFDictionaryRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn QLThumbnailRequestIsCancelled(thumbnail: QLThumbnailRequestRef) -> Boolean;
}
unsafe extern "C" {
    pub static kQLThumbnailPropertyExtensionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLThumbnailPropertyBadgeImageKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLThumbnailPropertyBaseBundlePathKey: CFStringRef;
}
unsafe extern "C" {
    pub fn QLPreviewRequestGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyDisplayNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyBaseBundlePathKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyStringEncodingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyPDFStyleKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewOptionCursorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLPreviewPropertyCursorKey: CFStringRef;
}
unsafe extern "C" {
    pub fn QLPreviewRequestCopyURL(preview: QLPreviewRequestRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn QLPreviewRequestCopyOptions(preview: QLPreviewRequestRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn QLPreviewRequestCopyContentUTI(preview: QLPreviewRequestRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn QLPreviewRequestGetGeneratorBundle(preview: QLPreviewRequestRef) -> CFBundleRef;
}
unsafe extern "C" {
    pub fn QLPreviewRequestSetDocumentObject(
        preview: QLPreviewRequestRef,
        object: *const ::std::os::raw::c_void,
        callbacks: *const CFArrayCallBacks,
    );
}
unsafe extern "C" {
    pub fn QLPreviewRequestGetDocumentObject(
        preview: QLPreviewRequestRef,
    ) -> *const ::std::os::raw::c_void;
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
    pub fn QLPreviewRequestSetURLRepresentation(
        preview: QLPreviewRequestRef,
        url: CFURLRef,
        contentTypeUTI: CFStringRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn QLPreviewRequestCreateContext(
        preview: QLPreviewRequestRef,
        size: CGSize,
        isBitmap: Boolean,
        properties: CFDictionaryRef,
    ) -> CGContextRef;
}
unsafe extern "C" {
    pub fn QLPreviewRequestCreatePDFContext(
        preview: QLPreviewRequestRef,
        mediaBox: *const CGRect,
        auxiliaryInfo: CFDictionaryRef,
        properties: CFDictionaryRef,
    ) -> CGContextRef;
}
unsafe extern "C" {
    pub fn QLPreviewRequestFlushContext(preview: QLPreviewRequestRef, context: CGContextRef);
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
unsafe extern "C" {
    pub fn QLThumbnailImageCreate(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        maxThumbnailSize: CGSize,
        options: CFDictionaryRef,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub static kQLThumbnailOptionIconModeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kQLThumbnailOptionScaleFactorKey: CFStringRef;
}
unsafe extern "C" {
    pub fn QLThumbnailGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn QLThumbnailCreate(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        maxThumbnailSize: CGSize,
        options: CFDictionaryRef,
    ) -> QLThumbnailRef;
}
unsafe extern "C" {
    pub fn QLThumbnailCopyDocumentURL(thumbnail: QLThumbnailRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn QLThumbnailGetMaximumSize(thumbnail: QLThumbnailRef) -> CGSize;
}
unsafe extern "C" {
    pub fn QLThumbnailCopyOptions(thumbnail: QLThumbnailRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn QLThumbnailDispatchAsync(
        thumbnail: QLThumbnailRef,
        queue: NSObject,
        completion: dispatch_block_t,
    );
}
unsafe extern "C" {
    pub fn QLThumbnailCopyImage(thumbnail: QLThumbnailRef) -> CGImageRef;
}
unsafe extern "C" {
    pub fn QLThumbnailGetContentRect(thumbnail: QLThumbnailRef) -> CGRect;
}
unsafe extern "C" {
    pub fn QLThumbnailCancel(thumbnail: QLThumbnailRef);
}
unsafe extern "C" {
    pub fn QLThumbnailIsCancelled(thumbnail: QLThumbnailRef) -> Boolean;
}

unsafe impl objc2::encode::RefEncode for __QLThumbnailRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __QLThumbnailRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__QLThumbnailRequest", &[]);
}
unsafe impl objc2::encode::RefEncode for __QLPreviewRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __QLPreviewRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__QLPreviewRequest", &[]);
}
unsafe impl objc2::encode::RefEncode for QLGeneratorInterfaceStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLGeneratorInterfaceStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("QLGeneratorInterfaceStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for __QLThumbnail {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __QLThumbnail {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__QLThumbnail", &[]);
}
