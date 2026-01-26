#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::Carbon::*;
#[allow(unused_imports)]
use crate::CloudKit::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreImage::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::ImageIO::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::OpenCL::*;
#[allow(unused_imports)]
use crate::OpenGL::*;
#[allow(unused_imports)]
use crate::QuartzCore::*;
#[allow(unused_imports)]
use crate::Symbols::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CGLPBufferObj = *mut _CGLPBufferObject;
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVTimeStamp {
    pub version: u32,
    pub videoTimeScale: i32,
    pub videoTime: i64,
    pub hostTime: u64,
    pub rateScalar: f64,
    pub videoRefreshPeriod: i64,
    pub smpteTime: CVSMPTETime,
    pub flags: u64,
    pub reserved: u64,
}
pub type CVBufferRef = *mut __CVBuffer;
pub type CVPixelBufferRef = CVImageBufferRef;
pub type CIFormat = ::std::os::raw::c_int;
pub trait CIImage_AutoAdjustment: Sized + std::ops::Deref {
    unsafe fn autoAdjustmentFilters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoAdjustmentFilters)
    }
    unsafe fn autoAdjustmentFiltersWithOptions_(&self, options: NSDictionary) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, autoAdjustmentFiltersWithOptions : options)
    }
}
pub trait CIImage_LabConversion: Sized + std::ops::Deref {
    unsafe fn imageByConvertingWorkingSpaceToLab(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByConvertingWorkingSpaceToLab)
    }
    unsafe fn imageByConvertingLabToWorkingSpace(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByConvertingLabToWorkingSpace)
    }
}
pub trait CIImage_AVDepthData: Sized + std::ops::Deref {
    unsafe fn initWithDepthData_options_(
        &self,
        data: AVDepthData,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDepthData : data, options : options)
    }
    unsafe fn initWithDepthData_(&self, data: AVDepthData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDepthData : data)
    }
    unsafe fn depthData(&self) -> AVDepthData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, depthData)
    }
    unsafe fn imageWithDepthData_options_(data: AVDepthData, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithDepthData : data, options : options)
    }
    unsafe fn imageWithDepthData_(data: AVDepthData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithDepthData : data)
    }
}
pub trait CIImage_AVPortraitEffectsMatte: Sized + std::ops::Deref {
    unsafe fn initWithPortaitEffectsMatte_options_(
        &self,
        matte: AVPortraitEffectsMatte,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPortaitEffectsMatte : matte, options : options)
    }
    unsafe fn initWithPortaitEffectsMatte_(&self, matte: AVPortraitEffectsMatte) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPortaitEffectsMatte : matte)
    }
    unsafe fn portraitEffectsMatte(&self) -> AVPortraitEffectsMatte
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, portraitEffectsMatte)
    }
    unsafe fn imageWithPortaitEffectsMatte_options_(
        matte: AVPortraitEffectsMatte,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithPortaitEffectsMatte : matte, options : options)
    }
    unsafe fn imageWithPortaitEffectsMatte_(matte: AVPortraitEffectsMatte) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithPortaitEffectsMatte : matte)
    }
}
pub trait CIImage_AVSemanticSegmentationMatte: Sized + std::ops::Deref {
    unsafe fn initWithSemanticSegmentationMatte_options_(
        &self,
        matte: AVSemanticSegmentationMatte,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSemanticSegmentationMatte : matte, options : options)
    }
    unsafe fn initWithSemanticSegmentationMatte_(
        &self,
        matte: AVSemanticSegmentationMatte,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSemanticSegmentationMatte : matte)
    }
    unsafe fn semanticSegmentationMatte(&self) -> AVSemanticSegmentationMatte
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationMatte)
    }
    unsafe fn imageWithSemanticSegmentationMatte_options_(
        matte: AVSemanticSegmentationMatte,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithSemanticSegmentationMatte : matte, options : options)
    }
    unsafe fn imageWithSemanticSegmentationMatte_(
        matte: AVSemanticSegmentationMatte,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithSemanticSegmentationMatte : matte)
    }
}
pub trait CIContext_createCGImage: Sized + std::ops::Deref {
    unsafe fn createCGImage_fromRect_(&self, image: CIImage, fromRect: CGRect) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGImage : image, fromRect : fromRect)
    }
    unsafe fn createCGImage_fromRect_format_colorSpace_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGImage : image, fromRect : fromRect, format : format, colorSpace : colorSpace)
    }
    unsafe fn createCGImage_fromRect_format_colorSpace_deferred_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        deferred: BOOL,
    ) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGImage : image, fromRect : fromRect, format : format, colorSpace : colorSpace, deferred : deferred)
    }
    unsafe fn createCGImage_fromRect_format_colorSpace_deferred_calculateHDRStats_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        deferred: BOOL,
        calculateHDRStats: BOOL,
    ) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGImage : image, fromRect : fromRect, format : format, colorSpace : colorSpace, deferred : deferred, calculateHDRStats : calculateHDRStats)
    }
}
pub trait CIContext_CalculateHDRStats: Sized + std::ops::Deref {
    unsafe fn calculateHDRStatsForIOSurface_(&self, surface: IOSurfaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateHDRStatsForIOSurface : surface)
    }
    unsafe fn calculateHDRStatsForCVPixelBuffer_(&self, buffer: CVPixelBufferRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateHDRStatsForCVPixelBuffer : buffer)
    }
    unsafe fn calculateHDRStatsForCGImage_(&self, cgimage: CGImageRef) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateHDRStatsForCGImage : cgimage)
    }
    unsafe fn calculateHDRStatsForImage_(&self, image: CIImage) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateHDRStatsForImage : image)
    }
}
pub trait CIContext_OfflineGPUSupport: Sized + std::ops::Deref {
    unsafe fn offlineGPUCount() -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), offlineGPUCount)
    }
    unsafe fn contextForOfflineGPUAtIndex_(index: ::std::os::raw::c_uint) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextForOfflineGPUAtIndex : index)
    }
    unsafe fn contextForOfflineGPUAtIndex_colorSpace_options_sharedContext_(
        index: ::std::os::raw::c_uint,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        sharedContext: CGLContextObj,
    ) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextForOfflineGPUAtIndex : index, colorSpace : colorSpace, options : options, sharedContext : sharedContext)
    }
}
pub trait CIContext_ImageRepresentation: Sized + std::ops::Deref {
    unsafe fn TIFFRepresentationOfImage_format_colorSpace_options_(
        &self,
        image: CIImage,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, TIFFRepresentationOfImage : image, format : format, colorSpace : colorSpace, options : options)
    }
    unsafe fn JPEGRepresentationOfImage_colorSpace_options_(
        &self,
        image: CIImage,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, JPEGRepresentationOfImage : image, colorSpace : colorSpace, options : options)
    }
    unsafe fn HEIFRepresentationOfImage_format_colorSpace_options_(
        &self,
        image: CIImage,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, HEIFRepresentationOfImage : image, format : format, colorSpace : colorSpace, options : options)
    }
    unsafe fn HEIF10RepresentationOfImage_colorSpace_options_error_(
        &self,
        image: CIImage,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, HEIF10RepresentationOfImage : image, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn PNGRepresentationOfImage_format_colorSpace_options_(
        &self,
        image: CIImage,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, PNGRepresentationOfImage : image, format : format, colorSpace : colorSpace, options : options)
    }
    unsafe fn OpenEXRRepresentationOfImage_options_error_(
        &self,
        image: CIImage,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OpenEXRRepresentationOfImage : image, options : options, error : errorPtr)
    }
    unsafe fn writeTIFFRepresentationOfImage_toURL_format_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeTIFFRepresentationOfImage : image, toURL : url, format : format, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writePNGRepresentationOfImage_toURL_format_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writePNGRepresentationOfImage : image, toURL : url, format : format, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writeJPEGRepresentationOfImage_toURL_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeJPEGRepresentationOfImage : image, toURL : url, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writeHEIFRepresentationOfImage_toURL_format_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeHEIFRepresentationOfImage : image, toURL : url, format : format, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writeHEIF10RepresentationOfImage_toURL_colorSpace_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeHEIF10RepresentationOfImage : image, toURL : url, colorSpace : colorSpace, options : options, error : errorPtr)
    }
    unsafe fn writeOpenEXRRepresentationOfImage_toURL_options_error_(
        &self,
        image: CIImage,
        url: NSURL,
        options: NSDictionary,
        errorPtr: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeOpenEXRRepresentationOfImage : image, toURL : url, options : options, error : errorPtr)
    }
}
pub trait CIContext_CIDepthBlurEffect: Sized + std::ops::Deref {
    unsafe fn depthBlurEffectFilterForImageURL_options_(
        &self,
        url: NSURL,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImageURL : url, options : options)
    }
    unsafe fn depthBlurEffectFilterForImageData_options_(
        &self,
        data: NSData,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImageData : data, options : options)
    }
    unsafe fn depthBlurEffectFilterForImage_disparityImage_portraitEffectsMatte_orientation_options_(
        &self,
        image: CIImage,
        disparityImage: CIImage,
        portraitEffectsMatte: CIImage,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImage : image, disparityImage : disparityImage, portraitEffectsMatte : portraitEffectsMatte, orientation : orientation, options : options)
    }
    unsafe fn depthBlurEffectFilterForImage_disparityImage_portraitEffectsMatte_hairSemanticSegmentation_orientation_options_(
        &self,
        image: CIImage,
        disparityImage: CIImage,
        portraitEffectsMatte: CIImage,
        hairSemanticSegmentation: CIImage,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImage : image, disparityImage : disparityImage, portraitEffectsMatte : portraitEffectsMatte, hairSemanticSegmentation : hairSemanticSegmentation, orientation : orientation, options : options)
    }
    unsafe fn depthBlurEffectFilterForImage_disparityImage_portraitEffectsMatte_hairSemanticSegmentation_glassesMatte_gainMap_orientation_options_(
        &self,
        image: CIImage,
        disparityImage: CIImage,
        portraitEffectsMatte: CIImage,
        hairSemanticSegmentation: CIImage,
        glassesMatte: CIImage,
        gainMap: CIImage,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthBlurEffectFilterForImage : image, disparityImage : disparityImage, portraitEffectsMatte : portraitEffectsMatte, hairSemanticSegmentation : hairSemanticSegmentation, glassesMatte : glassesMatte, gainMap : gainMap, orientation : orientation, options : options)
    }
}
pub trait CIFilter_CIFilterRegistry: Sized + std::ops::Deref {
    unsafe fn filterWithName_(name: NSString) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithName : name)
    }
    unsafe fn filterWithName_keysAndValues_(name: NSString, key0: id) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithName : name, keysAndValues : key0)
    }
    unsafe fn filterWithName_withInputParameters_(name: NSString, params: NSDictionary) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithName : name, withInputParameters : params)
    }
    unsafe fn filterNamesInCategory_(category: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterNamesInCategory : category)
    }
    unsafe fn filterNamesInCategories_(categories: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterNamesInCategories : categories)
    }
    unsafe fn registerFilterName_constructor_classAttributes_(
        name: NSString,
        anObject: *mut u64,
        attributes: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), registerFilterName : name, constructor : anObject, classAttributes : attributes)
    }
    unsafe fn localizedNameForFilterName_(filterName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), localizedNameForFilterName : filterName)
    }
    unsafe fn localizedNameForCategory_(category: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), localizedNameForCategory : category)
    }
    unsafe fn localizedDescriptionForFilterName_(filterName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), localizedDescriptionForFilterName : filterName)
    }
    unsafe fn localizedReferenceDocumentationForFilterName_(filterName: NSString) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), localizedReferenceDocumentationForFilterName : filterName)
    }
}
pub trait CIFilter_CIFilterXMPSerialization: Sized + std::ops::Deref {
    unsafe fn serializedXMPFromFilters_inputImageExtent_(filters: NSArray, extent: CGRect) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), serializedXMPFromFilters : filters, inputImageExtent : extent)
    }
    unsafe fn filterArrayFromSerializedXMP_inputImageExtent_error_(
        xmpData: NSData,
        extent: CGRect,
        outError: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterArrayFromSerializedXMP : xmpData, inputImageExtent : extent, error : outError)
    }
}
pub type CIKernelROICallback = *mut ::std::os::raw::c_void;
pub trait CIBlendKernel_BuiltIn: Sized + std::ops::Deref {
    unsafe fn componentAdd() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), componentAdd)
    }
    unsafe fn componentMultiply() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), componentMultiply)
    }
    unsafe fn componentMin() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), componentMin)
    }
    unsafe fn componentMax() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), componentMax)
    }
    unsafe fn clear() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), clear)
    }
    unsafe fn source() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), source)
    }
    unsafe fn destination() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destination)
    }
    unsafe fn sourceOver() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), sourceOver)
    }
    unsafe fn destinationOver() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destinationOver)
    }
    unsafe fn sourceIn() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), sourceIn)
    }
    unsafe fn destinationIn() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destinationIn)
    }
    unsafe fn sourceOut() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), sourceOut)
    }
    unsafe fn destinationOut() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destinationOut)
    }
    unsafe fn sourceAtop() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), sourceAtop)
    }
    unsafe fn destinationAtop() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), destinationAtop)
    }
    unsafe fn exclusiveOr() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), exclusiveOr)
    }
    unsafe fn multiply() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), multiply)
    }
    unsafe fn screen() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), screen)
    }
    unsafe fn overlay() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), overlay)
    }
    unsafe fn darken() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), darken)
    }
    unsafe fn lighten() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), lighten)
    }
    unsafe fn colorDodge() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), colorDodge)
    }
    unsafe fn colorBurn() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), colorBurn)
    }
    unsafe fn hardLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), hardLight)
    }
    unsafe fn softLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), softLight)
    }
    unsafe fn difference() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), difference)
    }
    unsafe fn exclusion() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), exclusion)
    }
    unsafe fn hue() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), hue)
    }
    unsafe fn saturation() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), saturation)
    }
    unsafe fn color() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), color)
    }
    unsafe fn luminosity() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), luminosity)
    }
    unsafe fn subtract() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), subtract)
    }
    unsafe fn divide() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), divide)
    }
    unsafe fn linearBurn() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), linearBurn)
    }
    unsafe fn linearDodge() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), linearDodge)
    }
    unsafe fn vividLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), vividLight)
    }
    unsafe fn linearLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), linearLight)
    }
    unsafe fn pinLight() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), pinLight)
    }
    unsafe fn hardMix() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), hardMix)
    }
    unsafe fn darkerColor() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), darkerColor)
    }
    unsafe fn lighterColor() -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), lighterColor)
    }
}
pub trait CIImage_CIImageProvider: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_size__format_colorSpace_options_(
        &self,
        provider: id,
        width: usize,
        height: usize,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : provider, size : width, height : height, format : format, colorSpace : colorSpace, options : options)
    }
    unsafe fn imageWithImageProvider_size__format_colorSpace_options_(
        provider: id,
        width: usize,
        height: usize,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithImageProvider : provider, size : width, height : height, format : format, colorSpace : colorSpace, options : options)
    }
}
pub trait NSObject_CIImageProvider: Sized + std::ops::Deref {
    unsafe fn provideImageData_bytesPerRow_origin__size__userInfo_(
        &self,
        data: *mut ::std::os::raw::c_void,
        rowbytes: usize,
        originx: usize,
        originy: usize,
        width: usize,
        height: usize,
        info: id,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provideImageData : data, bytesPerRow : rowbytes, origin : originx, originy : originy, size : width, height : height, userInfo : info)
    }
    unsafe fn provideImageToMTLTexture_commandBuffer_originx_originy_width_height_userInfo_(
        &self,
        texture: *mut u64,
        commandBuffer: *mut u64,
        originx: usize,
        originy: usize,
        width: usize,
        height: usize,
        info: id,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provideImageToMTLTexture : texture, commandBuffer : commandBuffer, originx : originx, originy : originy, width : width, height : height, userInfo : info)
    }
}
pub trait CIImageProcessorKernel_MultipleOutputSupport: Sized + std::ops::Deref {
    unsafe fn processWithInputs_arguments_outputs_error_(
        inputs: NSArray,
        arguments: NSDictionary,
        outputs: NSArray,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), processWithInputs : inputs, arguments : arguments, outputs : outputs, error : error)
    }
    unsafe fn outputFormatAtIndex_arguments_(
        outputIndex: ::std::os::raw::c_int,
        arguments: NSDictionary,
    ) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), outputFormatAtIndex : outputIndex, arguments : arguments)
    }
    unsafe fn applyWithExtents_inputs_arguments_error_(
        extents: NSArray,
        inputs: NSArray,
        arguments: NSDictionary,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), applyWithExtents : extents, inputs : inputs, arguments : arguments, error : error)
    }
}
pub trait CIFilter_CIRAWFilter: Sized + std::ops::Deref {
    unsafe fn filterWithImageURL_options_(url: NSURL, options: NSDictionary) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithImageURL : url, options : options)
    }
    unsafe fn filterWithImageData_options_(data: NSData, options: NSDictionary) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithImageData : data, options : options)
    }
    unsafe fn filterWithCVPixelBuffer_properties_options_(
        pixelBuffer: CVPixelBufferRef,
        properties: NSDictionary,
        options: NSDictionary,
    ) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), filterWithCVPixelBuffer : pixelBuffer, properties : properties, options : options)
    }
    unsafe fn supportedRawCameraModels() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), supportedRawCameraModels)
    }
}
pub type CIRAWDecoderVersion = NSString;
pub trait CIContext_CIRenderDestination: Sized + std::ops::Deref {
    unsafe fn startTaskToRender_fromRect_toDestination_atPoint_error_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        destination: CIRenderDestination,
        atPoint: CGPoint,
        error: *mut NSError,
    ) -> CIRenderTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startTaskToRender : image, fromRect : fromRect, toDestination : destination, atPoint : atPoint, error : error)
    }
    unsafe fn startTaskToRender_toDestination_error_(
        &self,
        image: CIImage,
        destination: CIRenderDestination,
        error: *mut NSError,
    ) -> CIRenderTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startTaskToRender : image, toDestination : destination, error : error)
    }
    unsafe fn prepareRender_fromRect_toDestination_atPoint_error_(
        &self,
        image: CIImage,
        fromRect: CGRect,
        destination: CIRenderDestination,
        atPoint: CGPoint,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareRender : image, fromRect : fromRect, toDestination : destination, atPoint : atPoint, error : error)
    }
    unsafe fn startTaskToClear_error_(
        &self,
        destination: CIRenderDestination,
        error: *mut NSError,
    ) -> CIRenderTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startTaskToClear : destination, error : error)
    }
}
pub trait NSUserActivity_CIBarcodeDescriptor: Sized + std::ops::Deref {
    unsafe fn detectedBarcodeDescriptor(&self) -> CIBarcodeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectedBarcodeDescriptor)
    }
}
pub trait CIFilter_Builtins: Sized + std::ops::Deref {
    unsafe fn distanceGradientFromRedMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), distanceGradientFromRedMaskFilter)
    }
    unsafe fn gaussianGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gaussianGradientFilter)
    }
    unsafe fn hueSaturationValueGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hueSaturationValueGradientFilter)
    }
    unsafe fn linearGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearGradientFilter)
    }
    unsafe fn radialGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), radialGradientFilter)
    }
    unsafe fn signedDistanceGradientFromRedMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), signedDistanceGradientFromRedMaskFilter)
    }
    unsafe fn smoothLinearGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), smoothLinearGradientFilter)
    }
    unsafe fn sharpenLuminanceFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sharpenLuminanceFilter)
    }
    unsafe fn unsharpMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), unsharpMaskFilter)
    }
    unsafe fn circularScreenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), circularScreenFilter)
    }
    unsafe fn CMYKHalftone() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), CMYKHalftone)
    }
    unsafe fn dotScreenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), dotScreenFilter)
    }
    unsafe fn hatchedScreenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hatchedScreenFilter)
    }
    unsafe fn lineScreenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lineScreenFilter)
    }
    unsafe fn bicubicScaleTransformFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bicubicScaleTransformFilter)
    }
    unsafe fn edgePreserveUpsampleFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), edgePreserveUpsampleFilter)
    }
    unsafe fn keystoneCorrectionCombinedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), keystoneCorrectionCombinedFilter)
    }
    unsafe fn keystoneCorrectionHorizontalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), keystoneCorrectionHorizontalFilter)
    }
    unsafe fn keystoneCorrectionVerticalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), keystoneCorrectionVerticalFilter)
    }
    unsafe fn lanczosScaleTransformFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lanczosScaleTransformFilter)
    }
    unsafe fn maximumScaleTransformFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maximumScaleTransformFilter)
    }
    unsafe fn perspectiveCorrectionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveCorrectionFilter)
    }
    unsafe fn perspectiveRotateFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveRotateFilter)
    }
    unsafe fn perspectiveTransformFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveTransformFilter)
    }
    unsafe fn perspectiveTransformWithExtentFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveTransformWithExtentFilter)
    }
    unsafe fn straightenFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), straightenFilter)
    }
    unsafe fn accordionFoldTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), accordionFoldTransitionFilter)
    }
    unsafe fn barsSwipeTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), barsSwipeTransitionFilter)
    }
    unsafe fn copyMachineTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), copyMachineTransitionFilter)
    }
    unsafe fn disintegrateWithMaskTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), disintegrateWithMaskTransitionFilter)
    }
    unsafe fn dissolveTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), dissolveTransitionFilter)
    }
    unsafe fn flashTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), flashTransitionFilter)
    }
    unsafe fn modTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), modTransitionFilter)
    }
    unsafe fn pageCurlTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pageCurlTransitionFilter)
    }
    unsafe fn pageCurlWithShadowTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pageCurlWithShadowTransitionFilter)
    }
    unsafe fn rippleTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), rippleTransitionFilter)
    }
    unsafe fn swipeTransitionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), swipeTransitionFilter)
    }
    unsafe fn additionCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), additionCompositingFilter)
    }
    unsafe fn colorBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorBlendModeFilter)
    }
    unsafe fn colorBurnBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorBurnBlendModeFilter)
    }
    unsafe fn colorDodgeBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorDodgeBlendModeFilter)
    }
    unsafe fn darkenBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), darkenBlendModeFilter)
    }
    unsafe fn differenceBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), differenceBlendModeFilter)
    }
    unsafe fn divideBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), divideBlendModeFilter)
    }
    unsafe fn exclusionBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), exclusionBlendModeFilter)
    }
    unsafe fn hardLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hardLightBlendModeFilter)
    }
    unsafe fn hueBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hueBlendModeFilter)
    }
    unsafe fn lightenBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lightenBlendModeFilter)
    }
    unsafe fn linearBurnBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearBurnBlendModeFilter)
    }
    unsafe fn linearDodgeBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearDodgeBlendModeFilter)
    }
    unsafe fn linearLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearLightBlendModeFilter)
    }
    unsafe fn luminosityBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), luminosityBlendModeFilter)
    }
    unsafe fn maximumCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maximumCompositingFilter)
    }
    unsafe fn minimumCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), minimumCompositingFilter)
    }
    unsafe fn multiplyBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), multiplyBlendModeFilter)
    }
    unsafe fn multiplyCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), multiplyCompositingFilter)
    }
    unsafe fn overlayBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), overlayBlendModeFilter)
    }
    unsafe fn pinLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pinLightBlendModeFilter)
    }
    unsafe fn saturationBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), saturationBlendModeFilter)
    }
    unsafe fn screenBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), screenBlendModeFilter)
    }
    unsafe fn softLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), softLightBlendModeFilter)
    }
    unsafe fn sourceAtopCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sourceAtopCompositingFilter)
    }
    unsafe fn sourceInCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sourceInCompositingFilter)
    }
    unsafe fn sourceOutCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sourceOutCompositingFilter)
    }
    unsafe fn sourceOverCompositingFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sourceOverCompositingFilter)
    }
    unsafe fn subtractBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), subtractBlendModeFilter)
    }
    unsafe fn vividLightBlendModeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vividLightBlendModeFilter)
    }
    unsafe fn colorAbsoluteDifferenceFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorAbsoluteDifferenceFilter)
    }
    unsafe fn colorClampFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorClampFilter)
    }
    unsafe fn colorControlsFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorControlsFilter)
    }
    unsafe fn colorMatrixFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorMatrixFilter)
    }
    unsafe fn colorPolynomialFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorPolynomialFilter)
    }
    unsafe fn colorThresholdFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorThresholdFilter)
    }
    unsafe fn colorThresholdOtsuFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorThresholdOtsuFilter)
    }
    unsafe fn depthToDisparityFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), depthToDisparityFilter)
    }
    unsafe fn disparityToDepthFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), disparityToDepthFilter)
    }
    unsafe fn exposureAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), exposureAdjustFilter)
    }
    unsafe fn gammaAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gammaAdjustFilter)
    }
    unsafe fn hueAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hueAdjustFilter)
    }
    unsafe fn linearToSRGBToneCurveFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), linearToSRGBToneCurveFilter)
    }
    unsafe fn sRGBToneCurveToLinearFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sRGBToneCurveToLinearFilter)
    }
    unsafe fn systemToneMapFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), systemToneMapFilter)
    }
    unsafe fn temperatureAndTintFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), temperatureAndTintFilter)
    }
    unsafe fn toneCurveFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), toneCurveFilter)
    }
    unsafe fn toneMapHeadroomFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), toneMapHeadroomFilter)
    }
    unsafe fn vibranceFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vibranceFilter)
    }
    unsafe fn whitePointAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), whitePointAdjustFilter)
    }
    unsafe fn colorCrossPolynomialFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCrossPolynomialFilter)
    }
    unsafe fn colorCubeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCubeFilter)
    }
    unsafe fn colorCubesMixedWithMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCubesMixedWithMaskFilter)
    }
    unsafe fn colorCubeWithColorSpaceFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCubeWithColorSpaceFilter)
    }
    unsafe fn colorCurvesFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorCurvesFilter)
    }
    unsafe fn colorInvertFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorInvertFilter)
    }
    unsafe fn colorMapFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorMapFilter)
    }
    unsafe fn colorMonochromeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorMonochromeFilter)
    }
    unsafe fn colorPosterizeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), colorPosterizeFilter)
    }
    unsafe fn convertLabToRGBFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convertLabToRGBFilter)
    }
    unsafe fn convertRGBtoLabFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convertRGBtoLabFilter)
    }
    unsafe fn ditherFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), ditherFilter)
    }
    unsafe fn documentEnhancerFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), documentEnhancerFilter)
    }
    unsafe fn falseColorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), falseColorFilter)
    }
    unsafe fn LabDeltaE() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), LabDeltaE)
    }
    unsafe fn maskToAlphaFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maskToAlphaFilter)
    }
    unsafe fn maximumComponentFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maximumComponentFilter)
    }
    unsafe fn minimumComponentFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), minimumComponentFilter)
    }
    unsafe fn paletteCentroidFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), paletteCentroidFilter)
    }
    unsafe fn palettizeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), palettizeFilter)
    }
    unsafe fn photoEffectChromeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectChromeFilter)
    }
    unsafe fn photoEffectFadeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectFadeFilter)
    }
    unsafe fn photoEffectInstantFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectInstantFilter)
    }
    unsafe fn photoEffectMonoFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectMonoFilter)
    }
    unsafe fn photoEffectNoirFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectNoirFilter)
    }
    unsafe fn photoEffectProcessFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectProcessFilter)
    }
    unsafe fn photoEffectTonalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectTonalFilter)
    }
    unsafe fn photoEffectTransferFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), photoEffectTransferFilter)
    }
    unsafe fn sepiaToneFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sepiaToneFilter)
    }
    unsafe fn thermalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), thermalFilter)
    }
    unsafe fn vignetteFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vignetteFilter)
    }
    unsafe fn vignetteEffectFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vignetteEffectFilter)
    }
    unsafe fn xRayFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), xRayFilter)
    }
    unsafe fn bumpDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bumpDistortionFilter)
    }
    unsafe fn bumpDistortionLinearFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bumpDistortionLinearFilter)
    }
    unsafe fn circleSplashDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), circleSplashDistortionFilter)
    }
    unsafe fn circularWrapFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), circularWrapFilter)
    }
    unsafe fn displacementDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), displacementDistortionFilter)
    }
    unsafe fn drosteFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), drosteFilter)
    }
    unsafe fn glassDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), glassDistortionFilter)
    }
    unsafe fn glassLozengeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), glassLozengeFilter)
    }
    unsafe fn holeDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), holeDistortionFilter)
    }
    unsafe fn lightTunnelFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lightTunnelFilter)
    }
    unsafe fn ninePartStretchedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), ninePartStretchedFilter)
    }
    unsafe fn ninePartTiledFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), ninePartTiledFilter)
    }
    unsafe fn pinchDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pinchDistortionFilter)
    }
    unsafe fn stretchCropFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), stretchCropFilter)
    }
    unsafe fn torusLensDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), torusLensDistortionFilter)
    }
    unsafe fn twirlDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), twirlDistortionFilter)
    }
    unsafe fn vortexDistortionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), vortexDistortionFilter)
    }
    unsafe fn affineClampFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), affineClampFilter)
    }
    unsafe fn affineTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), affineTileFilter)
    }
    unsafe fn eightfoldReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), eightfoldReflectedTileFilter)
    }
    unsafe fn fourfoldReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), fourfoldReflectedTileFilter)
    }
    unsafe fn fourfoldRotatedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), fourfoldRotatedTileFilter)
    }
    unsafe fn fourfoldTranslatedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), fourfoldTranslatedTileFilter)
    }
    unsafe fn glideReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), glideReflectedTileFilter)
    }
    unsafe fn kaleidoscopeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), kaleidoscopeFilter)
    }
    unsafe fn opTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), opTileFilter)
    }
    unsafe fn parallelogramTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), parallelogramTileFilter)
    }
    unsafe fn perspectiveTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), perspectiveTileFilter)
    }
    unsafe fn sixfoldReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sixfoldReflectedTileFilter)
    }
    unsafe fn sixfoldRotatedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sixfoldRotatedTileFilter)
    }
    unsafe fn triangleKaleidoscopeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), triangleKaleidoscopeFilter)
    }
    unsafe fn triangleTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), triangleTileFilter)
    }
    unsafe fn twelvefoldReflectedTileFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), twelvefoldReflectedTileFilter)
    }
    unsafe fn attributedTextImageGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), attributedTextImageGeneratorFilter)
    }
    unsafe fn aztecCodeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), aztecCodeGeneratorFilter)
    }
    unsafe fn barcodeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), barcodeGeneratorFilter)
    }
    unsafe fn blurredRectangleGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blurredRectangleGeneratorFilter)
    }
    unsafe fn blurredRoundedRectangleGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blurredRoundedRectangleGeneratorFilter)
    }
    unsafe fn checkerboardGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), checkerboardGeneratorFilter)
    }
    unsafe fn code128BarcodeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), code128BarcodeGeneratorFilter)
    }
    unsafe fn lenticularHaloGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lenticularHaloGeneratorFilter)
    }
    unsafe fn meshGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), meshGeneratorFilter)
    }
    unsafe fn PDF417BarcodeGenerator() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), PDF417BarcodeGenerator)
    }
    unsafe fn QRCodeGenerator() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), QRCodeGenerator)
    }
    unsafe fn randomGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), randomGeneratorFilter)
    }
    unsafe fn roundedQRCodeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), roundedQRCodeGeneratorFilter)
    }
    unsafe fn roundedRectangleGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), roundedRectangleGeneratorFilter)
    }
    unsafe fn roundedRectangleStrokeGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), roundedRectangleStrokeGeneratorFilter)
    }
    unsafe fn starShineGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), starShineGeneratorFilter)
    }
    unsafe fn stripesGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), stripesGeneratorFilter)
    }
    unsafe fn sunbeamsGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sunbeamsGeneratorFilter)
    }
    unsafe fn textImageGeneratorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), textImageGeneratorFilter)
    }
    unsafe fn blendWithAlphaMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blendWithAlphaMaskFilter)
    }
    unsafe fn blendWithBlueMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blendWithBlueMaskFilter)
    }
    unsafe fn blendWithMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blendWithMaskFilter)
    }
    unsafe fn blendWithRedMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), blendWithRedMaskFilter)
    }
    unsafe fn bloomFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bloomFilter)
    }
    unsafe fn cannyEdgeDetectorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), cannyEdgeDetectorFilter)
    }
    unsafe fn comicEffectFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), comicEffectFilter)
    }
    unsafe fn convolution3X3Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution3X3Filter)
    }
    unsafe fn convolution5X5Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution5X5Filter)
    }
    unsafe fn convolution7X7Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution7X7Filter)
    }
    unsafe fn convolution9HorizontalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution9HorizontalFilter)
    }
    unsafe fn convolution9VerticalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolution9VerticalFilter)
    }
    unsafe fn convolutionRGB3X3Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB3X3Filter)
    }
    unsafe fn convolutionRGB5X5Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB5X5Filter)
    }
    unsafe fn convolutionRGB7X7Filter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB7X7Filter)
    }
    unsafe fn convolutionRGB9HorizontalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB9HorizontalFilter)
    }
    unsafe fn convolutionRGB9VerticalFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), convolutionRGB9VerticalFilter)
    }
    unsafe fn coreMLModelFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), coreMLModelFilter)
    }
    unsafe fn crystallizeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), crystallizeFilter)
    }
    unsafe fn depthOfFieldFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), depthOfFieldFilter)
    }
    unsafe fn edgesFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), edgesFilter)
    }
    unsafe fn edgeWorkFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), edgeWorkFilter)
    }
    unsafe fn gaborGradientsFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gaborGradientsFilter)
    }
    unsafe fn gloomFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gloomFilter)
    }
    unsafe fn heightFieldFromMaskFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), heightFieldFromMaskFilter)
    }
    unsafe fn hexagonalPixellateFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), hexagonalPixellateFilter)
    }
    unsafe fn highlightShadowAdjustFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), highlightShadowAdjustFilter)
    }
    unsafe fn lineOverlayFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), lineOverlayFilter)
    }
    unsafe fn mixFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), mixFilter)
    }
    unsafe fn personSegmentationFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), personSegmentationFilter)
    }
    unsafe fn pixellateFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pixellateFilter)
    }
    unsafe fn pointillizeFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), pointillizeFilter)
    }
    unsafe fn saliencyMapFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), saliencyMapFilter)
    }
    unsafe fn shadedMaterialFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), shadedMaterialFilter)
    }
    unsafe fn sobelGradientsFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), sobelGradientsFilter)
    }
    unsafe fn spotColorFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), spotColorFilter)
    }
    unsafe fn spotLightFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), spotLightFilter)
    }
    unsafe fn bokehBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), bokehBlurFilter)
    }
    unsafe fn boxBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), boxBlurFilter)
    }
    unsafe fn discBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), discBlurFilter)
    }
    unsafe fn gaussianBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), gaussianBlurFilter)
    }
    unsafe fn maskedVariableBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), maskedVariableBlurFilter)
    }
    unsafe fn medianFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), medianFilter)
    }
    unsafe fn morphologyGradientFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyGradientFilter)
    }
    unsafe fn morphologyMaximumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyMaximumFilter)
    }
    unsafe fn morphologyMinimumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyMinimumFilter)
    }
    unsafe fn morphologyRectangleMaximumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyRectangleMaximumFilter)
    }
    unsafe fn morphologyRectangleMinimumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), morphologyRectangleMinimumFilter)
    }
    unsafe fn motionBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), motionBlurFilter)
    }
    unsafe fn noiseReductionFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), noiseReductionFilter)
    }
    unsafe fn zoomBlurFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), zoomBlurFilter)
    }
    unsafe fn areaAlphaWeightedHistogramFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaAlphaWeightedHistogramFilter)
    }
    unsafe fn areaAverageFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaAverageFilter)
    }
    unsafe fn areaAverageMaximumRedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaAverageMaximumRedFilter)
    }
    unsafe fn areaBoundsRedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaBoundsRedFilter)
    }
    unsafe fn areaHistogramFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaHistogramFilter)
    }
    unsafe fn areaLogarithmicHistogramFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaLogarithmicHistogramFilter)
    }
    unsafe fn areaMaximumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMaximumFilter)
    }
    unsafe fn areaMaximumAlphaFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMaximumAlphaFilter)
    }
    unsafe fn areaMinimumFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMinimumFilter)
    }
    unsafe fn areaMinimumAlphaFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMinimumAlphaFilter)
    }
    unsafe fn areaMinMaxFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMinMaxFilter)
    }
    unsafe fn areaMinMaxRedFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), areaMinMaxRedFilter)
    }
    unsafe fn columnAverageFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), columnAverageFilter)
    }
    unsafe fn histogramDisplayFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), histogramDisplayFilter)
    }
    unsafe fn KMeansFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), KMeansFilter)
    }
    unsafe fn rowAverageFilter() -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), rowAverageFilter)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CATransform3D {
    pub m11: CGFloat,
    pub m12: CGFloat,
    pub m13: CGFloat,
    pub m14: CGFloat,
    pub m21: CGFloat,
    pub m22: CGFloat,
    pub m23: CGFloat,
    pub m24: CGFloat,
    pub m31: CGFloat,
    pub m32: CGFloat,
    pub m33: CGFloat,
    pub m34: CGFloat,
    pub m41: CGFloat,
    pub m42: CGFloat,
    pub m43: CGFloat,
    pub m44: CGFloat,
}
pub trait NSValue_CATransform3DAdditions: Sized + std::ops::Deref {
    unsafe fn CATransform3DValue(&self) -> CATransform3D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CATransform3DValue)
    }
    unsafe fn valueWithCATransform3D_(t: CATransform3D) -> NSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSValue").unwrap(), valueWithCATransform3D : t)
    }
}
pub type CAMediaTimingFillMode = NSString;
pub type CALayerContentsGravity = NSString;
pub type CALayerContentsFormat = NSString;
pub type CALayerContentsFilter = NSString;
pub type CALayerCornerCurve = NSString;
pub type CAAutoresizingMask = ::std::os::raw::c_uint;
pub type CAToneMapMode = NSString;
pub type CADynamicRange = NSString;
pub type CAEdgeAntialiasingMask = ::std::os::raw::c_uint;
pub trait NSNull_CAActionAdditions: Sized + std::ops::Deref {}
pub type CAAnimationCalculationMode = NSString;
pub type CAAnimationRotationMode = NSString;
pub type CATransitionType = NSString;
pub type CATransitionSubtype = NSString;
pub trait CALayer_CAConstraintLayoutManager: Sized + std::ops::Deref {
    unsafe fn addConstraint_(&self, c: CAConstraint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConstraint : c)
    }
    unsafe fn constraints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constraints)
    }
    unsafe fn setConstraints_(&self, constraints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstraints : constraints)
    }
}
pub type CAEmitterLayerEmitterShape = NSString;
pub type CAEmitterLayerEmitterMode = NSString;
pub type CAEmitterLayerRenderMode = NSString;
pub type CAGradientLayerType = NSString;
pub trait CALayer_CARemoteLayerServer: Sized + std::ops::Deref {
    unsafe fn layerWithRemoteClientId_(client_id: u32) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CALayer").unwrap(), layerWithRemoteClientId : client_id)
    }
}
pub type CAScrollLayerScrollMode = NSString;
pub trait CALayer_CALayerScrolling: Sized + std::ops::Deref {
    unsafe fn scrollPoint_(&self, p: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollPoint : p)
    }
    unsafe fn scrollRectToVisible_(&self, r: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollRectToVisible : r)
    }
    unsafe fn visibleRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibleRect)
    }
}
pub type CAShapeLayerFillRule = NSString;
pub type CAShapeLayerLineJoin = NSString;
pub type CAShapeLayerLineCap = NSString;
pub type CATextLayerTruncationMode = NSString;
pub type CATextLayerAlignmentMode = NSString;
pub type CAValueFunctionName = NSString;
pub type NSColorSpaceName = NSString;
pub trait NSGraphicsContext_NSGraphicsContext_RenderingOptions: Sized + std::ops::Deref {
    unsafe fn shouldAntialias(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldAntialias)
    }
    unsafe fn setShouldAntialias_(&self, shouldAntialias: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldAntialias : shouldAntialias)
    }
    unsafe fn imageInterpolation(&self) -> NSImageInterpolation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageInterpolation)
    }
    unsafe fn setImageInterpolation_(&self, imageInterpolation: NSImageInterpolation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageInterpolation : imageInterpolation)
    }
    unsafe fn patternPhase(&self) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, patternPhase)
    }
    unsafe fn setPatternPhase_(&self, patternPhase: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPatternPhase : patternPhase)
    }
    unsafe fn compositingOperation(&self) -> NSCompositingOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compositingOperation)
    }
    unsafe fn setCompositingOperation_(&self, compositingOperation: NSCompositingOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompositingOperation : compositingOperation)
    }
    unsafe fn colorRenderingIntent(&self) -> NSColorRenderingIntent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorRenderingIntent)
    }
    unsafe fn setColorRenderingIntent_(&self, colorRenderingIntent: NSColorRenderingIntent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorRenderingIntent : colorRenderingIntent)
    }
}
pub trait NSGraphicsContext_NSQuartzCoreAdditions: Sized + std::ops::Deref {
    unsafe fn CIContext(&self) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CIContext)
    }
}
pub trait NSGraphicsContext_NSGraphicsContextDeprecated: Sized + std::ops::Deref {
    unsafe fn focusStack(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusStack)
    }
    unsafe fn setFocusStack_(&self, stack: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusStack : stack)
    }
    unsafe fn graphicsPort(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, graphicsPort)
    }
    unsafe fn setGraphicsState_(gState: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSGraphicsContext").unwrap(), setGraphicsState : gState)
    }
    unsafe fn graphicsContextWithGraphicsPort_flipped_(
        graphicsPort: *mut ::std::os::raw::c_void,
        initialFlippedState: BOOL,
    ) -> NSGraphicsContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSGraphicsContext").unwrap(), graphicsContextWithGraphicsPort : graphicsPort, flipped : initialFlippedState)
    }
    unsafe fn graphicsContextWithWindow_(window: NSWindow) -> NSGraphicsContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSGraphicsContext").unwrap(), graphicsContextWithWindow : window)
    }
}
pub type NSAccessibilityRole = NSString;
pub type NSAccessibilitySubrole = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSAccessibilityCustomAction(pub id);
impl std::ops::Deref for NSAccessibilityCustomAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSAccessibilityCustomAction {}
impl NSAccessibilityCustomAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSAccessibilityCustomAction").unwrap(), alloc) })
    }
}
impl INSObject for NSAccessibilityCustomAction {}
impl PNSObject for NSAccessibilityCustomAction {}
impl std::convert::TryFrom<NSObject> for NSAccessibilityCustomAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSAccessibilityCustomAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSAccessibilityCustomAction").unwrap()) };
        if is_kind_of {
            Ok(NSAccessibilityCustomAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSAccessibilityCustomAction")
        }
    }
}
impl INSAccessibilityCustomAction for NSAccessibilityCustomAction {}
pub trait INSAccessibilityCustomAction: Sized + std::ops::Deref {
    unsafe fn initWithName_handler_(
        &self,
        name: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, handler : handler)
    }
    unsafe fn initWithName_target_selector_(
        &self,
        name: NSString,
        target: *mut u64,
        selector: objc2::runtime::Sel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, target : target, selector : selector)
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
    unsafe fn handler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handler)
    }
    unsafe fn setHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHandler : handler)
    }
    unsafe fn target(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn selector(&self) -> objc2::runtime::Sel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selector)
    }
    unsafe fn setSelector_(&self, selector: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelector : selector)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSAccessibilityCustomRotor(pub id);
impl std::ops::Deref for NSAccessibilityCustomRotor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSAccessibilityCustomRotor {}
impl NSAccessibilityCustomRotor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSAccessibilityCustomRotor").unwrap(), alloc) })
    }
}
impl INSObject for NSAccessibilityCustomRotor {}
impl PNSObject for NSAccessibilityCustomRotor {}
impl std::convert::TryFrom<NSObject> for NSAccessibilityCustomRotor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSAccessibilityCustomRotor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSAccessibilityCustomRotor").unwrap()) };
        if is_kind_of {
            Ok(NSAccessibilityCustomRotor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSAccessibilityCustomRotor")
        }
    }
}
impl INSAccessibilityCustomRotor for NSAccessibilityCustomRotor {}
pub trait INSAccessibilityCustomRotor: Sized + std::ops::Deref {
    unsafe fn initWithLabel_itemSearchDelegate_(
        &self,
        label: NSString,
        itemSearchDelegate: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLabel : label, itemSearchDelegate : itemSearchDelegate)
    }
    unsafe fn initWithRotorType_itemSearchDelegate_(
        &self,
        rotorType: NSAccessibilityCustomRotorType,
        itemSearchDelegate: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRotorType : rotorType, itemSearchDelegate : itemSearchDelegate)
    }
    unsafe fn type_(&self) -> NSAccessibilityCustomRotorType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSAccessibilityCustomRotorType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn itemSearchDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemSearchDelegate)
    }
    unsafe fn setItemSearchDelegate_(&self, itemSearchDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setItemSearchDelegate : itemSearchDelegate)
    }
    unsafe fn itemLoadingDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemLoadingDelegate)
    }
    unsafe fn setItemLoadingDelegate_(&self, itemLoadingDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setItemLoadingDelegate : itemLoadingDelegate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSAccessibilityCustomRotorSearchParameters(pub id);
impl std::ops::Deref for NSAccessibilityCustomRotorSearchParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSAccessibilityCustomRotorSearchParameters {}
impl NSAccessibilityCustomRotorSearchParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSAccessibilityCustomRotorSearchParameters").unwrap(), alloc) })
    }
}
impl INSObject for NSAccessibilityCustomRotorSearchParameters {}
impl PNSObject for NSAccessibilityCustomRotorSearchParameters {}
impl std::convert::TryFrom<NSObject> for NSAccessibilityCustomRotorSearchParameters {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<NSAccessibilityCustomRotorSearchParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSAccessibilityCustomRotorSearchParameters").unwrap())
        };
        if is_kind_of {
            Ok(NSAccessibilityCustomRotorSearchParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSAccessibilityCustomRotorSearchParameters")
        }
    }
}
impl INSAccessibilityCustomRotorSearchParameters for NSAccessibilityCustomRotorSearchParameters {}
pub trait INSAccessibilityCustomRotorSearchParameters: Sized + std::ops::Deref {
    unsafe fn currentItem(&self) -> NSAccessibilityCustomRotorItemResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentItem)
    }
    unsafe fn setCurrentItem_(&self, currentItem: NSAccessibilityCustomRotorItemResult)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentItem : currentItem)
    }
    unsafe fn searchDirection(&self) -> NSAccessibilityCustomRotorSearchDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, searchDirection)
    }
    unsafe fn setSearchDirection_(&self, searchDirection: NSAccessibilityCustomRotorSearchDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSearchDirection : searchDirection)
    }
    unsafe fn filterString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterString)
    }
    unsafe fn setFilterString_(&self, filterString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterString : filterString)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSAccessibilityCustomRotorItemResult(pub id);
impl std::ops::Deref for NSAccessibilityCustomRotorItemResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSAccessibilityCustomRotorItemResult {}
impl NSAccessibilityCustomRotorItemResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSAccessibilityCustomRotorItemResult").unwrap(), alloc) })
    }
}
impl INSObject for NSAccessibilityCustomRotorItemResult {}
impl PNSObject for NSAccessibilityCustomRotorItemResult {}
impl std::convert::TryFrom<NSObject> for NSAccessibilityCustomRotorItemResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSAccessibilityCustomRotorItemResult, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSAccessibilityCustomRotorItemResult").unwrap())
        };
        if is_kind_of {
            Ok(NSAccessibilityCustomRotorItemResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSAccessibilityCustomRotorItemResult")
        }
    }
}
impl INSAccessibilityCustomRotorItemResult for NSAccessibilityCustomRotorItemResult {}
pub trait INSAccessibilityCustomRotorItemResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTargetElement_(&self, targetElement: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetElement : targetElement)
    }
    unsafe fn initWithItemLoadingToken_customLabel_(
        &self,
        itemLoadingToken: NSAccessibilityLoadingToken,
        customLabel: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithItemLoadingToken : itemLoadingToken, customLabel : customLabel)
    }
    unsafe fn targetElement(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetElement)
    }
    unsafe fn itemLoadingToken(&self) -> NSAccessibilityLoadingToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemLoadingToken)
    }
    unsafe fn targetRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetRange)
    }
    unsafe fn setTargetRange_(&self, targetRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetRange : targetRange)
    }
    unsafe fn customLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customLabel)
    }
    unsafe fn setCustomLabel_(&self, customLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomLabel : customLabel)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAccessibilityCustomRotorItemResult").unwrap(), new)
    }
}
pub trait NSWorkspace_NSDesktopImages: Sized + std::ops::Deref {
    unsafe fn setDesktopImageURL_forScreen_options_error_(
        &self,
        url: NSURL,
        screen: NSScreen,
        options: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesktopImageURL : url, forScreen : screen, options : options, error : error)
    }
    unsafe fn desktopImageURLForScreen_(&self, screen: NSScreen) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, desktopImageURLForScreen : screen)
    }
    unsafe fn desktopImageOptionsForScreen_(&self, screen: NSScreen) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, desktopImageOptionsForScreen : screen)
    }
}
pub trait NSWorkspace_NSWorkspaceAuthorization: Sized + std::ops::Deref {
    unsafe fn requestAuthorizationOfType_completionHandler_(
        &self,
        type_: NSWorkspaceAuthorizationType,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAuthorizationOfType : type_, completionHandler : completionHandler)
    }
}
pub trait NSFileManager_NSWorkspaceAuthorization: Sized + std::ops::Deref {
    unsafe fn fileManagerWithAuthorization_(authorization: NSWorkspaceAuthorization) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFileManager").unwrap(), fileManagerWithAuthorization : authorization)
    }
}
pub trait NSWorkspace_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn openFile_(&self, fullPath: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openFile : fullPath)
    }
    unsafe fn openFile_withApplication_(&self, fullPath: NSString, appName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openFile : fullPath, withApplication : appName)
    }
    unsafe fn openFile_withApplication_andDeactivate_(
        &self,
        fullPath: NSString,
        appName: NSString,
        flag: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openFile : fullPath, withApplication : appName, andDeactivate : flag)
    }
    unsafe fn launchApplication_(&self, appName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, launchApplication : appName)
    }
    unsafe fn launchApplicationAtURL_options_configuration_error_(
        &self,
        url: NSURL,
        options: NSWorkspaceLaunchOptions,
        configuration: NSDictionary,
        error: *mut NSError,
    ) -> NSRunningApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, launchApplicationAtURL : url, options : options, configuration : configuration, error : error)
    }
    unsafe fn openURL_options_configuration_error_(
        &self,
        url: NSURL,
        options: NSWorkspaceLaunchOptions,
        configuration: NSDictionary,
        error: *mut NSError,
    ) -> NSRunningApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openURL : url, options : options, configuration : configuration, error : error)
    }
    unsafe fn openURLs_withApplicationAtURL_options_configuration_error_(
        &self,
        urls: NSArray,
        applicationURL: NSURL,
        options: NSWorkspaceLaunchOptions,
        configuration: NSDictionary,
        error: *mut NSError,
    ) -> NSRunningApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openURLs : urls, withApplicationAtURL : applicationURL, options : options, configuration : configuration, error : error)
    }
    unsafe fn launchApplication_showIcon_autolaunch_(
        &self,
        appName: NSString,
        showIcon: BOOL,
        autolaunch: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, launchApplication : appName, showIcon : showIcon, autolaunch : autolaunch)
    }
    unsafe fn fullPathForApplication_(&self, appName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fullPathForApplication : appName)
    }
    unsafe fn absolutePathForAppBundleWithIdentifier_(&self, bundleIdentifier: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, absolutePathForAppBundleWithIdentifier : bundleIdentifier)
    }
    unsafe fn launchAppWithBundleIdentifier_options_additionalEventParamDescriptor_launchIdentifier_(
        &self,
        bundleIdentifier: NSString,
        options: NSWorkspaceLaunchOptions,
        descriptor: NSAppleEventDescriptor,
        identifier: *mut NSNumber,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, launchAppWithBundleIdentifier : bundleIdentifier, options : options, additionalEventParamDescriptor : descriptor, launchIdentifier : identifier)
    }
    unsafe fn openURLs_withAppBundleIdentifier_options_additionalEventParamDescriptor_launchIdentifiers_(
        &self,
        urls: NSArray,
        bundleIdentifier: NSString,
        options: NSWorkspaceLaunchOptions,
        descriptor: NSAppleEventDescriptor,
        identifiers: *mut NSArray,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openURLs : urls, withAppBundleIdentifier : bundleIdentifier, options : options, additionalEventParamDescriptor : descriptor, launchIdentifiers : identifiers)
    }
    unsafe fn openTempFile_(&self, fullPath: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openTempFile : fullPath)
    }
    unsafe fn findApplications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, findApplications)
    }
    unsafe fn noteUserDefaultsChanged(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noteUserDefaultsChanged)
    }
    unsafe fn slideImage_from_to_(&self, image: NSImage, fromPoint: NSPoint, toPoint: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, slideImage : image, from : fromPoint, to : toPoint)
    }
    unsafe fn checkForRemovableMedia(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, checkForRemovableMedia)
    }
    unsafe fn noteFileSystemChanged(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noteFileSystemChanged)
    }
    unsafe fn fileSystemChanged(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileSystemChanged)
    }
    unsafe fn userDefaultsChanged(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userDefaultsChanged)
    }
    unsafe fn mountNewRemovableMedia(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mountNewRemovableMedia)
    }
    unsafe fn activeApplication(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeApplication)
    }
    unsafe fn mountedLocalVolumePaths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mountedLocalVolumePaths)
    }
    unsafe fn mountedRemovableMedia(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mountedRemovableMedia)
    }
    unsafe fn launchedApplications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, launchedApplications)
    }
    unsafe fn openFile_fromImage_at_inView_(
        &self,
        fullPath: NSString,
        image: NSImage,
        point: NSPoint,
        view: NSView,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openFile : fullPath, fromImage : image, at : point, inView : view)
    }
    unsafe fn performFileOperation_source_destination_files_tag_(
        &self,
        operation: NSString,
        source: NSString,
        destination: NSString,
        files: NSArray,
        tag: *mut NSInteger,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performFileOperation : operation, source : source, destination : destination, files : files, tag : tag)
    }
    unsafe fn getInfoForFile_application_type_(
        &self,
        fullPath: NSString,
        appName: *mut NSString,
        type_: *mut NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getInfoForFile : fullPath, application : appName, r#type : type_)
    }
    unsafe fn iconForFileType_(&self, fileType: NSString) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, iconForFileType : fileType)
    }
    unsafe fn typeOfFile_error_(
        &self,
        absoluteFilePath: NSString,
        outError: *mut NSError,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, typeOfFile : absoluteFilePath, error : outError)
    }
    unsafe fn localizedDescriptionForType_(&self, typeName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localizedDescriptionForType : typeName)
    }
    unsafe fn preferredFilenameExtensionForType_(&self, typeName: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preferredFilenameExtensionForType : typeName)
    }
    unsafe fn filenameExtension_isValidForType_(
        &self,
        filenameExtension: NSString,
        typeName: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, filenameExtension : filenameExtension, isValidForType : typeName)
    }
    unsafe fn type_conformsToType_(&self, firstTypeName: NSString, secondTypeName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, type : firstTypeName, conformsToType : secondTypeName)
    }
}
pub trait NSObject_NSAccessibility: Sized + std::ops::Deref {
    unsafe fn accessibilityAttributeNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityAttributeNames)
    }
    unsafe fn accessibilityAttributeValue_(&self, attribute: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityAttributeValue : attribute)
    }
    unsafe fn accessibilityIsAttributeSettable_(&self, attribute: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityIsAttributeSettable : attribute)
    }
    unsafe fn accessibilitySetValue_forAttribute_(&self, value: id, attribute: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilitySetValue : value, forAttribute : attribute)
    }
    unsafe fn accessibilityParameterizedAttributeNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityParameterizedAttributeNames)
    }
    unsafe fn accessibilityAttributeValue_forParameter_(
        &self,
        attribute: NSString,
        parameter: id,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityAttributeValue : attribute, forParameter : parameter)
    }
    unsafe fn accessibilityActionNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityActionNames)
    }
    unsafe fn accessibilityActionDescription_(&self, action: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityActionDescription : action)
    }
    unsafe fn accessibilityPerformAction_(&self, action: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityPerformAction : action)
    }
    unsafe fn accessibilityIsIgnored(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityIsIgnored)
    }
    unsafe fn accessibilityHitTest_(&self, point: NSPoint) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityHitTest : point)
    }
    unsafe fn accessibilityIndexOfChild_(&self, child: id) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityIndexOfChild : child)
    }
    unsafe fn accessibilityArrayAttributeCount_(&self, attribute: NSString) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityArrayAttributeCount : attribute)
    }
    unsafe fn accessibilityArrayAttributeValues_index_maxCount_(
        &self,
        attribute: NSString,
        index: NSUInteger,
        maxCount: NSUInteger,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityArrayAttributeValues : attribute, index : index, maxCount : maxCount)
    }
    unsafe fn accessibilityFocusedUIElement(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityFocusedUIElement)
    }
    unsafe fn accessibilityNotifiesWhenDestroyed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityNotifiesWhenDestroyed)
    }
}
pub trait NSWorkspace_NSWorkspaceAccessibilityDisplay: Sized + std::ops::Deref {
    unsafe fn accessibilityDisplayShouldIncreaseContrast(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityDisplayShouldIncreaseContrast)
    }
    unsafe fn accessibilityDisplayShouldDifferentiateWithoutColor(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityDisplayShouldDifferentiateWithoutColor)
    }
    unsafe fn accessibilityDisplayShouldReduceTransparency(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityDisplayShouldReduceTransparency)
    }
    unsafe fn accessibilityDisplayShouldReduceMotion(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityDisplayShouldReduceMotion)
    }
    unsafe fn accessibilityDisplayShouldInvertColors(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityDisplayShouldInvertColors)
    }
}
pub trait NSWorkspace_NSWorkspaceAccessibility: Sized + std::ops::Deref {
    unsafe fn isVoiceOverEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVoiceOverEnabled)
    }
    unsafe fn isSwitchControlEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSwitchControlEnabled)
    }
}
pub trait NSObject_NSAccessibilityAdditions: Sized + std::ops::Deref {
    unsafe fn accessibilitySetOverrideValue_forAttribute_(
        &self,
        value: id,
        attribute: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilitySetOverrideValue : value, forAttribute : attribute)
    }
}
pub trait NSTouch_NSTouchBar: Sized + std::ops::Deref {
    unsafe fn locationInView_(&self, view: NSView) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationInView : view)
    }
    unsafe fn previousLocationInView_(&self, view: NSView) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, previousLocationInView : view)
    }
    unsafe fn type_(&self) -> NSTouchType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
pub type NSEventMask = ::std::os::raw::c_ulonglong;
pub type NSPasteboardType = NSString;
pub type NSPasteboardName = NSString;
pub trait NSPasteboard_FilterServices: Sized + std::ops::Deref {
    unsafe fn typesFilterableTo_(type_: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPasteboard").unwrap(), typesFilterableTo : type_)
    }
    unsafe fn pasteboardByFilteringFile_(filename: NSString) -> NSPasteboard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPasteboard").unwrap(), pasteboardByFilteringFile : filename)
    }
    unsafe fn pasteboardByFilteringData_ofType_(data: NSData, type_: NSString) -> NSPasteboard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPasteboard").unwrap(), pasteboardByFilteringData : data, ofType : type_)
    }
    unsafe fn pasteboardByFilteringTypesInPasteboard_(pboard: NSPasteboard) -> NSPasteboard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPasteboard").unwrap(), pasteboardByFilteringTypesInPasteboard : pboard)
    }
}
pub trait NSObject_NSPasteboardOwner: Sized + std::ops::Deref {
    unsafe fn pasteboard_provideDataForType_(&self, sender: NSPasteboard, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pasteboard : sender, provideDataForType : type_)
    }
    unsafe fn pasteboardChangedOwner_(&self, sender: NSPasteboard)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pasteboardChangedOwner : sender)
    }
}
pub trait NSURL_NSPasteboardSupport: Sized + std::ops::Deref {
    unsafe fn writeToPasteboard_(&self, pasteBoard: NSPasteboard)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToPasteboard : pasteBoard)
    }
    unsafe fn URLFromPasteboard_(pasteBoard: NSPasteboard) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSURL").unwrap(), URLFromPasteboard : pasteBoard)
    }
}
pub trait NSString_NSPasteboardSupport: Sized + std::ops::Deref {}
pub trait NSPasteboard_NSFileContents: Sized + std::ops::Deref {
    unsafe fn writeFileContents_(&self, filename: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeFileContents : filename)
    }
    unsafe fn readFileContentsType_toFile_(&self, type_: NSString, filename: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readFileContentsType : type_, toFile : filename)
    }
    unsafe fn writeFileWrapper_(&self, wrapper: NSFileWrapper) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeFileWrapper : wrapper)
    }
    unsafe fn readFileWrapper(&self) -> NSFileWrapper
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readFileWrapper)
    }
}
pub trait NSResponder_NSStandardKeyBindingMethods: Sized + std::ops::Deref {}
pub trait NSResponder_NSUndoSupport: Sized + std::ops::Deref {
    unsafe fn undoManager(&self) -> NSUndoManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, undoManager)
    }
}
pub trait NSResponder_NSControlEditingSupport: Sized + std::ops::Deref {
    unsafe fn validateProposedFirstResponder_forEvent_(
        &self,
        responder: NSResponder,
        event: NSEvent,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateProposedFirstResponder : responder, forEvent : event)
    }
}
pub trait NSResponder_NSErrorPresentation: Sized + std::ops::Deref {
    unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo_(
        &self,
        error: NSError,
        window: NSWindow,
        delegate: id,
        didPresentSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentError : error, modalForWindow : window, delegate : delegate, didPresentSelector : didPresentSelector, contextInfo : contextInfo)
    }
    unsafe fn presentError_(&self, error: NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentError : error)
    }
    unsafe fn willPresentError_(&self, error: NSError) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willPresentError : error)
    }
}
pub trait NSResponder_NSTextFinderSupport: Sized + std::ops::Deref {
    unsafe fn performTextFinderAction_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performTextFinderAction : sender)
    }
}
pub trait NSResponder_NSWindowTabbing: Sized + std::ops::Deref {
    unsafe fn newWindowForTab_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newWindowForTab : sender)
    }
}
pub trait NSResponder_NSWritingToolsSupport: Sized + std::ops::Deref {
    unsafe fn showWritingTools_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showWritingTools : sender)
    }
}
pub trait NSResponder_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn performMnemonic_(&self, string: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performMnemonic : string)
    }
}
pub trait NSWorkspace_NSWorkspaceRunningApplications: Sized + std::ops::Deref {
    unsafe fn runningApplications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, runningApplications)
    }
}
pub type NSNibName = NSString;
pub trait NSNib_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_(&self, nibFileURL: NSURL) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : nibFileURL)
    }
    unsafe fn instantiateNibWithExternalNameTable_(&self, externalNameTable: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, instantiateNibWithExternalNameTable : externalNameTable)
    }
    unsafe fn instantiateNibWithOwner_topLevelObjects_(
        &self,
        owner: id,
        topLevelObjects: *mut NSArray,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, instantiateNibWithOwner : owner, topLevelObjects : topLevelObjects)
    }
}
pub type NSUserInterfaceItemIdentifier = NSString;
pub type NSAppearanceName = NSString;
pub trait NSObject_NSDraggingSourceDeprecated: Sized + std::ops::Deref {
    unsafe fn namesOfPromisedFilesDroppedAtDestination_(&self, dropDestination: NSURL) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, namesOfPromisedFilesDroppedAtDestination : dropDestination)
    }
    unsafe fn draggingSourceOperationMaskForLocal_(&self, flag: BOOL) -> NSDragOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, draggingSourceOperationMaskForLocal : flag)
    }
    unsafe fn draggedImage_beganAt_(&self, image: NSImage, screenPoint: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, draggedImage : image, beganAt : screenPoint)
    }
    unsafe fn draggedImage_endedAt_operation_(
        &self,
        image: NSImage,
        screenPoint: NSPoint,
        operation: NSDragOperation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, draggedImage : image, endedAt : screenPoint, operation : operation)
    }
    unsafe fn draggedImage_movedTo_(&self, image: NSImage, screenPoint: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, draggedImage : image, movedTo : screenPoint)
    }
    unsafe fn ignoreModifierKeysWhileDragging(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoreModifierKeysWhileDragging)
    }
    unsafe fn draggedImage_endedAt_deposited_(
        &self,
        image: NSImage,
        screenPoint: NSPoint,
        flag: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, draggedImage : image, endedAt : screenPoint, deposited : flag)
    }
}
pub trait NSObject_NSLayerDelegateContentsScaleUpdating: Sized + std::ops::Deref {
    unsafe fn layer_shouldInheritContentsScale_fromWindow_(
        &self,
        layer: CALayer,
        newScale: CGFloat,
        window: NSWindow,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layer : layer, shouldInheritContentsScale : newScale, fromWindow : window)
    }
}
pub trait NSObject_NSToolTipOwner: Sized + std::ops::Deref {
    unsafe fn view_stringForToolTip_point_userData_(
        &self,
        view: NSView,
        tag: NSToolTipTag,
        point: NSPoint,
        data: *mut ::std::os::raw::c_void,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, view : view, stringForToolTip : tag, point : point, userData : data)
    }
}
pub trait NSView_NSKeyboardUI: Sized + std::ops::Deref {
    unsafe fn setKeyboardFocusRingNeedsDisplayInRect_(&self, rect: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyboardFocusRingNeedsDisplayInRect : rect)
    }
    unsafe fn drawFocusRingMask(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawFocusRingMask)
    }
    unsafe fn noteFocusRingMaskChanged(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noteFocusRingMaskChanged)
    }
    unsafe fn nextKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextKeyView)
    }
    unsafe fn setNextKeyView_(&self, nextKeyView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNextKeyView : nextKeyView)
    }
    unsafe fn previousKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousKeyView)
    }
    unsafe fn nextValidKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextValidKeyView)
    }
    unsafe fn previousValidKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousValidKeyView)
    }
    unsafe fn canBecomeKeyView(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canBecomeKeyView)
    }
    unsafe fn focusRingType(&self) -> NSFocusRingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusRingType)
    }
    unsafe fn setFocusRingType_(&self, focusRingType: NSFocusRingType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusRingType : focusRingType)
    }
    unsafe fn focusRingMaskBounds(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusRingMaskBounds)
    }
    unsafe fn defaultFocusRingType() -> NSFocusRingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSView").unwrap(), defaultFocusRingType)
    }
}
pub trait NSView_NSPrinting: Sized + std::ops::Deref {
    unsafe fn writeEPSInsideRect_toPasteboard_(&self, rect: NSRect, pasteboard: NSPasteboard)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeEPSInsideRect : rect, toPasteboard : pasteboard)
    }
    unsafe fn dataWithEPSInsideRect_(&self, rect: NSRect) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dataWithEPSInsideRect : rect)
    }
    unsafe fn writePDFInsideRect_toPasteboard_(&self, rect: NSRect, pasteboard: NSPasteboard)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writePDFInsideRect : rect, toPasteboard : pasteboard)
    }
    unsafe fn dataWithPDFInsideRect_(&self, rect: NSRect) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dataWithPDFInsideRect : rect)
    }
    unsafe fn print_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, print : sender)
    }
    unsafe fn knowsPageRange_(&self, range: NSRangePointer) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, knowsPageRange : range)
    }
    unsafe fn adjustPageWidthNew_left_right_limit_(
        &self,
        newRight: *mut CGFloat,
        oldLeft: CGFloat,
        oldRight: CGFloat,
        rightLimit: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, adjustPageWidthNew : newRight, left : oldLeft, right : oldRight, limit : rightLimit)
    }
    unsafe fn adjustPageHeightNew_top_bottom_limit_(
        &self,
        newBottom: *mut CGFloat,
        oldTop: CGFloat,
        oldBottom: CGFloat,
        bottomLimit: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, adjustPageHeightNew : newBottom, top : oldTop, bottom : oldBottom, limit : bottomLimit)
    }
    unsafe fn rectForPage_(&self, page: NSInteger) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectForPage : page)
    }
    unsafe fn locationOfPrintRect_(&self, rect: NSRect) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationOfPrintRect : rect)
    }
    unsafe fn drawPageBorderWithSize_(&self, borderSize: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawPageBorderWithSize : borderSize)
    }
    unsafe fn drawSheetBorderWithSize_(&self, borderSize: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawSheetBorderWithSize : borderSize)
    }
    unsafe fn beginDocument(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginDocument)
    }
    unsafe fn endDocument(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDocument)
    }
    unsafe fn beginPageInRect_atPlacement_(&self, rect: NSRect, location: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginPageInRect : rect, atPlacement : location)
    }
    unsafe fn endPage(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endPage)
    }
    unsafe fn heightAdjustLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightAdjustLimit)
    }
    unsafe fn widthAdjustLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widthAdjustLimit)
    }
    unsafe fn pageHeader(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageHeader)
    }
    unsafe fn pageFooter(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pageFooter)
    }
    unsafe fn printJobTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, printJobTitle)
    }
}
pub trait NSView_NSDrag: Sized + std::ops::Deref {
    unsafe fn beginDraggingSessionWithItems_event_source_(
        &self,
        items: NSArray,
        event: NSEvent,
        source: *mut u64,
    ) -> NSDraggingSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginDraggingSessionWithItems : items, event : event, source : source)
    }
    unsafe fn registerForDraggedTypes_(&self, newTypes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerForDraggedTypes : newTypes)
    }
    unsafe fn unregisterDraggedTypes(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unregisterDraggedTypes)
    }
    unsafe fn registeredDraggedTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registeredDraggedTypes)
    }
}
pub trait NSView_NSFullScreenMode: Sized + std::ops::Deref {
    unsafe fn enterFullScreenMode_withOptions_(
        &self,
        screen: NSScreen,
        options: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enterFullScreenMode : screen, withOptions : options)
    }
    unsafe fn exitFullScreenModeWithOptions_(&self, options: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exitFullScreenModeWithOptions : options)
    }
    unsafe fn isInFullScreenMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInFullScreenMode)
    }
}
pub trait NSView_NSDefinition: Sized + std::ops::Deref {
    unsafe fn showDefinitionForAttributedString_atPoint_(
        &self,
        attrString: NSAttributedString,
        textBaselineOrigin: NSPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showDefinitionForAttributedString : attrString, atPoint : textBaselineOrigin)
    }
    unsafe fn showDefinitionForAttributedString_range_options_baselineOriginProvider_(
        &self,
        attrString: NSAttributedString,
        targetRange: NSRange,
        options: NSDictionary,
        originProvider: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showDefinitionForAttributedString : attrString, range : targetRange, options : options, baselineOriginProvider : originProvider)
    }
}
pub trait NSView_NSFindIndicator: Sized + std::ops::Deref {
    unsafe fn isDrawingFindIndicator(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDrawingFindIndicator)
    }
}
pub trait NSView_NSGestureRecognizer: Sized + std::ops::Deref {
    unsafe fn addGestureRecognizer_(&self, gestureRecognizer: NSGestureRecognizer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addGestureRecognizer : gestureRecognizer)
    }
    unsafe fn removeGestureRecognizer_(&self, gestureRecognizer: NSGestureRecognizer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeGestureRecognizer : gestureRecognizer)
    }
    unsafe fn gestureRecognizers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gestureRecognizers)
    }
    unsafe fn setGestureRecognizers_(&self, gestureRecognizers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGestureRecognizers : gestureRecognizers)
    }
}
pub trait NSView_NSTouchBar: Sized + std::ops::Deref {
    unsafe fn allowedTouchTypes(&self) -> NSTouchTypeMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedTouchTypes)
    }
    unsafe fn setAllowedTouchTypes_(&self, allowedTouchTypes: NSTouchTypeMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedTouchTypes : allowedTouchTypes)
    }
}
pub trait NSView_NSSafeAreas: Sized + std::ops::Deref {
    unsafe fn safeAreaInsets(&self) -> NSEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, safeAreaInsets)
    }
    unsafe fn additionalSafeAreaInsets(&self) -> NSEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalSafeAreaInsets)
    }
    unsafe fn setAdditionalSafeAreaInsets_(&self, additionalSafeAreaInsets: NSEdgeInsets)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalSafeAreaInsets : additionalSafeAreaInsets)
    }
    unsafe fn safeAreaLayoutGuide(&self) -> NSLayoutGuide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, safeAreaLayoutGuide)
    }
    unsafe fn safeAreaRect(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, safeAreaRect)
    }
    unsafe fn layoutMarginsGuide(&self) -> NSLayoutGuide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutMarginsGuide)
    }
}
pub trait NSView_NSCompactControlSizeMetrics: Sized + std::ops::Deref {
    unsafe fn prefersCompactControlSizeMetrics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prefersCompactControlSizeMetrics)
    }
    unsafe fn setPrefersCompactControlSizeMetrics_(&self, prefersCompactControlSizeMetrics: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrefersCompactControlSizeMetrics : prefersCompactControlSizeMetrics)
    }
}
pub trait NSView_NSTrackingArea: Sized + std::ops::Deref {
    unsafe fn addTrackingArea_(&self, trackingArea: NSTrackingArea)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTrackingArea : trackingArea)
    }
    unsafe fn removeTrackingArea_(&self, trackingArea: NSTrackingArea)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTrackingArea : trackingArea)
    }
    unsafe fn updateTrackingAreas(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateTrackingAreas)
    }
    unsafe fn addCursorRect_cursor_(&self, rect: NSRect, object: NSCursor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addCursorRect : rect, cursor : object)
    }
    unsafe fn removeCursorRect_cursor_(&self, rect: NSRect, object: NSCursor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeCursorRect : rect, cursor : object)
    }
    unsafe fn discardCursorRects(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discardCursorRects)
    }
    unsafe fn resetCursorRects(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetCursorRects)
    }
    unsafe fn addTrackingRect_owner_userData_assumeInside_(
        &self,
        rect: NSRect,
        owner: id,
        data: *mut ::std::os::raw::c_void,
        flag: BOOL,
    ) -> NSTrackingRectTag
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTrackingRect : rect, owner : owner, userData : data, assumeInside : flag)
    }
    unsafe fn removeTrackingRect_(&self, tag: NSTrackingRectTag)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTrackingRect : tag)
    }
    unsafe fn trackingAreas(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackingAreas)
    }
}
pub trait NSView_NSDisplayLink: Sized + std::ops::Deref {
    unsafe fn displayLinkWithTarget_selector_(
        &self,
        target: id,
        selector: objc2::runtime::Sel,
    ) -> CADisplayLink
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayLinkWithTarget : target, selector : selector)
    }
}
pub trait NSView_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn dragImage_at_offset_event_pasteboard_source_slideBack_(
        &self,
        image: NSImage,
        viewLocation: NSPoint,
        initialOffset: NSSize,
        event: NSEvent,
        pboard: NSPasteboard,
        sourceObj: id,
        slideFlag: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dragImage : image, at : viewLocation, offset : initialOffset, event : event, pasteboard : pboard, source : sourceObj, slideBack : slideFlag)
    }
    unsafe fn dragFile_fromRect_slideBack_event_(
        &self,
        filename: NSString,
        rect: NSRect,
        flag: BOOL,
        event: NSEvent,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dragFile : filename, fromRect : rect, slideBack : flag, event : event)
    }
    unsafe fn dragPromisedFilesOfTypes_fromRect_source_slideBack_event_(
        &self,
        typeArray: NSArray,
        rect: NSRect,
        sourceObject: id,
        flag: BOOL,
        event: NSEvent,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dragPromisedFilesOfTypes : typeArray, fromRect : rect, source : sourceObject, slideBack : flag, event : event)
    }
    unsafe fn convertPointToBase_(&self, point: NSPoint) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPointToBase : point)
    }
    unsafe fn convertPointFromBase_(&self, point: NSPoint) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPointFromBase : point)
    }
    unsafe fn convertSizeToBase_(&self, size: NSSize) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertSizeToBase : size)
    }
    unsafe fn convertSizeFromBase_(&self, size: NSSize) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertSizeFromBase : size)
    }
    unsafe fn convertRectToBase_(&self, rect: NSRect) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRectToBase : rect)
    }
    unsafe fn convertRectFromBase_(&self, rect: NSRect) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRectFromBase : rect)
    }
    unsafe fn performMnemonic_(&self, string: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performMnemonic : string)
    }
    unsafe fn shouldDrawColor(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldDrawColor)
    }
    unsafe fn gState(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gState)
    }
    unsafe fn allocateGState(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allocateGState)
    }
    unsafe fn releaseGState(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseGState)
    }
    unsafe fn setUpGState(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setUpGState)
    }
    unsafe fn renewGState(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renewGState)
    }
}
pub trait NSView_NSWritingToolsCoordinator: Sized + std::ops::Deref {
    unsafe fn writingToolsCoordinator(&self) -> NSWritingToolsCoordinator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writingToolsCoordinator)
    }
    unsafe fn setWritingToolsCoordinator_(&self, writingToolsCoordinator: NSWritingToolsCoordinator)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWritingToolsCoordinator : writingToolsCoordinator)
    }
}
pub trait NSTextTab_: Sized + std::ops::Deref {
    unsafe fn initWithTextAlignment_location_options_(
        &self,
        alignment: NSTextAlignment,
        loc: CGFloat,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextAlignment : alignment, location : loc, options : options)
    }
    unsafe fn alignment(&self) -> NSTextAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignment)
    }
}
pub trait NSParagraphStyle_: Sized + std::ops::Deref {
    unsafe fn alignment(&self) -> NSTextAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignment)
    }
    unsafe fn tighteningFactorForTruncation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tighteningFactorForTruncation)
    }
    unsafe fn textBlocks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textBlocks)
    }
    unsafe fn headerLevel(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerLevel)
    }
}
pub trait NSMutableParagraphStyle_: Sized + std::ops::Deref {
    unsafe fn alignment(&self) -> NSTextAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignment)
    }
    unsafe fn setAlignment_(&self, alignment: NSTextAlignment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlignment : alignment)
    }
    unsafe fn tighteningFactorForTruncation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tighteningFactorForTruncation)
    }
    unsafe fn setTighteningFactorForTruncation_(&self, tighteningFactorForTruncation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTighteningFactorForTruncation : tighteningFactorForTruncation)
    }
    unsafe fn textBlocks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textBlocks)
    }
    unsafe fn setTextBlocks_(&self, textBlocks: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextBlocks : textBlocks)
    }
    unsafe fn headerLevel(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerLevel)
    }
    unsafe fn setHeaderLevel_(&self, headerLevel: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderLevel : headerLevel)
    }
}
pub trait NSTextTab_NSTextTabDeprecated: Sized + std::ops::Deref {
    unsafe fn initWithType_location_(&self, type_: NSTextTabType, loc: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : type_, location : loc)
    }
    unsafe fn tabStopType(&self) -> NSTextTabType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabStopType)
    }
}
pub type NSControlStateValue = NSInteger;
pub trait NSCell_NSKeyboardUI: Sized + std::ops::Deref {
    unsafe fn performClick_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performClick : sender)
    }
    unsafe fn drawFocusRingMaskWithFrame_inView_(&self, cellFrame: NSRect, controlView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawFocusRingMaskWithFrame : cellFrame, inView : controlView)
    }
    unsafe fn focusRingMaskBoundsForFrame_inView_(
        &self,
        cellFrame: NSRect,
        controlView: NSView,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, focusRingMaskBoundsForFrame : cellFrame, inView : controlView)
    }
    unsafe fn refusesFirstResponder(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refusesFirstResponder)
    }
    unsafe fn setRefusesFirstResponder_(&self, refusesFirstResponder: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRefusesFirstResponder : refusesFirstResponder)
    }
    unsafe fn acceptsFirstResponder(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptsFirstResponder)
    }
    unsafe fn showsFirstResponder(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsFirstResponder)
    }
    unsafe fn setShowsFirstResponder_(&self, showsFirstResponder: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsFirstResponder : showsFirstResponder)
    }
    unsafe fn focusRingType(&self) -> NSFocusRingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusRingType)
    }
    unsafe fn setFocusRingType_(&self, focusRingType: NSFocusRingType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusRingType : focusRingType)
    }
    unsafe fn wantsNotificationForMarkedText(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsNotificationForMarkedText)
    }
    unsafe fn defaultFocusRingType() -> NSFocusRingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCell").unwrap(), defaultFocusRingType)
    }
}
pub trait NSCell_NSCellAttributedStringMethods: Sized + std::ops::Deref {
    unsafe fn attributedStringValue(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedStringValue)
    }
    unsafe fn setAttributedStringValue_(&self, attributedStringValue: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedStringValue : attributedStringValue)
    }
    unsafe fn allowsEditingTextAttributes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsEditingTextAttributes)
    }
    unsafe fn setAllowsEditingTextAttributes_(&self, allowsEditingTextAttributes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsEditingTextAttributes : allowsEditingTextAttributes)
    }
    unsafe fn importsGraphics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, importsGraphics)
    }
    unsafe fn setImportsGraphics_(&self, importsGraphics: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImportsGraphics : importsGraphics)
    }
}
pub trait NSCell_NSCellMixedState: Sized + std::ops::Deref {
    unsafe fn setNextState(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setNextState)
    }
    unsafe fn allowsMixedState(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsMixedState)
    }
    unsafe fn setAllowsMixedState_(&self, allowsMixedState: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsMixedState : allowsMixedState)
    }
    unsafe fn nextState(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextState)
    }
}
pub trait NSCell_NSCellHitTest: Sized + std::ops::Deref {
    unsafe fn hitTestForEvent_inRect_ofView_(
        &self,
        event: NSEvent,
        cellFrame: NSRect,
        controlView: NSView,
    ) -> NSCellHitResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hitTestForEvent : event, inRect : cellFrame, ofView : controlView)
    }
}
pub trait NSCell_NSCellExpansion: Sized + std::ops::Deref {
    unsafe fn expansionFrameWithFrame_inView_(&self, cellFrame: NSRect, view: NSView) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, expansionFrameWithFrame : cellFrame, inView : view)
    }
    unsafe fn drawWithExpansionFrame_inView_(&self, cellFrame: NSRect, view: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawWithExpansionFrame : cellFrame, inView : view)
    }
}
pub trait NSCell_NSCellBackgroundStyle: Sized + std::ops::Deref {
    unsafe fn backgroundStyle(&self) -> NSBackgroundStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundStyle)
    }
    unsafe fn setBackgroundStyle_(&self, backgroundStyle: NSBackgroundStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundStyle : backgroundStyle)
    }
    unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interiorBackgroundStyle)
    }
}
pub trait NSCell_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn entryType(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entryType)
    }
    unsafe fn setEntryType_(&self, type_: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntryType : type_)
    }
    unsafe fn isEntryAcceptable_(&self, string: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEntryAcceptable : string)
    }
    unsafe fn setFloatingPointFormat_left_right_(
        &self,
        autoRange: BOOL,
        leftDigits: NSUInteger,
        rightDigits: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatingPointFormat : autoRange, left : leftDigits, right : rightDigits)
    }
    unsafe fn setMnemonicLocation_(&self, location: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMnemonicLocation : location)
    }
    unsafe fn mnemonicLocation(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mnemonicLocation)
    }
    unsafe fn mnemonic(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mnemonic)
    }
    unsafe fn setTitleWithMnemonic_(&self, stringWithAmpersand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleWithMnemonic : stringWithAmpersand)
    }
    unsafe fn controlTint(&self) -> NSControlTint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlTint)
    }
    unsafe fn setControlTint_(&self, controlTint: NSControlTint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlTint : controlTint)
    }
}
pub trait NSView_NSViewEnclosingMenuItem: Sized + std::ops::Deref {
    unsafe fn enclosingMenuItem(&self) -> NSMenuItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enclosingMenuItem)
    }
}
pub trait NSMenuItem_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setMnemonicLocation_(&self, location: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMnemonicLocation : location)
    }
    unsafe fn mnemonicLocation(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mnemonicLocation)
    }
    unsafe fn mnemonic(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mnemonic)
    }
    unsafe fn setTitleWithMnemonic_(&self, stringWithAmpersand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleWithMnemonic : stringWithAmpersand)
    }
}
pub trait NSMenu_NSPaletteMenus: Sized + std::ops::Deref {
    unsafe fn presentationStyle(&self) -> NSMenuPresentationStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationStyle)
    }
    unsafe fn setPresentationStyle_(&self, presentationStyle: NSMenuPresentationStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresentationStyle : presentationStyle)
    }
    unsafe fn selectionMode(&self) -> NSMenuSelectionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionMode)
    }
    unsafe fn setSelectionMode_(&self, selectionMode: NSMenuSelectionMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionMode : selectionMode)
    }
    unsafe fn selectedItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedItems)
    }
    unsafe fn setSelectedItems_(&self, selectedItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedItems : selectedItems)
    }
    unsafe fn paletteMenuWithColors_titles_selectionHandler_(
        colors: NSArray,
        itemTitles: NSArray,
        onSelectionChange: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMenu").unwrap(), paletteMenuWithColors : colors, titles : itemTitles, selectionHandler : onSelectionChange)
    }
    unsafe fn paletteMenuWithColors_titles_templateImage_selectionHandler_(
        colors: NSArray,
        itemTitles: NSArray,
        image: NSImage,
        onSelectionChange: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMenu").unwrap(), paletteMenuWithColors : colors, titles : itemTitles, templateImage : image, selectionHandler : onSelectionChange)
    }
}
pub trait NSMenu_NSSubmenuAction: Sized + std::ops::Deref {
    unsafe fn submenuAction_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, submenuAction : sender)
    }
}
pub trait NSObject_NSMenuValidation: Sized + std::ops::Deref {
    unsafe fn validateMenuItem_(&self, menuItem: NSMenuItem) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateMenuItem : menuItem)
    }
}
pub trait NSMenu_NSMenuPropertiesToUpdate: Sized + std::ops::Deref {
    unsafe fn propertiesToUpdate(&self) -> NSMenuProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertiesToUpdate)
    }
}
pub trait NSMenu_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setMenuRepresentation_(&self, menuRep: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMenuRepresentation : menuRep)
    }
    unsafe fn menuRepresentation(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menuRepresentation)
    }
    unsafe fn setContextMenuRepresentation_(&self, menuRep: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContextMenuRepresentation : menuRep)
    }
    unsafe fn contextMenuRepresentation(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contextMenuRepresentation)
    }
    unsafe fn setTearOffMenuRepresentation_(&self, menuRep: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTearOffMenuRepresentation : menuRep)
    }
    unsafe fn tearOffMenuRepresentation(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tearOffMenuRepresentation)
    }
    unsafe fn attachedMenu(&self) -> NSMenu
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachedMenu)
    }
    unsafe fn isAttached(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAttached)
    }
    unsafe fn sizeToFit(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeToFit)
    }
    unsafe fn locationForSubmenu_(&self, submenu: NSMenu) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationForSubmenu : submenu)
    }
    unsafe fn helpRequested_(&self, eventPtr: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, helpRequested : eventPtr)
    }
    unsafe fn menuChangedMessagesEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menuChangedMessagesEnabled)
    }
    unsafe fn setMenuChangedMessagesEnabled_(&self, menuChangedMessagesEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMenuChangedMessagesEnabled : menuChangedMessagesEnabled)
    }
    unsafe fn isTornOff(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTornOff)
    }
    unsafe fn menuZone() -> *mut NSZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMenu").unwrap(), menuZone)
    }
    unsafe fn setMenuZone_(zone: *mut NSZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMenu").unwrap(), setMenuZone : zone)
    }
}
pub type NSPrinterTypeName = NSString;
pub type NSPrinterPaperName = NSString;
pub trait NSPrinter_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn statusForTable_(&self, tableName: NSString) -> NSPrinterTableStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, statusForTable : tableName)
    }
    unsafe fn isKey_inTable_(&self, key: NSString, table: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isKey : key, inTable : table)
    }
    unsafe fn booleanForKey_inTable_(&self, key: NSString, table: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, booleanForKey : key, inTable : table)
    }
    unsafe fn floatForKey_inTable_(&self, key: NSString, table: NSString) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, floatForKey : key, inTable : table)
    }
    unsafe fn intForKey_inTable_(&self, key: NSString, table: NSString) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intForKey : key, inTable : table)
    }
    unsafe fn rectForKey_inTable_(&self, key: NSString, table: NSString) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectForKey : key, inTable : table)
    }
    unsafe fn sizeForKey_inTable_(&self, key: NSString, table: NSString) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sizeForKey : key, inTable : table)
    }
    unsafe fn stringForKey_inTable_(&self, key: NSString, table: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringForKey : key, inTable : table)
    }
    unsafe fn stringListForKey_inTable_(&self, key: NSString, table: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringListForKey : key, inTable : table)
    }
    unsafe fn imageRectForPaper_(&self, paperName: NSString) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageRectForPaper : paperName)
    }
    unsafe fn acceptsBinary(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptsBinary)
    }
    unsafe fn isColor(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isColor)
    }
    unsafe fn isFontAvailable_(&self, faceName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isFontAvailable : faceName)
    }
    unsafe fn isOutputStackInReverseOrder(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOutputStackInReverseOrder)
    }
    unsafe fn domain(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domain)
    }
    unsafe fn host(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, host)
    }
    unsafe fn note(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, note)
    }
    unsafe fn printerWithName_domain_includeUnavailable_(
        name: NSString,
        domain: NSString,
        flag: BOOL,
    ) -> NSPrinter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPrinter").unwrap(), printerWithName : name, domain : domain, includeUnavailable : flag)
    }
}
pub type NSPrintJobDispositionValue = NSString;
pub trait NSPrintInfo_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setDefaultPrinter_(printer: NSPrinter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPrintInfo").unwrap(), setDefaultPrinter : printer)
    }
    unsafe fn sizeForPaperName_(name: NSString) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSPrintInfo").unwrap(), sizeForPaperName : name)
    }
}
pub trait NSObject_NSKeyValueBindingCreation: Sized + std::ops::Deref {
    unsafe fn valueClassForBinding_(&self, binding: NSString) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueClassForBinding : binding)
    }
    unsafe fn bind_toObject_withKeyPath_options_(
        &self,
        binding: NSString,
        observable: id,
        keyPath: NSString,
        options: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bind : binding, toObject : observable, withKeyPath : keyPath, options : options)
    }
    unsafe fn unbind_(&self, binding: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unbind : binding)
    }
    unsafe fn infoForBinding_(&self, binding: NSString) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, infoForBinding : binding)
    }
    unsafe fn optionDescriptionsForBinding_(&self, binding: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, optionDescriptionsForBinding : binding)
    }
    unsafe fn exposedBindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposedBindings)
    }
    unsafe fn exposeBinding_(binding: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), exposeBinding : binding)
    }
}
pub trait NSObject_NSPlaceholders: Sized + std::ops::Deref {
    unsafe fn setDefaultPlaceholder_forMarker_withBinding_(
        placeholder: id,
        marker: id,
        binding: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), setDefaultPlaceholder : placeholder, forMarker : marker, withBinding : binding)
    }
    unsafe fn defaultPlaceholderForMarker_withBinding_(marker: id, binding: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), defaultPlaceholderForMarker : marker, withBinding : binding)
    }
}
pub trait NSObject_NSEditor: Sized + std::ops::Deref {
    unsafe fn discardEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discardEditing)
    }
    unsafe fn commitEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commitEditing)
    }
    unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo_(
        &self,
        delegate: id,
        didCommitSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingWithDelegate : delegate, didCommitSelector : didCommitSelector, contextInfo : contextInfo)
    }
    unsafe fn commitEditingAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingAndReturnError : error)
    }
}
pub trait NSObject_NSEditorRegistration: Sized + std::ops::Deref {
    unsafe fn objectDidBeginEditing_(&self, editor: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectDidBeginEditing : editor)
    }
    unsafe fn objectDidEndEditing_(&self, editor: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectDidEndEditing : editor)
    }
}
pub trait NSManagedObjectContext_NSEditorAndEditorRegistrationConformance:
    Sized + std::ops::Deref
{
}
pub trait NSDocument_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn saveToURL_ofType_forSaveOperation_error_(
        &self,
        url: NSURL,
        typeName: NSString,
        saveOperation: NSSaveOperationType,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveToURL : url, ofType : typeName, forSaveOperation : saveOperation, error : outError)
    }
    unsafe fn dataRepresentationOfType_(&self, type_: NSString) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dataRepresentationOfType : type_)
    }
    unsafe fn fileAttributesToWriteToFile_ofType_saveOperation_(
        &self,
        fullDocumentPath: NSString,
        documentTypeName: NSString,
        saveOperationType: NSSaveOperationType,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileAttributesToWriteToFile : fullDocumentPath, ofType : documentTypeName, saveOperation : saveOperationType)
    }
    unsafe fn fileName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileName)
    }
    unsafe fn fileWrapperRepresentationOfType_(&self, type_: NSString) -> NSFileWrapper
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileWrapperRepresentationOfType : type_)
    }
    unsafe fn initWithContentsOfFile_ofType_(
        &self,
        absolutePath: NSString,
        typeName: NSString,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfFile : absolutePath, ofType : typeName)
    }
    unsafe fn initWithContentsOfURL_ofType_(&self, url: NSURL, typeName: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url, ofType : typeName)
    }
    unsafe fn loadDataRepresentation_ofType_(&self, data: NSData, type_: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadDataRepresentation : data, ofType : type_)
    }
    unsafe fn loadFileWrapperRepresentation_ofType_(
        &self,
        wrapper: NSFileWrapper,
        type_: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFileWrapperRepresentation : wrapper, ofType : type_)
    }
    unsafe fn printShowingPrintPanel_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, printShowingPrintPanel : flag)
    }
    unsafe fn readFromFile_ofType_(&self, fileName: NSString, type_: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readFromFile : fileName, ofType : type_)
    }
    unsafe fn readFromURL_ofType_(&self, url: NSURL, type_: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readFromURL : url, ofType : type_)
    }
    unsafe fn revertToSavedFromFile_ofType_(&self, fileName: NSString, type_: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, revertToSavedFromFile : fileName, ofType : type_)
    }
    unsafe fn revertToSavedFromURL_ofType_(&self, url: NSURL, type_: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, revertToSavedFromURL : url, ofType : type_)
    }
    unsafe fn runModalPageLayoutWithPrintInfo_(&self, printInfo: NSPrintInfo) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalPageLayoutWithPrintInfo : printInfo)
    }
    unsafe fn saveToFile_saveOperation_delegate_didSaveSelector_contextInfo_(
        &self,
        fileName: NSString,
        saveOperation: NSSaveOperationType,
        delegate: id,
        didSaveSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveToFile : fileName, saveOperation : saveOperation, delegate : delegate, didSaveSelector : didSaveSelector, contextInfo : contextInfo)
    }
    unsafe fn setFileName_(&self, fileName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFileName : fileName)
    }
    unsafe fn writeToFile_ofType_(&self, fileName: NSString, type_: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToFile : fileName, ofType : type_)
    }
    unsafe fn writeToFile_ofType_originalFile_saveOperation_(
        &self,
        fullDocumentPath: NSString,
        documentTypeName: NSString,
        fullOriginalDocumentPath: NSString,
        saveOperationType: NSSaveOperationType,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToFile : fullDocumentPath, ofType : documentTypeName, originalFile : fullOriginalDocumentPath, saveOperation : saveOperationType)
    }
    unsafe fn writeToURL_ofType_(&self, url: NSURL, type_: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : url, ofType : type_)
    }
    unsafe fn writeWithBackupToFile_ofType_saveOperation_(
        &self,
        fullDocumentPath: NSString,
        documentTypeName: NSString,
        saveOperationType: NSSaveOperationType,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeWithBackupToFile : fullDocumentPath, ofType : documentTypeName, saveOperation : saveOperationType)
    }
    unsafe fn shouldRunSavePanelWithAccessoryView(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldRunSavePanelWithAccessoryView)
    }
}
pub trait NSResponder_NSUserActivity: Sized + std::ops::Deref {
    unsafe fn updateUserActivityState_(&self, userActivity: NSUserActivity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateUserActivityState : userActivity)
    }
    unsafe fn userActivity(&self) -> NSUserActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userActivity)
    }
    unsafe fn setUserActivity_(&self, userActivity: NSUserActivity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserActivity : userActivity)
    }
}
pub trait NSDocument_NSUserActivity: Sized + std::ops::Deref {
    unsafe fn updateUserActivityState_(&self, activity: NSUserActivity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateUserActivityState : activity)
    }
    unsafe fn userActivity(&self) -> NSUserActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userActivity)
    }
    unsafe fn setUserActivity_(&self, userActivity: NSUserActivity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserActivity : userActivity)
    }
}
pub type NSModalResponse = NSInteger;
pub trait NSApplication_NSAppearanceCustomization: Sized + std::ops::Deref {
    unsafe fn appearance(&self) -> NSAppearance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appearance)
    }
    unsafe fn setAppearance_(&self, appearance: NSAppearance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppearance : appearance)
    }
    unsafe fn effectiveAppearance(&self) -> NSAppearance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, effectiveAppearance)
    }
}
pub trait NSApplication_NSEvent: Sized + std::ops::Deref {
    unsafe fn sendEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendEvent : event)
    }
    unsafe fn postEvent_atStart_(&self, event: NSEvent, atStart: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, postEvent : event, atStart : atStart)
    }
    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(
        &self,
        mask: NSEventMask,
        expiration: NSDate,
        mode: NSString,
        deqFlag: BOOL,
    ) -> NSEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nextEventMatchingMask : mask, untilDate : expiration, inMode : mode, dequeue : deqFlag)
    }
    unsafe fn discardEventsMatchingMask_beforeEvent_(&self, mask: NSEventMask, lastEvent: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discardEventsMatchingMask : mask, beforeEvent : lastEvent)
    }
    unsafe fn currentEvent(&self) -> NSEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentEvent)
    }
}
pub trait NSApplication_NSResponder: Sized + std::ops::Deref {
    unsafe fn sendAction_to_from_(&self, action: objc2::runtime::Sel, target: id, sender: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendAction : action, to : target, from : sender)
    }
    unsafe fn targetForAction_(&self, action: objc2::runtime::Sel) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, targetForAction : action)
    }
    unsafe fn targetForAction_to_from_(
        &self,
        action: objc2::runtime::Sel,
        target: id,
        sender: id,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, targetForAction : action, to : target, from : sender)
    }
    unsafe fn tryToPerform_with_(&self, action: objc2::runtime::Sel, object: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tryToPerform : action, with : object)
    }
    unsafe fn validRequestorForSendType_returnType_(
        &self,
        sendType: NSString,
        returnType: NSString,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validRequestorForSendType : sendType, returnType : returnType)
    }
}
pub trait NSApplication_NSWindowsMenu: Sized + std::ops::Deref {
    unsafe fn arrangeInFront_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, arrangeInFront : sender)
    }
    unsafe fn removeWindowsItem_(&self, win: NSWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeWindowsItem : win)
    }
    unsafe fn addWindowsItem_title_filename_(
        &self,
        win: NSWindow,
        string: NSString,
        isFilename: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addWindowsItem : win, title : string, filename : isFilename)
    }
    unsafe fn changeWindowsItem_title_filename_(
        &self,
        win: NSWindow,
        string: NSString,
        isFilename: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeWindowsItem : win, title : string, filename : isFilename)
    }
    unsafe fn updateWindowsItem_(&self, win: NSWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWindowsItem : win)
    }
    unsafe fn miniaturizeAll_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, miniaturizeAll : sender)
    }
    unsafe fn windowsMenu(&self) -> NSMenu
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowsMenu)
    }
    unsafe fn setWindowsMenu_(&self, windowsMenu: NSMenu)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWindowsMenu : windowsMenu)
    }
}
pub trait NSApplication_NSFullKeyboardAccess: Sized + std::ops::Deref {
    unsafe fn isFullKeyboardAccessEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFullKeyboardAccessEnabled)
    }
}
pub trait NSApplication_NSServicesMenu: Sized + std::ops::Deref {
    unsafe fn registerServicesMenuSendTypes_returnTypes_(
        &self,
        sendTypes: NSArray,
        returnTypes: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerServicesMenuSendTypes : sendTypes, returnTypes : returnTypes)
    }
    unsafe fn servicesMenu(&self) -> NSMenu
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, servicesMenu)
    }
    unsafe fn setServicesMenu_(&self, servicesMenu: NSMenu)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServicesMenu : servicesMenu)
    }
}
pub trait NSApplication_NSServicesHandling: Sized + std::ops::Deref {
    unsafe fn servicesProvider(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, servicesProvider)
    }
    unsafe fn setServicesProvider_(&self, servicesProvider: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServicesProvider : servicesProvider)
    }
}
pub trait NSApplication_NSStandardAboutPanel: Sized + std::ops::Deref {
    unsafe fn orderFrontStandardAboutPanel_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, orderFrontStandardAboutPanel : sender)
    }
    unsafe fn orderFrontStandardAboutPanelWithOptions_(&self, optionsDictionary: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, orderFrontStandardAboutPanelWithOptions : optionsDictionary)
    }
}
pub trait NSApplication_NSApplicationLayoutDirection: Sized + std::ops::Deref {
    unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInterfaceLayoutDirection)
    }
}
pub trait NSApplication_NSRestorableUserInterface: Sized + std::ops::Deref {
    unsafe fn disableRelaunchOnLogin(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableRelaunchOnLogin)
    }
    unsafe fn enableRelaunchOnLogin(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableRelaunchOnLogin)
    }
}
pub trait NSApplication_NSRemoteNotifications: Sized + std::ops::Deref {
    unsafe fn registerForRemoteNotifications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registerForRemoteNotifications)
    }
    unsafe fn unregisterForRemoteNotifications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unregisterForRemoteNotifications)
    }
    unsafe fn registerForRemoteNotificationTypes_(&self, types: NSRemoteNotificationType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerForRemoteNotificationTypes : types)
    }
    unsafe fn isRegisteredForRemoteNotifications(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRegisteredForRemoteNotifications)
    }
    unsafe fn enabledRemoteNotificationTypes(&self) -> NSRemoteNotificationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabledRemoteNotificationTypes)
    }
}
pub trait NSApplication_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn runModalForWindow_relativeToWindow_(
        &self,
        window: NSWindow,
        docWindow: NSWindow,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForWindow : window, relativeToWindow : docWindow)
    }
    unsafe fn beginModalSessionForWindow_relativeToWindow_(
        &self,
        window: NSWindow,
        docWindow: NSWindow,
    ) -> NSModalSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginModalSessionForWindow : window, relativeToWindow : docWindow)
    }
    unsafe fn application_printFiles_(&self, sender: NSApplication, filenames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, application : sender, printFiles : filenames)
    }
    unsafe fn beginSheet_modalForWindow_modalDelegate_didEndSelector_contextInfo_(
        &self,
        sheet: NSWindow,
        docWindow: NSWindow,
        modalDelegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheet : sheet, modalForWindow : docWindow, modalDelegate : modalDelegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn endSheet_(&self, sheet: NSWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endSheet : sheet)
    }
    unsafe fn endSheet_returnCode_(&self, sheet: NSWindow, returnCode: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endSheet : sheet, returnCode : returnCode)
    }
    unsafe fn makeWindowsPerform_inOrder_(
        &self,
        selector: objc2::runtime::Sel,
        inOrder: BOOL,
    ) -> NSWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeWindowsPerform : selector, inOrder : inOrder)
    }
    unsafe fn context(&self) -> NSGraphicsContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
}
pub type NSColorListName = NSString;
pub type NSColorName = NSString;
pub trait NSColor_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn colorUsingColorSpaceName_device_(
        &self,
        name: NSString,
        deviceDescription: NSDictionary,
    ) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, colorUsingColorSpaceName : name, device : deviceDescription)
    }
    unsafe fn colorUsingColorSpaceName_(&self, name: NSString) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, colorUsingColorSpaceName : name)
    }
    unsafe fn colorSpaceName(&self) -> NSColorSpaceName
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpaceName)
    }
    unsafe fn controlHighlightColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), controlHighlightColor)
    }
    unsafe fn controlLightHighlightColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), controlLightHighlightColor)
    }
    unsafe fn controlShadowColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), controlShadowColor)
    }
    unsafe fn controlDarkShadowColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), controlDarkShadowColor)
    }
    unsafe fn scrollBarColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), scrollBarColor)
    }
    unsafe fn knobColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), knobColor)
    }
    unsafe fn selectedKnobColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), selectedKnobColor)
    }
    unsafe fn windowFrameColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), windowFrameColor)
    }
    unsafe fn selectedMenuItemColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), selectedMenuItemColor)
    }
    unsafe fn headerColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), headerColor)
    }
    unsafe fn secondarySelectedControlColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), secondarySelectedControlColor)
    }
    unsafe fn alternateSelectedControlColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), alternateSelectedControlColor)
    }
    unsafe fn controlAlternatingRowBackgroundColors() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), controlAlternatingRowBackgroundColors)
    }
}
pub trait NSColor_NSQuartzCoreAdditions: Sized + std::ops::Deref {
    unsafe fn colorWithCIColor_(color: CIColor) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSColor").unwrap(), colorWithCIColor : color)
    }
}
pub trait CIColor_NSAppKitAdditions: Sized + std::ops::Deref {
    unsafe fn initWithColor_(&self, color: NSColor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithColor : color)
    }
}
pub trait NSCoder_NSAppKitColorExtensions: Sized + std::ops::Deref {
    unsafe fn decodeNXColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decodeNXColor)
    }
}
pub trait NSColor_NSAccessibilityColorConformance: Sized + std::ops::Deref {}
pub type NSHelpAnchorName = NSString;
pub trait NSBundle_NSBundleHelpExtension: Sized + std::ops::Deref {
    unsafe fn contextHelpForKey_(&self, key: NSString) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contextHelpForKey : key)
    }
}
pub trait NSApplication_NSApplicationHelpExtension: Sized + std::ops::Deref {
    unsafe fn activateContextHelpMode_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateContextHelpMode : sender)
    }
    unsafe fn showHelp_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showHelp : sender)
    }
}
pub trait NSAlert_NSAlertDeprecated: Sized + std::ops::Deref {
    unsafe fn beginSheetModalForWindow_modalDelegate_didEndSelector_contextInfo_(
        &self,
        window: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetModalForWindow : window, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn alertWithMessageText_defaultButton_alternateButton_otherButton_informativeTextWithFormat_(
        message: NSString,
        defaultButton: NSString,
        alternateButton: NSString,
        otherButton: NSString,
        format: NSString,
    ) -> NSAlert
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAlert").unwrap(), alertWithMessageText : message, defaultButton : defaultButton, alternateButton : alternateButton, otherButton : otherButton, informativeTextWithFormat : format)
    }
}
pub trait NSAppleScript_NSExtensions: Sized + std::ops::Deref {
    unsafe fn richTextSource(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, richTextSource)
    }
}
pub trait NSBox_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setTitleWithMnemonic_(&self, stringWithAmpersand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleWithMnemonic : stringWithAmpersand)
    }
    unsafe fn borderType(&self) -> NSBorderType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderType)
    }
    unsafe fn setBorderType_(&self, borderType: NSBorderType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderType : borderType)
    }
}
pub trait NSControl_NSControlEditableTextMethods: Sized + std::ops::Deref {
    unsafe fn currentEditor(&self) -> NSText
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentEditor)
    }
    unsafe fn abortEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, abortEditing)
    }
    unsafe fn validateEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, validateEditing)
    }
    unsafe fn editWithFrame_editor_delegate_event_(
        &self,
        rect: NSRect,
        textObj: NSText,
        delegate: id,
        event: NSEvent,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, editWithFrame : rect, editor : textObj, delegate : delegate, event : event)
    }
    unsafe fn selectWithFrame_editor_delegate_start_length_(
        &self,
        rect: NSRect,
        textObj: NSText,
        delegate: id,
        selStart: NSInteger,
        selLength: NSInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectWithFrame : rect, editor : textObj, delegate : delegate, start : selStart, length : selLength)
    }
    unsafe fn endEditing_(&self, textObj: NSText)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endEditing : textObj)
    }
}
pub trait NSControl_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setFloatingPointFormat_left_right_(
        &self,
        autoRange: BOOL,
        leftDigits: NSUInteger,
        rightDigits: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatingPointFormat : autoRange, left : leftDigits, right : rightDigits)
    }
    unsafe fn selectedCell(&self) -> NSCell
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedCell)
    }
    unsafe fn selectedTag(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedTag)
    }
    unsafe fn setNeedsDisplay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setNeedsDisplay)
    }
    unsafe fn calcSize(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calcSize)
    }
    unsafe fn updateCell_(&self, cell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateCell : cell)
    }
    unsafe fn updateCellInside_(&self, cell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateCellInside : cell)
    }
    unsafe fn drawCellInside_(&self, cell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawCellInside : cell)
    }
    unsafe fn drawCell_(&self, cell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawCell : cell)
    }
    unsafe fn selectCell_(&self, cell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectCell : cell)
    }
    unsafe fn cell(&self) -> NSCell
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cell)
    }
    unsafe fn setCell_(&self, cell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCell : cell)
    }
    unsafe fn cellClass() -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSControl").unwrap(), cellClass)
    }
    unsafe fn setCellClass_(cellClass: Class)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSControl").unwrap(), setCellClass : cellClass)
    }
}
pub trait NSObject_NSControlSubclassNotifications: Sized + std::ops::Deref {
    unsafe fn controlTextDidBeginEditing_(&self, obj: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controlTextDidBeginEditing : obj)
    }
    unsafe fn controlTextDidEndEditing_(&self, obj: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controlTextDidEndEditing : obj)
    }
    unsafe fn controlTextDidChange_(&self, obj: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controlTextDidChange : obj)
    }
}
pub trait NSButtonCell_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setTitleWithMnemonic_(&self, stringWithAmpersand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleWithMnemonic : stringWithAmpersand)
    }
    unsafe fn setAlternateTitleWithMnemonic_(&self, stringWithAmpersand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlternateTitleWithMnemonic : stringWithAmpersand)
    }
    unsafe fn setAlternateMnemonicLocation_(&self, location: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlternateMnemonicLocation : location)
    }
    unsafe fn alternateMnemonicLocation(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternateMnemonicLocation)
    }
    unsafe fn alternateMnemonic(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternateMnemonic)
    }
    unsafe fn setKeyEquivalentFont_size_(&self, fontName: NSString, fontSize: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyEquivalentFont : fontName, size : fontSize)
    }
    unsafe fn gradientType(&self) -> NSGradientType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientType)
    }
    unsafe fn setGradientType_(&self, gradientType: NSGradientType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGradientType : gradientType)
    }
    unsafe fn keyEquivalentFont(&self) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyEquivalentFont)
    }
    unsafe fn setKeyEquivalentFont_(&self, keyEquivalentFont: NSFont)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyEquivalentFont : keyEquivalentFont)
    }
}
pub trait NSButton_NSButtonDeprecated: Sized + std::ops::Deref {
    unsafe fn setTitleWithMnemonic_(&self, stringWithAmpersand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleWithMnemonic : stringWithAmpersand)
    }
}
pub type NSTouchBarItemIdentifier = NSString;
pub type NSTouchBarItemPriority = f32;
pub type NSTouchBarCustomizationIdentifier = NSString;
pub trait NSResponder_NSTouchBarProvider: Sized + std::ops::Deref {
    unsafe fn makeTouchBar(&self) -> NSTouchBar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, makeTouchBar)
    }
    unsafe fn touchBar(&self) -> NSTouchBar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, touchBar)
    }
    unsafe fn setTouchBar_(&self, touchBar: NSTouchBar)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTouchBar : touchBar)
    }
}
pub trait NSApplication_NSTouchBarCustomization: Sized + std::ops::Deref {
    unsafe fn toggleTouchBarCustomizationPalette_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleTouchBarCustomizationPalette : sender)
    }
    unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticCustomizeTouchBarMenuItemEnabled)
    }
    unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled_(
        &self,
        automaticCustomizeTouchBarMenuItemEnabled: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticCustomizeTouchBarMenuItemEnabled : automaticCustomizeTouchBarMenuItemEnabled)
    }
}
pub trait NSView_NSCandidateListTouchBarItem: Sized + std::ops::Deref {
    unsafe fn candidateListTouchBarItem(&self) -> NSCandidateListTouchBarItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, candidateListTouchBarItem)
    }
}
pub trait NSView_NSClipViewSuperview: Sized + std::ops::Deref {
    unsafe fn reflectScrolledClipView_(&self, clipView: NSClipView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reflectScrolledClipView : clipView)
    }
    unsafe fn scrollClipView_toPoint_(&self, clipView: NSClipView, point: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollClipView : clipView, toPoint : point)
    }
}
pub trait NSClipView_: Sized + std::ops::Deref {
    unsafe fn constrainScrollPoint_(&self, newOrigin: NSPoint) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constrainScrollPoint : newOrigin)
    }
    unsafe fn copiesOnScroll(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, copiesOnScroll)
    }
    unsafe fn setCopiesOnScroll_(&self, copiesOnScroll: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCopiesOnScroll : copiesOnScroll)
    }
}
pub type NSStoryboardControllerCreator = *mut ::std::os::raw::c_void;
pub type NSStoryboardSegueIdentifier = NSString;
pub trait NSViewController_NSViewControllerPresentation: Sized + std::ops::Deref {
    unsafe fn presentViewController_animator_(
        &self,
        viewController: NSViewController,
        animator: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentViewController : viewController, animator : animator)
    }
    unsafe fn dismissViewController_(&self, viewController: NSViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismissViewController : viewController)
    }
    unsafe fn dismissController_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismissController : sender)
    }
    unsafe fn presentedViewControllers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentedViewControllers)
    }
    unsafe fn presentingViewController(&self) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentingViewController)
    }
}
pub trait NSViewController_NSViewControllerPresentationAndTransitionStyles:
    Sized + std::ops::Deref
{
    unsafe fn presentViewControllerAsSheet_(&self, viewController: NSViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentViewControllerAsSheet : viewController)
    }
    unsafe fn presentViewControllerAsModalWindow_(&self, viewController: NSViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentViewControllerAsModalWindow : viewController)
    }
    unsafe fn presentViewController_asPopoverRelativeToRect_ofView_preferredEdge_behavior_(
        &self,
        viewController: NSViewController,
        positioningRect: NSRect,
        positioningView: NSView,
        preferredEdge: NSRectEdge,
        behavior: NSPopoverBehavior,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentViewController : viewController, asPopoverRelativeToRect : positioningRect, ofView : positioningView, preferredEdge : preferredEdge, behavior : behavior)
    }
    unsafe fn presentViewController_asPopoverRelativeToRect_ofView_preferredEdge_behavior_hasFullSizeContent_(
        &self,
        viewController: NSViewController,
        positioningRect: NSRect,
        positioningView: NSView,
        preferredEdge: NSRectEdge,
        behavior: NSPopoverBehavior,
        hasFullSizeContent: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentViewController : viewController, asPopoverRelativeToRect : positioningRect, ofView : positioningView, preferredEdge : preferredEdge, behavior : behavior, hasFullSizeContent : hasFullSizeContent)
    }
    unsafe fn transitionFromViewController_toViewController_options_completionHandler_(
        &self,
        fromViewController: NSViewController,
        toViewController: NSViewController,
        options: NSViewControllerTransitionOptions,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transitionFromViewController : fromViewController, toViewController : toViewController, options : options, completionHandler : completion)
    }
}
pub trait NSViewController_NSViewControllerContainer: Sized + std::ops::Deref {
    unsafe fn addChildViewController_(&self, childViewController: NSViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addChildViewController : childViewController)
    }
    unsafe fn removeFromParentViewController(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeFromParentViewController)
    }
    unsafe fn insertChildViewController_atIndex_(
        &self,
        childViewController: NSViewController,
        index: NSInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertChildViewController : childViewController, atIndex : index)
    }
    unsafe fn removeChildViewControllerAtIndex_(&self, index: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeChildViewControllerAtIndex : index)
    }
    unsafe fn preferredContentSizeDidChangeForViewController_(
        &self,
        viewController: NSViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preferredContentSizeDidChangeForViewController : viewController)
    }
    unsafe fn viewWillTransitionToSize_(&self, newSize: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, viewWillTransitionToSize : newSize)
    }
    unsafe fn parentViewController(&self) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentViewController)
    }
    unsafe fn childViewControllers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childViewControllers)
    }
    unsafe fn setChildViewControllers_(&self, childViewControllers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChildViewControllers : childViewControllers)
    }
}
pub trait NSViewController_NSViewControllerStoryboardingMethods: Sized + std::ops::Deref {
    unsafe fn storyboard(&self) -> NSStoryboard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storyboard)
    }
}
pub trait NSViewController_NSExtensionAdditions: Sized + std::ops::Deref {
    unsafe fn extensionContext(&self) -> NSExtensionContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionContext)
    }
    unsafe fn sourceItemView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceItemView)
    }
    unsafe fn setSourceItemView_(&self, sourceItemView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceItemView : sourceItemView)
    }
    unsafe fn preferredScreenOrigin(&self) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredScreenOrigin)
    }
    unsafe fn setPreferredScreenOrigin_(&self, preferredScreenOrigin: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredScreenOrigin : preferredScreenOrigin)
    }
    unsafe fn preferredMinimumSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredMinimumSize)
    }
    unsafe fn preferredMaximumSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredMaximumSize)
    }
}
pub trait NSIndexPath_NSCollectionViewAdditions: Sized + std::ops::Deref {
    unsafe fn item(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, item)
    }
    unsafe fn section(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, section)
    }
    unsafe fn indexPathForItem_inSection_(item: NSInteger, section: NSInteger) -> NSIndexPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSIndexPath").unwrap(), indexPathForItem : item, inSection : section)
    }
}
pub trait NSSet_NSCollectionViewAdditions: Sized + std::ops::Deref {
    unsafe fn enumerateIndexPathsWithOptions_usingBlock_(
        &self,
        opts: NSEnumerationOptions,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateIndexPathsWithOptions : opts, usingBlock : block)
    }
    unsafe fn setWithCollectionViewIndexPath_(indexPath: NSIndexPath) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSet").unwrap(), setWithCollectionViewIndexPath : indexPath)
    }
    unsafe fn setWithCollectionViewIndexPaths_(indexPaths: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSet").unwrap(), setWithCollectionViewIndexPaths : indexPaths)
    }
}
pub trait NSCollectionView_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn newItemForRepresentedObject_(&self, object: id) -> NSCollectionViewItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newItemForRepresentedObject : object)
    }
    unsafe fn itemPrototype(&self) -> NSCollectionViewItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemPrototype)
    }
    unsafe fn setItemPrototype_(&self, itemPrototype: NSCollectionViewItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setItemPrototype : itemPrototype)
    }
    unsafe fn maxNumberOfRows(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxNumberOfRows)
    }
    unsafe fn setMaxNumberOfRows_(&self, maxNumberOfRows: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxNumberOfRows : maxNumberOfRows)
    }
    unsafe fn maxNumberOfColumns(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxNumberOfColumns)
    }
    unsafe fn setMaxNumberOfColumns_(&self, maxNumberOfColumns: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxNumberOfColumns : maxNumberOfColumns)
    }
    unsafe fn minItemSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minItemSize)
    }
    unsafe fn setMinItemSize_(&self, minItemSize: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinItemSize : minItemSize)
    }
    unsafe fn maxItemSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxItemSize)
    }
    unsafe fn setMaxItemSize_(&self, maxItemSize: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxItemSize : maxItemSize)
    }
}
pub trait NSCollectionViewLayout_NSSubclassingHooks: Sized + std::ops::Deref {
    unsafe fn prepareLayout(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareLayout)
    }
    unsafe fn layoutAttributesForElementsInRect_(&self, rect: NSRect) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutAttributesForElementsInRect : rect)
    }
    unsafe fn layoutAttributesForItemAtIndexPath_(
        &self,
        indexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutAttributesForItemAtIndexPath : indexPath)
    }
    unsafe fn layoutAttributesForSupplementaryViewOfKind_atIndexPath_(
        &self,
        elementKind: NSString,
        indexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutAttributesForSupplementaryViewOfKind : elementKind, atIndexPath : indexPath)
    }
    unsafe fn layoutAttributesForDecorationViewOfKind_atIndexPath_(
        &self,
        elementKind: NSString,
        indexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutAttributesForDecorationViewOfKind : elementKind, atIndexPath : indexPath)
    }
    unsafe fn layoutAttributesForDropTargetAtPoint_(
        &self,
        pointInCollectionView: NSPoint,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutAttributesForDropTargetAtPoint : pointInCollectionView)
    }
    unsafe fn layoutAttributesForInterItemGapBeforeIndexPath_(
        &self,
        indexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutAttributesForInterItemGapBeforeIndexPath : indexPath)
    }
    unsafe fn shouldInvalidateLayoutForBoundsChange_(&self, newBounds: NSRect) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldInvalidateLayoutForBoundsChange : newBounds)
    }
    unsafe fn invalidationContextForBoundsChange_(
        &self,
        newBounds: NSRect,
    ) -> NSCollectionViewLayoutInvalidationContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidationContextForBoundsChange : newBounds)
    }
    unsafe fn shouldInvalidateLayoutForPreferredLayoutAttributes_withOriginalAttributes_(
        &self,
        preferredAttributes: NSCollectionViewLayoutAttributes,
        originalAttributes: NSCollectionViewLayoutAttributes,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldInvalidateLayoutForPreferredLayoutAttributes : preferredAttributes, withOriginalAttributes : originalAttributes)
    }
    unsafe fn invalidationContextForPreferredLayoutAttributes_withOriginalAttributes_(
        &self,
        preferredAttributes: NSCollectionViewLayoutAttributes,
        originalAttributes: NSCollectionViewLayoutAttributes,
    ) -> NSCollectionViewLayoutInvalidationContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidationContextForPreferredLayoutAttributes : preferredAttributes, withOriginalAttributes : originalAttributes)
    }
    unsafe fn targetContentOffsetForProposedContentOffset_withScrollingVelocity_(
        &self,
        proposedContentOffset: NSPoint,
        velocity: NSPoint,
    ) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, targetContentOffsetForProposedContentOffset : proposedContentOffset, withScrollingVelocity : velocity)
    }
    unsafe fn targetContentOffsetForProposedContentOffset_(
        &self,
        proposedContentOffset: NSPoint,
    ) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, targetContentOffsetForProposedContentOffset : proposedContentOffset)
    }
    unsafe fn collectionViewContentSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collectionViewContentSize)
    }
    unsafe fn layoutAttributesClass() -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCollectionViewLayout").unwrap(), layoutAttributesClass)
    }
    unsafe fn invalidationContextClass() -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCollectionViewLayout").unwrap(), invalidationContextClass)
    }
}
pub trait NSCollectionViewLayout_NSUpdateSupportHooks: Sized + std::ops::Deref {
    unsafe fn prepareForCollectionViewUpdates_(&self, updateItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareForCollectionViewUpdates : updateItems)
    }
    unsafe fn finalizeCollectionViewUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finalizeCollectionViewUpdates)
    }
    unsafe fn prepareForAnimatedBoundsChange_(&self, oldBounds: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareForAnimatedBoundsChange : oldBounds)
    }
    unsafe fn finalizeAnimatedBoundsChange(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finalizeAnimatedBoundsChange)
    }
    unsafe fn prepareForTransitionToLayout_(&self, newLayout: NSCollectionViewLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareForTransitionToLayout : newLayout)
    }
    unsafe fn prepareForTransitionFromLayout_(&self, oldLayout: NSCollectionViewLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareForTransitionFromLayout : oldLayout)
    }
    unsafe fn finalizeLayoutTransition(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finalizeLayoutTransition)
    }
    unsafe fn initialLayoutAttributesForAppearingItemAtIndexPath_(
        &self,
        itemIndexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initialLayoutAttributesForAppearingItemAtIndexPath : itemIndexPath)
    }
    unsafe fn finalLayoutAttributesForDisappearingItemAtIndexPath_(
        &self,
        itemIndexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finalLayoutAttributesForDisappearingItemAtIndexPath : itemIndexPath)
    }
    unsafe fn initialLayoutAttributesForAppearingSupplementaryElementOfKind_atIndexPath_(
        &self,
        elementKind: NSString,
        elementIndexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initialLayoutAttributesForAppearingSupplementaryElementOfKind : elementKind, atIndexPath : elementIndexPath)
    }
    unsafe fn finalLayoutAttributesForDisappearingSupplementaryElementOfKind_atIndexPath_(
        &self,
        elementKind: NSString,
        elementIndexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finalLayoutAttributesForDisappearingSupplementaryElementOfKind : elementKind, atIndexPath : elementIndexPath)
    }
    unsafe fn initialLayoutAttributesForAppearingDecorationElementOfKind_atIndexPath_(
        &self,
        elementKind: NSString,
        decorationIndexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initialLayoutAttributesForAppearingDecorationElementOfKind : elementKind, atIndexPath : decorationIndexPath)
    }
    unsafe fn finalLayoutAttributesForDisappearingDecorationElementOfKind_atIndexPath_(
        &self,
        elementKind: NSString,
        decorationIndexPath: NSIndexPath,
    ) -> NSCollectionViewLayoutAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finalLayoutAttributesForDisappearingDecorationElementOfKind : elementKind, atIndexPath : decorationIndexPath)
    }
    unsafe fn indexPathsToDeleteForSupplementaryViewOfKind_(&self, elementKind: NSString) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexPathsToDeleteForSupplementaryViewOfKind : elementKind)
    }
    unsafe fn indexPathsToDeleteForDecorationViewOfKind_(&self, elementKind: NSString) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexPathsToDeleteForDecorationViewOfKind : elementKind)
    }
    unsafe fn indexPathsToInsertForSupplementaryViewOfKind_(&self, elementKind: NSString) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexPathsToInsertForSupplementaryViewOfKind : elementKind)
    }
    unsafe fn indexPathsToInsertForDecorationViewOfKind_(&self, elementKind: NSString) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexPathsToInsertForDecorationViewOfKind : elementKind)
    }
}
pub type NSCollectionViewCompositionalLayoutSectionProvider = *mut ::std::os::raw::c_void;
pub type NSCollectionLayoutSectionVisibleItemsInvalidationHandler = *mut ::std::os::raw::c_void;
pub type NSCollectionLayoutGroupCustomItemProvider = *mut ::std::os::raw::c_void;
pub type NSCollectionViewDiffableDataSourceItemProvider = *mut ::std::os::raw::c_void;
pub type NSCollectionViewDiffableDataSourceSupplementaryViewProvider = *mut ::std::os::raw::c_void;
pub type NSFontWeight = CGFloat;
pub type NSFontWidth = CGFloat;
pub trait NSFontDescriptor_NSFontDescriptor_TextStyles: Sized + std::ops::Deref {
    unsafe fn preferredFontDescriptorForTextStyle_options_(
        style: NSString,
        options: NSDictionary,
    ) -> NSFontDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFontDescriptor").unwrap(), preferredFontDescriptorForTextStyle : style, options : options)
    }
}
pub trait NSFont_NSFont_Deprecated: Sized + std::ops::Deref {
    unsafe fn glyphWithName_(&self, name: NSString) -> NSGlyph
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphWithName : name)
    }
    unsafe fn boundingRectForGlyph_(&self, glyph: NSGlyph) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingRectForGlyph : glyph)
    }
    unsafe fn advancementForGlyph_(&self, glyph: NSGlyph) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, advancementForGlyph : glyph)
    }
    unsafe fn getBoundingRects_forGlyphs_count_(
        &self,
        bounds: NSRectArray,
        glyphs: *const NSGlyph,
        glyphCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getBoundingRects : bounds, forGlyphs : glyphs, count : glyphCount)
    }
    unsafe fn getAdvancements_forGlyphs_count_(
        &self,
        advancements: NSSizeArray,
        glyphs: *const NSGlyph,
        glyphCount: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAdvancements : advancements, forGlyphs : glyphs, count : glyphCount)
    }
    unsafe fn getAdvancements_forPackedGlyphs_length_(
        &self,
        advancements: NSSizeArray,
        packedGlyphs: *const ::std::os::raw::c_void,
        length: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAdvancements : advancements, forPackedGlyphs : packedGlyphs, length : length)
    }
    unsafe fn screenFontWithRenderingMode_(&self, renderingMode: NSFontRenderingMode) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, screenFontWithRenderingMode : renderingMode)
    }
    unsafe fn printerFont(&self) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, printerFont)
    }
    unsafe fn screenFont(&self) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, screenFont)
    }
    unsafe fn renderingMode(&self) -> NSFontRenderingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderingMode)
    }
}
pub trait NSFont_NSFont_TextStyles: Sized + std::ops::Deref {
    unsafe fn preferredFontForTextStyle_options_(style: NSString, options: NSDictionary) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSFont").unwrap(), preferredFontForTextStyle : style, options : options)
    }
}
pub trait NSFontManager_NSFontManagerMenuActionMethods: Sized + std::ops::Deref {
    unsafe fn fontNamed_hasTraits_(&self, fName: NSString, someTraits: NSFontTraitMask) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fontNamed : fName, hasTraits : someTraits)
    }
    unsafe fn availableFontNamesWithTraits_(&self, someTraits: NSFontTraitMask) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, availableFontNamesWithTraits : someTraits)
    }
    unsafe fn addFontTrait_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addFontTrait : sender)
    }
    unsafe fn removeFontTrait_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFontTrait : sender)
    }
    unsafe fn modifyFontViaPanel_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, modifyFontViaPanel : sender)
    }
    unsafe fn modifyFont_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, modifyFont : sender)
    }
    unsafe fn orderFrontFontPanel_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, orderFrontFontPanel : sender)
    }
    unsafe fn orderFrontStylesPanel_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, orderFrontStylesPanel : sender)
    }
}
pub trait NSObject_NSFontManagerDelegate: Sized + std::ops::Deref {
    unsafe fn fontManager_willIncludeFont_(&self, sender: id, fontName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fontManager : sender, willIncludeFont : fontName)
    }
}
pub trait NSObject_NSFontManagerResponderMethod: Sized + std::ops::Deref {
    unsafe fn changeFont_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeFont : sender)
    }
}
pub type NSWindowLevel = NSInteger;
pub type NSWindowFrameAutosaveName = NSString;
pub type NSWindowPersistableFrameDescriptor = NSString;
pub type NSWindowTabbingIdentifier = NSString;
pub trait NSWindow_NSEvent: Sized + std::ops::Deref {
    unsafe fn trackEventsMatchingMask_timeout_mode_handler_(
        &self,
        mask: NSEventMask,
        timeout: NSTimeInterval,
        mode: NSString,
        trackingHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, trackEventsMatchingMask : mask, timeout : timeout, mode : mode, handler : trackingHandler)
    }
    unsafe fn nextEventMatchingMask_(&self, mask: NSEventMask) -> NSEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nextEventMatchingMask : mask)
    }
    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(
        &self,
        mask: NSEventMask,
        expiration: NSDate,
        mode: NSString,
        deqFlag: BOOL,
    ) -> NSEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nextEventMatchingMask : mask, untilDate : expiration, inMode : mode, dequeue : deqFlag)
    }
    unsafe fn discardEventsMatchingMask_beforeEvent_(&self, mask: NSEventMask, lastEvent: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discardEventsMatchingMask : mask, beforeEvent : lastEvent)
    }
    unsafe fn postEvent_atStart_(&self, event: NSEvent, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, postEvent : event, atStart : flag)
    }
    unsafe fn sendEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendEvent : event)
    }
    unsafe fn currentEvent(&self) -> NSEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentEvent)
    }
    unsafe fn acceptsMouseMovedEvents(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptsMouseMovedEvents)
    }
    unsafe fn setAcceptsMouseMovedEvents_(&self, acceptsMouseMovedEvents: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcceptsMouseMovedEvents : acceptsMouseMovedEvents)
    }
    unsafe fn ignoresMouseEvents(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoresMouseEvents)
    }
    unsafe fn setIgnoresMouseEvents_(&self, ignoresMouseEvents: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIgnoresMouseEvents : ignoresMouseEvents)
    }
    unsafe fn mouseLocationOutsideOfEventStream(&self) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mouseLocationOutsideOfEventStream)
    }
}
pub trait NSWindow_NSCursorRect: Sized + std::ops::Deref {
    unsafe fn disableCursorRects(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableCursorRects)
    }
    unsafe fn enableCursorRects(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableCursorRects)
    }
    unsafe fn discardCursorRects(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discardCursorRects)
    }
    unsafe fn invalidateCursorRectsForView_(&self, view: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateCursorRectsForView : view)
    }
    unsafe fn resetCursorRects(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetCursorRects)
    }
    unsafe fn areCursorRectsEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, areCursorRectsEnabled)
    }
}
pub trait NSWindow_NSDrag: Sized + std::ops::Deref {
    unsafe fn beginDraggingSessionWithItems_event_source_(
        &self,
        items: NSArray,
        event: NSEvent,
        source: *mut u64,
    ) -> NSDraggingSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginDraggingSessionWithItems : items, event : event, source : source)
    }
    unsafe fn dragImage_at_offset_event_pasteboard_source_slideBack_(
        &self,
        image: NSImage,
        baseLocation: NSPoint,
        initialOffset: NSSize,
        event: NSEvent,
        pboard: NSPasteboard,
        sourceObj: id,
        slideFlag: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dragImage : image, at : baseLocation, offset : initialOffset, event : event, pasteboard : pboard, source : sourceObj, slideBack : slideFlag)
    }
    unsafe fn registerForDraggedTypes_(&self, newTypes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerForDraggedTypes : newTypes)
    }
    unsafe fn unregisterDraggedTypes(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unregisterDraggedTypes)
    }
}
pub trait NSWindow_NSDisplayLink: Sized + std::ops::Deref {
    unsafe fn displayLinkWithTarget_selector_(
        &self,
        target: id,
        selector: objc2::runtime::Sel,
    ) -> CADisplayLink
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayLinkWithTarget : target, selector : selector)
    }
}
pub trait NSWindow_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn cacheImageInRect_(&self, rect: NSRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cacheImageInRect : rect)
    }
    unsafe fn restoreCachedImage(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restoreCachedImage)
    }
    unsafe fn discardCachedImage(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discardCachedImage)
    }
    unsafe fn gState(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gState)
    }
    unsafe fn convertBaseToScreen_(&self, point: NSPoint) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertBaseToScreen : point)
    }
    unsafe fn convertScreenToBase_(&self, point: NSPoint) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertScreenToBase : point)
    }
    unsafe fn userSpaceScaleFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userSpaceScaleFactor)
    }
    unsafe fn useOptimizedDrawing_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useOptimizedDrawing : flag)
    }
    unsafe fn canStoreColor(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canStoreColor)
    }
    unsafe fn disableFlushWindow(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableFlushWindow)
    }
    unsafe fn enableFlushWindow(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableFlushWindow)
    }
    unsafe fn flushWindow(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flushWindow)
    }
    unsafe fn flushWindowIfNeeded(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flushWindowIfNeeded)
    }
    unsafe fn initWithWindowRef_(&self, windowRef: *mut ::std::os::raw::c_void) -> NSWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWindowRef : windowRef)
    }
    unsafe fn disableScreenUpdatesUntilFlush(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableScreenUpdatesUntilFlush)
    }
    unsafe fn isFlushWindowDisabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFlushWindowDisabled)
    }
    unsafe fn isAutodisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutodisplay)
    }
    unsafe fn setAutodisplay_(&self, autodisplay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutodisplay : autodisplay)
    }
    unsafe fn graphicsContext(&self) -> NSGraphicsContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, graphicsContext)
    }
    unsafe fn isOneShot(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOneShot)
    }
    unsafe fn setOneShot_(&self, oneShot: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOneShot : oneShot)
    }
    unsafe fn preferredBackingLocation(&self) -> NSWindowBackingLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredBackingLocation)
    }
    unsafe fn setPreferredBackingLocation_(&self, preferredBackingLocation: NSWindowBackingLocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredBackingLocation : preferredBackingLocation)
    }
    unsafe fn backingLocation(&self) -> NSWindowBackingLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backingLocation)
    }
    unsafe fn showsResizeIndicator(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsResizeIndicator)
    }
    unsafe fn setShowsResizeIndicator_(&self, showsResizeIndicator: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsResizeIndicator : showsResizeIndicator)
    }
    unsafe fn windowRef(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowRef)
    }
    unsafe fn menuChanged_(menu: NSMenu)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSWindow").unwrap(), menuChanged : menu)
    }
}
pub trait NSObject_NSFontPanelValidationAdditions: Sized + std::ops::Deref {
    unsafe fn validModesForFontPanel_(&self, fontPanel: NSFontPanel) -> NSFontPanelModeMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validModesForFontPanel : fontPanel)
    }
}
pub trait NSMatrix_NSKeyboardUI: Sized + std::ops::Deref {
    unsafe fn tabKeyTraversesCells(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabKeyTraversesCells)
    }
    unsafe fn setTabKeyTraversesCells_(&self, tabKeyTraversesCells: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTabKeyTraversesCells : tabKeyTraversesCells)
    }
    unsafe fn keyCell(&self) -> NSCell
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyCell)
    }
    unsafe fn setKeyCell_(&self, keyCell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyCell : keyCell)
    }
}
pub trait NSFormCell_NSKeyboardUI: Sized + std::ops::Deref {
    unsafe fn setTitleWithMnemonic_(&self, stringWithAmpersand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleWithMnemonic : stringWithAmpersand)
    }
}
pub trait NSFormCell_NSFormCellAttributedStringMethods: Sized + std::ops::Deref {
    unsafe fn attributedTitle(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedTitle)
    }
    unsafe fn setAttributedTitle_(&self, attributedTitle: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedTitle : attributedTitle)
    }
}
pub trait NSBitmapImageRep_NSBitmapImageFileTypeExtensions: Sized + std::ops::Deref {
    unsafe fn representationUsingType_properties_(
        &self,
        storageType: NSBitmapImageFileType,
        properties: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, representationUsingType : storageType, properties : properties)
    }
    unsafe fn setProperty_withValue_(&self, property: NSString, value: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperty : property, withValue : value)
    }
    unsafe fn valueForProperty_(&self, property: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForProperty : property)
    }
    unsafe fn representationOfImageRepsInArray_usingType_properties_(
        imageReps: NSArray,
        storageType: NSBitmapImageFileType,
        properties: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSBitmapImageRep").unwrap(), representationOfImageRepsInArray : imageReps, usingType : storageType, properties : properties)
    }
}
pub type NSBrowserColumnsAutosaveName = NSString;
pub trait NSBrowser_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setAcceptsArrowKeys_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcceptsArrowKeys : flag)
    }
    unsafe fn acceptsArrowKeys(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptsArrowKeys)
    }
    unsafe fn displayColumn_(&self, column: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayColumn : column)
    }
    unsafe fn displayAllColumns(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayAllColumns)
    }
    unsafe fn scrollViaScroller_(&self, sender: NSScroller)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollViaScroller : sender)
    }
    unsafe fn updateScroller(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateScroller)
    }
    unsafe fn setMatrixClass_(&self, factoryId: Class)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatrixClass : factoryId)
    }
    unsafe fn matrixClass(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matrixClass)
    }
    unsafe fn columnOfMatrix_(&self, matrix: NSMatrix) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, columnOfMatrix : matrix)
    }
    unsafe fn matrixInColumn_(&self, column: NSInteger) -> NSMatrix
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matrixInColumn : column)
    }
}
pub trait CIImage_NSAppKitAdditions: Sized + std::ops::Deref {
    unsafe fn initWithBitmapImageRep_(&self, bitmapImageRep: NSBitmapImageRep) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBitmapImageRep : bitmapImageRep)
    }
    unsafe fn drawInRect_fromRect_operation_fraction_(
        &self,
        rect: NSRect,
        fromRect: NSRect,
        op: NSCompositingOperation,
        delta: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawInRect : rect, fromRect : fromRect, operation : op, fraction : delta)
    }
    unsafe fn drawAtPoint_fromRect_operation_fraction_(
        &self,
        point: NSPoint,
        fromRect: NSRect,
        op: NSCompositingOperation,
        delta: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawAtPoint : point, fromRect : fromRect, operation : op, fraction : delta)
    }
}
pub trait NSApplication_NSColorPanel: Sized + std::ops::Deref {
    unsafe fn orderFrontColorPanel_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, orderFrontColorPanel : sender)
    }
}
pub trait NSObject_NSColorPanelResponderMethod: Sized + std::ops::Deref {
    unsafe fn changeColor_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeColor : sender)
    }
}
pub trait NSCursor_Deprecated: Sized + std::ops::Deref {
    unsafe fn currentSystemCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), currentSystemCursor)
    }
    unsafe fn resizeLeftCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), resizeLeftCursor)
    }
    unsafe fn resizeRightCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), resizeRightCursor)
    }
    unsafe fn resizeLeftRightCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), resizeLeftRightCursor)
    }
    unsafe fn resizeUpCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), resizeUpCursor)
    }
    unsafe fn resizeDownCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), resizeDownCursor)
    }
    unsafe fn resizeUpDownCursor() -> NSCursor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSCursor").unwrap(), resizeUpDownCursor)
    }
}
pub trait NSCursor_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn initWithImage_foregroundColorHint_backgroundColorHint_hotSpot_(
        &self,
        newImage: NSImage,
        fg: NSColor,
        bg: NSColor,
        hotSpot: NSPoint,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : newImage, foregroundColorHint : fg, backgroundColorHint : bg, hotSpot : hotSpot)
    }
    unsafe fn setOnMouseExited_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOnMouseExited : flag)
    }
    unsafe fn setOnMouseEntered_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOnMouseEntered : flag)
    }
    unsafe fn mouseEntered_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mouseEntered : event)
    }
    unsafe fn mouseExited_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mouseExited : event)
    }
    unsafe fn isSetOnMouseExited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSetOnMouseExited)
    }
    unsafe fn isSetOnMouseEntered(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSetOnMouseEntered)
    }
}
pub trait NSDocumentController_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn openDocumentWithContentsOfURL_display_error_(
        &self,
        url: NSURL,
        displayDocument: BOOL,
        outError: *mut NSError,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openDocumentWithContentsOfURL : url, display : displayDocument, error : outError)
    }
    unsafe fn reopenDocumentForURL_withContentsOfURL_error_(
        &self,
        url: NSURL,
        contentsURL: NSURL,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reopenDocumentForURL : url, withContentsOfURL : contentsURL, error : outError)
    }
    unsafe fn fileExtensionsFromType_(&self, typeName: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileExtensionsFromType : typeName)
    }
    unsafe fn typeFromFileExtension_(&self, fileNameExtensionOrHFSFileType: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, typeFromFileExtension : fileNameExtensionOrHFSFileType)
    }
    unsafe fn documentForFileName_(&self, fileName: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, documentForFileName : fileName)
    }
    unsafe fn fileNamesFromRunningOpenPanel(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileNamesFromRunningOpenPanel)
    }
    unsafe fn makeDocumentWithContentsOfFile_ofType_(
        &self,
        fileName: NSString,
        type_: NSString,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeDocumentWithContentsOfFile : fileName, ofType : type_)
    }
    unsafe fn makeDocumentWithContentsOfURL_ofType_(&self, url: NSURL, type_: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeDocumentWithContentsOfURL : url, ofType : type_)
    }
    unsafe fn makeUntitledDocumentOfType_(&self, type_: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeUntitledDocumentOfType : type_)
    }
    unsafe fn openDocumentWithContentsOfFile_display_(
        &self,
        fileName: NSString,
        display: BOOL,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openDocumentWithContentsOfFile : fileName, display : display)
    }
    unsafe fn openDocumentWithContentsOfURL_display_(&self, url: NSURL, display: BOOL) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openDocumentWithContentsOfURL : url, display : display)
    }
    unsafe fn openUntitledDocumentOfType_display_(&self, type_: NSString, display: BOOL) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openUntitledDocumentOfType : type_, display : display)
    }
    unsafe fn setShouldCreateUI_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldCreateUI : flag)
    }
    unsafe fn shouldCreateUI(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldCreateUI)
    }
}
pub type NSDraggingImageComponentKey = NSString;
pub trait NSFileWrapper_NSExtensions: Sized + std::ops::Deref {
    unsafe fn icon(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, icon)
    }
    unsafe fn setIcon_(&self, icon: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIcon : icon)
    }
}
pub trait NSGestureRecognizer_NSTouchBar: Sized + std::ops::Deref {
    unsafe fn allowedTouchTypes(&self) -> NSTouchTypeMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedTouchTypes)
    }
    unsafe fn setAllowedTouchTypes_(&self, allowedTouchTypes: NSTouchTypeMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedTouchTypes : allowedTouchTypes)
    }
}
pub trait NSGestureRecognizer_NSSubclassUse: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn canPreventGestureRecognizer_(
        &self,
        preventedGestureRecognizer: NSGestureRecognizer,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canPreventGestureRecognizer : preventedGestureRecognizer)
    }
    unsafe fn canBePreventedByGestureRecognizer_(
        &self,
        preventingGestureRecognizer: NSGestureRecognizer,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canBePreventedByGestureRecognizer : preventingGestureRecognizer)
    }
    unsafe fn shouldRequireFailureOfGestureRecognizer_(
        &self,
        otherGestureRecognizer: NSGestureRecognizer,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldRequireFailureOfGestureRecognizer : otherGestureRecognizer)
    }
    unsafe fn shouldBeRequiredToFailByGestureRecognizer_(
        &self,
        otherGestureRecognizer: NSGestureRecognizer,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldBeRequiredToFailByGestureRecognizer : otherGestureRecognizer)
    }
    unsafe fn mouseDown_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mouseDown : event)
    }
    unsafe fn rightMouseDown_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rightMouseDown : event)
    }
    unsafe fn otherMouseDown_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, otherMouseDown : event)
    }
    unsafe fn mouseUp_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mouseUp : event)
    }
    unsafe fn rightMouseUp_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rightMouseUp : event)
    }
    unsafe fn otherMouseUp_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, otherMouseUp : event)
    }
    unsafe fn mouseDragged_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mouseDragged : event)
    }
    unsafe fn rightMouseDragged_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rightMouseDragged : event)
    }
    unsafe fn otherMouseDragged_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, otherMouseDragged : event)
    }
    unsafe fn mouseCancelled_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mouseCancelled : event)
    }
    unsafe fn keyDown_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, keyDown : event)
    }
    unsafe fn keyUp_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, keyUp : event)
    }
    unsafe fn flagsChanged_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, flagsChanged : event)
    }
    unsafe fn tabletPoint_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tabletPoint : event)
    }
    unsafe fn magnifyWithEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, magnifyWithEvent : event)
    }
    unsafe fn rotateWithEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotateWithEvent : event)
    }
    unsafe fn pressureChangeWithEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pressureChangeWithEvent : event)
    }
    unsafe fn touchesBeganWithEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, touchesBeganWithEvent : event)
    }
    unsafe fn touchesMovedWithEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, touchesMovedWithEvent : event)
    }
    unsafe fn touchesEndedWithEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, touchesEndedWithEvent : event)
    }
    unsafe fn touchesCancelledWithEvent_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, touchesCancelledWithEvent : event)
    }
    unsafe fn state(&self) -> NSGestureRecognizerState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn setState_(&self, state: NSGestureRecognizerState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setState : state)
    }
}
pub type NSLayoutPriority = f32;
pub trait NSLayoutConstraint_NSIdentifier: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn setIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifier : identifier)
    }
}
pub trait NSLayoutConstraint_: Sized + std::ops::Deref {}
pub trait NSView_NSConstraintBasedLayoutInstallingConstraints: Sized + std::ops::Deref {
    unsafe fn addConstraint_(&self, constraint: NSLayoutConstraint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConstraint : constraint)
    }
    unsafe fn addConstraints_(&self, constraints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConstraints : constraints)
    }
    unsafe fn removeConstraint_(&self, constraint: NSLayoutConstraint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeConstraint : constraint)
    }
    unsafe fn removeConstraints_(&self, constraints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeConstraints : constraints)
    }
    unsafe fn leadingAnchor(&self) -> NSLayoutXAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingAnchor)
    }
    unsafe fn trailingAnchor(&self) -> NSLayoutXAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingAnchor)
    }
    unsafe fn leftAnchor(&self) -> NSLayoutXAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftAnchor)
    }
    unsafe fn rightAnchor(&self) -> NSLayoutXAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightAnchor)
    }
    unsafe fn topAnchor(&self) -> NSLayoutYAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topAnchor)
    }
    unsafe fn bottomAnchor(&self) -> NSLayoutYAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomAnchor)
    }
    unsafe fn widthAnchor(&self) -> NSLayoutDimension
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widthAnchor)
    }
    unsafe fn heightAnchor(&self) -> NSLayoutDimension
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightAnchor)
    }
    unsafe fn centerXAnchor(&self) -> NSLayoutXAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerXAnchor)
    }
    unsafe fn centerYAnchor(&self) -> NSLayoutYAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerYAnchor)
    }
    unsafe fn firstBaselineAnchor(&self) -> NSLayoutYAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstBaselineAnchor)
    }
    unsafe fn lastBaselineAnchor(&self) -> NSLayoutYAxisAnchor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastBaselineAnchor)
    }
    unsafe fn constraints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constraints)
    }
}
pub trait NSWindow_NSConstraintBasedLayoutCoreMethods: Sized + std::ops::Deref {
    unsafe fn updateConstraintsIfNeeded(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateConstraintsIfNeeded)
    }
    unsafe fn layoutIfNeeded(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutIfNeeded)
    }
}
pub trait NSView_NSConstraintBasedLayoutCoreMethods: Sized + std::ops::Deref {
    unsafe fn updateConstraintsForSubtreeIfNeeded(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateConstraintsForSubtreeIfNeeded)
    }
    unsafe fn updateConstraints(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateConstraints)
    }
    unsafe fn needsUpdateConstraints(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, needsUpdateConstraints)
    }
    unsafe fn setNeedsUpdateConstraints_(&self, needsUpdateConstraints: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeedsUpdateConstraints : needsUpdateConstraints)
    }
}
pub trait NSView_NSConstraintBasedCompatibility: Sized + std::ops::Deref {
    unsafe fn translatesAutoresizingMaskIntoConstraints(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, translatesAutoresizingMaskIntoConstraints)
    }
    unsafe fn setTranslatesAutoresizingMaskIntoConstraints_(
        &self,
        translatesAutoresizingMaskIntoConstraints: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTranslatesAutoresizingMaskIntoConstraints : translatesAutoresizingMaskIntoConstraints)
    }
    unsafe fn requiresConstraintBasedLayout() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSView").unwrap(), requiresConstraintBasedLayout)
    }
}
pub trait NSView_NSConstraintBasedLayoutLayering: Sized + std::ops::Deref {
    unsafe fn alignmentRectForFrame_(&self, frame: NSRect) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, alignmentRectForFrame : frame)
    }
    unsafe fn frameForAlignmentRect_(&self, alignmentRect: NSRect) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, frameForAlignmentRect : alignmentRect)
    }
    unsafe fn invalidateIntrinsicContentSize(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidateIntrinsicContentSize)
    }
    unsafe fn contentHuggingPriorityForOrientation_(
        &self,
        orientation: NSLayoutConstraintOrientation,
    ) -> NSLayoutPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentHuggingPriorityForOrientation : orientation)
    }
    unsafe fn setContentHuggingPriority_forOrientation_(
        &self,
        priority: NSLayoutPriority,
        orientation: NSLayoutConstraintOrientation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentHuggingPriority : priority, forOrientation : orientation)
    }
    unsafe fn contentCompressionResistancePriorityForOrientation_(
        &self,
        orientation: NSLayoutConstraintOrientation,
    ) -> NSLayoutPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentCompressionResistancePriorityForOrientation : orientation)
    }
    unsafe fn setContentCompressionResistancePriority_forOrientation_(
        &self,
        priority: NSLayoutPriority,
        orientation: NSLayoutConstraintOrientation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentCompressionResistancePriority : priority, forOrientation : orientation)
    }
    unsafe fn alignmentRectInsets(&self) -> NSEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignmentRectInsets)
    }
    unsafe fn firstBaselineOffsetFromTop(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstBaselineOffsetFromTop)
    }
    unsafe fn lastBaselineOffsetFromBottom(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastBaselineOffsetFromBottom)
    }
    unsafe fn baselineOffsetFromBottom(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baselineOffsetFromBottom)
    }
    unsafe fn intrinsicContentSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intrinsicContentSize)
    }
    unsafe fn isHorizontalContentSizeConstraintActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHorizontalContentSizeConstraintActive)
    }
    unsafe fn setHorizontalContentSizeConstraintActive_(
        &self,
        horizontalContentSizeConstraintActive: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHorizontalContentSizeConstraintActive : horizontalContentSizeConstraintActive)
    }
    unsafe fn isVerticalContentSizeConstraintActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVerticalContentSizeConstraintActive)
    }
    unsafe fn setVerticalContentSizeConstraintActive_(
        &self,
        verticalContentSizeConstraintActive: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVerticalContentSizeConstraintActive : verticalContentSizeConstraintActive)
    }
}
pub trait NSControl_NSConstraintBasedLayoutLayering: Sized + std::ops::Deref {
    unsafe fn invalidateIntrinsicContentSizeForCell_(&self, cell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateIntrinsicContentSizeForCell : cell)
    }
}
pub trait NSWindow_NSConstraintBasedLayoutAnchoring: Sized + std::ops::Deref {
    unsafe fn anchorAttributeForOrientation_(
        &self,
        orientation: NSLayoutConstraintOrientation,
    ) -> NSLayoutAttribute
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, anchorAttributeForOrientation : orientation)
    }
    unsafe fn setAnchorAttribute_forOrientation_(
        &self,
        attr: NSLayoutAttribute,
        orientation: NSLayoutConstraintOrientation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorAttribute : attr, forOrientation : orientation)
    }
}
pub trait NSView_NSConstraintBasedLayoutFittingSize: Sized + std::ops::Deref {
    unsafe fn fittingSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fittingSize)
    }
}
pub trait NSView_NSConstraintBasedLayoutDebugging: Sized + std::ops::Deref {
    unsafe fn constraintsAffectingLayoutForOrientation_(
        &self,
        orientation: NSLayoutConstraintOrientation,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constraintsAffectingLayoutForOrientation : orientation)
    }
    unsafe fn exerciseAmbiguityInLayout(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exerciseAmbiguityInLayout)
    }
    unsafe fn hasAmbiguousLayout(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAmbiguousLayout)
    }
}
pub trait NSWindow_NSConstraintBasedLayoutDebugging: Sized + std::ops::Deref {
    unsafe fn visualizeConstraints_(&self, constraints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visualizeConstraints : constraints)
    }
}
pub trait NSView_NSLayoutGuideSupport: Sized + std::ops::Deref {
    unsafe fn addLayoutGuide_(&self, guide: NSLayoutGuide)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addLayoutGuide : guide)
    }
    unsafe fn removeLayoutGuide_(&self, guide: NSLayoutGuide)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeLayoutGuide : guide)
    }
    unsafe fn layoutGuides(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutGuides)
    }
}
pub type NSImageName = NSString;
pub trait NSBundle_NSBundleImageExtension: Sized + std::ops::Deref {
    unsafe fn imageForResource_(&self, name: NSString) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageForResource : name)
    }
    unsafe fn pathForImageResource_(&self, name: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pathForImageResource : name)
    }
    unsafe fn URLForImageResource_(&self, name: NSString) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, URLForImageResource : name)
    }
}
pub trait NSImage_Deprecated: Sized + std::ops::Deref {
    unsafe fn initWithIconRef_(&self, iconRef: IconRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIconRef : iconRef)
    }
    unsafe fn bestRepresentationForDevice_(&self, deviceDescription: NSDictionary) -> NSImageRep
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bestRepresentationForDevice : deviceDescription)
    }
    unsafe fn lockFocus(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lockFocus)
    }
    unsafe fn lockFocusFlipped_(&self, flipped: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockFocusFlipped : flipped)
    }
    unsafe fn unlockFocus(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unlockFocus)
    }
    unsafe fn setFlipped_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlipped : flag)
    }
    unsafe fn isFlipped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFlipped)
    }
    unsafe fn setScalesWhenResized_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScalesWhenResized : flag)
    }
    unsafe fn scalesWhenResized(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scalesWhenResized)
    }
    unsafe fn setDataRetained_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataRetained : flag)
    }
    unsafe fn isDataRetained(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDataRetained)
    }
    unsafe fn setCachedSeparately_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCachedSeparately : flag)
    }
    unsafe fn isCachedSeparately(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCachedSeparately)
    }
    unsafe fn setCacheDepthMatchesImageDepth_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCacheDepthMatchesImageDepth : flag)
    }
    unsafe fn cacheDepthMatchesImageDepth(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cacheDepthMatchesImageDepth)
    }
    unsafe fn dissolveToPoint_fraction_(&self, point: NSPoint, fraction_: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dissolveToPoint : point, fraction : fraction_)
    }
    unsafe fn dissolveToPoint_fromRect_fraction_(
        &self,
        point: NSPoint,
        rect: NSRect,
        fraction_: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dissolveToPoint : point, fromRect : rect, fraction : fraction_)
    }
    unsafe fn compositeToPoint_operation_(&self, point: NSPoint, operation: NSCompositingOperation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositeToPoint : point, operation : operation)
    }
    unsafe fn compositeToPoint_fromRect_operation_(
        &self,
        point: NSPoint,
        rect: NSRect,
        operation: NSCompositingOperation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositeToPoint : point, fromRect : rect, operation : operation)
    }
    unsafe fn compositeToPoint_operation_fraction_(
        &self,
        point: NSPoint,
        operation: NSCompositingOperation,
        fraction_: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositeToPoint : point, operation : operation, fraction : fraction_)
    }
    unsafe fn compositeToPoint_fromRect_operation_fraction_(
        &self,
        point: NSPoint,
        rect: NSRect,
        operation: NSCompositingOperation,
        fraction_: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositeToPoint : point, fromRect : rect, operation : operation, fraction : fraction_)
    }
    unsafe fn lockFocusOnRepresentation_(&self, imageRepresentation: NSImageRep)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockFocusOnRepresentation : imageRepresentation)
    }
    unsafe fn cancelIncrementalLoad(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelIncrementalLoad)
    }
    unsafe fn imageUnfilteredFileTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSImage").unwrap(), imageUnfilteredFileTypes)
    }
    unsafe fn imageUnfilteredPasteboardTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSImage").unwrap(), imageUnfilteredPasteboardTypes)
    }
    unsafe fn imageFileTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSImage").unwrap(), imageFileTypes)
    }
    unsafe fn imagePasteboardTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSImage").unwrap(), imagePasteboardTypes)
    }
}
pub trait NSImageView_NSSymbolEffect: Sized + std::ops::Deref {
    unsafe fn addSymbolEffect_(&self, symbolEffect: NSSymbolEffect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSymbolEffect : symbolEffect)
    }
    unsafe fn addSymbolEffect_options_(
        &self,
        symbolEffect: NSSymbolEffect,
        options: NSSymbolEffectOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSymbolEffect : symbolEffect, options : options)
    }
    unsafe fn addSymbolEffect_options_animated_(
        &self,
        symbolEffect: NSSymbolEffect,
        options: NSSymbolEffectOptions,
        animated: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSymbolEffect : symbolEffect, options : options, animated : animated)
    }
    unsafe fn removeSymbolEffectOfType_(&self, symbolEffect: NSSymbolEffect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeSymbolEffectOfType : symbolEffect)
    }
    unsafe fn removeSymbolEffectOfType_options_(
        &self,
        symbolEffect: NSSymbolEffect,
        options: NSSymbolEffectOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeSymbolEffectOfType : symbolEffect, options : options)
    }
    unsafe fn removeSymbolEffectOfType_options_animated_(
        &self,
        symbolEffect: NSSymbolEffect,
        options: NSSymbolEffectOptions,
        animated: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeSymbolEffectOfType : symbolEffect, options : options, animated : animated)
    }
    unsafe fn removeAllSymbolEffects(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllSymbolEffects)
    }
    unsafe fn removeAllSymbolEffectsWithOptions_(&self, options: NSSymbolEffectOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAllSymbolEffectsWithOptions : options)
    }
    unsafe fn removeAllSymbolEffectsWithOptions_animated_(
        &self,
        options: NSSymbolEffectOptions,
        animated: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAllSymbolEffectsWithOptions : options, animated : animated)
    }
    unsafe fn setSymbolImage_withContentTransition_(
        &self,
        symbolImage: NSImage,
        transition: NSSymbolContentTransition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSymbolImage : symbolImage, withContentTransition : transition)
    }
    unsafe fn setSymbolImage_withContentTransition_options_(
        &self,
        symbolImage: NSImage,
        transition: NSSymbolContentTransition,
        options: NSSymbolEffectOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSymbolImage : symbolImage, withContentTransition : transition, options : options)
    }
}
pub trait NSBundle_NSNibLoading: Sized + std::ops::Deref {
    unsafe fn loadNibNamed_owner_topLevelObjects_(
        &self,
        nibName: NSString,
        owner: id,
        topLevelObjects: *mut NSArray,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadNibNamed : nibName, owner : owner, topLevelObjects : topLevelObjects)
    }
}
pub trait NSObject_NSNibAwaking: Sized + std::ops::Deref {
    unsafe fn awakeFromNib(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, awakeFromNib)
    }
    unsafe fn prepareForInterfaceBuilder(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareForInterfaceBuilder)
    }
}
pub trait NSBundle_NSNibLoadingDeprecated: Sized + std::ops::Deref {
    unsafe fn loadNibFile_externalNameTable_withZone_(
        &self,
        fileName: NSString,
        context: NSDictionary,
        zone: *mut NSZone,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadNibFile : fileName, externalNameTable : context, withZone : zone)
    }
    unsafe fn class_loadNibFile_externalNameTable_withZone_(
        fileName: NSString,
        context: NSDictionary,
        zone: *mut NSZone,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSBundle").unwrap(), loadNibFile : fileName, externalNameTable : context, withZone : zone)
    }
    unsafe fn loadNibNamed_owner_(nibName: NSString, owner: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSBundle").unwrap(), loadNibNamed : nibName, owner : owner)
    }
}
pub trait NSItemProvider_NSCloudKitSharing: Sized + std::ops::Deref {
    unsafe fn registerCloudKitShareWithPreparationHandler_(
        &self,
        preparationHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerCloudKitShareWithPreparationHandler : preparationHandler)
    }
    unsafe fn registerCloudKitShare_container_(&self, share: CKShare, container: CKContainer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerCloudKitShare : share, container : container)
    }
}
pub trait NSSliderAccessory_: Sized + std::ops::Deref {}
pub type NSSliderAccessoryWidth = CGFloat;
pub type NSSpeechSynthesizerVoiceName = NSString;
pub trait NSSpellChecker_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn guessesForWord_(&self, word: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, guessesForWord : word)
    }
    unsafe fn forgetWord_(&self, word: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, forgetWord : word)
    }
}
pub type NSSplitViewAutosaveName = NSString;
pub trait NSSplitView_NSSplitViewArrangedSubviews: Sized + std::ops::Deref {
    unsafe fn addArrangedSubview_(&self, view: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addArrangedSubview : view)
    }
    unsafe fn insertArrangedSubview_atIndex_(&self, view: NSView, index: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertArrangedSubview : view, atIndex : index)
    }
    unsafe fn removeArrangedSubview_(&self, view: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeArrangedSubview : view)
    }
    unsafe fn arrangesAllSubviews(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrangesAllSubviews)
    }
    unsafe fn setArrangesAllSubviews_(&self, arrangesAllSubviews: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArrangesAllSubviews : arrangesAllSubviews)
    }
    unsafe fn arrangedSubviews(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrangedSubviews)
    }
}
pub trait NSSplitView_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setIsPaneSplitter_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsPaneSplitter : flag)
    }
    unsafe fn isPaneSplitter(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaneSplitter)
    }
}
pub trait NSSplitViewController_NSSplitViewControllerToggleSidebarAction:
    Sized + std::ops::Deref
{
    unsafe fn toggleSidebar_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleSidebar : sender)
    }
    unsafe fn toggleInspector_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleInspector : sender)
    }
}
pub trait NSObject_NSSavePanelDelegateDeprecated: Sized + std::ops::Deref {
    unsafe fn panel_isValidFilename_(&self, sender: id, filename: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, panel : sender, isValidFilename : filename)
    }
    unsafe fn panel_directoryDidChange_(&self, sender: id, path: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, panel : sender, directoryDidChange : path)
    }
    unsafe fn panel_compareFilename_with_caseSensitive_(
        &self,
        sender: id,
        name1: NSString,
        name2: NSString,
        caseSensitive: BOOL,
    ) -> NSComparisonResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, panel : sender, compareFilename : name1, with : name2, caseSensitive : caseSensitive)
    }
    unsafe fn panel_shouldShowFilename_(&self, sender: id, filename: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, panel : sender, shouldShowFilename : filename)
    }
}
pub trait NSSavePanel_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn filename(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filename)
    }
    unsafe fn directory(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, directory)
    }
    unsafe fn setDirectory_(&self, path: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDirectory : path)
    }
    unsafe fn requiredFileType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredFileType)
    }
    unsafe fn setRequiredFileType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredFileType : type_)
    }
    unsafe fn beginSheetForDirectory_file_modalForWindow_modalDelegate_didEndSelector_contextInfo_(
        &self,
        path: NSString,
        name: NSString,
        docWindow: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetForDirectory : path, file : name, modalForWindow : docWindow, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn runModalForDirectory_file_(&self, path: NSString, name: NSString) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForDirectory : path, file : name)
    }
    unsafe fn selectText_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectText : sender)
    }
    unsafe fn allowedFileTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedFileTypes)
    }
    unsafe fn setAllowedFileTypes_(&self, allowedFileTypes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedFileTypes : allowedFileTypes)
    }
}
pub trait NSOpenPanel_InheritedAndUnavailable: Sized + std::ops::Deref {
    unsafe fn showsContentTypes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsContentTypes)
    }
    unsafe fn setShowsContentTypes_(&self, showsContentTypes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsContentTypes : showsContentTypes)
    }
}
pub trait NSOpenPanel_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn filenames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filenames)
    }
    unsafe fn beginSheetForDirectory_file_types_modalForWindow_modalDelegate_didEndSelector_contextInfo_(
        &self,
        path: NSString,
        name: NSString,
        fileTypes: NSArray,
        docWindow: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetForDirectory : path, file : name, types : fileTypes, modalForWindow : docWindow, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn beginForDirectory_file_types_modelessDelegate_didEndSelector_contextInfo_(
        &self,
        path: NSString,
        name: NSString,
        fileTypes: NSArray,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginForDirectory : path, file : name, types : fileTypes, modelessDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn runModalForDirectory_file_types_(
        &self,
        path: NSString,
        name: NSString,
        fileTypes: NSArray,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForDirectory : path, file : name, types : fileTypes)
    }
    unsafe fn runModalForTypes_(&self, fileTypes: NSArray) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForTypes : fileTypes)
    }
}
pub trait NSPageLayout_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setAccessoryView_(&self, accessoryView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessoryView : accessoryView)
    }
    unsafe fn accessoryView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryView)
    }
    unsafe fn readPrintInfo(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readPrintInfo)
    }
    unsafe fn writePrintInfo(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writePrintInfo)
    }
}
pub trait NSApplication_NSPageLayoutPanel: Sized + std::ops::Deref {
    unsafe fn runPageLayout_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runPageLayout : sender)
    }
}
pub trait NSPrintOperation_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setAccessoryView_(&self, view: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessoryView : view)
    }
    unsafe fn accessoryView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryView)
    }
    unsafe fn setJobStyleHint_(&self, hint: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJobStyleHint : hint)
    }
    unsafe fn jobStyleHint(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jobStyleHint)
    }
    unsafe fn setShowPanels_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowPanels : flag)
    }
    unsafe fn showPanels(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showPanels)
    }
}
pub type NSPrintPanelJobStyleHint = NSString;
pub trait NSPrintPanel_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setAccessoryView_(&self, accessoryView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessoryView : accessoryView)
    }
    unsafe fn accessoryView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryView)
    }
    unsafe fn updateFromPrintInfo(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateFromPrintInfo)
    }
    unsafe fn finalWritePrintInfo(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finalWritePrintInfo)
    }
}
pub trait NSScreen_: Sized + std::ops::Deref {
    unsafe fn maximumExtendedDynamicRangeColorComponentValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumExtendedDynamicRangeColorComponentValue)
    }
    unsafe fn maximumPotentialExtendedDynamicRangeColorComponentValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumPotentialExtendedDynamicRangeColorComponentValue)
    }
    unsafe fn maximumReferenceExtendedDynamicRangeColorComponentValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumReferenceExtendedDynamicRangeColorComponentValue)
    }
    unsafe fn maximumFramesPerSecond(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumFramesPerSecond)
    }
    unsafe fn minimumRefreshInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumRefreshInterval)
    }
    unsafe fn maximumRefreshInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumRefreshInterval)
    }
    unsafe fn displayUpdateGranularity(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayUpdateGranularity)
    }
    unsafe fn lastDisplayUpdateTimestamp(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastDisplayUpdateTimestamp)
    }
}
pub trait NSScreen_NSDisplayLink: Sized + std::ops::Deref {
    unsafe fn displayLinkWithTarget_selector_(
        &self,
        target: id,
        selector: objc2::runtime::Sel,
    ) -> CADisplayLink
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayLinkWithTarget : target, selector : selector)
    }
}
pub trait NSScreen_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn userSpaceScaleFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userSpaceScaleFactor)
    }
}
pub trait NSScroller_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setFloatValue_knobProportion_(&self, value: f32, proportion: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatValue : value, knobProportion : proportion)
    }
    unsafe fn highlight_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, highlight : flag)
    }
    unsafe fn trackScrollButtons_(&self, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, trackScrollButtons : event)
    }
    unsafe fn drawParts(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawParts)
    }
    unsafe fn drawArrow_highlight_(&self, whichArrow: NSScrollerArrow, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawArrow : whichArrow, highlight : flag)
    }
    unsafe fn arrowsPosition(&self) -> NSScrollArrowPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrowsPosition)
    }
    unsafe fn setArrowsPosition_(&self, arrowsPosition: NSScrollArrowPosition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArrowsPosition : arrowsPosition)
    }
    unsafe fn controlTint(&self) -> NSControlTint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlTint)
    }
    unsafe fn setControlTint_(&self, controlTint: NSControlTint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlTint : controlTint)
    }
    unsafe fn scrollerWidthForControlSize_(controlSize: NSControlSize) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSScroller").unwrap(), scrollerWidthForControlSize : controlSize)
    }
    unsafe fn scrollerWidth() -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSScroller").unwrap(), scrollerWidth)
    }
}
pub trait NSScrollView_NSRulerSupport: Sized + std::ops::Deref {
    unsafe fn rulersVisible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rulersVisible)
    }
    unsafe fn setRulersVisible_(&self, rulersVisible: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRulersVisible : rulersVisible)
    }
    unsafe fn hasHorizontalRuler(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasHorizontalRuler)
    }
    unsafe fn setHasHorizontalRuler_(&self, hasHorizontalRuler: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasHorizontalRuler : hasHorizontalRuler)
    }
    unsafe fn hasVerticalRuler(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasVerticalRuler)
    }
    unsafe fn setHasVerticalRuler_(&self, hasVerticalRuler: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasVerticalRuler : hasVerticalRuler)
    }
    unsafe fn horizontalRulerView(&self) -> NSRulerView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalRulerView)
    }
    unsafe fn setHorizontalRulerView_(&self, horizontalRulerView: NSRulerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHorizontalRulerView : horizontalRulerView)
    }
    unsafe fn verticalRulerView(&self) -> NSRulerView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalRulerView)
    }
    unsafe fn setVerticalRulerView_(&self, verticalRulerView: NSRulerView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVerticalRulerView : verticalRulerView)
    }
    unsafe fn rulerViewClass() -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSScrollView").unwrap(), rulerViewClass)
    }
    unsafe fn setRulerViewClass_(rulerViewClass: Class)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSScrollView").unwrap(), setRulerViewClass : rulerViewClass)
    }
}
pub trait NSScrollView_NSFindBarSupport: Sized + std::ops::Deref {
    unsafe fn findBarPosition(&self) -> NSScrollViewFindBarPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, findBarPosition)
    }
    unsafe fn setFindBarPosition_(&self, findBarPosition: NSScrollViewFindBarPosition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFindBarPosition : findBarPosition)
    }
}
pub trait NSSegmentedControl_NSSegmentedControlConvenience: Sized + std::ops::Deref {
    unsafe fn segmentedControlWithLabels_trackingMode_target_action_(
        labels: NSArray,
        trackingMode: NSSegmentSwitchTracking,
        target: id,
        action: objc2::runtime::Sel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSegmentedControl").unwrap(), segmentedControlWithLabels : labels, trackingMode : trackingMode, target : target, action : action)
    }
    unsafe fn segmentedControlWithImages_trackingMode_target_action_(
        images: NSArray,
        trackingMode: NSSegmentSwitchTracking,
        target: id,
        action: objc2::runtime::Sel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSegmentedControl").unwrap(), segmentedControlWithImages : images, trackingMode : trackingMode, target : target, action : action)
    }
}
pub trait NSSegmentedCell_NSSegmentBackgroundStyle: Sized + std::ops::Deref {
    unsafe fn interiorBackgroundStyleForSegment_(&self, segment: NSInteger) -> NSBackgroundStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, interiorBackgroundStyleForSegment : segment)
    }
}
pub trait NSSliderCell_NSSliderCellVerticalGetter: Sized + std::ops::Deref {
    unsafe fn isVertical(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVertical)
    }
}
pub trait NSSliderCell_NSTickMarkSupport: Sized + std::ops::Deref {
    unsafe fn tickMarkValueAtIndex_(&self, index: NSInteger) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tickMarkValueAtIndex : index)
    }
    unsafe fn rectOfTickMarkAtIndex_(&self, index: NSInteger) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectOfTickMarkAtIndex : index)
    }
    unsafe fn indexOfTickMarkAtPoint_(&self, point: NSPoint) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexOfTickMarkAtPoint : point)
    }
    unsafe fn closestTickMarkValueToValue_(&self, value: f64) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, closestTickMarkValueToValue : value)
    }
    unsafe fn drawTickMarks(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawTickMarks)
    }
    unsafe fn numberOfTickMarks(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfTickMarks)
    }
    unsafe fn setNumberOfTickMarks_(&self, numberOfTickMarks: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfTickMarks : numberOfTickMarks)
    }
    unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tickMarkPosition)
    }
    unsafe fn setTickMarkPosition_(&self, tickMarkPosition: NSTickMarkPosition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTickMarkPosition : tickMarkPosition)
    }
    unsafe fn allowsTickMarkValuesOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsTickMarkValuesOnly)
    }
    unsafe fn setAllowsTickMarkValuesOnly_(&self, allowsTickMarkValuesOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsTickMarkValuesOnly : allowsTickMarkValuesOnly)
    }
}
pub trait NSSliderCell_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setTitleCell_(&self, cell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleCell : cell)
    }
    unsafe fn titleCell(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleCell)
    }
    unsafe fn setTitleColor_(&self, newColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleColor : newColor)
    }
    unsafe fn titleColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleColor)
    }
    unsafe fn setTitleFont_(&self, fontObj: NSFont)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleFont : fontObj)
    }
    unsafe fn titleFont(&self) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleFont)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : string)
    }
    unsafe fn setKnobThickness_(&self, thickness: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKnobThickness : thickness)
    }
    unsafe fn setImage_(&self, backgroundImage: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : backgroundImage)
    }
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
}
pub trait NSSlider_NSSliderVerticalGetter: Sized + std::ops::Deref {
    unsafe fn isVertical(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVertical)
    }
}
pub trait NSSlider_NSTickMarkSupport: Sized + std::ops::Deref {
    unsafe fn tickMarkValueAtIndex_(&self, index: NSInteger) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tickMarkValueAtIndex : index)
    }
    unsafe fn rectOfTickMarkAtIndex_(&self, index: NSInteger) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectOfTickMarkAtIndex : index)
    }
    unsafe fn indexOfTickMarkAtPoint_(&self, point: NSPoint) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexOfTickMarkAtPoint : point)
    }
    unsafe fn closestTickMarkValueToValue_(&self, value: f64) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, closestTickMarkValueToValue : value)
    }
    unsafe fn numberOfTickMarks(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfTickMarks)
    }
    unsafe fn setNumberOfTickMarks_(&self, numberOfTickMarks: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfTickMarks : numberOfTickMarks)
    }
    unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tickMarkPosition)
    }
    unsafe fn setTickMarkPosition_(&self, tickMarkPosition: NSTickMarkPosition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTickMarkPosition : tickMarkPosition)
    }
    unsafe fn allowsTickMarkValuesOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsTickMarkValuesOnly)
    }
    unsafe fn setAllowsTickMarkValuesOnly_(&self, allowsTickMarkValuesOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsTickMarkValuesOnly : allowsTickMarkValuesOnly)
    }
}
pub trait NSSlider_NSSliderConvenience: Sized + std::ops::Deref {
    unsafe fn sliderWithTarget_action_(target: id, action: objc2::runtime::Sel) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSlider").unwrap(), sliderWithTarget : target, action : action)
    }
    unsafe fn sliderWithValue_minValue_maxValue_target_action_(
        value: f64,
        minValue: f64,
        maxValue: f64,
        target: id,
        action: objc2::runtime::Sel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSlider").unwrap(), sliderWithValue : value, minValue : minValue, maxValue : maxValue, target : target, action : action)
    }
}
pub trait NSSlider_NSSliderDeprecated: Sized + std::ops::Deref {
    unsafe fn setTitleCell_(&self, cell: NSCell)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleCell : cell)
    }
    unsafe fn titleCell(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleCell)
    }
    unsafe fn setTitleColor_(&self, newColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleColor : newColor)
    }
    unsafe fn titleColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleColor)
    }
    unsafe fn setTitleFont_(&self, fontObj: NSFont)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleFont : fontObj)
    }
    unsafe fn titleFont(&self) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleFont)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : string)
    }
    unsafe fn setKnobThickness_(&self, thickness: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKnobThickness : thickness)
    }
    unsafe fn setImage_(&self, backgroundImage: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : backgroundImage)
    }
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
}
pub type NSStackViewVisibilityPriority = f32;
pub trait NSStackView_NSStackViewGravityAreas: Sized + std::ops::Deref {
    unsafe fn addView_inGravity_(&self, view: NSView, gravity: NSStackViewGravity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addView : view, inGravity : gravity)
    }
    unsafe fn insertView_atIndex_inGravity_(
        &self,
        view: NSView,
        index: NSUInteger,
        gravity: NSStackViewGravity,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertView : view, atIndex : index, inGravity : gravity)
    }
    unsafe fn removeView_(&self, view: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeView : view)
    }
    unsafe fn viewsInGravity_(&self, gravity: NSStackViewGravity) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, viewsInGravity : gravity)
    }
    unsafe fn setViews_inGravity_(&self, views: NSArray, gravity: NSStackViewGravity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setViews : views, inGravity : gravity)
    }
    unsafe fn views(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, views)
    }
}
pub trait NSStackView_NSStackViewDeprecated: Sized + std::ops::Deref {
    unsafe fn hasEqualSpacing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasEqualSpacing)
    }
    unsafe fn setHasEqualSpacing_(&self, hasEqualSpacing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasEqualSpacing : hasEqualSpacing)
    }
}
pub type NSTextContentType = NSString;
pub trait NSTextField_NSTouchBar: Sized + std::ops::Deref {
    unsafe fn isAutomaticTextCompletionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticTextCompletionEnabled)
    }
    unsafe fn setAutomaticTextCompletionEnabled_(&self, automaticTextCompletionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticTextCompletionEnabled : automaticTextCompletionEnabled)
    }
    unsafe fn allowsCharacterPickerTouchBarItem(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsCharacterPickerTouchBarItem)
    }
    unsafe fn setAllowsCharacterPickerTouchBarItem_(&self, allowsCharacterPickerTouchBarItem: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsCharacterPickerTouchBarItem : allowsCharacterPickerTouchBarItem)
    }
}
pub trait NSTextField_NSTextFieldConvenience: Sized + std::ops::Deref {
    unsafe fn labelWithString_(stringValue: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextField").unwrap(), labelWithString : stringValue)
    }
    unsafe fn wrappingLabelWithString_(stringValue: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextField").unwrap(), wrappingLabelWithString : stringValue)
    }
    unsafe fn labelWithAttributedString_(attributedStringValue: NSAttributedString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextField").unwrap(), labelWithAttributedString : attributedStringValue)
    }
    unsafe fn textFieldWithString_(stringValue: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextField").unwrap(), textFieldWithString : stringValue)
    }
}
pub trait NSTextField_NSTextFieldAttributedStringMethods: Sized + std::ops::Deref {
    unsafe fn allowsEditingTextAttributes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsEditingTextAttributes)
    }
    unsafe fn setAllowsEditingTextAttributes_(&self, allowsEditingTextAttributes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsEditingTextAttributes : allowsEditingTextAttributes)
    }
    unsafe fn importsGraphics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, importsGraphics)
    }
    unsafe fn setImportsGraphics_(&self, importsGraphics: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImportsGraphics : importsGraphics)
    }
}
pub trait NSTextField_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setTitleWithMnemonic_(&self, stringWithAmpersand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleWithMnemonic : stringWithAmpersand)
    }
}
pub trait NSMutableAttributedString_NSAttributedStringAttributeFixing:
    Sized + std::ops::Deref
{
    unsafe fn fixAttributesInRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fixAttributesInRange : range)
    }
}
pub trait NSAttributedString_NSAttributedStringDocumentFormats: Sized + std::ops::Deref {
    unsafe fn initWithURL_options_documentAttributes_error_(
        &self,
        url: NSURL,
        options: NSDictionary,
        dict: *mut NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, options : options, documentAttributes : dict, error : error)
    }
    unsafe fn initWithData_options_documentAttributes_error_(
        &self,
        data: NSData,
        options: NSDictionary,
        dict: *mut NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, options : options, documentAttributes : dict, error : error)
    }
    unsafe fn dataFromRange_documentAttributes_error_(
        &self,
        range: NSRange,
        dict: NSDictionary,
        error: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dataFromRange : range, documentAttributes : dict, error : error)
    }
    unsafe fn fileWrapperFromRange_documentAttributes_error_(
        &self,
        range: NSRange,
        dict: NSDictionary,
        error: *mut NSError,
    ) -> NSFileWrapper
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileWrapperFromRange : range, documentAttributes : dict, error : error)
    }
}
pub trait NSMutableAttributedString_NSMutableAttributedStringDocumentFormats:
    Sized + std::ops::Deref
{
    unsafe fn readFromURL_options_documentAttributes_error_(
        &self,
        url: NSURL,
        opts: NSDictionary,
        dict: *mut NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readFromURL : url, options : opts, documentAttributes : dict, error : error)
    }
    unsafe fn readFromData_options_documentAttributes_error_(
        &self,
        data: NSData,
        opts: NSDictionary,
        dict: *mut NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readFromData : data, options : opts, documentAttributes : dict, error : error)
    }
}
pub trait NSAttributedString_NSAttributedStringKitAdditions: Sized + std::ops::Deref {
    unsafe fn containsAttachmentsInRange_(&self, range: NSRange) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsAttachmentsInRange : range)
    }
    unsafe fn prefersRTFDInRange_(&self, range: NSRange) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prefersRTFDInRange : range)
    }
}
pub trait NSMutableAttributedString_NSAttributedStringAppKitAttributeFixing:
    Sized + std::ops::Deref
{
    unsafe fn fixFontAttributeInRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fixFontAttributeInRange : range)
    }
    unsafe fn fixParagraphStyleAttributeInRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fixParagraphStyleAttributeInRange : range)
    }
    unsafe fn fixAttachmentAttributeInRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fixAttachmentAttributeInRange : range)
    }
}
pub trait NSAttributedString_NSAttributedStringAppKitDocumentFormats:
    Sized + std::ops::Deref
{
    unsafe fn initWithRTF_documentAttributes_(
        &self,
        data: NSData,
        dict: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRTF : data, documentAttributes : dict)
    }
    unsafe fn initWithRTFD_documentAttributes_(
        &self,
        data: NSData,
        dict: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRTFD : data, documentAttributes : dict)
    }
    unsafe fn initWithHTML_documentAttributes_(
        &self,
        data: NSData,
        dict: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHTML : data, documentAttributes : dict)
    }
    unsafe fn initWithHTML_baseURL_documentAttributes_(
        &self,
        data: NSData,
        base: NSURL,
        dict: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHTML : data, baseURL : base, documentAttributes : dict)
    }
    unsafe fn initWithDocFormat_documentAttributes_(
        &self,
        data: NSData,
        dict: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDocFormat : data, documentAttributes : dict)
    }
    unsafe fn initWithHTML_options_documentAttributes_(
        &self,
        data: NSData,
        options: NSDictionary,
        dict: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHTML : data, options : options, documentAttributes : dict)
    }
    unsafe fn initWithRTFDFileWrapper_documentAttributes_(
        &self,
        wrapper: NSFileWrapper,
        dict: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRTFDFileWrapper : wrapper, documentAttributes : dict)
    }
    unsafe fn RTFFromRange_documentAttributes_(&self, range: NSRange, dict: NSDictionary) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, RTFFromRange : range, documentAttributes : dict)
    }
    unsafe fn RTFDFromRange_documentAttributes_(&self, range: NSRange, dict: NSDictionary) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, RTFDFromRange : range, documentAttributes : dict)
    }
    unsafe fn RTFDFileWrapperFromRange_documentAttributes_(
        &self,
        range: NSRange,
        dict: NSDictionary,
    ) -> NSFileWrapper
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, RTFDFileWrapperFromRange : range, documentAttributes : dict)
    }
    unsafe fn docFormatFromRange_documentAttributes_(
        &self,
        range: NSRange,
        dict: NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, docFormatFromRange : range, documentAttributes : dict)
    }
}
pub trait NSAttributedString_NSAttributedStringAppKitAdditions: Sized + std::ops::Deref {
    unsafe fn fontAttributesInRange_(&self, range: NSRange) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fontAttributesInRange : range)
    }
    unsafe fn rulerAttributesInRange_(&self, range: NSRange) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerAttributesInRange : range)
    }
    unsafe fn lineBreakBeforeIndex_withinRange_(
        &self,
        location: NSUInteger,
        aRange: NSRange,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineBreakBeforeIndex : location, withinRange : aRange)
    }
    unsafe fn lineBreakByHyphenatingBeforeIndex_withinRange_(
        &self,
        location: NSUInteger,
        aRange: NSRange,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineBreakByHyphenatingBeforeIndex : location, withinRange : aRange)
    }
    unsafe fn doubleClickAtIndex_(&self, location: NSUInteger) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, doubleClickAtIndex : location)
    }
    unsafe fn nextWordFromIndex_forward_(&self, location: NSUInteger, isForward: BOOL) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nextWordFromIndex : location, forward : isForward)
    }
    unsafe fn rangeOfTextBlock_atIndex_(&self, block: NSTextBlock, location: NSUInteger) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rangeOfTextBlock : block, atIndex : location)
    }
    unsafe fn rangeOfTextTable_atIndex_(&self, table: NSTextTable, location: NSUInteger) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rangeOfTextTable : table, atIndex : location)
    }
    unsafe fn rangeOfTextList_atIndex_(&self, list: NSTextList, location: NSUInteger) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rangeOfTextList : list, atIndex : location)
    }
    unsafe fn itemNumberInTextList_atIndex_(
        &self,
        list: NSTextList,
        location: NSUInteger,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, itemNumberInTextList : list, atIndex : location)
    }
}
pub trait NSAttributedString_NSAttributedStringPasteboardAdditions:
    Sized + std::ops::Deref
{
    unsafe fn textTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), textTypes)
    }
    unsafe fn textUnfilteredTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), textUnfilteredTypes)
    }
}
pub trait NSMutableAttributedString_NSMutableAttributedStringAppKitAdditions:
    Sized + std::ops::Deref
{
    unsafe fn superscriptRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, superscriptRange : range)
    }
    unsafe fn subscriptRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, subscriptRange : range)
    }
    unsafe fn unscriptRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unscriptRange : range)
    }
    unsafe fn applyFontTraits_range_(&self, traitMask: NSFontTraitMask, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyFontTraits : traitMask, range : range)
    }
    unsafe fn setAlignment_range_(&self, alignment: NSTextAlignment, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlignment : alignment, range : range)
    }
    unsafe fn setBaseWritingDirection_range_(
        &self,
        writingDirection: NSWritingDirection,
        range: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaseWritingDirection : writingDirection, range : range)
    }
}
pub trait NSAttributedString_NSDeprecatedKitAdditions: Sized + std::ops::Deref {
    unsafe fn initWithURL_documentAttributes_(
        &self,
        url: NSURL,
        dict: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, documentAttributes : dict)
    }
    unsafe fn initWithPath_documentAttributes_(
        &self,
        path: NSString,
        dict: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPath : path, documentAttributes : dict)
    }
    unsafe fn URLAtIndex_effectiveRange_(
        &self,
        location: NSUInteger,
        effectiveRange: NSRangePointer,
    ) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, URLAtIndex : location, effectiveRange : effectiveRange)
    }
    unsafe fn containsAttachments(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containsAttachments)
    }
    unsafe fn textFileTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), textFileTypes)
    }
    unsafe fn textPasteboardTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), textPasteboardTypes)
    }
    unsafe fn textUnfilteredFileTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), textUnfilteredFileTypes)
    }
    unsafe fn textUnfilteredPasteboardTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), textUnfilteredPasteboardTypes)
    }
}
pub trait NSMutableAttributedString_NSDeprecatedKitAdditions: Sized + std::ops::Deref {
    unsafe fn readFromURL_options_documentAttributes_(
        &self,
        url: NSURL,
        options: NSDictionary,
        dict: *mut NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readFromURL : url, options : options, documentAttributes : dict)
    }
    unsafe fn readFromData_options_documentAttributes_(
        &self,
        data: NSData,
        options: NSDictionary,
        dict: *mut NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readFromData : data, options : options, documentAttributes : dict)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSTextStorage(pub id);
impl std::ops::Deref for NSTextStorage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSTextStorage {}
impl NSTextStorage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextStorage").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NSTextStorage {}
impl INSMutableAttributedString for NSTextStorage {}
impl std::convert::TryFrom<NSMutableAttributedString> for NSTextStorage {
    type Error = &'static str;
    fn try_from(parent: NSMutableAttributedString) -> Result<NSTextStorage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSTextStorage").unwrap()) };
        if is_kind_of {
            Ok(NSTextStorage(parent.0))
        } else {
            Err("This NSMutableAttributedString cannot be downcasted to NSTextStorage")
        }
    }
}
impl INSAttributedString for NSTextStorage {}
impl PNSCopying for NSTextStorage {}
impl PNSMutableCopying for NSTextStorage {}
impl INSObject for NSTextStorage {}
impl PNSObject for NSTextStorage {}
impl INSTextStorage for NSTextStorage {}
pub trait INSTextStorage: Sized + std::ops::Deref {
    unsafe fn addLayoutManager_(&self, aLayoutManager: NSLayoutManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addLayoutManager : aLayoutManager)
    }
    unsafe fn removeLayoutManager_(&self, aLayoutManager: NSLayoutManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeLayoutManager : aLayoutManager)
    }
    unsafe fn edited_range_changeInLength_(
        &self,
        editedMask: NSTextStorageEditActions,
        editedRange: NSRange,
        delta: NSInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, edited : editedMask, range : editedRange, changeInLength : delta)
    }
    unsafe fn processEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, processEditing)
    }
    unsafe fn invalidateAttributesInRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateAttributesInRange : range)
    }
    unsafe fn ensureAttributesAreFixedInRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ensureAttributesAreFixedInRange : range)
    }
    unsafe fn layoutManagers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutManagers)
    }
    unsafe fn editedMask(&self) -> NSTextStorageEditActions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, editedMask)
    }
    unsafe fn editedRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, editedRange)
    }
    unsafe fn changeInLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeInLength)
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
    unsafe fn fixesAttributesLazily(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fixesAttributesLazily)
    }
    unsafe fn textStorageObserver(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textStorageObserver)
    }
    unsafe fn setTextStorageObserver_(&self, textStorageObserver: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextStorageObserver : textStorageObserver)
    }
}
pub trait NSObject_NSDeprecatedTextStorageDelegateInterface: Sized + std::ops::Deref {
    unsafe fn textStorageWillProcessEditing_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textStorageWillProcessEditing : notification)
    }
    unsafe fn textStorageDidProcessEditing_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textStorageDidProcessEditing : notification)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSLayoutManager(pub id);
impl std::ops::Deref for NSLayoutManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSLayoutManager {}
impl NSLayoutManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSLayoutManager").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NSLayoutManager {}
impl INSObject for NSLayoutManager {}
impl PNSObject for NSLayoutManager {}
impl std::convert::TryFrom<NSObject> for NSLayoutManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSLayoutManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSLayoutManager").unwrap()) };
        if is_kind_of {
            Ok(NSLayoutManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSLayoutManager")
        }
    }
}
impl INSLayoutManager for NSLayoutManager {}
pub trait INSLayoutManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
    unsafe fn replaceTextStorage_(&self, newTextStorage: NSTextStorage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceTextStorage : newTextStorage)
    }
    unsafe fn addTextContainer_(&self, container: NSTextContainer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTextContainer : container)
    }
    unsafe fn insertTextContainer_atIndex_(&self, container: NSTextContainer, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertTextContainer : container, atIndex : index)
    }
    unsafe fn removeTextContainerAtIndex_(&self, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTextContainerAtIndex : index)
    }
    unsafe fn textContainerChangedGeometry_(&self, container: NSTextContainer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textContainerChangedGeometry : container)
    }
    unsafe fn textContainerChangedTextView_(&self, container: NSTextContainer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textContainerChangedTextView : container)
    }
    unsafe fn invalidateGlyphsForCharacterRange_changeInLength_actualCharacterRange_(
        &self,
        charRange: NSRange,
        delta: NSInteger,
        actualCharRange: NSRangePointer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateGlyphsForCharacterRange : charRange, changeInLength : delta, actualCharacterRange : actualCharRange)
    }
    unsafe fn invalidateLayoutForCharacterRange_actualCharacterRange_(
        &self,
        charRange: NSRange,
        actualCharRange: NSRangePointer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateLayoutForCharacterRange : charRange, actualCharacterRange : actualCharRange)
    }
    unsafe fn invalidateDisplayForCharacterRange_(&self, charRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateDisplayForCharacterRange : charRange)
    }
    unsafe fn invalidateDisplayForGlyphRange_(&self, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateDisplayForGlyphRange : glyphRange)
    }
    unsafe fn processEditingForTextStorage_edited_range_changeInLength_invalidatedRange_(
        &self,
        textStorage: NSTextStorage,
        editMask: NSTextStorageEditActions,
        newCharRange: NSRange,
        delta: NSInteger,
        invalidatedCharRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processEditingForTextStorage : textStorage, edited : editMask, range : newCharRange, changeInLength : delta, invalidatedRange : invalidatedCharRange)
    }
    unsafe fn ensureGlyphsForCharacterRange_(&self, charRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ensureGlyphsForCharacterRange : charRange)
    }
    unsafe fn ensureGlyphsForGlyphRange_(&self, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ensureGlyphsForGlyphRange : glyphRange)
    }
    unsafe fn ensureLayoutForCharacterRange_(&self, charRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ensureLayoutForCharacterRange : charRange)
    }
    unsafe fn ensureLayoutForGlyphRange_(&self, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ensureLayoutForGlyphRange : glyphRange)
    }
    unsafe fn ensureLayoutForTextContainer_(&self, container: NSTextContainer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ensureLayoutForTextContainer : container)
    }
    unsafe fn ensureLayoutForBoundingRect_inTextContainer_(
        &self,
        bounds: NSRect,
        container: NSTextContainer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ensureLayoutForBoundingRect : bounds, inTextContainer : container)
    }
    unsafe fn setGlyphs_properties_characterIndexes_font_forGlyphRange_(
        &self,
        glyphs: *const CGGlyph,
        props: *const NSGlyphProperty,
        charIndexes: *const NSUInteger,
        aFont: NSFont,
        glyphRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGlyphs : glyphs, properties : props, characterIndexes : charIndexes, font : aFont, forGlyphRange : glyphRange)
    }
    unsafe fn CGGlyphAtIndex_isValidIndex_(
        &self,
        glyphIndex: NSUInteger,
        isValidIndex: *mut BOOL,
    ) -> CGGlyph
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, CGGlyphAtIndex : glyphIndex, isValidIndex : isValidIndex)
    }
    unsafe fn CGGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> CGGlyph
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, CGGlyphAtIndex : glyphIndex)
    }
    unsafe fn isValidGlyphIndex_(&self, glyphIndex: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isValidGlyphIndex : glyphIndex)
    }
    unsafe fn propertyForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> NSGlyphProperty
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, propertyForGlyphAtIndex : glyphIndex)
    }
    unsafe fn characterIndexForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, characterIndexForGlyphAtIndex : glyphIndex)
    }
    unsafe fn glyphIndexForCharacterAtIndex_(&self, charIndex: NSUInteger) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphIndexForCharacterAtIndex : charIndex)
    }
    unsafe fn getGlyphsInRange_glyphs_properties_characterIndexes_bidiLevels_(
        &self,
        glyphRange: NSRange,
        glyphBuffer: *mut CGGlyph,
        props: *mut NSGlyphProperty,
        charIndexBuffer: *mut NSUInteger,
        bidiLevelBuffer: *mut ::std::os::raw::c_uchar,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getGlyphsInRange : glyphRange, glyphs : glyphBuffer, properties : props, characterIndexes : charIndexBuffer, bidiLevels : bidiLevelBuffer)
    }
    unsafe fn setTextContainer_forGlyphRange_(
        &self,
        container: NSTextContainer,
        glyphRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextContainer : container, forGlyphRange : glyphRange)
    }
    unsafe fn setLineFragmentRect_forGlyphRange_usedRect_(
        &self,
        fragmentRect: NSRect,
        glyphRange: NSRange,
        usedRect: NSRect,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineFragmentRect : fragmentRect, forGlyphRange : glyphRange, usedRect : usedRect)
    }
    unsafe fn setExtraLineFragmentRect_usedRect_textContainer_(
        &self,
        fragmentRect: NSRect,
        usedRect: NSRect,
        container: NSTextContainer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtraLineFragmentRect : fragmentRect, usedRect : usedRect, textContainer : container)
    }
    unsafe fn setLocation_forStartOfGlyphRange_(&self, location: NSPoint, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocation : location, forStartOfGlyphRange : glyphRange)
    }
    unsafe fn setNotShownAttribute_forGlyphAtIndex_(&self, flag: BOOL, glyphIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotShownAttribute : flag, forGlyphAtIndex : glyphIndex)
    }
    unsafe fn setDrawsOutsideLineFragment_forGlyphAtIndex_(
        &self,
        flag: BOOL,
        glyphIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawsOutsideLineFragment : flag, forGlyphAtIndex : glyphIndex)
    }
    unsafe fn setAttachmentSize_forGlyphRange_(&self, attachmentSize: NSSize, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachmentSize : attachmentSize, forGlyphRange : glyphRange)
    }
    unsafe fn getFirstUnlaidCharacterIndex_glyphIndex_(
        &self,
        charIndex: *mut NSUInteger,
        glyphIndex: *mut NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getFirstUnlaidCharacterIndex : charIndex, glyphIndex : glyphIndex)
    }
    unsafe fn firstUnlaidCharacterIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstUnlaidCharacterIndex)
    }
    unsafe fn firstUnlaidGlyphIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstUnlaidGlyphIndex)
    }
    unsafe fn textContainerForGlyphAtIndex_effectiveRange_(
        &self,
        glyphIndex: NSUInteger,
        effectiveGlyphRange: NSRangePointer,
    ) -> NSTextContainer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textContainerForGlyphAtIndex : glyphIndex, effectiveRange : effectiveGlyphRange)
    }
    unsafe fn textContainerForGlyphAtIndex_effectiveRange_withoutAdditionalLayout_(
        &self,
        glyphIndex: NSUInteger,
        effectiveGlyphRange: NSRangePointer,
        flag: BOOL,
    ) -> NSTextContainer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textContainerForGlyphAtIndex : glyphIndex, effectiveRange : effectiveGlyphRange, withoutAdditionalLayout : flag)
    }
    unsafe fn usedRectForTextContainer_(&self, container: NSTextContainer) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, usedRectForTextContainer : container)
    }
    unsafe fn lineFragmentRectForGlyphAtIndex_effectiveRange_(
        &self,
        glyphIndex: NSUInteger,
        effectiveGlyphRange: NSRangePointer,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineFragmentRectForGlyphAtIndex : glyphIndex, effectiveRange : effectiveGlyphRange)
    }
    unsafe fn lineFragmentRectForGlyphAtIndex_effectiveRange_withoutAdditionalLayout_(
        &self,
        glyphIndex: NSUInteger,
        effectiveGlyphRange: NSRangePointer,
        flag: BOOL,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineFragmentRectForGlyphAtIndex : glyphIndex, effectiveRange : effectiveGlyphRange, withoutAdditionalLayout : flag)
    }
    unsafe fn lineFragmentUsedRectForGlyphAtIndex_effectiveRange_(
        &self,
        glyphIndex: NSUInteger,
        effectiveGlyphRange: NSRangePointer,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineFragmentUsedRectForGlyphAtIndex : glyphIndex, effectiveRange : effectiveGlyphRange)
    }
    unsafe fn lineFragmentUsedRectForGlyphAtIndex_effectiveRange_withoutAdditionalLayout_(
        &self,
        glyphIndex: NSUInteger,
        effectiveGlyphRange: NSRangePointer,
        flag: BOOL,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineFragmentUsedRectForGlyphAtIndex : glyphIndex, effectiveRange : effectiveGlyphRange, withoutAdditionalLayout : flag)
    }
    unsafe fn locationForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationForGlyphAtIndex : glyphIndex)
    }
    unsafe fn notShownAttributeForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notShownAttributeForGlyphAtIndex : glyphIndex)
    }
    unsafe fn drawsOutsideLineFragmentForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawsOutsideLineFragmentForGlyphAtIndex : glyphIndex)
    }
    unsafe fn attachmentSizeForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attachmentSizeForGlyphAtIndex : glyphIndex)
    }
    unsafe fn truncatedGlyphRangeInLineFragmentForGlyphAtIndex_(
        &self,
        glyphIndex: NSUInteger,
    ) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, truncatedGlyphRangeInLineFragmentForGlyphAtIndex : glyphIndex)
    }
    unsafe fn glyphRangeForCharacterRange_actualCharacterRange_(
        &self,
        charRange: NSRange,
        actualCharRange: NSRangePointer,
    ) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphRangeForCharacterRange : charRange, actualCharacterRange : actualCharRange)
    }
    unsafe fn characterRangeForGlyphRange_actualGlyphRange_(
        &self,
        glyphRange: NSRange,
        actualGlyphRange: NSRangePointer,
    ) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, characterRangeForGlyphRange : glyphRange, actualGlyphRange : actualGlyphRange)
    }
    unsafe fn glyphRangeForTextContainer_(&self, container: NSTextContainer) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphRangeForTextContainer : container)
    }
    unsafe fn rangeOfNominallySpacedGlyphsContainingIndex_(&self, glyphIndex: NSUInteger) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rangeOfNominallySpacedGlyphsContainingIndex : glyphIndex)
    }
    unsafe fn boundingRectForGlyphRange_inTextContainer_(
        &self,
        glyphRange: NSRange,
        container: NSTextContainer,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingRectForGlyphRange : glyphRange, inTextContainer : container)
    }
    unsafe fn glyphRangeForBoundingRect_inTextContainer_(
        &self,
        bounds: NSRect,
        container: NSTextContainer,
    ) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphRangeForBoundingRect : bounds, inTextContainer : container)
    }
    unsafe fn glyphRangeForBoundingRectWithoutAdditionalLayout_inTextContainer_(
        &self,
        bounds: NSRect,
        container: NSTextContainer,
    ) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphRangeForBoundingRectWithoutAdditionalLayout : bounds, inTextContainer : container)
    }
    unsafe fn glyphIndexForPoint_inTextContainer_fractionOfDistanceThroughGlyph_(
        &self,
        point: NSPoint,
        container: NSTextContainer,
        partialFraction: *mut CGFloat,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphIndexForPoint : point, inTextContainer : container, fractionOfDistanceThroughGlyph : partialFraction)
    }
    unsafe fn glyphIndexForPoint_inTextContainer_(
        &self,
        point: NSPoint,
        container: NSTextContainer,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphIndexForPoint : point, inTextContainer : container)
    }
    unsafe fn fractionOfDistanceThroughGlyphForPoint_inTextContainer_(
        &self,
        point: NSPoint,
        container: NSTextContainer,
    ) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fractionOfDistanceThroughGlyphForPoint : point, inTextContainer : container)
    }
    unsafe fn characterIndexForPoint_inTextContainer_fractionOfDistanceBetweenInsertionPoints_(
        &self,
        point: NSPoint,
        container: NSTextContainer,
        partialFraction: *mut CGFloat,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, characterIndexForPoint : point, inTextContainer : container, fractionOfDistanceBetweenInsertionPoints : partialFraction)
    }
    unsafe fn getLineFragmentInsertionPointsForCharacterAtIndex_alternatePositions_inDisplayOrder_positions_characterIndexes_(
        &self,
        charIndex: NSUInteger,
        aFlag: BOOL,
        dFlag: BOOL,
        positions: *mut CGFloat,
        charIndexes: *mut NSUInteger,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getLineFragmentInsertionPointsForCharacterAtIndex : charIndex, alternatePositions : aFlag, inDisplayOrder : dFlag, positions : positions, characterIndexes : charIndexes)
    }
    unsafe fn enumerateLineFragmentsForGlyphRange_usingBlock_(
        &self,
        glyphRange: NSRange,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateLineFragmentsForGlyphRange : glyphRange, usingBlock : block)
    }
    unsafe fn enumerateEnclosingRectsForGlyphRange_withinSelectedGlyphRange_inTextContainer_usingBlock_(
        &self,
        glyphRange: NSRange,
        selectedRange: NSRange,
        textContainer: NSTextContainer,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateEnclosingRectsForGlyphRange : glyphRange, withinSelectedGlyphRange : selectedRange, inTextContainer : textContainer, usingBlock : block)
    }
    unsafe fn drawBackgroundForGlyphRange_atPoint_(&self, glyphsToShow: NSRange, origin: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawBackgroundForGlyphRange : glyphsToShow, atPoint : origin)
    }
    unsafe fn drawGlyphsForGlyphRange_atPoint_(&self, glyphsToShow: NSRange, origin: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawGlyphsForGlyphRange : glyphsToShow, atPoint : origin)
    }
    unsafe fn showCGGlyphs_positions_count_font_textMatrix_attributes_inContext_(
        &self,
        glyphs: *const CGGlyph,
        positions: *const CGPoint,
        glyphCount: NSInteger,
        font: NSFont,
        textMatrix: CGAffineTransform,
        attributes: NSDictionary,
        CGContext: CGContextRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showCGGlyphs : glyphs, positions : positions, count : glyphCount, font : font, textMatrix : textMatrix, attributes : attributes, inContext : CGContext)
    }
    unsafe fn fillBackgroundRectArray_count_forCharacterRange_color_(
        &self,
        rectArray: *const NSRect,
        rectCount: NSUInteger,
        charRange: NSRange,
        color: NSColor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fillBackgroundRectArray : rectArray, count : rectCount, forCharacterRange : charRange, color : color)
    }
    unsafe fn drawUnderlineForGlyphRange_underlineType_baselineOffset_lineFragmentRect_lineFragmentGlyphRange_containerOrigin_(
        &self,
        glyphRange: NSRange,
        underlineVal: NSUnderlineStyle,
        baselineOffset: CGFloat,
        lineRect: NSRect,
        lineGlyphRange: NSRange,
        containerOrigin: NSPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawUnderlineForGlyphRange : glyphRange, underlineType : underlineVal, baselineOffset : baselineOffset, lineFragmentRect : lineRect, lineFragmentGlyphRange : lineGlyphRange, containerOrigin : containerOrigin)
    }
    unsafe fn underlineGlyphRange_underlineType_lineFragmentRect_lineFragmentGlyphRange_containerOrigin_(
        &self,
        glyphRange: NSRange,
        underlineVal: NSUnderlineStyle,
        lineRect: NSRect,
        lineGlyphRange: NSRange,
        containerOrigin: NSPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, underlineGlyphRange : glyphRange, underlineType : underlineVal, lineFragmentRect : lineRect, lineFragmentGlyphRange : lineGlyphRange, containerOrigin : containerOrigin)
    }
    unsafe fn drawStrikethroughForGlyphRange_strikethroughType_baselineOffset_lineFragmentRect_lineFragmentGlyphRange_containerOrigin_(
        &self,
        glyphRange: NSRange,
        strikethroughVal: NSUnderlineStyle,
        baselineOffset: CGFloat,
        lineRect: NSRect,
        lineGlyphRange: NSRange,
        containerOrigin: NSPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawStrikethroughForGlyphRange : glyphRange, strikethroughType : strikethroughVal, baselineOffset : baselineOffset, lineFragmentRect : lineRect, lineFragmentGlyphRange : lineGlyphRange, containerOrigin : containerOrigin)
    }
    unsafe fn strikethroughGlyphRange_strikethroughType_lineFragmentRect_lineFragmentGlyphRange_containerOrigin_(
        &self,
        glyphRange: NSRange,
        strikethroughVal: NSUnderlineStyle,
        lineRect: NSRect,
        lineGlyphRange: NSRange,
        containerOrigin: NSPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, strikethroughGlyphRange : glyphRange, strikethroughType : strikethroughVal, lineFragmentRect : lineRect, lineFragmentGlyphRange : lineGlyphRange, containerOrigin : containerOrigin)
    }
    unsafe fn showAttachmentCell_inRect_characterIndex_(
        &self,
        cell: NSCell,
        rect: NSRect,
        attachmentIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showAttachmentCell : cell, inRect : rect, characterIndex : attachmentIndex)
    }
    unsafe fn setLayoutRect_forTextBlock_glyphRange_(
        &self,
        rect: NSRect,
        block: NSTextBlock,
        glyphRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayoutRect : rect, forTextBlock : block, glyphRange : glyphRange)
    }
    unsafe fn setBoundsRect_forTextBlock_glyphRange_(
        &self,
        rect: NSRect,
        block: NSTextBlock,
        glyphRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundsRect : rect, forTextBlock : block, glyphRange : glyphRange)
    }
    unsafe fn layoutRectForTextBlock_glyphRange_(
        &self,
        block: NSTextBlock,
        glyphRange: NSRange,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutRectForTextBlock : block, glyphRange : glyphRange)
    }
    unsafe fn boundsRectForTextBlock_glyphRange_(
        &self,
        block: NSTextBlock,
        glyphRange: NSRange,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundsRectForTextBlock : block, glyphRange : glyphRange)
    }
    unsafe fn layoutRectForTextBlock_atIndex_effectiveRange_(
        &self,
        block: NSTextBlock,
        glyphIndex: NSUInteger,
        effectiveGlyphRange: NSRangePointer,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutRectForTextBlock : block, atIndex : glyphIndex, effectiveRange : effectiveGlyphRange)
    }
    unsafe fn boundsRectForTextBlock_atIndex_effectiveRange_(
        &self,
        block: NSTextBlock,
        glyphIndex: NSUInteger,
        effectiveGlyphRange: NSRangePointer,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundsRectForTextBlock : block, atIndex : glyphIndex, effectiveRange : effectiveGlyphRange)
    }
    unsafe fn temporaryAttributesAtCharacterIndex_effectiveRange_(
        &self,
        charIndex: NSUInteger,
        effectiveCharRange: NSRangePointer,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, temporaryAttributesAtCharacterIndex : charIndex, effectiveRange : effectiveCharRange)
    }
    unsafe fn setTemporaryAttributes_forCharacterRange_(
        &self,
        attrs: NSDictionary,
        charRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTemporaryAttributes : attrs, forCharacterRange : charRange)
    }
    unsafe fn addTemporaryAttributes_forCharacterRange_(
        &self,
        attrs: NSDictionary,
        charRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTemporaryAttributes : attrs, forCharacterRange : charRange)
    }
    unsafe fn removeTemporaryAttribute_forCharacterRange_(
        &self,
        attrName: NSString,
        charRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTemporaryAttribute : attrName, forCharacterRange : charRange)
    }
    unsafe fn temporaryAttribute_atCharacterIndex_effectiveRange_(
        &self,
        attrName: NSString,
        location: NSUInteger,
        range: NSRangePointer,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, temporaryAttribute : attrName, atCharacterIndex : location, effectiveRange : range)
    }
    unsafe fn temporaryAttribute_atCharacterIndex_longestEffectiveRange_inRange_(
        &self,
        attrName: NSString,
        location: NSUInteger,
        range: NSRangePointer,
        rangeLimit: NSRange,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, temporaryAttribute : attrName, atCharacterIndex : location, longestEffectiveRange : range, inRange : rangeLimit)
    }
    unsafe fn temporaryAttributesAtCharacterIndex_longestEffectiveRange_inRange_(
        &self,
        location: NSUInteger,
        range: NSRangePointer,
        rangeLimit: NSRange,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, temporaryAttributesAtCharacterIndex : location, longestEffectiveRange : range, inRange : rangeLimit)
    }
    unsafe fn addTemporaryAttribute_value_forCharacterRange_(
        &self,
        attrName: NSString,
        value: id,
        charRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTemporaryAttribute : attrName, value : value, forCharacterRange : charRange)
    }
    unsafe fn defaultLineHeightForFont_(&self, theFont: NSFont) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, defaultLineHeightForFont : theFont)
    }
    unsafe fn defaultBaselineOffsetForFont_(&self, theFont: NSFont) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, defaultBaselineOffsetForFont : theFont)
    }
    unsafe fn textStorage(&self) -> NSTextStorage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textStorage)
    }
    unsafe fn setTextStorage_(&self, textStorage: NSTextStorage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextStorage : textStorage)
    }
    unsafe fn textContainers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textContainers)
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
    unsafe fn showsInvisibleCharacters(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsInvisibleCharacters)
    }
    unsafe fn setShowsInvisibleCharacters_(&self, showsInvisibleCharacters: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsInvisibleCharacters : showsInvisibleCharacters)
    }
    unsafe fn showsControlCharacters(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsControlCharacters)
    }
    unsafe fn setShowsControlCharacters_(&self, showsControlCharacters: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsControlCharacters : showsControlCharacters)
    }
    unsafe fn usesDefaultHyphenation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesDefaultHyphenation)
    }
    unsafe fn setUsesDefaultHyphenation_(&self, usesDefaultHyphenation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesDefaultHyphenation : usesDefaultHyphenation)
    }
    unsafe fn usesFontLeading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesFontLeading)
    }
    unsafe fn setUsesFontLeading_(&self, usesFontLeading: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesFontLeading : usesFontLeading)
    }
    unsafe fn allowsNonContiguousLayout(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsNonContiguousLayout)
    }
    unsafe fn setAllowsNonContiguousLayout_(&self, allowsNonContiguousLayout: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsNonContiguousLayout : allowsNonContiguousLayout)
    }
    unsafe fn hasNonContiguousLayout(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasNonContiguousLayout)
    }
    unsafe fn limitsLayoutForSuspiciousContents(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, limitsLayoutForSuspiciousContents)
    }
    unsafe fn setLimitsLayoutForSuspiciousContents_(&self, limitsLayoutForSuspiciousContents: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLimitsLayoutForSuspiciousContents : limitsLayoutForSuspiciousContents)
    }
    unsafe fn backgroundLayoutEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundLayoutEnabled)
    }
    unsafe fn setBackgroundLayoutEnabled_(&self, backgroundLayoutEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundLayoutEnabled : backgroundLayoutEnabled)
    }
    unsafe fn defaultAttachmentScaling(&self) -> NSImageScaling
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultAttachmentScaling)
    }
    unsafe fn setDefaultAttachmentScaling_(&self, defaultAttachmentScaling: NSImageScaling)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultAttachmentScaling : defaultAttachmentScaling)
    }
    unsafe fn typesetter(&self) -> NSTypesetter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typesetter)
    }
    unsafe fn setTypesetter_(&self, typesetter: NSTypesetter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTypesetter : typesetter)
    }
    unsafe fn typesetterBehavior(&self) -> NSTypesetterBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typesetterBehavior)
    }
    unsafe fn setTypesetterBehavior_(&self, typesetterBehavior: NSTypesetterBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTypesetterBehavior : typesetterBehavior)
    }
    unsafe fn numberOfGlyphs(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfGlyphs)
    }
    unsafe fn extraLineFragmentRect(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extraLineFragmentRect)
    }
    unsafe fn extraLineFragmentUsedRect(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extraLineFragmentUsedRect)
    }
    unsafe fn extraLineFragmentTextContainer(&self) -> NSTextContainer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extraLineFragmentTextContainer)
    }
}
impl NSLayoutManager_NSTextViewSupport for NSLayoutManager {}
pub trait NSLayoutManager_NSTextViewSupport: Sized + std::ops::Deref {
    unsafe fn rulerMarkersForTextView_paragraphStyle_ruler_(
        &self,
        view: NSTextView,
        style: NSParagraphStyle,
        ruler: NSRulerView,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerMarkersForTextView : view, paragraphStyle : style, ruler : ruler)
    }
    unsafe fn rulerAccessoryViewForTextView_paragraphStyle_ruler_enabled_(
        &self,
        view: NSTextView,
        style: NSParagraphStyle,
        ruler: NSRulerView,
        isEnabled: BOOL,
    ) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerAccessoryViewForTextView : view, paragraphStyle : style, ruler : ruler, enabled : isEnabled)
    }
    unsafe fn layoutManagerOwnsFirstResponderInWindow_(&self, window: NSWindow) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutManagerOwnsFirstResponderInWindow : window)
    }
    unsafe fn firstTextView(&self) -> NSTextView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstTextView)
    }
    unsafe fn textViewForBeginningOfSelection(&self) -> NSTextView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textViewForBeginningOfSelection)
    }
}
impl NSLayoutManager_NSLayoutManagerDeprecated for NSLayoutManager {}
pub trait NSLayoutManager_NSLayoutManagerDeprecated: Sized + std::ops::Deref {
    unsafe fn glyphAtIndex_isValidIndex_(
        &self,
        glyphIndex: NSUInteger,
        isValidIndex: *mut BOOL,
    ) -> NSGlyph
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphAtIndex : glyphIndex, isValidIndex : isValidIndex)
    }
    unsafe fn glyphAtIndex_(&self, glyphIndex: NSUInteger) -> NSGlyph
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphAtIndex : glyphIndex)
    }
    unsafe fn rectArrayForCharacterRange_withinSelectedCharacterRange_inTextContainer_rectCount_(
        &self,
        charRange: NSRange,
        selCharRange: NSRange,
        container: NSTextContainer,
        rectCount: *mut NSUInteger,
    ) -> NSRectArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectArrayForCharacterRange : charRange, withinSelectedCharacterRange : selCharRange, inTextContainer : container, rectCount : rectCount)
    }
    unsafe fn rectArrayForGlyphRange_withinSelectedGlyphRange_inTextContainer_rectCount_(
        &self,
        glyphRange: NSRange,
        selGlyphRange: NSRange,
        container: NSTextContainer,
        rectCount: *mut NSUInteger,
    ) -> NSRectArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectArrayForGlyphRange : glyphRange, withinSelectedGlyphRange : selGlyphRange, inTextContainer : container, rectCount : rectCount)
    }
    unsafe fn substituteFontForFont_(&self, originalFont: NSFont) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, substituteFontForFont : originalFont)
    }
    unsafe fn insertGlyphs_length_forStartingGlyphAtIndex_characterIndex_(
        &self,
        glyphs: *const NSGlyph,
        length: NSUInteger,
        glyphIndex: NSUInteger,
        charIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertGlyphs : glyphs, length : length, forStartingGlyphAtIndex : glyphIndex, characterIndex : charIndex)
    }
    unsafe fn insertGlyph_atGlyphIndex_characterIndex_(
        &self,
        glyph: NSGlyph,
        glyphIndex: NSUInteger,
        charIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertGlyph : glyph, atGlyphIndex : glyphIndex, characterIndex : charIndex)
    }
    unsafe fn replaceGlyphAtIndex_withGlyph_(&self, glyphIndex: NSUInteger, newGlyph: NSGlyph)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceGlyphAtIndex : glyphIndex, withGlyph : newGlyph)
    }
    unsafe fn deleteGlyphsInRange_(&self, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteGlyphsInRange : glyphRange)
    }
    unsafe fn setCharacterIndex_forGlyphAtIndex_(
        &self,
        charIndex: NSUInteger,
        glyphIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharacterIndex : charIndex, forGlyphAtIndex : glyphIndex)
    }
    unsafe fn setIntAttribute_value_forGlyphAtIndex_(
        &self,
        attributeTag: NSInteger,
        val: NSInteger,
        glyphIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntAttribute : attributeTag, value : val, forGlyphAtIndex : glyphIndex)
    }
    unsafe fn invalidateGlyphsOnLayoutInvalidationForGlyphRange_(&self, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateGlyphsOnLayoutInvalidationForGlyphRange : glyphRange)
    }
    unsafe fn intAttribute_forGlyphAtIndex_(
        &self,
        attributeTag: NSInteger,
        glyphIndex: NSUInteger,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intAttribute : attributeTag, forGlyphAtIndex : glyphIndex)
    }
    unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits_(
        &self,
        glyphRange: NSRange,
        glyphBuffer: *mut NSGlyph,
        charIndexBuffer: *mut NSUInteger,
        inscribeBuffer: *mut NSGlyphInscription,
        elasticBuffer: *mut BOOL,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getGlyphsInRange : glyphRange, glyphs : glyphBuffer, characterIndexes : charIndexBuffer, glyphInscriptions : inscribeBuffer, elasticBits : elasticBuffer)
    }
    unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits_bidiLevels_(
        &self,
        glyphRange: NSRange,
        glyphBuffer: *mut NSGlyph,
        charIndexBuffer: *mut NSUInteger,
        inscribeBuffer: *mut NSGlyphInscription,
        elasticBuffer: *mut BOOL,
        bidiLevelBuffer: *mut ::std::os::raw::c_uchar,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getGlyphsInRange : glyphRange, glyphs : glyphBuffer, characterIndexes : charIndexBuffer, glyphInscriptions : inscribeBuffer, elasticBits : elasticBuffer, bidiLevels : bidiLevelBuffer)
    }
    unsafe fn getGlyphs_range_(&self, glyphArray: *mut NSGlyph, glyphRange: NSRange) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getGlyphs : glyphArray, range : glyphRange)
    }
    unsafe fn invalidateLayoutForCharacterRange_isSoft_actualCharacterRange_(
        &self,
        charRange: NSRange,
        flag: BOOL,
        actualCharRange: NSRangePointer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateLayoutForCharacterRange : charRange, isSoft : flag, actualCharacterRange : actualCharRange)
    }
    unsafe fn textStorage_edited_range_changeInLength_invalidatedRange_(
        &self,
        str_: NSTextStorage,
        editedMask: NSTextStorageEditedOptions,
        newCharRange: NSRange,
        delta: NSInteger,
        invalidatedCharRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textStorage : str_, edited : editedMask, range : newCharRange, changeInLength : delta, invalidatedRange : invalidatedCharRange)
    }
    unsafe fn setLocations_startingGlyphIndexes_count_forGlyphRange_(
        &self,
        locations: NSPointArray,
        glyphIndexes: *mut NSUInteger,
        count: NSUInteger,
        glyphRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocations : locations, startingGlyphIndexes : glyphIndexes, count : count, forGlyphRange : glyphRange)
    }
    unsafe fn showPackedGlyphs_length_glyphRange_atPoint_font_color_printingAdjustment_(
        &self,
        glyphs: *mut ::std::os::raw::c_char,
        glyphLen: NSUInteger,
        glyphRange: NSRange,
        point: NSPoint,
        font: NSFont,
        color: NSColor,
        printingAdjustment: NSSize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showPackedGlyphs : glyphs, length : glyphLen, glyphRange : glyphRange, atPoint : point, font : font, color : color, printingAdjustment : printingAdjustment)
    }
    unsafe fn showCGGlyphs_positions_count_font_matrix_attributes_inContext_(
        &self,
        glyphs: *const CGGlyph,
        positions: *const NSPoint,
        glyphCount: NSUInteger,
        font: NSFont,
        textMatrix: NSAffineTransform,
        attributes: NSDictionary,
        graphicsContext: NSGraphicsContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showCGGlyphs : glyphs, positions : positions, count : glyphCount, font : font, matrix : textMatrix, attributes : attributes, inContext : graphicsContext)
    }
    unsafe fn usesScreenFonts(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesScreenFonts)
    }
    unsafe fn setUsesScreenFonts_(&self, usesScreenFonts: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesScreenFonts : usesScreenFonts)
    }
    unsafe fn hyphenationFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hyphenationFactor)
    }
    unsafe fn setHyphenationFactor_(&self, hyphenationFactor: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHyphenationFactor : hyphenationFactor)
    }
}
impl NSLayoutManager_NSGlyphGeneration for NSLayoutManager {}
pub trait NSLayoutManager_NSGlyphGeneration: Sized + std::ops::Deref {
    unsafe fn glyphGenerator(&self) -> NSGlyphGenerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, glyphGenerator)
    }
    unsafe fn setGlyphGenerator_(&self, glyphGenerator: NSGlyphGenerator)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGlyphGenerator : glyphGenerator)
    }
}
pub trait NSTextContainer_: Sized + std::ops::Deref {
    unsafe fn layoutManager(&self) -> NSLayoutManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutManager)
    }
    unsafe fn setLayoutManager_(&self, layoutManager: NSLayoutManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayoutManager : layoutManager)
    }
    unsafe fn replaceLayoutManager_(&self, newLayoutManager: NSLayoutManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceLayoutManager : newLayoutManager)
    }
    unsafe fn exclusionPaths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exclusionPaths)
    }
    unsafe fn setExclusionPaths_(&self, exclusionPaths: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExclusionPaths : exclusionPaths)
    }
    unsafe fn textView(&self) -> NSTextView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textView)
    }
    unsafe fn setTextView_(&self, textView: NSTextView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextView : textView)
    }
}
pub trait NSTextContainer_NSTextContainerDeprecated: Sized + std::ops::Deref {
    unsafe fn initWithContainerSize_(&self, aContainerSize: NSSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContainerSize : aContainerSize)
    }
    unsafe fn lineFragmentRectForProposedRect_sweepDirection_movementDirection_remainingRect_(
        &self,
        proposedRect: NSRect,
        sweepDirection: NSLineSweepDirection,
        movementDirection: NSLineMovementDirection,
        remainingRect: NSRectPointer,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineFragmentRectForProposedRect : proposedRect, sweepDirection : sweepDirection, movementDirection : movementDirection, remainingRect : remainingRect)
    }
    unsafe fn containsPoint_(&self, point: NSPoint) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsPoint : point)
    }
    unsafe fn containerSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerSize)
    }
    unsafe fn setContainerSize_(&self, containerSize: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContainerSize : containerSize)
    }
}
pub type NSToolbarIdentifier = NSString;
pub type NSToolbarItemIdentifier = NSString;
pub trait NSToolbar_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setConfigurationFromDictionary_(&self, configDict: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfigurationFromDictionary : configDict)
    }
    unsafe fn sizeMode(&self) -> NSToolbarSizeMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeMode)
    }
    unsafe fn setSizeMode_(&self, sizeMode: NSToolbarSizeMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSizeMode : sizeMode)
    }
    unsafe fn centeredItemIdentifier(&self) -> NSToolbarItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centeredItemIdentifier)
    }
    unsafe fn setCenteredItemIdentifier_(&self, centeredItemIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenteredItemIdentifier : centeredItemIdentifier)
    }
    unsafe fn fullScreenAccessoryView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullScreenAccessoryView)
    }
    unsafe fn setFullScreenAccessoryView_(&self, fullScreenAccessoryView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFullScreenAccessoryView : fullScreenAccessoryView)
    }
    unsafe fn fullScreenAccessoryViewMinHeight(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullScreenAccessoryViewMinHeight)
    }
    unsafe fn setFullScreenAccessoryViewMinHeight_(&self, fullScreenAccessoryViewMinHeight: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFullScreenAccessoryViewMinHeight : fullScreenAccessoryViewMinHeight)
    }
    unsafe fn fullScreenAccessoryViewMaxHeight(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullScreenAccessoryViewMaxHeight)
    }
    unsafe fn setFullScreenAccessoryViewMaxHeight_(&self, fullScreenAccessoryViewMaxHeight: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFullScreenAccessoryViewMaxHeight : fullScreenAccessoryViewMaxHeight)
    }
    unsafe fn showsBaselineSeparator(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsBaselineSeparator)
    }
    unsafe fn setShowsBaselineSeparator_(&self, showsBaselineSeparator: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsBaselineSeparator : showsBaselineSeparator)
    }
    unsafe fn configurationDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationDictionary)
    }
}
pub type NSToolbarItemVisibilityPriority = NSInteger;
pub trait NSToolbarItem_: Sized + std::ops::Deref {}
pub trait NSObject_NSToolbarItemValidation: Sized + std::ops::Deref {
    unsafe fn validateToolbarItem_(&self, item: NSToolbarItem) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateToolbarItem : item)
    }
}
pub trait NSView_LayoutRegions: Sized + std::ops::Deref {
    unsafe fn layoutGuideForLayoutRegion_(&self, layoutRegion: NSViewLayoutRegion) -> NSLayoutGuide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutGuideForLayoutRegion : layoutRegion)
    }
    unsafe fn edgeInsetsForLayoutRegion_(&self, layoutRegion: NSViewLayoutRegion) -> NSEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, edgeInsetsForLayoutRegion : layoutRegion)
    }
    unsafe fn rectForLayoutRegion_(&self, layoutRegion: NSViewLayoutRegion) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectForLayoutRegion : layoutRegion)
    }
}
pub trait NSWindowController_NSWindowControllerStoryboardingMethods:
    Sized + std::ops::Deref
{
    unsafe fn storyboard(&self) -> NSStoryboard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storyboard)
    }
}
pub trait NSWindowController_NSWindowControllerDismissing: Sized + std::ops::Deref {
    unsafe fn dismissController_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismissController : sender)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSTextAttachment(pub id);
impl std::ops::Deref for NSTextAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSTextAttachment {}
impl NSTextAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextAttachment").unwrap(), alloc) })
    }
}
impl PNSTextAttachmentLayout for NSTextAttachment {}
impl PNSSecureCoding for NSTextAttachment {}
impl INSObject for NSTextAttachment {}
impl PNSObject for NSTextAttachment {}
impl std::convert::TryFrom<NSObject> for NSTextAttachment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSTextAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSTextAttachment").unwrap()) };
        if is_kind_of {
            Ok(NSTextAttachment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSTextAttachment")
        }
    }
}
impl INSTextAttachment for NSTextAttachment {}
pub trait INSTextAttachment: Sized + std::ops::Deref {
    unsafe fn initWithData_ofType_(&self, contentData: NSData, uti: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : contentData, ofType : uti)
    }
    unsafe fn initWithFileWrapper_(&self, fileWrapper: NSFileWrapper) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileWrapper : fileWrapper)
    }
    unsafe fn contents(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn fileType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileType)
    }
    unsafe fn setFileType_(&self, fileType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFileType : fileType)
    }
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn setBounds_(&self, bounds: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBounds : bounds)
    }
    unsafe fn fileWrapper(&self) -> NSFileWrapper
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileWrapper)
    }
    unsafe fn setFileWrapper_(&self, fileWrapper: NSFileWrapper)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFileWrapper : fileWrapper)
    }
    unsafe fn attachmentCell(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachmentCell)
    }
    unsafe fn setAttachmentCell_(&self, attachmentCell: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachmentCell : attachmentCell)
    }
    unsafe fn lineLayoutPadding(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineLayoutPadding)
    }
    unsafe fn setLineLayoutPadding_(&self, lineLayoutPadding: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineLayoutPadding : lineLayoutPadding)
    }
    unsafe fn allowsTextAttachmentView(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsTextAttachmentView)
    }
    unsafe fn setAllowsTextAttachmentView_(&self, allowsTextAttachmentView: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsTextAttachmentView : allowsTextAttachmentView)
    }
    unsafe fn usesTextAttachmentView(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesTextAttachmentView)
    }
    unsafe fn textAttachmentViewProviderClassForFileType_(fileType: NSString) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextAttachment").unwrap(), textAttachmentViewProviderClassForFileType : fileType)
    }
    unsafe fn registerTextAttachmentViewProviderClass_forFileType_(
        textAttachmentViewProviderClass: Class,
        fileType: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextAttachment").unwrap(), registerTextAttachmentViewProviderClass : textAttachmentViewProviderClass, forFileType : fileType)
    }
}
pub trait NSAttributedString_NSAttributedStringAttachmentConveniences:
    Sized + std::ops::Deref
{
    unsafe fn attributedStringWithAttachment_(attachment: NSTextAttachment) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), attributedStringWithAttachment : attachment)
    }
    unsafe fn attributedStringWithAttachment_attributes_(
        attachment: NSTextAttachment,
        attributes: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), attributedStringWithAttachment : attachment, attributes : attributes)
    }
}
pub trait NSMutableAttributedString_NSMutableAttributedStringAttachmentConveniences:
    Sized + std::ops::Deref
{
    unsafe fn updateAttachmentsFromPath_(&self, path: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateAttachmentsFromPath : path)
    }
}
impl NSTextAttachment_NSTextAttachment_Deprecation for NSTextAttachment {}
pub trait NSTextAttachment_NSTextAttachment_Deprecation: Sized + std::ops::Deref {}
pub trait NSTextView_NSCompletion: Sized + std::ops::Deref {
    unsafe fn complete_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, complete : sender)
    }
    unsafe fn completionsForPartialWordRange_indexOfSelectedItem_(
        &self,
        charRange: NSRange,
        index: *mut NSInteger,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completionsForPartialWordRange : charRange, indexOfSelectedItem : index)
    }
    unsafe fn insertCompletion_forPartialWordRange_movement_isFinal_(
        &self,
        word: NSString,
        charRange: NSRange,
        movement: NSInteger,
        flag: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertCompletion : word, forPartialWordRange : charRange, movement : movement, isFinal : flag)
    }
    unsafe fn rangeForUserCompletion(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rangeForUserCompletion)
    }
}
pub trait NSTextView_NSPasteboard: Sized + std::ops::Deref {
    unsafe fn writeSelectionToPasteboard_type_(&self, pboard: NSPasteboard, type_: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeSelectionToPasteboard : pboard, r#type : type_)
    }
    unsafe fn writeSelectionToPasteboard_types_(&self, pboard: NSPasteboard, types: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeSelectionToPasteboard : pboard, types : types)
    }
    unsafe fn preferredPasteboardTypeFromArray_restrictedToTypesFromArray_(
        &self,
        availableTypes: NSArray,
        allowedTypes: NSArray,
    ) -> NSPasteboardType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preferredPasteboardTypeFromArray : availableTypes, restrictedToTypesFromArray : allowedTypes)
    }
    unsafe fn readSelectionFromPasteboard_type_(
        &self,
        pboard: NSPasteboard,
        type_: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readSelectionFromPasteboard : pboard, r#type : type_)
    }
    unsafe fn readSelectionFromPasteboard_(&self, pboard: NSPasteboard) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readSelectionFromPasteboard : pboard)
    }
    unsafe fn validRequestorForSendType_returnType_(
        &self,
        sendType: NSString,
        returnType: NSString,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validRequestorForSendType : sendType, returnType : returnType)
    }
    unsafe fn pasteAsPlainText_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pasteAsPlainText : sender)
    }
    unsafe fn pasteAsRichText_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pasteAsRichText : sender)
    }
    unsafe fn writablePasteboardTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writablePasteboardTypes)
    }
    unsafe fn readablePasteboardTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readablePasteboardTypes)
    }
    unsafe fn registerForServices()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextView").unwrap(), registerForServices)
    }
}
pub trait NSTextView_NSDragging: Sized + std::ops::Deref {
    unsafe fn dragSelectionWithEvent_offset_slideBack_(
        &self,
        event: NSEvent,
        mouseOffset: NSSize,
        slideBack: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dragSelectionWithEvent : event, offset : mouseOffset, slideBack : slideBack)
    }
    unsafe fn dragImageForSelectionWithEvent_origin_(
        &self,
        event: NSEvent,
        origin: NSPointPointer,
    ) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dragImageForSelectionWithEvent : event, origin : origin)
    }
    unsafe fn dragOperationForDraggingInfo_type_(
        &self,
        dragInfo: *mut u64,
        type_: NSString,
    ) -> NSDragOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dragOperationForDraggingInfo : dragInfo, r#type : type_)
    }
    unsafe fn cleanUpAfterDragOperation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cleanUpAfterDragOperation)
    }
    unsafe fn acceptableDragTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptableDragTypes)
    }
}
pub trait NSTextView_NSSharing: Sized + std::ops::Deref {
    unsafe fn setSelectedRanges_affinity_stillSelecting_(
        &self,
        ranges: NSArray,
        affinity: NSSelectionAffinity,
        stillSelectingFlag: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedRanges : ranges, affinity : affinity, stillSelecting : stillSelectingFlag)
    }
    unsafe fn setSelectedRange_affinity_stillSelecting_(
        &self,
        charRange: NSRange,
        affinity: NSSelectionAffinity,
        stillSelectingFlag: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedRange : charRange, affinity : affinity, stillSelecting : stillSelectingFlag)
    }
    unsafe fn updateInsertionPointStateAndRestartTimer_(&self, restartFlag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateInsertionPointStateAndRestartTimer : restartFlag)
    }
    unsafe fn toggleContinuousSpellChecking_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleContinuousSpellChecking : sender)
    }
    unsafe fn toggleGrammarChecking_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleGrammarChecking : sender)
    }
    unsafe fn setSpellingState_range_(&self, value: NSInteger, charRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpellingState : value, range : charRange)
    }
    unsafe fn shouldChangeTextInRanges_replacementStrings_(
        &self,
        affectedRanges: NSArray,
        replacementStrings: NSArray,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldChangeTextInRanges : affectedRanges, replacementStrings : replacementStrings)
    }
    unsafe fn shouldChangeTextInRange_replacementString_(
        &self,
        affectedCharRange: NSRange,
        replacementString: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldChangeTextInRange : affectedCharRange, replacementString : replacementString)
    }
    unsafe fn didChangeText(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didChangeText)
    }
    unsafe fn breakUndoCoalescing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, breakUndoCoalescing)
    }
    unsafe fn showFindIndicatorForRange_(&self, charRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showFindIndicatorForRange : charRange)
    }
    unsafe fn setSelectedRange_(&self, charRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedRange : charRange)
    }
    unsafe fn selectedRanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedRanges)
    }
    unsafe fn setSelectedRanges_(&self, selectedRanges: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedRanges : selectedRanges)
    }
    unsafe fn selectionAffinity(&self) -> NSSelectionAffinity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionAffinity)
    }
    unsafe fn selectionGranularity(&self) -> NSSelectionGranularity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionGranularity)
    }
    unsafe fn setSelectionGranularity_(&self, selectionGranularity: NSSelectionGranularity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionGranularity : selectionGranularity)
    }
    unsafe fn selectedTextAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedTextAttributes)
    }
    unsafe fn setSelectedTextAttributes_(&self, selectedTextAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedTextAttributes : selectedTextAttributes)
    }
    unsafe fn insertionPointColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insertionPointColor)
    }
    unsafe fn setInsertionPointColor_(&self, insertionPointColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInsertionPointColor : insertionPointColor)
    }
    unsafe fn markedTextAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, markedTextAttributes)
    }
    unsafe fn setMarkedTextAttributes_(&self, markedTextAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarkedTextAttributes : markedTextAttributes)
    }
    unsafe fn linkTextAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkTextAttributes)
    }
    unsafe fn setLinkTextAttributes_(&self, linkTextAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinkTextAttributes : linkTextAttributes)
    }
    unsafe fn displaysLinkToolTips(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaysLinkToolTips)
    }
    unsafe fn setDisplaysLinkToolTips_(&self, displaysLinkToolTips: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaysLinkToolTips : displaysLinkToolTips)
    }
    unsafe fn acceptsGlyphInfo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptsGlyphInfo)
    }
    unsafe fn setAcceptsGlyphInfo_(&self, acceptsGlyphInfo: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcceptsGlyphInfo : acceptsGlyphInfo)
    }
    unsafe fn usesRuler(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesRuler)
    }
    unsafe fn setUsesRuler_(&self, usesRuler: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesRuler : usesRuler)
    }
    unsafe fn usesInspectorBar(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesInspectorBar)
    }
    unsafe fn setUsesInspectorBar_(&self, usesInspectorBar: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesInspectorBar : usesInspectorBar)
    }
    unsafe fn isContinuousSpellCheckingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContinuousSpellCheckingEnabled)
    }
    unsafe fn setContinuousSpellCheckingEnabled_(&self, continuousSpellCheckingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContinuousSpellCheckingEnabled : continuousSpellCheckingEnabled)
    }
    unsafe fn spellCheckerDocumentTag(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spellCheckerDocumentTag)
    }
    unsafe fn isGrammarCheckingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGrammarCheckingEnabled)
    }
    unsafe fn setGrammarCheckingEnabled_(&self, grammarCheckingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrammarCheckingEnabled : grammarCheckingEnabled)
    }
    unsafe fn typingAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typingAttributes)
    }
    unsafe fn setTypingAttributes_(&self, typingAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTypingAttributes : typingAttributes)
    }
    unsafe fn rangesForUserTextChange(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rangesForUserTextChange)
    }
    unsafe fn rangesForUserCharacterAttributeChange(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rangesForUserCharacterAttributeChange)
    }
    unsafe fn rangesForUserParagraphAttributeChange(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rangesForUserParagraphAttributeChange)
    }
    unsafe fn rangeForUserTextChange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rangeForUserTextChange)
    }
    unsafe fn rangeForUserCharacterAttributeChange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rangeForUserCharacterAttributeChange)
    }
    unsafe fn rangeForUserParagraphAttributeChange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rangeForUserParagraphAttributeChange)
    }
    unsafe fn allowsDocumentBackgroundColorChange(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsDocumentBackgroundColorChange)
    }
    unsafe fn setAllowsDocumentBackgroundColorChange_(
        &self,
        allowsDocumentBackgroundColorChange: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsDocumentBackgroundColorChange : allowsDocumentBackgroundColorChange)
    }
    unsafe fn defaultParagraphStyle(&self) -> NSParagraphStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultParagraphStyle)
    }
    unsafe fn setDefaultParagraphStyle_(&self, defaultParagraphStyle: NSParagraphStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultParagraphStyle : defaultParagraphStyle)
    }
    unsafe fn allowsUndo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsUndo)
    }
    unsafe fn setAllowsUndo_(&self, allowsUndo: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsUndo : allowsUndo)
    }
    unsafe fn isCoalescingUndo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCoalescingUndo)
    }
    unsafe fn allowsImageEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsImageEditing)
    }
    unsafe fn setAllowsImageEditing_(&self, allowsImageEditing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsImageEditing : allowsImageEditing)
    }
    unsafe fn usesRolloverButtonForSelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesRolloverButtonForSelection)
    }
    unsafe fn setUsesRolloverButtonForSelection_(&self, usesRolloverButtonForSelection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesRolloverButtonForSelection : usesRolloverButtonForSelection)
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
    unsafe fn isEditable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEditable)
    }
    unsafe fn setEditable_(&self, editable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditable : editable)
    }
    unsafe fn isSelectable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSelectable)
    }
    unsafe fn setSelectable_(&self, selectable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectable : selectable)
    }
    unsafe fn isRichText(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRichText)
    }
    unsafe fn setRichText_(&self, richText: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRichText : richText)
    }
    unsafe fn importsGraphics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, importsGraphics)
    }
    unsafe fn setImportsGraphics_(&self, importsGraphics: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImportsGraphics : importsGraphics)
    }
    unsafe fn drawsBackground(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawsBackground)
    }
    unsafe fn setDrawsBackground_(&self, drawsBackground: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawsBackground : drawsBackground)
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
    unsafe fn isFieldEditor(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFieldEditor)
    }
    unsafe fn setFieldEditor_(&self, fieldEditor: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldEditor : fieldEditor)
    }
    unsafe fn usesFontPanel(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesFontPanel)
    }
    unsafe fn setUsesFontPanel_(&self, usesFontPanel: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesFontPanel : usesFontPanel)
    }
    unsafe fn isRulerVisible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRulerVisible)
    }
    unsafe fn setRulerVisible_(&self, rulerVisible: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRulerVisible : rulerVisible)
    }
    unsafe fn allowedInputSourceLocales(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedInputSourceLocales)
    }
    unsafe fn setAllowedInputSourceLocales_(&self, allowedInputSourceLocales: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedInputSourceLocales : allowedInputSourceLocales)
    }
    unsafe fn isWritingToolsActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isWritingToolsActive)
    }
    unsafe fn writingToolsBehavior(&self) -> NSWritingToolsBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writingToolsBehavior)
    }
    unsafe fn setWritingToolsBehavior_(&self, writingToolsBehavior: NSWritingToolsBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWritingToolsBehavior : writingToolsBehavior)
    }
    unsafe fn allowedWritingToolsResultOptions(&self) -> NSWritingToolsResultOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedWritingToolsResultOptions)
    }
    unsafe fn setAllowedWritingToolsResultOptions_(
        &self,
        allowedWritingToolsResultOptions: NSWritingToolsResultOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedWritingToolsResultOptions : allowedWritingToolsResultOptions)
    }
}
pub trait NSTextView_NSTextChecking: Sized + std::ops::Deref {
    unsafe fn smartDeleteRangeForProposedRange_(&self, proposedCharRange: NSRange) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, smartDeleteRangeForProposedRange : proposedCharRange)
    }
    unsafe fn toggleSmartInsertDelete_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleSmartInsertDelete : sender)
    }
    unsafe fn smartInsertForString_replacingRange_beforeString_afterString_(
        &self,
        pasteString: NSString,
        charRangeToReplace: NSRange,
        beforeString: *mut NSString,
        afterString: *mut NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, smartInsertForString : pasteString, replacingRange : charRangeToReplace, beforeString : beforeString, afterString : afterString)
    }
    unsafe fn smartInsertBeforeStringForString_replacingRange_(
        &self,
        pasteString: NSString,
        charRangeToReplace: NSRange,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, smartInsertBeforeStringForString : pasteString, replacingRange : charRangeToReplace)
    }
    unsafe fn smartInsertAfterStringForString_replacingRange_(
        &self,
        pasteString: NSString,
        charRangeToReplace: NSRange,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, smartInsertAfterStringForString : pasteString, replacingRange : charRangeToReplace)
    }
    unsafe fn toggleAutomaticQuoteSubstitution_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleAutomaticQuoteSubstitution : sender)
    }
    unsafe fn toggleAutomaticLinkDetection_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleAutomaticLinkDetection : sender)
    }
    unsafe fn toggleAutomaticDataDetection_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleAutomaticDataDetection : sender)
    }
    unsafe fn toggleAutomaticDashSubstitution_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleAutomaticDashSubstitution : sender)
    }
    unsafe fn toggleAutomaticTextReplacement_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleAutomaticTextReplacement : sender)
    }
    unsafe fn toggleAutomaticSpellingCorrection_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleAutomaticSpellingCorrection : sender)
    }
    unsafe fn checkTextInRange_types_options_(
        &self,
        range: NSRange,
        checkingTypes: NSTextCheckingTypes,
        options: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkTextInRange : range, types : checkingTypes, options : options)
    }
    unsafe fn handleTextCheckingResults_forRange_types_options_orthography_wordCount_(
        &self,
        results: NSArray,
        range: NSRange,
        checkingTypes: NSTextCheckingTypes,
        options: NSDictionary,
        orthography: NSOrthography,
        wordCount: NSInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleTextCheckingResults : results, forRange : range, types : checkingTypes, options : options, orthography : orthography, wordCount : wordCount)
    }
    unsafe fn orderFrontSubstitutionsPanel_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, orderFrontSubstitutionsPanel : sender)
    }
    unsafe fn checkTextInSelection_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkTextInSelection : sender)
    }
    unsafe fn checkTextInDocument_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkTextInDocument : sender)
    }
    unsafe fn smartInsertDeleteEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smartInsertDeleteEnabled)
    }
    unsafe fn setSmartInsertDeleteEnabled_(&self, smartInsertDeleteEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmartInsertDeleteEnabled : smartInsertDeleteEnabled)
    }
    unsafe fn isAutomaticQuoteSubstitutionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticQuoteSubstitutionEnabled)
    }
    unsafe fn setAutomaticQuoteSubstitutionEnabled_(&self, automaticQuoteSubstitutionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticQuoteSubstitutionEnabled : automaticQuoteSubstitutionEnabled)
    }
    unsafe fn isAutomaticLinkDetectionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticLinkDetectionEnabled)
    }
    unsafe fn setAutomaticLinkDetectionEnabled_(&self, automaticLinkDetectionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticLinkDetectionEnabled : automaticLinkDetectionEnabled)
    }
    unsafe fn isAutomaticDataDetectionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticDataDetectionEnabled)
    }
    unsafe fn setAutomaticDataDetectionEnabled_(&self, automaticDataDetectionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticDataDetectionEnabled : automaticDataDetectionEnabled)
    }
    unsafe fn isAutomaticDashSubstitutionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticDashSubstitutionEnabled)
    }
    unsafe fn setAutomaticDashSubstitutionEnabled_(&self, automaticDashSubstitutionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticDashSubstitutionEnabled : automaticDashSubstitutionEnabled)
    }
    unsafe fn isAutomaticTextReplacementEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticTextReplacementEnabled)
    }
    unsafe fn setAutomaticTextReplacementEnabled_(&self, automaticTextReplacementEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticTextReplacementEnabled : automaticTextReplacementEnabled)
    }
    unsafe fn isAutomaticSpellingCorrectionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticSpellingCorrectionEnabled)
    }
    unsafe fn setAutomaticSpellingCorrectionEnabled_(
        &self,
        automaticSpellingCorrectionEnabled: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticSpellingCorrectionEnabled : automaticSpellingCorrectionEnabled)
    }
    unsafe fn enabledTextCheckingTypes(&self) -> NSTextCheckingTypes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabledTextCheckingTypes)
    }
    unsafe fn setEnabledTextCheckingTypes_(&self, enabledTextCheckingTypes: NSTextCheckingTypes)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabledTextCheckingTypes : enabledTextCheckingTypes)
    }
    unsafe fn usesFindPanel(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesFindPanel)
    }
    unsafe fn setUsesFindPanel_(&self, usesFindPanel: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesFindPanel : usesFindPanel)
    }
    unsafe fn usesFindBar(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesFindBar)
    }
    unsafe fn setUsesFindBar_(&self, usesFindBar: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesFindBar : usesFindBar)
    }
    unsafe fn isIncrementalSearchingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIncrementalSearchingEnabled)
    }
    unsafe fn setIncrementalSearchingEnabled_(&self, incrementalSearchingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncrementalSearchingEnabled : incrementalSearchingEnabled)
    }
    unsafe fn inlinePredictionType(&self) -> NSTextInputTraitType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inlinePredictionType)
    }
    unsafe fn setInlinePredictionType_(&self, inlinePredictionType: NSTextInputTraitType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInlinePredictionType : inlinePredictionType)
    }
    unsafe fn mathExpressionCompletionType(&self) -> NSTextInputTraitType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mathExpressionCompletionType)
    }
    unsafe fn setMathExpressionCompletionType_(
        &self,
        mathExpressionCompletionType: NSTextInputTraitType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMathExpressionCompletionType : mathExpressionCompletionType)
    }
}
pub trait NSTextView_NSQuickLookPreview: Sized + std::ops::Deref {
    unsafe fn toggleQuickLookPreviewPanel_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleQuickLookPreviewPanel : sender)
    }
    unsafe fn quickLookPreviewableItemsInRanges_(&self, ranges: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, quickLookPreviewableItemsInRanges : ranges)
    }
    unsafe fn updateQuickLookPreviewPanel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateQuickLookPreviewPanel)
    }
}
pub trait NSTextView_NSTextView_SharingService: Sized + std::ops::Deref {
    unsafe fn orderFrontSharingServicePicker_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, orderFrontSharingServicePicker : sender)
    }
}
pub trait NSTextView_NSTextView_TouchBar: Sized + std::ops::Deref {
    unsafe fn toggleAutomaticTextCompletion_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleAutomaticTextCompletion : sender)
    }
    unsafe fn updateTouchBarItemIdentifiers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateTouchBarItemIdentifiers)
    }
    unsafe fn updateTextTouchBarItems(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateTextTouchBarItems)
    }
    unsafe fn updateCandidates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateCandidates)
    }
    unsafe fn isAutomaticTextCompletionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutomaticTextCompletionEnabled)
    }
    unsafe fn setAutomaticTextCompletionEnabled_(&self, automaticTextCompletionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticTextCompletionEnabled : automaticTextCompletionEnabled)
    }
    unsafe fn allowsCharacterPickerTouchBarItem(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsCharacterPickerTouchBarItem)
    }
    unsafe fn setAllowsCharacterPickerTouchBarItem_(&self, allowsCharacterPickerTouchBarItem: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsCharacterPickerTouchBarItem : allowsCharacterPickerTouchBarItem)
    }
    unsafe fn candidateListTouchBarItem(&self) -> NSCandidateListTouchBarItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, candidateListTouchBarItem)
    }
}
pub trait NSTextView_NSTextView_Factory: Sized + std::ops::Deref {
    unsafe fn scrollableTextView() -> NSScrollView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextView").unwrap(), scrollableTextView)
    }
    unsafe fn fieldEditor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextView").unwrap(), fieldEditor)
    }
    unsafe fn scrollableDocumentContentTextView() -> NSScrollView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextView").unwrap(), scrollableDocumentContentTextView)
    }
    unsafe fn scrollablePlainDocumentContentTextView() -> NSScrollView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSTextView").unwrap(), scrollablePlainDocumentContentTextView)
    }
}
pub trait NSTextView_NSTextView_TextHighlight: Sized + std::ops::Deref {
    unsafe fn drawTextHighlightBackgroundForTextRange_origin_(
        &self,
        textRange: NSTextRange,
        origin: NSPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawTextHighlightBackgroundForTextRange : textRange, origin : origin)
    }
    unsafe fn highlight_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, highlight : sender)
    }
    unsafe fn textHighlightAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textHighlightAttributes)
    }
    unsafe fn setTextHighlightAttributes_(&self, textHighlightAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextHighlightAttributes : textHighlightAttributes)
    }
}
pub trait NSTextView_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn toggleBaseWritingDirection_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toggleBaseWritingDirection : sender)
    }
}
pub type NSTableViewAutosaveName = NSString;
pub trait NSObject_NSTableViewDataSourceDeprecated: Sized + std::ops::Deref {
    unsafe fn tableView_writeRows_toPasteboard_(
        &self,
        tableView: NSTableView,
        rows: NSArray,
        pboard: NSPasteboard,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tableView : tableView, writeRows : rows, toPasteboard : pboard)
    }
}
pub trait NSTableView_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setDrawsGrid_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawsGrid : flag)
    }
    unsafe fn drawsGrid(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawsGrid)
    }
    unsafe fn selectColumn_byExtendingSelection_(&self, column: NSInteger, extend_: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectColumn : column, byExtendingSelection : extend_)
    }
    unsafe fn selectRow_byExtendingSelection_(&self, row: NSInteger, extend_: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectRow : row, byExtendingSelection : extend_)
    }
    unsafe fn selectedColumnEnumerator(&self) -> NSEnumerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedColumnEnumerator)
    }
    unsafe fn selectedRowEnumerator(&self) -> NSEnumerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedRowEnumerator)
    }
    unsafe fn dragImageForRows_event_dragImageOffset_(
        &self,
        dragRows: NSArray,
        dragEvent: NSEvent,
        dragImageOffset: NSPointPointer,
    ) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dragImageForRows : dragRows, event : dragEvent, dragImageOffset : dragImageOffset)
    }
    unsafe fn setAutoresizesAllColumnsToFit_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoresizesAllColumnsToFit : flag)
    }
    unsafe fn autoresizesAllColumnsToFit(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoresizesAllColumnsToFit)
    }
    unsafe fn columnsInRect_(&self, rect: NSRect) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, columnsInRect : rect)
    }
    unsafe fn preparedCellAtColumn_row_(&self, column: NSInteger, row: NSInteger) -> NSCell
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preparedCellAtColumn : column, row : row)
    }
    unsafe fn textShouldBeginEditing_(&self, textObject: NSText) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textShouldBeginEditing : textObject)
    }
    unsafe fn textShouldEndEditing_(&self, textObject: NSText) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textShouldEndEditing : textObject)
    }
    unsafe fn textDidBeginEditing_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textDidBeginEditing : notification)
    }
    unsafe fn textDidEndEditing_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textDidEndEditing : notification)
    }
    unsafe fn textDidChange_(&self, notification: NSNotification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textDidChange : notification)
    }
    unsafe fn shouldFocusCell_atColumn_row_(
        &self,
        cell: NSCell,
        column: NSInteger,
        row: NSInteger,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldFocusCell : cell, atColumn : column, row : row)
    }
    unsafe fn focusedColumn(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusedColumn)
    }
    unsafe fn setFocusedColumn_(&self, focusedColumn: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusedColumn : focusedColumn)
    }
    unsafe fn performClickOnCellAtColumn_row_(&self, column: NSInteger, row: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performClickOnCellAtColumn : column, row : row)
    }
}
pub trait NSTableColumn_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn setResizable_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResizable : flag)
    }
    unsafe fn isResizable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isResizable)
    }
    unsafe fn dataCellForRow_(&self, row: NSInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dataCellForRow : row)
    }
    unsafe fn dataCell(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataCell)
    }
    unsafe fn setDataCell_(&self, dataCell: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataCell : dataCell)
    }
}
pub type NSTableViewDiffableDataSourceCellProvider = *mut ::std::os::raw::c_void;
pub type NSTableViewDiffableDataSourceRowProvider = *mut ::std::os::raw::c_void;
pub type NSTableViewDiffableDataSourceSectionHeaderViewProvider = *mut ::std::os::raw::c_void;
pub trait NSString_NSStringDrawing: Sized + std::ops::Deref {
    unsafe fn sizeWithAttributes_(&self, attrs: NSDictionary) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sizeWithAttributes : attrs)
    }
    unsafe fn drawAtPoint_withAttributes_(&self, point: CGPoint, attrs: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawAtPoint : point, withAttributes : attrs)
    }
    unsafe fn drawInRect_withAttributes_(&self, rect: CGRect, attrs: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawInRect : rect, withAttributes : attrs)
    }
}
pub trait NSAttributedString_NSStringDrawing: Sized + std::ops::Deref {
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn drawAtPoint_(&self, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawAtPoint : point)
    }
    unsafe fn drawInRect_(&self, rect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawInRect : rect)
    }
}
pub trait NSString_NSExtendedStringDrawing: Sized + std::ops::Deref {
    unsafe fn drawWithRect_options_attributes_context_(
        &self,
        rect: CGRect,
        options: NSStringDrawingOptions,
        attributes: NSDictionary,
        context: NSStringDrawingContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawWithRect : rect, options : options, attributes : attributes, context : context)
    }
    unsafe fn boundingRectWithSize_options_attributes_context_(
        &self,
        size: CGSize,
        options: NSStringDrawingOptions,
        attributes: NSDictionary,
        context: NSStringDrawingContext,
    ) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingRectWithSize : size, options : options, attributes : attributes, context : context)
    }
}
pub trait NSAttributedString_NSExtendedStringDrawing: Sized + std::ops::Deref {
    unsafe fn drawWithRect_options_context_(
        &self,
        rect: CGRect,
        options: NSStringDrawingOptions,
        context: NSStringDrawingContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawWithRect : rect, options : options, context : context)
    }
    unsafe fn boundingRectWithSize_options_context_(
        &self,
        size: CGSize,
        options: NSStringDrawingOptions,
        context: NSStringDrawingContext,
    ) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingRectWithSize : size, options : options, context : context)
    }
}
pub trait NSString_NSStringDrawingDeprecated: Sized + std::ops::Deref {
    unsafe fn drawWithRect_options_attributes_(
        &self,
        rect: NSRect,
        options: NSStringDrawingOptions,
        attributes: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawWithRect : rect, options : options, attributes : attributes)
    }
    unsafe fn boundingRectWithSize_options_attributes_(
        &self,
        size: NSSize,
        options: NSStringDrawingOptions,
        attributes: NSDictionary,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingRectWithSize : size, options : options, attributes : attributes)
    }
}
pub trait NSAttributedString_NSStringDrawingDeprecated: Sized + std::ops::Deref {
    unsafe fn drawWithRect_options_(&self, rect: NSRect, options: NSStringDrawingOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawWithRect : rect, options : options)
    }
    unsafe fn boundingRectWithSize_options_(
        &self,
        size: NSSize,
        options: NSStringDrawingOptions,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingRectWithSize : size, options : options)
    }
}
pub type NSRulerViewUnitName = NSString;
pub trait NSView_NSRulerMarkerClientViewDelegation: Sized + std::ops::Deref {
    unsafe fn rulerView_shouldMoveMarker_(&self, ruler: NSRulerView, marker: NSRulerMarker) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, shouldMoveMarker : marker)
    }
    unsafe fn rulerView_willMoveMarker_toLocation_(
        &self,
        ruler: NSRulerView,
        marker: NSRulerMarker,
        location: CGFloat,
    ) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, willMoveMarker : marker, toLocation : location)
    }
    unsafe fn rulerView_didMoveMarker_(&self, ruler: NSRulerView, marker: NSRulerMarker)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, didMoveMarker : marker)
    }
    unsafe fn rulerView_shouldRemoveMarker_(
        &self,
        ruler: NSRulerView,
        marker: NSRulerMarker,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, shouldRemoveMarker : marker)
    }
    unsafe fn rulerView_didRemoveMarker_(&self, ruler: NSRulerView, marker: NSRulerMarker)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, didRemoveMarker : marker)
    }
    unsafe fn rulerView_shouldAddMarker_(&self, ruler: NSRulerView, marker: NSRulerMarker) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, shouldAddMarker : marker)
    }
    unsafe fn rulerView_willAddMarker_atLocation_(
        &self,
        ruler: NSRulerView,
        marker: NSRulerMarker,
        location: CGFloat,
    ) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, willAddMarker : marker, atLocation : location)
    }
    unsafe fn rulerView_didAddMarker_(&self, ruler: NSRulerView, marker: NSRulerMarker)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, didAddMarker : marker)
    }
    unsafe fn rulerView_handleMouseDown_(&self, ruler: NSRulerView, event: NSEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, handleMouseDown : event)
    }
    unsafe fn rulerView_willSetClientView_(&self, ruler: NSRulerView, newClient: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, willSetClientView : newClient)
    }
    unsafe fn rulerView_locationForPoint_(&self, ruler: NSRulerView, point: NSPoint) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, locationForPoint : point)
    }
    unsafe fn rulerView_pointForLocation_(&self, ruler: NSRulerView, point: CGFloat) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rulerView : ruler, pointForLocation : point)
    }
}
pub type NSInterfaceStyle = NSUInteger;
pub trait NSResponder_NSInterfaceStyle: Sized + std::ops::Deref {
    unsafe fn interfaceStyle(&self) -> NSInterfaceStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceStyle)
    }
    unsafe fn setInterfaceStyle_(&self, interfaceStyle: NSInterfaceStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterfaceStyle : interfaceStyle)
    }
}
pub trait NSProgressIndicator_NSProgressIndicatorDeprecated: Sized + std::ops::Deref {
    unsafe fn animationDelay(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationDelay)
    }
    unsafe fn setAnimationDelay_(&self, delay: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimationDelay : delay)
    }
    unsafe fn animate_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, animate : sender)
    }
    unsafe fn isBezeled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBezeled)
    }
    unsafe fn setBezeled_(&self, bezeled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBezeled : bezeled)
    }
    unsafe fn controlTint(&self) -> NSControlTint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlTint)
    }
    unsafe fn setControlTint_(&self, controlTint: NSControlTint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControlTint : controlTint)
    }
}
pub trait NSAffineTransform_NSAppKitAdditions: Sized + std::ops::Deref {
    unsafe fn transformBezierPath_(&self, path: NSBezierPath) -> NSBezierPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transformBezierPath : path)
    }
    unsafe fn set(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, set)
    }
    unsafe fn concat(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, concat)
    }
}
pub trait NSBezierPath_NSBezierPathDeprecated: Sized + std::ops::Deref {
    unsafe fn cachesBezierPath(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cachesBezierPath)
    }
    unsafe fn setCachesBezierPath_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCachesBezierPath : flag)
    }
    unsafe fn appendBezierPathWithGlyph_inFont_(&self, glyph: NSGlyph, font: NSFont)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendBezierPathWithGlyph : glyph, inFont : font)
    }
    unsafe fn appendBezierPathWithGlyphs_count_inFont_(
        &self,
        glyphs: *mut NSGlyph,
        count: NSInteger,
        font: NSFont,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendBezierPathWithGlyphs : glyphs, count : count, inFont : font)
    }
    unsafe fn appendBezierPathWithPackedGlyphs_(&self, packedGlyphs: *const ::std::os::raw::c_char)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendBezierPathWithPackedGlyphs : packedGlyphs)
    }
}
pub type NSStatusItemAutosaveName = NSString;
pub trait NSStatusItem_NSStatusItemDeprecated: Sized + std::ops::Deref {
    unsafe fn sendActionOn_(&self, mask: NSEventMask) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendActionOn : mask)
    }
    unsafe fn drawStatusBarBackgroundInRect_withHighlight_(&self, rect: NSRect, highlight: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawStatusBarBackgroundInRect : rect, withHighlight : highlight)
    }
    unsafe fn popUpStatusItemMenu_(&self, menu: NSMenu)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popUpStatusItemMenu : menu)
    }
    unsafe fn action(&self) -> objc2::runtime::Sel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn setAction_(&self, action: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAction : action)
    }
    unsafe fn doubleAction(&self) -> objc2::runtime::Sel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doubleAction)
    }
    unsafe fn setDoubleAction_(&self, doubleAction: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoubleAction : doubleAction)
    }
    unsafe fn target(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : title)
    }
    unsafe fn attributedTitle(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedTitle)
    }
    unsafe fn setAttributedTitle_(&self, attributedTitle: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedTitle : attributedTitle)
    }
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn alternateImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternateImage)
    }
    unsafe fn setAlternateImage_(&self, alternateImage: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlternateImage : alternateImage)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn highlightMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightMode)
    }
    unsafe fn setHighlightMode_(&self, highlightMode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightMode : highlightMode)
    }
    unsafe fn toolTip(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toolTip)
    }
    unsafe fn setToolTip_(&self, toolTip: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setToolTip : toolTip)
    }
    unsafe fn view(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, view)
    }
    unsafe fn setView_(&self, view: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setView : view)
    }
}
pub type NSSoundName = NSString;
pub type NSSoundPlaybackDeviceIdentifier = NSString;
pub trait NSSound_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn soundUnfilteredFileTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSound").unwrap(), soundUnfilteredFileTypes)
    }
    unsafe fn soundUnfilteredPasteboardTypes() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSSound").unwrap(), soundUnfilteredPasteboardTypes)
    }
}
pub trait NSBundle_NSBundleSoundExtensions: Sized + std::ops::Deref {
    unsafe fn pathForSoundResource_(&self, name: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pathForSoundResource : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QTMovie(pub id);
impl std::ops::Deref for QTMovie {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for QTMovie {}
impl QTMovie {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"QTMovie").unwrap(), alloc) })
    }
}
impl IQTMovie for QTMovie {}
pub trait IQTMovie: Sized + std::ops::Deref {}
pub trait NSWindow_NSDrawers: Sized + std::ops::Deref {
    unsafe fn drawers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawers)
    }
}
pub type NSOpenGLPixelFormatAttribute = u32;
pub trait NSOpenGLContext_NSOpenGLPixelBuffer: Sized + std::ops::Deref {
    unsafe fn setPixelBuffer_cubeMapFace_mipMapLevel_currentVirtualScreen_(
        &self,
        pixelBuffer: NSOpenGLPixelBuffer,
        face: GLenum,
        level: GLint,
        screen: GLint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelBuffer : pixelBuffer, cubeMapFace : face, mipMapLevel : level, currentVirtualScreen : screen)
    }
    unsafe fn pixelBuffer(&self) -> NSOpenGLPixelBuffer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBuffer)
    }
    unsafe fn pixelBufferCubeMapFace(&self) -> GLenum
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBufferCubeMapFace)
    }
    unsafe fn pixelBufferMipMapLevel(&self) -> GLint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBufferMipMapLevel)
    }
    unsafe fn setTextureImageToPixelBuffer_colorBuffer_(
        &self,
        pixelBuffer: NSOpenGLPixelBuffer,
        source: GLenum,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureImageToPixelBuffer : pixelBuffer, colorBuffer : source)
    }
}
pub trait NSView_NSOpenGLSurfaceResolution: Sized + std::ops::Deref {
    unsafe fn wantsBestResolutionOpenGLSurface(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsBestResolutionOpenGLSurface)
    }
    unsafe fn setWantsBestResolutionOpenGLSurface_(&self, wantsBestResolutionOpenGLSurface: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsBestResolutionOpenGLSurface : wantsBestResolutionOpenGLSurface)
    }
}
pub trait NSView_NSExtendedDynamicRange: Sized + std::ops::Deref {
    unsafe fn wantsExtendedDynamicRangeOpenGLSurface(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsExtendedDynamicRangeOpenGLSurface)
    }
    unsafe fn setWantsExtendedDynamicRangeOpenGLSurface_(
        &self,
        wantsExtendedDynamicRangeOpenGLSurface: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsExtendedDynamicRangeOpenGLSurface : wantsExtendedDynamicRangeOpenGLSurface)
    }
}
pub trait NSApplication_NSScripting: Sized + std::ops::Deref {
    unsafe fn orderedDocuments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orderedDocuments)
    }
    unsafe fn orderedWindows(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orderedWindows)
    }
}
pub trait NSObject_NSApplicationScriptingDelegation: Sized + std::ops::Deref {
    unsafe fn application_delegateHandlesKey_(&self, sender: NSApplication, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, application : sender, delegateHandlesKey : key)
    }
}
pub trait NSDocument_NSScripting: Sized + std::ops::Deref {
    unsafe fn handleSaveScriptCommand_(&self, command: NSScriptCommand) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleSaveScriptCommand : command)
    }
    unsafe fn handleCloseScriptCommand_(&self, command: NSCloseCommand) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleCloseScriptCommand : command)
    }
    unsafe fn handlePrintScriptCommand_(&self, command: NSScriptCommand) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handlePrintScriptCommand : command)
    }
    unsafe fn lastComponentOfFileName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastComponentOfFileName)
    }
    unsafe fn setLastComponentOfFileName_(&self, lastComponentOfFileName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLastComponentOfFileName : lastComponentOfFileName)
    }
    unsafe fn objectSpecifier(&self) -> NSScriptObjectSpecifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectSpecifier)
    }
}
impl NSTextStorage_Scripting for NSTextStorage {}
pub trait NSTextStorage_Scripting: Sized + std::ops::Deref {
    unsafe fn attributeRuns(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeRuns)
    }
    unsafe fn setAttributeRuns_(&self, attributeRuns: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeRuns : attributeRuns)
    }
    unsafe fn paragraphs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paragraphs)
    }
    unsafe fn setParagraphs_(&self, paragraphs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParagraphs : paragraphs)
    }
    unsafe fn words(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, words)
    }
    unsafe fn setWords_(&self, words: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWords : words)
    }
    unsafe fn characters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characters)
    }
    unsafe fn setCharacters_(&self, characters: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharacters : characters)
    }
    unsafe fn font(&self) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, font)
    }
    unsafe fn setFont_(&self, font: NSFont)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFont : font)
    }
    unsafe fn foregroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foregroundColor)
    }
    unsafe fn setForegroundColor_(&self, foregroundColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setForegroundColor : foregroundColor)
    }
}
pub trait NSWindow_NSScripting: Sized + std::ops::Deref {
    unsafe fn setIsMiniaturized_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsMiniaturized : flag)
    }
    unsafe fn setIsVisible_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsVisible : flag)
    }
    unsafe fn setIsZoomed_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsZoomed : flag)
    }
    unsafe fn handleCloseScriptCommand_(&self, command: NSCloseCommand) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleCloseScriptCommand : command)
    }
    unsafe fn handlePrintScriptCommand_(&self, command: NSScriptCommand) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handlePrintScriptCommand : command)
    }
    unsafe fn handleSaveScriptCommand_(&self, command: NSScriptCommand) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleSaveScriptCommand : command)
    }
    unsafe fn hasCloseBox(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasCloseBox)
    }
    unsafe fn hasTitleBar(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasTitleBar)
    }
    unsafe fn isFloatingPanel(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFloatingPanel)
    }
    unsafe fn isMiniaturizable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMiniaturizable)
    }
    unsafe fn isModalPanel(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isModalPanel)
    }
    unsafe fn isResizable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isResizable)
    }
    unsafe fn isZoomable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isZoomable)
    }
    unsafe fn orderedIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orderedIndex)
    }
    unsafe fn setOrderedIndex_(&self, orderedIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrderedIndex : orderedIndex)
    }
}
pub trait NSGlyphInfo_NSGlyphInfo_Deprecated: Sized + std::ops::Deref {
    unsafe fn glyphName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, glyphName)
    }
    unsafe fn characterIdentifier(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characterIdentifier)
    }
    unsafe fn characterCollection(&self) -> NSCharacterCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characterCollection)
    }
    unsafe fn glyphInfoWithGlyphName_forFont_baseString_(
        glyphName: NSString,
        font: NSFont,
        string: NSString,
    ) -> NSGlyphInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSGlyphInfo").unwrap(), glyphInfoWithGlyphName : glyphName, forFont : font, baseString : string)
    }
    unsafe fn glyphInfoWithGlyph_forFont_baseString_(
        glyph: NSGlyph,
        font: NSFont,
        string: NSString,
    ) -> NSGlyphInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSGlyphInfo").unwrap(), glyphInfoWithGlyph : glyph, forFont : font, baseString : string)
    }
    unsafe fn glyphInfoWithCharacterIdentifier_collection_baseString_(
        cid: NSUInteger,
        characterCollection: NSCharacterCollection,
        string: NSString,
    ) -> NSGlyphInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSGlyphInfo").unwrap(), glyphInfoWithCharacterIdentifier : cid, collection : characterCollection, baseString : string)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSShadow(pub id);
impl std::ops::Deref for NSShadow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSShadow {}
impl NSShadow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSShadow").unwrap(), alloc) })
    }
}
impl PNSCopying for NSShadow {}
impl PNSSecureCoding for NSShadow {}
impl INSObject for NSShadow {}
impl PNSObject for NSShadow {}
impl std::convert::TryFrom<NSObject> for NSShadow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSShadow, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSShadow").unwrap()) };
        if is_kind_of {
            Ok(NSShadow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSShadow")
        }
    }
}
impl INSShadow for NSShadow {}
pub trait INSShadow: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn set(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, set)
    }
    unsafe fn shadowOffset(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowOffset)
    }
    unsafe fn setShadowOffset_(&self, shadowOffset: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowOffset : shadowOffset)
    }
    unsafe fn shadowBlurRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowBlurRadius)
    }
    unsafe fn setShadowBlurRadius_(&self, shadowBlurRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowBlurRadius : shadowBlurRadius)
    }
    unsafe fn shadowColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowColor)
    }
    unsafe fn setShadowColor_(&self, shadowColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowColor : shadowColor)
    }
}
pub trait NSTypesetter_NSLayoutPhaseInterface: Sized + std::ops::Deref {
    unsafe fn willSetLineFragmentRect_forGlyphRange_usedRect_baselineOffset_(
        &self,
        lineRect: NSRectPointer,
        glyphRange: NSRange,
        usedRect: NSRectPointer,
        baselineOffset: *mut CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willSetLineFragmentRect : lineRect, forGlyphRange : glyphRange, usedRect : usedRect, baselineOffset : baselineOffset)
    }
    unsafe fn shouldBreakLineByWordBeforeCharacterAtIndex_(&self, charIndex: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldBreakLineByWordBeforeCharacterAtIndex : charIndex)
    }
    unsafe fn shouldBreakLineByHyphenatingBeforeCharacterAtIndex_(
        &self,
        charIndex: NSUInteger,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldBreakLineByHyphenatingBeforeCharacterAtIndex : charIndex)
    }
    unsafe fn hyphenationFactorForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hyphenationFactorForGlyphAtIndex : glyphIndex)
    }
    unsafe fn hyphenCharacterForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> UTF32Char
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hyphenCharacterForGlyphAtIndex : glyphIndex)
    }
    unsafe fn boundingBoxForControlGlyphAtIndex_forTextContainer_proposedLineFragment_glyphPosition_characterIndex_(
        &self,
        glyphIndex: NSUInteger,
        textContainer: NSTextContainer,
        proposedRect: NSRect,
        glyphPosition: NSPoint,
        charIndex: NSUInteger,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingBoxForControlGlyphAtIndex : glyphIndex, forTextContainer : textContainer, proposedLineFragment : proposedRect, glyphPosition : glyphPosition, characterIndex : charIndex)
    }
}
pub trait NSTypesetter_NSGlyphStorageInterface: Sized + std::ops::Deref {
    unsafe fn characterRangeForGlyphRange_actualGlyphRange_(
        &self,
        glyphRange: NSRange,
        actualGlyphRange: NSRangePointer,
    ) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, characterRangeForGlyphRange : glyphRange, actualGlyphRange : actualGlyphRange)
    }
    unsafe fn glyphRangeForCharacterRange_actualCharacterRange_(
        &self,
        charRange: NSRange,
        actualCharRange: NSRangePointer,
    ) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, glyphRangeForCharacterRange : charRange, actualCharacterRange : actualCharRange)
    }
    unsafe fn getLineFragmentRect_usedRect_remainingRect_forStartingGlyphAtIndex_proposedRect_lineSpacing_paragraphSpacingBefore_paragraphSpacingAfter_(
        &self,
        lineFragmentRect: NSRectPointer,
        lineFragmentUsedRect: NSRectPointer,
        remainingRect: NSRectPointer,
        startingGlyphIndex: NSUInteger,
        proposedRect: NSRect,
        lineSpacing: CGFloat,
        paragraphSpacingBefore: CGFloat,
        paragraphSpacingAfter: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getLineFragmentRect : lineFragmentRect, usedRect : lineFragmentUsedRect, remainingRect : remainingRect, forStartingGlyphAtIndex : startingGlyphIndex, proposedRect : proposedRect, lineSpacing : lineSpacing, paragraphSpacingBefore : paragraphSpacingBefore, paragraphSpacingAfter : paragraphSpacingAfter)
    }
    unsafe fn setLineFragmentRect_forGlyphRange_usedRect_baselineOffset_(
        &self,
        fragmentRect: NSRect,
        glyphRange: NSRange,
        usedRect: NSRect,
        baselineOffset: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineFragmentRect : fragmentRect, forGlyphRange : glyphRange, usedRect : usedRect, baselineOffset : baselineOffset)
    }
    unsafe fn setNotShownAttribute_forGlyphRange_(&self, flag: BOOL, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotShownAttribute : flag, forGlyphRange : glyphRange)
    }
    unsafe fn setDrawsOutsideLineFragment_forGlyphRange_(&self, flag: BOOL, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawsOutsideLineFragment : flag, forGlyphRange : glyphRange)
    }
    unsafe fn setLocation_withAdvancements_forStartOfGlyphRange_(
        &self,
        location: NSPoint,
        advancements: *const CGFloat,
        glyphRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocation : location, withAdvancements : advancements, forStartOfGlyphRange : glyphRange)
    }
    unsafe fn setAttachmentSize_forGlyphRange_(&self, attachmentSize: NSSize, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachmentSize : attachmentSize, forGlyphRange : glyphRange)
    }
    unsafe fn setBidiLevels_forGlyphRange_(&self, levels: *const u8, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBidiLevels : levels, forGlyphRange : glyphRange)
    }
}
pub trait NSTypesetter_NSTypesetter_Deprecated: Sized + std::ops::Deref {
    unsafe fn actionForControlCharacterAtIndex_(
        &self,
        charIndex: NSUInteger,
    ) -> NSTypesetterControlCharacterAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, actionForControlCharacterAtIndex : charIndex)
    }
    unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits_bidiLevels_(
        &self,
        glyphsRange: NSRange,
        glyphBuffer: *mut NSGlyph,
        charIndexBuffer: *mut NSUInteger,
        inscribeBuffer: *mut NSGlyphInscription,
        elasticBuffer: *mut BOOL,
        bidiLevelBuffer: *mut ::std::os::raw::c_uchar,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getGlyphsInRange : glyphsRange, glyphs : glyphBuffer, characterIndexes : charIndexBuffer, glyphInscriptions : inscribeBuffer, elasticBits : elasticBuffer, bidiLevels : bidiLevelBuffer)
    }
    unsafe fn substituteGlyphsInRange_withGlyphs_(&self, glyphRange: NSRange, glyphs: *mut NSGlyph)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, substituteGlyphsInRange : glyphRange, withGlyphs : glyphs)
    }
    unsafe fn insertGlyph_atGlyphIndex_characterIndex_(
        &self,
        glyph: NSGlyph,
        glyphIndex: NSUInteger,
        characterIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertGlyph : glyph, atGlyphIndex : glyphIndex, characterIndex : characterIndex)
    }
    unsafe fn deleteGlyphsInRange_(&self, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteGlyphsInRange : glyphRange)
    }
}
pub trait NSATSTypesetter_NSPantherCompatibility: Sized + std::ops::Deref {
    unsafe fn lineFragmentRectForProposedRect_remainingRect_(
        &self,
        proposedRect: NSRect,
        remainingRect: NSRectPointer,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineFragmentRectForProposedRect : proposedRect, remainingRect : remainingRect)
    }
}
pub trait NSATSTypesetter_NSPrimitiveInterface: Sized + std::ops::Deref {
    unsafe fn substituteFontForFont_(&self, originalFont: NSFont) -> NSFont
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, substituteFontForFont : originalFont)
    }
    unsafe fn textTabForGlyphLocation_writingDirection_maxLocation_(
        &self,
        glyphLocation: CGFloat,
        direction: NSWritingDirection,
        maxLocation: CGFloat,
    ) -> NSTextTab
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textTabForGlyphLocation : glyphLocation, writingDirection : direction, maxLocation : maxLocation)
    }
    unsafe fn setParagraphGlyphRange_separatorGlyphRange_(
        &self,
        paragraphRange: NSRange,
        paragraphSeparatorRange: NSRange,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParagraphGlyphRange : paragraphRange, separatorGlyphRange : paragraphSeparatorRange)
    }
    unsafe fn layoutParagraphAtPoint_(&self, lineFragmentOrigin: *mut NSPoint) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutParagraphAtPoint : lineFragmentOrigin)
    }
    unsafe fn lineSpacingAfterGlyphAtIndex_withProposedLineFragmentRect_(
        &self,
        glyphIndex: NSUInteger,
        rect: NSRect,
    ) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineSpacingAfterGlyphAtIndex : glyphIndex, withProposedLineFragmentRect : rect)
    }
    unsafe fn paragraphSpacingBeforeGlyphAtIndex_withProposedLineFragmentRect_(
        &self,
        glyphIndex: NSUInteger,
        rect: NSRect,
    ) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paragraphSpacingBeforeGlyphAtIndex : glyphIndex, withProposedLineFragmentRect : rect)
    }
    unsafe fn paragraphSpacingAfterGlyphAtIndex_withProposedLineFragmentRect_(
        &self,
        glyphIndex: NSUInteger,
        rect: NSRect,
    ) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paragraphSpacingAfterGlyphAtIndex : glyphIndex, withProposedLineFragmentRect : rect)
    }
    unsafe fn setHardInvalidation_forGlyphRange_(&self, flag: BOOL, glyphRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHardInvalidation : flag, forGlyphRange : glyphRange)
    }
    unsafe fn getLineFragmentRect_usedRect_forParagraphSeparatorGlyphRange_atProposedOrigin_(
        &self,
        lineFragmentRect: *mut NSRect,
        lineFragmentUsedRect: *mut NSRect,
        paragraphSeparatorGlyphRange: NSRange,
        lineOrigin: NSPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getLineFragmentRect : lineFragmentRect, usedRect : lineFragmentUsedRect, forParagraphSeparatorGlyphRange : paragraphSeparatorGlyphRange, atProposedOrigin : lineOrigin)
    }
    unsafe fn usesFontLeading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesFontLeading)
    }
    unsafe fn setUsesFontLeading_(&self, usesFontLeading: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesFontLeading : usesFontLeading)
    }
    unsafe fn typesetterBehavior(&self) -> NSTypesetterBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typesetterBehavior)
    }
    unsafe fn setTypesetterBehavior_(&self, typesetterBehavior: NSTypesetterBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTypesetterBehavior : typesetterBehavior)
    }
    unsafe fn hyphenationFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hyphenationFactor)
    }
    unsafe fn setHyphenationFactor_(&self, hyphenationFactor: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHyphenationFactor : hyphenationFactor)
    }
    unsafe fn lineFragmentPadding(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineFragmentPadding)
    }
    unsafe fn setLineFragmentPadding_(&self, lineFragmentPadding: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineFragmentPadding : lineFragmentPadding)
    }
    unsafe fn bidiProcessingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bidiProcessingEnabled)
    }
    unsafe fn setBidiProcessingEnabled_(&self, bidiProcessingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBidiProcessingEnabled : bidiProcessingEnabled)
    }
    unsafe fn attributedString(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedString)
    }
    unsafe fn setAttributedString_(&self, attributedString: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedString : attributedString)
    }
    unsafe fn paragraphGlyphRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paragraphGlyphRange)
    }
    unsafe fn paragraphSeparatorGlyphRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paragraphSeparatorGlyphRange)
    }
    unsafe fn layoutManager(&self) -> NSLayoutManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutManager)
    }
    unsafe fn currentTextContainer(&self) -> NSTextContainer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentTextContainer)
    }
}
pub trait NSATSTypesetter_NSLayoutPhaseInterface: Sized + std::ops::Deref {
    unsafe fn willSetLineFragmentRect_forGlyphRange_usedRect_baselineOffset_(
        &self,
        lineRect: *mut NSRect,
        glyphRange: NSRange,
        usedRect: *mut NSRect,
        baselineOffset: *mut CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willSetLineFragmentRect : lineRect, forGlyphRange : glyphRange, usedRect : usedRect, baselineOffset : baselineOffset)
    }
    unsafe fn shouldBreakLineByWordBeforeCharacterAtIndex_(&self, charIndex: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldBreakLineByWordBeforeCharacterAtIndex : charIndex)
    }
    unsafe fn shouldBreakLineByHyphenatingBeforeCharacterAtIndex_(
        &self,
        charIndex: NSUInteger,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldBreakLineByHyphenatingBeforeCharacterAtIndex : charIndex)
    }
    unsafe fn hyphenationFactorForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hyphenationFactorForGlyphAtIndex : glyphIndex)
    }
    unsafe fn hyphenCharacterForGlyphAtIndex_(&self, glyphIndex: NSUInteger) -> UTF32Char
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hyphenCharacterForGlyphAtIndex : glyphIndex)
    }
    unsafe fn boundingBoxForControlGlyphAtIndex_forTextContainer_proposedLineFragment_glyphPosition_characterIndex_(
        &self,
        glyphIndex: NSUInteger,
        textContainer: NSTextContainer,
        proposedRect: NSRect,
        glyphPosition: NSPoint,
        charIndex: NSUInteger,
    ) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingBoxForControlGlyphAtIndex : glyphIndex, forTextContainer : textContainer, proposedLineFragment : proposedRect, glyphPosition : glyphPosition, characterIndex : charIndex)
    }
}
pub trait NSATSTypesetter_NSGlyphStorageInterface: Sized + std::ops::Deref {
    unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits_(
        &self,
        glyphsRange: NSRange,
        glyphBuffer: *mut NSGlyph,
        charIndexBuffer: *mut NSUInteger,
        inscribeBuffer: *mut NSGlyphInscription,
        elasticBuffer: *mut BOOL,
    ) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getGlyphsInRange : glyphsRange, glyphs : glyphBuffer, characterIndexes : charIndexBuffer, glyphInscriptions : inscribeBuffer, elasticBits : elasticBuffer)
    }
}
pub type NSSearchFieldRecentsAutosaveName = NSString;
pub trait NSSearchField_NSSearchField_Deprecated: Sized + std::ops::Deref {
    unsafe fn rectForSearchTextWhenCentered_(&self, isCentered: BOOL) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectForSearchTextWhenCentered : isCentered)
    }
    unsafe fn rectForSearchButtonWhenCentered_(&self, isCentered: BOOL) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectForSearchButtonWhenCentered : isCentered)
    }
    unsafe fn rectForCancelButtonWhenCentered_(&self, isCentered: BOOL) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectForCancelButtonWhenCentered : isCentered)
    }
    unsafe fn centersPlaceholder(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centersPlaceholder)
    }
    unsafe fn setCentersPlaceholder_(&self, centersPlaceholder: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCentersPlaceholder : centersPlaceholder)
    }
}
pub trait NSObjectController_NSManagedController: Sized + std::ops::Deref {
    unsafe fn fetchWithRequest_merge_error_(
        &self,
        fetchRequest: NSFetchRequest,
        merge: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchWithRequest : fetchRequest, merge : merge, error : error)
    }
    unsafe fn fetch_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetch : sender)
    }
    unsafe fn defaultFetchRequest(&self) -> NSFetchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultFetchRequest)
    }
    unsafe fn managedObjectContext(&self) -> NSManagedObjectContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managedObjectContext)
    }
    unsafe fn setManagedObjectContext_(&self, managedObjectContext: NSManagedObjectContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManagedObjectContext : managedObjectContext)
    }
    unsafe fn entityName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityName)
    }
    unsafe fn setEntityName_(&self, entityName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntityName : entityName)
    }
    unsafe fn fetchPredicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fetchPredicate)
    }
    unsafe fn setFetchPredicate_(&self, fetchPredicate: NSPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFetchPredicate : fetchPredicate)
    }
    unsafe fn usesLazyFetching(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesLazyFetching)
    }
    unsafe fn setUsesLazyFetching_(&self, usesLazyFetching: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesLazyFetching : usesLazyFetching)
    }
}
pub type NSTextListMarkerFormat = NSString;
pub trait NSPersistentDocument_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn configurePersistentStoreCoordinatorForURL_ofType_error_(
        &self,
        url: NSURL,
        fileType: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, configurePersistentStoreCoordinatorForURL : url, ofType : fileType, error : error)
    }
}
pub trait NSPathControl_NSDeprecated: Sized + std::ops::Deref {
    unsafe fn clickedPathComponentCell(&self) -> NSPathComponentCell
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clickedPathComponentCell)
    }
    unsafe fn pathComponentCells(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pathComponentCells)
    }
    unsafe fn setPathComponentCells_(&self, cells: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPathComponentCells : cells)
    }
}
pub type NSTextInputSourceIdentifier = NSString;
pub trait NSApplication_NSUserInterfaceItemSearching: Sized + std::ops::Deref {
    unsafe fn registerUserInterfaceItemSearchHandler_(&self, handler: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerUserInterfaceItemSearchHandler : handler)
    }
    unsafe fn unregisterUserInterfaceItemSearchHandler_(&self, handler: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterUserInterfaceItemSearchHandler : handler)
    }
    unsafe fn searchString_inUserInterfaceItemString_searchRange_foundRange_(
        &self,
        searchString: NSString,
        stringToSearch: NSString,
        searchRange: NSRange,
        foundRange: *mut NSRange,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchString : searchString, inUserInterfaceItemString : stringToSearch, searchRange : searchRange, foundRange : foundRange)
    }
}
pub trait NSDocumentController_NSWindowRestoration: Sized + std::ops::Deref {}
pub trait NSApplication_NSWindowRestoration: Sized + std::ops::Deref {
    unsafe fn restoreWindowWithIdentifier_state_completionHandler_(
        &self,
        identifier: NSString,
        state: NSCoder,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, restoreWindowWithIdentifier : identifier, state : state, completionHandler : completionHandler)
    }
}
pub trait NSWindow_NSUserInterfaceRestoration: Sized + std::ops::Deref {
    unsafe fn disableSnapshotRestoration(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableSnapshotRestoration)
    }
    unsafe fn enableSnapshotRestoration(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableSnapshotRestoration)
    }
    unsafe fn isRestorable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRestorable)
    }
    unsafe fn setRestorable_(&self, restorable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRestorable : restorable)
    }
    unsafe fn restorationClass(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restorationClass)
    }
    unsafe fn setRestorationClass_(&self, restorationClass: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRestorationClass : restorationClass)
    }
}
pub trait NSResponder_NSRestorableState: Sized + std::ops::Deref {
    unsafe fn encodeRestorableStateWithCoder_(&self, coder: NSCoder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeRestorableStateWithCoder : coder)
    }
    unsafe fn encodeRestorableStateWithCoder_backgroundQueue_(
        &self,
        coder: NSCoder,
        queue: NSOperationQueue,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeRestorableStateWithCoder : coder, backgroundQueue : queue)
    }
    unsafe fn restoreStateWithCoder_(&self, coder: NSCoder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, restoreStateWithCoder : coder)
    }
    unsafe fn invalidateRestorableState(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidateRestorableState)
    }
    unsafe fn allowedClassesForRestorableStateKeyPath_(keyPath: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSResponder").unwrap(), allowedClassesForRestorableStateKeyPath : keyPath)
    }
    unsafe fn restorableStateKeyPaths() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSResponder").unwrap(), restorableStateKeyPaths)
    }
}
pub trait NSApplication_NSRestorableStateExtension: Sized + std::ops::Deref {
    unsafe fn extendStateRestoration(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extendStateRestoration)
    }
    unsafe fn completeStateRestoration(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completeStateRestoration)
    }
}
pub trait NSDocument_NSRestorableState: Sized + std::ops::Deref {
    unsafe fn restoreDocumentWindowWithIdentifier_state_completionHandler_(
        &self,
        identifier: NSString,
        state: NSCoder,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, restoreDocumentWindowWithIdentifier : identifier, state : state, completionHandler : completionHandler)
    }
    unsafe fn encodeRestorableStateWithCoder_(&self, coder: NSCoder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeRestorableStateWithCoder : coder)
    }
    unsafe fn encodeRestorableStateWithCoder_backgroundQueue_(
        &self,
        coder: NSCoder,
        queue: NSOperationQueue,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeRestorableStateWithCoder : coder, backgroundQueue : queue)
    }
    unsafe fn restoreStateWithCoder_(&self, coder: NSCoder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, restoreStateWithCoder : coder)
    }
    unsafe fn invalidateRestorableState(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidateRestorableState)
    }
    unsafe fn allowedClassesForRestorableStateKeyPath_(keyPath: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSDocument").unwrap(), allowedClassesForRestorableStateKeyPath : keyPath)
    }
    unsafe fn restorableStateKeyPaths() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSDocument").unwrap(), restorableStateKeyPaths)
    }
}
pub trait NSItemProvider_NSItemSourceInfo: Sized + std::ops::Deref {
    unsafe fn sourceFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceFrame)
    }
    unsafe fn containerFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerFrame)
    }
    unsafe fn preferredPresentationSize(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredPresentationSize)
    }
}
pub type NSDataAssetName = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSDataAsset(pub id);
impl std::ops::Deref for NSDataAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSDataAsset {}
impl NSDataAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSDataAsset").unwrap(), alloc) })
    }
}
impl PNSCopying for NSDataAsset {}
impl INSObject for NSDataAsset {}
impl PNSObject for NSDataAsset {}
impl std::convert::TryFrom<NSObject> for NSDataAsset {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NSDataAsset, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NSDataAsset").unwrap()) };
        if is_kind_of {
            Ok(NSDataAsset(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NSDataAsset")
        }
    }
}
impl INSDataAsset for NSDataAsset {}
pub trait INSDataAsset: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithName_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name)
    }
    unsafe fn initWithName_bundle_(&self, name: NSString, bundle: NSBundle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, bundle : bundle)
    }
    unsafe fn name(&self) -> NSDataAssetName
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn typeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typeIdentifier)
    }
}
pub trait NSView_NSPressureConfiguration: Sized + std::ops::Deref {
    unsafe fn pressureConfiguration(&self) -> NSPressureConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pressureConfiguration)
    }
    unsafe fn setPressureConfiguration_(&self, pressureConfiguration: NSPressureConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPressureConfiguration : pressureConfiguration)
    }
}
pub trait NSAttributedString_NSAttributedStringAdaptiveImageGlyphConveniences:
    Sized + std::ops::Deref
{
    unsafe fn attributedStringWithAdaptiveImageGlyph_attributes_(
        adaptiveImageGlyph: NSAdaptiveImageGlyph,
        attributes: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAttributedString").unwrap(), attributedStringWithAdaptiveImageGlyph : adaptiveImageGlyph, attributes : attributes)
    }
}
pub type ABPropertyType = CFIndex;
pub type ABSearchComparison = CFIndex;
pub type ABSearchConjunction = CFIndex;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABAddressBook(pub id);
impl std::ops::Deref for ABAddressBook {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABAddressBook {}
impl ABAddressBook {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABAddressBook").unwrap(), alloc) })
    }
}
impl INSObject for ABAddressBook {}
impl PNSObject for ABAddressBook {}
impl std::convert::TryFrom<NSObject> for ABAddressBook {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ABAddressBook, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABAddressBook").unwrap()) };
        if is_kind_of {
            Ok(ABAddressBook(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ABAddressBook")
        }
    }
}
impl IABAddressBook for ABAddressBook {}
pub trait IABAddressBook: Sized + std::ops::Deref {
    unsafe fn recordsMatchingSearchElement_(&self, search: ABSearchElement) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordsMatchingSearchElement : search)
    }
    unsafe fn save(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, save)
    }
    unsafe fn saveAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveAndReturnError : error)
    }
    unsafe fn hasUnsavedChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasUnsavedChanges)
    }
    unsafe fn me(&self) -> ABPerson
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, me)
    }
    unsafe fn setMe_(&self, moi: ABPerson)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMe : moi)
    }
    unsafe fn recordForUniqueId_(&self, uniqueId: NSString) -> ABRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordForUniqueId : uniqueId)
    }
    unsafe fn addRecord_error_(&self, record: ABRecord, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRecord : record, error : error)
    }
    unsafe fn addRecord_(&self, record: ABRecord) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRecord : record)
    }
    unsafe fn removeRecord_error_(&self, record: ABRecord, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRecord : record, error : error)
    }
    unsafe fn removeRecord_(&self, record: ABRecord) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRecord : record)
    }
    unsafe fn people(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, people)
    }
    unsafe fn groups(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groups)
    }
    unsafe fn recordClassFromUniqueId_(&self, uniqueId: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordClassFromUniqueId : uniqueId)
    }
    unsafe fn formattedAddressFromDictionary_(&self, address: NSDictionary) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, formattedAddressFromDictionary : address)
    }
    unsafe fn defaultCountryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultCountryCode)
    }
    unsafe fn defaultNameOrdering(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultNameOrdering)
    }
    unsafe fn sharedAddressBook() -> ABAddressBook
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABAddressBook").unwrap(), sharedAddressBook)
    }
    unsafe fn addressBook() -> ABAddressBook
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABAddressBook").unwrap(), addressBook)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABRecord(pub id);
impl std::ops::Deref for ABRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABRecord {}
impl ABRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABRecord").unwrap(), alloc) })
    }
}
impl INSObject for ABRecord {}
impl PNSObject for ABRecord {}
impl std::convert::TryFrom<NSObject> for ABRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ABRecord, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABRecord").unwrap()) };
        if is_kind_of {
            Ok(ABRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ABRecord")
        }
    }
}
impl IABRecord for ABRecord {}
pub trait IABRecord: Sized + std::ops::Deref {
    unsafe fn init(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAddressBook_(&self, addressBook: ABAddressBook) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAddressBook : addressBook)
    }
    unsafe fn valueForProperty_(&self, property: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForProperty : property)
    }
    unsafe fn setValue_forProperty_error_(
        &self,
        value: id,
        property: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forProperty : property, error : error)
    }
    unsafe fn setValue_forProperty_(&self, value: id, property: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forProperty : property)
    }
    unsafe fn removeValueForProperty_(&self, property: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeValueForProperty : property)
    }
    unsafe fn isReadOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadOnly)
    }
}
impl ABRecord_ABRecord_Convenience for ABRecord {}
pub trait ABRecord_ABRecord_Convenience: Sized + std::ops::Deref {
    unsafe fn uniqueId(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueId)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABGroup(pub id);
impl std::ops::Deref for ABGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABGroup {}
impl ABGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABGroup").unwrap(), alloc) })
    }
}
impl IABRecord for ABGroup {}
impl From<ABGroup> for ABRecord {
    fn from(child: ABGroup) -> ABRecord {
        ABRecord(child.0)
    }
}
impl std::convert::TryFrom<ABRecord> for ABGroup {
    type Error = &'static str;
    fn try_from(parent: ABRecord) -> Result<ABGroup, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABGroup").unwrap()) };
        if is_kind_of {
            Ok(ABGroup(parent.0))
        } else {
            Err("This ABRecord cannot be downcasted to ABGroup")
        }
    }
}
impl INSObject for ABGroup {}
impl PNSObject for ABGroup {}
impl IABGroup for ABGroup {}
pub trait IABGroup: Sized + std::ops::Deref {
    unsafe fn members(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, members)
    }
    unsafe fn addMember_(&self, person: ABPerson) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMember : person)
    }
    unsafe fn removeMember_(&self, person: ABPerson) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeMember : person)
    }
    unsafe fn subgroups(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subgroups)
    }
    unsafe fn addSubgroup_(&self, group: ABGroup) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSubgroup : group)
    }
    unsafe fn removeSubgroup_(&self, group: ABGroup) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeSubgroup : group)
    }
    unsafe fn parentGroups(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentGroups)
    }
    unsafe fn setDistributionIdentifier_forProperty_person_(
        &self,
        identifier: NSString,
        property: NSString,
        person: ABPerson,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDistributionIdentifier : identifier, forProperty : property, person : person)
    }
    unsafe fn distributionIdentifierForProperty_person_(
        &self,
        property: NSString,
        person: ABPerson,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, distributionIdentifierForProperty : property, person : person)
    }
}
impl ABGroup_ABGroup_Properties for ABGroup {}
pub trait ABGroup_ABGroup_Properties: Sized + std::ops::Deref {
    unsafe fn addPropertiesAndTypes_(properties: NSDictionary) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABGroup").unwrap(), addPropertiesAndTypes : properties)
    }
    unsafe fn removeProperties_(properties: NSArray) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABGroup").unwrap(), removeProperties : properties)
    }
    unsafe fn properties() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABGroup").unwrap(), properties)
    }
    unsafe fn typeOfProperty_(property: NSString) -> ABPropertyType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABGroup").unwrap(), typeOfProperty : property)
    }
}
impl ABGroup_ABGroup_Searching for ABGroup {}
pub trait ABGroup_ABGroup_Searching: Sized + std::ops::Deref {
    unsafe fn searchElementForProperty_label_key_value_comparison_(
        property: NSString,
        label: NSString,
        key: NSString,
        value: id,
        comparison: ABSearchComparison,
    ) -> ABSearchElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABGroup").unwrap(), searchElementForProperty : property, label : label, key : key, value : value, comparison : comparison)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABPerson(pub id);
impl std::ops::Deref for ABPerson {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABPerson {}
impl ABPerson {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABPerson").unwrap(), alloc) })
    }
}
impl IABRecord for ABPerson {}
impl std::convert::TryFrom<ABRecord> for ABPerson {
    type Error = &'static str;
    fn try_from(parent: ABRecord) -> Result<ABPerson, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABPerson").unwrap()) };
        if is_kind_of {
            Ok(ABPerson(parent.0))
        } else {
            Err("This ABRecord cannot be downcasted to ABPerson")
        }
    }
}
impl INSObject for ABPerson {}
impl PNSObject for ABPerson {}
impl IABPerson for ABPerson {}
pub trait IABPerson: Sized + std::ops::Deref {
    unsafe fn parentGroups(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentGroups)
    }
    unsafe fn linkedPeople(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkedPeople)
    }
}
impl ABPerson_ABPerson_Properties for ABPerson {}
pub trait ABPerson_ABPerson_Properties: Sized + std::ops::Deref {
    unsafe fn addPropertiesAndTypes_(properties: NSDictionary) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABPerson").unwrap(), addPropertiesAndTypes : properties)
    }
    unsafe fn removeProperties_(properties: NSArray) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABPerson").unwrap(), removeProperties : properties)
    }
    unsafe fn properties() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABPerson").unwrap(), properties)
    }
    unsafe fn typeOfProperty_(property: NSString) -> ABPropertyType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABPerson").unwrap(), typeOfProperty : property)
    }
}
impl ABPerson_ABPerson_Searching for ABPerson {}
pub trait ABPerson_ABPerson_Searching: Sized + std::ops::Deref {
    unsafe fn searchElementForProperty_label_key_value_comparison_(
        property: NSString,
        label: NSString,
        key: NSString,
        value: id,
        comparison: ABSearchComparison,
    ) -> ABSearchElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABPerson").unwrap(), searchElementForProperty : property, label : label, key : key, value : value, comparison : comparison)
    }
}
impl ABPerson_ABPerson_vCard for ABPerson {}
pub trait ABPerson_ABPerson_vCard: Sized + std::ops::Deref {
    unsafe fn initWithVCardRepresentation_(&self, vCardData: NSData) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVCardRepresentation : vCardData)
    }
    unsafe fn vCardRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vCardRepresentation)
    }
}
pub trait PABImageClient: Sized + std::ops::Deref {
    unsafe fn consumeImageData_forTag_(&self, data: NSData, tag: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, consumeImageData : data, forTag : tag)
    }
}
impl ABPerson_ABPersonImageAdditions for ABPerson {}
pub trait ABPerson_ABPersonImageAdditions: Sized + std::ops::Deref {
    unsafe fn setImageData_(&self, data: NSData) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageData : data)
    }
    unsafe fn imageData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageData)
    }
    unsafe fn beginLoadingImageDataForClient_(&self, client: *mut u64) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginLoadingImageDataForClient : client)
    }
    unsafe fn cancelLoadingImageDataForTag_(tag: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABPerson").unwrap(), cancelLoadingImageDataForTag : tag)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABSearchElement(pub id);
impl std::ops::Deref for ABSearchElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABSearchElement {}
impl ABSearchElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABSearchElement").unwrap(), alloc) })
    }
}
impl INSObject for ABSearchElement {}
impl PNSObject for ABSearchElement {}
impl std::convert::TryFrom<NSObject> for ABSearchElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ABSearchElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABSearchElement").unwrap()) };
        if is_kind_of {
            Ok(ABSearchElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ABSearchElement")
        }
    }
}
impl IABSearchElement for ABSearchElement {}
pub trait IABSearchElement: Sized + std::ops::Deref {
    unsafe fn matchesRecord_(&self, record: ABRecord) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchesRecord : record)
    }
    unsafe fn searchElementForConjunction_children_(
        conjuction: ABSearchConjunction,
        children: NSArray,
    ) -> ABSearchElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ABSearchElement").unwrap(), searchElementForConjunction : conjuction, children : children)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABMultiValue(pub id);
impl std::ops::Deref for ABMultiValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABMultiValue {}
impl ABMultiValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABMultiValue").unwrap(), alloc) })
    }
}
impl PNSCopying for ABMultiValue {}
impl PNSMutableCopying for ABMultiValue {}
impl PNSFastEnumeration for ABMultiValue {}
impl INSObject for ABMultiValue {}
impl PNSObject for ABMultiValue {}
impl std::convert::TryFrom<NSObject> for ABMultiValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ABMultiValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABMultiValue").unwrap()) };
        if is_kind_of {
            Ok(ABMultiValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ABMultiValue")
        }
    }
}
impl IABMultiValue for ABMultiValue {}
pub trait IABMultiValue: Sized + std::ops::Deref {
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn valueAtIndex_(&self, index: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueAtIndex : index)
    }
    unsafe fn labelAtIndex_(&self, index: NSUInteger) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, labelAtIndex : index)
    }
    unsafe fn identifierAtIndex_(&self, index: NSUInteger) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, identifierAtIndex : index)
    }
    unsafe fn indexForIdentifier_(&self, identifier: NSString) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexForIdentifier : identifier)
    }
    unsafe fn primaryIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryIdentifier)
    }
    unsafe fn propertyType(&self) -> ABPropertyType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertyType)
    }
    unsafe fn valueForIdentifier_(&self, identifier: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForIdentifier : identifier)
    }
    unsafe fn labelForIdentifier_(&self, identifier: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, labelForIdentifier : identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABMutableMultiValue(pub id);
impl std::ops::Deref for ABMutableMultiValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABMutableMultiValue {}
impl ABMutableMultiValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABMutableMultiValue").unwrap(), alloc) })
    }
}
impl IABMultiValue for ABMutableMultiValue {}
impl PNSCopying for ABMutableMultiValue {}
impl PNSMutableCopying for ABMutableMultiValue {}
impl PNSFastEnumeration for ABMutableMultiValue {}
impl From<ABMutableMultiValue> for ABMultiValue {
    fn from(child: ABMutableMultiValue) -> ABMultiValue {
        ABMultiValue(child.0)
    }
}
impl std::convert::TryFrom<ABMultiValue> for ABMutableMultiValue {
    type Error = &'static str;
    fn try_from(parent: ABMultiValue) -> Result<ABMutableMultiValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABMutableMultiValue").unwrap()) };
        if is_kind_of {
            Ok(ABMutableMultiValue(parent.0))
        } else {
            Err("This ABMultiValue cannot be downcasted to ABMutableMultiValue")
        }
    }
}
impl INSObject for ABMutableMultiValue {}
impl PNSObject for ABMutableMultiValue {}
impl IABMutableMultiValue for ABMutableMultiValue {}
pub trait IABMutableMultiValue: Sized + std::ops::Deref {
    unsafe fn addValue_withLabel_(&self, value: id, label: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addValue : value, withLabel : label)
    }
    unsafe fn insertValue_withLabel_atIndex_(
        &self,
        value: id,
        label: NSString,
        index: NSUInteger,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertValue : value, withLabel : label, atIndex : index)
    }
    unsafe fn removeValueAndLabelAtIndex_(&self, index: NSUInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeValueAndLabelAtIndex : index)
    }
    unsafe fn replaceValueAtIndex_withValue_(&self, index: NSUInteger, value: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceValueAtIndex : index, withValue : value)
    }
    unsafe fn replaceLabelAtIndex_withLabel_(&self, index: NSUInteger, label: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceLabelAtIndex : index, withLabel : label)
    }
    unsafe fn setPrimaryIdentifier_(&self, identifier: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimaryIdentifier : identifier)
    }
}
pub trait NSObject_ABActionDelegate: Sized + std::ops::Deref {
    unsafe fn actionProperty(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actionProperty)
    }
    unsafe fn titleForPerson_identifier_(&self, person: ABPerson, identifier: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, titleForPerson : person, identifier : identifier)
    }
    unsafe fn performActionForPerson_identifier_(&self, person: ABPerson, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performActionForPerson : person, identifier : identifier)
    }
    unsafe fn shouldEnableActionForPerson_identifier_(
        &self,
        person: ABPerson,
        identifier: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldEnableActionForPerson : person, identifier : identifier)
    }
}
pub type ABPeoplePickerSelectionBehavior = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABPeoplePickerView(pub id);
impl std::ops::Deref for ABPeoplePickerView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABPeoplePickerView {}
impl ABPeoplePickerView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABPeoplePickerView").unwrap(), alloc) })
    }
}
impl INSView for ABPeoplePickerView {}
impl PNSAnimatablePropertyContainer for ABPeoplePickerView {}
impl PNSUserInterfaceItemIdentification for ABPeoplePickerView {}
impl PNSDraggingDestination for ABPeoplePickerView {}
impl PNSAppearanceCustomization for ABPeoplePickerView {}
impl PNSAccessibilityElement for ABPeoplePickerView {}
impl PNSAccessibility for ABPeoplePickerView {}
impl std::convert::TryFrom<NSView> for ABPeoplePickerView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<ABPeoplePickerView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABPeoplePickerView").unwrap()) };
        if is_kind_of {
            Ok(ABPeoplePickerView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to ABPeoplePickerView")
        }
    }
}
impl INSResponder for ABPeoplePickerView {}
impl PNSCoding for ABPeoplePickerView {}
impl INSObject for ABPeoplePickerView {}
impl PNSObject for ABPeoplePickerView {}
impl IABPeoplePickerView for ABPeoplePickerView {}
pub trait IABPeoplePickerView: Sized + std::ops::Deref {
    unsafe fn addProperty_(&self, property: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addProperty : property)
    }
    unsafe fn removeProperty_(&self, property: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeProperty : property)
    }
    unsafe fn properties(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setColumnTitle_forProperty_(&self, title: NSString, property: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColumnTitle : title, forProperty : property)
    }
    unsafe fn columnTitleForProperty_(&self, property: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, columnTitleForProperty : property)
    }
    unsafe fn selectedIdentifiersForPerson_(&self, person: ABPerson) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectedIdentifiersForPerson : person)
    }
    unsafe fn selectGroup_byExtendingSelection_(&self, group: ABGroup, extend_: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectGroup : group, byExtendingSelection : extend_)
    }
    unsafe fn selectRecord_byExtendingSelection_(&self, record: ABRecord, extend_: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectRecord : record, byExtendingSelection : extend_)
    }
    unsafe fn selectIdentifier_forPerson_byExtendingSelection_(
        &self,
        identifier: NSString,
        person: ABPerson,
        extend_: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectIdentifier : identifier, forPerson : person, byExtendingSelection : extend_)
    }
    unsafe fn deselectGroup_(&self, group: ABGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deselectGroup : group)
    }
    unsafe fn deselectRecord_(&self, record: ABRecord)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deselectRecord : record)
    }
    unsafe fn deselectIdentifier_forPerson_(&self, identifier: NSString, person: ABPerson)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deselectIdentifier : identifier, forPerson : person)
    }
    unsafe fn deselectAll_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deselectAll : sender)
    }
    unsafe fn clearSearchField_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clearSearchField : sender)
    }
    unsafe fn accessoryView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryView)
    }
    unsafe fn setAccessoryView_(&self, accessoryView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessoryView : accessoryView)
    }
    unsafe fn valueSelectionBehavior(&self) -> ABPeoplePickerSelectionBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueSelectionBehavior)
    }
    unsafe fn setValueSelectionBehavior_(
        &self,
        valueSelectionBehavior: ABPeoplePickerSelectionBehavior,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueSelectionBehavior : valueSelectionBehavior)
    }
    unsafe fn allowsGroupSelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsGroupSelection)
    }
    unsafe fn setAllowsGroupSelection_(&self, allowsGroupSelection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsGroupSelection : allowsGroupSelection)
    }
    unsafe fn allowsMultipleSelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsMultipleSelection)
    }
    unsafe fn setAllowsMultipleSelection_(&self, allowsMultipleSelection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsMultipleSelection : allowsMultipleSelection)
    }
    unsafe fn displayedProperty(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayedProperty)
    }
    unsafe fn setDisplayedProperty_(&self, displayedProperty: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayedProperty : displayedProperty)
    }
    unsafe fn autosaveName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autosaveName)
    }
    unsafe fn setAutosaveName_(&self, autosaveName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutosaveName : autosaveName)
    }
    unsafe fn selectedGroups(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedGroups)
    }
    unsafe fn selectedRecords(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedRecords)
    }
    unsafe fn target(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, target)
    }
    unsafe fn setTarget_(&self, target: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTarget : target)
    }
    unsafe fn groupDoubleAction(&self) -> objc2::runtime::Sel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupDoubleAction)
    }
    unsafe fn setGroupDoubleAction_(&self, groupDoubleAction: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroupDoubleAction : groupDoubleAction)
    }
    unsafe fn nameDoubleAction(&self) -> objc2::runtime::Sel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nameDoubleAction)
    }
    unsafe fn setNameDoubleAction_(&self, nameDoubleAction: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNameDoubleAction : nameDoubleAction)
    }
}
impl ABPeoplePickerView_ABPeoplePickerConvenience for ABPeoplePickerView {}
pub trait ABPeoplePickerView_ABPeoplePickerConvenience: Sized + std::ops::Deref {
    unsafe fn selectedValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedValues)
    }
    unsafe fn editInAddressBook_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, editInAddressBook : sender)
    }
    unsafe fn selectInAddressBook_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectInAddressBook : sender)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABPersonView(pub id);
impl std::ops::Deref for ABPersonView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABPersonView {}
impl ABPersonView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABPersonView").unwrap(), alloc) })
    }
}
impl INSView for ABPersonView {}
impl PNSAnimatablePropertyContainer for ABPersonView {}
impl PNSUserInterfaceItemIdentification for ABPersonView {}
impl PNSDraggingDestination for ABPersonView {}
impl PNSAppearanceCustomization for ABPersonView {}
impl PNSAccessibilityElement for ABPersonView {}
impl PNSAccessibility for ABPersonView {}
impl std::convert::TryFrom<NSView> for ABPersonView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<ABPersonView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABPersonView").unwrap()) };
        if is_kind_of {
            Ok(ABPersonView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to ABPersonView")
        }
    }
}
impl INSResponder for ABPersonView {}
impl PNSCoding for ABPersonView {}
impl INSObject for ABPersonView {}
impl PNSObject for ABPersonView {}
impl IABPersonView for ABPersonView {}
pub trait IABPersonView: Sized + std::ops::Deref {
    unsafe fn editing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, editing)
    }
    unsafe fn setEditing_(&self, editing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditing : editing)
    }
    unsafe fn person(&self) -> ABPerson
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, person)
    }
    unsafe fn setPerson_(&self, person: ABPerson)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerson : person)
    }
    unsafe fn shouldShowLinkedPeople(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldShowLinkedPeople)
    }
    unsafe fn setShouldShowLinkedPeople_(&self, shouldShowLinkedPeople: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldShowLinkedPeople : shouldShowLinkedPeople)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABPersonPicker(pub id);
impl std::ops::Deref for ABPersonPicker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABPersonPicker {}
impl ABPersonPicker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABPersonPicker").unwrap(), alloc) })
    }
}
impl INSObject for ABPersonPicker {}
impl PNSObject for ABPersonPicker {}
impl std::convert::TryFrom<NSObject> for ABPersonPicker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ABPersonPicker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABPersonPicker").unwrap()) };
        if is_kind_of {
            Ok(ABPersonPicker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ABPersonPicker")
        }
    }
}
impl IABPersonPicker for ABPersonPicker {}
pub trait IABPersonPicker: Sized + std::ops::Deref {
    unsafe fn showRelativeToRect_ofView_preferredEdge_(
        &self,
        positioningRect: NSRect,
        positioningView: NSView,
        preferredEdge: NSRectEdge,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showRelativeToRect : positioningRect, ofView : positioningView, preferredEdge : preferredEdge)
    }
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn properties(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setProperties_(&self, properties: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties)
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
}
pub trait PABPersonPickerDelegate: Sized + std::ops::Deref {
    unsafe fn personPicker_didChoosePerson_property_identifier_(
        &self,
        picker: ABPersonPicker,
        person: ABPerson,
        property: NSString,
        identifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, personPicker : picker, didChoosePerson : person, property : property, identifier : identifier)
    }
    unsafe fn personPickerDidClose_(&self, picker: ABPersonPicker)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, personPickerDidClose : picker)
    }
}
pub type ABRecordRef = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ABPerson {
    _unused: [u8; 0],
}
pub type ABPersonRef = *mut __ABPerson;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ABGroup {
    _unused: [u8; 0],
}
pub type ABGroupRef = *mut __ABGroup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ABSearchElementRef {
    _unused: [u8; 0],
}
pub type ABSearchElementRef = *mut __ABSearchElementRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ABAddressBookRef {
    _unused: [u8; 0],
}
pub type ABAddressBookRef = *mut __ABAddressBookRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __ABMultiValue {
    _unused: [u8; 0],
}
pub type ABMultiValueRef = *const __ABMultiValue;
pub type ABMutableMultiValueRef = *mut __ABMultiValue;
pub type ABActionGetPropertyCallback = ::std::option::Option<unsafe extern "C" fn() -> CFStringRef>;
pub type ABActionCopyTitleCallback = ::std::option::Option<
    unsafe extern "C" fn(person: ABPersonRef, identifier: CFStringRef) -> CFStringRef,
>;
pub type ABActionEnabledCallback = ::std::option::Option<
    unsafe extern "C" fn(person: ABPersonRef, identifier: CFStringRef) -> Boolean,
>;
pub type ABActionSelectedCallback =
    ::std::option::Option<unsafe extern "C" fn(person: ABPersonRef, identifier: CFStringRef)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ABActionCallbacks {
    pub version: CFIndex,
    pub property: ABActionGetPropertyCallback,
    pub title: ABActionCopyTitleCallback,
    pub enabled: ABActionEnabledCallback,
    pub selected: ABActionSelectedCallback,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueABPicker {
    _unused: [u8; 0],
}
pub type ABPickerRef = *mut OpaqueABPicker;
pub type ABPickerAttributes = OptionBits;
unsafe extern "C" {
    pub static ABPeoplePickerGroupSelectionDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static ABPeoplePickerNameSelectionDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static ABPeoplePickerValueSelectionDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static ABPeoplePickerDisplayedPropertyDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub fn ABPickerCreate() -> ABPickerRef;
}
unsafe extern "C" {
    pub fn ABPickerSetFrame(inPicker: ABPickerRef, inFrame: *const HIRect);
}
unsafe extern "C" {
    pub fn ABPickerGetFrame(inPicker: ABPickerRef, outFrame: *mut HIRect);
}
unsafe extern "C" {
    pub fn ABPickerSetVisibility(inPicker: ABPickerRef, visible: bool);
}
unsafe extern "C" {
    pub fn ABPickerIsVisible(inPicker: ABPickerRef) -> bool;
}
unsafe extern "C" {
    pub fn ABPickerGetAttributes(inPicker: ABPickerRef) -> ABPickerAttributes;
}
unsafe extern "C" {
    pub fn ABPickerChangeAttributes(
        inPicker: ABPickerRef,
        inAttributesToSet: ABPickerAttributes,
        inAttributesToClear: ABPickerAttributes,
    );
}
unsafe extern "C" {
    pub fn ABPickerAddProperty(inPicker: ABPickerRef, inProperty: CFStringRef);
}
unsafe extern "C" {
    pub fn ABPickerRemoveProperty(inPicker: ABPickerRef, inProperty: CFStringRef);
}
unsafe extern "C" {
    pub fn ABPickerCopyProperties(inPicker: ABPickerRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ABPickerSetColumnTitle(
        inPicker: ABPickerRef,
        inTitle: CFStringRef,
        inProperty: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn ABPickerCopyColumnTitle(inPicker: ABPickerRef, inProperty: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn ABPickerSetDisplayedProperty(inPicker: ABPickerRef, inProperty: CFStringRef);
}
unsafe extern "C" {
    pub fn ABPickerCopyDisplayedProperty(inPicker: ABPickerRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn ABPickerCopySelectedGroups(inPicker: ABPickerRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ABPickerCopySelectedRecords(inPicker: ABPickerRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ABPickerCopySelectedIdentifiers(
        inPicker: ABPickerRef,
        inPerson: ABPersonRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ABPickerCopySelectedValues(inPicker: ABPickerRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ABPickerSelectGroup(inPicker: ABPickerRef, inGroup: ABGroupRef, inExtendSelection: bool);
}
unsafe extern "C" {
    pub fn ABPickerSelectRecord(
        inPicker: ABPickerRef,
        inRecord: ABRecordRef,
        inExtendSelection: bool,
    );
}
unsafe extern "C" {
    pub fn ABPickerSelectIdentifier(
        inPicker: ABPickerRef,
        inPerson: ABPersonRef,
        inIdentifier: CFStringRef,
        inExtendSelection: bool,
    );
}
unsafe extern "C" {
    pub fn ABPickerDeselectGroup(inPicker: ABPickerRef, inGroup: ABGroupRef);
}
unsafe extern "C" {
    pub fn ABPickerDeselectRecord(inPicker: ABPickerRef, inRecord: ABRecordRef);
}
unsafe extern "C" {
    pub fn ABPickerDeselectIdentifier(
        inPicker: ABPickerRef,
        inPerson: ABPersonRef,
        inIdentifier: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn ABPickerDeselectAll(inPicker: ABPickerRef);
}
unsafe extern "C" {
    pub fn ABPickerSetDelegate(inPicker: ABPickerRef, inDelegate: EventTargetRef);
}
unsafe extern "C" {
    pub fn ABPickerGetDelegate(inPicker: ABPickerRef) -> EventTargetRef;
}
unsafe extern "C" {
    pub fn ABPickerClearSearchField(inPicker: ABPickerRef);
}
unsafe extern "C" {
    pub fn ABPickerEditInAddressBook(inPicker: ABPickerRef);
}
unsafe extern "C" {
    pub fn ABPickerSelectInAddressBook(inPicker: ABPickerRef);
}

unsafe impl objc2::encode::RefEncode for CVTimeStamp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVTimeStamp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVTimeStamp", &[]);
}
unsafe impl objc2::encode::RefEncode for CATransform3D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CATransform3D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CATransform3D", &[]);
}
unsafe impl objc2::encode::RefEncode for NSAccessibilityCustomAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSAccessibilityCustomAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSAccessibilityCustomRotor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSAccessibilityCustomRotor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSAccessibilityCustomRotorSearchParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSAccessibilityCustomRotorSearchParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSAccessibilityCustomRotorItemResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSAccessibilityCustomRotorItemResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSTextStorage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSTextStorage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSLayoutManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSLayoutManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSTextAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSTextAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for QTMovie {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for QTMovie {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSShadow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSShadow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NSDataAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSDataAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABAddressBook {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABAddressBook {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABPerson {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABPerson {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABSearchElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABSearchElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABMultiValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABMultiValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABMutableMultiValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABMutableMultiValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABPeoplePickerView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABPeoplePickerView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABPersonView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABPersonView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABPersonPicker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABPersonPicker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for __ABPerson {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ABPerson {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ABPerson", &[]);
}
unsafe impl objc2::encode::RefEncode for __ABGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ABGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ABGroup", &[]);
}
unsafe impl objc2::encode::RefEncode for __ABSearchElementRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ABSearchElementRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ABSearchElementRef", &[]);
}
unsafe impl objc2::encode::RefEncode for __ABAddressBookRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ABAddressBookRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ABAddressBookRef", &[]);
}
unsafe impl objc2::encode::RefEncode for __ABMultiValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __ABMultiValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__ABMultiValue", &[]);
}
unsafe impl objc2::encode::RefEncode for ABActionCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABActionCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ABActionCallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueABPicker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueABPicker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueABPicker", &[]);
}
