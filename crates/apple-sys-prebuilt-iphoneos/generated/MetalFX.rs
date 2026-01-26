#[allow(unused_imports)]
use crate::objc::*;
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
pub struct MTLFXTemporalScalerDescriptor(pub id);
impl std::ops::Deref for MTLFXTemporalScalerDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFXTemporalScalerDescriptor {}
impl MTLFXTemporalScalerDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalScalerDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLFXTemporalScalerDescriptor {}
impl INSObject for MTLFXTemporalScalerDescriptor {}
impl PNSObject for MTLFXTemporalScalerDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLFXTemporalScalerDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFXTemporalScalerDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFXTemporalScalerDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLFXTemporalScalerDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFXTemporalScalerDescriptor")
        }
    }
}
impl IMTLFXTemporalScalerDescriptor for MTLFXTemporalScalerDescriptor {}
pub trait IMTLFXTemporalScalerDescriptor: Sized + std::ops::Deref {
    unsafe fn newTemporalScalerWithDevice_(&self, device: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTemporalScalerWithDevice : device)
    }
    unsafe fn newTemporalScalerWithDevice_compiler_(
        &self,
        device: *mut u64,
        compiler: *mut u64,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTemporalScalerWithDevice : device, compiler : compiler)
    }
    unsafe fn colorTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureFormat)
    }
    unsafe fn setColorTextureFormat_(&self, colorTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorTextureFormat : colorTextureFormat)
    }
    unsafe fn depthTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTextureFormat)
    }
    unsafe fn setDepthTextureFormat_(&self, depthTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthTextureFormat : depthTextureFormat)
    }
    unsafe fn motionTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTextureFormat)
    }
    unsafe fn setMotionTextureFormat_(&self, motionTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTextureFormat : motionTextureFormat)
    }
    unsafe fn outputTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureFormat)
    }
    unsafe fn setOutputTextureFormat_(&self, outputTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputTextureFormat : outputTextureFormat)
    }
    unsafe fn inputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWidth)
    }
    unsafe fn setInputWidth_(&self, inputWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputWidth : inputWidth)
    }
    unsafe fn inputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputHeight)
    }
    unsafe fn setInputHeight_(&self, inputHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputHeight : inputHeight)
    }
    unsafe fn outputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputWidth)
    }
    unsafe fn setOutputWidth_(&self, outputWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputWidth : outputWidth)
    }
    unsafe fn outputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputHeight)
    }
    unsafe fn setOutputHeight_(&self, outputHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputHeight : outputHeight)
    }
    unsafe fn isAutoExposureEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutoExposureEnabled)
    }
    unsafe fn setAutoExposureEnabled_(&self, autoExposureEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoExposureEnabled : autoExposureEnabled)
    }
    unsafe fn requiresSynchronousInitialization(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresSynchronousInitialization)
    }
    unsafe fn setRequiresSynchronousInitialization_(&self, requiresSynchronousInitialization: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresSynchronousInitialization : requiresSynchronousInitialization)
    }
    unsafe fn isInputContentPropertiesEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInputContentPropertiesEnabled)
    }
    unsafe fn setInputContentPropertiesEnabled_(&self, inputContentPropertiesEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputContentPropertiesEnabled : inputContentPropertiesEnabled)
    }
    unsafe fn inputContentMinScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentMinScale)
    }
    unsafe fn setInputContentMinScale_(&self, inputContentMinScale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputContentMinScale : inputContentMinScale)
    }
    unsafe fn inputContentMaxScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentMaxScale)
    }
    unsafe fn setInputContentMaxScale_(&self, inputContentMaxScale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputContentMaxScale : inputContentMaxScale)
    }
    unsafe fn isReactiveMaskTextureEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReactiveMaskTextureEnabled)
    }
    unsafe fn setReactiveMaskTextureEnabled_(&self, reactiveMaskTextureEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReactiveMaskTextureEnabled : reactiveMaskTextureEnabled)
    }
    unsafe fn reactiveMaskTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactiveMaskTextureFormat)
    }
    unsafe fn setReactiveMaskTextureFormat_(&self, reactiveMaskTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReactiveMaskTextureFormat : reactiveMaskTextureFormat)
    }
    unsafe fn supportedInputContentMinScaleForDevice_(device: *mut u64) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalScalerDescriptor").unwrap(), supportedInputContentMinScaleForDevice : device)
    }
    unsafe fn supportedInputContentMaxScaleForDevice_(device: *mut u64) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalScalerDescriptor").unwrap(), supportedInputContentMaxScaleForDevice : device)
    }
    unsafe fn supportsDevice_(device: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalScalerDescriptor").unwrap(), supportsDevice : device)
    }
    unsafe fn supportsMetal4FX_(device: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalScalerDescriptor").unwrap(), supportsMetal4FX : device)
    }
}
pub trait PMTLFXFrameInterpolatableScaler: Sized + std::ops::Deref {}
pub trait PMTLFXTemporalScalerBase: Sized + std::ops::Deref {
    unsafe fn colorTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureUsage)
    }
    unsafe fn depthTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTextureUsage)
    }
    unsafe fn motionTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTextureUsage)
    }
    unsafe fn reactiveTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactiveTextureUsage)
    }
    unsafe fn outputTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureUsage)
    }
    unsafe fn inputContentWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentWidth)
    }
    unsafe fn setInputContentWidth_(&self, inputContentWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputContentWidth : inputContentWidth)
    }
    unsafe fn inputContentHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentHeight)
    }
    unsafe fn setInputContentHeight_(&self, inputContentHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputContentHeight : inputContentHeight)
    }
    unsafe fn colorTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTexture)
    }
    unsafe fn setColorTexture_(&self, colorTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorTexture : colorTexture)
    }
    unsafe fn depthTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTexture)
    }
    unsafe fn setDepthTexture_(&self, depthTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthTexture : depthTexture)
    }
    unsafe fn motionTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTexture)
    }
    unsafe fn setMotionTexture_(&self, motionTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTexture : motionTexture)
    }
    unsafe fn outputTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTexture)
    }
    unsafe fn setOutputTexture_(&self, outputTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputTexture : outputTexture)
    }
    unsafe fn exposureTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureTexture)
    }
    unsafe fn setExposureTexture_(&self, exposureTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureTexture : exposureTexture)
    }
    unsafe fn reactiveMaskTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactiveMaskTexture)
    }
    unsafe fn setReactiveMaskTexture_(&self, reactiveMaskTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReactiveMaskTexture : reactiveMaskTexture)
    }
    unsafe fn preExposure(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preExposure)
    }
    unsafe fn setPreExposure_(&self, preExposure: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreExposure : preExposure)
    }
    unsafe fn jitterOffsetX(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jitterOffsetX)
    }
    unsafe fn setJitterOffsetX_(&self, jitterOffsetX: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJitterOffsetX : jitterOffsetX)
    }
    unsafe fn jitterOffsetY(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jitterOffsetY)
    }
    unsafe fn setJitterOffsetY_(&self, jitterOffsetY: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJitterOffsetY : jitterOffsetY)
    }
    unsafe fn motionVectorScaleX(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionVectorScaleX)
    }
    unsafe fn setMotionVectorScaleX_(&self, motionVectorScaleX: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionVectorScaleX : motionVectorScaleX)
    }
    unsafe fn motionVectorScaleY(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionVectorScaleY)
    }
    unsafe fn setMotionVectorScaleY_(&self, motionVectorScaleY: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionVectorScaleY : motionVectorScaleY)
    }
    unsafe fn reset(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn setReset_(&self, reset: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReset : reset)
    }
    unsafe fn isDepthReversed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDepthReversed)
    }
    unsafe fn setDepthReversed_(&self, depthReversed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthReversed : depthReversed)
    }
    unsafe fn colorTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureFormat)
    }
    unsafe fn depthTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTextureFormat)
    }
    unsafe fn motionTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTextureFormat)
    }
    unsafe fn reactiveMaskTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactiveMaskTextureFormat)
    }
    unsafe fn outputTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureFormat)
    }
    unsafe fn inputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWidth)
    }
    unsafe fn inputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputHeight)
    }
    unsafe fn outputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputWidth)
    }
    unsafe fn outputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputHeight)
    }
    unsafe fn inputContentMinScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentMinScale)
    }
    unsafe fn inputContentMaxScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentMaxScale)
    }
    unsafe fn fence(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fence)
    }
    unsafe fn setFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFence : fence)
    }
}
pub trait PMTLFXTemporalScaler: Sized + std::ops::Deref {
    unsafe fn encodeToCommandBuffer_(&self, commandBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFXTemporalDenoisedScalerDescriptor(pub id);
impl std::ops::Deref for MTLFXTemporalDenoisedScalerDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFXTemporalDenoisedScalerDescriptor {}
impl MTLFXTemporalDenoisedScalerDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalDenoisedScalerDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLFXTemporalDenoisedScalerDescriptor {}
impl INSObject for MTLFXTemporalDenoisedScalerDescriptor {}
impl PNSObject for MTLFXTemporalDenoisedScalerDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLFXTemporalDenoisedScalerDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFXTemporalDenoisedScalerDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFXTemporalDenoisedScalerDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLFXTemporalDenoisedScalerDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFXTemporalDenoisedScalerDescriptor")
        }
    }
}
impl IMTLFXTemporalDenoisedScalerDescriptor for MTLFXTemporalDenoisedScalerDescriptor {}
pub trait IMTLFXTemporalDenoisedScalerDescriptor: Sized + std::ops::Deref {
    unsafe fn newTemporalDenoisedScalerWithDevice_(&self, device: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTemporalDenoisedScalerWithDevice : device)
    }
    unsafe fn newTemporalDenoisedScalerWithDevice_compiler_(
        &self,
        device: *mut u64,
        compiler: *mut u64,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTemporalDenoisedScalerWithDevice : device, compiler : compiler)
    }
    unsafe fn colorTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureFormat)
    }
    unsafe fn setColorTextureFormat_(&self, colorTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorTextureFormat : colorTextureFormat)
    }
    unsafe fn depthTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTextureFormat)
    }
    unsafe fn setDepthTextureFormat_(&self, depthTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthTextureFormat : depthTextureFormat)
    }
    unsafe fn motionTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTextureFormat)
    }
    unsafe fn setMotionTextureFormat_(&self, motionTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTextureFormat : motionTextureFormat)
    }
    unsafe fn diffuseAlbedoTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diffuseAlbedoTextureFormat)
    }
    unsafe fn setDiffuseAlbedoTextureFormat_(&self, diffuseAlbedoTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiffuseAlbedoTextureFormat : diffuseAlbedoTextureFormat)
    }
    unsafe fn specularAlbedoTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularAlbedoTextureFormat)
    }
    unsafe fn setSpecularAlbedoTextureFormat_(&self, specularAlbedoTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecularAlbedoTextureFormat : specularAlbedoTextureFormat)
    }
    unsafe fn normalTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalTextureFormat)
    }
    unsafe fn setNormalTextureFormat_(&self, normalTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalTextureFormat : normalTextureFormat)
    }
    unsafe fn roughnessTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roughnessTextureFormat)
    }
    unsafe fn setRoughnessTextureFormat_(&self, roughnessTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoughnessTextureFormat : roughnessTextureFormat)
    }
    unsafe fn specularHitDistanceTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularHitDistanceTextureFormat)
    }
    unsafe fn setSpecularHitDistanceTextureFormat_(
        &self,
        specularHitDistanceTextureFormat: MTLPixelFormat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecularHitDistanceTextureFormat : specularHitDistanceTextureFormat)
    }
    unsafe fn denoiseStrengthMaskTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, denoiseStrengthMaskTextureFormat)
    }
    unsafe fn setDenoiseStrengthMaskTextureFormat_(
        &self,
        denoiseStrengthMaskTextureFormat: MTLPixelFormat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDenoiseStrengthMaskTextureFormat : denoiseStrengthMaskTextureFormat)
    }
    unsafe fn transparencyOverlayTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transparencyOverlayTextureFormat)
    }
    unsafe fn setTransparencyOverlayTextureFormat_(
        &self,
        transparencyOverlayTextureFormat: MTLPixelFormat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransparencyOverlayTextureFormat : transparencyOverlayTextureFormat)
    }
    unsafe fn outputTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureFormat)
    }
    unsafe fn setOutputTextureFormat_(&self, outputTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputTextureFormat : outputTextureFormat)
    }
    unsafe fn inputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWidth)
    }
    unsafe fn setInputWidth_(&self, inputWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputWidth : inputWidth)
    }
    unsafe fn inputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputHeight)
    }
    unsafe fn setInputHeight_(&self, inputHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputHeight : inputHeight)
    }
    unsafe fn outputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputWidth)
    }
    unsafe fn setOutputWidth_(&self, outputWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputWidth : outputWidth)
    }
    unsafe fn outputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputHeight)
    }
    unsafe fn setOutputHeight_(&self, outputHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputHeight : outputHeight)
    }
    unsafe fn requiresSynchronousInitialization(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresSynchronousInitialization)
    }
    unsafe fn setRequiresSynchronousInitialization_(&self, requiresSynchronousInitialization: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresSynchronousInitialization : requiresSynchronousInitialization)
    }
    unsafe fn isAutoExposureEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutoExposureEnabled)
    }
    unsafe fn setAutoExposureEnabled_(&self, autoExposureEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoExposureEnabled : autoExposureEnabled)
    }
    unsafe fn isReactiveMaskTextureEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReactiveMaskTextureEnabled)
    }
    unsafe fn setReactiveMaskTextureEnabled_(&self, reactiveMaskTextureEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReactiveMaskTextureEnabled : reactiveMaskTextureEnabled)
    }
    unsafe fn reactiveMaskTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactiveMaskTextureFormat)
    }
    unsafe fn setReactiveMaskTextureFormat_(&self, reactiveMaskTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReactiveMaskTextureFormat : reactiveMaskTextureFormat)
    }
    unsafe fn isSpecularHitDistanceTextureEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSpecularHitDistanceTextureEnabled)
    }
    unsafe fn setSpecularHitDistanceTextureEnabled_(&self, specularHitDistanceTextureEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecularHitDistanceTextureEnabled : specularHitDistanceTextureEnabled)
    }
    unsafe fn isDenoiseStrengthMaskTextureEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDenoiseStrengthMaskTextureEnabled)
    }
    unsafe fn setDenoiseStrengthMaskTextureEnabled_(&self, denoiseStrengthMaskTextureEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDenoiseStrengthMaskTextureEnabled : denoiseStrengthMaskTextureEnabled)
    }
    unsafe fn isTransparencyOverlayTextureEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTransparencyOverlayTextureEnabled)
    }
    unsafe fn setTransparencyOverlayTextureEnabled_(&self, transparencyOverlayTextureEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransparencyOverlayTextureEnabled : transparencyOverlayTextureEnabled)
    }
    unsafe fn supportedInputContentMinScaleForDevice_(device: *mut u64) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalDenoisedScalerDescriptor").unwrap(), supportedInputContentMinScaleForDevice : device)
    }
    unsafe fn supportedInputContentMaxScaleForDevice_(device: *mut u64) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalDenoisedScalerDescriptor").unwrap(), supportedInputContentMaxScaleForDevice : device)
    }
    unsafe fn supportsMetal4FX_(device: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalDenoisedScalerDescriptor").unwrap(), supportsMetal4FX : device)
    }
    unsafe fn supportsDevice_(device: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXTemporalDenoisedScalerDescriptor").unwrap(), supportsDevice : device)
    }
}
pub trait PMTLFXTemporalDenoisedScalerBase: Sized + std::ops::Deref {
    unsafe fn colorTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureUsage)
    }
    unsafe fn depthTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTextureUsage)
    }
    unsafe fn motionTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTextureUsage)
    }
    unsafe fn reactiveTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactiveTextureUsage)
    }
    unsafe fn diffuseAlbedoTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diffuseAlbedoTextureUsage)
    }
    unsafe fn specularAlbedoTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularAlbedoTextureUsage)
    }
    unsafe fn normalTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalTextureUsage)
    }
    unsafe fn roughnessTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roughnessTextureUsage)
    }
    unsafe fn specularHitDistanceTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularHitDistanceTextureUsage)
    }
    unsafe fn denoiseStrengthMaskTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, denoiseStrengthMaskTextureUsage)
    }
    unsafe fn transparencyOverlayTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transparencyOverlayTextureUsage)
    }
    unsafe fn outputTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureUsage)
    }
    unsafe fn colorTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTexture)
    }
    unsafe fn setColorTexture_(&self, colorTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorTexture : colorTexture)
    }
    unsafe fn depthTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTexture)
    }
    unsafe fn setDepthTexture_(&self, depthTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthTexture : depthTexture)
    }
    unsafe fn motionTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTexture)
    }
    unsafe fn setMotionTexture_(&self, motionTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTexture : motionTexture)
    }
    unsafe fn diffuseAlbedoTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diffuseAlbedoTexture)
    }
    unsafe fn setDiffuseAlbedoTexture_(&self, diffuseAlbedoTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiffuseAlbedoTexture : diffuseAlbedoTexture)
    }
    unsafe fn specularAlbedoTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularAlbedoTexture)
    }
    unsafe fn setSpecularAlbedoTexture_(&self, specularAlbedoTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecularAlbedoTexture : specularAlbedoTexture)
    }
    unsafe fn normalTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalTexture)
    }
    unsafe fn setNormalTexture_(&self, normalTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalTexture : normalTexture)
    }
    unsafe fn roughnessTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roughnessTexture)
    }
    unsafe fn setRoughnessTexture_(&self, roughnessTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoughnessTexture : roughnessTexture)
    }
    unsafe fn specularHitDistanceTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularHitDistanceTexture)
    }
    unsafe fn setSpecularHitDistanceTexture_(&self, specularHitDistanceTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecularHitDistanceTexture : specularHitDistanceTexture)
    }
    unsafe fn denoiseStrengthMaskTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, denoiseStrengthMaskTexture)
    }
    unsafe fn setDenoiseStrengthMaskTexture_(&self, denoiseStrengthMaskTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDenoiseStrengthMaskTexture : denoiseStrengthMaskTexture)
    }
    unsafe fn transparencyOverlayTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transparencyOverlayTexture)
    }
    unsafe fn setTransparencyOverlayTexture_(&self, transparencyOverlayTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransparencyOverlayTexture : transparencyOverlayTexture)
    }
    unsafe fn outputTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTexture)
    }
    unsafe fn setOutputTexture_(&self, outputTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputTexture : outputTexture)
    }
    unsafe fn exposureTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposureTexture)
    }
    unsafe fn setExposureTexture_(&self, exposureTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposureTexture : exposureTexture)
    }
    unsafe fn preExposure(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preExposure)
    }
    unsafe fn setPreExposure_(&self, preExposure: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreExposure : preExposure)
    }
    unsafe fn reactiveMaskTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactiveMaskTexture)
    }
    unsafe fn setReactiveMaskTexture_(&self, reactiveMaskTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReactiveMaskTexture : reactiveMaskTexture)
    }
    unsafe fn jitterOffsetX(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jitterOffsetX)
    }
    unsafe fn setJitterOffsetX_(&self, jitterOffsetX: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJitterOffsetX : jitterOffsetX)
    }
    unsafe fn jitterOffsetY(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jitterOffsetY)
    }
    unsafe fn setJitterOffsetY_(&self, jitterOffsetY: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJitterOffsetY : jitterOffsetY)
    }
    unsafe fn motionVectorScaleX(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionVectorScaleX)
    }
    unsafe fn setMotionVectorScaleX_(&self, motionVectorScaleX: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionVectorScaleX : motionVectorScaleX)
    }
    unsafe fn motionVectorScaleY(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionVectorScaleY)
    }
    unsafe fn setMotionVectorScaleY_(&self, motionVectorScaleY: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionVectorScaleY : motionVectorScaleY)
    }
    unsafe fn shouldResetHistory(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldResetHistory)
    }
    unsafe fn setShouldResetHistory_(&self, shouldResetHistory: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldResetHistory : shouldResetHistory)
    }
    unsafe fn isDepthReversed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDepthReversed)
    }
    unsafe fn setDepthReversed_(&self, depthReversed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthReversed : depthReversed)
    }
    unsafe fn colorTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureFormat)
    }
    unsafe fn depthTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTextureFormat)
    }
    unsafe fn motionTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTextureFormat)
    }
    unsafe fn diffuseAlbedoTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diffuseAlbedoTextureFormat)
    }
    unsafe fn specularAlbedoTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularAlbedoTextureFormat)
    }
    unsafe fn normalTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalTextureFormat)
    }
    unsafe fn roughnessTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roughnessTextureFormat)
    }
    unsafe fn specularHitDistanceTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specularHitDistanceTextureFormat)
    }
    unsafe fn denoiseStrengthMaskTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, denoiseStrengthMaskTextureFormat)
    }
    unsafe fn transparencyOverlayTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transparencyOverlayTextureFormat)
    }
    unsafe fn reactiveMaskTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactiveMaskTextureFormat)
    }
    unsafe fn outputTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureFormat)
    }
    unsafe fn inputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWidth)
    }
    unsafe fn inputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputHeight)
    }
    unsafe fn outputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputWidth)
    }
    unsafe fn outputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputHeight)
    }
    unsafe fn inputContentMinScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentMinScale)
    }
    unsafe fn inputContentMaxScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentMaxScale)
    }
    unsafe fn fence(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fence)
    }
    unsafe fn setFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFence : fence)
    }
}
pub trait PMTLFXTemporalDenoisedScaler: Sized + std::ops::Deref {
    unsafe fn encodeToCommandBuffer_(&self, commandBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer)
    }
}
pub type MTLFXSpatialScalerColorProcessingMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFXSpatialScalerDescriptor(pub id);
impl std::ops::Deref for MTLFXSpatialScalerDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFXSpatialScalerDescriptor {}
impl MTLFXSpatialScalerDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXSpatialScalerDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLFXSpatialScalerDescriptor {}
impl INSObject for MTLFXSpatialScalerDescriptor {}
impl PNSObject for MTLFXSpatialScalerDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLFXSpatialScalerDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFXSpatialScalerDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFXSpatialScalerDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MTLFXSpatialScalerDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFXSpatialScalerDescriptor")
        }
    }
}
impl IMTLFXSpatialScalerDescriptor for MTLFXSpatialScalerDescriptor {}
pub trait IMTLFXSpatialScalerDescriptor: Sized + std::ops::Deref {
    unsafe fn newSpatialScalerWithDevice_(&self, device: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newSpatialScalerWithDevice : device)
    }
    unsafe fn newSpatialScalerWithDevice_compiler_(
        &self,
        device: *mut u64,
        compiler: *mut u64,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newSpatialScalerWithDevice : device, compiler : compiler)
    }
    unsafe fn colorTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureFormat)
    }
    unsafe fn setColorTextureFormat_(&self, colorTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorTextureFormat : colorTextureFormat)
    }
    unsafe fn outputTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureFormat)
    }
    unsafe fn setOutputTextureFormat_(&self, outputTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputTextureFormat : outputTextureFormat)
    }
    unsafe fn inputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWidth)
    }
    unsafe fn setInputWidth_(&self, inputWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputWidth : inputWidth)
    }
    unsafe fn inputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputHeight)
    }
    unsafe fn setInputHeight_(&self, inputHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputHeight : inputHeight)
    }
    unsafe fn outputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputWidth)
    }
    unsafe fn setOutputWidth_(&self, outputWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputWidth : outputWidth)
    }
    unsafe fn outputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputHeight)
    }
    unsafe fn setOutputHeight_(&self, outputHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputHeight : outputHeight)
    }
    unsafe fn colorProcessingMode(&self) -> MTLFXSpatialScalerColorProcessingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorProcessingMode)
    }
    unsafe fn setColorProcessingMode_(
        &self,
        colorProcessingMode: MTLFXSpatialScalerColorProcessingMode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorProcessingMode : colorProcessingMode)
    }
    unsafe fn supportsMetal4FX_(device: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXSpatialScalerDescriptor").unwrap(), supportsMetal4FX : device)
    }
    unsafe fn supportsDevice_(device: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXSpatialScalerDescriptor").unwrap(), supportsDevice : device)
    }
}
pub trait PMTLFXSpatialScalerBase: Sized + std::ops::Deref {
    unsafe fn colorTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureUsage)
    }
    unsafe fn outputTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureUsage)
    }
    unsafe fn inputContentWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentWidth)
    }
    unsafe fn setInputContentWidth_(&self, inputContentWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputContentWidth : inputContentWidth)
    }
    unsafe fn inputContentHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputContentHeight)
    }
    unsafe fn setInputContentHeight_(&self, inputContentHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputContentHeight : inputContentHeight)
    }
    unsafe fn colorTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTexture)
    }
    unsafe fn setColorTexture_(&self, colorTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorTexture : colorTexture)
    }
    unsafe fn outputTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTexture)
    }
    unsafe fn setOutputTexture_(&self, outputTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputTexture : outputTexture)
    }
    unsafe fn colorTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureFormat)
    }
    unsafe fn outputTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureFormat)
    }
    unsafe fn inputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWidth)
    }
    unsafe fn inputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputHeight)
    }
    unsafe fn outputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputWidth)
    }
    unsafe fn outputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputHeight)
    }
    unsafe fn colorProcessingMode(&self) -> MTLFXSpatialScalerColorProcessingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorProcessingMode)
    }
    unsafe fn fence(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fence)
    }
    unsafe fn setFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFence : fence)
    }
}
pub trait PMTLFXSpatialScaler: Sized + std::ops::Deref {
    unsafe fn encodeToCommandBuffer_(&self, commandBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MTLFXFrameInterpolatorDescriptor(pub id);
impl std::ops::Deref for MTLFXFrameInterpolatorDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MTLFXFrameInterpolatorDescriptor {}
impl MTLFXFrameInterpolatorDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXFrameInterpolatorDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MTLFXFrameInterpolatorDescriptor {}
impl INSObject for MTLFXFrameInterpolatorDescriptor {}
impl PNSObject for MTLFXFrameInterpolatorDescriptor {}
impl std::convert::TryFrom<NSObject> for MTLFXFrameInterpolatorDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MTLFXFrameInterpolatorDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MTLFXFrameInterpolatorDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MTLFXFrameInterpolatorDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MTLFXFrameInterpolatorDescriptor")
        }
    }
}
impl IMTLFXFrameInterpolatorDescriptor for MTLFXFrameInterpolatorDescriptor {}
pub trait IMTLFXFrameInterpolatorDescriptor: Sized + std::ops::Deref {
    unsafe fn newFrameInterpolatorWithDevice_(&self, device: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newFrameInterpolatorWithDevice : device)
    }
    unsafe fn newFrameInterpolatorWithDevice_compiler_(
        &self,
        device: *mut u64,
        compiler: *mut u64,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newFrameInterpolatorWithDevice : device, compiler : compiler)
    }
    unsafe fn colorTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureFormat)
    }
    unsafe fn setColorTextureFormat_(&self, colorTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorTextureFormat : colorTextureFormat)
    }
    unsafe fn outputTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureFormat)
    }
    unsafe fn setOutputTextureFormat_(&self, outputTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputTextureFormat : outputTextureFormat)
    }
    unsafe fn depthTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTextureFormat)
    }
    unsafe fn setDepthTextureFormat_(&self, depthTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthTextureFormat : depthTextureFormat)
    }
    unsafe fn motionTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTextureFormat)
    }
    unsafe fn setMotionTextureFormat_(&self, motionTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTextureFormat : motionTextureFormat)
    }
    unsafe fn uiTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uiTextureFormat)
    }
    unsafe fn setUITextureFormat_(&self, uiTextureFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUITextureFormat : uiTextureFormat)
    }
    unsafe fn scaler(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaler)
    }
    unsafe fn setScaler_(&self, scaler: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaler : scaler)
    }
    unsafe fn inputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWidth)
    }
    unsafe fn setInputWidth_(&self, inputWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputWidth : inputWidth)
    }
    unsafe fn inputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputHeight)
    }
    unsafe fn setInputHeight_(&self, inputHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputHeight : inputHeight)
    }
    unsafe fn outputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputWidth)
    }
    unsafe fn setOutputWidth_(&self, outputWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputWidth : outputWidth)
    }
    unsafe fn outputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputHeight)
    }
    unsafe fn setOutputHeight_(&self, outputHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputHeight : outputHeight)
    }
    unsafe fn supportsMetal4FX_(device: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXFrameInterpolatorDescriptor").unwrap(), supportsMetal4FX : device)
    }
    unsafe fn supportsDevice_(device: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MTLFXFrameInterpolatorDescriptor").unwrap(), supportsDevice : device)
    }
}
pub trait PMTLFXFrameInterpolatorBase: Sized + std::ops::Deref {
    unsafe fn colorTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureUsage)
    }
    unsafe fn outputTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureUsage)
    }
    unsafe fn depthTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTextureUsage)
    }
    unsafe fn motionTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTextureUsage)
    }
    unsafe fn uiTextureUsage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uiTextureUsage)
    }
    unsafe fn colorTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTextureFormat)
    }
    unsafe fn depthTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTextureFormat)
    }
    unsafe fn motionTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTextureFormat)
    }
    unsafe fn outputTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTextureFormat)
    }
    unsafe fn inputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWidth)
    }
    unsafe fn inputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputHeight)
    }
    unsafe fn outputWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputWidth)
    }
    unsafe fn outputHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputHeight)
    }
    unsafe fn uiTextureFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uiTextureFormat)
    }
    unsafe fn colorTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorTexture)
    }
    unsafe fn setColorTexture_(&self, colorTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorTexture : colorTexture)
    }
    unsafe fn prevColorTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prevColorTexture)
    }
    unsafe fn setPrevColorTexture_(&self, prevColorTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrevColorTexture : prevColorTexture)
    }
    unsafe fn depthTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthTexture)
    }
    unsafe fn setDepthTexture_(&self, depthTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthTexture : depthTexture)
    }
    unsafe fn motionTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionTexture)
    }
    unsafe fn setMotionTexture_(&self, motionTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionTexture : motionTexture)
    }
    unsafe fn motionVectorScaleX(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionVectorScaleX)
    }
    unsafe fn setMotionVectorScaleX_(&self, motionVectorScaleX: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionVectorScaleX : motionVectorScaleX)
    }
    unsafe fn motionVectorScaleY(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, motionVectorScaleY)
    }
    unsafe fn setMotionVectorScaleY_(&self, motionVectorScaleY: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMotionVectorScaleY : motionVectorScaleY)
    }
    unsafe fn deltaTime(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deltaTime)
    }
    unsafe fn setDeltaTime_(&self, deltaTime: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeltaTime : deltaTime)
    }
    unsafe fn nearPlane(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nearPlane)
    }
    unsafe fn setNearPlane_(&self, nearPlane: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNearPlane : nearPlane)
    }
    unsafe fn farPlane(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, farPlane)
    }
    unsafe fn setFarPlane_(&self, farPlane: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFarPlane : farPlane)
    }
    unsafe fn fieldOfView(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldOfView)
    }
    unsafe fn setFieldOfView_(&self, fieldOfView: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldOfView : fieldOfView)
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
    unsafe fn uiTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uiTexture)
    }
    unsafe fn setUITexture_(&self, uiTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUITexture : uiTexture)
    }
    unsafe fn jitterOffsetX(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jitterOffsetX)
    }
    unsafe fn setJitterOffsetX_(&self, jitterOffsetX: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJitterOffsetX : jitterOffsetX)
    }
    unsafe fn jitterOffsetY(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jitterOffsetY)
    }
    unsafe fn setJitterOffsetY_(&self, jitterOffsetY: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJitterOffsetY : jitterOffsetY)
    }
    unsafe fn isUITextureComposited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUITextureComposited)
    }
    unsafe fn setIsUITextureComposited_(&self, uiTextureComposited: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsUITextureComposited : uiTextureComposited)
    }
    unsafe fn shouldResetHistory(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldResetHistory)
    }
    unsafe fn setShouldResetHistory_(&self, shouldResetHistory: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldResetHistory : shouldResetHistory)
    }
    unsafe fn outputTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTexture)
    }
    unsafe fn setOutputTexture_(&self, outputTexture: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputTexture : outputTexture)
    }
    unsafe fn fence(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fence)
    }
    unsafe fn setFence_(&self, fence: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFence : fence)
    }
    unsafe fn isDepthReversed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDepthReversed)
    }
    unsafe fn setDepthReversed_(&self, depthReversed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepthReversed : depthReversed)
    }
}
pub trait PMTLFXFrameInterpolator: Sized + std::ops::Deref {
    unsafe fn encodeToCommandBuffer_(&self, commandBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer)
    }
}
pub trait PMTL4FXSpatialScaler: Sized + std::ops::Deref {
    unsafe fn encodeToCommandBuffer_(&self, commandBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer)
    }
}
pub trait PMTL4FXTemporalScaler: Sized + std::ops::Deref {
    unsafe fn encodeToCommandBuffer_(&self, commandBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer)
    }
}
pub trait PMTL4FXTemporalDenoisedScaler: Sized + std::ops::Deref {
    unsafe fn encodeToCommandBuffer_(&self, commandBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer)
    }
}
pub trait PMTL4FXFrameInterpolator: Sized + std::ops::Deref {
    unsafe fn encodeToCommandBuffer_(&self, commandBuffer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer)
    }
}

unsafe impl objc2::encode::RefEncode for MTLFXTemporalScalerDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFXTemporalScalerDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFXTemporalDenoisedScalerDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFXTemporalDenoisedScalerDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFXSpatialScalerDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFXSpatialScalerDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MTLFXFrameInterpolatorDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTLFXFrameInterpolatorDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
