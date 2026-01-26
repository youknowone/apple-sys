#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreImage::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::ImageCaptureCore::*;
#[allow(unused_imports)]
use crate::OpenGL::*;
#[allow(unused_imports)]
use crate::QuartzCore::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type QCPlugInExecutionMode = ::std::os::raw::c_uint;
pub type QCPlugInTimeMode = ::std::os::raw::c_uint;
pub type QCPlugInBufferReleaseCallback = ::std::option::Option<
    unsafe extern "C" fn(
        address: *const ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    ),
>;
pub type QCPlugInTextureReleaseCallback = ::std::option::Option<
    unsafe extern "C" fn(
        cgl_ctx: CGLContextObj,
        name: GLuint,
        context: *mut ::std::os::raw::c_void,
    ),
>;
pub trait PQCPlugInContext: Sized + std::ops::Deref {
    unsafe fn compositionURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compositionURL)
    }
    unsafe fn logMessage_(&self, format: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logMessage : format)
    }
    unsafe fn userInfo(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn bounds(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn CGLContextObj(&self) -> CGLContextObj
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGLContextObj)
    }
    unsafe fn outputImageProviderFromBufferWithPixelFormat_pixelsWide_pixelsHigh_baseAddress_bytesPerRow_releaseCallback_releaseContext_colorSpace_shouldColorMatch_(
        &self,
        format: NSString,
        width: NSUInteger,
        height: NSUInteger,
        baseAddress: *const ::std::os::raw::c_void,
        rowBytes: NSUInteger,
        callback: QCPlugInBufferReleaseCallback,
        context: *mut ::std::os::raw::c_void,
        colorSpace: CGColorSpaceRef,
        colorMatch: BOOL,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, outputImageProviderFromBufferWithPixelFormat : format, pixelsWide : width, pixelsHigh : height, baseAddress : baseAddress, bytesPerRow : rowBytes, releaseCallback : callback, releaseContext : context, colorSpace : colorSpace, shouldColorMatch : colorMatch)
    }
    unsafe fn outputImageProviderFromTextureWithPixelFormat_pixelsWide_pixelsHigh_name_flipped_releaseCallback_releaseContext_colorSpace_shouldColorMatch_(
        &self,
        format: NSString,
        width: NSUInteger,
        height: NSUInteger,
        name: GLuint,
        flipped: BOOL,
        callback: QCPlugInTextureReleaseCallback,
        context: *mut ::std::os::raw::c_void,
        colorSpace: CGColorSpaceRef,
        colorMatch: BOOL,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, outputImageProviderFromTextureWithPixelFormat : format, pixelsWide : width, pixelsHigh : height, name : name, flipped : flipped, releaseCallback : callback, releaseContext : context, colorSpace : colorSpace, shouldColorMatch : colorMatch)
    }
}
pub trait PQCPlugInInputImageSource: Sized + std::ops::Deref {
    unsafe fn imageBounds(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageBounds)
    }
    unsafe fn imageColorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageColorSpace)
    }
    unsafe fn shouldColorMatch(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldColorMatch)
    }
    unsafe fn lockBufferRepresentationWithPixelFormat_colorSpace_forBounds_(
        &self,
        format: NSString,
        colorSpace: CGColorSpaceRef,
        bounds: NSRect,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockBufferRepresentationWithPixelFormat : format, colorSpace : colorSpace, forBounds : bounds)
    }
    unsafe fn bufferPixelsWide(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferPixelsWide)
    }
    unsafe fn bufferPixelsHigh(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferPixelsHigh)
    }
    unsafe fn bufferPixelFormat(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferPixelFormat)
    }
    unsafe fn bufferColorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferColorSpace)
    }
    unsafe fn bufferBaseAddress(&self) -> *const ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferBaseAddress)
    }
    unsafe fn bufferBytesPerRow(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferBytesPerRow)
    }
    unsafe fn unlockBufferRepresentation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unlockBufferRepresentation)
    }
    unsafe fn lockTextureRepresentationWithColorSpace_forBounds_(
        &self,
        colorSpace: CGColorSpaceRef,
        bounds: NSRect,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockTextureRepresentationWithColorSpace : colorSpace, forBounds : bounds)
    }
    unsafe fn texturePixelsWide(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texturePixelsWide)
    }
    unsafe fn texturePixelsHigh(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texturePixelsHigh)
    }
    unsafe fn textureTarget(&self) -> GLenum
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureTarget)
    }
    unsafe fn textureName(&self) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureName)
    }
    unsafe fn textureColorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureColorSpace)
    }
    unsafe fn textureFlipped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureFlipped)
    }
    unsafe fn textureMatrix(&self) -> *const GLfloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureMatrix)
    }
    unsafe fn bindTextureRepresentationToCGLContext_textureUnit_normalizeCoordinates_(
        &self,
        cgl_ctx: CGLContextObj,
        unit: GLenum,
        flag: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bindTextureRepresentationToCGLContext : cgl_ctx, textureUnit : unit, normalizeCoordinates : flag)
    }
    unsafe fn unbindTextureRepresentationFromCGLContext_textureUnit_(
        &self,
        cgl_ctx: CGLContextObj,
        unit: GLenum,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unbindTextureRepresentationFromCGLContext : cgl_ctx, textureUnit : unit)
    }
    unsafe fn unlockTextureRepresentation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unlockTextureRepresentation)
    }
}
pub trait PQCPlugInOutputImageProvider: Sized + std::ops::Deref {
    unsafe fn imageBounds(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageBounds)
    }
    unsafe fn imageColorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageColorSpace)
    }
    unsafe fn shouldColorMatch(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldColorMatch)
    }
    unsafe fn supportedBufferPixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedBufferPixelFormats)
    }
    unsafe fn renderToBuffer_withBytesPerRow_pixelFormat_forBounds_(
        &self,
        baseAddress: *mut ::std::os::raw::c_void,
        rowBytes: NSUInteger,
        format: NSString,
        bounds: NSRect,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderToBuffer : baseAddress, withBytesPerRow : rowBytes, pixelFormat : format, forBounds : bounds)
    }
    unsafe fn supportedRenderedTexturePixelFormats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedRenderedTexturePixelFormats)
    }
    unsafe fn copyRenderedTextureForCGLContext_pixelFormat_bounds_isFlipped_(
        &self,
        cgl_ctx: CGLContextObj,
        format: NSString,
        bounds: NSRect,
        flipped: *mut BOOL,
    ) -> GLuint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyRenderedTextureForCGLContext : cgl_ctx, pixelFormat : format, bounds : bounds, isFlipped : flipped)
    }
    unsafe fn releaseRenderedTexture_forCGLContext_(&self, name: GLuint, cgl_ctx: CGLContextObj)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, releaseRenderedTexture : name, forCGLContext : cgl_ctx)
    }
    unsafe fn canRenderWithCGLContext_(&self, cgl_ctx: CGLContextObj) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canRenderWithCGLContext : cgl_ctx)
    }
    unsafe fn renderWithCGLContext_forBounds_(&self, cgl_ctx: CGLContextObj, bounds: NSRect) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderWithCGLContext : cgl_ctx, forBounds : bounds)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCPlugIn(pub id);
impl std::ops::Deref for QCPlugIn {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCPlugIn {}
impl QCPlugIn {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap(), alloc) })
    }
}
impl INSObject for QCPlugIn {}
impl PNSObject for QCPlugIn {}
impl std::convert::TryFrom<NSObject> for QCPlugIn {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QCPlugIn, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap()) };
        if is_kind_of {
            Ok(QCPlugIn(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QCPlugIn")
        }
    }
}
impl IQCPlugIn for QCPlugIn {}
pub trait IQCPlugIn: Sized + std::ops::Deref {
    unsafe fn startExecution_(&self, context: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startExecution : context)
    }
    unsafe fn enableExecution_(&self, context: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enableExecution : context)
    }
    unsafe fn executionTimeForContext_atTime_withArguments_(
        &self,
        context: *mut u64,
        time: NSTimeInterval,
        arguments: NSDictionary,
    ) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executionTimeForContext : context, atTime : time, withArguments : arguments)
    }
    unsafe fn execute_atTime_withArguments_(
        &self,
        context: *mut u64,
        time: NSTimeInterval,
        arguments: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, execute : context, atTime : time, withArguments : arguments)
    }
    unsafe fn disableExecution_(&self, context: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disableExecution : context)
    }
    unsafe fn stopExecution_(&self, context: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopExecution : context)
    }
    unsafe fn serializedValueForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, serializedValueForKey : key)
    }
    unsafe fn setSerializedValue_forKey_(&self, serializedValue: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSerializedValue : serializedValue, forKey : key)
    }
    unsafe fn attributes() -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap(), attributes)
    }
    unsafe fn attributesForPropertyPortWithKey_(key: NSString) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap(), attributesForPropertyPortWithKey : key)
    }
    unsafe fn sortedPropertyPortKeys() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap(), sortedPropertyPortKeys)
    }
    unsafe fn executionMode() -> QCPlugInExecutionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap(), executionMode)
    }
    unsafe fn timeMode() -> QCPlugInTimeMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap(), timeMode)
    }
    unsafe fn plugInKeys() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap(), plugInKeys)
    }
}
impl QCPlugIn_Ports for QCPlugIn {}
pub trait QCPlugIn_Ports: Sized + std::ops::Deref {
    unsafe fn didValueForInputKeyChange_(&self, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didValueForInputKeyChange : key)
    }
    unsafe fn valueForInputKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForInputKey : key)
    }
    unsafe fn setValue_forOutputKey_(&self, value: id, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forOutputKey : key)
    }
    unsafe fn addInputPortWithType_forKey_withAttributes_(
        &self,
        type_: NSString,
        key: NSString,
        attributes: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addInputPortWithType : type_, forKey : key, withAttributes : attributes)
    }
    unsafe fn removeInputPortForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeInputPortForKey : key)
    }
    unsafe fn addOutputPortWithType_forKey_withAttributes_(
        &self,
        type_: NSString,
        key: NSString,
        attributes: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOutputPortWithType : type_, forKey : key, withAttributes : attributes)
    }
    unsafe fn removeOutputPortForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeOutputPortForKey : key)
    }
}
impl QCPlugIn_Registry for QCPlugIn {}
pub trait QCPlugIn_Registry: Sized + std::ops::Deref {
    unsafe fn loadPlugInAtPath_(path: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap(), loadPlugInAtPath : path)
    }
    unsafe fn registerPlugInClass_(aClass: Class)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugIn").unwrap(), registerPlugInClass : aClass)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCComposition(pub id);
