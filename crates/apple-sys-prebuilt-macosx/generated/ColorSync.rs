#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColorSyncProfile {
    _unused: [u8; 0],
}
pub type ColorSyncMutableProfileRef = *mut ColorSyncProfile;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColorSyncMD5 {
    pub digest: [u8; 16usize],
}
pub type ColorSyncProfileIterateCallback = ::std::option::Option<
    unsafe extern "C" fn(
        profileInfo: CFDictionaryRef,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> bool,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColorSyncTransform {
    _unused: [u8; 0],
}
pub type ColorSyncTransformRef = *mut ColorSyncTransform;
pub type ColorSyncDataDepth = ::std::os::raw::c_uint;
pub type ColorSyncAlphaInfo = ::std::os::raw::c_uint;
pub type ColorSyncDataLayout = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColorSyncCMM {
    _unused: [u8; 0],
}
pub type ColorSyncCMMRef = *mut ColorSyncCMM;
pub type ColorSyncCMMIterateCallback = ::std::option::Option<
    unsafe extern "C" fn(cmm: ColorSyncCMMRef, userInfo: *mut ::std::os::raw::c_void) -> bool,
>;
pub type ColorSyncDeviceProfileIterateCallback = ::std::option::Option<
    unsafe extern "C" fn(
        colorSyncDeviceProfileInfo: CFDictionaryRef,
        userInfo: *mut ::std::os::raw::c_void,
    ) -> bool,
>;
unsafe extern "C" {
    pub static mut kColorSyncGenericGrayProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncGenericGrayGamma22Profile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncGenericRGBProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncGenericCMYKProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDisplayP3Profile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSRGBProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncAdobeRGB1998Profile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncGenericLabProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncGenericXYZProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncACESCGLinearProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDCIP3Profile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncITUR709Profile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncITUR2020Profile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncROMMRGBProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncWebSafeColorsProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileHeader: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileColorSpace: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfilePCS: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileURL: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileDescription: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileMD5Digest: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileIsValid: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigAToB0Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigAToB1Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigAToB2Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigBToA0Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigBToA1Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigBToA2Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigCmykData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigGrayData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigLabData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigRgbData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigXYZData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigAbstractClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigBlueTRCTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigBlueColorantTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigMediaBlackPointTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigCopyrightTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigProfileDescriptionTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigDeviceModelDescTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigDeviceMfgDescTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigGreenTRCTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigGreenColorantTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigGamutTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigGrayTRCTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigLinkClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigDisplayClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigNamedColor2Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigNamedColorClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigPreview0Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigPreview1Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigPreview2Tag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigOutputClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigProfileSequenceDescTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigRedTRCTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigRedColorantTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigInputClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigColorSpaceClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigTechnologyTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigViewingConditionsTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigViewingCondDescTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncSigMediaWhitePointTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileComputerDomain: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileUserDomain: CFStringRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreate(data: CFDataRef, error: *mut CFErrorRef) -> ColorSyncProfileRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreateWithURL(
        url: CFURLRef,
        error: *mut CFErrorRef,
    ) -> ColorSyncProfileRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDoNotSubstituteProfiles: CFStringRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreateWithURLAndOptions(
        url: CFURLRef,
        options: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> ColorSyncProfileRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreateWithName(name: CFStringRef) -> ColorSyncProfileRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreateWithDisplayID(displayID: u32) -> ColorSyncProfileRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreateDeviceProfile(
        deviceClass: CFStringRef,
        deviceID: CFUUIDRef,
        profileID: CFTypeRef,
    ) -> ColorSyncProfileRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreateMutable() -> ColorSyncMutableProfileRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreateMutableCopy(
        prof: ColorSyncProfileRef,
    ) -> ColorSyncMutableProfileRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreateLink(
        profileInfo: CFArrayRef,
        options: CFDictionaryRef,
    ) -> ColorSyncProfileRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileVerify(
        prof: ColorSyncProfileRef,
        errors: *mut CFErrorRef,
        warnings: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncProfileIsWideGamut(arg1: ColorSyncProfileRef) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncProfileIsMatrixBased(arg1: ColorSyncProfileRef) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncProfileIsPQBased(arg1: ColorSyncProfileRef) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncProfileIsHLGBased(arg1: ColorSyncProfileRef) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncProfileEstimateGammaWithDisplayID(
        displayID: i32,
        error: *mut CFErrorRef,
    ) -> f32;
}
unsafe extern "C" {
    pub fn ColorSyncProfileGetDisplayTransferFormulaFromVCGT(
        profile: ColorSyncProfileRef,
        redMin: *mut f32,
        redMax: *mut f32,
        redGamma: *mut f32,
        greenMin: *mut f32,
        greenMax: *mut f32,
        greenGamma: *mut f32,
        blueMin: *mut f32,
        blueMax: *mut f32,
        blueGamma: *mut f32,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCreateDisplayTransferTablesFromVCGT(
        profile: ColorSyncProfileRef,
        nSamplesPerChannel: *mut usize,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileEstimateGamma(prof: ColorSyncProfileRef, error: *mut CFErrorRef) -> f32;
}
unsafe extern "C" {
    pub fn ColorSyncProfileGetMD5(prof: ColorSyncProfileRef) -> ColorSyncMD5;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCopyData(prof: ColorSyncProfileRef, error: *mut CFErrorRef)
        -> CFDataRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileGetURL(prof: ColorSyncProfileRef, error: *mut CFErrorRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCopyHeader(prof: ColorSyncProfileRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileSetHeader(prof: ColorSyncMutableProfileRef, header: CFDataRef);
}
unsafe extern "C" {
    pub fn ColorSyncProfileCopyDescriptionString(prof: ColorSyncProfileRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCopyTagSignatures(prof: ColorSyncProfileRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileContainsTag(prof: ColorSyncProfileRef, signature: CFStringRef) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncProfileCopyTag(prof: ColorSyncProfileRef, signature: CFStringRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn ColorSyncProfileSetTag(
        prof: ColorSyncMutableProfileRef,
        signature: CFStringRef,
        data: CFDataRef,
    );
}
unsafe extern "C" {
    pub fn ColorSyncProfileRemoveTag(prof: ColorSyncMutableProfileRef, signature: CFStringRef);
}
unsafe extern "C" {
    pub static mut kColorSyncProfileCacheSeed: CFStringRef;
}
unsafe extern "C" {
    pub fn ColorSyncIterateInstalledProfiles(
        callBack: ColorSyncProfileIterateCallback,
        seed: *mut u32,
        userInfo: *mut ::std::os::raw::c_void,
        error: *mut CFErrorRef,
    );
}
unsafe extern "C" {
    pub static mut kColorSyncWaitForCacheReply: CFStringRef;
}
unsafe extern "C" {
    pub fn ColorSyncIterateInstalledProfilesWithOptions(
        callBack: ColorSyncProfileIterateCallback,
        seed: *mut u32,
        userInfo: *mut ::std::os::raw::c_void,
        options: CFDictionaryRef,
        error: *mut CFErrorRef,
    );
}
unsafe extern "C" {
    pub fn ColorSyncProfileInstall(
        profile: ColorSyncProfileRef,
        domain: CFStringRef,
        subpath: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncProfileUninstall(profile: ColorSyncProfileRef, error: *mut CFErrorRef) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncTransformGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn ColorSyncTransformCreate(
        profileSequence: CFArrayRef,
        options: CFDictionaryRef,
    ) -> ColorSyncTransformRef;
}
unsafe extern "C" {
    pub fn ColorSyncTransformCopyProperty(
        transform: ColorSyncTransformRef,
        key: CFTypeRef,
        options: CFDictionaryRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn ColorSyncTransformSetProperty(
        transform: ColorSyncTransformRef,
        key: CFTypeRef,
        property: CFTypeRef,
    );
}
unsafe extern "C" {
    pub fn ColorSyncTransformGetProfileSequence(transform: ColorSyncTransformRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn ColorSyncTransformConvert(
        transform: ColorSyncTransformRef,
        width: usize,
        height: usize,
        dst: *mut ::std::os::raw::c_void,
        dstDepth: ColorSyncDataDepth,
        dstLayout: ColorSyncDataLayout,
        dstBytesPerRow: usize,
        src: *const ::std::os::raw::c_void,
        srcDepth: ColorSyncDataDepth,
        srcLayout: ColorSyncDataLayout,
        srcBytesPerRow: usize,
        options: CFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub static mut kColorSyncProfile: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncRenderingIntent: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncRenderingIntentPerceptual: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncRenderingIntentRelative: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncRenderingIntentSaturation: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncRenderingIntentAbsolute: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncRenderingIntentUseProfileHeader: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformTag: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformDeviceToPCS: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformPCSToPCS: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformPCSToDevice: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformDeviceToDevice: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformGamutCheck: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncBlackPointCompensation: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncExtendedRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncHDRDerivative: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncPQDerivative: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncHLGDerivative: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncPreferredCMM: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConvertQuality: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncBestQuality: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncNormalQuality: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDraftQuality: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConvertUseExtendedRange: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformInfo: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformCreator: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformSrcSpace: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformDstSpace: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformCodeFragmentType: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformCodeFragmentMD5: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformFullConversionData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformSimplifiedConversionData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformParametricConversionData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformProfileSequnce: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncTransformUseITU709OETF: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionMatrix: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionParamCurve0: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionParamCurve1: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionParamCurve2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionParamCurve3: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionParamCurve4: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversion1DLut: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionGridPoints: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionChannelID: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversion3DLut: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionNDLut: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionInpChan: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionOutChan: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncConversionBPC: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncFixedPointRange: CFStringRef;
}
unsafe extern "C" {
    pub fn ColorSyncCreateCodeFragment(
        profileSequence: CFArrayRef,
        options: CFDictionaryRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn ColorSyncCMMGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn ColorSyncCMMCreate(cmmBundle: CFBundleRef) -> ColorSyncCMMRef;
}
unsafe extern "C" {
    pub fn ColorSyncCMMGetBundle(arg1: ColorSyncCMMRef) -> CFBundleRef;
}
unsafe extern "C" {
    pub fn ColorSyncCMMCopyLocalizedName(arg1: ColorSyncCMMRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn ColorSyncCMMCopyCMMIdentifier(arg1: ColorSyncCMMRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn ColorSyncIterateInstalledCMMs(
        callBack: ColorSyncCMMIterateCallback,
        userInfo: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub static mut kCMMInitializeLinkProfileProcName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMMInitializeTransformProcName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMMApplyTransformProcName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMMCreateTransformPropertyProcName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceID: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncCameraDeviceClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDisplayDeviceClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncPrinterDeviceClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncScannerDeviceClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceProfileURL: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceDescription: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceDescriptions: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncFactoryProfiles: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncCustomProfiles: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceModeDescription: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceModeDescriptions: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceDefaultProfileID: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceHostScope: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceUserScope: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileHostScope: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileUserScope: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceProfileIsFactory: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceProfileIsDefault: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceProfileIsCurrent: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceProfileID: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceRegisteredNotification: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceUnregisteredNotification: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDeviceProfilesNotification: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncDisplayDeviceProfilesNotification: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncProfileRepositoryChangeNotification: CFStringRef;
}
unsafe extern "C" {
    pub static mut kColorSyncRegistrationUpdateWindowServer: CFStringRef;
}
unsafe extern "C" {
    pub fn ColorSyncRegisterDevice(
        deviceClass: CFStringRef,
        deviceID: CFUUIDRef,
        deviceInfo: CFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncUnregisterDevice(deviceClass: CFStringRef, deviceID: CFUUIDRef) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncDeviceSetCustomProfiles(
        deviceClass: CFStringRef,
        deviceID: CFUUIDRef,
        profileInfo: CFDictionaryRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn ColorSyncDeviceCopyDeviceInfo(
        deviceClass: CFStringRef,
        devID: CFUUIDRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn ColorSyncIterateDeviceProfiles(
        callBack: ColorSyncDeviceProfileIterateCallback,
        userInfo: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CGDisplayCreateUUIDFromDisplayID(displayID: u32) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn CGDisplayGetDisplayIDFromUUID(uuid: CFUUIDRef) -> u32;
}
unsafe extern "C" {
    pub fn ColorSyncAPIVersion() -> u32;
}

unsafe impl objc2::encode::RefEncode for ColorSyncProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ColorSyncProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ColorSyncProfile", &[]);
}
unsafe impl objc2::encode::RefEncode for ColorSyncMD5 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ColorSyncMD5 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ColorSyncMD5", &[]);
}
unsafe impl objc2::encode::RefEncode for ColorSyncTransform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ColorSyncTransform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ColorSyncTransform", &[]);
}
unsafe impl objc2::encode::RefEncode for ColorSyncCMM {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ColorSyncCMM {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ColorSyncCMM", &[]);
}
