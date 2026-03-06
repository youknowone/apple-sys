#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::ModelIO::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTKView(pub id);
impl std::ops::Deref for MTKView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTKView {}
impl MTKView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTKView").unwrap(), alloc) })
    }
}
impl PNSCoding for MTKView {}
impl INSView for MTKView {}
impl PNSAnimatablePropertyContainer for MTKView {}
impl PNSUserInterfaceItemIdentification for MTKView {}
impl PNSDraggingDestination for MTKView {}
impl PNSAppearanceCustomization for MTKView {}
impl PNSAccessibilityElement for MTKView {}
impl PNSAccessibility for MTKView {}
impl std::convert::TryFrom<NSView> for MTKView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<MTKView, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTKView").unwrap()) };
        if is_kind_of {
            Ok(MTKView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to MTKView")
        }
    }
}
impl INSResponder for MTKView {}
impl INSObject for MTKView {}
impl PNSObject for MTKView {}
impl IMTKView for MTKView {}
pub trait IMTKView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_device_(&self, frameRect: CGRect, device: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frameRect, device : device)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
    unsafe fn releaseDrawables(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseDrawables)
    }
    unsafe fn draw(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, draw)
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
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn setDevice_(&self, device: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDevice : device)
    }
    unsafe fn currentDrawable(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentDrawable)
    }
    unsafe fn framebufferOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, framebufferOnly)
    }
    unsafe fn setFramebufferOnly_(&self, framebufferOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFramebufferOnly : framebufferOnly)
    }
    unsafe fn depthStencilAttachmentTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthStencilAttachmentTextureUsage)
    }
    unsafe fn setDepthStencilAttachmentTextureUsage_(
        &self,
        depthStencilAttachmentTextureUsage: MTLTextureUsage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStencilAttachmentTextureUsage : depthStencilAttachmentTextureUsage)
    }
    unsafe fn multisampleColorAttachmentTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multisampleColorAttachmentTextureUsage)
    }
    unsafe fn setMultisampleColorAttachmentTextureUsage_(
        &self,
        multisampleColorAttachmentTextureUsage: MTLTextureUsage,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMultisampleColorAttachmentTextureUsage : multisampleColorAttachmentTextureUsage)
    }
    unsafe fn presentsWithTransaction(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentsWithTransaction)
    }
    unsafe fn setPresentsWithTransaction_(&self, presentsWithTransaction: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresentsWithTransaction : presentsWithTransaction)
    }
    unsafe fn colorPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorPixelFormat)
    }
    unsafe fn setColorPixelFormat_(&self, colorPixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorPixelFormat : colorPixelFormat)
    }
    unsafe fn depthStencilPixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthStencilPixelFormat)
    }
    unsafe fn setDepthStencilPixelFormat_(&self, depthStencilPixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStencilPixelFormat : depthStencilPixelFormat)
    }
    unsafe fn depthStencilStorageMode(&self) -> MTLStorageMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthStencilStorageMode)
    }
    unsafe fn setDepthStencilStorageMode_(&self, depthStencilStorageMode: MTLStorageMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthStencilStorageMode : depthStencilStorageMode)
    }
    unsafe fn sampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
    unsafe fn setSampleCount_(&self, sampleCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleCount : sampleCount)
    }
    unsafe fn clearColor(&self) -> MTLClearColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearColor)
    }
    unsafe fn setClearColor_(&self, clearColor: MTLClearColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClearColor : clearColor)
    }
    unsafe fn clearDepth(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearDepth)
    }
    unsafe fn setClearDepth_(&self, clearDepth: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClearDepth : clearDepth)
    }
    unsafe fn clearStencil(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearStencil)
    }
    unsafe fn setClearStencil_(&self, clearStencil: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClearStencil : clearStencil)
    }
    unsafe fn depthStencilTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthStencilTexture)
    }
    unsafe fn multisampleColorTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multisampleColorTexture)
    }
    unsafe fn currentRenderPassDescriptor(&self) -> MTLRenderPassDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentRenderPassDescriptor)
    }
    unsafe fn currentMTL4RenderPassDescriptor(&self) -> MTL4RenderPassDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentMTL4RenderPassDescriptor)
    }
    unsafe fn preferredFramesPerSecond(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFramesPerSecond)
    }
    unsafe fn setPreferredFramesPerSecond_(&self, preferredFramesPerSecond: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredFramesPerSecond : preferredFramesPerSecond)
    }
    unsafe fn enableSetNeedsDisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableSetNeedsDisplay)
    }
    unsafe fn setEnableSetNeedsDisplay_(&self, enableSetNeedsDisplay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnableSetNeedsDisplay : enableSetNeedsDisplay)
    }
    unsafe fn autoResizeDrawable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoResizeDrawable)
    }
    unsafe fn setAutoResizeDrawable_(&self, autoResizeDrawable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoResizeDrawable : autoResizeDrawable)
    }
    unsafe fn drawableSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawableSize)
    }
    unsafe fn setDrawableSize_(&self, drawableSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawableSize : drawableSize)
    }
    unsafe fn preferredDrawableSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDrawableSize)
    }
    unsafe fn preferredDevice(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDevice)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
    unsafe fn setPaused_(&self, paused: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaused : paused)
    }
    unsafe fn colorspace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorspace)
    }
    unsafe fn setColorspace_(&self, colorspace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorspace : colorspace)
    }
}
pub trait PMTKViewDelegate: Sized + std::ops::Deref {
    unsafe fn mtkView_drawableSizeWillChange_(&self, view: MTKView, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mtkView : view, drawableSizeWillChange : size)
    }
    unsafe fn drawInMTKView_(&self, view: MTKView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawInMTKView : view)
    }
}
pub type MTKTextureLoaderError = NSString;
pub type MTKTextureLoaderOption = NSString;
pub type MTKTextureLoaderCubeLayout = NSString;
pub type MTKTextureLoaderOrigin = NSString;
pub type MTKTextureLoaderCallback = *mut ::std::os::raw::c_void;
pub type MTKTextureLoaderArrayCallback = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTKTextureLoader(pub id);
impl std::ops::Deref for MTKTextureLoader {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTKTextureLoader {}
impl MTKTextureLoader {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTKTextureLoader").unwrap(), alloc) })
    }
}
impl INSObject for MTKTextureLoader {}
impl PNSObject for MTKTextureLoader {}
impl std::convert::TryFrom<NSObject> for MTKTextureLoader {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTKTextureLoader, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTKTextureLoader").unwrap()) };
        if is_kind_of {
            Ok(MTKTextureLoader(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTKTextureLoader")
        }
    }
}
impl IMTKTextureLoader for MTKTextureLoader {}
pub trait IMTKTextureLoader: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDevice_(&self, device: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDevice : device)
    }
    unsafe fn newTextureWithContentsOfURL_options_completionHandler_(
        &self,
        URL: NSURL,
        options: NSDictionary,
        completionHandler: MTKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithContentsOfURL : URL, options : options, completionHandler : completionHandler)
    }
    unsafe fn newTextureWithName_scaleFactor_bundle_options_completionHandler_(
        &self,
        name: NSString,
        scaleFactor: CGFloat,
        bundle: NSBundle,
        options: NSDictionary,
        completionHandler: MTKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithName : name, scaleFactor : scaleFactor, bundle : bundle, options : options, completionHandler : completionHandler)
    }
    unsafe fn newTextureWithName_scaleFactor_displayGamut_bundle_options_completionHandler_(
        &self,
        name: NSString,
        scaleFactor: CGFloat,
        displayGamut: NSDisplayGamut,
        bundle: NSBundle,
        options: NSDictionary,
        completionHandler: MTKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithName : name, scaleFactor : scaleFactor, displayGamut : displayGamut, bundle : bundle, options : options, completionHandler : completionHandler)
    }
    unsafe fn newTexturesWithContentsOfURLs_options_completionHandler_(
        &self,
        URLs: NSArray,
        options: NSDictionary,
        completionHandler: MTKTextureLoaderArrayCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTexturesWithContentsOfURLs : URLs, options : options, completionHandler : completionHandler)
    }
    unsafe fn newTexturesWithNames_scaleFactor_bundle_options_completionHandler_(
        &self,
        names: NSArray,
        scaleFactor: CGFloat,
        bundle: NSBundle,
        options: NSDictionary,
        completionHandler: MTKTextureLoaderArrayCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTexturesWithNames : names, scaleFactor : scaleFactor, bundle : bundle, options : options, completionHandler : completionHandler)
    }
    unsafe fn newTexturesWithNames_scaleFactor_displayGamut_bundle_options_completionHandler_(
        &self,
        names: NSArray,
        scaleFactor: CGFloat,
        displayGamut: NSDisplayGamut,
        bundle: NSBundle,
        options: NSDictionary,
        completionHandler: MTKTextureLoaderArrayCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTexturesWithNames : names, scaleFactor : scaleFactor, displayGamut : displayGamut, bundle : bundle, options : options, completionHandler : completionHandler)
    }
    unsafe fn newTextureWithData_options_completionHandler_(
        &self,
        data: NSData,
        options: NSDictionary,
        completionHandler: MTKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithData : data, options : options, completionHandler : completionHandler)
    }
    unsafe fn newTextureWithCGImage_options_completionHandler_(
        &self,
        cgImage: CGImageRef,
        options: NSDictionary,
        completionHandler: MTKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithCGImage : cgImage, options : options, completionHandler : completionHandler)
    }
    unsafe fn newTextureWithMDLTexture_options_completionHandler_(
        &self,
        texture: MDLTexture,
        options: NSDictionary,
        completionHandler: MTKTextureLoaderCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithMDLTexture : texture, options : options, completionHandler : completionHandler)
    }
    unsafe fn newTextureWithContentsOfURL_options_error_(
        &self,
        URL: NSURL,
        options: NSDictionary,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithContentsOfURL : URL, options : options, error : error)
    }
    unsafe fn newTexturesWithContentsOfURLs_options_error_(
        &self,
        URLs: NSArray,
        options: NSDictionary,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTexturesWithContentsOfURLs : URLs, options : options, error : error)
    }
    unsafe fn newTextureWithData_options_error_(
        &self,
        data: NSData,
        options: NSDictionary,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithData : data, options : options, error : error)
    }
    unsafe fn newTextureWithCGImage_options_error_(
        &self,
        cgImage: CGImageRef,
        options: NSDictionary,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithCGImage : cgImage, options : options, error : error)
    }
    unsafe fn newTextureWithMDLTexture_options_error_(
        &self,
        texture: MDLTexture,
        options: NSDictionary,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithMDLTexture : texture, options : options, error : error)
    }
    unsafe fn newTextureWithName_scaleFactor_bundle_options_error_(
        &self,
        name: NSString,
        scaleFactor: CGFloat,
        bundle: NSBundle,
        options: NSDictionary,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithName : name, scaleFactor : scaleFactor, bundle : bundle, options : options, error : error)
    }
    unsafe fn newTextureWithName_scaleFactor_displayGamut_bundle_options_error_(
        &self,
        name: NSString,
        scaleFactor: CGFloat,
        displayGamut: NSDisplayGamut,
        bundle: NSBundle,
        options: NSDictionary,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithName : name, scaleFactor : scaleFactor, displayGamut : displayGamut, bundle : bundle, options : options, error : error)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
}
pub type MTKModelError = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTKMeshBufferAllocator(pub id);
impl std::ops::Deref for MTKMeshBufferAllocator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTKMeshBufferAllocator {}
impl MTKMeshBufferAllocator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTKMeshBufferAllocator").unwrap(), alloc) })
    }
}
impl PMDLMeshBufferAllocator for MTKMeshBufferAllocator {}
impl INSObject for MTKMeshBufferAllocator {}
impl PNSObject for MTKMeshBufferAllocator {}
impl std::convert::TryFrom<NSObject> for MTKMeshBufferAllocator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTKMeshBufferAllocator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTKMeshBufferAllocator").unwrap()) };
        if is_kind_of {
            Ok(MTKMeshBufferAllocator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTKMeshBufferAllocator")
        }
    }
}
impl IMTKMeshBufferAllocator for MTKMeshBufferAllocator {}
pub trait IMTKMeshBufferAllocator: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDevice_(&self, device: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDevice : device)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTKMeshBuffer(pub id);
impl std::ops::Deref for MTKMeshBuffer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTKMeshBuffer {}
impl MTKMeshBuffer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTKMeshBuffer").unwrap(), alloc) })
    }
}
impl PMDLMeshBuffer for MTKMeshBuffer {}
impl PMDLNamed for MTKMeshBuffer {}
impl INSObject for MTKMeshBuffer {}
impl PNSObject for MTKMeshBuffer {}
impl std::convert::TryFrom<NSObject> for MTKMeshBuffer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTKMeshBuffer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTKMeshBuffer").unwrap()) };
        if is_kind_of {
            Ok(MTKMeshBuffer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTKMeshBuffer")
        }
    }
}
impl IMTKMeshBuffer for MTKMeshBuffer {}
pub trait IMTKMeshBuffer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn length(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn allocator(&self) -> MTKMeshBufferAllocator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocator)
    }
    unsafe fn zone(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zone)
    }
    unsafe fn buffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buffer)
    }
    unsafe fn offset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn type_(&self) -> MDLMeshBufferType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTKSubmesh(pub id);