impl std::ops::Deref for QCComposition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCComposition {}
impl QCComposition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCComposition").unwrap(), alloc) })
    }
}
impl PNSCopying for QCComposition {}
impl INSObject for QCComposition {}
impl PNSObject for QCComposition {}
impl std::convert::TryFrom<NSObject> for QCComposition {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QCComposition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCComposition").unwrap()) };
        if is_kind_of {
            Ok(QCComposition(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QCComposition")
        }
    }
}
impl IQCComposition for QCComposition {}
pub trait IQCComposition: Sized + std::ops::Deref {
    unsafe fn protocols(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocols)
    }
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
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
    unsafe fn compositionWithFile_(path: NSString) -> QCComposition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCComposition").unwrap(), compositionWithFile : path)
    }
    unsafe fn compositionWithData_(data: NSData) -> QCComposition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCComposition").unwrap(), compositionWithData : data)
    }
}
impl QCComposition_QCCompositionRepository for QCComposition {}
pub trait QCComposition_QCCompositionRepository: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCCompositionRepository(pub id);
impl std::ops::Deref for QCCompositionRepository {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCCompositionRepository {}
impl QCCompositionRepository {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCCompositionRepository").unwrap(), alloc) })
    }
}
impl INSObject for QCCompositionRepository {}
impl PNSObject for QCCompositionRepository {}
impl std::convert::TryFrom<NSObject> for QCCompositionRepository {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QCCompositionRepository, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCCompositionRepository").unwrap()) };
        if is_kind_of {
            Ok(QCCompositionRepository(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QCCompositionRepository")
        }
    }
}
impl IQCCompositionRepository for QCCompositionRepository {}
pub trait IQCCompositionRepository: Sized + std::ops::Deref {
    unsafe fn compositionWithIdentifier_(&self, identifier: NSString) -> QCComposition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositionWithIdentifier : identifier)
    }
    unsafe fn compositionsWithProtocols_andAttributes_(
        &self,
        protocols: NSArray,
        attributes: NSDictionary,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositionsWithProtocols : protocols, andAttributes : attributes)
    }
    unsafe fn allCompositions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allCompositions)
    }
    unsafe fn sharedCompositionRepository() -> QCCompositionRepository
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCCompositionRepository").unwrap(), sharedCompositionRepository)
    }
}
pub trait PQCCompositionRenderer: Sized + std::ops::Deref {
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
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
    unsafe fn setValue_forInputKey_(&self, value: id, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forInputKey : key)
    }
    unsafe fn valueForInputKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForInputKey : key)
    }
    unsafe fn valueForOutputKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForOutputKey : key)
    }
    unsafe fn valueForOutputKey_ofType_(&self, key: NSString, type_: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForOutputKey : key, ofType : type_)
    }
    unsafe fn propertyListFromInputValues(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertyListFromInputValues)
    }
    unsafe fn setInputValuesWithPropertyList_(&self, plist: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputValuesWithPropertyList : plist)
    }
    unsafe fn userInfo(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCRenderer(pub id);
impl std::ops::Deref for QCRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCRenderer {}
impl QCRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCRenderer").unwrap(), alloc) })
    }
}
impl PQCCompositionRenderer for QCRenderer {}
impl INSObject for QCRenderer {}
impl PNSObject for QCRenderer {}
impl std::convert::TryFrom<NSObject> for QCRenderer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QCRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCRenderer").unwrap()) };
        if is_kind_of {
            Ok(QCRenderer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QCRenderer")
        }
    }
}
impl IQCRenderer for QCRenderer {}
pub trait IQCRenderer: Sized + std::ops::Deref {
    unsafe fn initWithComposition_colorSpace_(
        &self,
        composition: QCComposition,
        colorSpace: CGColorSpaceRef,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithComposition : composition, colorSpace : colorSpace)
    }
    unsafe fn initWithCGLContext_pixelFormat_colorSpace_composition_(
        &self,
        context: CGLContextObj,
        format: CGLPixelFormatObj,
        colorSpace: CGColorSpaceRef,
        composition: QCComposition,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGLContext : context, pixelFormat : format, colorSpace : colorSpace, composition : composition)
    }
    unsafe fn initOffScreenWithSize_colorSpace_composition_(
        &self,
        size: NSSize,
        colorSpace: CGColorSpaceRef,
        composition: QCComposition,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initOffScreenWithSize : size, colorSpace : colorSpace, composition : composition)
    }
    unsafe fn initWithOpenGLContext_pixelFormat_file_(
        &self,
        context: NSOpenGLContext,
        format: NSOpenGLPixelFormat,
        path: NSString,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOpenGLContext : context, pixelFormat : format, file : path)
    }
    unsafe fn renderAtTime_arguments_(&self, time: NSTimeInterval, arguments: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderAtTime : time, arguments : arguments)
    }
    unsafe fn renderingTimeForTime_arguments_(
        &self,
        time: NSTimeInterval,
        arguments: NSDictionary,
    ) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderingTimeForTime : time, arguments : arguments)
    }
    unsafe fn composition(&self) -> QCComposition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composition)
    }
    unsafe fn snapshotImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshotImage)
    }
    unsafe fn createSnapshotImageOfType_(&self, type_: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createSnapshotImageOfType : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCCompositionLayer(pub id);
impl std::ops::Deref for QCCompositionLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCCompositionLayer {}
impl QCCompositionLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCCompositionLayer").unwrap(), alloc) })
    }
}
impl PQCCompositionRenderer for QCCompositionLayer {}
impl ICAOpenGLLayer for QCCompositionLayer {}
impl std::convert::TryFrom<CAOpenGLLayer> for QCCompositionLayer {
    type Error = &'static str;
    fn try_from(parent: CAOpenGLLayer) -> Result<QCCompositionLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCCompositionLayer").unwrap()) };
        if is_kind_of {
            Ok(QCCompositionLayer(parent.0))
        } else {
            Err("This CAOpenGLLayer cannot be downcasted to QCCompositionLayer")
        }
    }
}
impl ICALayer for QCCompositionLayer {}
impl PNSSecureCoding for QCCompositionLayer {}
impl PCAMediaTiming for QCCompositionLayer {}
impl INSObject for QCCompositionLayer {}
impl PNSObject for QCCompositionLayer {}
impl IQCCompositionLayer for QCCompositionLayer {}
pub trait IQCCompositionLayer: Sized + std::ops::Deref {
    unsafe fn initWithFile_(&self, path: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFile : path)
    }
    unsafe fn initWithComposition_(&self, composition: QCComposition) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithComposition : composition)
    }
    unsafe fn composition(&self) -> QCComposition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composition)
    }
    unsafe fn compositionLayerWithFile_(path: NSString) -> QCCompositionLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCCompositionLayer").unwrap(), compositionLayerWithFile : path)
    }
    unsafe fn compositionLayerWithComposition_(composition: QCComposition) -> QCCompositionLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCCompositionLayer").unwrap(), compositionLayerWithComposition : composition)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCView(pub id);
impl std::ops::Deref for QCView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCView {}
impl QCView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCView").unwrap(), alloc) })
    }
}
impl PQCCompositionRenderer for QCView {}
impl INSView for QCView {}
impl PNSAnimatablePropertyContainer for QCView {}
impl PNSUserInterfaceItemIdentification for QCView {}
impl PNSDraggingDestination for QCView {}
impl PNSAppearanceCustomization for QCView {}
impl PNSAccessibilityElement for QCView {}
impl PNSAccessibility for QCView {}
impl std::convert::TryFrom<NSView> for QCView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<QCView, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCView").unwrap()) };
        if is_kind_of {
            Ok(QCView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to QCView")
        }
    }
}
impl INSResponder for QCView {}
impl PNSCoding for QCView {}
impl INSObject for QCView {}
impl PNSObject for QCView {}
impl IQCView for QCView {}
pub trait IQCView: Sized + std::ops::Deref {
    unsafe fn loadCompositionFromFile_(&self, path: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadCompositionFromFile : path)
    }
    unsafe fn loadComposition_(&self, composition: QCComposition) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadComposition : composition)
    }
    unsafe fn loadedComposition(&self) -> QCComposition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loadedComposition)
    }
    unsafe fn unloadComposition(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unloadComposition)
    }
    unsafe fn setAutostartsRendering_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutostartsRendering : flag)
    }
    unsafe fn autostartsRendering(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autostartsRendering)
    }
    unsafe fn setEraseColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEraseColor : color)
    }
    unsafe fn eraseColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eraseColor)
    }
    unsafe fn setEventForwardingMask_(&self, mask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEventForwardingMask : mask)
    }
    unsafe fn eventForwardingMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventForwardingMask)
    }
    unsafe fn setMaxRenderingFrameRate_(&self, maxFPS: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxRenderingFrameRate : maxFPS)
    }
    unsafe fn maxRenderingFrameRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxRenderingFrameRate)
    }
    unsafe fn erase(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, erase)
    }
    unsafe fn startRendering(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startRendering)
    }
    unsafe fn renderAtTime_arguments_(&self, time: NSTimeInterval, arguments: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderAtTime : time, arguments : arguments)
    }
    unsafe fn pauseRendering(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pauseRendering)
    }
    unsafe fn isPausedRendering(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPausedRendering)
    }
    unsafe fn resumeRendering(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resumeRendering)
    }
    unsafe fn stopRendering(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopRendering)
    }
    unsafe fn isRendering(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRendering)
    }
    unsafe fn snapshotImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, snapshotImage)
    }
    unsafe fn createSnapshotImageOfType_(&self, type_: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createSnapshotImageOfType : type_)
    }
    unsafe fn openGLContext(&self) -> NSOpenGLContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, openGLContext)
    }
    unsafe fn openGLPixelFormat(&self) -> NSOpenGLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, openGLPixelFormat)
    }
}
impl QCView_IBExtensions for QCView {}
pub trait QCView_IBExtensions: Sized + std::ops::Deref {
    unsafe fn start_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, start : sender)
    }
    unsafe fn stop_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stop : sender)
    }
    unsafe fn play_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, play : sender)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCPatchController(pub id);
impl std::ops::Deref for QCPatchController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCPatchController {}
impl QCPatchController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCPatchController").unwrap(), alloc) })
    }
}
impl INSController for QCPatchController {}
impl PNSCoding for QCPatchController {}
impl PNSEditor for QCPatchController {}
impl PNSEditorRegistration for QCPatchController {}
impl std::convert::TryFrom<NSController> for QCPatchController {
    type Error = &'static str;
    fn try_from(parent: NSController) -> Result<QCPatchController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCPatchController").unwrap()) };
        if is_kind_of {
            Ok(QCPatchController(parent.0))
        } else {
            Err("This NSController cannot be downcasted to QCPatchController")
        }
    }
}
impl INSObject for QCPatchController {}
impl PNSObject for QCPatchController {}
impl IQCPatchController for QCPatchController {}
pub trait IQCPatchController: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCCompositionParameterView(pub id);
impl std::ops::Deref for QCCompositionParameterView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCCompositionParameterView {}
impl QCCompositionParameterView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCCompositionParameterView").unwrap(), alloc) })
    }
}
impl INSView for QCCompositionParameterView {}
impl PNSAnimatablePropertyContainer for QCCompositionParameterView {}
impl PNSUserInterfaceItemIdentification for QCCompositionParameterView {}
impl PNSDraggingDestination for QCCompositionParameterView {}
impl PNSAppearanceCustomization for QCCompositionParameterView {}
impl PNSAccessibilityElement for QCCompositionParameterView {}
impl PNSAccessibility for QCCompositionParameterView {}
impl std::convert::TryFrom<NSView> for QCCompositionParameterView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<QCCompositionParameterView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCCompositionParameterView").unwrap()) };
        if is_kind_of {
            Ok(QCCompositionParameterView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to QCCompositionParameterView")
        }
    }
}
impl INSResponder for QCCompositionParameterView {}
impl PNSCoding for QCCompositionParameterView {}
impl INSObject for QCCompositionParameterView {}
impl PNSObject for QCCompositionParameterView {}
impl IQCCompositionParameterView for QCCompositionParameterView {}
pub trait IQCCompositionParameterView: Sized + std::ops::Deref {
    unsafe fn setCompositionRenderer_(&self, renderer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompositionRenderer : renderer)
    }
    unsafe fn compositionRenderer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compositionRenderer)
    }
    unsafe fn hasParameters(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasParameters)
    }
    unsafe fn setBackgroundColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : color)
    }
    unsafe fn backgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setDrawsBackground_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawsBackground : flag)
    }
    unsafe fn drawsBackground(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawsBackground)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
}
pub trait NSObject_QCCompositionParameterViewDelegate: Sized + std::ops::Deref {
    unsafe fn compositionParameterView_shouldDisplayParameterWithKey_attributes_(
        &self,
        parameterView: QCCompositionParameterView,
        portKey: NSString,
        portAttributes: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositionParameterView : parameterView, shouldDisplayParameterWithKey : portKey, attributes : portAttributes)
    }
    unsafe fn compositionParameterView_didChangeParameterWithKey_(
        &self,
        parameterView: QCCompositionParameterView,
        portKey: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositionParameterView : parameterView, didChangeParameterWithKey : portKey)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCCompositionPickerView(pub id);
