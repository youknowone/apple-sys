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
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UIImage(pub id);
impl std::ops::Deref for UIImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UIImage {}
impl UIImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UIImage").unwrap(), alloc) })
    }
}
impl IUIImage for UIImage {}
pub trait IUIImage: Sized + std::ops::Deref {}
pub type QLThumbnailRepresentationType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLThumbnailRepresentation(pub id);
impl std::ops::Deref for QLThumbnailRepresentation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLThumbnailRepresentation {}
impl QLThumbnailRepresentation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailRepresentation").unwrap(), alloc) })
    }
}
impl INSObject for QLThumbnailRepresentation {}
impl PNSObject for QLThumbnailRepresentation {}
impl std::convert::TryFrom<NSObject> for QLThumbnailRepresentation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLThumbnailRepresentation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLThumbnailRepresentation").unwrap()) };
        if is_kind_of {
            Ok(QLThumbnailRepresentation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLThumbnailRepresentation")
        }
    }
}
impl IQLThumbnailRepresentation for QLThumbnailRepresentation {}
pub trait IQLThumbnailRepresentation: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> QLThumbnailRepresentationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn CGImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGImage)
    }
    unsafe fn UIImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UIImage)
    }
    unsafe fn NSImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, NSImage)
    }
    unsafe fn contentRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentRect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLThumbnailProvider(pub id);