impl std::ops::Deref for MTKSubmesh {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTKSubmesh {}
impl MTKSubmesh {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTKSubmesh").unwrap(), alloc) })
    }
}
impl INSObject for MTKSubmesh {}
impl PNSObject for MTKSubmesh {}
impl std::convert::TryFrom<NSObject> for MTKSubmesh {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTKSubmesh, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTKSubmesh").unwrap()) };
        if is_kind_of {
            Ok(MTKSubmesh(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTKSubmesh")
        }
    }
}
impl IMTKSubmesh for MTKSubmesh {}
pub trait IMTKSubmesh: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn primitiveType(&self) -> MTLPrimitiveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primitiveType)
    }
    unsafe fn indexType(&self) -> MTLIndexType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexType)
    }
    unsafe fn indexBuffer(&self) -> MTKMeshBuffer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexBuffer)
    }
    unsafe fn indexCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexCount)
    }
    unsafe fn mesh(&self) -> MTKMesh
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mesh)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTKMesh(pub id);
impl std::ops::Deref for MTKMesh {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTKMesh {}
impl MTKMesh {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTKMesh").unwrap(), alloc) })
    }
}
impl INSObject for MTKMesh {}
impl PNSObject for MTKMesh {}
impl std::convert::TryFrom<NSObject> for MTKMesh {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTKMesh, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTKMesh").unwrap()) };
        if is_kind_of {
            Ok(MTKMesh(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTKMesh")
        }
    }
}
impl IMTKMesh for MTKMesh {}
pub trait IMTKMesh: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMesh_device_error_(
        &self,
        mesh: MDLMesh,
        device: *mut u64,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMesh : mesh, device : device, error : error)
    }
    unsafe fn vertexBuffers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexBuffers)
    }
    unsafe fn vertexDescriptor(&self) -> MDLVertexDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexDescriptor)
    }
    unsafe fn submeshes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, submeshes)
    }
    unsafe fn vertexCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexCount)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn newMeshesFromAsset_device_sourceMeshes_error_(
        asset: MDLAsset,
        device: *mut u64,
        sourceMeshes: *mut NSArray,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTKMesh").unwrap(), newMeshesFromAsset : asset, device : device, sourceMeshes : sourceMeshes, error : error)
    }
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderErrorDomain: MTKTextureLoaderError;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderErrorKey: MTKTextureLoaderError;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOptionAllocateMipmaps: MTKTextureLoaderOption;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOptionGenerateMipmaps: MTKTextureLoaderOption;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOptionSRGB: MTKTextureLoaderOption;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOptionTextureUsage: MTKTextureLoaderOption;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOptionTextureCPUCacheMode: MTKTextureLoaderOption;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOptionTextureStorageMode: MTKTextureLoaderOption;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOptionCubeLayout: MTKTextureLoaderOption;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderCubeLayoutVertical: MTKTextureLoaderCubeLayout;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOptionOrigin: MTKTextureLoaderOption;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOriginTopLeft: MTKTextureLoaderOrigin;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOriginBottomLeft: MTKTextureLoaderOrigin;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOriginFlippedVertically: MTKTextureLoaderOrigin;
}
unsafe extern "C" {
    pub static mut MTKTextureLoaderOptionLoadAsArray: MTKTextureLoaderOption;
}
unsafe extern "C" {
    pub static mut MTKModelErrorDomain: MTKModelError;
}
unsafe extern "C" {
    pub static mut MTKModelErrorKey: MTKModelError;
}
unsafe extern "C" {
    pub fn MTKModelIOVertexDescriptorFromMetal(
        metalDescriptor: MTLVertexDescriptor,
    ) -> MDLVertexDescriptor;
}
unsafe extern "C" {
    pub fn MTKModelIOVertexDescriptorFromMetalWithError(
        metalDescriptor: MTLVertexDescriptor,
        error: *mut NSError,
    ) -> MDLVertexDescriptor;
}
unsafe extern "C" {
    pub fn MTKMetalVertexDescriptorFromModelIO(
        modelIODescriptor: MDLVertexDescriptor,
    ) -> MTLVertexDescriptor;
}
unsafe extern "C" {
    pub fn MTKMetalVertexDescriptorFromModelIOWithError(
        modelIODescriptor: MDLVertexDescriptor,
        error: *mut NSError,
    ) -> MTLVertexDescriptor;
}
unsafe extern "C" {
    pub fn MTKModelIOVertexFormatFromMetal(vertexFormat: MTLVertexFormat) -> MDLVertexFormat;
}
unsafe extern "C" {
    pub fn MTKMetalVertexFormatFromModelIO(vertexFormat: MDLVertexFormat) -> MTLVertexFormat;
}

unsafe impl objc2::encode::RefEncode for MTKView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTKView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTKTextureLoader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTKTextureLoader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTKMeshBufferAllocator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTKMeshBufferAllocator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTKMeshBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTKMeshBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTKSubmesh {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTKSubmesh {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTKMesh {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTKMesh {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