impl std::ops::Deref for QCCompositionPickerView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCCompositionPickerView {}
impl QCCompositionPickerView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCCompositionPickerView").unwrap(), alloc) })
    }
}
impl INSView for QCCompositionPickerView {}
impl PNSAnimatablePropertyContainer for QCCompositionPickerView {}
impl PNSUserInterfaceItemIdentification for QCCompositionPickerView {}
impl PNSDraggingDestination for QCCompositionPickerView {}
impl PNSAppearanceCustomization for QCCompositionPickerView {}
impl PNSAccessibilityElement for QCCompositionPickerView {}
impl PNSAccessibility for QCCompositionPickerView {}
impl std::convert::TryFrom<NSView> for QCCompositionPickerView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<QCCompositionPickerView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCCompositionPickerView").unwrap()) };
        if is_kind_of {
            Ok(QCCompositionPickerView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to QCCompositionPickerView")
        }
    }
}
impl INSResponder for QCCompositionPickerView {}
impl PNSCoding for QCCompositionPickerView {}
impl INSObject for QCCompositionPickerView {}
impl PNSObject for QCCompositionPickerView {}
impl IQCCompositionPickerView for QCCompositionPickerView {}
pub trait IQCCompositionPickerView: Sized + std::ops::Deref {
    unsafe fn setCompositionsFromRepositoryWithProtocol_andAttributes_(
        &self,
        protocol: NSString,
        attributes: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompositionsFromRepositoryWithProtocol : protocol, andAttributes : attributes)
    }
    unsafe fn compositions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compositions)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setShowsCompositionNames_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsCompositionNames : flag)
    }
    unsafe fn showsCompositionNames(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsCompositionNames)
    }
    unsafe fn setAllowsEmptySelection_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsEmptySelection : flag)
    }
    unsafe fn allowsEmptySelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsEmptySelection)
    }
    unsafe fn setCompositionAspectRatio_(&self, ratio: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompositionAspectRatio : ratio)
    }
    unsafe fn compositionAspectRatio(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compositionAspectRatio)
    }
    unsafe fn setDefaultValue_forInputKey_(&self, value: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultValue : value, forInputKey : key)
    }
    unsafe fn resetDefaultInputValues(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetDefaultInputValues)
    }
    unsafe fn setSelectedComposition_(&self, composition: QCComposition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedComposition : composition)
    }
    unsafe fn selectedComposition(&self) -> QCComposition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedComposition)
    }
    unsafe fn startAnimation_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAnimation : sender)
    }
    unsafe fn stopAnimation_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopAnimation : sender)
    }
    unsafe fn isAnimating(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnimating)
    }
    unsafe fn setMaxAnimationFrameRate_(&self, maxFPS: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxAnimationFrameRate : maxFPS)
    }
    unsafe fn maxAnimationFrameRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxAnimationFrameRate)
    }
    unsafe fn setBackgroundColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : color)
    }
    unsafe fn backgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setDrawsBackground_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawsBackground : flag)
    }
    unsafe fn drawsBackground(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawsBackground)
    }
    unsafe fn numberOfColumns(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfColumns)
    }
    unsafe fn setNumberOfColumns_(&self, columns: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfColumns : columns)
    }
    unsafe fn numberOfRows(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfRows)
    }
    unsafe fn setNumberOfRows_(&self, rows: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfRows : rows)
    }
}
pub trait NSObject_QCCompositionPickerViewDelegate: Sized + std::ops::Deref {
    unsafe fn compositionPickerView_didSelectComposition_(
        &self,
        pickerView: QCCompositionPickerView,
        composition: QCComposition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositionPickerView : pickerView, didSelectComposition : composition)
    }
    unsafe fn compositionPickerViewDidStartAnimating_(&self, pickerView: QCCompositionPickerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositionPickerViewDidStartAnimating : pickerView)
    }
    unsafe fn compositionPickerViewWillStopAnimating_(&self, pickerView: QCCompositionPickerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositionPickerViewWillStopAnimating : pickerView)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCCompositionPickerPanel(pub id);
impl std::ops::Deref for QCCompositionPickerPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCCompositionPickerPanel {}
impl QCCompositionPickerPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCCompositionPickerPanel").unwrap(), alloc) })
    }
}
impl INSPanel for QCCompositionPickerPanel {}
impl std::convert::TryFrom<NSPanel> for QCCompositionPickerPanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<QCCompositionPickerPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCCompositionPickerPanel").unwrap()) };
        if is_kind_of {
            Ok(QCCompositionPickerPanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to QCCompositionPickerPanel")
        }
    }
}
impl INSWindow for QCCompositionPickerPanel {}
impl PNSAnimatablePropertyContainer for QCCompositionPickerPanel {}
impl PNSMenuItemValidation for QCCompositionPickerPanel {}
impl PNSUserInterfaceValidations for QCCompositionPickerPanel {}
impl PNSUserInterfaceItemIdentification for QCCompositionPickerPanel {}
impl PNSAppearanceCustomization for QCCompositionPickerPanel {}
impl PNSAccessibilityElement for QCCompositionPickerPanel {}
impl PNSAccessibility for QCCompositionPickerPanel {}
impl INSResponder for QCCompositionPickerPanel {}
impl PNSCoding for QCCompositionPickerPanel {}
impl INSObject for QCCompositionPickerPanel {}
impl PNSObject for QCCompositionPickerPanel {}
impl IQCCompositionPickerPanel for QCCompositionPickerPanel {}
pub trait IQCCompositionPickerPanel: Sized + std::ops::Deref {
    unsafe fn compositionPickerView(&self) -> QCCompositionPickerView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compositionPickerView)
    }
    unsafe fn sharedCompositionPickerPanel() -> QCCompositionPickerPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QCCompositionPickerPanel").unwrap(), sharedCompositionPickerPanel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QCPlugInViewController(pub id);
impl std::ops::Deref for QCPlugInViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QCPlugInViewController {}
impl QCPlugInViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QCPlugInViewController").unwrap(), alloc) })
    }
}
impl INSViewController for QCPlugInViewController {}
impl PNSEditor for QCPlugInViewController {}
impl PNSSeguePerforming for QCPlugInViewController {}
impl PNSUserInterfaceItemIdentification for QCPlugInViewController {}
impl std::convert::TryFrom<NSViewController> for QCPlugInViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<QCPlugInViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QCPlugInViewController").unwrap()) };
        if is_kind_of {
            Ok(QCPlugInViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to QCPlugInViewController")
        }
    }
}
impl INSResponder for QCPlugInViewController {}
impl PNSCoding for QCPlugInViewController {}
impl INSObject for QCPlugInViewController {}
impl PNSObject for QCPlugInViewController {}
impl IQCPlugInViewController for QCPlugInViewController {}
pub trait IQCPlugInViewController: Sized + std::ops::Deref {
    unsafe fn initWithPlugIn_viewNibName_(&self, plugIn: QCPlugIn, name: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlugIn : plugIn, viewNibName : name)
    }
    unsafe fn plugIn(&self) -> QCPlugIn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, plugIn)
    }
}
impl QCPlugIn_QCPlugInViewController for QCPlugIn {}
pub trait QCPlugIn_QCPlugInViewController: Sized + std::ops::Deref {
    unsafe fn createViewController(&self) -> QCPlugInViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createViewController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QuartzFilter(pub id);
impl std::ops::Deref for QuartzFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QuartzFilter {}
impl QuartzFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QuartzFilter").unwrap(), alloc) })
    }
}
impl INSObject for QuartzFilter {}
impl PNSObject for QuartzFilter {}
impl std::convert::TryFrom<NSObject> for QuartzFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QuartzFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QuartzFilter").unwrap()) };
        if is_kind_of {
            Ok(QuartzFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QuartzFilter")
        }
    }
}
impl IQuartzFilter for QuartzFilter {}
pub trait IQuartzFilter: Sized + std::ops::Deref {
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn applyToContext_(&self, aContext: CGContextRef) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyToContext : aContext)
    }
    unsafe fn removeFromContext_(&self, aContext: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromContext : aContext)
    }
    unsafe fn quartzFilterWithURL_(aURL: NSURL) -> QuartzFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QuartzFilter").unwrap(), quartzFilterWithURL : aURL)
    }
    unsafe fn quartzFilterWithProperties_(properties: NSDictionary) -> QuartzFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QuartzFilter").unwrap(), quartzFilterWithProperties : properties)
    }
    unsafe fn quartzFilterWithOutputIntents_(outputIntents: NSArray) -> QuartzFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QuartzFilter").unwrap(), quartzFilterWithOutputIntents : outputIntents)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QuartzFilterView(pub id);
impl std::ops::Deref for QuartzFilterView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QuartzFilterView {}
impl QuartzFilterView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QuartzFilterView").unwrap(), alloc) })
    }
}
impl INSView for QuartzFilterView {}
impl PNSAnimatablePropertyContainer for QuartzFilterView {}
impl PNSUserInterfaceItemIdentification for QuartzFilterView {}
impl PNSDraggingDestination for QuartzFilterView {}
impl PNSAppearanceCustomization for QuartzFilterView {}
impl PNSAccessibilityElement for QuartzFilterView {}
impl PNSAccessibility for QuartzFilterView {}
impl std::convert::TryFrom<NSView> for QuartzFilterView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<QuartzFilterView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QuartzFilterView").unwrap()) };
        if is_kind_of {
            Ok(QuartzFilterView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to QuartzFilterView")
        }
    }
}
impl INSResponder for QuartzFilterView {}
impl PNSCoding for QuartzFilterView {}
impl INSObject for QuartzFilterView {}
impl PNSObject for QuartzFilterView {}
impl IQuartzFilterView for QuartzFilterView {}
pub trait IQuartzFilterView: Sized + std::ops::Deref {
    unsafe fn sizeToFit(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeToFit)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QuartzFilterManager(pub id);
impl std::ops::Deref for QuartzFilterManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QuartzFilterManager {}
impl QuartzFilterManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QuartzFilterManager").unwrap(), alloc) })
    }
}
impl INSObject for QuartzFilterManager {}
impl PNSObject for QuartzFilterManager {}
impl std::convert::TryFrom<NSObject> for QuartzFilterManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<QuartzFilterManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"QuartzFilterManager").unwrap()) };
        if is_kind_of {
            Ok(QuartzFilterManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to QuartzFilterManager")
        }
    }
}
impl IQuartzFilterManager for QuartzFilterManager {}
pub trait IQuartzFilterManager: Sized + std::ops::Deref {
    unsafe fn filterPanel(&self) -> NSPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterPanel)
    }
    unsafe fn filterView(&self) -> QuartzFilterView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterView)
    }
    unsafe fn selectedFilter(&self) -> QuartzFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedFilter)
    }
    unsafe fn selectFilter_(&self, filter: QuartzFilter) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectFilter : filter)
    }
    unsafe fn setDelegate_(&self, aDelegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : aDelegate)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn importFilter_(&self, filterProperties: NSDictionary) -> QuartzFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, importFilter : filterProperties)
    }
    unsafe fn filterManager() -> QuartzFilterManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QuartzFilterManager").unwrap(), filterManager)
    }
    unsafe fn filtersInDomains_(domains: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"QuartzFilterManager").unwrap(), filtersInDomains : domains)
    }
}
pub trait NSObject_QuartzFilterManagerDelegate: Sized + std::ops::Deref {
    unsafe fn quartzFilterManager_didAddFilter_(
        &self,
        sender: QuartzFilterManager,
        filter: QuartzFilter,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, quartzFilterManager : sender, didAddFilter : filter)
    }
    unsafe fn quartzFilterManager_didRemoveFilter_(
        &self,
        sender: QuartzFilterManager,
        filter: QuartzFilter,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, quartzFilterManager : sender, didRemoveFilter : filter)
    }
    unsafe fn quartzFilterManager_didModifyFilter_(
        &self,
        sender: QuartzFilterManager,
        filter: QuartzFilter,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, quartzFilterManager : sender, didModifyFilter : filter)
    }
    unsafe fn quartzFilterManager_didSelectFilter_(
        &self,
        sender: QuartzFilterManager,
        filter: QuartzFilter,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, quartzFilterManager : sender, didSelectFilter : filter)
    }
}
pub type IKImageBrowserDropOperation = ::std::os::raw::c_uint;
pub trait NSObject_IKImageBrowserDataSource: Sized + std::ops::Deref {
    unsafe fn numberOfItemsInImageBrowser_(&self, aBrowser: IKImageBrowserView) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, numberOfItemsInImageBrowser : aBrowser)
    }
    unsafe fn imageBrowser_itemAtIndex_(
        &self,
        aBrowser: IKImageBrowserView,
        index: NSUInteger,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBrowser : aBrowser, itemAtIndex : index)
    }
    unsafe fn imageBrowser_removeItemsAtIndexes_(
        &self,
        aBrowser: IKImageBrowserView,
        indexes: NSIndexSet,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBrowser : aBrowser, removeItemsAtIndexes : indexes)
    }
    unsafe fn imageBrowser_moveItemsAtIndexes_toIndex_(
        &self,
        aBrowser: IKImageBrowserView,
        indexes: NSIndexSet,
        destinationIndex: NSUInteger,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBrowser : aBrowser, moveItemsAtIndexes : indexes, toIndex : destinationIndex)
    }
    unsafe fn imageBrowser_writeItemsAtIndexes_toPasteboard_(
        &self,
        aBrowser: IKImageBrowserView,
        itemIndexes: NSIndexSet,
        pasteboard: NSPasteboard,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBrowser : aBrowser, writeItemsAtIndexes : itemIndexes, toPasteboard : pasteboard)
    }
    unsafe fn numberOfGroupsInImageBrowser_(&self, aBrowser: IKImageBrowserView) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, numberOfGroupsInImageBrowser : aBrowser)
    }
    unsafe fn imageBrowser_groupAtIndex_(
        &self,
        aBrowser: IKImageBrowserView,
        index: NSUInteger,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBrowser : aBrowser, groupAtIndex : index)
    }
}
pub trait NSObject_IKImageBrowserItem: Sized + std::ops::Deref {
    unsafe fn imageUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageUID)
    }
    unsafe fn imageRepresentationType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageRepresentationType)
    }
    unsafe fn imageRepresentation(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageRepresentation)
    }
    unsafe fn imageVersion(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageVersion)
    }
    unsafe fn imageTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageTitle)
    }
    unsafe fn imageSubtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSubtitle)
    }
    unsafe fn isSelectable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSelectable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKImageBrowserView(pub id);
