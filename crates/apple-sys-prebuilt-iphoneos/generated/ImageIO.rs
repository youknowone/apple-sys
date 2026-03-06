#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGImageSource {
    _unused: [u8; 0],
}
pub type CGImageSourceRef = *mut CGImageSource;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGImageMetadata {
    _unused: [u8; 0],
}
pub type CGImageMetadataRef = *const CGImageMetadata;
pub type CGMutableImageMetadataRef = *mut CGImageMetadata;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGImageMetadataTag {
    _unused: [u8; 0],
}
pub type CGImageMetadataTagRef = *mut CGImageMetadataTag;
pub type CGImageMetadataType = i32;
pub type CGImageMetadataTagBlock = *mut ::std::os::raw::c_void;
pub type CGImageMetadataErrors = i32;
pub type CGImageSourceStatus = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGImageDestination {
    _unused: [u8; 0],
}
pub type CGImageDestinationRef = *mut CGImageDestination;
pub type CGImagePropertyOrientation = u32;
pub type CGImagePropertyTGACompression = u32;
pub type CGImageAnimationStatus = OSStatus;
pub type CGImageSourceAnimationBlock = *mut ::std::os::raw::c_void;
unsafe extern "C" {
    pub fn CGImageMetadataGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGImageMetadataCreateMutable() -> CGMutableImageMetadataRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataCreateMutableCopy(
        metadata: CGImageMetadataRef,
    ) -> CGMutableImageMetadataRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataTagGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespaceExif: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespaceExifAux: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespaceExifEX: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespaceDublinCore: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespaceIPTCCore: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespaceIPTCExtension: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespacePhotoshop: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespaceTIFF: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespaceXMPBasic: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataNamespaceXMPRights: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixExif: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixExifAux: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixExifEX: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixDublinCore: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixIPTCCore: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixIPTCExtension: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixPhotoshop: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixTIFF: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixXMPBasic: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataPrefixXMPRights: CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataTagCreate(
        xmlns: CFStringRef,
        prefix: CFStringRef,
        name: CFStringRef,
        type_: CGImageMetadataType,
        value: CFTypeRef,
    ) -> CGImageMetadataTagRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataTagCopyNamespace(tag: CGImageMetadataTagRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataTagCopyPrefix(tag: CGImageMetadataTagRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataTagCopyName(tag: CGImageMetadataTagRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataTagCopyValue(tag: CGImageMetadataTagRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataTagGetType(tag: CGImageMetadataTagRef) -> CGImageMetadataType;
}
unsafe extern "C" {
    pub fn CGImageMetadataTagCopyQualifiers(tag: CGImageMetadataTagRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataCopyTags(metadata: CGImageMetadataRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataCopyTagWithPath(
        metadata: CGImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
    ) -> CGImageMetadataTagRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataCopyStringValueWithPath(
        metadata: CGImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataRegisterNamespaceForPrefix(
        metadata: CGMutableImageMetadataRef,
        xmlns: CFStringRef,
        prefix: CFStringRef,
        err: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGImageMetadataSetTagWithPath(
        metadata: CGMutableImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
        tag: CGImageMetadataTagRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGImageMetadataSetValueWithPath(
        metadata: CGMutableImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
        value: CFTypeRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGImageMetadataRemoveTagWithPath(
        metadata: CGMutableImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGImageMetadataEnumerateTagsUsingBlock(
        metadata: CGImageMetadataRef,
        rootPath: CFStringRef,
        options: CFDictionaryRef,
        block: CGImageMetadataTagBlock,
    );
}
unsafe extern "C" {
    pub static mut kCGImageMetadataEnumerateRecursively: CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataCopyTagMatchingImageProperty(
        metadata: CGImageMetadataRef,
        dictionaryName: CFStringRef,
        propertyName: CFStringRef,
    ) -> CGImageMetadataTagRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataSetValueMatchingImageProperty(
        metadata: CGMutableImageMetadataRef,
        dictionaryName: CFStringRef,
        propertyName: CFStringRef,
        value: CFTypeRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGImageMetadataCreateXMPData(
        metadata: CGImageMetadataRef,
        options: CFDictionaryRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CGImageMetadataCreateFromXMPData(data: CFDataRef) -> CGImageMetadataRef;
}
unsafe extern "C" {
    pub static mut kCFErrorDomainCGImageMetadata: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceTypeIdentifierHint: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceShouldCache: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceShouldCacheImmediately: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceShouldAllowFloat: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceCreateThumbnailFromImageIfAbsent: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceCreateThumbnailFromImageAlways: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceThumbnailMaxPixelSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceCreateThumbnailWithTransform: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceSubsampleFactor: CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageSourceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGImageSourceCopyTypeIdentifiers() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGImageSourceCreateWithDataProvider(
        provider: CGDataProviderRef,
        options: CFDictionaryRef,
    ) -> CGImageSourceRef;
}
unsafe extern "C" {
    pub fn CGImageSourceCreateWithData(
        data: CFDataRef,
        options: CFDictionaryRef,
    ) -> CGImageSourceRef;
}
unsafe extern "C" {
    pub fn CGImageSourceCreateWithURL(url: CFURLRef, options: CFDictionaryRef) -> CGImageSourceRef;
}
unsafe extern "C" {
    pub fn CGImageSourceGetType(isrc: CGImageSourceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageSourceGetCount(isrc: CGImageSourceRef) -> usize;
}
unsafe extern "C" {
    pub fn CGImageSourceCopyProperties(
        isrc: CGImageSourceRef,
        options: CFDictionaryRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGImageSourceCopyPropertiesAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CGImageSourceCopyMetadataAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CGImageMetadataRef;
}
unsafe extern "C" {
    pub fn CGImageSourceCreateImageAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageSourceRemoveCacheAtIndex(isrc: CGImageSourceRef, index: usize);
}
unsafe extern "C" {
    pub fn CGImageSourceCreateThumbnailAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CGImageRef;
}
unsafe extern "C" {
    pub fn CGImageSourceCreateIncremental(options: CFDictionaryRef) -> CGImageSourceRef;
}
unsafe extern "C" {
    pub fn CGImageSourceUpdateData(isrc: CGImageSourceRef, data: CFDataRef, final_: bool);
}
unsafe extern "C" {
    pub fn CGImageSourceUpdateDataProvider(
        isrc: CGImageSourceRef,
        provider: CGDataProviderRef,
        final_: bool,
    );
}
unsafe extern "C" {
    pub fn CGImageSourceGetStatus(isrc: CGImageSourceRef) -> CGImageSourceStatus;
}
unsafe extern "C" {
    pub fn CGImageSourceGetStatusAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
    ) -> CGImageSourceStatus;
}
unsafe extern "C" {
    pub fn CGImageSourceGetPrimaryImageIndex(isrc: CGImageSourceRef) -> usize;
}
unsafe extern "C" {
    pub fn CGImageSourceCopyAuxiliaryDataInfoAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        auxiliaryImageDataType: CFStringRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub static kCGImageSourceDecodeRequest: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceDecodeToHDR: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceDecodeToSDR: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceGenerateImageSpecificLumaScaling: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageSourceDecodeRequestOptions: CFStringRef;
}
unsafe extern "C" {
    pub static kCGComputeHDRStats: CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageSourceSetAllowableTypes(allowableTypes: CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub static kCGImageDestinationLossyCompressionQuality: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationBackgroundColor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationImageMaxPixelSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEmbedThumbnail: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationOptimizeColorForSharing: CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageDestinationGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CGImageDestinationCopyTypeIdentifiers() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CGImageDestinationCreateWithDataConsumer(
        consumer: CGDataConsumerRef,
        type_: CFStringRef,
        count: usize,
        options: CFDictionaryRef,
    ) -> CGImageDestinationRef;
}
unsafe extern "C" {
    pub fn CGImageDestinationCreateWithData(
        data: CFMutableDataRef,
        type_: CFStringRef,
        count: usize,
        options: CFDictionaryRef,
    ) -> CGImageDestinationRef;
}
unsafe extern "C" {
    pub fn CGImageDestinationCreateWithURL(
        url: CFURLRef,
        type_: CFStringRef,
        count: usize,
        options: CFDictionaryRef,
    ) -> CGImageDestinationRef;
}
unsafe extern "C" {
    pub fn CGImageDestinationSetProperties(
        idst: CGImageDestinationRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn CGImageDestinationAddImage(
        idst: CGImageDestinationRef,
        image: CGImageRef,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn CGImageDestinationAddImageFromSource(
        idst: CGImageDestinationRef,
        isrc: CGImageSourceRef,
        index: usize,
        properties: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub fn CGImageDestinationFinalize(idst: CGImageDestinationRef) -> bool;
}
unsafe extern "C" {
    pub fn CGImageDestinationAddImageAndMetadata(
        idst: CGImageDestinationRef,
        image: CGImageRef,
        metadata: CGImageMetadataRef,
        options: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub static kCGImageDestinationPreserveGainMap: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationMetadata: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationMergeMetadata: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataShouldExcludeXMP: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageMetadataShouldExcludeGPS: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationDateTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationOrientation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyEncoder: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyASTCEncoder: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPVREncoder: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyBCEncoder: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyBCFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyASTCBlockSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyASTCBlockSize4x4: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyASTCBlockSize8x8: CFStringRef;
}
unsafe extern "C" {
    pub fn CGImageDestinationCopyImageSource(
        idst: CGImageDestinationRef,
        isrc: CGImageSourceRef,
        options: CFDictionaryRef,
        err: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CGImageDestinationAddAuxiliaryDataInfo(
        idst: CGImageDestinationRef,
        auxiliaryImageDataType: CFStringRef,
        auxiliaryDataInfoDictionary: CFDictionaryRef,
    );
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeRequest: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeToSDR: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeToISOHDR: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeToISOGainmap: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeRequestOptions: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeBaseIsSDR: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeTonemapMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeIsBaseImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeBaseColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeBasePixelFormatRequest: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeGenerateGainMapWithBaseImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeGainMapPixelFormatRequest: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeGainMapSubsampleFactor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageDestinationEncodeAlternateColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGIFDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyJFIFDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHEIFDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHEICSDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyRawDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerCanonDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerMinoltaDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerFujiDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerOlympusDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerPentaxDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageProperty8BIMDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyOpenEXRDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerAppleDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyFileContentsDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyWebPDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyAVISDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTGADictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyFileSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPixelHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPixelWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDPIHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDPIWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDepth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyOrientation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIsFloat: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIsIndexed: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHasAlpha: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyColorModel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyProfileName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPrimaryImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyColorModelRGB: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyColorModelGray: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyColorModelCMYK: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyColorModelLab: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFCompression: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFPhotometricInterpretation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFDocumentName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFImageDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFMake: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFModel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFOrientation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFXResolution: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFYResolution: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFXPosition: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFYPosition: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFResolutionUnit: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFSoftware: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFTransferFunction: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFDateTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFArtist: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFHostComputer: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFCopyright: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFWhitePoint: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFPrimaryChromaticities: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFTileWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTIFFTileLength: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyJFIFVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyJFIFXDensity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyJFIFYDensity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyJFIFDensityUnit: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyJFIFIsProgressive: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHEICSLoopCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHEICSDelayTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHEICSUnclampedDelayTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHEICSCanvasPixelWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHEICSCanvasPixelHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHEICSFrameInfoArray: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifExposureTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifExposureProgram: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSpectralSensitivity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifISOSpeedRatings: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifOECF: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSensitivityType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifStandardOutputSensitivity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifRecommendedExposureIndex: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifISOSpeed: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifISOSpeedLatitudeyyy: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifISOSpeedLatitudezzz: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifDateTimeOriginal: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifDateTimeDigitized: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifOffsetTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifOffsetTimeOriginal: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifOffsetTimeDigitized: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifComponentsConfiguration: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifCompressedBitsPerPixel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifShutterSpeedValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifApertureValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifBrightnessValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifExposureBiasValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifMaxApertureValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSubjectDistance: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifMeteringMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifLightSource: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFlash: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFocalLength: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSubjectArea: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifMakerNote: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifUserComment: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSubsecTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSubsecTimeOriginal: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSubsecTimeDigitized: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFlashPixVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifPixelXDimension: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifPixelYDimension: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifRelatedSoundFile: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFlashEnergy: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSpatialFrequencyResponse: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFocalPlaneXResolution: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFocalPlaneYResolution: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFocalPlaneResolutionUnit: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSubjectLocation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifExposureIndex: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSensingMethod: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFileSource: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSceneType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifCFAPattern: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifCustomRendered: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifExposureMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifWhiteBalance: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifDigitalZoomRatio: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifFocalLenIn35mmFilm: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSceneCaptureType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifGainControl: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifContrast: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSaturation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSharpness: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifDeviceSettingDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSubjectDistRange: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifImageUniqueID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifCameraOwnerName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifBodySerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifLensSpecification: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifLensMake: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifLensModel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifLensSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifGamma: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifCompositeImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSourceImageNumberOfCompositeImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSourceExposureTimesOfCompositeImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifSubsecTimeOrginal: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxLensInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxLensModel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxLensID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxLensSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxImageNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxFlashCompensation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxOwnerName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyExifAuxFirmware: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGIFLoopCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGIFDelayTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGIFImageColorMap: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGIFHasGlobalColorMap: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGIFUnclampedDelayTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGIFCanvasPixelWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGIFCanvasPixelHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGIFFrameInfoArray: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGAuthor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGChromaticities: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGComment: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGCopyright: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGCreationTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGDisclaimer: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGGamma: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGInterlaceType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGModificationTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGSoftware: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGSource: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGsRGBIntent: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGTitle: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGWarning: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGXPixelsPerMeter: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGYPixelsPerMeter: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGPixelsAspectRatio: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyAPNGLoopCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyAPNGDelayTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyAPNGUnclampedDelayTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyAPNGFrameInfoArray: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyAPNGCanvasPixelWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyAPNGCanvasPixelHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyWebPLoopCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyWebPDelayTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyWebPUnclampedDelayTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyWebPFrameInfoArray: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyWebPCanvasPixelWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyWebPCanvasPixelHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSLatitudeRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSLatitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSLongitudeRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSLongitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSAltitudeRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSAltitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSTimeStamp: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSSatellites: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSMeasureMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDOP: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSSpeedRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSSpeed: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSTrackRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSTrack: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSImgDirectionRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSImgDirection: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSMapDatum: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDestLatitudeRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDestLatitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDestLongitudeRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDestLongitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDestBearingRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDestBearing: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDestDistanceRef: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDestDistance: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSProcessingMethod: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSAreaInformation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDateStamp: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSDifferental: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGPSHPositioningError: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCObjectTypeReference: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCObjectAttributeReference: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCObjectName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCEditStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCEditorialUpdate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCUrgency: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCSubjectReference: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCCategory: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCSupplementalCategory: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCFixtureIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCKeywords: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContentLocationCode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContentLocationName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCReleaseDate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCReleaseTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExpirationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExpirationTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCSpecialInstructions: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCActionAdvised: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCReferenceService: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCReferenceDate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCReferenceNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCDateCreated: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCTimeCreated: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCDigitalCreationDate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCDigitalCreationTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCOriginatingProgram: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCProgramVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCObjectCycle: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCByline: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCBylineTitle: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCCity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCSubLocation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCProvinceState: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCCountryPrimaryLocationCode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCCountryPrimaryLocationName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCOriginalTransmissionReference: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCHeadline: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCCredit: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCSource: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCCopyrightNotice: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContact: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCCaptionAbstract: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCWriterEditor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCImageType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCImageOrientation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCLanguageIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCStarRating: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCCreatorContactInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCRightsUsageTerms: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCScene: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtAboutCvTerm: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtAboutCvTermCvId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtAboutCvTermId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtAboutCvTermName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtAboutCvTermRefinedAbout: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtAddlModelInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkOrObject: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkCircaDateCreated: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkContentDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkContributionDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkCopyrightNotice: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkCreator: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkCreatorID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkCopyrightOwnerID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkCopyrightOwnerName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkLicensorID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkLicensorName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkDateCreated: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkPhysicalDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkSource: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkSourceInventoryNo: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkSourceInvURL: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkStylePeriod: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtArtworkTitle: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtAudioBitrate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtAudioBitrateMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtAudioChannelCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtCircaDateCreated: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtContainerFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtContainerFormatIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtContainerFormatName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtContributor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtContributorIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtContributorName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtContributorRole: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtCopyrightYear: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtCreator: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtCreatorIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtCreatorName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtCreatorRole: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtControlledVocabularyTerm: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDataOnScreen: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDataOnScreenRegion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDataOnScreenRegionD: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDataOnScreenRegionH: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDataOnScreenRegionText: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDataOnScreenRegionUnit: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDataOnScreenRegionW: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDataOnScreenRegionX: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDataOnScreenRegionY: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDigitalImageGUID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDigitalSourceFileType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDigitalSourceType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDopesheet: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDopesheetLink: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDopesheetLinkLink: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtDopesheetLinkLinkQualifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtEmbdEncRightsExpr: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtEmbeddedEncodedRightsExpr: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtEmbeddedEncodedRightsExprType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtEmbeddedEncodedRightsExprLangID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtEpisode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtEpisodeIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtEpisodeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtEpisodeNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtEvent: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtShownEvent: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtShownEventIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtShownEventName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtExternalMetadataLink: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtFeedIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtGenre: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtGenreCvId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtGenreCvTermId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtGenreCvTermName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtGenreCvTermRefinedAbout: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtHeadline: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtIPTCLastEdited: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLinkedEncRightsExpr: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLinkedEncodedRightsExpr: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLinkedEncodedRightsExprType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLinkedEncodedRightsExprLangID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationCreated: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationCity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationCountryCode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationCountryName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationGPSAltitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationGPSLatitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationGPSLongitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationLocationId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationLocationName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationProvinceState: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationSublocation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationWorldRegion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtLocationShown: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtMaxAvailHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtMaxAvailWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtModelAge: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtOrganisationInImageCode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtOrganisationInImageName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonHeard: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonHeardIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonHeardName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImageWDetails: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImageCharacteristic: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImageCvTermCvId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImageCvTermId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImageCvTermName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImageCvTermRefinedAbout: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImageDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImageId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPersonInImageName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtProductInImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtProductInImageDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtProductInImageGTIN: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtProductInImageName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPublicationEvent: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPublicationEventDate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPublicationEventIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtPublicationEventName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRating: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRatingRegion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionCity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionCountryCode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionCountryName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionGPSAltitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionGPSLatitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionGPSLongitude: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionLocationId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionLocationName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionProvinceState: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionSublocation: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingRegionWorldRegion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingScaleMaxValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingScaleMinValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingSourceLink: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingValue: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRatingValueLogoLink: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRegistryID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRegistryEntryRole: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRegistryItemID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtRegistryOrganisationID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtReleaseReady: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSeason: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSeasonIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSeasonName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSeasonNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSeries: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSeriesIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSeriesName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtStorylineIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtStreamReady: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtStylePeriod: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSupplyChainSource: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSupplyChainSourceIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtSupplyChainSourceName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtTemporalCoverage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtTemporalCoverageFrom: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtTemporalCoverageTo: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtTranscript: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtTranscriptLink: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtTranscriptLinkLink: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtTranscriptLinkLinkQualifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtVideoBitrate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtVideoBitrateMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtVideoDisplayAspectRatio: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtVideoEncodingProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtVideoShotType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtVideoShotTypeIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtVideoShotTypeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtVideoStreamsCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtVisualColor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtWorkflowTag: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtWorkflowTagCvId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtWorkflowTagCvTermId: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtWorkflowTagCvTermName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCExtWorkflowTagCvTermRefinedAbout: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContactInfoCity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContactInfoCountry: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContactInfoAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContactInfoPostalCode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContactInfoStateProvince: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContactInfoEmails: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContactInfoPhones: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyIPTCContactInfoWebURLs: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageProperty8BIMLayerNames: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageProperty8BIMVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBackwardVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGUniqueCameraModel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGLocalizedCameraModel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCameraSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGLensInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBlackLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGWhiteLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCalibrationIlluminant1: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCalibrationIlluminant2: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGColorMatrix1: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGColorMatrix2: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCameraCalibration1: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCameraCalibration2: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGAsShotNeutral: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGAsShotWhiteXY: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBaselineExposure: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBaselineNoise: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBaselineSharpness: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGPrivateData: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCameraCalibrationSignature: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileCalibrationSignature: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGNoiseProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGWarpRectilinear: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGWarpFisheye: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGFixVignetteRadial: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGActiveArea: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGAnalogBalance: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGAntiAliasStrength: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGAsShotICCProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGAsShotPreProfileMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGAsShotProfileName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBaselineExposureOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBayerGreenSplit: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBestQualityScale: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBlackLevelDeltaH: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBlackLevelDeltaV: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGBlackLevelRepeatDim: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCFALayout: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCFAPlaneColor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGChromaBlurRadius: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGColorimetricReference: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCurrentICCProfile: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGCurrentPreProfileMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGDefaultBlackRender: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGDefaultCropOrigin: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGDefaultCropSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGDefaultScale: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGDefaultUserCrop: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGExtraCameraProfiles: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGForwardMatrix1: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGForwardMatrix2: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGLinearizationTable: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGLinearResponseLimit: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGMakerNoteSafety: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGMaskedAreas: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGNewRawImageDigest: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGNoiseReductionApplied: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGOpcodeList1: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGOpcodeList2: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGOpcodeList3: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGOriginalBestQualityFinalSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGOriginalDefaultCropSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGOriginalDefaultFinalSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGOriginalRawFileData: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGOriginalRawFileDigest: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGOriginalRawFileName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGPreviewApplicationName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGPreviewApplicationVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGPreviewColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGPreviewDateTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGPreviewSettingsDigest: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGPreviewSettingsName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileCopyright: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileEmbedPolicy: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileHueSatMapData1: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileHueSatMapData2: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileHueSatMapDims: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileHueSatMapEncoding: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileLookTableData: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileLookTableDims: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileLookTableEncoding: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGProfileToneCurve: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGRawDataUniqueID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGRawImageDigest: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGRawToPreviewGain: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGReductionMatrix1: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGReductionMatrix2: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGRowInterleaveFactor: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGShadowScale: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyDNGSubTileBlockSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFFirmware: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFOwnerName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFImageName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFImageFileName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFReleaseMethod: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFReleaseTiming: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFRecordID: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFSelfTimingTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFCameraSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFImageSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFContinuousDrive: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFFocusMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFMeteringMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFShootingMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFLensModel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFLensMaxMM: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFLensMinMM: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFWhiteBalanceIndex: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFFlashExposureComp: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyCIFFMeasuredEV: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonISOSetting: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonColorMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonQuality: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonWhiteBalanceMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonSharpenMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonFocusMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonFlashSetting: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonISOSelection: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonFlashExposureComp: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonImageAdjustment: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonLensAdapter: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonLensType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonLensInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonFocusDistance: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonDigitalZoom: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonShootingMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonCameraSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerNikonShutterCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerCanonOwnerName: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerCanonCameraSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerCanonImageSerialNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerCanonFlashExposureComp: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerCanonContinuousDrive: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerCanonLensModel: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerCanonFirmware: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyMakerCanonAspectRatioInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyOpenEXRCompression: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyOpenEXRAspectRatio: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyTGACompression: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGCompressionFilter: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPNGTransparency: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypeDepth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypeDisparity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypePortraitEffectsMatte: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypeSemanticSegmentationSkinMatte: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypeSemanticSegmentationHairMatte: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypeSemanticSegmentationTeethMatte: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypeSemanticSegmentationGlassesMatte: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypeSemanticSegmentationSkyMatte: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypeHDRGainMap: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataTypeISOGainMap: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataInfoData: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataInfoDataDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataInfoMetadata: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAuxiliaryDataInfoColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyImageCount: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyBytesPerRow: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyNamedColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyPixelFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyImages: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyThumbnailImages: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyAuxiliaryData: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyAuxiliaryDataType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyImageIndex: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroups: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupIndex: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupType: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupTypeStereoPair: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupTypeAlternate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImagesAlternate: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageIndexLeft: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageIndexRight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageIndexMonoscopic: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageIsLeftImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageIsRightImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageIsMonoscopicImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageIsAlternateImage: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageBaseline: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageDisparityAdjustment: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupImageStereoAggressors: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOStereoAggressors_Type: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOStereoAggressors_SubTypeURI: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOStereoAggressors_Severity: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImagePropertyGroupMonoscopicImageLocation: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOMonoscopicImageLocation_Unspecified: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOMonoscopicImageLocation_Left: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOMonoscopicImageLocation_Right: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOMonoscopicImageLocation_Center: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOMetadata_CameraExtrinsicsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOCameraExtrinsics_CoordinateSystemID: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOCameraExtrinsics_Position: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOCameraExtrinsics_Rotation: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOMetadata_CameraModelKey: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOCameraModel_ModelType: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOCameraModelType_SimplifiedPinhole: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOCameraModelType_GenericPinhole: CFStringRef;
}
unsafe extern "C" {
    pub static kIIOCameraModel_Intrinsics: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageProviderPreferredTileWidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageProviderPreferredTileHeight: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAnimationStartIndex: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAnimationDelayTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCGImageAnimationLoopCount: CFStringRef;
}
unsafe extern "C" {
    pub fn CGAnimateImageAtURLWithBlock(
        url: CFURLRef,
        options: CFDictionaryRef,
        block: CGImageSourceAnimationBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CGAnimateImageDataWithBlock(
        data: CFDataRef,
        options: CFDictionaryRef,
        block: CGImageSourceAnimationBlock,
    ) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for CGImageSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGImageSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGImageSource", &[]);
}
unsafe impl objc2::encode::RefEncode for CGImageMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGImageMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGImageMetadata", &[]);
}
unsafe impl objc2::encode::RefEncode for CGImageMetadataTag {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGImageMetadataTag {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGImageMetadataTag", &[]);
}
unsafe impl objc2::encode::RefEncode for CGImageDestination {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGImageDestination {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGImageDestination", &[]);
}