impl std::ops::Deref for QLThumbnailProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLThumbnailProvider {}
impl QLThumbnailProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailProvider").unwrap(), alloc) })
    }
}
impl INSObject for QLThumbnailProvider {}
impl PNSObject for QLThumbnailProvider {}
impl std::convert::TryFrom<NSObject> for QLThumbnailProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLThumbnailProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLThumbnailProvider").unwrap()) };
        if is_kind_of {
            Ok(QLThumbnailProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLThumbnailProvider")
        }
    }
}
impl IQLThumbnailProvider for QLThumbnailProvider {}
pub trait IQLThumbnailProvider: Sized + std::ops::Deref {
    unsafe fn provideThumbnailForFileRequest_completionHandler_(
        &self,
        request: QLFileThumbnailRequest,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provideThumbnailForFileRequest : request, completionHandler : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLThumbnailReply(pub id);
impl std::ops::Deref for QLThumbnailReply {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLThumbnailReply {}
impl QLThumbnailReply {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailReply").unwrap(), alloc) })
    }
}
impl INSObject for QLThumbnailReply {}
impl PNSObject for QLThumbnailReply {}
impl std::convert::TryFrom<NSObject> for QLThumbnailReply {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLThumbnailReply, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLThumbnailReply").unwrap()) };
        if is_kind_of {
            Ok(QLThumbnailReply(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLThumbnailReply")
        }
    }
}
impl IQLThumbnailReply for QLThumbnailReply {}
pub trait IQLThumbnailReply: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn extensionBadge(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionBadge)
    }
    unsafe fn setExtensionBadge_(&self, extensionBadge: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtensionBadge : extensionBadge)
    }
    unsafe fn replyWithContextSize_drawingBlock_(
        contextSize: CGSize,
        drawingBlock: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailReply").unwrap(), replyWithContextSize : contextSize, drawingBlock : drawingBlock)
    }
    unsafe fn replyWithContextSize_currentContextDrawingBlock_(
        contextSize: CGSize,
        drawingBlock: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailReply").unwrap(), replyWithContextSize : contextSize, currentContextDrawingBlock : drawingBlock)
    }
    unsafe fn replyWithImageFileURL_(fileURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailReply").unwrap(), replyWithImageFileURL : fileURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLFileThumbnailRequest(pub id);
impl std::ops::Deref for QLFileThumbnailRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLFileThumbnailRequest {}
impl QLFileThumbnailRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLFileThumbnailRequest").unwrap(), alloc) })
    }
}
impl INSObject for QLFileThumbnailRequest {}
impl PNSObject for QLFileThumbnailRequest {}
impl std::convert::TryFrom<NSObject> for QLFileThumbnailRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLFileThumbnailRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLFileThumbnailRequest").unwrap()) };
        if is_kind_of {
            Ok(QLFileThumbnailRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLFileThumbnailRequest")
        }
    }
}
impl IQLFileThumbnailRequest for QLFileThumbnailRequest {}
pub trait IQLFileThumbnailRequest: Sized + std::ops::Deref {
    unsafe fn maximumSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumSize)
    }
    unsafe fn minimumSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumSize)
    }
    unsafe fn scale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn fileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileURL)
    }
}
pub type QLThumbnailGenerationRequestRepresentationTypes = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLThumbnailGenerationRequest(pub id);
impl std::ops::Deref for QLThumbnailGenerationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLThumbnailGenerationRequest {}
impl QLThumbnailGenerationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailGenerationRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for QLThumbnailGenerationRequest {}
impl PNSSecureCoding for QLThumbnailGenerationRequest {}
impl INSObject for QLThumbnailGenerationRequest {}
impl PNSObject for QLThumbnailGenerationRequest {}
impl std::convert::TryFrom<NSObject> for QLThumbnailGenerationRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLThumbnailGenerationRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLThumbnailGenerationRequest").unwrap()) };
        if is_kind_of {
            Ok(QLThumbnailGenerationRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLThumbnailGenerationRequest")
        }
    }
}
impl IQLThumbnailGenerationRequest for QLThumbnailGenerationRequest {}
pub trait IQLThumbnailGenerationRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFileAtURL_size_scale_representationTypes_(
        &self,
        url: NSURL,
        size: CGSize,
        scale: CGFloat,
        representationTypes: QLThumbnailGenerationRequestRepresentationTypes,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileAtURL : url, size : size, scale : scale, representationTypes : representationTypes)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
    unsafe fn setContentType_(&self, contentType: UTType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentType : contentType)
    }
    unsafe fn minimumDimension(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumDimension)
    }
    unsafe fn setMinimumDimension_(&self, minimumDimension: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumDimension : minimumDimension)
    }
    unsafe fn iconMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iconMode)
    }
    unsafe fn setIconMode_(&self, iconMode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIconMode : iconMode)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn scale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn representationTypes(&self) -> QLThumbnailGenerationRequestRepresentationTypes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, representationTypes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailGenerationRequest").unwrap(), new)
    }
}
pub type QLThumbnailError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QLThumbnailGenerator(pub id);
impl std::ops::Deref for QLThumbnailGenerator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QLThumbnailGenerator {}
impl QLThumbnailGenerator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailGenerator").unwrap(), alloc) })
    }
}
impl INSObject for QLThumbnailGenerator {}
impl PNSObject for QLThumbnailGenerator {}
impl std::convert::TryFrom<NSObject> for QLThumbnailGenerator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QLThumbnailGenerator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QLThumbnailGenerator").unwrap()) };
        if is_kind_of {
            Ok(QLThumbnailGenerator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QLThumbnailGenerator")
        }
    }
}
impl IQLThumbnailGenerator for QLThumbnailGenerator {}
pub trait IQLThumbnailGenerator: Sized + std::ops::Deref {
    unsafe fn generateBestRepresentationForRequest_completionHandler_(
        &self,
        request: QLThumbnailGenerationRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateBestRepresentationForRequest : request, completionHandler : completionHandler)
    }
    unsafe fn generateRepresentationsForRequest_updateHandler_(
        &self,
        request: QLThumbnailGenerationRequest,
        updateHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateRepresentationsForRequest : request, updateHandler : updateHandler)
    }
    unsafe fn cancelRequest_(&self, request: QLThumbnailGenerationRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelRequest : request)
    }
    unsafe fn saveBestRepresentationForRequest_toFileAtURL_asContentType_completionHandler_(
        &self,
        request: QLThumbnailGenerationRequest,
        fileURL: NSURL,
        contentType: UTType,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveBestRepresentationForRequest : request, toFileAtURL : fileURL, asContentType : contentType, completionHandler : completionHandler)
    }
    unsafe fn saveBestRepresentationForRequest_toFileAtURL_withContentType_completionHandler_(
        &self,
        request: QLThumbnailGenerationRequest,
        fileURL: NSURL,
        contentType: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveBestRepresentationForRequest : request, toFileAtURL : fileURL, withContentType : contentType, completionHandler : completionHandler)
    }
    unsafe fn sharedGenerator() -> QLThumbnailGenerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QLThumbnailGenerator").unwrap(), sharedGenerator)
    }
}
unsafe extern "C" {
    pub static QLThumbnailErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for UIImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UIImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLThumbnailRepresentation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLThumbnailRepresentation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLThumbnailProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLThumbnailProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLThumbnailReply {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLThumbnailReply {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLFileThumbnailRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLFileThumbnailRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLThumbnailGenerationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLThumbnailGenerationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QLThumbnailGenerator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QLThumbnailGenerator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