impl std::ops::Deref for IKImageBrowserView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKImageBrowserView {}
impl IKImageBrowserView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKImageBrowserView").unwrap(), alloc) })
    }
}
impl PNSDraggingSource for IKImageBrowserView {}
impl INSView for IKImageBrowserView {}
impl PNSAnimatablePropertyContainer for IKImageBrowserView {}
impl PNSUserInterfaceItemIdentification for IKImageBrowserView {}
impl PNSDraggingDestination for IKImageBrowserView {}
impl PNSAppearanceCustomization for IKImageBrowserView {}
impl PNSAccessibilityElement for IKImageBrowserView {}
impl PNSAccessibility for IKImageBrowserView {}
impl std::convert::TryFrom<NSView> for IKImageBrowserView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<IKImageBrowserView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKImageBrowserView").unwrap()) };
        if is_kind_of {
            Ok(IKImageBrowserView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to IKImageBrowserView")
        }
    }
}
impl INSResponder for IKImageBrowserView {}
impl PNSCoding for IKImageBrowserView {}
impl INSObject for IKImageBrowserView {}
impl PNSObject for IKImageBrowserView {}
impl IIKImageBrowserView for IKImageBrowserView {}
pub trait IIKImageBrowserView: Sized + std::ops::Deref {}
impl IKImageBrowserView_IKMainMethods for IKImageBrowserView {}
pub trait IKImageBrowserView_IKMainMethods: Sized + std::ops::Deref {
    unsafe fn initWithFrame_(&self, frame: NSRect) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame)
    }
    unsafe fn reloadData(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadData)
    }
    unsafe fn dataSource(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSource)
    }
    unsafe fn setDataSource_(&self, dataSource: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataSource : dataSource)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
}
impl IKImageBrowserView_IKAppearance for IKImageBrowserView {}
pub trait IKImageBrowserView_IKAppearance: Sized + std::ops::Deref {
    unsafe fn setCellsStyleMask_(&self, mask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCellsStyleMask : mask)
    }
    unsafe fn cellsStyleMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cellsStyleMask)
    }
    unsafe fn setConstrainsToOriginalSize_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstrainsToOriginalSize : flag)
    }
    unsafe fn constrainsToOriginalSize(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constrainsToOriginalSize)
    }
    unsafe fn setBackgroundLayer_(&self, aLayer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundLayer : aLayer)
    }
    unsafe fn backgroundLayer(&self) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundLayer)
    }
    unsafe fn setForegroundLayer_(&self, aLayer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setForegroundLayer : aLayer)
    }
    unsafe fn foregroundLayer(&self) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foregroundLayer)
    }
    unsafe fn newCellForRepresentedItem_(&self, anItem: id) -> IKImageBrowserCell
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCellForRepresentedItem : anItem)
    }
    unsafe fn cellForItemAtIndex_(&self, index: NSUInteger) -> IKImageBrowserCell
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cellForItemAtIndex : index)
    }
}
impl IKImageBrowserView_IKBrowsing for IKImageBrowserView {}
pub trait IKImageBrowserView_IKBrowsing: Sized + std::ops::Deref {
    unsafe fn setZoomValue_(&self, aValue: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZoomValue : aValue)
    }
    unsafe fn zoomValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoomValue)
    }
    unsafe fn setContentResizingMask_(&self, mask: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentResizingMask : mask)
    }
    unsafe fn contentResizingMask(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentResizingMask)
    }
    unsafe fn scrollIndexToVisible_(&self, index: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollIndexToVisible : index)
    }
    unsafe fn setCellSize_(&self, size: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCellSize : size)
    }
    unsafe fn cellSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cellSize)
    }
    unsafe fn intercellSpacing(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intercellSpacing)
    }
    unsafe fn setIntercellSpacing_(&self, aSize: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntercellSpacing : aSize)
    }
    unsafe fn indexOfItemAtPoint_(&self, point: NSPoint) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexOfItemAtPoint : point)
    }
    unsafe fn itemFrameAtIndex_(&self, index: NSInteger) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, itemFrameAtIndex : index)
    }
    unsafe fn visibleItemIndexes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibleItemIndexes)
    }
    unsafe fn rowIndexesInRect_(&self, rect: NSRect) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rowIndexesInRect : rect)
    }
    unsafe fn columnIndexesInRect_(&self, rect: NSRect) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, columnIndexesInRect : rect)
    }
    unsafe fn rectOfColumn_(&self, columnIndex: NSUInteger) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectOfColumn : columnIndex)
    }
    unsafe fn rectOfRow_(&self, rowIndex: NSUInteger) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectOfRow : rowIndex)
    }
    unsafe fn numberOfRows(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfRows)
    }
    unsafe fn numberOfColumns(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfColumns)
    }
    unsafe fn setCanControlQuickLookPanel_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanControlQuickLookPanel : flag)
    }
    unsafe fn canControlQuickLookPanel(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canControlQuickLookPanel)
    }
}
impl IKImageBrowserView_IKSelectionReorderingAndGrouping for IKImageBrowserView {}
pub trait IKImageBrowserView_IKSelectionReorderingAndGrouping: Sized + std::ops::Deref {
    unsafe fn selectionIndexes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionIndexes)
    }
    unsafe fn setSelectionIndexes_byExtendingSelection_(
        &self,
        indexes: NSIndexSet,
        extendSelection: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionIndexes : indexes, byExtendingSelection : extendSelection)
    }
    unsafe fn setAllowsMultipleSelection_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsMultipleSelection : flag)
    }
    unsafe fn allowsMultipleSelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsMultipleSelection)
    }
    unsafe fn setAllowsEmptySelection_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsEmptySelection : flag)
    }
    unsafe fn allowsEmptySelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsEmptySelection)
    }
    unsafe fn setAllowsReordering_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsReordering : flag)
    }
    unsafe fn allowsReordering(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsReordering)
    }
    unsafe fn setAnimates_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimates : flag)
    }
    unsafe fn animates(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animates)
    }
    unsafe fn expandGroupAtIndex_(&self, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, expandGroupAtIndex : index)
    }
    unsafe fn collapseGroupAtIndex_(&self, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, collapseGroupAtIndex : index)
    }
    unsafe fn isGroupExpandedAtIndex_(&self, index: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isGroupExpandedAtIndex : index)
    }
}
impl IKImageBrowserView_IKDragNDrop for IKImageBrowserView {}
pub trait IKImageBrowserView_IKDragNDrop: Sized + std::ops::Deref {
    unsafe fn setDraggingDestinationDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDraggingDestinationDelegate : delegate)
    }
    unsafe fn draggingDestinationDelegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, draggingDestinationDelegate)
    }
    unsafe fn indexAtLocationOfDroppedItem(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexAtLocationOfDroppedItem)
    }
    unsafe fn dropOperation(&self) -> IKImageBrowserDropOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dropOperation)
    }
    unsafe fn setAllowsDroppingOnItems_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsDroppingOnItems : flag)
    }
    unsafe fn allowsDroppingOnItems(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsDroppingOnItems)
    }
    unsafe fn setDropIndex_dropOperation_(
        &self,
        index: NSInteger,
        operation: IKImageBrowserDropOperation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDropIndex : index, dropOperation : operation)
    }
}
pub trait NSObject_IKImageBrowserDelegate: Sized + std::ops::Deref {
    unsafe fn imageBrowserSelectionDidChange_(&self, aBrowser: IKImageBrowserView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBrowserSelectionDidChange : aBrowser)
    }
    unsafe fn imageBrowser_cellWasDoubleClickedAtIndex_(
        &self,
        aBrowser: IKImageBrowserView,
        index: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBrowser : aBrowser, cellWasDoubleClickedAtIndex : index)
    }
    unsafe fn imageBrowser_cellWasRightClickedAtIndex_withEvent_(
        &self,
        aBrowser: IKImageBrowserView,
        index: NSUInteger,
        event: NSEvent,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBrowser : aBrowser, cellWasRightClickedAtIndex : index, withEvent : event)
    }
    unsafe fn imageBrowser_backgroundWasRightClickedWithEvent_(
        &self,
        aBrowser: IKImageBrowserView,
        event: NSEvent,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBrowser : aBrowser, backgroundWasRightClickedWithEvent : event)
    }
}
pub type IKImageBrowserCellState = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKImageBrowserCell(pub id);
impl std::ops::Deref for IKImageBrowserCell {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKImageBrowserCell {}
impl IKImageBrowserCell {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKImageBrowserCell").unwrap(), alloc) })
    }
}
impl INSObject for IKImageBrowserCell {}
impl PNSObject for IKImageBrowserCell {}
impl std::convert::TryFrom<NSObject> for IKImageBrowserCell {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IKImageBrowserCell, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKImageBrowserCell").unwrap()) };
        if is_kind_of {
            Ok(IKImageBrowserCell(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IKImageBrowserCell")
        }
    }
}
impl IIKImageBrowserCell for IKImageBrowserCell {}
pub trait IIKImageBrowserCell: Sized + std::ops::Deref {
    unsafe fn imageBrowserView(&self) -> IKImageBrowserView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageBrowserView)
    }
    unsafe fn representedItem(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, representedItem)
    }
    unsafe fn indexOfRepresentedItem(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexOfRepresentedItem)
    }
    unsafe fn frame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frame)
    }
    unsafe fn imageContainerFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageContainerFrame)
    }
    unsafe fn imageFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageFrame)
    }
    unsafe fn selectionFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionFrame)
    }
    unsafe fn titleFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleFrame)
    }
    unsafe fn subtitleFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitleFrame)
    }
    unsafe fn imageAlignment(&self) -> NSImageAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageAlignment)
    }
    unsafe fn isSelected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSelected)
    }
    unsafe fn cellState(&self) -> IKImageBrowserCellState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cellState)
    }
    unsafe fn opacity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opacity)
    }
    unsafe fn layerForType_(&self, type_: NSString) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layerForType : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKPictureTaker(pub id);
