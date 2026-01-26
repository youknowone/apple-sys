#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOSurface::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub trait PMTLBuffer: Sized + std::ops::Deref {
    unsafe fn contents(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn didModifyRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didModifyRange : range)
    }
    unsafe fn newTextureWithDescriptor_offset_bytesPerRow_(
        &self,
        descriptor: MTLTextureDescriptor,
        offset: NSUInteger,
        bytesPerRow: NSUInteger,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithDescriptor : descriptor, offset : offset, bytesPerRow : bytesPerRow)
    }
    unsafe fn newTensorWithDescriptor_offset_error_(
        &self,
        descriptor: MTLTensorDescriptor,
        offset: NSUInteger,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTensorWithDescriptor : descriptor, offset : offset, error : error)
    }
    unsafe fn addDebugMarker_range_(&self, marker: NSString, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addDebugMarker : marker, range : range)
    }
    unsafe fn removeAllDebugMarkers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllDebugMarkers)
    }
    unsafe fn newRemoteBufferViewForDevice_(&self, device: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRemoteBufferViewForDevice : device)
    }
    unsafe fn length(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn remoteStorageBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteStorageBuffer)
    }
    unsafe fn gpuAddress(&self) -> MTLGPUAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuAddress)
    }
    unsafe fn sparseBufferTier(&self) -> MTLBufferSparseTier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sparseBufferTier)
    }
}
pub trait PMTLTexture: Sized + std::ops::Deref {
    unsafe fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice_(
        &self,
        pixelBytes: *mut ::std::os::raw::c_void,
        bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger,
        region: MTLRegion,
        level: NSUInteger,
        slice: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getBytes : pixelBytes, bytesPerRow : bytesPerRow, bytesPerImage : bytesPerImage, fromRegion : region, mipmapLevel : level, slice : slice)
    }
    unsafe fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage_(
        &self,
        region: MTLRegion,
        level: NSUInteger,
        slice: NSUInteger,
        pixelBytes: *const ::std::os::raw::c_void,
        bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceRegion : region, mipmapLevel : level, slice : slice, withBytes : pixelBytes, bytesPerRow : bytesPerRow, bytesPerImage : bytesPerImage)
    }
    unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel_(
        &self,
        pixelBytes: *mut ::std::os::raw::c_void,
        bytesPerRow: NSUInteger,
        region: MTLRegion,
        level: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getBytes : pixelBytes, bytesPerRow : bytesPerRow, fromRegion : region, mipmapLevel : level)
    }
    unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow_(
        &self,
        region: MTLRegion,
        level: NSUInteger,
        pixelBytes: *const ::std::os::raw::c_void,
        bytesPerRow: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceRegion : region, mipmapLevel : level, withBytes : pixelBytes, bytesPerRow : bytesPerRow)
    }
    unsafe fn newTextureViewWithPixelFormat_(&self, pixelFormat: MTLPixelFormat) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureViewWithPixelFormat : pixelFormat)
    }
    unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices_(
        &self,
        pixelFormat: MTLPixelFormat,
        textureType: MTLTextureType,
        levelRange: NSRange,
        sliceRange: NSRange,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureViewWithPixelFormat : pixelFormat, textureType : textureType, levels : levelRange, slices : sliceRange)
    }
    unsafe fn newSharedTextureHandle(&self) -> MTLSharedTextureHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newSharedTextureHandle)
    }
    unsafe fn newTextureViewWithDescriptor_(&self, descriptor: MTLTextureViewDescriptor) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureViewWithDescriptor : descriptor)
    }
    unsafe fn newRemoteTextureViewForDevice_(&self, device: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRemoteTextureViewForDevice : device)
    }
    unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices_swizzle_(
        &self,
        pixelFormat: MTLPixelFormat,
        textureType: MTLTextureType,
        levelRange: NSRange,
        sliceRange: NSRange,
        swizzle: MTLTextureSwizzleChannels,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureViewWithPixelFormat : pixelFormat, textureType : textureType, levels : levelRange, slices : sliceRange, swizzle : swizzle)
    }
    unsafe fn rootResource(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootResource)
    }
    unsafe fn parentTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentTexture)
    }
    unsafe fn parentRelativeLevel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentRelativeLevel)
    }
    unsafe fn parentRelativeSlice(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentRelativeSlice)
    }
    unsafe fn buffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buffer)
    }
    unsafe fn bufferOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferOffset)
    }
    unsafe fn bufferBytesPerRow(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferBytesPerRow)
    }
    unsafe fn iosurface(&self) -> IOSurfaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iosurface)
    }
    unsafe fn iosurfacePlane(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iosurfacePlane)
    }
    unsafe fn textureType(&self) -> MTLTextureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureType)
    }
    unsafe fn pixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn width(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn depth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depth)
    }
    unsafe fn mipmapLevelCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mipmapLevelCount)
    }
    unsafe fn sampleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
    unsafe fn arrayLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrayLength)
    }
    unsafe fn usage(&self) -> MTLTextureUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usage)
    }
    unsafe fn isShareable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isShareable)
    }
    unsafe fn isFramebufferOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFramebufferOnly)
    }
    unsafe fn firstMipmapInTail(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstMipmapInTail)
    }
    unsafe fn tailSizeInBytes(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tailSizeInBytes)
    }
    unsafe fn isSparse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSparse)
    }
    unsafe fn allowGPUOptimizedContents(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowGPUOptimizedContents)
    }
    unsafe fn compressionType(&self) -> MTLTextureCompressionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compressionType)
    }
    unsafe fn gpuResourceID(&self) -> MTLResourceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuResourceID)
    }
    unsafe fn remoteStorageTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteStorageTexture)
    }
    unsafe fn swizzle(&self) -> MTLTextureSwizzleChannels
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, swizzle)
    }
    unsafe fn sparseTextureTier(&self) -> MTLTextureSparseTier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sparseTextureTier)
    }
}
pub type MTLNewLibraryCompletionHandler = *mut ::std::os::raw::c_void;
pub type MTLNewRenderPipelineStateCompletionHandler = *mut ::std::os::raw::c_void;
pub type MTLNewRenderPipelineStateWithReflectionCompletionHandler = *mut ::std::os::raw::c_void;
pub type MTLNewComputePipelineStateCompletionHandler = *mut ::std::os::raw::c_void;
pub type MTLNewComputePipelineStateWithReflectionCompletionHandler = *mut ::std::os::raw::c_void;
pub type MTLDeviceLocation = NSUInteger;
pub trait PMTLDevice: Sized + std::ops::Deref {
    unsafe fn newLogStateWithDescriptor_error_(
        &self,
        descriptor: MTLLogStateDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLogStateWithDescriptor : descriptor, error : error)
    }
    unsafe fn newCommandQueue(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newCommandQueue)
    }
    unsafe fn newCommandQueueWithMaxCommandBufferCount_(
        &self,
        maxCommandBufferCount: NSUInteger,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCommandQueueWithMaxCommandBufferCount : maxCommandBufferCount)
    }
    unsafe fn newCommandQueueWithDescriptor_(
        &self,
        descriptor: MTLCommandQueueDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCommandQueueWithDescriptor : descriptor)
    }
    unsafe fn heapTextureSizeAndAlignWithDescriptor_(
        &self,
        desc: MTLTextureDescriptor,
    ) -> MTLSizeAndAlign
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, heapTextureSizeAndAlignWithDescriptor : desc)
    }
    unsafe fn heapBufferSizeAndAlignWithLength_options_(
        &self,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> MTLSizeAndAlign
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, heapBufferSizeAndAlignWithLength : length, options : options)
    }
    unsafe fn newHeapWithDescriptor_(&self, descriptor: MTLHeapDescriptor) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newHeapWithDescriptor : descriptor)
    }
    unsafe fn newBufferWithLength_options_(
        &self,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBufferWithLength : length, options : options)
    }
    unsafe fn newBufferWithBytes_length_options_(
        &self,
        pointer: *const ::std::os::raw::c_void,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBufferWithBytes : pointer, length : length, options : options)
    }
    unsafe fn newBufferWithBytesNoCopy_length_options_deallocator_(
        &self,
        pointer: *mut ::std::os::raw::c_void,
        length: NSUInteger,
        options: MTLResourceOptions,
        deallocator: *mut ::std::os::raw::c_void,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBufferWithBytesNoCopy : pointer, length : length, options : options, deallocator : deallocator)
    }
    unsafe fn newDepthStencilStateWithDescriptor_(
        &self,
        descriptor: MTLDepthStencilDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newDepthStencilStateWithDescriptor : descriptor)
    }
    unsafe fn newTextureWithDescriptor_(&self, descriptor: MTLTextureDescriptor) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithDescriptor : descriptor)
    }
    unsafe fn newTextureWithDescriptor_iosurface_plane_(
        &self,
        descriptor: MTLTextureDescriptor,
        iosurface: IOSurfaceRef,
        plane: NSUInteger,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureWithDescriptor : descriptor, iosurface : iosurface, plane : plane)
    }
    unsafe fn newSharedTextureWithDescriptor_(&self, descriptor: MTLTextureDescriptor) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newSharedTextureWithDescriptor : descriptor)
    }
    unsafe fn newSharedTextureWithHandle_(&self, sharedHandle: MTLSharedTextureHandle) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newSharedTextureWithHandle : sharedHandle)
    }
    unsafe fn newSamplerStateWithDescriptor_(&self, descriptor: MTLSamplerDescriptor) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newSamplerStateWithDescriptor : descriptor)
    }
    unsafe fn newDefaultLibrary(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newDefaultLibrary)
    }
    unsafe fn newDefaultLibraryWithBundle_error_(
        &self,
        bundle: NSBundle,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newDefaultLibraryWithBundle : bundle, error : error)
    }
    unsafe fn newLibraryWithFile_error_(&self, filepath: NSString, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLibraryWithFile : filepath, error : error)
    }
    unsafe fn newLibraryWithURL_error_(&self, url: NSURL, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLibraryWithURL : url, error : error)
    }
    unsafe fn newLibraryWithData_error_(&self, data: NSObject, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLibraryWithData : data, error : error)
    }
    unsafe fn newLibraryWithSource_options_error_(
        &self,
        source: NSString,
        options: MTLCompileOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLibraryWithSource : source, options : options, error : error)
    }
    unsafe fn newLibraryWithSource_options_completionHandler_(
        &self,
        source: NSString,
        options: MTLCompileOptions,
        completionHandler: MTLNewLibraryCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLibraryWithSource : source, options : options, completionHandler : completionHandler)
    }
    unsafe fn newLibraryWithStitchedDescriptor_error_(
        &self,
        descriptor: MTLStitchedLibraryDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLibraryWithStitchedDescriptor : descriptor, error : error)
    }
    unsafe fn newLibraryWithStitchedDescriptor_completionHandler_(
        &self,
        descriptor: MTLStitchedLibraryDescriptor,
        completionHandler: MTLNewLibraryCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newLibraryWithStitchedDescriptor : descriptor, completionHandler : completionHandler)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_error_(
        &self,
        descriptor: MTLRenderPipelineDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, error : error)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_options_reflection_error_(
        &self,
        descriptor: MTLRenderPipelineDescriptor,
        options: MTLPipelineOption,
        reflection: *mut MTLRenderPipelineReflection,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, options : options, reflection : reflection, error : error)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_completionHandler_(
        &self,
        descriptor: MTLRenderPipelineDescriptor,
        completionHandler: MTLNewRenderPipelineStateCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, completionHandler : completionHandler)
    }
    unsafe fn newRenderPipelineStateWithDescriptor_options_completionHandler_(
        &self,
        descriptor: MTLRenderPipelineDescriptor,
        options: MTLPipelineOption,
        completionHandler: MTLNewRenderPipelineStateWithReflectionCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithDescriptor : descriptor, options : options, completionHandler : completionHandler)
    }
    unsafe fn newComputePipelineStateWithFunction_error_(
        &self,
        computeFunction: *mut u64,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithFunction : computeFunction, error : error)
    }
    unsafe fn newComputePipelineStateWithFunction_options_reflection_error_(
        &self,
        computeFunction: *mut u64,
        options: MTLPipelineOption,
        reflection: *mut MTLComputePipelineReflection,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithFunction : computeFunction, options : options, reflection : reflection, error : error)
    }
    unsafe fn newComputePipelineStateWithFunction_completionHandler_(
        &self,
        computeFunction: *mut u64,
        completionHandler: MTLNewComputePipelineStateCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithFunction : computeFunction, completionHandler : completionHandler)
    }
    unsafe fn newComputePipelineStateWithFunction_options_completionHandler_(
        &self,
        computeFunction: *mut u64,
        options: MTLPipelineOption,
        completionHandler: MTLNewComputePipelineStateWithReflectionCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithFunction : computeFunction, options : options, completionHandler : completionHandler)
    }
    unsafe fn newComputePipelineStateWithDescriptor_options_reflection_error_(
        &self,
        descriptor: MTLComputePipelineDescriptor,
        options: MTLPipelineOption,
        reflection: *mut MTLComputePipelineReflection,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithDescriptor : descriptor, options : options, reflection : reflection, error : error)
    }
    unsafe fn newComputePipelineStateWithDescriptor_options_completionHandler_(
        &self,
        descriptor: MTLComputePipelineDescriptor,
        options: MTLPipelineOption,
        completionHandler: MTLNewComputePipelineStateWithReflectionCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newComputePipelineStateWithDescriptor : descriptor, options : options, completionHandler : completionHandler)
    }
    unsafe fn newFence(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newFence)
    }
    unsafe fn supportsFeatureSet_(&self, featureSet: MTLFeatureSet) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsFeatureSet : featureSet)
    }
    unsafe fn supportsFamily_(&self, gpuFamily: MTLGPUFamily) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsFamily : gpuFamily)
    }
    unsafe fn supportsTextureSampleCount_(&self, sampleCount: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsTextureSampleCount : sampleCount)
    }
    unsafe fn minimumLinearTextureAlignmentForPixelFormat_(
        &self,
        format: MTLPixelFormat,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, minimumLinearTextureAlignmentForPixelFormat : format)
    }
    unsafe fn minimumTextureBufferAlignmentForPixelFormat_(
        &self,
        format: MTLPixelFormat,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, minimumTextureBufferAlignmentForPixelFormat : format)
    }
    unsafe fn newRenderPipelineStateWithTileDescriptor_options_reflection_error_(
        &self,
        descriptor: MTLTileRenderPipelineDescriptor,
        options: MTLPipelineOption,
        reflection: *mut MTLRenderPipelineReflection,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithTileDescriptor : descriptor, options : options, reflection : reflection, error : error)
    }
    unsafe fn newRenderPipelineStateWithTileDescriptor_options_completionHandler_(
        &self,
        descriptor: MTLTileRenderPipelineDescriptor,
        options: MTLPipelineOption,
        completionHandler: MTLNewRenderPipelineStateWithReflectionCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithTileDescriptor : descriptor, options : options, completionHandler : completionHandler)
    }
    unsafe fn newRenderPipelineStateWithMeshDescriptor_options_reflection_error_(
        &self,
        descriptor: MTLMeshRenderPipelineDescriptor,
        options: MTLPipelineOption,
        reflection: *mut MTLRenderPipelineReflection,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithMeshDescriptor : descriptor, options : options, reflection : reflection, error : error)
    }
    unsafe fn newRenderPipelineStateWithMeshDescriptor_options_completionHandler_(
        &self,
        descriptor: MTLMeshRenderPipelineDescriptor,
        options: MTLPipelineOption,
        completionHandler: MTLNewRenderPipelineStateWithReflectionCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRenderPipelineStateWithMeshDescriptor : descriptor, options : options, completionHandler : completionHandler)
    }
    unsafe fn getDefaultSamplePositions_count_(
        &self,
        positions: *mut MTLSamplePosition,
        count: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDefaultSamplePositions : positions, count : count)
    }
    unsafe fn newArgumentEncoderWithArguments_(&self, arguments: NSArray) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newArgumentEncoderWithArguments : arguments)
    }
    unsafe fn supportsRasterizationRateMapWithLayerCount_(&self, layerCount: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsRasterizationRateMapWithLayerCount : layerCount)
    }
    unsafe fn newRasterizationRateMapWithDescriptor_(
        &self,
        descriptor: MTLRasterizationRateMapDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newRasterizationRateMapWithDescriptor : descriptor)
    }
    unsafe fn newIndirectCommandBufferWithDescriptor_maxCommandCount_options_(
        &self,
        descriptor: MTLIndirectCommandBufferDescriptor,
        maxCount: NSUInteger,
        options: MTLResourceOptions,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIndirectCommandBufferWithDescriptor : descriptor, maxCommandCount : maxCount, options : options)
    }
    unsafe fn newEvent(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newEvent)
    }
    unsafe fn newSharedEvent(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newSharedEvent)
    }
    unsafe fn newSharedEventWithHandle_(&self, sharedEventHandle: MTLSharedEventHandle) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newSharedEventWithHandle : sharedEventHandle)
    }
    unsafe fn newIOHandleWithURL_error_(&self, url: NSURL, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIOHandleWithURL : url, error : error)
    }
    unsafe fn newIOCommandQueueWithDescriptor_error_(
        &self,
        descriptor: MTLIOCommandQueueDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIOCommandQueueWithDescriptor : descriptor, error : error)
    }
    unsafe fn newIOHandleWithURL_compressionMethod_error_(
        &self,
        url: NSURL,
        compressionMethod: MTLIOCompressionMethod,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIOHandleWithURL : url, compressionMethod : compressionMethod, error : error)
    }
    unsafe fn newIOFileHandleWithURL_error_(&self, url: NSURL, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIOFileHandleWithURL : url, error : error)
    }
    unsafe fn newIOFileHandleWithURL_compressionMethod_error_(
        &self,
        url: NSURL,
        compressionMethod: MTLIOCompressionMethod,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newIOFileHandleWithURL : url, compressionMethod : compressionMethod, error : error)
    }
    unsafe fn sparseTileSizeWithTextureType_pixelFormat_sampleCount_(
        &self,
        textureType: MTLTextureType,
        pixelFormat: MTLPixelFormat,
        sampleCount: NSUInteger,
    ) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sparseTileSizeWithTextureType : textureType, pixelFormat : pixelFormat, sampleCount : sampleCount)
    }
    unsafe fn convertSparsePixelRegions_toTileRegions_withTileSize_alignmentMode_numRegions_(
        &self,
        pixelRegions: *const MTLRegion,
        tileRegions: *mut MTLRegion,
        tileSize: MTLSize,
        mode: MTLSparseTextureRegionAlignmentMode,
        numRegions: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertSparsePixelRegions : pixelRegions, toTileRegions : tileRegions, withTileSize : tileSize, alignmentMode : mode, numRegions : numRegions)
    }
    unsafe fn convertSparseTileRegions_toPixelRegions_withTileSize_numRegions_(
        &self,
        tileRegions: *const MTLRegion,
        pixelRegions: *mut MTLRegion,
        tileSize: MTLSize,
        numRegions: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertSparseTileRegions : tileRegions, toPixelRegions : pixelRegions, withTileSize : tileSize, numRegions : numRegions)
    }
    unsafe fn sparseTileSizeInBytesForSparsePageSize_(
        &self,
        sparsePageSize: MTLSparsePageSize,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sparseTileSizeInBytesForSparsePageSize : sparsePageSize)
    }
    unsafe fn sparseTileSizeWithTextureType_pixelFormat_sampleCount_sparsePageSize_(
        &self,
        textureType: MTLTextureType,
        pixelFormat: MTLPixelFormat,
        sampleCount: NSUInteger,
        sparsePageSize: MTLSparsePageSize,
    ) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sparseTileSizeWithTextureType : textureType, pixelFormat : pixelFormat, sampleCount : sampleCount, sparsePageSize : sparsePageSize)
    }
    unsafe fn newCounterSampleBufferWithDescriptor_error_(
        &self,
        descriptor: MTLCounterSampleBufferDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCounterSampleBufferWithDescriptor : descriptor, error : error)
    }
    unsafe fn sampleTimestamps_gpuTimestamp_(
        &self,
        cpuTimestamp: *mut MTLTimestamp,
        gpuTimestamp: *mut MTLTimestamp,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleTimestamps : cpuTimestamp, gpuTimestamp : gpuTimestamp)
    }
    unsafe fn newArgumentEncoderWithBufferBinding_(&self, bufferBinding: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newArgumentEncoderWithBufferBinding : bufferBinding)
    }
    unsafe fn supportsCounterSampling_(&self, samplingPoint: MTLCounterSamplingPoint) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsCounterSampling : samplingPoint)
    }
    unsafe fn supportsVertexAmplificationCount_(&self, count: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsVertexAmplificationCount : count)
    }
    unsafe fn newDynamicLibrary_error_(&self, library: *mut u64, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newDynamicLibrary : library, error : error)
    }
    unsafe fn newDynamicLibraryWithURL_error_(&self, url: NSURL, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newDynamicLibraryWithURL : url, error : error)
    }
    unsafe fn newBinaryArchiveWithDescriptor_error_(
        &self,
        descriptor: MTLBinaryArchiveDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBinaryArchiveWithDescriptor : descriptor, error : error)
    }
    unsafe fn accelerationStructureSizesWithDescriptor_(
        &self,
        descriptor: MTLAccelerationStructureDescriptor,
    ) -> MTLAccelerationStructureSizes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accelerationStructureSizesWithDescriptor : descriptor)
    }
    unsafe fn newAccelerationStructureWithSize_(&self, size: NSUInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newAccelerationStructureWithSize : size)
    }
    unsafe fn newAccelerationStructureWithDescriptor_(
        &self,
        descriptor: MTLAccelerationStructureDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newAccelerationStructureWithDescriptor : descriptor)
    }
    unsafe fn heapAccelerationStructureSizeAndAlignWithSize_(
        &self,
        size: NSUInteger,
    ) -> MTLSizeAndAlign
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, heapAccelerationStructureSizeAndAlignWithSize : size)
    }
    unsafe fn heapAccelerationStructureSizeAndAlignWithDescriptor_(
        &self,
        descriptor: MTLAccelerationStructureDescriptor,
    ) -> MTLSizeAndAlign
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, heapAccelerationStructureSizeAndAlignWithDescriptor : descriptor)
    }
    unsafe fn newResidencySetWithDescriptor_error_(
        &self,
        desc: MTLResidencySetDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newResidencySetWithDescriptor : desc, error : error)
    }
    unsafe fn tensorSizeAndAlignWithDescriptor_(
        &self,
        descriptor: MTLTensorDescriptor,
    ) -> MTLSizeAndAlign
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tensorSizeAndAlignWithDescriptor : descriptor)
    }
    unsafe fn newTensorWithDescriptor_error_(
        &self,
        descriptor: MTLTensorDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTensorWithDescriptor : descriptor, error : error)
    }
    unsafe fn functionHandleWithFunction_(&self, function: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, functionHandleWithFunction : function)
    }
    unsafe fn newCommandAllocator(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newCommandAllocator)
    }
    unsafe fn newCommandAllocatorWithDescriptor_error_(
        &self,
        descriptor: MTL4CommandAllocatorDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCommandAllocatorWithDescriptor : descriptor, error : error)
    }
    unsafe fn newMTL4CommandQueue(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newMTL4CommandQueue)
    }
    unsafe fn newMTL4CommandQueueWithDescriptor_error_(
        &self,
        descriptor: MTL4CommandQueueDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newMTL4CommandQueueWithDescriptor : descriptor, error : error)
    }
    unsafe fn newCommandBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newCommandBuffer)
    }
    unsafe fn newArgumentTableWithDescriptor_error_(
        &self,
        descriptor: MTL4ArgumentTableDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newArgumentTableWithDescriptor : descriptor, error : error)
    }
    unsafe fn newTextureViewPoolWithDescriptor_error_(
        &self,
        descriptor: MTLResourceViewPoolDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newTextureViewPoolWithDescriptor : descriptor, error : error)
    }
    unsafe fn newCompilerWithDescriptor_error_(
        &self,
        descriptor: MTL4CompilerDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCompilerWithDescriptor : descriptor, error : error)
    }
    unsafe fn newArchiveWithURL_error_(&self, url: NSURL, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newArchiveWithURL : url, error : error)
    }
    unsafe fn newPipelineDataSetSerializerWithDescriptor_(
        &self,
        descriptor: MTL4PipelineDataSetSerializerDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newPipelineDataSetSerializerWithDescriptor : descriptor)
    }
    unsafe fn newBufferWithLength_options_placementSparsePageSize_(
        &self,
        length: NSUInteger,
        options: MTLResourceOptions,
        placementSparsePageSize: MTLSparsePageSize,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newBufferWithLength : length, options : options, placementSparsePageSize : placementSparsePageSize)
    }
    unsafe fn newCounterHeapWithDescriptor_error_(
        &self,
        descriptor: MTL4CounterHeapDescriptor,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newCounterHeapWithDescriptor : descriptor, error : error)
    }
    unsafe fn sizeOfCounterHeapEntry_(&self, type_: MTL4CounterHeapType) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sizeOfCounterHeapEntry : type_)
    }
    unsafe fn queryTimestampFrequency(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queryTimestampFrequency)
    }
    unsafe fn functionHandleWithBinaryFunction_(&self, function: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, functionHandleWithBinaryFunction : function)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn registryID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registryID)
    }
    unsafe fn architecture(&self) -> MTLArchitecture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, architecture)
    }
    unsafe fn maxThreadsPerThreadgroup(&self) -> MTLSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxThreadsPerThreadgroup)
    }
    unsafe fn isLowPower(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLowPower)
    }
    unsafe fn isHeadless(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHeadless)
    }
    unsafe fn isRemovable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRemovable)
    }
    unsafe fn hasUnifiedMemory(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasUnifiedMemory)
    }
    unsafe fn recommendedMaxWorkingSetSize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recommendedMaxWorkingSetSize)
    }
    unsafe fn location(&self) -> MTLDeviceLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn locationNumber(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationNumber)
    }
    unsafe fn maxTransferRate(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxTransferRate)
    }
    unsafe fn isDepth24Stencil8PixelFormatSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDepth24Stencil8PixelFormatSupported)
    }
    unsafe fn readWriteTextureSupport(&self) -> MTLReadWriteTextureTier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readWriteTextureSupport)
    }
    unsafe fn argumentBuffersSupport(&self) -> MTLArgumentBuffersTier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentBuffersSupport)
    }
    unsafe fn areRasterOrderGroupsSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areRasterOrderGroupsSupported)
    }
    unsafe fn supports32BitFloatFiltering(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supports32BitFloatFiltering)
    }
    unsafe fn supports32BitMSAA(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supports32BitMSAA)
    }
    unsafe fn supportsQueryTextureLOD(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsQueryTextureLOD)
    }
    unsafe fn supportsBCTextureCompression(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsBCTextureCompression)
    }
    unsafe fn supportsPullModelInterpolation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsPullModelInterpolation)
    }
    unsafe fn areBarycentricCoordsSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areBarycentricCoordsSupported)
    }
    unsafe fn supportsShaderBarycentricCoordinates(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsShaderBarycentricCoordinates)
    }
    unsafe fn currentAllocatedSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentAllocatedSize)
    }
    unsafe fn maxThreadgroupMemoryLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxThreadgroupMemoryLength)
    }
    unsafe fn maxArgumentBufferSamplerCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxArgumentBufferSamplerCount)
    }
    unsafe fn areProgrammableSamplePositionsSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areProgrammableSamplePositionsSupported)
    }
    unsafe fn peerGroupID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peerGroupID)
    }
    unsafe fn peerIndex(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peerIndex)
    }
    unsafe fn peerCount(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peerCount)
    }
    unsafe fn sparseTileSizeInBytes(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sparseTileSizeInBytes)
    }
    unsafe fn maxBufferLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxBufferLength)
    }
    unsafe fn counterSets(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, counterSets)
    }
    unsafe fn supportsDynamicLibraries(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsDynamicLibraries)
    }
    unsafe fn supportsRenderDynamicLibraries(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsRenderDynamicLibraries)
    }
    unsafe fn supportsRaytracing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsRaytracing)
    }
    unsafe fn supportsFunctionPointers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsFunctionPointers)
    }
    unsafe fn supportsFunctionPointersFromRender(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsFunctionPointersFromRender)
    }
    unsafe fn supportsRaytracingFromRender(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsRaytracingFromRender)
    }
    unsafe fn supportsPrimitiveMotionBlur(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsPrimitiveMotionBlur)
    }
    unsafe fn shouldMaximizeConcurrentCompilation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldMaximizeConcurrentCompilation)
    }
    unsafe fn setShouldMaximizeConcurrentCompilation_(
        &self,
        shouldMaximizeConcurrentCompilation: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldMaximizeConcurrentCompilation : shouldMaximizeConcurrentCompilation)
    }
    unsafe fn maximumConcurrentCompilationTaskCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumConcurrentCompilationTaskCount)
    }
}
pub type NSDeviceCertification = NSInteger;
pub type NSProcessPerformanceProfile = NSInteger;
pub trait NSProcessInfo_NSDeviceCertification: Sized + std::ops::Deref {
    unsafe fn isDeviceCertifiedFor_(&self, performanceTier: NSDeviceCertification) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isDeviceCertifiedFor : performanceTier)
    }
    unsafe fn hasPerformanceProfile_(&self, performanceProfile: NSProcessPerformanceProfile) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasPerformanceProfile : performanceProfile)
    }
}
pub type MTL4CommitFeedbackHandler = *mut ::std::os::raw::c_void;
pub trait MTLRasterizationRateLayerDescriptor_: Sized + std::ops::Deref {
    unsafe fn setSampleCount_(&self, sampleCount: MTLSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleCount : sampleCount)
    }
}
pub type CVOptionFlags = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVSMPTETime {
    pub subframes: SInt16,
    pub subframeDivisor: SInt16,
    pub counter: UInt32,
    pub type_: UInt32,
    pub flags: UInt32,
    pub hours: SInt16,
    pub minutes: SInt16,
    pub seconds: SInt16,
    pub frames: SInt16,
}
pub type CVSMPTETimeType = u32;
pub type CVSMPTETimeFlags = u32;
pub type CVTimeFlags = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVTime {
    pub timeValue: i64,
    pub timeScale: i32,
    pub flags: i32,
}
pub type CVTimeStampFlags = u64;
pub type CVReturn = i32;
pub type CVAttachmentMode = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVBuffer {
    _unused: [u8; 0],
}
pub type CVBufferRef = *mut __CVBuffer;
pub type CVImageBufferRef = CVBufferRef;
pub type CVPixelBufferLockFlags = CVOptionFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVPlanarComponentInfo {
    pub offset: i32,
    pub rowBytes: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVPlanarPixelBufferInfo {
    pub componentInfo: [CVPlanarComponentInfo; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVPlanarPixelBufferInfo_YCbCrPlanar {
    pub componentInfoY: CVPlanarComponentInfo,
    pub componentInfoCb: CVPlanarComponentInfo,
    pub componentInfoCr: CVPlanarComponentInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVPlanarPixelBufferInfo_YCbCrBiPlanar {
    pub componentInfoY: CVPlanarComponentInfo,
    pub componentInfoCbCr: CVPlanarComponentInfo,
}
pub type CVPixelBufferRef = CVImageBufferRef;
pub type CVPixelBufferReleaseBytesCallback = ::std::option::Option<
    unsafe extern "C" fn(
        releaseRefCon: *mut ::std::os::raw::c_void,
        baseAddress: *const ::std::os::raw::c_void,
    ),
>;
pub type CVPixelBufferReleasePlanarBytesCallback = ::std::option::Option<
    unsafe extern "C" fn(
        releaseRefCon: *mut ::std::os::raw::c_void,
        dataPtr: *const ::std::os::raw::c_void,
        dataSize: usize,
        numberOfPlanes: usize,
        planeAddresses: *mut *const ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVPixelBufferPool {
    _unused: [u8; 0],
}
pub type CVPixelBufferPoolRef = *mut __CVPixelBufferPool;
pub type CVPixelBufferPoolFlushFlags = CVOptionFlags;
pub type CVMetalTextureRef = CVImageBufferRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVMetalTextureCache {
    _unused: [u8; 0],
}
pub type CVMetalTextureCacheRef = *mut __CVMetalTextureCache;
pub type CVMetalBufferRef = CVBufferRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVMetalBufferCache {
    _unused: [u8; 0],
}
pub type CVMetalBufferCacheRef = *mut __CVMetalBufferCache;
unsafe extern "C" {
    pub static kCVZeroTime: CVTime;
}
unsafe extern "C" {
    pub static kCVIndefiniteTime: CVTime;
}
unsafe extern "C" {
    pub fn CVGetCurrentHostTime() -> u64;
}
unsafe extern "C" {
    pub fn CVGetHostClockFrequency() -> f64;
}
unsafe extern "C" {
    pub fn CVGetHostClockMinimumTimeDelta() -> u32;
}
unsafe extern "C" {
    pub static mut kCVBufferPropagatedAttachmentsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVBufferNonPropagatedAttachmentsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVBufferMovieTimeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVBufferTimeValueKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVBufferTimeScaleKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVBufferRetain(buffer: CVBufferRef) -> CVBufferRef;
}
unsafe extern "C" {
    pub fn CVBufferRelease(buffer: CVBufferRef);
}
unsafe extern "C" {
    pub fn CVBufferSetAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        value: CFTypeRef,
        attachmentMode: CVAttachmentMode,
    );
}
unsafe extern "C" {
    pub fn CVBufferGetAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        attachmentMode: *mut CVAttachmentMode,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CVBufferRemoveAttachment(buffer: CVBufferRef, key: CFStringRef);
}
unsafe extern "C" {
    pub fn CVBufferRemoveAllAttachments(buffer: CVBufferRef);
}
unsafe extern "C" {
    pub fn CVBufferGetAttachments(
        buffer: CVBufferRef,
        attachmentMode: CVAttachmentMode,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVBufferSetAttachments(
        buffer: CVBufferRef,
        theAttachments: CFDictionaryRef,
        attachmentMode: CVAttachmentMode,
    );
}
unsafe extern "C" {
    pub fn CVBufferPropagateAttachments(sourceBuffer: CVBufferRef, destinationBuffer: CVBufferRef);
}
unsafe extern "C" {
    pub fn CVBufferCopyAttachments(
        buffer: CVBufferRef,
        attachmentMode: CVAttachmentMode,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVBufferCopyAttachment(
        buffer: CVBufferRef,
        key: CFStringRef,
        attachmentMode: *mut CVAttachmentMode,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CVBufferHasAttachment(buffer: CVBufferRef, key: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCGColorSpaceKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureHorizontalOffsetKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferCleanApertureVerticalOffsetKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPreferredCleanApertureKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailTemporalTopFirst: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailTemporalBottomFirst: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailSpatialFirstLineEarly: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferFieldDetailSpatialFirstLineLate: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPixelAspectRatioKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPixelAspectRatioHorizontalSpacingKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferPixelAspectRatioVerticalSpacingKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayDimensionsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferGammaLevelKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferICCProfileKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrixKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_ITU_R_601_4: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_DCI_P3: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_P3_D65: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferYCbCrMatrix_ITU_R_2020: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimariesKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_EBU_3213: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_SMPTE_C: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_P22: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_DCI_P3: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_P3_D65: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferColorPrimaries_ITU_R_2020: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunctionKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_ITU_R_709_2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_SMPTE_240M_1995: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_UseGamma: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_sRGB: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_ITU_R_2020: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_SMPTE_ST_428_1: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_ITU_R_2100_HLG: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferTransferFunction_Linear: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocationTopFieldKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocationBottomFieldKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_Left: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_Center: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_TopLeft: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_Top: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_BottomLeft: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_Bottom: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaLocation_DV420: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaSubsamplingKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaSubsampling_420: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaSubsampling_422: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferChromaSubsampling_411: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAlphaChannelIsOpaque: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAlphaChannelModeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAlphaChannelMode_StraightAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAlphaChannelMode_PremultipliedAlpha: CFStringRef;
}
unsafe extern "C" {
    pub fn CVYCbCrMatrixGetIntegerCodePointForString(
        yCbCrMatrixString: CFStringRef,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CVColorPrimariesGetIntegerCodePointForString(
        colorPrimariesString: CFStringRef,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CVTransferFunctionGetIntegerCodePointForString(
        transferFunctionString: CFStringRef,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn CVYCbCrMatrixGetStringForIntegerCodePoint(
        yCbCrMatrixCodePoint: ::std::os::raw::c_int,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CVColorPrimariesGetStringForIntegerCodePoint(
        colorPrimariesCodePoint: ::std::os::raw::c_int,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CVTransferFunctionGetStringForIntegerCodePoint(
        transferFunctionCodePoint: ::std::os::raw::c_int,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CVImageBufferGetEncodedSize(imageBuffer: CVImageBufferRef) -> CGSize;
}
unsafe extern "C" {
    pub fn CVImageBufferGetDisplaySize(imageBuffer: CVImageBufferRef) -> CGSize;
}
unsafe extern "C" {
    pub fn CVImageBufferGetCleanRect(imageBuffer: CVImageBufferRef) -> CGRect;
}
unsafe extern "C" {
    pub fn CVImageBufferIsFlipped(imageBuffer: CVImageBufferRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CVImageBufferGetColorSpace(imageBuffer: CVImageBufferRef) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn CVImageBufferCreateColorSpaceFromAttachments(
        attachments: CFDictionaryRef,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferMasteringDisplayColorVolumeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferContentLightLevelInfoKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferAmbientViewingEnvironmentKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferSceneIlluminationKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferRegionOfInterestKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferLogTransferFunctionKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferLogTransferFunction_AppleLog: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferLogTransferFunction_AppleLog2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangleKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_ReferenceRasterWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_ReferenceRasterHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RectangleLeftKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RectangleWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RectangleTopKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RectangleHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangleStereoLeftKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangleStereoRightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_LeftEdgePointsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVImageBufferDisplayMaskRectangle_RightEdgePointsKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPixelFormatTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferMemoryAllocatorKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferExtendedPixelsLeftKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferExtendedPixelsTopKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferExtendedPixelsRightKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferExtendedPixelsBottomKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferBytesPerRowAlignmentKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferCGBitmapContextCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferCGImageCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferOpenGLCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPlaneAlignmentKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfacePropertiesKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferOpenGLESCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferMetalCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferOpenGLESTextureCacheCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferVersatileBayerKey_BayerPattern: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_SenselSitingOffsets: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_BlackLevel: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_WhiteLevel: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_WhiteBalanceCCT: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_WhiteBalanceRedFactor: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_WhiteBalanceBlueFactor: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_ColorMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_GainFactor: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_RecommendedCrop: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferProResRAWKey_MetadataExtension: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfacePurgeableKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVPixelBufferRetain(texture: CVPixelBufferRef) -> CVPixelBufferRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferRelease(texture: CVPixelBufferRef);
}
unsafe extern "C" {
    pub fn CVPixelBufferCreateResolvedAttributesDictionary(
        allocator: CFAllocatorRef,
        attributes: CFArrayRef,
        resolvedDictionaryOut: *mut CFDictionaryRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferCreate(
        allocator: CFAllocatorRef,
        width: usize,
        height: usize,
        pixelFormatType: OSType,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferCreateWithBytes(
        allocator: CFAllocatorRef,
        width: usize,
        height: usize,
        pixelFormatType: OSType,
        baseAddress: *mut ::std::os::raw::c_void,
        bytesPerRow: usize,
        releaseCallback: CVPixelBufferReleaseBytesCallback,
        releaseRefCon: *mut ::std::os::raw::c_void,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferCreateWithPlanarBytes(
        allocator: CFAllocatorRef,
        width: usize,
        height: usize,
        pixelFormatType: OSType,
        dataPtr: *mut ::std::os::raw::c_void,
        dataSize: usize,
        numberOfPlanes: usize,
        planeBaseAddress: *mut *mut ::std::os::raw::c_void,
        planeWidth: *mut usize,
        planeHeight: *mut usize,
        planeBytesPerRow: *mut usize,
        releaseCallback: CVPixelBufferReleasePlanarBytesCallback,
        releaseRefCon: *mut ::std::os::raw::c_void,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferLockBaseAddress(
        pixelBuffer: CVPixelBufferRef,
        lockFlags: CVPixelBufferLockFlags,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferUnlockBaseAddress(
        pixelBuffer: CVPixelBufferRef,
        unlockFlags: CVPixelBufferLockFlags,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetWidth(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetHeight(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetPixelFormatType(pixelBuffer: CVPixelBufferRef) -> OSType;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetBaseAddress(
        pixelBuffer: CVPixelBufferRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetBytesPerRow(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetDataSize(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferIsPlanar(pixelBuffer: CVPixelBufferRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetPlaneCount(pixelBuffer: CVPixelBufferRef) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetWidthOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: usize) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetHeightOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: usize)
        -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetBaseAddressOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: usize,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetBytesPerRowOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: usize,
    ) -> usize;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetExtendedPixels(
        pixelBuffer: CVPixelBufferRef,
        extraColumnsOnLeft: *mut usize,
        extraColumnsOnRight: *mut usize,
        extraRowsOnTop: *mut usize,
        extraRowsOnBottom: *mut usize,
    );
}
unsafe extern "C" {
    pub fn CVPixelBufferFillExtendedPixels(pixelBuffer: CVPixelBufferRef) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferCopyCreationAttributes(pixelBuffer: CVPixelBufferRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferIsCompatibleWithAttributes(
        pixelBuffer: CVPixelBufferRef,
        attributes: CFDictionaryRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceOpenGLESTextureCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferIOSurfaceOpenGLESFBOCompatibilityKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferGetIOSurface(pixelBuffer: CVPixelBufferRef) -> IOSurfaceRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferCreateWithIOSurface(
        allocator: CFAllocatorRef,
        surface: IOSurfaceRef,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPoolMinimumBufferCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPoolMaximumBufferAgeKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolRetain(pixelBufferPool: CVPixelBufferPoolRef) -> CVPixelBufferPoolRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolRelease(pixelBufferPool: CVPixelBufferPoolRef);
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolCreate(
        allocator: CFAllocatorRef,
        poolAttributes: CFDictionaryRef,
        pixelBufferAttributes: CFDictionaryRef,
        poolOut: *mut CVPixelBufferPoolRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolGetAttributes(pool: CVPixelBufferPoolRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolGetPixelBufferAttributes(pool: CVPixelBufferPoolRef)
        -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolCreatePixelBuffer(
        allocator: CFAllocatorRef,
        pixelBufferPool: CVPixelBufferPoolRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
        allocator: CFAllocatorRef,
        pixelBufferPool: CVPixelBufferPoolRef,
        auxAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPoolAllocationThresholdKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelBufferPoolFreeBufferNotification: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelBufferPoolFlush(pool: CVPixelBufferPoolRef, options: CVPixelBufferPoolFlushFlags);
}
unsafe extern "C" {
    pub static mut kCVPixelFormatName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatConstant: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatCodecType: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatFourCC: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsYCbCr: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsRGB: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsGrayscale: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatContainsSenselArray: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatComponentRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatComponentRange_VideoRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatComponentRange_FullRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatComponentRange_WideRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatPlanes: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlockWidth: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlockHeight: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBitsPerBlock: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBitsPerComponent: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlockHorizontalAlignment: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlockVerticalAlignment: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatBlackBlock: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatHorizontalSubsampling: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatVerticalSubsampling: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLFormat: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLType: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLInternalFormat: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatCGBitmapInfo: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatQDCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatCGBitmapContextCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatCGImageCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatOpenGLESCompatibility: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVPixelFormatFillExtendedPixelsCallback: CFStringRef;
}
unsafe extern "C" {
    pub fn CVPixelFormatDescriptionCreateWithPixelFormatType(
        allocator: CFAllocatorRef,
        pixelFormat: OSType,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes(
        allocator: CFAllocatorRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType(
        description: CFDictionaryRef,
        pixelFormat: OSType,
    );
}
unsafe extern "C" {
    pub fn CVPixelFormatTypeCopyFourCharCodeString(pixelFormat: OSType) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CVIsCompressedPixelFormatAvailable(pixelFormatType: OSType) -> Boolean;
}
unsafe extern "C" {
    pub fn CVMetalTextureGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVMetalTextureGetTexture(image: CVMetalTextureRef) -> *mut u64;
}
unsafe extern "C" {
    pub fn CVMetalTextureIsFlipped(image: CVMetalTextureRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CVMetalTextureGetCleanTexCoords(
        image: CVMetalTextureRef,
        lowerLeft: *mut f32,
        lowerRight: *mut f32,
        upperRight: *mut f32,
        upperLeft: *mut f32,
    );
}
unsafe extern "C" {
    pub static mut kCVMetalTextureUsage: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVMetalTextureStorageMode: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCVMetalTextureCacheMaximumTextureAgeKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVMetalTextureCacheGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVMetalTextureCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        metalDevice: *mut u64,
        textureAttributes: CFDictionaryRef,
        cacheOut: *mut CVMetalTextureCacheRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVMetalTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVMetalTextureCacheRef,
        sourceImage: CVImageBufferRef,
        textureAttributes: CFDictionaryRef,
        pixelFormat: MTLPixelFormat,
        width: usize,
        height: usize,
        planeIndex: usize,
        textureOut: *mut CVMetalTextureRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVMetalTextureCacheFlush(textureCache: CVMetalTextureCacheRef, options: CVOptionFlags);
}
unsafe extern "C" {
    pub fn CVMetalBufferGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVMetalBufferGetBuffer(buffer: CVMetalBufferRef) -> *mut u64;
}
unsafe extern "C" {
    pub static mut kCVMetalBufferCacheMaximumBufferAgeKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CVMetalBufferCacheGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CVMetalBufferCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        metalDevice: *mut u64,
        cacheOut: *mut CVMetalBufferCacheRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVMetalBufferCacheCreateBufferFromImage(
        allocator: CFAllocatorRef,
        bufferCache: CVMetalBufferCacheRef,
        imageBuffer: CVImageBufferRef,
        bufferOut: *mut CVMetalBufferRef,
    ) -> CVReturn;
}
unsafe extern "C" {
    pub fn CVMetalBufferCacheFlush(bufferCache: CVMetalBufferCacheRef, options: CVOptionFlags);
}

unsafe impl objc2::encode::RefEncode for CVSMPTETime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVSMPTETime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVSMPTETime", &[]);
}
unsafe impl objc2::encode::RefEncode for CVTime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVTime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVTime", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for CVPlanarComponentInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVPlanarComponentInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVPlanarComponentInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CVPlanarPixelBufferInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVPlanarPixelBufferInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVPlanarPixelBufferInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CVPlanarPixelBufferInfo_YCbCrPlanar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVPlanarPixelBufferInfo_YCbCrPlanar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVPlanarPixelBufferInfo_YCbCrPlanar", &[]);
}
unsafe impl objc2::encode::RefEncode for CVPlanarPixelBufferInfo_YCbCrBiPlanar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVPlanarPixelBufferInfo_YCbCrBiPlanar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVPlanarPixelBufferInfo_YCbCrBiPlanar", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVPixelBufferPool {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVPixelBufferPool {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVPixelBufferPool", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVMetalTextureCache {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVMetalTextureCache {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVMetalTextureCache", &[]);
}
unsafe impl objc2::encode::RefEncode for __CVMetalBufferCache {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CVMetalBufferCache {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CVMetalBufferCache", &[]);
}