impl std::ops::Deref for IKPictureTaker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKPictureTaker {}
impl IKPictureTaker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKPictureTaker").unwrap(), alloc) })
    }
}
impl INSPanel for IKPictureTaker {}
impl std::convert::TryFrom<NSPanel> for IKPictureTaker {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<IKPictureTaker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKPictureTaker").unwrap()) };
        if is_kind_of {
            Ok(IKPictureTaker(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to IKPictureTaker")
        }
    }
}
impl INSWindow for IKPictureTaker {}
impl PNSAnimatablePropertyContainer for IKPictureTaker {}
impl PNSMenuItemValidation for IKPictureTaker {}
impl PNSUserInterfaceValidations for IKPictureTaker {}
impl PNSUserInterfaceItemIdentification for IKPictureTaker {}
impl PNSAppearanceCustomization for IKPictureTaker {}
impl PNSAccessibilityElement for IKPictureTaker {}
impl PNSAccessibility for IKPictureTaker {}
impl INSResponder for IKPictureTaker {}
impl PNSCoding for IKPictureTaker {}
impl INSObject for IKPictureTaker {}
impl PNSObject for IKPictureTaker {}
impl IIKPictureTaker for IKPictureTaker {}
pub trait IIKPictureTaker: Sized + std::ops::Deref {
    unsafe fn runModal(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, runModal)
    }
    unsafe fn beginPictureTakerWithDelegate_didEndSelector_contextInfo_(
        &self,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginPictureTakerWithDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn beginPictureTakerSheetForWindow_withDelegate_didEndSelector_contextInfo_(
        &self,
        aWindow: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginPictureTakerSheetForWindow : aWindow, withDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn popUpRecentsMenuForView_withDelegate_didEndSelector_contextInfo_(
        &self,
        aView: NSView,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popUpRecentsMenuForView : aView, withDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn setInputImage_(&self, image: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : image)
    }
    unsafe fn inputImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn outputImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputImage)
    }
    unsafe fn setMirroring_(&self, b: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMirroring : b)
    }
    unsafe fn mirroring(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mirroring)
    }
    unsafe fn pictureTaker() -> IKPictureTaker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IKPictureTaker").unwrap(), pictureTaker)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKFilterUIView(pub id);
impl std::ops::Deref for IKFilterUIView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKFilterUIView {}
impl IKFilterUIView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKFilterUIView").unwrap(), alloc) })
    }
}
impl INSView for IKFilterUIView {}
impl PNSAnimatablePropertyContainer for IKFilterUIView {}
impl PNSUserInterfaceItemIdentification for IKFilterUIView {}
impl PNSDraggingDestination for IKFilterUIView {}
impl PNSAppearanceCustomization for IKFilterUIView {}
impl PNSAccessibilityElement for IKFilterUIView {}
impl PNSAccessibility for IKFilterUIView {}
impl std::convert::TryFrom<NSView> for IKFilterUIView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<IKFilterUIView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKFilterUIView").unwrap()) };
        if is_kind_of {
            Ok(IKFilterUIView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to IKFilterUIView")
        }
    }
}
impl INSResponder for IKFilterUIView {}
impl PNSCoding for IKFilterUIView {}
impl INSObject for IKFilterUIView {}
impl PNSObject for IKFilterUIView {}
impl IIKFilterUIView for IKFilterUIView {}
pub trait IIKFilterUIView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_filter_(&self, frameRect: NSRect, inFilter: CIFilter) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frameRect, filter : inFilter)
    }
    unsafe fn filter(&self) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filter)
    }
    unsafe fn objectController(&self) -> NSObjectController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectController)
    }
    unsafe fn viewWithFrame_filter_(frameRect: NSRect, inFilter: CIFilter) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IKFilterUIView").unwrap(), viewWithFrame : frameRect, filter : inFilter)
    }
}
pub trait CIFilter_IKFilterUIAddition: Sized + std::ops::Deref {
    unsafe fn viewForUIConfiguration_excludedKeys_(
        &self,
        inUIConfiguration: NSDictionary,
        inKeys: NSArray,
    ) -> IKFilterUIView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, viewForUIConfiguration : inUIConfiguration, excludedKeys : inKeys)
    }
}
pub trait PIKFilterCustomUIProvider: Sized + std::ops::Deref {
    unsafe fn provideViewForUIConfiguration_excludedKeys_(
        &self,
        inUIConfiguration: NSDictionary,
        inKeys: NSArray,
    ) -> IKFilterUIView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provideViewForUIConfiguration : inUIConfiguration, excludedKeys : inKeys)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKFilterBrowserPanel(pub id);
impl std::ops::Deref for IKFilterBrowserPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKFilterBrowserPanel {}
impl IKFilterBrowserPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKFilterBrowserPanel").unwrap(), alloc) })
    }
}
impl INSPanel for IKFilterBrowserPanel {}
impl std::convert::TryFrom<NSPanel> for IKFilterBrowserPanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<IKFilterBrowserPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKFilterBrowserPanel").unwrap()) };
        if is_kind_of {
            Ok(IKFilterBrowserPanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to IKFilterBrowserPanel")
        }
    }
}
impl INSWindow for IKFilterBrowserPanel {}
impl PNSAnimatablePropertyContainer for IKFilterBrowserPanel {}
impl PNSMenuItemValidation for IKFilterBrowserPanel {}
impl PNSUserInterfaceValidations for IKFilterBrowserPanel {}
impl PNSUserInterfaceItemIdentification for IKFilterBrowserPanel {}
impl PNSAppearanceCustomization for IKFilterBrowserPanel {}
impl PNSAccessibilityElement for IKFilterBrowserPanel {}
impl PNSAccessibility for IKFilterBrowserPanel {}
impl INSResponder for IKFilterBrowserPanel {}
impl PNSCoding for IKFilterBrowserPanel {}
impl INSObject for IKFilterBrowserPanel {}
impl PNSObject for IKFilterBrowserPanel {}
impl IIKFilterBrowserPanel for IKFilterBrowserPanel {}
pub trait IIKFilterBrowserPanel: Sized + std::ops::Deref {
    unsafe fn filterName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterName)
    }
    unsafe fn filterBrowserPanelWithStyleMask_(styleMask: ::std::os::raw::c_uint) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IKFilterBrowserPanel").unwrap(), filterBrowserPanelWithStyleMask : styleMask)
    }
}
impl IKFilterBrowserPanel_IKFilterBrowserPanelRuntime for IKFilterBrowserPanel {}
pub trait IKFilterBrowserPanel_IKFilterBrowserPanelRuntime: Sized + std::ops::Deref {
    unsafe fn beginWithOptions_modelessDelegate_didEndSelector_contextInfo_(
        &self,
        inOptions: NSDictionary,
        modelessDelegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginWithOptions : inOptions, modelessDelegate : modelessDelegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn beginSheetWithOptions_modalForWindow_modalDelegate_didEndSelector_contextInfo_(
        &self,
        inOptions: NSDictionary,
        docWindow: NSWindow,
        modalDelegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetWithOptions : inOptions, modalForWindow : docWindow, modalDelegate : modalDelegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn runModalWithOptions_(&self, inOptions: NSDictionary) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalWithOptions : inOptions)
    }
    unsafe fn filterBrowserViewWithOptions_(&self, inOptions: NSDictionary) -> IKFilterBrowserView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, filterBrowserViewWithOptions : inOptions)
    }
    unsafe fn finish_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finish : sender)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKFilterBrowserView(pub id);
impl std::ops::Deref for IKFilterBrowserView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKFilterBrowserView {}
impl IKFilterBrowserView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKFilterBrowserView").unwrap(), alloc) })
    }
}
impl INSView for IKFilterBrowserView {}
impl PNSAnimatablePropertyContainer for IKFilterBrowserView {}
impl PNSUserInterfaceItemIdentification for IKFilterBrowserView {}
impl PNSDraggingDestination for IKFilterBrowserView {}
impl PNSAppearanceCustomization for IKFilterBrowserView {}
impl PNSAccessibilityElement for IKFilterBrowserView {}
impl PNSAccessibility for IKFilterBrowserView {}
impl std::convert::TryFrom<NSView> for IKFilterBrowserView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<IKFilterBrowserView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKFilterBrowserView").unwrap()) };
        if is_kind_of {
            Ok(IKFilterBrowserView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to IKFilterBrowserView")
        }
    }
}
impl INSResponder for IKFilterBrowserView {}
impl PNSCoding for IKFilterBrowserView {}
impl INSObject for IKFilterBrowserView {}
impl PNSObject for IKFilterBrowserView {}
impl IIKFilterBrowserView for IKFilterBrowserView {}
pub trait IIKFilterBrowserView: Sized + std::ops::Deref {
    unsafe fn setPreviewState_(&self, inState: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviewState : inState)
    }
    unsafe fn filterName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterName)
    }
}
pub trait PIKSlideshowDataSource: Sized + std::ops::Deref {
    unsafe fn numberOfSlideshowItems(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfSlideshowItems)
    }
    unsafe fn slideshowItemAtIndex_(&self, index: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, slideshowItemAtIndex : index)
    }
    unsafe fn nameOfSlideshowItemAtIndex_(&self, index: NSUInteger) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nameOfSlideshowItemAtIndex : index)
    }
    unsafe fn canExportSlideshowItemAtIndex_toApplication_(
        &self,
        index: NSUInteger,
        applicationBundleIdentifier: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canExportSlideshowItemAtIndex : index, toApplication : applicationBundleIdentifier)
    }
    unsafe fn slideshowWillStart(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, slideshowWillStart)
    }
    unsafe fn slideshowDidStop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, slideshowDidStop)
    }
    unsafe fn slideshowDidChangeCurrentIndex_(&self, newIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, slideshowDidChangeCurrentIndex : newIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKSlideshow(pub id);
impl std::ops::Deref for IKSlideshow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKSlideshow {}
impl IKSlideshow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKSlideshow").unwrap(), alloc) })
    }
}
impl INSObject for IKSlideshow {}
impl PNSObject for IKSlideshow {}
impl std::convert::TryFrom<NSObject> for IKSlideshow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IKSlideshow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKSlideshow").unwrap()) };
        if is_kind_of {
            Ok(IKSlideshow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IKSlideshow")
        }
    }
}
impl IIKSlideshow for IKSlideshow {}
pub trait IIKSlideshow: Sized + std::ops::Deref {
    unsafe fn runSlideshowWithDataSource_inMode_options_(
        &self,
        dataSource: *mut u64,
        slideshowMode: NSString,
        slideshowOptions: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runSlideshowWithDataSource : dataSource, inMode : slideshowMode, options : slideshowOptions)
    }
    unsafe fn stopSlideshow_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopSlideshow : sender)
    }
    unsafe fn reloadData(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadData)
    }
    unsafe fn reloadSlideshowItemAtIndex_(&self, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reloadSlideshowItemAtIndex : index)
    }
    unsafe fn indexOfCurrentSlideshowItem(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexOfCurrentSlideshowItem)
    }
    unsafe fn autoPlayDelay(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoPlayDelay)
    }
    unsafe fn setAutoPlayDelay_(&self, autoPlayDelay: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoPlayDelay : autoPlayDelay)
    }
    unsafe fn sharedSlideshow() -> IKSlideshow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IKSlideshow").unwrap(), sharedSlideshow)
    }
    unsafe fn canExportToApplication_(applicationBundleIdentifier: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IKSlideshow").unwrap(), canExportToApplication : applicationBundleIdentifier)
    }
    unsafe fn exportSlideshowItem_toApplication_(item: id, applicationBundleIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IKSlideshow").unwrap(), exportSlideshowItem : item, toApplication : applicationBundleIdentifier)
    }
}
pub trait NSObject_IKSaveOptionsDelegate: Sized + std::ops::Deref {
    unsafe fn saveOptions_shouldShowUTType_(
        &self,
        saveOptions: IKSaveOptions,
        utType: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveOptions : saveOptions, shouldShowUTType : utType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKSaveOptions(pub id);
impl std::ops::Deref for IKSaveOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKSaveOptions {}
impl IKSaveOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKSaveOptions").unwrap(), alloc) })
    }
}
impl INSObject for IKSaveOptions {}
impl PNSObject for IKSaveOptions {}
impl std::convert::TryFrom<NSObject> for IKSaveOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IKSaveOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKSaveOptions").unwrap()) };
        if is_kind_of {
            Ok(IKSaveOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IKSaveOptions")
        }
    }
}
impl IIKSaveOptions for IKSaveOptions {}
pub trait IIKSaveOptions: Sized + std::ops::Deref {
    unsafe fn initWithImageProperties_imageUTType_(
        &self,
        imageProperties: NSDictionary,
        imageUTType: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProperties : imageProperties, imageUTType : imageUTType)
    }
    unsafe fn addSaveOptionsAccessoryViewToSavePanel_(&self, savePanel: NSSavePanel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSaveOptionsAccessoryViewToSavePanel : savePanel)
    }
    unsafe fn addSaveOptionsToView_(&self, view: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSaveOptionsToView : view)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn imageProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProperties)
    }
    unsafe fn imageUTType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageUTType)
    }
    unsafe fn userSelection(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userSelection)
    }
    unsafe fn rememberLastSetting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rememberLastSetting)
    }
    unsafe fn setRememberLastSetting_(&self, rememberLastSetting: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRememberLastSetting : rememberLastSetting)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKImageView(pub id);
impl std::ops::Deref for IKImageView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKImageView {}
impl IKImageView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKImageView").unwrap(), alloc) })
    }
}
impl INSView for IKImageView {}
impl PNSAnimatablePropertyContainer for IKImageView {}
impl PNSUserInterfaceItemIdentification for IKImageView {}
impl PNSDraggingDestination for IKImageView {}
impl PNSAppearanceCustomization for IKImageView {}
impl PNSAccessibilityElement for IKImageView {}
impl PNSAccessibility for IKImageView {}
impl std::convert::TryFrom<NSView> for IKImageView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<IKImageView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKImageView").unwrap()) };
        if is_kind_of {
            Ok(IKImageView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to IKImageView")
        }
    }
}
impl INSResponder for IKImageView {}
impl PNSCoding for IKImageView {}
impl INSObject for IKImageView {}
impl PNSObject for IKImageView {}
impl IIKImageView for IKImageView {}
pub trait IIKImageView: Sized + std::ops::Deref {
    unsafe fn setImage_imageProperties_(&self, image: CGImageRef, metaData: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image, imageProperties : metaData)
    }
    unsafe fn setImageWithURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageWithURL : url)
    }
    unsafe fn image(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn imageSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSize)
    }
    unsafe fn imageProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProperties)
    }
    unsafe fn setRotationAngle_centerPoint_(&self, rotationAngle: CGFloat, centerPoint: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotationAngle : rotationAngle, centerPoint : centerPoint)
    }
    unsafe fn rotateImageLeft_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotateImageLeft : sender)
    }
    unsafe fn rotateImageRight_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotateImageRight : sender)
    }
    unsafe fn setImageZoomFactor_centerPoint_(&self, zoomFactor: CGFloat, centerPoint: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageZoomFactor : zoomFactor, centerPoint : centerPoint)
    }
    unsafe fn zoomImageToRect_(&self, rect: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, zoomImageToRect : rect)
    }
    unsafe fn zoomImageToFit_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, zoomImageToFit : sender)
    }
    unsafe fn zoomImageToActualSize_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, zoomImageToActualSize : sender)
    }
    unsafe fn zoomIn_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, zoomIn : sender)
    }
    unsafe fn zoomOut_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, zoomOut : sender)
    }
    unsafe fn flipImageHorizontal_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, flipImageHorizontal : sender)
    }
    unsafe fn flipImageVertical_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, flipImageVertical : sender)
    }
    unsafe fn crop_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, crop : sender)
    }
    unsafe fn setOverlay_forType_(&self, layer: CALayer, layerType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverlay : layer, forType : layerType)
    }
    unsafe fn overlayForType_(&self, layerType: NSString) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, overlayForType : layerType)
    }
    unsafe fn scrollToPoint_(&self, point: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollToPoint : point)
    }
    unsafe fn scrollToRect_(&self, rect: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollToRect : rect)
    }
    unsafe fn convertViewPointToImagePoint_(&self, viewPoint: NSPoint) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertViewPointToImagePoint : viewPoint)
    }
    unsafe fn convertViewRectToImageRect_(&self, viewRect: NSRect) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertViewRectToImageRect : viewRect)
    }
    unsafe fn convertImagePointToViewPoint_(&self, imagePoint: NSPoint) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertImagePointToViewPoint : imagePoint)
    }
    unsafe fn convertImageRectToViewRect_(&self, imageRect: NSRect) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertImageRectToViewRect : imageRect)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn zoomFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoomFactor)
    }
    unsafe fn setZoomFactor_(&self, zoomFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZoomFactor : zoomFactor)
    }
    unsafe fn rotationAngle(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationAngle)
    }
    unsafe fn setRotationAngle_(&self, rotationAngle: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotationAngle : rotationAngle)
    }
    unsafe fn currentToolMode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentToolMode)
    }
    unsafe fn setCurrentToolMode_(&self, currentToolMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentToolMode : currentToolMode)
    }
    unsafe fn autoresizes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoresizes)
    }
    unsafe fn setAutoresizes_(&self, autoresizes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoresizes : autoresizes)
    }
    unsafe fn hasHorizontalScroller(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasHorizontalScroller)
    }
    unsafe fn setHasHorizontalScroller_(&self, hasHorizontalScroller: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasHorizontalScroller : hasHorizontalScroller)
    }
    unsafe fn hasVerticalScroller(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasVerticalScroller)
    }
    unsafe fn setHasVerticalScroller_(&self, hasVerticalScroller: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasVerticalScroller : hasVerticalScroller)
    }
    unsafe fn autohidesScrollers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autohidesScrollers)
    }
    unsafe fn setAutohidesScrollers_(&self, autohidesScrollers: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutohidesScrollers : autohidesScrollers)
    }
    unsafe fn supportsDragAndDrop(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsDragAndDrop)
    }
    unsafe fn setSupportsDragAndDrop_(&self, supportsDragAndDrop: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsDragAndDrop : supportsDragAndDrop)
    }
    unsafe fn editable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, editable)
    }
    unsafe fn setEditable_(&self, editable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditable : editable)
    }
    unsafe fn doubleClickOpensImageEditPanel(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doubleClickOpensImageEditPanel)
    }
    unsafe fn setDoubleClickOpensImageEditPanel_(&self, doubleClickOpensImageEditPanel: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoubleClickOpensImageEditPanel : doubleClickOpensImageEditPanel)
    }
    unsafe fn imageCorrection(&self) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageCorrection)
    }
    unsafe fn setImageCorrection_(&self, imageCorrection: CIFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageCorrection : imageCorrection)
    }
    unsafe fn backgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setBackgroundColor_(&self, backgroundColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : backgroundColor)
    }
}
pub trait PIKImageEditPanelDataSource: Sized + std::ops::Deref {
    unsafe fn setImage_imageProperties_(&self, image: CGImageRef, metaData: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image, imageProperties : metaData)
    }
    unsafe fn thumbnailWithMaximumSize_(&self, size: NSSize) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, thumbnailWithMaximumSize : size)
    }
    unsafe fn image(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn imageProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProperties)
    }
    unsafe fn hasAdjustMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAdjustMode)
    }
    unsafe fn hasEffectsMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasEffectsMode)
    }
    unsafe fn hasDetailsMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasDetailsMode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKImageEditPanel(pub id);
impl std::ops::Deref for IKImageEditPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKImageEditPanel {}
impl IKImageEditPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKImageEditPanel").unwrap(), alloc) })
    }
}
impl INSPanel for IKImageEditPanel {}
impl std::convert::TryFrom<NSPanel> for IKImageEditPanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<IKImageEditPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKImageEditPanel").unwrap()) };
        if is_kind_of {
            Ok(IKImageEditPanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to IKImageEditPanel")
        }
    }
}
impl INSWindow for IKImageEditPanel {}
impl PNSAnimatablePropertyContainer for IKImageEditPanel {}
impl PNSMenuItemValidation for IKImageEditPanel {}
impl PNSUserInterfaceValidations for IKImageEditPanel {}
impl PNSUserInterfaceItemIdentification for IKImageEditPanel {}
impl PNSAppearanceCustomization for IKImageEditPanel {}
impl PNSAccessibilityElement for IKImageEditPanel {}
impl PNSAccessibility for IKImageEditPanel {}
impl INSResponder for IKImageEditPanel {}
impl PNSCoding for IKImageEditPanel {}
impl INSObject for IKImageEditPanel {}
impl PNSObject for IKImageEditPanel {}
impl IIKImageEditPanel for IKImageEditPanel {}
pub trait IIKImageEditPanel: Sized + std::ops::Deref {
    unsafe fn reloadData(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadData)
    }
    unsafe fn dataSource(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSource)
    }
    unsafe fn setDataSource_(&self, dataSource: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataSource : dataSource)
    }
    unsafe fn filterArray(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterArray)
    }
    unsafe fn sharedImageEditPanel() -> IKImageEditPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IKImageEditPanel").unwrap(), sharedImageEditPanel)
    }
}
pub trait PIKDeviceBrowserViewDelegate: Sized + std::ops::Deref {
    unsafe fn deviceBrowserView_selectionDidChange_(
        &self,
        deviceBrowserView: IKDeviceBrowserView,
        device: ICDevice,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowserView : deviceBrowserView, selectionDidChange : device)
    }
    unsafe fn deviceBrowserView_didEncounterError_(
        &self,
        deviceBrowserView: IKDeviceBrowserView,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deviceBrowserView : deviceBrowserView, didEncounterError : error)
    }
}
pub type IKDeviceBrowserViewDisplayMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKDeviceBrowserView(pub id);
impl std::ops::Deref for IKDeviceBrowserView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKDeviceBrowserView {}
impl IKDeviceBrowserView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKDeviceBrowserView").unwrap(), alloc) })
    }
}
impl INSView for IKDeviceBrowserView {}
impl PNSAnimatablePropertyContainer for IKDeviceBrowserView {}
impl PNSUserInterfaceItemIdentification for IKDeviceBrowserView {}
impl PNSDraggingDestination for IKDeviceBrowserView {}
impl PNSAppearanceCustomization for IKDeviceBrowserView {}
impl PNSAccessibilityElement for IKDeviceBrowserView {}
impl PNSAccessibility for IKDeviceBrowserView {}
impl std::convert::TryFrom<NSView> for IKDeviceBrowserView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<IKDeviceBrowserView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKDeviceBrowserView").unwrap()) };
        if is_kind_of {
            Ok(IKDeviceBrowserView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to IKDeviceBrowserView")
        }
    }
}
impl INSResponder for IKDeviceBrowserView {}
impl PNSCoding for IKDeviceBrowserView {}
impl INSObject for IKDeviceBrowserView {}
impl PNSObject for IKDeviceBrowserView {}
impl IIKDeviceBrowserView for IKDeviceBrowserView {}
pub trait IIKDeviceBrowserView: Sized + std::ops::Deref {
    unsafe fn delegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn displaysLocalCameras(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysLocalCameras)
    }
    unsafe fn setDisplaysLocalCameras_(&self, displaysLocalCameras: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysLocalCameras : displaysLocalCameras)
    }
    unsafe fn displaysNetworkCameras(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysNetworkCameras)
    }
    unsafe fn setDisplaysNetworkCameras_(&self, displaysNetworkCameras: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysNetworkCameras : displaysNetworkCameras)
    }
    unsafe fn displaysLocalScanners(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysLocalScanners)
    }
    unsafe fn setDisplaysLocalScanners_(&self, displaysLocalScanners: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysLocalScanners : displaysLocalScanners)
    }
    unsafe fn displaysNetworkScanners(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysNetworkScanners)
    }
    unsafe fn setDisplaysNetworkScanners_(&self, displaysNetworkScanners: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysNetworkScanners : displaysNetworkScanners)
    }
    unsafe fn mode(&self) -> IKDeviceBrowserViewDisplayMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn setMode_(&self, mode: IKDeviceBrowserViewDisplayMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMode : mode)
    }
    unsafe fn selectedDevice(&self) -> ICDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedDevice)
    }
}
pub trait PIKCameraDeviceViewDelegate: Sized + std::ops::Deref {
    unsafe fn cameraDeviceViewSelectionDidChange_(&self, cameraDeviceView: IKCameraDeviceView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDeviceViewSelectionDidChange : cameraDeviceView)
    }
    unsafe fn cameraDeviceView_didDownloadFile_location_fileData_error_(
        &self,
        cameraDeviceView: IKCameraDeviceView,
        file: ICCameraFile,
        url: NSURL,
        data: NSData,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDeviceView : cameraDeviceView, didDownloadFile : file, location : url, fileData : data, error : error)
    }
    unsafe fn cameraDeviceView_didEncounterError_(
        &self,
        cameraDeviceView: IKCameraDeviceView,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cameraDeviceView : cameraDeviceView, didEncounterError : error)
    }
}
pub type IKCameraDeviceViewDisplayMode = NSInteger;
pub type IKCameraDeviceViewTransferMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKCameraDeviceView(pub id);
impl std::ops::Deref for IKCameraDeviceView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKCameraDeviceView {}
impl IKCameraDeviceView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKCameraDeviceView").unwrap(), alloc) })
    }
}
impl INSView for IKCameraDeviceView {}
impl PNSAnimatablePropertyContainer for IKCameraDeviceView {}
impl PNSUserInterfaceItemIdentification for IKCameraDeviceView {}
impl PNSDraggingDestination for IKCameraDeviceView {}
impl PNSAppearanceCustomization for IKCameraDeviceView {}
impl PNSAccessibilityElement for IKCameraDeviceView {}
impl PNSAccessibility for IKCameraDeviceView {}
impl std::convert::TryFrom<NSView> for IKCameraDeviceView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<IKCameraDeviceView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKCameraDeviceView").unwrap()) };
        if is_kind_of {
            Ok(IKCameraDeviceView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to IKCameraDeviceView")
        }
    }
}
impl INSResponder for IKCameraDeviceView {}
impl PNSCoding for IKCameraDeviceView {}
impl INSObject for IKCameraDeviceView {}
impl PNSObject for IKCameraDeviceView {}
impl IIKCameraDeviceView for IKCameraDeviceView {}
pub trait IIKCameraDeviceView: Sized + std::ops::Deref {
    unsafe fn selectedIndexes(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedIndexes)
    }
    unsafe fn selectIndexes_byExtendingSelection_(&self, indexes: NSIndexSet, extend_: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectIndexes : indexes, byExtendingSelection : extend_)
    }
    unsafe fn rotateLeft_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotateLeft : sender)
    }
    unsafe fn rotateRight_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotateRight : sender)
    }
    unsafe fn deleteSelectedItems_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteSelectedItems : sender)
    }
    unsafe fn downloadSelectedItems_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadSelectedItems : sender)
    }
    unsafe fn downloadAllItems_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, downloadAllItems : sender)
    }
    unsafe fn setCustomIconSizeSlider_(&self, slider: NSSlider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomIconSizeSlider : slider)
    }
    unsafe fn setCustomModeControl_(&self, control: NSSegmentedControl)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomModeControl : control)
    }
    unsafe fn setCustomActionControl_(&self, control: NSSegmentedControl)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomActionControl : control)
    }
    unsafe fn setCustomRotateControl_(&self, control: NSSegmentedControl)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomRotateControl : control)
    }
    unsafe fn setCustomDeleteControl_(&self, control: NSSegmentedControl)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomDeleteControl : control)
    }
    unsafe fn setShowStatusInfoAsWindowSubtitle_(&self, value: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowStatusInfoAsWindowSubtitle : value)
    }
    unsafe fn delegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn cameraDevice(&self) -> ICCameraDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraDevice)
    }
    unsafe fn setCameraDevice_(&self, cameraDevice: ICCameraDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraDevice : cameraDevice)
    }
    unsafe fn mode(&self) -> IKCameraDeviceViewDisplayMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn setMode_(&self, mode: IKCameraDeviceViewDisplayMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMode : mode)
    }
    unsafe fn hasDisplayModeTable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasDisplayModeTable)
    }
    unsafe fn setHasDisplayModeTable_(&self, hasDisplayModeTable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasDisplayModeTable : hasDisplayModeTable)
    }
    unsafe fn hasDisplayModeIcon(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasDisplayModeIcon)
    }
    unsafe fn setHasDisplayModeIcon_(&self, hasDisplayModeIcon: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasDisplayModeIcon : hasDisplayModeIcon)
    }
    unsafe fn downloadAllControlLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadAllControlLabel)
    }
    unsafe fn setDownloadAllControlLabel_(&self, downloadAllControlLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownloadAllControlLabel : downloadAllControlLabel)
    }
    unsafe fn downloadSelectedControlLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadSelectedControlLabel)
    }
    unsafe fn setDownloadSelectedControlLabel_(&self, downloadSelectedControlLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownloadSelectedControlLabel : downloadSelectedControlLabel)
    }
    unsafe fn iconSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iconSize)
    }
    unsafe fn setIconSize_(&self, iconSize: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIconSize : iconSize)
    }
    unsafe fn transferMode(&self) -> IKCameraDeviceViewTransferMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transferMode)
    }
    unsafe fn setTransferMode_(&self, transferMode: IKCameraDeviceViewTransferMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransferMode : transferMode)
    }
    unsafe fn displaysDownloadsDirectoryControl(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysDownloadsDirectoryControl)
    }
    unsafe fn setDisplaysDownloadsDirectoryControl_(&self, displaysDownloadsDirectoryControl: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysDownloadsDirectoryControl : displaysDownloadsDirectoryControl)
    }
    unsafe fn downloadsDirectory(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadsDirectory)
    }
    unsafe fn setDownloadsDirectory_(&self, downloadsDirectory: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownloadsDirectory : downloadsDirectory)
    }
    unsafe fn displaysPostProcessApplicationControl(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysPostProcessApplicationControl)
    }
    unsafe fn setDisplaysPostProcessApplicationControl_(
        &self,
        displaysPostProcessApplicationControl: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysPostProcessApplicationControl : displaysPostProcessApplicationControl)
    }
    unsafe fn postProcessApplication(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postProcessApplication)
    }
    unsafe fn setPostProcessApplication_(&self, postProcessApplication: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPostProcessApplication : postProcessApplication)
    }
    unsafe fn canRotateSelectedItemsLeft(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canRotateSelectedItemsLeft)
    }
    unsafe fn canRotateSelectedItemsRight(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canRotateSelectedItemsRight)
    }
    unsafe fn canDeleteSelectedItems(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canDeleteSelectedItems)
    }
    unsafe fn canDownloadSelectedItems(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canDownloadSelectedItems)
    }
}
pub trait PIKScannerDeviceViewDelegate: Sized + std::ops::Deref {
    unsafe fn scannerDeviceView_didScanToURL_fileData_error_(
        &self,
        scannerDeviceView: IKScannerDeviceView,
        url: NSURL,
        data: NSData,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDeviceView : scannerDeviceView, didScanToURL : url, fileData : data, error : error)
    }
    unsafe fn scannerDeviceView_didScanToURL_error_(
        &self,
        scannerDeviceView: IKScannerDeviceView,
        url: NSURL,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDeviceView : scannerDeviceView, didScanToURL : url, error : error)
    }
    unsafe fn scannerDeviceView_didScanToBandData_scanInfo_error_(
        &self,
        scannerDeviceView: IKScannerDeviceView,
        data: ICScannerBandData,
        scanInfo: NSDictionary,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDeviceView : scannerDeviceView, didScanToBandData : data, scanInfo : scanInfo, error : error)
    }
    unsafe fn scannerDeviceView_didEncounterError_(
        &self,
        scannerDeviceView: IKScannerDeviceView,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scannerDeviceView : scannerDeviceView, didEncounterError : error)
    }
}
pub type IKScannerDeviceViewTransferMode = NSInteger;
pub type IKScannerDeviceViewDisplayMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IKScannerDeviceView(pub id);
impl std::ops::Deref for IKScannerDeviceView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IKScannerDeviceView {}
impl IKScannerDeviceView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IKScannerDeviceView").unwrap(), alloc) })
    }
}
impl INSView for IKScannerDeviceView {}
impl PNSAnimatablePropertyContainer for IKScannerDeviceView {}
impl PNSUserInterfaceItemIdentification for IKScannerDeviceView {}
impl PNSDraggingDestination for IKScannerDeviceView {}
impl PNSAppearanceCustomization for IKScannerDeviceView {}
impl PNSAccessibilityElement for IKScannerDeviceView {}
impl PNSAccessibility for IKScannerDeviceView {}
impl std::convert::TryFrom<NSView> for IKScannerDeviceView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<IKScannerDeviceView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IKScannerDeviceView").unwrap()) };
        if is_kind_of {
            Ok(IKScannerDeviceView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to IKScannerDeviceView")
        }
    }
}
impl INSResponder for IKScannerDeviceView {}
impl PNSCoding for IKScannerDeviceView {}
impl INSObject for IKScannerDeviceView {}
impl PNSObject for IKScannerDeviceView {}
impl IIKScannerDeviceView for IKScannerDeviceView {}
pub trait IIKScannerDeviceView: Sized + std::ops::Deref {
    unsafe fn delegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn scannerDevice(&self) -> ICScannerDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scannerDevice)
    }
    unsafe fn setScannerDevice_(&self, scannerDevice: ICScannerDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScannerDevice : scannerDevice)
    }
    unsafe fn mode(&self) -> IKScannerDeviceViewDisplayMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn setMode_(&self, mode: IKScannerDeviceViewDisplayMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMode : mode)
    }
    unsafe fn hasDisplayModeSimple(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasDisplayModeSimple)
    }
    unsafe fn setHasDisplayModeSimple_(&self, hasDisplayModeSimple: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasDisplayModeSimple : hasDisplayModeSimple)
    }
    unsafe fn hasDisplayModeAdvanced(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasDisplayModeAdvanced)
    }
    unsafe fn setHasDisplayModeAdvanced_(&self, hasDisplayModeAdvanced: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasDisplayModeAdvanced : hasDisplayModeAdvanced)
    }
    unsafe fn transferMode(&self) -> IKScannerDeviceViewTransferMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transferMode)
    }
    unsafe fn setTransferMode_(&self, transferMode: IKScannerDeviceViewTransferMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransferMode : transferMode)
    }
    unsafe fn scanControlLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scanControlLabel)
    }
    unsafe fn setScanControlLabel_(&self, scanControlLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScanControlLabel : scanControlLabel)
    }
    unsafe fn overviewControlLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overviewControlLabel)
    }
    unsafe fn setOverviewControlLabel_(&self, overviewControlLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverviewControlLabel : overviewControlLabel)
    }
    unsafe fn displaysDownloadsDirectoryControl(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysDownloadsDirectoryControl)
    }
    unsafe fn setDisplaysDownloadsDirectoryControl_(&self, displaysDownloadsDirectoryControl: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysDownloadsDirectoryControl : displaysDownloadsDirectoryControl)
    }
    unsafe fn downloadsDirectory(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadsDirectory)
    }
    unsafe fn setDownloadsDirectory_(&self, downloadsDirectory: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownloadsDirectory : downloadsDirectory)
    }
    unsafe fn documentName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentName)
    }
    unsafe fn setDocumentName_(&self, documentName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDocumentName : documentName)
    }
    unsafe fn displaysPostProcessApplicationControl(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysPostProcessApplicationControl)
    }
    unsafe fn setDisplaysPostProcessApplicationControl_(
        &self,
        displaysPostProcessApplicationControl: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysPostProcessApplicationControl : displaysPostProcessApplicationControl)
    }
    unsafe fn postProcessApplication(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postProcessApplication)
    }
    unsafe fn setPostProcessApplication_(&self, postProcessApplication: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPostProcessApplication : postProcessApplication)
    }
}
unsafe extern "C" {
    pub static QCPlugInAttributeNameKey: NSString;
}
unsafe extern "C" {
    pub static QCPlugInAttributeDescriptionKey: NSString;
}
unsafe extern "C" {
    pub static QCPlugInAttributeCopyrightKey: NSString;
}
unsafe extern "C" {
    pub static QCPlugInAttributeCategoriesKey: NSString;
}
unsafe extern "C" {
    pub static QCPlugInAttributeExamplesKey: NSString;
}
unsafe extern "C" {
    pub static QCPortAttributeTypeKey: NSString;
}
unsafe extern "C" {
    pub static QCPortAttributeNameKey: NSString;
}
unsafe extern "C" {
    pub static QCPortAttributeMinimumValueKey: NSString;
}
unsafe extern "C" {
    pub static QCPortAttributeMaximumValueKey: NSString;
}
unsafe extern "C" {
    pub static QCPortAttributeDefaultValueKey: NSString;
}
unsafe extern "C" {
    pub static QCPortAttributeMenuItemsKey: NSString;
}
unsafe extern "C" {
    pub static QCPortTypeBoolean: NSString;
}
unsafe extern "C" {
    pub static QCPortTypeIndex: NSString;
}
unsafe extern "C" {
    pub static QCPortTypeNumber: NSString;
}
unsafe extern "C" {
    pub static QCPortTypeString: NSString;
}
unsafe extern "C" {
    pub static QCPortTypeColor: NSString;
}
unsafe extern "C" {
    pub static QCPortTypeImage: NSString;
}
unsafe extern "C" {
    pub static QCPortTypeStructure: NSString;
}
unsafe extern "C" {
    pub static QCPlugInPixelFormatARGB8: NSString;
}
unsafe extern "C" {
    pub static QCPlugInPixelFormatBGRA8: NSString;
}
unsafe extern "C" {
    pub static QCPlugInPixelFormatRGBAf: NSString;
}
unsafe extern "C" {
    pub static QCPlugInPixelFormatI8: NSString;
}
unsafe extern "C" {
    pub static QCPlugInPixelFormatIf: NSString;
}
unsafe extern "C" {
    pub static QCPlugInExecutionArgumentEventKey: NSString;
}
unsafe extern "C" {
    pub static QCPlugInExecutionArgumentMouseLocationKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionAttributeNameKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionAttributeDescriptionKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionAttributeCopyrightKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionAttributeBuiltInKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionAttributeIsTimeDependentKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionAttributeHasConsumersKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionAttributeCategoryKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionCategoryDistortion: NSString;
}
unsafe extern "C" {
    pub static QCCompositionCategoryStylize: NSString;
}
unsafe extern "C" {
    pub static QCCompositionCategoryUtility: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputImageKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputSourceImageKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputDestinationImageKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputPreviewModeKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputXKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputYKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputScreenImageKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputAudioPeakKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputAudioSpectrumKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputTrackPositionKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputTrackInfoKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputTrackSignalKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputPrimaryColorKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputSecondaryColorKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionInputPaceKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionOutputImageKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionOutputWebPageURLKey: NSString;
}
unsafe extern "C" {
    pub static QCCompositionProtocolGraphicAnimation: NSString;
}
unsafe extern "C" {
    pub static QCCompositionProtocolGraphicTransition: NSString;
}
unsafe extern "C" {
    pub static QCCompositionProtocolImageFilter: NSString;
}
unsafe extern "C" {
    pub static QCCompositionProtocolScreenSaver: NSString;
}
unsafe extern "C" {
    pub static QCCompositionProtocolMusicVisualizer: NSString;
}
unsafe extern "C" {
    pub static QCCompositionRepositoryDidUpdateNotification: NSString;
}
unsafe extern "C" {
    pub static QCRendererEventKey: NSString;
}
unsafe extern "C" {
    pub static QCRendererMouseLocationKey: NSString;
}
unsafe extern "C" {
    pub static QCViewDidStartRenderingNotification: NSString;
}
unsafe extern "C" {
    pub static QCViewDidStopRenderingNotification: NSString;
}
unsafe extern "C" {
    pub static QCCompositionPickerViewDidSelectCompositionNotification: NSString;
}
unsafe extern "C" {
    pub static QCCompositionPickerPanelDidSelectCompositionNotification: NSString;
}
unsafe extern "C" {
    pub static mut globalUpdateOK: Boolean;
}
unsafe extern "C" {
    pub static mut kQuartzFilterApplicationDomain: NSString;
}
unsafe extern "C" {
    pub static mut kQuartzFilterPDFWorkflowDomain: NSString;
}
unsafe extern "C" {
    pub static mut kQuartzFilterPrintingDomain: NSString;
}
unsafe extern "C" {
    pub static mut kQuartzFilterManagerDidAddFilterNotification: NSString;
}
unsafe extern "C" {
    pub static mut kQuartzFilterManagerDidRemoveFilterNotification: NSString;
}
unsafe extern "C" {
    pub static mut kQuartzFilterManagerDidModifyFilterNotification: NSString;
}
unsafe extern "C" {
    pub static mut kQuartzFilterManagerDidSelectFilterNotification: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserPathRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserNSURLRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserNSImageRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCGImageRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCGImageSourceRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserNSDataRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserNSBitmapImageRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserQTMovieRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserQTMoviePathRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserQCCompositionRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserQCCompositionPathRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserQuickLookPathRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserIconRefPathRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserIconRefRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserPDFPageRepresentationType: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserBackgroundColorKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserSelectionColorKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCellsOutlineColorKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCellsTitleAttributesKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCellsHighlightedTitleAttributesKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCellsSubtitleAttributesKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserGroupRangeKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserGroupBackgroundColorKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserGroupTitleKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserGroupStyleKey: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserGroupHeaderLayer: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserGroupFooterLayer: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCellBackgroundLayer: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCellForegroundLayer: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCellSelectionLayer: NSString;
}
unsafe extern "C" {
    pub static IKImageBrowserCellPlaceHolderLayer: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerAllowsVideoCaptureKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerAllowsFileChoosingKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerShowRecentPictureKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerUpdateRecentPictureKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerAllowsEditingKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerShowEffectsKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerInformationalTextKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerImageTransformsKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerOutputImageMaxSizeKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerShowAddressBookPictureKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerShowEmptyPictureKey: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerRemainOpenAfterValidateKey: NSString;
}
unsafe extern "C" {
    pub static mut IKUISizeFlavor: NSString;
}
unsafe extern "C" {
    pub static mut IKUISizeMini: NSString;
}
unsafe extern "C" {
    pub static mut IKUISizeSmall: NSString;
}
unsafe extern "C" {
    pub static mut IKUISizeRegular: NSString;
}
unsafe extern "C" {
    pub static mut IKUImaxSize: NSString;
}
unsafe extern "C" {
    pub static mut IKUIFlavorAllowFallback: NSString;
}
unsafe extern "C" {
    pub static IKFilterBrowserFilterSelectedNotification: NSString;
}
unsafe extern "C" {
    pub static IKFilterBrowserFilterDoubleClickNotification: NSString;
}
unsafe extern "C" {
    pub static IKFilterBrowserWillPreviewFilterNotification: NSString;
}
unsafe extern "C" {
    pub static IKFilterBrowserShowCategories: NSString;
}
unsafe extern "C" {
    pub static IKFilterBrowserShowPreview: NSString;
}
unsafe extern "C" {
    pub static IKFilterBrowserExcludeCategories: NSString;
}
unsafe extern "C" {
    pub static IKFilterBrowserExcludeFilters: NSString;
}
unsafe extern "C" {
    pub static IKFilterBrowserDefaultInputImage: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowModeImages: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowModePDF: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowModeOther: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowWrapAround: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowStartPaused: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowStartIndex: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowScreen: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowAudioFile: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowPDFDisplayBox: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowPDFDisplayMode: NSString;
}
unsafe extern "C" {
    pub static IKSlideshowPDFDisplaysAsBook: NSString;
}
unsafe extern "C" {
    pub static IK_iPhotoBundleIdentifier: NSString;
}
unsafe extern "C" {
    pub static IK_ApertureBundleIdentifier: NSString;
}
unsafe extern "C" {
    pub static IK_MailBundleIdentifier: NSString;
}
unsafe extern "C" {
    pub static IK_PhotosBundleIdentifier: NSString;
}
unsafe extern "C" {
    pub static IKToolModeNone: NSString;
}
unsafe extern "C" {
    pub static IKToolModeMove: NSString;
}
unsafe extern "C" {
    pub static IKToolModeSelect: NSString;
}
unsafe extern "C" {
    pub static IKToolModeSelectRect: NSString;
}
unsafe extern "C" {
    pub static IKToolModeSelectEllipse: NSString;
}
unsafe extern "C" {
    pub static IKToolModeSelectLasso: NSString;
}
unsafe extern "C" {
    pub static IKToolModeCrop: NSString;
}
unsafe extern "C" {
    pub static IKToolModeRotate: NSString;
}
unsafe extern "C" {
    pub static IKToolModeAnnotate: NSString;
}
unsafe extern "C" {
    pub static IKOverlayTypeBackground: NSString;
}
unsafe extern "C" {
    pub static IKOverlayTypeImage: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerShowAddressBookPicture: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerShowEmptyPicture: NSString;
}
unsafe extern "C" {
    pub static IKPictureTakerCropAreaSizeKey: NSString;
}

unsafe impl objc2::encode::RefEncode for QCPlugIn {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCPlugIn {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCComposition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCComposition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCCompositionRepository {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCCompositionRepository {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCCompositionLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCCompositionLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCPatchController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCPatchController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCCompositionParameterView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCCompositionParameterView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCCompositionPickerView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCCompositionPickerView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCCompositionPickerPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCCompositionPickerPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QCPlugInViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QCPlugInViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QuartzFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QuartzFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QuartzFilterView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QuartzFilterView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QuartzFilterManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QuartzFilterManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKImageBrowserView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKImageBrowserView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKImageBrowserCell {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKImageBrowserCell {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKPictureTaker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKPictureTaker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKFilterUIView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKFilterUIView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKFilterBrowserPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKFilterBrowserPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKFilterBrowserView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKFilterBrowserView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKSlideshow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKSlideshow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKSaveOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKSaveOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKImageView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKImageView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKImageEditPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKImageEditPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKDeviceBrowserView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKDeviceBrowserView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKCameraDeviceView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKCameraDeviceView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IKScannerDeviceView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IKScannerDeviceView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
